#![feature(const_generics)]

extern crate ni_fpga_sys as ffi;

mod datatype;
mod session;
mod status;

pub use datatype::Datatype;
pub type Offset = ffi::Offset;
pub use session::Session;
pub use status::Status;

// Included for use in ni-fpga-macros
pub use bitvec;
