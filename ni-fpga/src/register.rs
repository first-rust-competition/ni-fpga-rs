use std::{borrow::Borrow, marker::PhantomData};

use crate::{Datatype, Error, Offset, SessionAccess};

pub struct ReadOnly;
pub struct WriteOnly;
pub struct ReadWrite;

pub trait RegisterPermission {}

impl RegisterPermission for ReadOnly {}
impl RegisterPermission for ReadWrite {}
impl RegisterPermission for WriteOnly {}

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

pub trait RegisterOffset: Copy + Clone {}
impl RegisterOffset for StoredOffset {}
impl<const N: Offset> RegisterOffset for ConstOffset<N> {}

pub struct Register<T, P, O>
where
    T: Datatype,
    P: RegisterPermission,
    O: RegisterOffset,
{
    _offset_type: O,
    _type: PhantomData<T>,
    _perms: PhantomData<P>,
}

impl<T, P, O> Register<T, P, O>
where
    T: Datatype,
    P: RegisterPermission,
    O: RegisterOffset,
{
    pub unsafe fn duplicate(&self) -> Self {
        Self {
            _offset_type: self._offset_type,
            _type: PhantomData,
            _perms: PhantomData,
        }
    }
}

impl<T, P> Register<T, P, StoredOffset>
where
    T: Datatype,
    P: RegisterPermission,
{
    #[inline]
    pub const unsafe fn new(offset: Offset) -> Self {
        Self {
            _offset_type: StoredOffset(offset),
            _type: PhantomData,
            _perms: PhantomData,
        }
    }

    pub const unsafe fn transmute_type<NT>(self) -> Register<NT, P, StoredOffset>
    where
        NT: Datatype,
    {
        Register {
            _offset_type: self._offset_type,
            _type: PhantomData,
            _perms: PhantomData,
        }
    }

    pub const unsafe fn transmute_permissions<NP>(self) -> Register<T, NP, StoredOffset>
    where
        NP: RegisterPermission,
    {
        Register {
            _offset_type: self._offset_type,
            _type: PhantomData,
            _perms: PhantomData,
        }
    }
}

impl<T, P, const N: Offset> Register<T, P, ConstOffset<N>>
where
    T: Datatype,
    P: RegisterPermission,
{
    #[inline]
    pub const unsafe fn new_const() -> Self {
        Self {
            _offset_type: ConstOffset,
            _type: PhantomData,
            _perms: PhantomData,
        }
    }

    pub const unsafe fn transmute_type<NT>(self) -> Register<NT, P, ConstOffset<N>>
    where
        NT: Datatype,
    {
        Register {
            _offset_type: self._offset_type,
            _type: PhantomData,
            _perms: PhantomData,
        }
    }

    pub const unsafe fn transmute_permissions<NP>(self) -> Register<T, NP, ConstOffset<N>>
    where
        NP: RegisterPermission,
    {
        Register {
            _offset_type: self._offset_type,
            _type: PhantomData,
            _perms: PhantomData,
        }
    }
}

impl<T, P, const N: Offset> From<Register<T, P, ConstOffset<N>>> for Register<T, P, StoredOffset>
where
    T: Datatype,
    P: RegisterPermission,
{
    #[inline]
    fn from(_: Register<T, P, ConstOffset<N>>) -> Self {
        Self {
            _offset_type: StoredOffset(N),
            _type: PhantomData,
            _perms: PhantomData,
        }
    }
}

pub trait RegisterRead<T>
where
    T: Datatype,
{
    #[doc(hidden)]
    fn offset_read(&self) -> Offset;

    #[inline(never)]
    fn read(&self, session: &impl SessionAccess) -> Result<T, Error> {
        unsafe { T::read(session, self.offset_read()) }
    }
}

pub trait RegisterWrite<T>
where
    T: Datatype,
{
    #[doc(hidden)]
    fn offset_write(&self) -> Offset;

    #[inline]
    fn write(&mut self, session: &impl SessionAccess, value: impl Borrow<T>) -> Result<(), Error> {
        unsafe { T::write(session, self.offset_write(), value) }
    }
}

impl<T, U> RegisterRead<T> for Register<T, ReadOnly, U>
where
    T: Datatype,
    U: RegisterOffset,
    Offset: From<U>,
    U: Copy,
{
    #[inline]
    fn offset_read(&self) -> Offset {
        self._offset_type.into()
    }
}

impl<T, U> RegisterWrite<T> for Register<T, WriteOnly, U>
where
    T: Datatype,
    U: RegisterOffset,
    Offset: From<U>,
    U: Copy,
{
    #[inline]
    fn offset_write(&self) -> Offset {
        self._offset_type.into()
    }
}

impl<T, U> RegisterRead<T> for Register<T, ReadWrite, U>
where
    T: Datatype,
    U: RegisterOffset,
    Offset: From<U>,
    U: Copy,
{
    #[inline]
    fn offset_read(&self) -> Offset {
        self._offset_type.into()
    }
}

impl<T, U> RegisterWrite<T> for Register<T, ReadWrite, U>
where
    T: Datatype,
    U: RegisterOffset,
    Offset: From<U>,
    U: Copy,
{
    #[inline]
    fn offset_write(&self) -> Offset {
        self._offset_type.into()
    }
}
