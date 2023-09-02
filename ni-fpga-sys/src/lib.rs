#![allow(dead_code)]

use core::ffi::c_char;
use std::ffi::CString;

pub type Offset = u32;
pub type Session = u32;
pub type Status = i32;

trait StatusHelper {
    fn to_result(self) -> Result<(), Status>;
}

impl StatusHelper for Status {
    fn to_result(self) -> Result<(), Status> {
        if self == 0 {
            Ok(())
        } else {
            Err(self)
        }
    }
}

extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{Container, WrapperApi};

pub use dlopen::Error as dlopen_Error;

#[derive(WrapperApi)]
struct NiFpgaApi {
    NiFpgaDll_Open: extern "C" fn(
        bitfile: *const c_char,
        signature: *const c_char,
        resource: *const c_char,
        attribute: u32,
        session: *mut Session,
    ) -> Status,

    NiFpgaDll_Close: extern "C" fn(session: Session, attribute: u32) -> Status,
    NiFpgaDll_ReadBool:
        extern "C" fn(session: Session, indicator: Offset, value: *mut bool) -> Status,
    NiFpgaDll_ReadArrayBool:
        extern "C" fn(session: Session, indicator: Offset, array: *mut bool, size: usize) -> Status,
    NiFpgaDll_WriteBool: extern "C" fn(session: Session, indicator: Offset, value: bool) -> Status,
}

pub struct NiFpga {
    session: Session,
    api: Container<NiFpgaApi>,
}

pub enum OpenError {
    NiFpgaError(Status),
    DlOpenError(dlopen::Error),
}

impl NiFpga {
    pub fn open(
        bitfile: &CString,
        signature: &CString,
        resource: &CString,
        attribute: u32,
    ) -> Result<NiFpga, OpenError> {
        let api: Container<NiFpgaApi> = match unsafe { Container::load("nifpga") } {
            Ok(api) => api,
            Err(e) => return Err(OpenError::DlOpenError(e)),
        };

        Ok(Self { session: 0, api })
    }

    pub fn close(self, attribute: u32) -> Result<(), Status> {
        self.api
            .NiFpgaDll_Close(self.session, attribute)
            .to_result()
    }
}

impl Drop for NiFpga {
    fn drop(&mut self) {
        // TODO figure out what to do here with attribute
        // and the return value
        self.api.NiFpgaDll_Close(self.session, 0);
    }
}

// #[link(name = "NiFpga")]
// extern "C" {
//     #[link_name = "NiFpgaDll_Open"]
//     pub fn Open(
//         bitfile: *const c_char,
//         signature: *const c_char,
//         resource: *const c_char,
//         attribute: u32,
//         session: *mut Session,
//     ) -> Status;
//     #[link_name = "NiFpgaDll_Close"]
//     pub fn Close(session: Session, attribute: u32) -> Status;

//     #[link_name = "NiFpgaDll_ReadBool"]
//     pub fn ReadBool(session: Session, indicator: Offset, value: *mut bool) -> Status;
//     #[link_name = "NiFpgaDll_ReadArrayBool"]
//     pub fn ReadArrayBool(
//         session: Session,
//         indicator: Offset,
//         array: *mut bool,
//         size: usize,
//     ) -> Status;
//     #[link_name = "NiFpgaDll_WriteBool"]
//     pub fn WriteBool(session: Session, indicator: Offset, value: bool) -> Status;
//     #[link_name = "NiFpgaDll_WriteArrayBool"]
//     pub fn WriteArrayBool(
//         session: Session,
//         indicator: Offset,
//         array: *const bool,
//         size: usize,
//     ) -> Status;

//     #[link_name = "NiFpgaDll_ReadU8"]
//     pub fn ReadU8(session: Session, indicator: Offset, value: *mut u8) -> Status;
//     #[link_name = "NiFpgaDll_ReadArrayU8"]
//     pub fn ReadArrayU8(session: Session, indicator: Offset, array: *mut u8, size: usize) -> Status;
//     #[link_name = "NiFpgaDll_WriteU8"]
//     pub fn WriteU8(session: Session, indicator: Offset, value: u8) -> Status;
//     #[link_name = "NiFpgaDll_WriteArrayU8"]
//     pub fn WriteArrayU8(
//         session: Session,
//         indicator: Offset,
//         array: *const u8,
//         size: usize,
//     ) -> Status;

//     #[link_name = "NiFpgaDll_ReadU16"]
//     pub fn ReadU16(session: Session, indicator: Offset, value: *mut u16) -> Status;
//     #[link_name = "NiFpgaDll_ReadArrayU16"]
//     pub fn ReadArrayU16(
//         session: Session,
//         indicator: Offset,
//         array: *mut u16,
//         size: usize,
//     ) -> Status;
//     #[link_name = "NiFpgaDll_WriteU16"]
//     pub fn WriteU16(session: Session, indicator: Offset, value: u16) -> Status;
//     #[link_name = "NiFpgaDll_WriteArrayU16"]
//     pub fn WriteArrayU16(
//         session: Session,
//         indicator: Offset,
//         array: *const u16,
//         size: usize,
//     ) -> Status;

