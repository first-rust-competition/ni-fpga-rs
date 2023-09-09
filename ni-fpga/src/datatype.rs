use std::{borrow::Borrow, mem::MaybeUninit};

use bitvec::prelude::*;

use crate::{errors::Error, Offset, SessionAccess};

#[cfg(target_endian = "little")]
pub type FpgaBits = BitSlice<Msb0, u8>;
#[cfg(target_endian = "big")]
pub type FpgaBits = BitSlice<Lsb0, u8>;

pub trait DatatypePacker: Sized {
    const SIZE_IN_BITS: usize;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error>;
    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error>;
}

pub trait Datatype: DatatypePacker {
    #[inline]
    fn read(session: &impl SessionAccess, offset: Offset) -> Result<Self, Error> {
        session.read(offset)
    }

    #[inline]
    fn write(
        session: &impl SessionAccess,
        offset: Offset,
        value: impl Borrow<Self>,
    ) -> Result<(), Error> {
        session.write(offset, value.borrow())
    }
}

// Support array versions of any Datatype
impl<T: Datatype, const N: usize> DatatypePacker for [T; N] {
    const SIZE_IN_BITS: usize = T::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(fpga_bits.chunks_mut(T::SIZE_IN_BITS))
            .try_for_each(|(src, bits)| DatatypePacker::pack(bits, src))
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data: [std::mem::MaybeUninit<T>; N] =
            unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() };
        data.iter_mut()
            .zip(fpga_bits.chunks(T::SIZE_IN_BITS))
            .try_for_each::<_, Result<(), Error>>(|(dest, bits)| {
                *dest = std::mem::MaybeUninit::new(DatatypePacker::unpack(bits)?);
                Ok(())
            })?;
        // This is hack until https://github.com/rust-lang/rust/issues/61956 is addressed
        let ptr = &mut data as *mut _ as *mut [T; N];
        let res = unsafe { ptr.read() };
        std::mem::forget(data);
        Ok(res)
    }
}

impl Datatype for bool {}
impl Datatype for u8 {}
impl Datatype for u16 {}
impl Datatype for u32 {}
impl Datatype for u64 {}
impl Datatype for i8 {}
impl Datatype for i16 {}
impl Datatype for i32 {}
impl Datatype for i64 {}
impl Datatype for f32 {}
impl Datatype for f64 {}

impl<const N: usize> Datatype for [bool; N] {}
impl<const N: usize> Datatype for [u8; N] {}
impl<const N: usize> Datatype for [u16; N] {}
impl<const N: usize> Datatype for [u32; N] {}
impl<const N: usize> Datatype for [u64; N] {}
impl<const N: usize> Datatype for [i8; N] {}
impl<const N: usize> Datatype for [i16; N] {}
impl<const N: usize> Datatype for [i32; N] {}
impl<const N: usize> Datatype for [i64; N] {}
impl<const N: usize> Datatype for [f32; N] {}
impl<const N: usize> Datatype for [f64; N] {}

pub trait DerivedDatatype {}
impl<T, const N: usize> Datatype for [T; N] where T: DerivedDatatype + Datatype {}

impl DatatypePacker for bool {
    const SIZE_IN_BITS: usize = 1;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        fpga_bits.set(0, *data);
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(fpga_bits[0])
    }
}

impl DatatypePacker for u8 {
    const SIZE_IN_BITS: usize = 8;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        fpga_bits.store_be::<Self>(*data);
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(fpga_bits.load_be::<Self>())
    }
}

impl DatatypePacker for u16 {
    const SIZE_IN_BITS: usize = 16;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        fpga_bits.store_be::<Self>(*data);
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(fpga_bits.load_be::<Self>())
    }
}

impl DatatypePacker for u32 {
    const SIZE_IN_BITS: usize = 32;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        fpga_bits.store_be::<Self>(*data);
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(fpga_bits.load_be::<Self>())
    }
}

