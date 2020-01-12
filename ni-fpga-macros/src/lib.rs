#![allow(unused_imports)]

#[macro_use] extern crate ni_fpga_derive;
pub use ni_fpga_derive::cluster;

#[macro_use] extern crate packed_struct_codegen;
pub use packed_struct_codegen::PackedStruct;

pub mod prelude {
    pub use packed_struct;
    pub use packed_struct::prelude::*;
    pub use packed_struct::PackedStructInfo;
}
