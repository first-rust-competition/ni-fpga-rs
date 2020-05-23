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
        impl ni_fpga::Datatype for #struct_name {
            const SIZE_IN_BITS: usize = 0 #( + <#field_types as ni_fpga::Datatype>::SIZE_IN_BITS )*;

            fn pack(fpga_bits: &mut ni_fpga::FpgaBits, data: &Self) {
                let mut remaining_bits = fpga_bits;
                #({
                    let (field_bits, other_remaining_bits) = remaining_bits.split_at_mut(
                        <#field_types_for_pack as ni_fpga::Datatype>::SIZE_IN_BITS
                    );
                    remaining_bits = other_remaining_bits;
                    ni_fpga::Datatype::pack(field_bits, &data.#field_names_for_pack);
                })*
            }

            #[allow(clippy::eval_order_dependence)]
            fn unpack(fpga_bits: &ni_fpga::FpgaBits) -> Self {
                let mut remaining_bits = fpga_bits;
                Self{#(
                    #field_names_for_unpack: {
                        let (field_bits, other_remaining_bits) = remaining_bits.split_at(
                            <#field_types_for_unpack as ni_fpga::Datatype>::SIZE_IN_BITS
                        );
                        remaining_bits = other_remaining_bits;
                        ni_fpga::Datatype::unpack(field_bits)
                    }
                ) , *}
            }
        }
    };

    output.into()
}