//     #[link_name = "NiFpgaDll_ReadU32"]
//     pub fn ReadU32(session: Session, indicator: Offset, value: *mut u32) -> Status;
//     #[link_name = "NiFpgaDll_ReadArrayU32"]
//     pub fn ReadArrayU32(
//         session: Session,
//         indicator: Offset,
//         array: *mut u32,
//         size: usize,
//     ) -> Status;
//     #[link_name = "NiFpgaDll_WriteU32"]
//     pub fn WriteU32(session: Session, indicator: Offset, value: u32) -> Status;
//     #[link_name = "NiFpgaDll_WriteArrayU32"]
//     pub fn WriteArrayU32(
//         session: Session,
//         indicator: Offset,
//         array: *const u32,
//         size: usize,
//     ) -> Status;

//     #[link_name = "NiFpgaDll_ReadU64"]
//     pub fn ReadU64(session: Session, indicator: Offset, value: *mut u64) -> Status;
//     #[link_name = "NiFpgaDll_ReadArrayU64"]
//     pub fn ReadArrayU64(
//         session: Session,
//         indicator: Offset,
//         array: *mut u64,
//         size: usize,
//     ) -> Status;
//     #[link_name = "NiFpgaDll_WriteU64"]
//     pub fn WriteU64(session: Session, indicator: Offset, value: u64) -> Status;
//     #[link_name = "NiFpgaDll_WriteArrayU64"]
//     pub fn WriteArrayU64(
//         session: Session,
//         indicator: Offset,
//         array: *const u64,
//         size: usize,
//     ) -> Status;

//     #[link_name = "NiFpgaDll_ReadI8"]
//     pub fn ReadI8(session: Session, indicator: Offset, value: *mut i8) -> Status;
//     #[link_name = "NiFpgaDll_ReadArrayI8"]
//     pub fn ReadArrayI8(session: Session, indicator: Offset, array: *mut i8, size: usize) -> Status;
//     #[link_name = "NiFpgaDll_WriteI8"]
//     pub fn WriteI8(session: Session, indicator: Offset, value: i8) -> Status;
//     #[link_name = "NiFpgaDll_WriteArrayI8"]
//     pub fn WriteArrayI8(
//         session: Session,
//         indicator: Offset,
//         array: *const i8,
//         size: usize,
//     ) -> Status;

//     #[link_name = "NiFpgaDll_ReadI16"]
//     pub fn ReadI16(session: Session, indicator: Offset, value: *mut i16) -> Status;
//     #[link_name = "NiFpgaDll_ReadArrayI16"]
//     pub fn ReadArrayI16(
//         session: Session,
//         indicator: Offset,
//         array: *mut i16,
//         size: usize,
//     ) -> Status;
//     #[link_name = "NiFpgaDll_WriteI16"]
//     pub fn WriteI16(session: Session, indicator: Offset, value: i16) -> Status;
//     #[link_name = "NiFpgaDll_WriteArrayI16"]
//     pub fn WriteArrayI16(
//         session: Session,
//         indicator: Offset,
//         array: *const i16,
//         size: usize,
//     ) -> Status;

//     #[link_name = "NiFpgaDll_ReadI32"]
//     pub fn ReadI32(session: Session, indicator: Offset, value: *mut i32) -> Status;
//     #[link_name = "NiFpgaDll_ReadArrayI32"]
//     pub fn ReadArrayI32(
//         session: Session,
//         indicator: Offset,
//         array: *mut i32,
//         size: usize,
//     ) -> Status;
//     #[link_name = "NiFpgaDll_WriteI32"]
//     pub fn WriteI32(session: Session, indicator: Offset, value: i32) -> Status;
//     #[link_name = "NiFpgaDll_WriteArrayI32"]
//     pub fn WriteArrayI32(
//         session: Session,
//         indicator: Offset,
//         array: *const i32,
//         size: usize,
//     ) -> Status;

//     #[link_name = "NiFpgaDll_ReadI64"]
//     pub fn ReadI64(session: Session, indicator: Offset, value: *mut i64) -> Status;
//     #[link_name = "NiFpgaDll_ReadArrayI64"]
//     pub fn ReadArrayI64(
//         session: Session,
//         indicator: Offset,
//         array: *mut i64,
//         size: usize,
//     ) -> Status;
//     #[link_name = "NiFpgaDll_WriteI64"]
//     pub fn WriteI64(session: Session, indicator: Offset, value: i64) -> Status;
//     #[link_name = "NiFpgaDll_WriteArrayI64"]
//     pub fn WriteArrayI64(
//         session: Session,
//         indicator: Offset,
//         array: *const i64,
//         size: usize,
//     ) -> Status;
// }
