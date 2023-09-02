#![allow(non_snake_case)]

#[derive(Debug)]
pub struct DlOpenError(dlopen::Error);

impl fmt::Display for DlOpenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            dlopen::Error::NullCharacter(err) => f.write_fmt(format_args!(
                "Null character in library or function name found {}",
                err
            )),
            dlopen::Error::OpeningLibraryError(err) => {
                f.write_fmt(format_args!("Error opening library {}", err))
            }
            dlopen::Error::SymbolGettingError(err) => {
                f.write_fmt(format_args!("Error loading symbol {}", err))
            }
            dlopen::Error::NullSymbol => f.write_str("Null symbol was found"),
            dlopen::Error::AddrNotMatchingDll(err) => {
                f.write_fmt(format_args!("Address does not match dll {}", err))
            }
        }
    }
}

impl PartialEq for DlOpenError {
    fn eq(&self, other: &Self) -> bool {
        match &self.0 {
            dlopen::Error::NullCharacter(_) => matches!(other.0, dlopen::Error::NullCharacter(_)),
            dlopen::Error::OpeningLibraryError(_) => {
                matches!(other.0, dlopen::Error::OpeningLibraryError(_))
            }
            dlopen::Error::SymbolGettingError(_) => {
                matches!(other.0, dlopen::Error::SymbolGettingError(_))
            }
            dlopen::Error::NullSymbol => matches!(other.0, dlopen::Error::NullSymbol),
            dlopen::Error::AddrNotMatchingDll(_) => {
                matches!(other.0, dlopen::Error::AddrNotMatchingDll(_))
            }
        }
    }
}

pub type Offset = u32;
pub type Session = u32;
pub type Status = i32;

use std::{ffi::c_char, fmt};

use dlopen::wrapper::{Container, WrapperApi};

use dlopen_derive::WrapperApi;

pub type NiFpgaApiContainer = Container<NiFpgaApi>;

#[derive(WrapperApi)]
pub struct NiFpgaApi {
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

impl NiFpgaApi {
    pub fn load() -> Result<NiFpgaApiContainer, DlOpenError> {
        match unsafe { Container::load("NiFpga") } {
            Ok(api) => Ok(api),
            Err(err) => Err(DlOpenError(err)),
        }
    }
}
