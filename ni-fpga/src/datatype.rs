use std::{borrow::Borrow, mem::MaybeUninit};

use bitvec::{access::BitSafeU8, prelude::*};

use crate::{errors::Error, Offset, SessionAccess};

#[cfg(target_endian = "little")]
pub type FpgaBits = BitSlice<BitSafeU8, Msb0>;
#[cfg(target_endian = "little")]
pub type FpgaBitsRaw = BitSlice<u8, Msb0>;
#[cfg(target_endian = "big")]
pub type FpgaBits = BitSlice<BitSafeU8a, Lsb0>;
#[cfg(target_endian = "big")]
pub type FpgaBitsRaw = BitSlice<u8, Lsb0>;

pub trait DatatypePacker: Sized {
    const SIZE_IN_BITS: usize;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error>;
    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error>;
}

enum SmallBuffer<T, const N: usize> {
    InPlace(([T; N], usize)),
    Alloc(Vec<T>),
}

impl<T: Copy, const N: usize> SmallBuffer<T, N> {
    pub fn new(size: usize, val: T) -> Self {
        if size <= N {
            Self::InPlace(([val; N], size))
        } else {
            Self::Alloc(vec![val; size])
        }
    }

    pub fn buffer(&mut self) -> &mut [T] {
        match self {
            SmallBuffer::InPlace(ref mut b) => &mut b.0[0..b.1],
            SmallBuffer::Alloc(ref mut b) => b,
        }
    }
}

pub trait Datatype: DatatypePacker {
    #[inline]
    fn read(session: &impl SessionAccess, offset: Offset) -> Result<Self, Error> {
        // Most types are smaller then 4, so preallocate for 4
        let byte_size = (Self::SIZE_IN_BITS - 1) / 8 + 1;
        let mut buffer: SmallBuffer<u8, 4> = SmallBuffer::new(byte_size, 0u8);
        match session.fpga().read_u8_array(offset, buffer.buffer()) {
            Ok(_) => {
                // Values larger then a single element (32 bit) are left justified, not right
                let bit_slice = crate::datatype::FpgaBitsRaw::from_slice_mut(buffer.buffer());
                let bit_slice = if byte_size <= 4 {
                    bit_slice.split_at_mut(byte_size * 8 - Self::SIZE_IN_BITS).1
                } else {
                    bit_slice.split_at_mut(Self::SIZE_IN_BITS).0
                };

                Ok(DatatypePacker::unpack(bit_slice)?)
            }
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn write(
        session: &impl SessionAccess,
        offset: Offset,
        value: impl Borrow<Self>,
    ) -> Result<(), Error> {
        // Most types are smaller then 4, so preallocate for 4
        let byte_size = (Self::SIZE_IN_BITS - 1) / 8 + 1;
        let mut buffer: SmallBuffer<u8, 4> = SmallBuffer::new(byte_size, 0u8);

        // Values larger then a single element (32 bit) are left justified, not right
        let bit_slice = crate::datatype::FpgaBitsRaw::from_slice_mut(buffer.buffer());
        let bit_slice = if byte_size <= 4 {
            bit_slice.split_at_mut(byte_size * 8 - Self::SIZE_IN_BITS).1
        } else {
            bit_slice.split_at_mut(Self::SIZE_IN_BITS).0
        };

        DatatypePacker::pack(bit_slice, value.borrow())?;

        session.fpga().write_u8_array(offset, buffer.buffer())
    }
}

pub trait DerivedDatatype {}
impl<T, const N: usize> Datatype for [T; N] where T: DerivedDatatype + Datatype + std::fmt::Debug {}

// Support array versions of derived datatypes
impl<T, const N: usize> DatatypePacker for [T; N]
where
    T: DerivedDatatype + Datatype,
{
    const SIZE_IN_BITS: usize = T::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe { fpga_bits.chunks_mut(T::SIZE_IN_BITS).remove_alias() })
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

impl<const N: usize> DatatypePacker for [bool; N] {
    const SIZE_IN_BITS: usize = <bool as DatatypePacker>::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe {
                fpga_bits
                    .chunks_mut(<bool as DatatypePacker>::SIZE_IN_BITS)
                    .remove_alias()
            })
            .for_each(|(src, bits)| bits.set(0, *src));
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data = [bool::default(); N];
        data.iter_mut()
            .zip(fpga_bits.chunks(<bool as DatatypePacker>::SIZE_IN_BITS))
            .for_each(|(dest, bits)| *dest = bits[0]);
        Ok(data)
    }
}
impl<const N: usize> DatatypePacker for [u8; N] {
    const SIZE_IN_BITS: usize = <u8 as DatatypePacker>::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe {
                fpga_bits
                    .chunks_mut(<u8 as DatatypePacker>::SIZE_IN_BITS)
                    .remove_alias()
            })
            .for_each(|(src, bits)| bits.store_be(*src));
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data = [u8::default(); N];
        data.iter_mut()
            .zip(fpga_bits.chunks(<u8 as DatatypePacker>::SIZE_IN_BITS))
            .for_each(|(dest, bits)| *dest = bits.load_be());
        Ok(data)
    }
}

