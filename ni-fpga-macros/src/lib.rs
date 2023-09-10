extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Cluster)]
pub fn derive_cluster(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);

    let field_names: Vec<_> = input.fields.iter().map(|field| &field.ident).collect();
    let field_names_for_pack = &field_names;
    let field_names_for_unpack = &field_names;
    let field_types: Vec<_> = input.fields.iter().map(|field| &field.ty).collect();
    let field_types_for_pack = &field_types;
    let field_types_for_unpack = &field_types;
    let struct_name = &input.ident;

    let output = quote! {
        impl ni_fpga::DatatypePacker for #struct_name {
            const SIZE_IN_BITS: usize = 0 #( + <#field_types as ni_fpga::DatatypePacker>::SIZE_IN_BITS )*;

            fn pack(fpga_bits: &mut ni_fpga::FpgaBits, data: &Self) -> Result<(), ni_fpga::Error> {
                let mut remaining_bits = fpga_bits;
                #(
                {
                    let (field_bits, other_remaining_bits) = remaining_bits.split_at_mut(
                        <#field_types_for_pack as ni_fpga::DatatypePacker>::SIZE_IN_BITS
                    );
                    remaining_bits = other_remaining_bits;
                    ni_fpga::DatatypePacker::pack(field_bits, &data.#field_names_for_pack)?;
                }
                )*
                Ok(())
            }

            #[allow(clippy::eval_order_dependence)]
            fn unpack(fpga_bits: &ni_fpga::FpgaBits) -> Result<Self, ni_fpga::Error> {
                let mut remaining_bits = fpga_bits;
                Ok(
                    Self{
                        #(
                        #field_names_for_unpack: {
                            let (field_bits, other_remaining_bits) = remaining_bits.split_at(
                                <#field_types_for_unpack as ni_fpga::DatatypePacker>::SIZE_IN_BITS
                            );
                            remaining_bits = other_remaining_bits;
                            ni_fpga::DatatypePacker::unpack(field_bits)?
                        }
                        ),*
                    },
                )
            }
        }
        impl ni_fpga::DerivedDatatype for #struct_name {}
        impl ni_fpga::StockAccessDatatype for #struct_name {}

        impl ni_fpga::Datatype for #struct_name
        {
            unsafe fn read(session: &impl ni_fpga::SessionAccess, offset: ni_fpga::Offset) -> Result<Self, ni_fpga::Error> {
                <Self as ni_fpga::StockAccessDatatype>::read(session, offset)
            }

            unsafe fn write(
                session: &impl ni_fpga::SessionAccess,
                offset: ni_fpga::Offset,
                value: impl std::borrow::Borrow<Self>,
            ) -> Result<(), ni_fpga::Error> {
                <Self as ni_fpga::StockAccessDatatype>::write(session, offset, value)
            }
        }
    };

    output.into()
}

#[proc_macro_derive(Enum)]
pub fn derive_enum(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemEnum);

    let backing_size = match (input.variants.len() as f64).log2().ceil().exp2() as usize {
        x if x < 8 => 8,
        x if x > 64 => return quote!{compile_error!("Enum can only be derived for enums with at most 2^64 variants!")}.into(),
        x => x,
    };
    let backing_type = syn::Type::Verbatim(match backing_size {
        8 => quote! {u8},
        16 => quote! {u16},
        32 => quote! {u32},
        64 => quote! {u64},
        _ => unreachable!(),
    });

    let discriminants: Vec<_> = (0..input.variants.len())
        .map(|discriminant| {
            syn::LitInt::new(
                &discriminant.to_string(),
                proc_macro::Span::call_site().into(),
            )
        })
        .collect();
    let discriminants_for_pack = &discriminants;
    let discriminants_for_unpack = &discriminants;
    let variants: Vec<_> = input.variants.iter().map(|v| &v.ident).collect();
    let variants_for_pack = &variants;
    let variants_for_unpack = &variants;
    let enum_name = &input.ident;

    let output = quote! {
        impl ni_fpga::DatatypePacker for #enum_name {
            const SIZE_IN_BITS: usize = #backing_size;

            fn pack(fpga_bits: &mut ni_fpga::FpgaBits, data: &Self) -> Result<(), ni_fpga::Error> {
                match data {
                    #(
                    Self::#variants_for_pack => <#backing_type as ni_fpga::DatatypePacker>::pack(fpga_bits, &#discriminants_for_pack)?
                    ),*
                };
                Ok(())
            }

            fn unpack(fpga_bits: &ni_fpga::FpgaBits) -> Result<Self, ni_fpga::Error> {
                match <#backing_type as ni_fpga::DatatypePacker>::unpack(fpga_bits)? {
                    #(
                    #discriminants_for_unpack => Ok(Self::#variants_for_unpack)
                    ),*,
                    unknown => Err(ni_fpga::Error::InvalidEnumDiscriminant(unknown as u64)),
                }
            }
        }

        impl #enum_name {
            pub fn from_backing(value: #backing_type) -> Result<Self, ni_fpga::Error> {
                match value {
                    #(
                    #discriminants_for_unpack => Ok(Self::#variants_for_unpack)
                    ),*,
                    unknown => Err(ni_fpga::Error::InvalidEnumDiscriminant(unknown as u64)),
                }
            }

            fn into_backing(&self) -> #backing_type {
                match self {
                    #(
                    #enum_name::#variants_for_pack => #discriminants_for_pack
                    ),*
                }
            }
        }

        impl ni_fpga::Datatype for #enum_name {
            unsafe fn read(session: &impl ni_fpga::SessionAccess, offset: ni_fpga::Offset) -> Result<Self, ni_fpga::Error> {
                Self::from_backing(ni_fpga::Datatype::read(session, offset)?)
            }

            unsafe fn write(session: &impl ni_fpga::SessionAccess, offset: ni_fpga::Offset, value: impl std::borrow::Borrow<Self>) -> Result<(), ni_fpga::Error> {
                <#backing_type as ni_fpga::Datatype>::write(session, offset, value.borrow().into_backing())
            }
        }

        impl ni_fpga::DerivedDatatype for #enum_name {}
    };

    output.into()
}
