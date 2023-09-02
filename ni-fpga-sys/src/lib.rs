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

use std::{
    ffi::{c_char, c_void},
    fmt,
};

use dlopen::{
    utils::platform_file_name,
    wrapper::{Container, WrapperApi, WrapperMultiApi},
};

use dlopen_derive::{WrapperApi, WrapperMultiApi};

pub type NiFpgaApiContainer = Container<NiFpgaApi>;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct OpenAttributes: u32 {
        const NoRun = 1;
        const BitfilePathIsUTF8 = 2;
        const BitfileContentsNotPath = 1u32 << 30;
        const IgnoreSignatureArgument = 1u32 << 31;
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct CloseAttributes: u32 {
        const NoResetIfLastSession = 1;
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RunAttribute: u32 {
        const WaitUntilDone = 1;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FpgaViState {
    NotRunning = 0,
    Invalid = 1,
    Running = 2,
    NaturallyStopped = 3,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Irq: u32 {
        const Irq0  = 1u32 << 0;
        const Irq1  = 1u32 << 1;
        const Irq2  = 1u32 << 2;
        const Irq3  = 1u32 << 3;
        const Irq4  = 1u32 << 4;
        const Irq5  = 1u32 << 5;
        const Irq6  = 1u32 << 6;
        const Irq7  = 1u32 << 7;
        const Irq8  = 1u32 << 8;
        const Irq9  = 1u32 << 9;
        const Irq10 = 1u32 << 10;
        const Irq11 = 1u32 << 11;
        const Irq12 = 1u32 << 12;
        const Irq13 = 1u32 << 13;
        const Irq14 = 1u32 << 14;
        const Irq15 = 1u32 << 15;
        const Irq16 = 1u32 << 16;
        const Irq17 = 1u32 << 17;
        const Irq18 = 1u32 << 18;
        const Irq19 = 1u32 << 19;
        const Irq20 = 1u32 << 20;
        const Irq21 = 1u32 << 21;
        const Irq22 = 1u32 << 22;
        const Irq23 = 1u32 << 23;
        const Irq24 = 1u32 << 24;
        const Irq25 = 1u32 << 25;
        const Irq26 = 1u32 << 26;
        const Irq27 = 1u32 << 27;
        const Irq28 = 1u32 << 28;
        const Irq29 = 1u32 << 29;
        const Irq30 = 1u32 << 30;
        const Irq31 = 1u32 << 31;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FifoProperty {
    BytesPerElement = 1,
    HostBufferAllocationGranularity = 2,
    HostBufferSize = 3,
    HostBufferMirrorSize = 4,
    HostBufferType = 5,
    HostBuffer = 6,
    FlowControl = 7,
    ElementsCurrentlyAcquired = 8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HostBufferType {
    AllocatedByRIO = 1,
    AllocatedByUser = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FifoFlowControl {
    Disabled = 1,
    Enabled = 2,
}

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
