use std::marker::PhantomData;

use crate::{Datatype, Error, Offset, SessionAccess};

#[derive(Clone, Copy)]
pub struct ConstOffset<const N: Offset>;

impl<const N: Offset> From<ConstOffset<N>> for Offset {
    #[inline]
    fn from(_value: ConstOffset<N>) -> Self {
        N
    }
}

#[derive(Clone, Copy)]
pub struct StoredOffset(Offset);

impl From<StoredOffset> for Offset {
    #[inline]
    fn from(value: StoredOffset) -> Self {
        value.0
    }
}

pub struct Register<T, O> {
    _offset_type: O,
    _type: PhantomData<T>,
}

impl<T> Register<T, StoredOffset> {
    #[inline]
    pub fn new(offset: Offset) -> Self {
        Self {
            _offset_type: StoredOffset(offset),
            _type: PhantomData,
        }
    }
}

impl<T, const N: Offset> Register<T, ConstOffset<N>> {
    #[inline]
    pub fn new_const() -> Self {
        Self {
            _offset_type: ConstOffset,
            _type: PhantomData,
        }
    }
}

impl<T, const N: Offset> From<Register<T, ConstOffset<N>>> for Register<T, StoredOffset> {
    #[inline]
    fn from(_: Register<T, ConstOffset<N>>) -> Self {
        Self {
            _offset_type: StoredOffset(N),
            _type: PhantomData,
        }
    }
}

// pub trait GetOffset {

// }

// impl<T> GetOffset for Register<T, StoredOffset> {
//     fn offset(&self) -> Offset {
//         self._offset_type.0
//     }
// }

// impl<T, const N: Offset> GetOffset for Register<T, ConstOffset<N>> {
//     fn offset(&self) -> Offset {
//         N
//     }
// }

pub trait RegisterAccess<T>: RegisterAccessGeneric<T>
where
    T: Datatype,
{
    #[inline]
    fn read(&self, session: &impl SessionAccess) -> Result<T, Error> {
        session.read(self.offset())
    }
    #[inline]
    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &impl SessionAccess,
    ) -> Result<[T; LEN], Error> {
        session.read(self.offset())
    }
    #[inline]
    fn write<Fpga>(&mut self, session: &impl SessionAccess, value: T) -> Result<(), Error> {
        session.write(self.offset(), &value)
    }
    #[inline]
    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &impl SessionAccess,
        value: &[T; LEN],
    ) -> Result<(), Error> {
        session.write(self.offset(), value)
    }
}

pub trait RegisterAccessGeneric<T>
where
    T: Datatype,
{
    fn offset(&self) -> Offset;

    #[inline]
    fn read_generic(&self, session: &impl SessionAccess) -> Result<T, Error> {
        session.read(self.offset())
    }
    #[inline]
    fn read_array_generic<Fpga, const LEN: usize>(
        &self,
        session: &impl SessionAccess,
    ) -> Result<[T; LEN], Error> {
        session.read(self.offset())
    }
    #[inline]
    fn write_generic<Fpga>(&mut self, session: &impl SessionAccess, value: T) -> Result<(), Error> {
        session.write(self.offset(), &value)
    }
    #[inline]
    fn write_array_generic<Fpga, const LEN: usize>(
        &self,
        session: &impl SessionAccess,
        value: &[T; LEN],
    ) -> Result<(), Error> {
        session.write(self.offset(), value)
    }
}

impl<T, U> RegisterAccessGeneric<T> for Register<T, U>
where
    T: Datatype,
    u32: From<U>,
    U: Copy,
{
    fn offset(&self) -> Offset {
        self._offset_type.into()
    }
}

pub trait RegisterAccessRef<T>: RegisterAccessGeneric<T>
where
    T: Datatype,
{
    #[inline]
    fn write_ref<Fpga>(&mut self, session: &impl SessionAccess, value: &T) -> Result<(), Error> {
        session.write(self.offset(), value)
    }
}

impl<U: Copy> RegisterAccess<u32> for Register<u32, U>
where
    Offset: From<U>,
{
    #[inline]
    fn read(&self, session: &impl SessionAccess) -> Result<u32, Error> {
        session.fpga().read_u32(self.offset())
    }
    #[inline]
    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &impl SessionAccess,
    ) -> Result<[u32; LEN], Error> {
        session.read(self.offset())
    }
    #[inline]
    fn write<Fpga>(&mut self, session: &impl SessionAccess, value: u32) -> Result<(), Error> {
        session.write(self.offset(), &value)
    }
    #[inline]
    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &impl SessionAccess,
        value: &[u32; LEN],
    ) -> Result<(), Error> {
        session.write(self.offset(), value)
    }
}

impl<U: Copy> RegisterAccess<u16> for Register<u16, U> where Offset: From<U> {}
