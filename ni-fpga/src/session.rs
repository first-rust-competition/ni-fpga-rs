use std::ffi::CString;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

use ni_fpga_sys::{CloseAttribute, OpenAttribute};

use crate::datatype::Datatype;
use crate::errors::Error;
use crate::hmb::Hmb;
use crate::nifpga::NiFpga;
use crate::register::{ConstOffset, Register, StoredOffset};
use crate::session_lifetimes::{ArcStorage, InPlaceStorage, StorageClone};
use crate::Offset;

pub struct Session<FpgaStorage> {
    fpga_storage: FpgaStorage,
}

impl<'a, FpgaStorage> Session<FpgaStorage>
where
    FpgaStorage: StorageClone<'a>,
    FpgaStorage: Deref,
    FpgaStorage: Deref<Target = NiFpga>,
{
    fn open_local(
        bitfile: &str,
        signature: &str,
        resource: &str,
        create_self: impl FnOnce(NiFpga) -> Self,
    ) -> Result<Self, Error> {
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
            Ok(api) => Ok(create_self(api)),
            Err(err) => Err(err),
        }
    }

    fn from_session_local(
        session: ffi::Session,
        create_self: impl FnOnce(NiFpga) -> Self,
    ) -> Result<Self, Error> {
        match NiFpga::from_session(session) {
            Ok(api) => Ok(create_self(api)),
            Err(err) => Err(err),
        }
    }

    pub fn open_const_register<T: Datatype, const N: Offset>(&self) -> Register<ConstOffset<T, N>> {
        Register::new_const()
    }

    pub fn open_register<T: Datatype>(&self, offset: Offset) -> Register<StoredOffset<T>> {
        Register::new(offset)
    }

    pub fn open_hmb(
        &'a self,
        memory_name: &str,
    ) -> Result<Hmb<<FpgaStorage as StorageClone<'a>>::Target>, Error>
    where
        <FpgaStorage as StorageClone<'a>>::Target: Deref,
        <FpgaStorage as StorageClone<'a>>::Target: Deref<Target = NiFpga>,
    {
        let c_memory_name = CString::new(memory_name).unwrap();
        Hmb::new(
            Session {
                fpga_storage: self.fpga_storage.storage_clone(),
            },
            &c_memory_name,
        )
    }
}

impl Clone for Session<ArcStorage> {
    fn clone(&self) -> Self {
        Self {
            fpga_storage: self.fpga_storage.clone(),
        }
    }
}

impl Session<InPlaceStorage<'_>> {
    fn create_self(api: NiFpga) -> Self {
        Self {
            fpga_storage: InPlaceStorage {
                api,
                _marker: PhantomData,
            },
        }
    }

    pub fn open(bitfile: &str, signature: &str, resource: &str) -> Result<Self, Error> {
        Self::open_local(bitfile, signature, resource, Self::create_self)
    }

    pub fn from_session(session: ffi::Session) -> Result<Self, Error> {
        Self::from_session_local(session, Self::create_self)
    }
}

impl Session<ArcStorage> {
    fn create_self(api: NiFpga) -> Self {
        Self {
            fpga_storage: ArcStorage { api: Arc::new(api) },
        }
    }

    pub fn open_arc(bitfile: &str, signature: &str, resource: &str) -> Result<Self, Error> {
        Self::open_local(bitfile, signature, resource, Self::create_self)
    }

    pub fn from_session_arc(session: ffi::Session) -> Result<Self, Error> {
        Self::from_session_local(session, Self::create_self)
    }
}

pub trait SessionAccess {
    fn fpga(&self) -> &NiFpga;
}

impl<Fpga> SessionAccess for Session<Fpga>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn fpga(&self) -> &NiFpga {
        &self.fpga_storage
    }
}

#[cfg(feature = "use_generic_const_exprs")]
impl<Fpga> Session<Fpga>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    pub fn read<T: Datatype>(&self, offset: Offset) -> Result<T, Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        let mut buffer = [0u8; (T::SIZE_IN_BITS - 1) / 8 + 1];
        if (T::SIZE_IN_BITS - 1) / 8 + 1 <= 4 {
            match self.fpga_storage.read_u8_array(offset, &mut buffer) {
                Ok(_) => Ok(Datatype::unpack(
                    &crate::FpgaBits::from_slice(&buffer)
                        [((T::SIZE_IN_BITS - 1) / 8 + 1) * 8 - T::SIZE_IN_BITS..],
                )?),
                Err(err) => Err(err),
            }
        } else {
            match self.fpga_storage.read_u8_array(offset, &mut buffer) {
                Ok(_) => Ok(Datatype::unpack(
                    &crate::FpgaBits::from_slice(&buffer),
                )?),
                Err(err) => Err(err),
            }
        }
    }

    pub fn write<T: Datatype>(&self, offset: Offset, data: &T) -> Result<(), Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        let mut buffer = [0u8; (T::SIZE_IN_BITS - 1) / 8 + 1];
        if (T::SIZE_IN_BITS - 1) / 8 + 1 <= 4 {
            Datatype::pack(
                &mut crate::FpgaBits::from_slice_mut(&mut buffer)
                    [((T::SIZE_IN_BITS - 1) / 8 + 1) * 8 - T::SIZE_IN_BITS..],
                data,
            )?;
        } else {
            Datatype::pack(
                &mut crate::FpgaBits::from_slice_mut(&mut buffer),
                data,
            )?;
        }
        match self.fpga_storage.write_u8_array(offset, &buffer) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
