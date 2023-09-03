use std::ops::Deref;

use crate::{
    nifpga::NiFpga, register::GetOffset, Datatype, Error, Register, Session, SessionAccess,
};

pub trait FixedRegisterAccess<T>
where
    T: Datatype,
{
    fn read_direct<Fpga>(&self, session: &Session<Fpga>) -> Result<T, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>;
    fn write_direct<Fpga>(&mut self, session: &Session<Fpga>, value: T) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>;
}

impl<N: GetOffset<u16>> FixedRegisterAccess<u16> for Register<N> {
    fn read_direct<Fpga>(&self, session: &Session<Fpga>) -> Result<u16, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().read_u16(self.offset())
    }

    fn write_direct<Fpga>(&mut self, session: &Session<Fpga>, value: u16) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_u16(self.offset(), value)
    }
}
