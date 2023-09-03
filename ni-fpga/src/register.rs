use std::marker::PhantomData;

use crate::{Datatype, Offset};

pub trait GetOffset<T: Datatype> {
    fn offset(&self) -> Offset;
}

pub struct ConstOffset<T: Datatype, const N: Offset>(PhantomData<T>);

impl<T: Datatype, const N: Offset> GetOffset<T> for ConstOffset<T, N> {
    fn offset(&self) -> Offset {
        N
    }
}

pub struct StoredOffset<T: Datatype>(Offset, PhantomData<T>);

impl<T: Datatype> GetOffset<T> for StoredOffset<T> {
    fn offset(&self) -> Offset {
        self.0
    }
}

pub struct Register<O> {
    offset_type: O,
}

impl<T: Datatype> Register<StoredOffset<T>> {
    pub fn new(offset: Offset) -> Self {
        Self {
            offset_type: StoredOffset(offset, PhantomData),
        }
    }
}

impl<T: Datatype, const N: Offset> Register<ConstOffset<T, N>> {
    pub fn new_const() -> Self {
        Self {
            offset_type: ConstOffset(PhantomData),
        }
    }
}

impl<O> Register<O> {
    pub fn offset<T: Datatype>(&self) -> Offset
    where
        O: GetOffset<T>,
    {
        self.offset_type.offset()
    }
}

impl<T: Datatype, const N: Offset> From<Register<ConstOffset<T, N>>> for Register<StoredOffset<T>> {
    fn from(_: Register<ConstOffset<T, N>>) -> Self {
        Self {
            offset_type: StoredOffset(N, PhantomData),
        }
    }
}

#[cfg(feature = "use_generic_const_exprs")]
use crate::{nifpga::NiFpga, session::SessionAccess, Error, Session};
#[cfg(feature = "use_generic_const_exprs")]
use std::ops::Deref;
#[cfg(feature = "use_generic_const_exprs")]
pub trait RegisterAccess<T>
where
    T: Datatype,
{
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<T, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized;

    fn write<Fpga>(&mut self, session: &Session<Fpga>, data: &T) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized;
}

#[cfg(feature = "use_generic_const_exprs")]
impl<T, N: GetOffset<T>> RegisterAccess<T> for Register<N>
where
    T: Datatype,
{
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<T, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        session.read(self.offset())
    }

    fn write<Fpga>(&mut self, session: &Session<Fpga>, data: &T) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        session.write(self.offset(), data)
    }
}
