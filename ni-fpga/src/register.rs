use std::marker::PhantomData;

use crate::{Datatype, Error, Offset, SessionAccess};

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
    fn read(&self, session: &impl SessionAccess) -> Result<T, Error>;
    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &impl SessionAccess,
    ) -> Result<[T; LEN], Error>;
    fn write<Fpga>(&mut self, session: &impl SessionAccess, value: T) -> Result<(), Error>;
    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &impl SessionAccess,
        value: &[T; LEN],
    ) -> Result<(), Error>;
}

impl<T: Datatype, N: GetOffset<T>> RegisterAccess<T> for Register<N> {
    fn read(&self, session: &impl SessionAccess) -> Result<T, Error> {
        session.read(self.offset())
    }

    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &impl SessionAccess,
    ) -> Result<[T; LEN], Error> {
        session.read(self.offset())
    }

    fn write<Fpga>(&mut self, session: &impl SessionAccess, value: T) -> Result<(), Error> {
        session.write(self.offset(), &value)
    }

    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &impl SessionAccess,
        value: &[T; LEN],
    ) -> Result<(), Error> {
        session.write(self.offset(), value)
    }
}

pub trait RegisterAccessRef<T>
where
    T: Datatype,
{
    fn write_ref<Fpga>(&mut self, session: &impl SessionAccess, value: &T) -> Result<(), Error>;
}

impl<T: Datatype, N: GetOffset<T>> RegisterAccessRef<T> for Register<N> {
    fn write_ref<Fpga>(&mut self, session: &impl SessionAccess, value: &T) -> Result<(), Error>
    {
        session.write(self.offset(), value)
    }
}
