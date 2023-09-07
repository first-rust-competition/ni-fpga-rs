use std::ffi::CString;

use crate::datatype::{Datatype, FpgaBits};
use crate::errors::Error;
use crate::ffi;
use crate::ffi::Offset;
use crate::status::Status;

pub struct Session {
    pub handle: ffi::Session,
}

impl Session {
    pub fn open(bitfile: &str, signature: &str, resource: &str) -> Result<Self, Error> {
        let mut handle: ffi::Session = Default::default();
        let c_bitfile = CString::new(bitfile).unwrap();
        let c_signature = CString::new(signature).unwrap();
        let c_resource = CString::new(resource).unwrap();
        let status = Status::from(unsafe {
            ffi::Open(
                c_bitfile.as_ptr(),
                c_signature.as_ptr(),
                c_resource.as_ptr(),
                0,
                &mut handle as *mut ffi::Session,
            )
        });
        match status {
            Status::Success => Ok(Session { handle }),
            _ => Err(Error::FPGA(status)),
        }
    }
    pub fn read<T: Datatype>(&self, offset: Offset) -> Result<T, Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        let mut buffer = [0u8; (T::SIZE_IN_BITS - 1) / 8 + 1];
        let status = Status::from(unsafe {
            ffi::ReadArrayU8(
                self.handle,
                offset,
                buffer.as_mut_ptr(),
                (T::SIZE_IN_BITS - 1) / 8 + 1,
            )
        });
        match status {
            Status::Success => {
                let slice_start = if (T::SIZE_IN_BITS - 1) / 8 < 4 {
                    ((T::SIZE_IN_BITS - 1) / 8 + 1) * 8 - T::SIZE_IN_BITS
                } else {
                    0
                };
                Ok(Datatype::unpack(
                    &FpgaBits::from_slice(&buffer)[slice_start..],
                )?)
            }
            _ => Err(Error::FPGA(status)),
        }
    }
    pub fn write<T: Datatype>(&self, offset: Offset, data: &T) -> Result<(), Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        let mut buffer = [0u8; (T::SIZE_IN_BITS - 1) / 8 + 1];
        let slice_start = if (T::SIZE_IN_BITS - 1) / 8 < 4 {
            ((T::SIZE_IN_BITS - 1) / 8 + 1) * 8 - T::SIZE_IN_BITS
        } else {
            0
        };
        Datatype::pack(
            &mut FpgaBits::from_slice_mut(&mut buffer)[slice_start..],
            data,
        )?;
        let status = Status::from(unsafe {
            ffi::WriteArrayU8(
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
        unsafe { ffi::Close(self.handle, 0) };
    }
}
