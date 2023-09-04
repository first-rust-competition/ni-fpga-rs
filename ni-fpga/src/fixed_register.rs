use std::ops::Deref;

use crate::{
    nifpga::NiFpga, register::GetOffset, Error, Register, RegisterAccess, Session, SessionAccess,
};

// impl<N: GetOffset<f32>> RegisterAccess<f32> for Register<N> {
//     fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<f32, Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().read_f32(self.offset())
//     }

//     fn read_array<Fpga, const LEN: usize>(
//         &self,
//         session: &Session<Fpga>,
//     ) -> Result<[f32; LEN], Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         let mut buffer = [0f32; LEN];
//         match session.fpga().read_f32_array(self.offset(), &mut buffer) {
//             Ok(_) => Ok(buffer),
//             Err(err) => Err(err),
//         }
//     }

//     fn write<Fpga>(&mut self, session: &Session<Fpga>, value: f32) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_f32(self.offset(), value)
//     }

//     fn write_array<Fpga>(&self, session: &Session<Fpga>, value: &[f32]) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_f32_array(self.offset(), value)
//     }
// }

// impl<N: GetOffset<f64>> RegisterAccess<f64> for Register<N> {
//     fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<f64, Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().read_f64(self.offset())
//     }

//     fn read_array<Fpga, const LEN: usize>(
//         &self,
//         session: &Session<Fpga>,
//     ) -> Result<[f64; LEN], Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         let mut buffer = [0f64; LEN];
//         match session.fpga().read_f64_array(self.offset(), &mut buffer) {
//             Ok(_) => Ok(buffer),
//             Err(err) => Err(err),
//         }
//     }

//     fn write<Fpga>(&mut self, session: &Session<Fpga>, value: f64) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_f64(self.offset(), value)
//     }

//     fn write_array<Fpga>(&self, session: &Session<Fpga>, value: &[f64]) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_f64_array(self.offset(), value)
//     }
// }

// impl<N: GetOffset<bool>> RegisterAccess<bool> for Register<N> {
//     fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<bool, Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().read_bool(self.offset())
//     }

//     fn read_array<Fpga, const LEN: usize>(
//         &self,
//         session: &Session<Fpga>,
//     ) -> Result<[bool; LEN], Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         let mut raw_array = [Default::default(); LEN];

//         session.fpga().read_bool_array_fast(self.offset(), &mut raw_array)?;
//         Ok(raw_array.map(|x| x != 0))
//     }

//     fn write<Fpga>(&mut self, session: &Session<Fpga>, value: bool) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_bool(self.offset(), value)
//     }

//     fn write_array<Fpga, const LEN: usize>(&self, session: &Session<Fpga>, value: &[bool; LEN]) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         let mapped = value.map(|x| if x {1} else {0});
//         session.fpga().write_bool_array_fast(self.offset(), &mapped)
//     }
// }

// impl<N: GetOffset<i8>> RegisterAccess<i8> for Register<N> {
//     fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<i8, Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().read_i8(self.offset())
//     }

//     fn read_array<Fpga, const LEN: usize>(
//         &self,
//         session: &Session<Fpga>,
//     ) -> Result<[i8; LEN], Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         let mut buffer = [0i8; LEN];
//         match session.fpga().read_i8_array(self.offset(), &mut buffer) {
//             Ok(_) => Ok(buffer),
//             Err(err) => Err(err),
//         }
//     }

//     fn write<Fpga>(&mut self, session: &Session<Fpga>, value: i8) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_i8(self.offset(), value)
//     }

//     fn write_array<Fpga>(&self, session: &Session<Fpga>, value: &[i8]) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_i8_array(self.offset(), value)
//     }
// }

// impl<N: GetOffset<u8>> RegisterAccess<u8> for Register<N> {
//     fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<u8, Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().read_u8(self.offset())
//     }

//     fn read_array<Fpga, const LEN: usize>(
//         &self,
//         session: &Session<Fpga>,
//     ) -> Result<[u8; LEN], Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         let mut buffer = [0u8; LEN];
//         match session.fpga().read_u8_array(self.offset(), &mut buffer) {
//             Ok(_) => Ok(buffer),
//             Err(err) => Err(err),
//         }
//     }

