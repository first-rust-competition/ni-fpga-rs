mod ffi;
use std::{ffi::CString, fmt};

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

use dlopen::wrapper::Container;
use ffi::NiFpgaApi;

pub type Offset = u32;
pub type Session = u32;
pub type Status = i32;

trait StatusHelper {
    fn to_result(self) -> Result<(), Status>;
}

impl StatusHelper for Status {
    fn to_result(self) -> Result<(), Status> {
        match self {
            0 => Ok(()),
            _ => Err(self),
        }
    }
}

pub struct NiFpga {
    session: Session,
    api: Container<NiFpgaApi>,
}

pub enum OpenError {
    NiFpgaError(Status),
    DlOpenError(DlOpenError),
}

macro_rules! type_wrapper {
    ($type:ident, $read_fun_name:ident, $read_ffi_name:ident, $write_fun_name:ident, $write_ffi_name:ident,
        $readarr_fun_name:ident, $readarr_ffi_name:ident, $writearr_fun_name:ident, $writearr_ffi_name:ident) => {
        pub fn $read_fun_name(&self, indicator: Offset, value: &mut $type) -> Result<(), Status> {
            self.api
                .$read_ffi_name(self.session, indicator, value as *mut $type)
                .to_result()
        }

        pub fn $write_fun_name(&self, indicator: Offset, value: $type) -> Result<(), Status> {
            self.api
                .$write_ffi_name(self.session, indicator, value)
                .to_result()
        }

        pub fn $readarr_fun_name(
            &self,
            indicator: Offset,
            value: &mut [$type],
        ) -> Result<(), Status> {
            self.api
                .$readarr_ffi_name(self.session, indicator, value.as_mut_ptr(), value.len())
                .to_result()
        }

        pub fn $writearr_fun_name(&self, indicator: Offset, value: &[$type]) -> Result<(), Status> {
            self.api
                .$writearr_ffi_name(self.session, indicator, value.as_ptr(), value.len())
                .to_result()
        }
    };
}

impl NiFpga {
    type_wrapper!(
        bool,
        read_bool,
        NiFpgaDll_ReadBool,
        write_bool,
        NiFpgaDll_WriteBool,
        read_bool_array,
        NiFpgaDll_ReadArrayBool,
        write_bool_array,
        NiFpgaDll_WriteArrayBool
    );
    type_wrapper!(
        u8,
        read_u8,
        NiFpgaDll_ReadU8,
        write_u8,
        NiFpgaDll_WriteU8,
        read_u8_array,
        NiFpgaDll_ReadArrayU8,
        write_u8_array,
        NiFpgaDll_WriteArrayU8
    );
    type_wrapper!(
        i8,
        read_i8,
        NiFpgaDll_ReadI8,
        write_i8,
        NiFpgaDll_WriteI8,
        read_i8_array,
        NiFpgaDll_ReadArrayI8,
        write_i8_array,
        NiFpgaDll_WriteArrayI8
    );
    type_wrapper!(
        u16,
        read_u16,
        NiFpgaDll_ReadU16,
        write_u16,
        NiFpgaDll_WriteU16,
        read_u16_array,
        NiFpgaDll_ReadArrayU16,
        write_u16_array,
        NiFpgaDll_WriteArrayU16
    );
    type_wrapper!(
        i16,
        read_i16,
        NiFpgaDll_ReadI16,
        write_i16,
        NiFpgaDll_WriteI16,
        read_i16_array,
        NiFpgaDll_ReadArrayI16,
        write_i16_array,
        NiFpgaDll_WriteArrayI16
    );
    type_wrapper!(
        u32,
        read_u32,
        NiFpgaDll_ReadU32,
        write_u32,
        NiFpgaDll_WriteU32,
        read_u32_array,
        NiFpgaDll_ReadArrayU32,
        write_u32_array,
        NiFpgaDll_WriteArrayU32
    );
    type_wrapper!(
        i32,
        read_i32,
        NiFpgaDll_ReadI32,
        write_i32,
        NiFpgaDll_WriteI32,
        read_i32_array,
        NiFpgaDll_ReadArrayI32,
        write_i32_array,
        NiFpgaDll_WriteArrayI32
    );
    type_wrapper!(
        u64,
        read_u64,
        NiFpgaDll_ReadU64,
        write_u64,
        NiFpgaDll_WriteU64,
        read_u64_array,
        NiFpgaDll_ReadArrayU64,
        write_u64_array,
        NiFpgaDll_WriteArrayU64
    );
    type_wrapper!(
        i64,
        read_i64,
        NiFpgaDll_ReadI64,
        write_i64,
        NiFpgaDll_WriteI64,
        read_i64_array,
        NiFpgaDll_ReadArrayI64,
        write_i64_array,
        NiFpgaDll_WriteArrayI64
    );

    pub fn open(
        bitfile: &CString,
        signature: &CString,
        resource: &CString,
        attribute: u32,
    ) -> Result<Self, OpenError> {
        let api: Container<NiFpgaApi> = match unsafe { Container::load("NiFpga") } {
            Ok(api) => api,
            Err(err) => return Err(OpenError::DlOpenError(DlOpenError(err))),
        };

        let mut session: u32 = 0;
        match api
            .NiFpgaDll_Open(
                bitfile.as_ptr(),
                signature.as_ptr(),
                resource.as_ptr(),
                attribute,
                &mut session as *mut Session,
            )
            .to_result()
        {
            Ok(_) => Ok(Self { session, api }),
            Err(err) => Err(OpenError::NiFpgaError(err)),
        }
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