impl<const N: usize> DatatypePacker for [u16; N] {
    const SIZE_IN_BITS: usize = <u16 as DatatypePacker>::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe {
                fpga_bits
                    .chunks_mut(<u16 as DatatypePacker>::SIZE_IN_BITS)
                    .remove_alias()
            })
            .for_each(|(src, bits)| bits.store_be(*src));
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data = [u16::default(); N];
        data.iter_mut()
            .zip(fpga_bits.chunks(<u16 as DatatypePacker>::SIZE_IN_BITS))
            .for_each(|(dest, bits)| *dest = bits.load_be());
        Ok(data)
    }
}

impl<const N: usize> DatatypePacker for [u32; N] {
    const SIZE_IN_BITS: usize = <u32 as DatatypePacker>::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe {
                fpga_bits
                    .chunks_mut(<u32 as DatatypePacker>::SIZE_IN_BITS)
                    .remove_alias()
            })
            .for_each(|(src, bits)| bits.store_be(*src));
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data = [u32::default(); N];
        data.iter_mut()
            .zip(fpga_bits.chunks(<u32 as DatatypePacker>::SIZE_IN_BITS))
            .for_each(|(dest, bits)| *dest = bits.load_be());
        Ok(data)
    }
}

impl<const N: usize> DatatypePacker for [u64; N] {
    const SIZE_IN_BITS: usize = <u64 as DatatypePacker>::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe {
                fpga_bits
                    .chunks_mut(<u64 as DatatypePacker>::SIZE_IN_BITS)
                    .remove_alias()
            })
            .for_each(|(src, bits)| bits.store_be(*src));
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data = [u64::default(); N];
        data.iter_mut()
            .zip(fpga_bits.chunks(<u64 as DatatypePacker>::SIZE_IN_BITS))
            .for_each(|(dest, bits)| *dest = bits.load_be());
        Ok(data)
    }
}

impl<const N: usize> DatatypePacker for [i8; N] {
    const SIZE_IN_BITS: usize = <i8 as DatatypePacker>::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe {
                fpga_bits
                    .chunks_mut(<i8 as DatatypePacker>::SIZE_IN_BITS)
                    .remove_alias()
            })
            .for_each(|(src, bits)| bits.store_be(*src));
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data = [i8::default(); N];
        data.iter_mut()
            .zip(fpga_bits.chunks(<i8 as DatatypePacker>::SIZE_IN_BITS))
            .for_each(|(dest, bits)| *dest = bits.load_be());
        Ok(data)
    }
}

impl<const N: usize> DatatypePacker for [i16; N] {
    const SIZE_IN_BITS: usize = <i16 as DatatypePacker>::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe {
                fpga_bits
                    .chunks_mut(<i16 as DatatypePacker>::SIZE_IN_BITS)
                    .remove_alias()
            })
            .for_each(|(src, bits)| bits.store_be(*src));
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data = [i16::default(); N];
        data.iter_mut()
            .zip(fpga_bits.chunks(<i16 as DatatypePacker>::SIZE_IN_BITS))
            .for_each(|(dest, bits)| *dest = bits.load_be());
        Ok(data)
    }
}

impl<const N: usize> DatatypePacker for [i32; N] {
    const SIZE_IN_BITS: usize = <i32 as DatatypePacker>::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe {
                fpga_bits
                    .chunks_mut(<i32 as DatatypePacker>::SIZE_IN_BITS)
                    .remove_alias()
            })
            .for_each(|(src, bits)| bits.store_be(*src));
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data = [i32::default(); N];
        data.iter_mut()
            .zip(fpga_bits.chunks(<i32 as DatatypePacker>::SIZE_IN_BITS))
            .for_each(|(dest, bits)| *dest = bits.load_be());
        Ok(data)
    }
}

impl<const N: usize> DatatypePacker for [i64; N] {
    const SIZE_IN_BITS: usize = <i64 as DatatypePacker>::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe {
                fpga_bits
                    .chunks_mut(<i64 as DatatypePacker>::SIZE_IN_BITS)
                    .remove_alias()
            })
            .for_each(|(src, bits)| bits.store_be(*src));
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data = [i64::default(); N];
        data.iter_mut()
            .zip(fpga_bits.chunks(<i64 as DatatypePacker>::SIZE_IN_BITS))
            .for_each(|(dest, bits)| *dest = bits.load_be());
        Ok(data)
    }
}

impl<const N: usize> DatatypePacker for [f32; N] {
    const SIZE_IN_BITS: usize = <f32 as DatatypePacker>::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe {
                fpga_bits
                    .chunks_mut(<f32 as DatatypePacker>::SIZE_IN_BITS)
                    .remove_alias()
            })
            .for_each(|(src, bits)| bits.store_be(src.to_bits()));
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data = [f32::default(); N];
        data.iter_mut()
            .zip(fpga_bits.chunks(<f32 as DatatypePacker>::SIZE_IN_BITS))
            .for_each(|(dest, bits)| *dest = f32::from_bits(bits.load_be()));
        Ok(data)
    }
}

impl<const N: usize> DatatypePacker for [f64; N] {
    const SIZE_IN_BITS: usize = <f64 as DatatypePacker>::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe {
                fpga_bits
                    .chunks_mut(<f64 as DatatypePacker>::SIZE_IN_BITS)
                    .remove_alias()
            })
            .for_each(|(src, bits)| bits.store_be(src.to_bits()));
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data = [f64::default(); N];
        data.iter_mut()
            .zip(fpga_bits.chunks(<f64 as DatatypePacker>::SIZE_IN_BITS))
            .for_each(|(dest, bits)| *dest = f64::from_bits(bits.load_be()));
        Ok(data)
    }
}

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
