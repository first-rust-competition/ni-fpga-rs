use std::ffi::CString;

use crate::datatype::{Datatype, FpgaBits};
use crate::errors::Error;
use crate::ffi;
use crate::ffi::Offset;
use crate::status::Status;

pub struct Session {
    pub handle: ffi::Session,
    api: ffi::NiFpgaApiContainer,
}

impl Session {
    pub fn open(bitfile: &str, signature: &str, resource: &str) -> Result<Self, Error> {
        let mut handle: ffi::Session = Default::default();
        let c_bitfile = CString::new(bitfile).unwrap();
        let c_signature = CString::new(signature).unwrap();
        let c_resource = CString::new(resource).unwrap();
        let api = match ffi::NiFpgaApi::load() {
            Ok(api) => api,
            Err(err) => match err {
                // Map the explicit opening errors to what the C API
                // returns for the same errors.
                ffi::Error::OpeningLibraryError(_) => {
                    return Err(Error::FPGA(Status::ResourceNotFound))
                }
                ffi::Error::SymbolGettingError(_) => {
                    return Err(Error::FPGA(Status::VersionMismatch))
                }
                // Map unknowns (Which are impossible to hit)
                // just as a generic error. All 3 other enum states
                // for this are impossible with the FPGA library.
                _ => return Err(Error::FPGA(Status::ResourceNotFound)),
            },
        };

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
        let mut buffer = [0u8; (T::SIZE_IN_BITS - 1) / 8 + 1];
        let status = Status::from(unsafe {
            self.api.base.NiFpgaDll_ReadArrayU8(
                self.handle,
                offset,
                buffer.as_mut_ptr(),
                (T::SIZE_IN_BITS - 1) / 8 + 1,
            )
        });
        match status {
            Status::Success => Ok(Datatype::unpack(
                &FpgaBits::from_slice(&buffer)
                    [((T::SIZE_IN_BITS - 1) / 8 + 1) * 8 - T::SIZE_IN_BITS..],
            )?),
            _ => Err(Error::FPGA(status)),
        }
    }
    pub fn write<T: Datatype>(&self, offset: Offset, data: &T) -> Result<(), Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        let mut buffer = [0u8; (T::SIZE_IN_BITS - 1) / 8 + 1];
        Datatype::pack(
            &mut FpgaBits::from_slice_mut(&mut buffer)
                [((T::SIZE_IN_BITS - 1) / 8 + 1) * 8 - T::SIZE_IN_BITS..],
            data,
        )?;
        let status = Status::from(unsafe {
            self.api.base.NiFpgaDll_WriteArrayU8(
                self.handle,
                offset,
                buffer.as_ptr(),
                (T::SIZE_IN_BITS - 1) / 8 + 1,
            )
        });
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
