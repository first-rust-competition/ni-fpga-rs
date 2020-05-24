use bitvec::prelude::*;

use crate::Error;

#[cfg(target_endian = "little")]
pub type FpgaBits = BitSlice<Msb0, u8>;
#[cfg(target_endian = "big")]
pub type FpgaBits = BitSlice<Lsb0, u8>;

pub trait Datatype: Sized {
    const SIZE_IN_BITS: usize;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error>;
    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error>;
}

// Support array versions of any Datatype
impl<T: Datatype, const N: usize> Datatype for [T; N] {
    const SIZE_IN_BITS: usize = T::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(fpga_bits.chunks_mut(T::SIZE_IN_BITS))
            .try_for_each(|(src, bits)| Datatype::pack(bits, src))
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data: [std::mem::MaybeUninit<T>; N] = std::mem::MaybeUninit::uninit_array();
        data.iter_mut()
            .zip(fpga_bits.chunks(T::SIZE_IN_BITS))
            .try_for_each::<_, Result<(), Error>>(|(dest, bits)| {
                *dest = std::mem::MaybeUninit::new(Datatype::unpack(bits)?);
                Ok(())
            })?;
        // This is hack until https://github.com/rust-lang/rust/issues/61956 is addressed
        let ptr = &mut data as *mut _ as *mut [T; N];
        let res = unsafe { ptr.read() };
        std::mem::forget(data);
        Ok(res)
    }
}

impl Datatype for bool {
    const SIZE_IN_BITS: usize = 1;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        fpga_bits.set(0, *data);
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(fpga_bits[0])
    }
}

impl Datatype for u8 {
    const SIZE_IN_BITS: usize = 8;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        fpga_bits.store_be::<Self>(*data);
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(fpga_bits.load_be::<Self>())
    }
}

impl Datatype for u16 {
    const SIZE_IN_BITS: usize = 16;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        fpga_bits.store_be::<Self>(*data);
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(fpga_bits.load_be::<Self>())
    }
}

impl Datatype for u32 {
    const SIZE_IN_BITS: usize = 32;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        fpga_bits.store_be::<Self>(*data);
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(fpga_bits.load_be::<Self>())
    }
}

impl Datatype for u64 {
    const SIZE_IN_BITS: usize = 64;

    fn pack(_fpga_bits: &mut FpgaBits, _data: &Self) -> Result<(), Error> {
        unimplemented!();
    }

    fn unpack(_fpga_bits: &FpgaBits) -> Result<Self, Error> {
        unimplemented!();
    }
}

impl Datatype for i8 {
    const SIZE_IN_BITS: usize = 8;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        u8::pack(fpga_bits, &(*data as u8))
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(u8::unpack(fpga_bits)? as Self)
    }
}

impl Datatype for i16 {
    const SIZE_IN_BITS: usize = 16;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        u16::pack(fpga_bits, &(*data as u16))
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(u16::unpack(fpga_bits)? as Self)
    }
}

impl Datatype for i32 {
    const SIZE_IN_BITS: usize = 32;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        u32::pack(fpga_bits, &(*data as u32))
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(u32::unpack(fpga_bits)? as Self)
    }
}

impl Datatype for i64 {
    const SIZE_IN_BITS: usize = 64;

    fn pack(_fpga_bits: &mut FpgaBits, _data: &Self) -> Result<(), Error> {
        unimplemented!();
    }

    fn unpack(_fpga_bits: &FpgaBits) -> Result<Self, Error> {
        unimplemented!();
    }
}
