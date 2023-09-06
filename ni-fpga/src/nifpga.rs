use std::ffi::CString;

use ni_fpga_sys::{
    Bool, CloseAttribute, DlOpenError, NiFpgaApi, NiFpgaApiContainer, Offset, OpenAttribute,
    Session,
};

use crate::Error;

pub(crate) trait StatusHelper {
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

impl From<DlOpenError> for Error {
    fn from(value: DlOpenError) -> Self {
        Self::DlOpen(value)
    }
}

pub struct NiFpga {
    pub(crate) session: Session,
    pub(crate) api: NiFpgaApiContainer,
    close_attribute: Option<CloseAttribute>,
}

macro_rules! type_wrapper {
    ($type:ident, $read_fun_name:ident, $read_ffi_name:ident, $write_fun_name:ident, $write_ffi_name:ident,
        $readarr_fun_name:ident, $readarr_ffi_name:ident, $writearr_fun_name:ident, $writearr_ffi_name:ident) => {
        pub fn $read_fun_name(&self, indicator: Offset) -> Result<$type, Error> {
            let mut value: $type = Default::default();
            match self
                .api
                .base
                .$read_ffi_name(self.session, indicator, &mut value as *mut $type)
                .to_result()
            {
                Ok(_) => Ok(value),
                Err(err) => Err(err),
            }
        }

        pub fn $write_fun_name(&self, indicator: Offset, value: $type) -> Result<(), Error> {
            self.api
                .base
                .$write_ffi_name(self.session, indicator, value)
                .to_result()
        }

        pub fn $readarr_fun_name(
            &self,
            indicator: Offset,
            value: &mut [$type],
        ) -> Result<(), Error> {
            self.api
                .base
                .$readarr_ffi_name(self.session, indicator, value.as_mut_ptr(), value.len())
                .to_result()
        }

        pub fn $writearr_fun_name(&self, indicator: Offset, value: &[$type]) -> Result<(), Error> {
            self.api
                .base
                .$writearr_ffi_name(self.session, indicator, value.as_ptr(), value.len())
                .to_result()
        }
    };
}

impl NiFpga {
    pub fn ffi(&self) -> &NiFpgaApiContainer {
        &self.api
    }

    pub fn session(&self) -> Session {
        self.session
    }

    pub fn read_bool(&self, indicator: Offset) -> Result<bool, Error> {
        let mut value: u8 = 0;
        match self
            .api
            .base
            .NiFpgaDll_ReadBool(self.session, indicator, &mut value as *mut Bool)
            .to_result()
        {
            Ok(_) => Ok(value != 0),
            Err(err) => Err(err),
        }
    }
    pub fn write_bool(&self, indicator: Offset, value: bool) -> Result<(), Error> {
        let value = if value { 1 } else { 0 };
        self.api
            .base
            .NiFpgaDll_WriteBool(self.session, indicator, value)
            .to_result()
    }
    pub fn read_bool_array_fast(&self, indicator: Offset, value: &mut [u8]) -> Result<(), Error> {
        self.api
            .base
            .NiFpgaDll_ReadArrayBool(self.session, indicator, value.as_mut_ptr(), value.len())
            .to_result()
    }
    pub fn write_bool_array_fast(&self, indicator: Offset, value: &[u8]) -> Result<(), Error> {
        self.api
            .base
            .NiFpgaDll_WriteArrayBool(self.session, indicator, value.as_ptr(), value.len())
            .to_result()
    }

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

    type_wrapper!(
        f32,
        read_f32,
        NiFpgaDll_ReadSgl,
        write_f32,
        NiFpgaDll_WriteSgl,
        read_f32_array,
        NiFpgaDll_ReadArraySgl,
        write_f32_array,
        NiFpgaDll_WriteArraySgl
    );

    type_wrapper!(
        f64,
        read_f64,
        NiFpgaDll_ReadDbl,
        write_f64,
        NiFpgaDll_WriteDbl,
        read_f64_array,
        NiFpgaDll_ReadArrayDbl,
        write_f64_array,
        NiFpgaDll_WriteArrayDbl
    );

    pub fn from_session(session: Session) -> Result<Self, Error> {
        let api = NiFpgaApi::load()?;

        Ok(Self {
            session,
            api,
            close_attribute: None,
        })
    }

    pub fn open(
        bitfile: &CString,
        signature: &CString,
        resource: &CString,
        open_attribute: OpenAttribute,
        close_attribute: CloseAttribute,
    ) -> Result<Self, Error> {
        let api = NiFpgaApi::load()?;

        let mut session: Session = 0;
        match api
            .base
            .NiFpgaDll_Open(
                bitfile.as_ptr(),
                signature.as_ptr(),
                resource.as_ptr(),
                open_attribute.bits(),
                &mut session,
            )
            .to_result()
        {
            Ok(_) => Ok(Self {
                session,
                api,
                close_attribute: Some(close_attribute),
            }),
            Err(err) => Err(err),
        }
    }

    pub fn close(self, attribute: CloseAttribute) -> Result<(), Error> {
        match self.close_attribute {
            Some(_) => self
                .api
                .base
                .NiFpgaDll_Close(self.session, attribute.bits())
                .to_result(),
            None => Err(Error::ClosingUnownedSession),
        }
    }
}

impl Drop for NiFpga {
    fn drop(&mut self) {
        // TODO figure out what to do here with attribute
        // and the return value
        if let Some(attr) = self.close_attribute {
            self.api.base.NiFpgaDll_Close(self.session, attr.bits());
        }
    }
}
