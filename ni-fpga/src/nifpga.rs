use std::ffi::CString;

use ni_fpga_sys::{NiFpgaApi, NiFpgaApiContainer, Offset, Session};

use crate::Error;

trait StatusHelper {
    fn to_result(self) -> Result<(), Error>;
}

impl StatusHelper for ffi::Status {
    fn to_result(self) -> Result<(), Error> {
        match self {
            0 => Ok(()),
            _ => Err(Error::FPGA(self.into())),
        }
    }
}

pub struct NiFpga {
    session: Session,
    api: NiFpgaApiContainer,
}

macro_rules! type_wrapper {
    ($type:ident, $read_fun_name:ident, $read_ffi_name:ident, $write_fun_name:ident, $write_ffi_name:ident,
        $readarr_fun_name:ident, $readarr_ffi_name:ident, $writearr_fun_name:ident, $writearr_ffi_name:ident) => {
        pub fn $read_fun_name(&self, indicator: Offset, value: &mut $type) -> Result<(), Error> {
            self.api
                .$read_ffi_name(self.session, indicator, value as *mut $type)
                .to_result()
        }

        pub fn $write_fun_name(&self, indicator: Offset, value: $type) -> Result<(), Error> {
            self.api
                .$write_ffi_name(self.session, indicator, value)
                .to_result()
        }

        pub fn $readarr_fun_name(
            &self,
            indicator: Offset,
            value: &mut [$type],
        ) -> Result<(), Error> {
            self.api
                .$readarr_ffi_name(self.session, indicator, value.as_mut_ptr(), value.len())
                .to_result()
        }

        pub fn $writearr_fun_name(&self, indicator: Offset, value: &[$type]) -> Result<(), Error> {
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
    ) -> Result<Self, Error> {
        let api = match NiFpgaApi::load() {
            Ok(api) => api,
            Err(err) => return Err(Error::DlOpen(err)),
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
            Err(err) => Err(err),
        }
    }

    pub fn close(self, attribute: u32) -> Result<(), Error> {
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
