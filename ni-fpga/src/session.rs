use std::ffi::CString;

use crate::Datatype;
use crate::ffi;
use crate::Offset;
use crate::Status;

pub struct Session {
    pub handle: ffi::Session,
}

impl Session {
    pub fn open(
        bitfile: &str,
        signature: &str,
        resource: &str,
    ) -> Result<Self, Status> {
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
            Status::Success => Ok(Session {
                handle,
            }),
            _ => Err(status),
        }
    }
    pub fn read<T: Datatype>(
        &self,
        offset: Offset,
    ) -> Result<T, Status> {
        Datatype::read(
            self,
            offset,
        )
    }
    pub fn write<T: Datatype>(
        &self,
        offset: Offset,
        value: T,
    ) -> Result<(), Status> {
        Datatype::write(
            self,
            offset,
            value,
        )
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            ffi::Close(
                self.handle,
                0,
            )
        };
    }
}
