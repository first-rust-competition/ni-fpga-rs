extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn cluster(item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemStruct);

    // We rely on the packed_struct library to pack and unpack clusters. This may
    // change in the future, but as long as we use it we need to add some necessary
    // annotations to each cluster.
    let extra_attrs: Vec<syn::Attribute> = vec![
        syn::parse_quote! { #[derive(Clone, Copy)] },
        syn::parse_quote! { #[derive(PackedStruct)] },
        syn::parse_quote! { #[packed_struct(bit_numbering="msb0", endian="msb")] },
    ];
    input.attrs.extend(extra_attrs);

    // Packed_struct also requires that we explicitly annotate that the first field
    // of each cluster starts at bit 0.
    let first_field_attr: syn::Attribute = syn::parse_quote! { #[packed_field(bits="0..")] };
    let maybe_first_field = match input.fields {
        syn::Fields::Named(ref mut fields) => fields.named.iter_mut().next(),
        syn::Fields::Unnamed(ref mut fields) => fields.unnamed.iter_mut().next(),
        syn::Fields::Unit => None,
    };
    if let Some(field) = maybe_first_field {
        field.attrs.push(first_field_attr);
    }

    // Clusters need to be public to work with the ni-fpga interface. We can save
    // the user a little effort by ensuring the cluster is public for them.
    input.vis = syn::parse_quote! { pub };

    let name = &input.ident;

    let output = quote! {
        #input

        impl ni_fpga::Datatype for #name {
            fn read(
                session: &ni_fpga::Session,
                offset:ni_fpga:: Offset,
            ) -> Result<Self, ni_fpga::Status> {
                let raw_data: [u8; std::mem::size_of::<Self>()] = session.read(offset)?;
                let slice_size = Self::packed_bytes();
                Ok(Self::unpack_from_slice(&raw_data[0..slice_size]).unwrap())
            }
            fn write(
                session: &ni_fpga::Session,
                offset: ni_fpga::Offset,
                value: Self,
            ) -> Result<(), ni_fpga::Status> {
                let mut raw_data: [u8; std::mem::size_of::<Self>()] = Default::default();
                let slice_size = Self::packed_bytes();
                value.pack_to_slice(&mut raw_data[0..slice_size]).unwrap();
                session.write(offset, raw_data)
            }
        }
    };

    output.into()
}
