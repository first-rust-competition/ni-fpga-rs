use std::ffi::CString;

use bitvec::prelude::*;

use crate::ffi;
use crate::Datatype;
use crate::Error;
use crate::Offset;
use crate::Status;

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
    pub fn read<T: Datatype>(&self, offset: Offset) -> Result<T, Error> {
        // Ideally we would declare an array of `[u8; (T::SIZE_IN_BITS - 1) / 8 + 1`,
        // read from the FPGA, and take a BitSlice from it. However, this declaration
        // is not allowed: https://github.com/rust-lang/rust/issues/68436.
        // When this issue is fixed, we should use the described approach.
        let mut bv = BitVec::with_capacity(((T::SIZE_IN_BITS - 1) / 8 + 1) * 8);
        unsafe {
            bv.set_len(((T::SIZE_IN_BITS - 1) / 8 + 1) * 8);
        }
        let fpga_bits = bv.as_mut_bitslice();
        let status = Status::from(unsafe {
            ffi::ReadArrayU8(
                self.handle,
                offset,
                fpga_bits.as_mut_slice().as_mut_ptr(),
                (T::SIZE_IN_BITS - 1) / 8 + 1,
            )
        });
        match status {
            Status::Success => Ok(Datatype::unpack(
                &fpga_bits[((T::SIZE_IN_BITS - 1) / 8 + 1) * 8 - T::SIZE_IN_BITS..],
            )?),
            _ => Err(Error::FPGA(status)),
        }
    }
    pub fn write<T: Datatype>(&self, offset: Offset, data: &T) -> Result<(), Error> {
        // Same as above - it would be better to declare an uninit fixed size array,
        // take a mutable BitSlice from it, pack into it, then write the array to the
        // FPGA.
        let mut bv = BitVec::with_capacity(((T::SIZE_IN_BITS - 1) / 8 + 1) * 8);
        unsafe {
            bv.set_len(((T::SIZE_IN_BITS - 1) / 8 + 1) * 8);
        }
        let fpga_bits = bv.as_mut_bitslice();
        Datatype::pack(
            &mut fpga_bits[((T::SIZE_IN_BITS - 1) / 8 + 1) * 8 - T::SIZE_IN_BITS..],
            data,
        )?;
        let status = Status::from(unsafe {
            ffi::WriteArrayU8(
                self.handle,
                offset,
                fpga_bits.as_slice().as_ptr(),
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
