extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Cluster)]
pub fn derive_cluster(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);

    let field_names = input.fields.iter().map(|field| &field.ident);
    let field_types = input.fields.iter().map(|field| &field.ty).collect::<Vec<&syn::Type>>();
    let field_types_for_unmarshal1 = &field_types;
    let struct_name = &input.ident;

    let output = quote!{
        impl ni_fpga::Datatype for #struct_name {
            const SIZE_IN_BITS: usize = 0 #( + <#field_types as ni_fpga::Datatype>::SIZE_IN_BITS )*;

            fn pack(fpga_bits: &mut ni_fpga::FpgaBits, data: Self) {
                unimplemented!();
            }

            fn unpack(fpga_bits: &ni_fpga::FpgaBits) -> Self {
                let mut remaining_bits = fpga_bits;
                Self{
                    #(
                        #field_names: {
                            let (field_bits, other_remaining_bits) = remaining_bits.split_at(
                                <#field_types_for_unmarshal1 as ni_fpga::Datatype>::SIZE_IN_BITS
                            );
                            remaining_bits = other_remaining_bits;
                            ni_fpga::Datatype::unpack(field_bits)
                        } ,
                    )*
                }
            }
        }
    };

    output.into()
}
