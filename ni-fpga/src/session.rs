use std::ffi::CString;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

use ni_fpga_sys::{CloseAttribute, OpenAttribute};

use crate::datatype::Datatype;
use crate::errors::Error;
use crate::hmb::Hmb;
use crate::nifpga::NiFpga;
use crate::register::{ConstOffset, Register, StoredOffset, ReadOnly};
use crate::session_lifetimes::{ArcStorage, InPlaceStorage, StorageClone};
use crate::Offset;

pub struct Session<FpgaStorage> {
    fpga_storage: FpgaStorage,
}

impl<'a, FpgaStorage> Session<FpgaStorage>
where
    FpgaStorage: StorageClone<'a>,
    FpgaStorage: Deref<Target = NiFpga>,
{
    fn open_local(
        bitfile: CString,
        signature: CString,
        resource: CString,
        open_attribute: OpenAttribute,
        close_attribute: CloseAttribute,
        create_self: impl FnOnce(NiFpga) -> Self,
    ) -> Result<Self, Error> {
        Ok(create_self(NiFpga::open(
            bitfile,
            signature,
            resource,
            open_attribute,
            close_attribute,
        )?))
    }

    fn from_session_local(
        session: ffi::Session,
        create_self: impl FnOnce(NiFpga) -> Self,
    ) -> Result<Self, Error> {
        Ok(create_self(NiFpga::from_session(session)?))
    }

    pub fn open_hmb(
        &'a self,
        memory_name: &str,
    ) -> Result<Hmb<<FpgaStorage as StorageClone<'a>>::Target>, Error>
    where
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

    fn open_inplace(
        bitfile: CString,
        signature: CString,
        resource: CString,
        open_attribute: OpenAttribute,
        close_attribute: CloseAttribute,
    ) -> Result<Self, Error> {
        Self::open_local(
            bitfile,
            signature,
            resource,
            open_attribute,
            close_attribute,
            Self::create_self,
        )
    }

    pub fn open(bitfile: &str, signature: &str, resource: &str) -> Result<Self, Error> {
        SessionBuilder::new()
            .bitfile_path(bitfile)?
            .signature(signature)?
            .resource(resource)?
            .build()
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

    fn open_arc(
        bitfile: CString,
        signature: CString,
        resource: CString,
        open_attribute: OpenAttribute,
        close_attribute: CloseAttribute,
    ) -> Result<Self, Error> {
        Self::open_local(
            bitfile,
            signature,
            resource,
            open_attribute,
            close_attribute,
            Self::create_self,
        )
    }

    pub fn from_session_arc(session: ffi::Session) -> Result<Self, Error> {
        Self::from_session_local(session, Self::create_self)
    }
}

enum BitfileType {
    Path(CString),
    Contents(CString),
}

#[derive(Default)]
pub struct SessionBuilder {
    bitfile_type: Option<BitfileType>,
    signature: Option<CString>,
    resource: Option<CString>,
    bitfile_utf8: bool,
    ignore_signature: bool,
    no_run: bool,
    no_reset: bool,
}

impl SessionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bitfile_path(mut self, path: impl AsRef<str>) -> Result<Self, Error> {
        self.bitfile_type = Some(BitfileType::Path(CString::new(path.as_ref())?));
        Ok(self)
    }

    pub fn bitfile_contents(mut self, contents: impl AsRef<str>) -> Result<Self, Error> {
        self.bitfile_type = Some(BitfileType::Contents(CString::new(contents.as_ref())?));
        Ok(self)
    }

    pub fn signature(mut self, signature: impl AsRef<str>) -> Result<Self, Error> {
        self.signature = Some(CString::new(signature.as_ref())?);
        Ok(self)
    }

    pub fn resource(mut self, resource: impl AsRef<str>) -> Result<Self, Error> {
        self.resource = Some(CString::new(resource.as_ref())?);
        Ok(self)
    }

    pub fn treat_bitfile_path_as_utf8(mut self) -> Self {
        self.bitfile_utf8 = true;
        self
    }

    pub fn ignore_signature(mut self) -> Self {
        self.ignore_signature = true;
        self
    }

    pub fn no_run(mut self) -> Self {
        self.no_run = true;
        self
    }

    pub fn no_reset_if_last_session(mut self) -> Self {
        self.no_reset = true;
        self
    }

    fn build_args(
        self,
    ) -> Result<(CString, CString, CString, OpenAttribute, CloseAttribute), Error> {
        let mut open_attr = OpenAttribute::empty();

        let bitfile = match self.bitfile_type {
            Some(BitfileType::Contents(s)) => {
                open_attr |= OpenAttribute::BitfileContentsNotPath;
                s
            }
            Some(BitfileType::Path(s)) => s,
            None => return Err(Error::NoBitfileSpecified),
        };
        let signature = match self.signature {
            Some(s) => s,
            None => match self.ignore_signature {
                true => CString::new("")?,
                false => return Err(Error::NoSignatureSpecified),
            },
        };
        let resource = match self.resource {
            Some(s) => s,
            None => return Err(Error::NoResourceSpecified),
        };

        if self.ignore_signature {
            open_attr |= OpenAttribute::IgnoreSignatureArgument;
        }

        if self.bitfile_utf8 {
            open_attr |= OpenAttribute::BitfilePathIsUTF8;
        }

        if self.no_run {
            open_attr |= OpenAttribute::NoRun;
        }

        let close_attr = if self.no_reset {
            CloseAttribute::NoResetIfLastSession
        } else {
            CloseAttribute::empty()
        };

        Ok((bitfile, signature, resource, open_attr, close_attr))
    }

    pub fn build(self) -> Result<Session<InPlaceStorage<'static>>, Error> {
        let (bitfile, signature, resource, open_args, close_args) = self.build_args()?;
        Session::open_inplace(bitfile, signature, resource, open_args, close_args)
    }

    pub fn build_arc(self) -> Result<Session<ArcStorage>, Error> {
        let (bitfile, signature, resource, open_args, close_args) = self.build_args()?;
        Session::open_arc(bitfile, signature, resource, open_args, close_args)
    }
}

pub trait SessionAccess {
    fn fpga(&self) -> &NiFpga;

    fn read<T: Datatype>(&self, offset: Offset) -> Result<T, Error>;

    fn write<T: Datatype>(&self, offset: Offset, data: &T) -> Result<(), Error>;

    fn open_const_register<T: Datatype, P, const N: Offset>(&self) -> Register<T, P, ConstOffset<N>> {
        Register::new_const()
    }

    fn open_register<T: Datatype, P>(&self, offset: Offset) -> Register<T, P, StoredOffset> {
        Register::new(offset)
    }

    fn open_readonly_const_register<T: Datatype, const N: Offset>(&self) -> Register<T, ReadOnly, ConstOffset<N>> {
        Register::new_const()
    }

    fn open_readonly_register<T: Datatype>(&self, offset: Offset) -> Register<T, ReadOnly, StoredOffset> {
        Register::new(offset)
    }
}

impl<Fpga> SessionAccess for Session<Fpga>
where
    Fpga: Deref<Target = NiFpga>,
{
    fn fpga(&self) -> &NiFpga {
        &self.fpga_storage
    }

    fn read<T: Datatype>(&self, offset: Offset) -> Result<T, Error> {
        T::read(self, offset)
    }

    fn write<T: Datatype>(&self, offset: Offset, data: &T) -> Result<(), Error> {
        T::write(self, offset, data)
    }
}
