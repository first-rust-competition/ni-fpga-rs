extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Cluster)]
pub fn derive_cluster(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);

    let field_names = input.fields.iter().map(|field| &field.ident);
    let field_types = input.fields.iter().map(|field| &field.ty).collect::<Vec<&syn::Type>>();
    let field_types_for_unmarshal1 = &field_types;
    let field_types_for_unmarshal2 = &field_types;
    let struct_name = &input.ident;

    let output = quote!{
        impl ni_fpga::Datatype for #struct_name {
            const FPGA_SIZE: usize = 0 #( + <#field_types as ni_fpga::Datatype>::FPGA_SIZE )*;
            fn read(
                session: &ni_fpga::Session,
                offset:ni_fpga:: Offset,
            ) -> Result<Self, ni_fpga::Status> {
                use ni_fpga::bitvec::prelude::*;
                let raw: [u8; (Self::FPGA_SIZE - 1)/8 + 1] = session.read(offset)?;
                let mut bits: &BitSlice<Msb0, u8> = BitSlice::from_slice(&raw[..]);
                Ok(Self{
                    #(
                        #field_names: {
                            let (field_bits, rest) = bits.split_at(
                                <#field_types_for_unmarshal1 as ni_fpga::Datatype>::FPGA_SIZE
                            );
                            bits = rest;
                            // TODO: use type-specific methods to convert field_bits to each field type
                            0
                        } ,
                    )*
                })
            }
            fn write(
                session: &ni_fpga::Session,
                offset: ni_fpga::Offset,
                value: Self,
            ) -> Result<(), ni_fpga::Status> {
                use ni_fpga::bitvec::prelude::*;
                unimplemented!();
            }
        }
    };

    output.into()
}
