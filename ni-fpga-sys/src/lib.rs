#![allow(non_snake_case)]

pub type Offset = u32;
pub type Session = u32;
pub type Status = i32;

mod dlopenerror;
mod enums;

pub use dlopenerror::DlOpenError;
pub use enums::*;

use std::ffi::{c_char, c_void};

use dlopen::{
    utils::platform_file_name,
    wrapper::{Container, WrapperApi, WrapperMultiApi},
};

use dlopen_derive::{WrapperApi, WrapperMultiApi};

pub type NiFpgaApiContainer = Container<NiFpgaApi>;

#[derive(WrapperApi)]
pub struct NiFpgaHmbApi {
    NiFpgaDll_OpenHmb: extern "C" fn(
        session: Session,
        memory_name: *const c_char,
        memory_size: *mut usize,
        virual_address: *mut *mut c_void,
    ) -> Status,
    NiFpgaDll_CloseHmb: extern "C" fn(session: Session, memory_name: *const c_char) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaBaseApi {
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
    NiFpgaDll_WriteArrayBool: extern "C" fn(
        session: Session,
        indicator: Offset,
        array: *const bool,
        size: usize,
    ) -> Status,

    NiFpgaDll_ReadU8: extern "C" fn(session: Session, indicator: Offset, value: *mut u8) -> Status,
    NiFpgaDll_ReadArrayU8:
        extern "C" fn(session: Session, indicator: Offset, array: *mut u8, size: usize) -> Status,
    NiFpgaDll_WriteU8: extern "C" fn(session: Session, indicator: Offset, value: u8) -> Status,
    NiFpgaDll_WriteArrayU8:
        extern "C" fn(session: Session, indicator: Offset, array: *const u8, size: usize) -> Status,

    NiFpgaDll_ReadU16:
        extern "C" fn(session: Session, indicator: Offset, value: *mut u16) -> Status,
    NiFpgaDll_ReadArrayU16:
        extern "C" fn(session: Session, indicator: Offset, array: *mut u16, size: usize) -> Status,
    NiFpgaDll_WriteU16: extern "C" fn(session: Session, indicator: Offset, value: u16) -> Status,
    NiFpgaDll_WriteArrayU16: extern "C" fn(
        session: Session,
        indicator: Offset,
        array: *const u16,
        size: usize,
    ) -> Status,

    NiFpgaDll_ReadU32:
        extern "C" fn(session: Session, indicator: Offset, value: *mut u32) -> Status,
    NiFpgaDll_ReadArrayU32:
        extern "C" fn(session: Session, indicator: Offset, array: *mut u32, size: usize) -> Status,
    NiFpgaDll_WriteU32: extern "C" fn(session: Session, indicator: Offset, value: u32) -> Status,
    NiFpgaDll_WriteArrayU32: extern "C" fn(
        session: Session,
        indicator: Offset,
        array: *const u32,
        size: usize,
    ) -> Status,

    NiFpgaDll_ReadU64:
        extern "C" fn(session: Session, indicator: Offset, value: *mut u64) -> Status,
    NiFpgaDll_ReadArrayU64:
        extern "C" fn(session: Session, indicator: Offset, array: *mut u64, size: usize) -> Status,
    NiFpgaDll_WriteU64: extern "C" fn(session: Session, indicator: Offset, value: u64) -> Status,
    NiFpgaDll_WriteArrayU64: extern "C" fn(
        session: Session,
        indicator: Offset,
        array: *const u64,
        size: usize,
    ) -> Status,

    NiFpgaDll_ReadI8: extern "C" fn(session: Session, indicator: Offset, value: *mut i8) -> Status,
    NiFpgaDll_ReadArrayI8:
        extern "C" fn(session: Session, indicator: Offset, array: *mut i8, size: usize) -> Status,
    NiFpgaDll_WriteI8: extern "C" fn(session: Session, indicator: Offset, value: i8) -> Status,
    NiFpgaDll_WriteArrayI8:
        extern "C" fn(session: Session, indicator: Offset, array: *const i8, size: usize) -> Status,

    NiFpgaDll_ReadI16:
        extern "C" fn(session: Session, indicator: Offset, value: *mut i16) -> Status,
    NiFpgaDll_ReadArrayI16:
        extern "C" fn(session: Session, indicator: Offset, array: *mut i16, size: usize) -> Status,
    NiFpgaDll_WriteI16: extern "C" fn(session: Session, indicator: Offset, value: i16) -> Status,
    NiFpgaDll_WriteArrayI16: extern "C" fn(
        session: Session,
        indicator: Offset,
        array: *const i16,
        size: usize,
    ) -> Status,

    NiFpgaDll_ReadI32:
        extern "C" fn(session: Session, indicator: Offset, value: *mut i32) -> Status,
    NiFpgaDll_ReadArrayI32:
        extern "C" fn(session: Session, indicator: Offset, array: *mut i32, size: usize) -> Status,
    NiFpgaDll_WriteI32: extern "C" fn(session: Session, indicator: Offset, value: i32) -> Status,
    NiFpgaDll_WriteArrayI32: extern "C" fn(
        session: Session,
        indicator: Offset,
        array: *const i32,
        size: usize,
    ) -> Status,

    NiFpgaDll_ReadI64:
        extern "C" fn(session: Session, indicator: Offset, value: *mut i64) -> Status,
    NiFpgaDll_ReadArrayI64:
        extern "C" fn(session: Session, indicator: Offset, array: *mut i64, size: usize) -> Status,
    NiFpgaDll_WriteI64: extern "C" fn(session: Session, indicator: Offset, value: i64) -> Status,
    NiFpgaDll_WriteArrayI64: extern "C" fn(
        session: Session,
        indicator: Offset,
        array: *const i64,
        size: usize,
    ) -> Status,
}

#[derive(WrapperMultiApi)]
pub struct NiFpgaApi {
    pub base: NiFpgaBaseApi,
    pub hmb: Option<NiFpgaHmbApi>,
}

impl NiFpgaApi {
    pub fn load() -> Result<NiFpgaApiContainer, DlOpenError> {
        match unsafe { Container::load(platform_file_name("NiFpga")) } {
            Ok(api) => Ok(api),
            Err(err) => Err(DlOpenError(err)),
        }
    }
}
