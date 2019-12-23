extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

struct ClusterInput {
    ident: syn::Ident,
    _brace_token: syn::token::Brace,
    fields: syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
}

impl syn::parse::Parse for ClusterInput {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let content;
        Ok(ClusterInput {
            ident: input.parse()?,
            _brace_token: syn::braced!(content in input),
            fields: content.parse_terminated(syn::Field::parse_named)?,
        })
    }
}

#[proc_macro]
pub fn cluster(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ClusterInput);

    let name = &input.ident;
    let fields = &input.fields;

    let output = quote! {
        #[repr(C, align(4))]
        #[derive(Clone, Copy)]
        #[derive(PackedStruct)]
        #[packed_struct(bit_numbering="msb0", endian="msb")]
        pub struct #name {
            #[packed_field(bits="0..")]
            #fields
        }

        impl ni_fpga::Datatype for #name {
            fn read(session: &ni_fpga::Session, offset:ni_fpga:: Offset) -> Result<Self, ni_fpga::Status> {
                let raw_data: [u8; std::mem::size_of::<Self>()] = session.read(offset)?;
                let slice_size = Self::packed_bytes();
                Ok(Self::unpack_from_slice(&raw_data[0..slice_size]).unwrap())
            }
            fn write(session: &ni_fpga::Session, offset: ni_fpga::Offset, value: Self) -> Result<(), ni_fpga::Status> {
                let mut raw_data: [u8; std::mem::size_of::<Self>()] = Default::default();
                let slice_size = Self::packed_bytes();
                value.pack_to_slice(&mut raw_data[0..slice_size]).unwrap();
                session.write(offset, raw_data)
            }
        }
    };

    output.into()
}