//     fn write<Fpga>(&mut self, session: &Session<Fpga>, value: u8) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_u8(self.offset(), value)
//     }

//     fn write_array<Fpga>(&self, session: &Session<Fpga>, value: &[u8]) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_u8_array(self.offset(), value)
//     }
// }

// impl<N: GetOffset<i16>> RegisterAccess<i16> for Register<N> {
//     fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<i16, Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().read_i16(self.offset())
//     }

//     fn read_array<Fpga, const LEN: usize>(
//         &self,
//         session: &Session<Fpga>,
//     ) -> Result<[i16; LEN], Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         let mut buffer = [0i16; LEN];
//         match session.fpga().read_i16_array(self.offset(), &mut buffer) {
//             Ok(_) => Ok(buffer),
//             Err(err) => Err(err),
//         }
//     }

//     fn write<Fpga>(&mut self, session: &Session<Fpga>, value: i16) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_i16(self.offset(), value)
//     }

//     fn write_array<Fpga>(&self, session: &Session<Fpga>, value: &[i16]) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_i16_array(self.offset(), value)
//     }
// }

// impl<N: GetOffset<u16>> RegisterAccess<u16> for Register<N> {
//     fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<u16, Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().read_u16(self.offset())
//     }

//     fn read_array<Fpga, const LEN: usize>(
//         &self,
//         session: &Session<Fpga>,
//     ) -> Result<[u16; LEN], Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         let mut buffer = [0u16; LEN];
//         match session.fpga().read_u16_array(self.offset(), &mut buffer) {
//             Ok(_) => Ok(buffer),
//             Err(err) => Err(err),
//         }
//     }

//     fn write<Fpga>(&mut self, session: &Session<Fpga>, value: u16) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_u16(self.offset(), value)
//     }

//     fn write_array<Fpga>(&self, session: &Session<Fpga>, value: &[u16]) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_u16_array(self.offset(), value)
//     }
// }

// impl<N: GetOffset<i32>> RegisterAccess<i32> for Register<N> {
//     fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<i32, Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().read_i32(self.offset())
//     }

//     fn read_array<Fpga, const LEN: usize>(
//         &self,
//         session: &Session<Fpga>,
//     ) -> Result<[i32; LEN], Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         let mut buffer = [0i32; LEN];
//         match session.fpga().read_i32_array(self.offset(), &mut buffer) {
//             Ok(_) => Ok(buffer),
//             Err(err) => Err(err),
//         }
//     }

//     fn write<Fpga>(&mut self, session: &Session<Fpga>, value: i32) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_i32(self.offset(), value)
//     }

//     fn write_array<Fpga>(&self, session: &Session<Fpga>, value: &[i32]) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_i32_array(self.offset(), value)
//     }
// }

// impl<N: GetOffset<u32>> RegisterAccess<u32> for Register<N> {
//     fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<u32, Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().read_u32(self.offset())
//     }

//     fn read_array<Fpga, const LEN: usize>(
//         &self,
//         session: &Session<Fpga>,
//     ) -> Result<[u32; LEN], Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         let mut buffer = [0u32; LEN];
//         match session.fpga().read_u32_array(self.offset(), &mut buffer) {
//             Ok(_) => Ok(buffer),
//             Err(err) => Err(err),
//         }
//     }

//     fn write<Fpga>(&mut self, session: &Session<Fpga>, value: u32) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_u32(self.offset(), value)
//     }

//     fn write_array<Fpga>(&self, session: &Session<Fpga>, value: &[u32]) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_u32_array(self.offset(), value)
//     }
// }

// impl<N: GetOffset<i64>> RegisterAccess<i64> for Register<N> {
//     fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<i64, Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().read_i64(self.offset())
//     }

