use std::ffi::CString;
use std::sync::Arc;

use ni_fpga_sys::{CloseAttribute, OpenAttribute};

use crate::datatype::{Datatype, FpgaBits};
use crate::errors::Error;
use crate::ffi::Offset;
use crate::hmb::Hmb;
use crate::nifpga::NiFpga;

#[derive(Clone)]
pub struct Session {
    pub api: Arc<Box<NiFpga>>,
}

impl Session {
    pub fn open(bitfile: &str, signature: &str, resource: &str) -> Result<Self, Error> {
        let c_bitfile = CString::new(bitfile).unwrap();
        let c_signature = CString::new(signature).unwrap();
        let c_resource = CString::new(resource).unwrap();
        match NiFpga::open(
            &c_bitfile,
            &c_signature,
            &c_resource,
            OpenAttribute::empty(),
            CloseAttribute::empty(),
        ) {
            Ok(api) => Ok(Self {
                api: Arc::new(Box::new(api)),
            }),
            Err(err) => Err(err),
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
            Err(err) => Err(err),
        }
    }
    pub fn write<T: Datatype>(&mut self, offset: Offset, data: &T) -> Result<(), Error>
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
            Err(err) => Err(err),
        }
    }

    pub fn open_hmb(&self, memory_name: &str) -> Result<Hmb, Error> {
        let c_memory_name = CString::new(memory_name).unwrap();
        Hmb::new(self.api.clone(), &c_memory_name)
    }
}
