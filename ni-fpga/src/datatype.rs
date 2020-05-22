use bitvec::prelude::*;

#[cfg(target_endian="little")]
pub type FpgaBits = BitSlice<Msb0, u8>;
#[cfg(target_endian="big")]
pub type FpgaBits = BitSlice<Lsb0, u8>;

pub trait Datatype: Copy {
    const SIZE_IN_BITS: usize;

    fn pack(fpga_bits: &mut FpgaBits, data: Self);
    fn unpack(fpga_bits: &FpgaBits) -> Self;
}

// Support array versions of any Datatype
impl<T: Datatype, const N: usize> Datatype for [T; N] {
    const SIZE_IN_BITS: usize = T::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: Self) {
        data.iter().zip(fpga_bits.chunks_mut(T::SIZE_IN_BITS)).for_each(
            |(src, bits)|
            Datatype::pack(bits, *src)
        )
    }

    fn unpack(fpga_bits: &FpgaBits) -> Self {
        let mut data: [std::mem::MaybeUninit<T>; N] = std::mem::MaybeUninit::uninit_array();
        data.iter_mut().zip(fpga_bits.chunks(T::SIZE_IN_BITS)).for_each(
            |(dest, bits)|
            *dest = std::mem::MaybeUninit::new(Datatype::unpack(bits))
        );
        // This is hack until https://github.com/rust-lang/rust/issues/61956 is addressed
        let ptr = &mut data as *mut _ as *mut [T; N];
        let res = unsafe { ptr.read() };
        std::mem::forget(data);
        res
    }
}

impl Datatype for bool {
    const SIZE_IN_BITS: usize = 1;

    fn pack(fpga_bits: &mut FpgaBits, data: Self) {
        fpga_bits.set(0, data);
    }

    fn unpack(fpga_bits: &FpgaBits) -> Self {
        fpga_bits[0]
    }
}

impl Datatype for u8 {
    const SIZE_IN_BITS: usize = 8;

    fn pack(fpga_bits: &mut FpgaBits, data: Self) {
        fpga_bits.store_be::<Self>(data);
    }

    fn unpack(fpga_bits: &FpgaBits) -> Self {
        fpga_bits.load_be::<Self>()
    }
}

impl Datatype for u16 {
    const SIZE_IN_BITS: usize = 16;

    fn pack(fpga_bits: &mut FpgaBits, data: Self) {
        fpga_bits.store_be::<Self>(data);
    }

    fn unpack(fpga_bits: &FpgaBits) -> Self {
        fpga_bits.load_be::<Self>()
    }
}
