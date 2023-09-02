use std::ffi::CString;

use crate::datatype::{Datatype, FpgaBits};
use crate::errors::Error;
use crate::ffi;
use crate::ffi::Offset;
use crate::status::Status;

pub struct Session {
    pub api: crate::ffi::NiFpga,
}

impl Session {
    pub fn open(bitfile: &str, signature: &str, resource: &str) -> Result<Self, Error> {
        let c_bitfile = CString::new(bitfile).unwrap();
        let c_signature = CString::new(signature).unwrap();
        let c_resource = CString::new(resource).unwrap();
        match ffi::NiFpga::open(&c_bitfile, &c_signature, &c_resource, 0) {
            Ok(api) => Ok(Self { api }),
            Err(err) => match err {
                ffi::OpenError::NiFpgaError(fpga) => Err(Error::FPGA(Status::from(fpga))),
                ffi::OpenError::DlOpenError(dlopen) => Err(Error::DlOpen(dlopen)),
            },
        }
    }
    pub fn read<T: Datatype>(&self, offset: Offset) -> Result<T, Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        let mut buffer = [0u8; (T::SIZE_IN_BITS - 1) / 8 + 1];
        match self.api.read_u8_array(offset, &mut buffer) {
            Ok(_) => Ok(Datatype::unpack(
                &FpgaBits::from_slice(&buffer)
                    [((T::SIZE_IN_BITS - 1) / 8 + 1) * 8 - T::SIZE_IN_BITS..],
            )?),
            Err(err) => Err(Error::FPGA(err.into())),
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
        match self.api.write_u8_array(offset, &buffer) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::FPGA(err.into())),
        }
    }
}
