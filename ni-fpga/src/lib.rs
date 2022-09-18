#![feature(const_evaluatable_checked)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_uninit_array)]

extern crate ni_fpga_sys as ffi;

mod datatype;
mod errors;
pub mod fxp;
mod session;
mod status;

// Keep these for backwards compatibility, but don't use them internally
pub use datatype::{Datatype, FpgaBits};
pub use errors::Error;
pub type Offset = ffi::Offset;
pub use session::Session;
pub use status::Status;
