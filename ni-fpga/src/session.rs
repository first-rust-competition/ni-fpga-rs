use std::ffi::CString;

use crate::datatype::{Datatype, FpgaBitsRaw};
use crate::errors::Error;
use crate::ffi;
use crate::ffi::Offset;
use crate::status::Status;

pub struct Session {
    pub handle: ffi::Session,
    api: ffi::NiFpgaApiContainer,
}

impl From<ffi::Error> for Error {
    fn from(value: ffi::Error) -> Self {
        match value {
            // Map the explicit opening errors to what the C API
            // returns for the same errors.
            ffi::Error::OpeningLibraryError(_) => Error::FPGA(Status::ResourceNotFound),
            ffi::Error::SymbolGettingError(_) => Error::FPGA(Status::VersionMismatch),
            // Map unknowns (Which are impossible to hit)
            // just as a generic error. All 3 other enum states
            // for this are impossible with the FPGA library.
            _ => Error::FPGA(Status::ResourceNotFound),
        }
    }
}

impl Session {
    pub fn open(bitfile: &str, signature: &str, resource: &str) -> Result<Self, Error> {
        let mut handle: ffi::Session = Default::default();
        let c_bitfile = CString::new(bitfile).unwrap();
        let c_signature = CString::new(signature).unwrap();
        let c_resource = CString::new(resource).unwrap();
        let api = ffi::NiFpgaApi::load()?;

        let status = Status::from(unsafe {
            api.base.NiFpgaDll_Open(
                c_bitfile.as_ptr(),
                c_signature.as_ptr(),
                c_resource.as_ptr(),
                0,
                &mut handle as *mut ffi::Session,
            )
        });
        match status {
            Status::Success => Ok(Session { api, handle }),
            _ => Err(Error::FPGA(status)),
        }
    }
    pub fn read<T: Datatype>(&self, offset: Offset) -> Result<T, Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        let byte_size = (T::SIZE_IN_BITS - 1) / 8 + 1;
        let mut buffer = [0u8; (T::SIZE_IN_BITS - 1) / 8 + 1];
        let status = unsafe {
            self.api
                .base
                .NiFpgaDll_ReadArrayU8(self.handle, offset, buffer.as_mut_ptr(), buffer.len())
                .into()
        };
        if status != Status::Success {
            return Err(Error::FPGA(status));
        }
        // Values larger then a single element (32 bit) are left justified, not right
        let bit_slice = FpgaBitsRaw::from_slice_mut(&mut buffer);
        let bit_slice = if byte_size <= 4 {
            bit_slice.split_at_mut(byte_size * 8 - T::SIZE_IN_BITS).1
        } else {
            bit_slice.split_at_mut(T::SIZE_IN_BITS).0
        };

        Datatype::unpack(bit_slice)
    }
    pub fn write<T: Datatype>(&self, offset: Offset, data: &T) -> Result<(), Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        let byte_size = (T::SIZE_IN_BITS - 1) / 8 + 1;
        let mut buffer = [0u8; (T::SIZE_IN_BITS - 1) / 8 + 1];
        // Values larger then a single element (32 bit) are left justified, not right
        let bit_slice = FpgaBitsRaw::from_slice_mut(&mut buffer);
        let bit_slice = if byte_size <= 4 {
            bit_slice.split_at_mut(byte_size * 8 - T::SIZE_IN_BITS).1
        } else {
            bit_slice.split_at_mut(T::SIZE_IN_BITS).0
        };
        Datatype::pack(bit_slice, data)?;
        let status = unsafe {
            self.api
                .base
                .NiFpgaDll_WriteArrayU8(
                    self.handle,
                    offset,
                    buffer.as_ptr(),
                    (T::SIZE_IN_BITS - 1) / 8 + 1,
                )
                .into()
        };
        match status {
            Status::Success => Ok(()),
            _ => Err(Error::FPGA(status)),
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { self.api.base.NiFpgaDll_Close(self.handle, 0) };
    }
}
