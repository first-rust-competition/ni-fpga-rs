use std::ops::Deref;

use crate::{
    erased_register::ErasedRegister, nifpga::NiFpga, Error, Offset, Register, SessionAccess,
};

pub trait FixedRegisterAccess<T> {
    fn read_direct(&self) -> Result<T, Error>;
    fn write_direct(&mut self, value: T) -> Result<(), Error>;
}

impl<Fpga, const N: Offset> FixedRegisterAccess<f32> for Register<Fpga, f32, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<f32, Error> {
        self.session.fpga().read_f32(N)
    }

    fn write_direct(&mut self, value: f32) -> Result<(), Error> {
        self.session.fpga().write_f32(N, value)
    }
}

impl<Fpga> FixedRegisterAccess<f32> for ErasedRegister<Fpga, f32>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<f32, Error> {
        self.session.fpga().read_f32(self.offset)
    }

    fn write_direct(&mut self, value: f32) -> Result<(), Error> {
        self.session.fpga().write_f32(self.offset, value)
    }
}

impl<Fpga, const N: Offset> FixedRegisterAccess<f64> for Register<Fpga, f64, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<f64, Error> {
        self.session.fpga().read_f64(N)
    }

    fn write_direct(&mut self, value: f64) -> Result<(), Error> {
        self.session.fpga().write_f64(N, value)
    }
}

impl<Fpga> FixedRegisterAccess<f64> for ErasedRegister<Fpga, f64>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<f64, Error> {
        self.session.fpga().read_f64(self.offset)
    }

    fn write_direct(&mut self, value: f64) -> Result<(), Error> {
        self.session.fpga().write_f64(self.offset, value)
    }
}

impl<Fpga, const N: Offset> FixedRegisterAccess<bool> for Register<Fpga, bool, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<bool, Error> {
        self.session.fpga().read_bool(N)
    }

    fn write_direct(&mut self, value: bool) -> Result<(), Error> {
        self.session.fpga().write_bool(N, value)
    }
}

impl<Fpga> FixedRegisterAccess<bool> for ErasedRegister<Fpga, bool>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<bool, Error> {
        self.session.fpga().read_bool(self.offset)
    }

    fn write_direct(&mut self, value: bool) -> Result<(), Error> {
        self.session.fpga().write_bool(self.offset, value)
    }
}

impl<Fpga, const N: Offset> FixedRegisterAccess<i8> for Register<Fpga, i8, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<i8, Error> {
        self.session.fpga().read_i8(N)
    }

    fn write_direct(&mut self, value: i8) -> Result<(), Error> {
        self.session.fpga().write_i8(N, value)
    }
}

impl<Fpga> FixedRegisterAccess<i8> for ErasedRegister<Fpga, i8>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<i8, Error> {
        self.session.fpga().read_i8(self.offset)
    }

    fn write_direct(&mut self, value: i8) -> Result<(), Error> {
        self.session.fpga().write_i8(self.offset, value)
    }
}

impl<Fpga, const N: Offset> FixedRegisterAccess<u8> for Register<Fpga, u8, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<u8, Error> {
        self.session.fpga().read_u8(N)
    }

    fn write_direct(&mut self, value: u8) -> Result<(), Error> {
        self.session.fpga().write_u8(N, value)
    }
}

impl<Fpga> FixedRegisterAccess<u8> for ErasedRegister<Fpga, u8>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<u8, Error> {
        self.session.fpga().read_u8(self.offset)
    }

    fn write_direct(&mut self, value: u8) -> Result<(), Error> {
        self.session.fpga().write_u8(self.offset, value)
    }
}

impl<Fpga, const N: Offset> FixedRegisterAccess<i16> for Register<Fpga, i16, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<i16, Error> {
        self.session.fpga().read_i16(N)
    }

    fn write_direct(&mut self, value: i16) -> Result<(), Error> {
        self.session.fpga().write_i16(N, value)
    }
}

impl<Fpga> FixedRegisterAccess<i16> for ErasedRegister<Fpga, i16>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<i16, Error> {
        self.session.fpga().read_i16(self.offset)
    }

    fn write_direct(&mut self, value: i16) -> Result<(), Error> {
        self.session.fpga().write_i16(self.offset, value)
    }
}

impl<Fpga, const N: Offset> FixedRegisterAccess<u16> for Register<Fpga, u16, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<u16, Error> {
        self.session.fpga().read_u16(N)
    }

    fn write_direct(&mut self, value: u16) -> Result<(), Error> {
        self.session.fpga().write_u16(N, value)
    }
}

impl<Fpga> FixedRegisterAccess<u16> for ErasedRegister<Fpga, u16>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<u16, Error> {
        self.session.fpga().read_u16(self.offset)
    }

    fn write_direct(&mut self, value: u16) -> Result<(), Error> {
        self.session.fpga().write_u16(self.offset, value)
    }
}

impl<Fpga, const N: Offset> FixedRegisterAccess<i32> for Register<Fpga, i32, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<i32, Error> {
        self.session.fpga().read_i32(N)
    }

    fn write_direct(&mut self, value: i32) -> Result<(), Error> {
        self.session.fpga().write_i32(N, value)
    }
}

impl<Fpga> FixedRegisterAccess<i32> for ErasedRegister<Fpga, i32>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<i32, Error> {
        self.session.fpga().read_i32(self.offset)
    }

    fn write_direct(&mut self, value: i32) -> Result<(), Error> {
        self.session.fpga().write_i32(self.offset, value)
    }
}

impl<Fpga, const N: Offset> FixedRegisterAccess<u32> for Register<Fpga, u32, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<u32, Error> {
        self.session.fpga().read_u32(N)
    }

    fn write_direct(&mut self, value: u32) -> Result<(), Error> {
        self.session.fpga().write_u32(N, value)
    }
}

impl<Fpga> FixedRegisterAccess<u32> for ErasedRegister<Fpga, u32>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<u32, Error> {
        self.session.fpga().read_u32(self.offset)
    }

    fn write_direct(&mut self, value: u32) -> Result<(), Error> {
        self.session.fpga().write_u32(self.offset, value)
    }
}

impl<Fpga, const N: Offset> FixedRegisterAccess<i64> for Register<Fpga, i64, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<i64, Error> {
        self.session.fpga().read_i64(N)
    }

    fn write_direct(&mut self, value: i64) -> Result<(), Error> {
        self.session.fpga().write_i64(N, value)
    }
}

impl<Fpga> FixedRegisterAccess<i64> for ErasedRegister<Fpga, i64>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<i64, Error> {
        self.session.fpga().read_i64(self.offset)
    }

    fn write_direct(&mut self, value: i64) -> Result<(), Error> {
        self.session.fpga().write_i64(self.offset, value)
    }
}

impl<Fpga, const N: Offset> FixedRegisterAccess<u64> for Register<Fpga, u64, N>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<u64, Error> {
        self.session.fpga().read_u64(N)
    }

    fn write_direct(&mut self, value: u64) -> Result<(), Error> {
        self.session.fpga().write_u64(N, value)
    }
}

impl<Fpga> FixedRegisterAccess<u64> for ErasedRegister<Fpga, u64>
where
    Fpga: Deref,
    Fpga: Deref<Target = NiFpga>,
{
    fn read_direct(&self) -> Result<u64, Error> {
        self.session.fpga().read_u64(self.offset)
    }

    fn write_direct(&mut self, value: u64) -> Result<(), Error> {
        self.session.fpga().write_u64(self.offset, value)
    }
}