impl DatatypePacker for u64 {
    const SIZE_IN_BITS: usize = 64;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        fpga_bits[..32].store_be::<u32>((*data >> 32) as u32);
        fpga_bits[32..].store_be::<u32>(*data as u32);
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(((fpga_bits[..32].load_be::<u32>() as u64) << 32)
            | fpga_bits[32..].load_be::<u32>() as u64)
    }
}

impl DatatypePacker for i8 {
    const SIZE_IN_BITS: usize = 8;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        u8::pack(fpga_bits, &(*data as u8))
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(u8::unpack(fpga_bits)? as Self)
    }
}

impl DatatypePacker for i16 {
    const SIZE_IN_BITS: usize = 16;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        u16::pack(fpga_bits, &(*data as u16))
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(u16::unpack(fpga_bits)? as Self)
    }
}

impl DatatypePacker for i32 {
    const SIZE_IN_BITS: usize = 32;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        u32::pack(fpga_bits, &(*data as u32))
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(u32::unpack(fpga_bits)? as Self)
    }
}

impl DatatypePacker for i64 {
    const SIZE_IN_BITS: usize = 64;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        u64::pack(fpga_bits, &(*data as u64))
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(u64::unpack(fpga_bits)? as Self)
    }
}

impl DatatypePacker for f32 {
    const SIZE_IN_BITS: usize = 32;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        u32::pack(fpga_bits, &data.to_bits())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(Self::from_bits(u32::unpack(fpga_bits)?))
    }
}

impl DatatypePacker for f64 {
    const SIZE_IN_BITS: usize = 64;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        u64::pack(fpga_bits, &data.to_bits())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(Self::from_bits(u64::unpack(fpga_bits)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn round_trip_test<T: Datatype + PartialEq + std::fmt::Debug>(data: &T) -> Result<(), Error> {
        let mut bv = BitVec::with_capacity(T::SIZE_IN_BITS);
        unsafe {
            bv.set_len(T::SIZE_IN_BITS);
        }
        let mut fpga_bits = bv.as_mut_bitslice();
        T::pack(&mut fpga_bits, data)?;
        assert_eq!(T::unpack(&fpga_bits)?, *data);
        Ok(())
    }

    #[test]
    fn test_bool() -> Result<(), Error> {
        round_trip_test(&true)?;
        Ok(())
    }

    #[test]
    fn test_bool_array() -> Result<(), Error> {
        round_trip_test(&[true, false])?;
        Ok(())
    }

    #[test]
    fn test_u8() -> Result<(), Error> {
        round_trip_test(&0b00000001u8)?;
        Ok(())
    }
    #[test]
    fn test_u16() -> Result<(), Error> {
        round_trip_test(&0b0000001100000001u16)?;
        Ok(())
    }
    #[test]
    fn test_u32() -> Result<(), Error> {
        round_trip_test(&0b00001111000001110000001100000001u32)?;
        Ok(())
    }
    #[test]
    fn test_u64() -> Result<(), Error> {
        round_trip_test(&0b1111111101111111001111110001111100001111000001110000001100000001u64)?;
        Ok(())
    }

    #[test]
    #[allow(overflowing_literals)]
    fn test_i8() -> Result<(), Error> {
        round_trip_test(&0b10000000i8)?;
        Ok(())
    }
    #[test]
    #[allow(overflowing_literals)]
    fn test_i16() -> Result<(), Error> {
        round_trip_test(&0b1100000010000000i16)?;
        Ok(())
    }
    #[test]
    #[allow(overflowing_literals)]
    fn test_i32() -> Result<(), Error> {
        round_trip_test(&0b11110000111000001100000010000000i32)?;
        Ok(())
    }
    #[test]
    #[allow(overflowing_literals)]
    fn test_i64() -> Result<(), Error> {
        round_trip_test(&0b1111111111111110111111001111100011110000111000001100000010000000i64)?;
        Ok(())
    }

    #[test]
    fn test_f32() -> Result<(), Error> {
        round_trip_test(&3.14f32)?;
        Ok(())
    }
    #[test]
    fn test_f64() -> Result<(), Error> {
        round_trip_test(&3.14f64)?;
        Ok(())
    }
}