//     fn read_array<Fpga, const LEN: usize>(
//         &self,
//         session: &Session<Fpga>,
//     ) -> Result<[i64; LEN], Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         let mut buffer = [0i64; LEN];
//         match session.fpga().read_i64_array(self.offset(), &mut buffer) {
//             Ok(_) => Ok(buffer),
//             Err(err) => Err(err),
//         }
//     }

//     fn write<Fpga>(&mut self, session: &Session<Fpga>, value: i64) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_i64(self.offset(), value)
//     }

//     fn write_array<Fpga>(&self, session: &Session<Fpga>, value: &[i64]) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_i64_array(self.offset(), value)
//     }
// }

// impl<N: GetOffset<u64>> RegisterAccess<u64> for Register<N> {
//     fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<u64, Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().read_u64(self.offset())
//     }

//     fn read_array<Fpga, const LEN: usize>(
//         &self,
//         session: &Session<Fpga>,
//     ) -> Result<[u64; LEN], Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         let mut buffer = [0u64; LEN];
//         match session.fpga().read_u64_array(self.offset(), &mut buffer) {
//             Ok(_) => Ok(buffer),
//             Err(err) => Err(err),
//         }
//     }

//     fn write<Fpga>(&mut self, session: &Session<Fpga>, value: u64) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_u64(self.offset(), value)
//     }

//     fn write_array<Fpga>(&self, session: &Session<Fpga>, value: &[u64]) -> Result<(), Error>
//     where
//         Fpga: Deref,
//         Fpga: Deref<Target = NiFpga>,
//     {
//         session.fpga().write_u64_array(self.offset(), value)
//     }
// }

impl<N: GetOffset<f32>> RegisterAccess<f32> for Register<N> {
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<f32, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().read_f32(self.offset())
    }

    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
    ) -> Result<[f32; LEN], Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        let mut buffer = [0f32; LEN];
        match session.fpga().read_f32_array(self.offset(), &mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err),
        }
    }

    fn write<Fpga>(&mut self, session: &Session<Fpga>, value: f32) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_f32(self.offset(), value)
    }

    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
        value: &[f32; LEN],
    ) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_f32_array(self.offset(), value)
    }
}

impl<N: GetOffset<f64>> RegisterAccess<f64> for Register<N> {
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<f64, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().read_f64(self.offset())
    }

    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
    ) -> Result<[f64; LEN], Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        let mut buffer = [0f64; LEN];
        match session.fpga().read_f64_array(self.offset(), &mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err),
        }
    }

    fn write<Fpga>(&mut self, session: &Session<Fpga>, value: f64) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_f64(self.offset(), value)
    }

    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
        value: &[f64; LEN],
    ) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_f64_array(self.offset(), value)
    }
}

impl<N: GetOffset<bool>> RegisterAccess<bool> for Register<N> {
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<bool, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().read_bool(self.offset())
    }

    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
    ) -> Result<[bool; LEN], Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        let mut raw_array = [Default::default(); LEN];

        session
            .fpga()
            .read_bool_array_fast(self.offset(), &mut raw_array)?;
        Ok(raw_array.map(|x| x != 0))
    }

    fn write<Fpga>(&mut self, session: &Session<Fpga>, value: bool) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_bool(self.offset(), value)
    }

    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
        value: &[bool; LEN],
    ) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        let mapped = value.map(|x| if x { 1 } else { 0 });
        session.fpga().write_bool_array_fast(self.offset(), &mapped)
    }
}

impl<N: GetOffset<i8>> RegisterAccess<i8> for Register<N> {
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<i8, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().read_i8(self.offset())
    }

    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
    ) -> Result<[i8; LEN], Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        let mut buffer = [0i8; LEN];
        match session.fpga().read_i8_array(self.offset(), &mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err),
        }
    }

    fn write<Fpga>(&mut self, session: &Session<Fpga>, value: i8) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_i8(self.offset(), value)
    }

    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
        value: &[i8; LEN],
    ) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_i8_array(self.offset(), value)
    }
}

