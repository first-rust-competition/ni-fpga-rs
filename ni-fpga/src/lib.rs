#![feature(const_generics)]
#![feature(maybe_uninit_uninit_array)]

pub extern crate ni_fpga_sys as ffi;

mod datatype;
mod session;
mod status;

pub use datatype::{Datatype, FpgaBits};
pub type Offset = ffi::Offset;
pub use session::Session;
pub use status::Status;

// Included for use in ni-fpga-macros
pub use bitvec;
