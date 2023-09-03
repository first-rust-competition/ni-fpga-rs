use std::{marker::PhantomData, ops::Deref};

use crate::{nifpga::NiFpga, Datatype, Offset, Session};

pub struct Register<Fpga, T, const N: Offset>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
    T: Datatype,
{
    pub(crate) session: Session<Fpga>,
    _type: PhantomData<T>,
}

impl<Fpga, T, const N: Offset> Register<Fpga, T, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
    T: Datatype,
{
    pub(crate) fn new(session: Session<Fpga>) -> Register<Fpga, T, N> {
        Register {
            session,
            _type: PhantomData,
        }
    }
}

#[cfg(feature = "use_generic_const_exprs")]
use crate::{session::SessionAccess, Error};
#[cfg(feature = "use_generic_const_exprs")]
pub trait RegisterAccess<T>
where
    T: Datatype,
{
    fn read(&self) -> Result<T, Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized;

    fn write(&mut self, data: &T) -> Result<(), Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized;
}

#[cfg(feature = "use_generic_const_exprs")]
impl<Fpga, T, const N: Offset> RegisterAccess<T> for Register<Fpga, T, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
    T: Datatype,
{
    fn read(&self) -> Result<T, Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        self.session.read(N)
    }

    fn write(&mut self, data: &T) -> Result<(), Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        self.session.write(N, data)
    }
}