impl<N: GetOffset<u8>> RegisterAccess<u8> for Register<N> {
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<u8, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().read_u8(self.offset())
    }

    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
    ) -> Result<[u8; LEN], Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        let mut buffer = [0u8; LEN];
        match session.fpga().read_u8_array(self.offset(), &mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err),
        }
    }

    fn write<Fpga>(&mut self, session: &Session<Fpga>, value: u8) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_u8(self.offset(), value)
    }

    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
        value: &[u8; LEN],
    ) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_u8_array(self.offset(), value)
    }
}

impl<N: GetOffset<i16>> RegisterAccess<i16> for Register<N> {
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<i16, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().read_i16(self.offset())
    }

    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
    ) -> Result<[i16; LEN], Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        let mut buffer = [0i16; LEN];
        match session.fpga().read_i16_array(self.offset(), &mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err),
        }
    }

    fn write<Fpga>(&mut self, session: &Session<Fpga>, value: i16) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_i16(self.offset(), value)
    }

    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
        value: &[i16; LEN],
    ) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_i16_array(self.offset(), value)
    }
}

impl<N: GetOffset<u16>> RegisterAccess<u16> for Register<N> {
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<u16, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().read_u16(self.offset())
    }

    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
    ) -> Result<[u16; LEN], Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        let mut buffer = [0u16; LEN];
        match session.fpga().read_u16_array(self.offset(), &mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err),
        }
    }

    fn write<Fpga>(&mut self, session: &Session<Fpga>, value: u16) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_u16(self.offset(), value)
    }

    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
        value: &[u16; LEN],
    ) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_u16_array(self.offset(), value)
    }
}

impl<N: GetOffset<i32>> RegisterAccess<i32> for Register<N> {
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<i32, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().read_i32(self.offset())
    }

    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
    ) -> Result<[i32; LEN], Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        let mut buffer = [0i32; LEN];
        match session.fpga().read_i32_array(self.offset(), &mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err),
        }
    }

    fn write<Fpga>(&mut self, session: &Session<Fpga>, value: i32) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_i32(self.offset(), value)
    }

    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
        value: &[i32; LEN],
    ) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_i32_array(self.offset(), value)
    }
}

impl<N: GetOffset<u32>> RegisterAccess<u32> for Register<N> {
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<u32, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().read_u32(self.offset())
    }

    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
    ) -> Result<[u32; LEN], Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        let mut buffer = [0u32; LEN];
        match session.fpga().read_u32_array(self.offset(), &mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err),
        }
    }

    fn write<Fpga>(&mut self, session: &Session<Fpga>, value: u32) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_u32(self.offset(), value)
    }

    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
        value: &[u32; LEN],
    ) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_u32_array(self.offset(), value)
    }
}

impl<N: GetOffset<i64>> RegisterAccess<i64> for Register<N> {
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<i64, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().read_i64(self.offset())
    }

    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
    ) -> Result<[i64; LEN], Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        let mut buffer = [0i64; LEN];
        match session.fpga().read_i64_array(self.offset(), &mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err),
        }
    }

    fn write<Fpga>(&mut self, session: &Session<Fpga>, value: i64) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_i64(self.offset(), value)
    }

    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
        value: &[i64; LEN],
    ) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_i64_array(self.offset(), value)
    }
}

impl<N: GetOffset<u64>> RegisterAccess<u64> for Register<N> {
    fn read<Fpga>(&self, session: &Session<Fpga>) -> Result<u64, Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().read_u64(self.offset())
    }

    fn read_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
    ) -> Result<[u64; LEN], Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        let mut buffer = [0u64; LEN];
        match session.fpga().read_u64_array(self.offset(), &mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err),
        }
    }

    fn write<Fpga>(&mut self, session: &Session<Fpga>, value: u64) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_u64(self.offset(), value)
    }

    fn write_array<Fpga, const LEN: usize>(
        &self,
        session: &Session<Fpga>,
        value: &[u64; LEN],
    ) -> Result<(), Error>
    where
        Fpga: Deref,
        Fpga: Deref<Target = NiFpga>,
    {
        session.fpga().write_u64_array(self.offset(), value)
    }
}
