#![allow(clippy::missing_safety_doc)]

extern crate ni_fpga_sys as ffi;

mod datatype;
mod errors;
pub mod fxp;
mod hmb;
mod interrupt_manager;
mod nifpga;
mod register;
mod session;
pub mod session_lifetimes;
mod status;

pub use interrupt_manager::InterruptContext;
pub use interrupt_manager::InterruptWaiter;

// Keep these for backwards compatibility, but don't use them internally
pub use datatype::{Datatype, DatatypePacker, FpgaBits};
pub use errors::Error;
pub type Offset = ffi::Offset;
pub use datatype::DerivedDatatype;
pub use datatype::StockAccessDatatype;
pub use hmb::Hmb;
pub use hmb::HmbAccess;
pub use nifpga::NiFpga;
pub use register::ConstOffset;
pub use register::Register;
pub use register::RegisterRead;
pub use register::RegisterWrite;
pub use register::StoredOffset;
pub use session::Session;
pub use session::SessionAccess;
pub use session::SessionBuilder;
pub use status::Status;

pub use register::ReadOnly;
pub use register::ReadWrite;
pub use register::WriteOnly;

pub use hmb::HmbDefinition;
