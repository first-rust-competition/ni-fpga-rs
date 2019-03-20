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
        let status = Status::from(unsafe {
            ffi::Open(
                CString::new(bitfile).unwrap().as_ptr(),
                CString::new(signature).unwrap().as_ptr(),
                CString::new(resource).unwrap().as_ptr(),
                0,
                &mut handle as *mut ffi::Session,
            )
        });
        match status {
            Status::Success => Ok(Session {
                handle: handle,
            }),
            _ => Err(status),
        }
    }
    pub fn read<T: Datatype>(
        self,
        offset: Offset,
    ) -> Result<T, Status> {
        Datatype::read(
            self,
            offset,
        )
    }
    pub fn write<T: Datatype>(
        self,
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
