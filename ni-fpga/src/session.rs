use std::ffi::CString;
use std::ops::Deref;
use std::sync::Arc;

use ni_fpga_sys::{CloseAttribute, OpenAttribute};

use crate::datatype::{Datatype, FpgaBits};
use crate::errors::Error;
use crate::ffi::Offset;
use crate::hmb::Hmb;
use crate::nifpga::NiFpga;

#[derive(Clone)]
pub struct ArcStorage {
    api: Arc<NiFpga>,
}

impl Deref for ArcStorage {
    type Target = NiFpga;

    fn deref(&self) -> &Self::Target {
        &self.api
    }
}

pub struct InPlaceStorage {
    api: NiFpga,
}

impl Deref for InPlaceStorage {
    type Target = NiFpga;

    fn deref(&self) -> &Self::Target {
        &self.api
    }
}

impl<'a> From<&'a InPlaceStorage> for RefStorage<'a> {
    fn from(value: &'a InPlaceStorage) -> Self {
        Self { api: &value.api }
    }
}

pub struct RefStorage<'a> {
    api: &'a NiFpga,
}

impl<'a> Deref for RefStorage<'a> {
    type Target = NiFpga;

    fn deref(&self) -> &Self::Target {
        self.api
    }
}

pub struct Session<Fpga> {
    api: Fpga,
}

impl Session<InPlaceStorage> {
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
                api: InPlaceStorage { api },
            }),
            Err(err) => Err(err),
        }
    }

    pub fn from_session(session: ffi::Session) -> Result<Self, Error> {
        match NiFpga::from_session(session) {
            Ok(api) => Ok(Self {
                api: InPlaceStorage { api },
            }),
            Err(err) => Err(err),
        }
    }

    pub fn open_hmb(&self, memory_name: &str) -> Result<Hmb<RefStorage>, Error> {
        let c_memory_name = CString::new(memory_name).unwrap();
        Hmb::new((&self.api).into(), &c_memory_name)
    }
}

impl Session<ArcStorage> {
    pub fn open_arc(bitfile: &str, signature: &str, resource: &str) -> Result<Self, Error> {
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
                api: ArcStorage { api: Arc::new(api) },
            }),
            Err(err) => Err(err),
        }
    }

    pub fn from_session_arc(session: ffi::Session) -> Result<Self, Error> {
        match NiFpga::from_session(session) {
            Ok(api) => Ok(Self {
                api: ArcStorage { api: Arc::new(api) },
            }),
            Err(err) => Err(err),
        }
    }

    pub fn open_hmb(&self, memory_name: &str) -> Result<Hmb<ArcStorage>, Error> {
        let c_memory_name = CString::new(memory_name).unwrap();
        Hmb::new(self.api.clone(), &c_memory_name)
    }
}

impl<Fpga> Session<Fpga>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    pub fn fpga(&self) -> &NiFpga {
        &self.api
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
}
