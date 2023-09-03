use std::{marker::PhantomData, ops::Deref};

use crate::{nifpga::NiFpga, Datatype, Offset, Register, Session};

pub struct ErasedRegister<Fpga, T>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
    T: Datatype,
{
    pub(crate) session: Session<Fpga>,
    pub(crate) offset: Offset,
    _type: PhantomData<T>,
}

impl<Fpga, T> ErasedRegister<Fpga, T>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
    T: Datatype,
{
    pub(crate) fn new(session: Session<Fpga>, offset: Offset) -> ErasedRegister<Fpga, T> {
        ErasedRegister {
            session,
            offset,
            _type: PhantomData,
        }
    }
}

impl<Fpga, T, const N: Offset> From<Register<Fpga, T, N>> for ErasedRegister<Fpga, T>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
    T: Datatype,
{
    fn from(value: Register<Fpga, T, N>) -> Self {
        ErasedRegister {
            session: value.session,
            offset: N,
            _type: PhantomData,
        }
    }
}

#[cfg(feature = "use_generic_const_exprs")]
use crate::{Error, RegisterAccess, SessionAccess};
#[cfg(feature = "use_generic_const_exprs")]
impl<Fpga, T> RegisterAccess<T> for ErasedRegister<Fpga, T>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
    T: Datatype,
{
    fn read(&self) -> Result<T, Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        self.session.read(self.offset)
    }

    fn write(&mut self, data: &T) -> Result<(), Error>
    where
        [u8; (T::SIZE_IN_BITS - 1) / 8 + 1]: Sized,
    {
        self.session.write(self.offset, data)
    }
}
