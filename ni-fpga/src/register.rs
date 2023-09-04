use std::{marker::PhantomData, ops::Deref};

use crate::{nifpga::NiFpga, Datatype, Error, Offset, Session};

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

pub trait RegisterAccess<T>
where
    T: Datatype,
{
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<T, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>;
    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
    ) -> Result<[T; LEN], Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>;
    fn write<Fpga>(&mut self, session: &Session<Fpga>, value: T) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>;
    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
        value: &[T; LEN],
    ) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>;
}
