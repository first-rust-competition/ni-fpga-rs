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
            const FFI_READ: ni_fpga::ffi::Func<*mut Self> = |session, offset, target| unsafe {
                let mut uninit_raw_target = std::mem::MaybeUninit::<[u8; std::mem::size_of::<Self>()]>::uninit();
                let status = ni_fpga::ffi::ReadArrayU8(
                    session,
                    offset,
                    uninit_raw_target.as_mut_ptr() as *mut u8,
                    std::mem::size_of::<Self>(),
                );
                if status != ni_fpga::Status::Success.into() {
                    return status;
                }
                let raw_target = uninit_raw_target.assume_init();
                let slice_size_needed: usize = (Self::packed_bits() - 1) / 8 + 1;
                *target = Self::unpack_from_slice(&raw_target[0..slice_size_needed]).unwrap();
                ni_fpga::Status::Success.into()
            };
            const FFI_WRITE: ni_fpga::ffi::Func<Self> = |session, offset, value| unsafe {
                ni_fpga::Status::Success.into()
            };
        }
    };

    output.into()
}
