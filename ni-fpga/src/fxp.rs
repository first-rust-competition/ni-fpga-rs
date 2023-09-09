use crate::datatype::{Datatype, DatatypePacker, FpgaBits};
use crate::errors::Error;

use bitvec::prelude::*;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
pub struct FXP<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool>(u64);

pub type PackedNumber<const LENGTH: u8, const SIGNED: bool> = FXP<LENGTH, LENGTH, SIGNED>;
pub type SignedPackedNumber<const LENGTH: u8> = PackedNumber<LENGTH, true>;
pub type UnsignedPackedNumber<const LENGTH: u8> = PackedNumber<LENGTH, false>;

pub type SignedFXP<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> =
    FXP<WORD_LENGTH, INTEGER_LENGTH, true>;
pub type UnsignedFXP<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> =
    FXP<WORD_LENGTH, INTEGER_LENGTH, false>;

impl<const LENGTH: u8, const SIGNED: bool> FXP<LENGTH, LENGTH, SIGNED> {
    pub fn from_int(num: i64) -> Result<Self, Error> {
        if !SIGNED {
            if num >= (1 << LENGTH) || num < 0 {
                Err(Error::FixedPointOutOfBounds(
                    num as f64, LENGTH, LENGTH, SIGNED,
                ))
            } else {
                Ok(Self(num as u64))
            }
        } else if num >= (1 << (LENGTH - 1)) || num < -(1 << (LENGTH - 1)) {
            Err(Error::FixedPointOutOfBounds(
                num as f64, LENGTH, LENGTH, SIGNED,
            ))
        } else {
            let abs_bits = num.unsigned_abs();
            if num >= 0 {
                Ok(Self(abs_bits))
            } else {
                Ok(Self((abs_bits ^ Self::WORD_MASK) + 1))
            }
        }
    }

    pub fn to_int(&self) -> i64 {
        let mut bits = self.0;
        if SIGNED && self.0 & Self::SIGN_MASK != 0 {
            bits = (bits ^ Self::WORD_MASK) + 1
        }
        let result = bits as i64;
        if SIGNED && self.0 & Self::SIGN_MASK != 0 {
            -result
        } else {
            result
        }
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool>
    FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    const SCALING_FACTOR: i16 = WORD_LENGTH as i16 - INTEGER_LENGTH as i16;
    const WORD_MASK: u64 = (1 << WORD_LENGTH) - 1;
    const SIGN_MASK: u64 = 1 << (WORD_LENGTH - 1);

    pub fn from_raw(num: u64) -> Result<Self, Error> {
        if num >= (1 << WORD_LENGTH) {
            Err(Error::FixedPointRawOutOfBounds(
                num,
                WORD_LENGTH,
                INTEGER_LENGTH,
                SIGNED,
            ))
        } else {
            Ok(Self(num))
        }
    }

    fn from_float_unbounded(num: f64) -> Result<u64, Error> {
        let scaled = num.abs() * 2.0f64.powi(Self::SCALING_FACTOR as i32);
        if scaled.fract() != 0.0 {
            Err(Error::FixedPointPrecision(
                num,
                WORD_LENGTH,
                INTEGER_LENGTH,
                SIGNED,
            ))
        } else {
            Ok(scaled as u64)
        }
    }

    pub fn from_float(num: f64) -> Result<Self, Error> {
        if !SIGNED {
            if num >= (1 << INTEGER_LENGTH) as f64 || num < 0.0 {
                Err(Error::FixedPointOutOfBounds(
                    num,
                    WORD_LENGTH,
                    INTEGER_LENGTH,
                    SIGNED,
                ))
            } else {
                Ok(Self(Self::from_float_unbounded(num)?))
            }
        } else if num >= (1 << (INTEGER_LENGTH - 1)) as f64
            || num < -(1 << (INTEGER_LENGTH - 1)) as f64
        {
            Err(Error::FixedPointOutOfBounds(
                num,
                WORD_LENGTH,
                INTEGER_LENGTH,
                SIGNED,
            ))
        } else {
            let abs_bits = Self::from_float_unbounded(num)?;
            if num >= 0.0 {
                Ok(Self(abs_bits))
            } else {
                Ok(Self((abs_bits ^ Self::WORD_MASK) + 1))
            }
        }
    }

    pub fn to_float(&self) -> f64 {
        let mut bits = self.0;
        if SIGNED && self.0 & Self::SIGN_MASK != 0 {
            bits = (bits ^ Self::WORD_MASK) + 1
        }
        let result = bits as f64 / 2.0f64.powi(Self::SCALING_FACTOR as i32);
        if SIGNED && self.0 & Self::SIGN_MASK != 0 {
            -result
        } else {
            result
        }
    }

    fn masked(self) -> Self {
        Self(self.0 & Self::WORD_MASK)
    }

    // fn scale(bits: u128) -> u128 {
    //     if Self::SCALING_FACTOR >= 0 {
    //         bits << Self::SCALING_FACTOR as u8
    //     } else {
    //         bits >> -Self::SCALING_FACTOR as u8
    //     }
    // }

    fn unscale(bits: u128) -> u128 {
        if Self::SCALING_FACTOR >= 0 {
            bits >> Self::SCALING_FACTOR as u8
        } else {
            bits << -Self::SCALING_FACTOR as u8
        }
    }

    fn sign_extended(self) -> u128 {
        if !SIGNED || self.0 & Self::SIGN_MASK == 0 {
            self.0 as u128
        } else {
            self.0 as u128 | (u128::MAX - Self::WORD_MASK as u128)
        }
    }

    pub fn abs(self) -> Self {
        if !SIGNED || self.0 & Self::SIGN_MASK == 0 {
            self
        } else {
            Self((self.0 ^ Self::WORD_MASK) + 1)
        }
    }

    pub fn max_value() -> Self {
        if !SIGNED {
            Self(Self::WORD_MASK)
        } else {
            Self(Self::WORD_MASK ^ Self::SIGN_MASK)
        }
    }

    pub fn min_value() -> Self {
        if !SIGNED {
            Self(0)
        } else {
            Self(Self::SIGN_MASK)
        }
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::ops::Add
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0.add(other.0)).masked()
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::ops::BitAnd
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self(self.0.bitand(other.0))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::ops::BitOr
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0.bitor(other.0))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::ops::BitXor
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self(self.0.bitxor(other.0))
    }
}

// TODO: Implement Div and make tests pass.
// This routine only works for unsigned integers.
//
// impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::ops::Div
//     for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
// {
//     type Output = Self;
//
//     fn div(self, other: Self) -> Self {
//         Self((Self::scale(self.0 as u128).div(other.0 as u128)) as u64).masked()
//     }
// }

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::ops::Mul
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(Self::unscale(self.sign_extended().wrapping_mul(other.sign_extended())) as u64)
            .masked()
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::ops::Rem
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        if SIGNED && self.0 & Self::SIGN_MASK != 0 {
            Self((self.0.rem(other.abs().0) ^ Self::WORD_MASK) + 1)
        } else {
            Self(self.0.rem(other.abs().0))
        }
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::ops::Shl<u8>
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    type Output = Self;

    fn shl(self, other: u8) -> Self {
        Self(self.0.shl(other)).masked()
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::ops::Shr<u8>
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    type Output = Self;

    fn shr(self, other: u8) -> Self {
        if !SIGNED {
            Self(self.0.shr(other))
        } else {
            // arithmetic shift
            Self((self.0 & Self::SIGN_MASK) | self.0.shr(other))
        }
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::ops::Sub
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.wrapping_sub(other.0)).masked()
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::cmp::PartialEq
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::cmp::Eq
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::cmp::PartialOrd
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::cmp::Ord
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if SIGNED && self.0 & Self::SIGN_MASK != other.0 & Self::SIGN_MASK {
            (other.0 & Self::SIGN_MASK).cmp(&(self.0 & Self::SIGN_MASK))
        } else {
            self.0.cmp(&other.0)
        }
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::fmt::Debug
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "FXP<word={:?}, integer={:?}, signed={:?}>({:?})",
            WORD_LENGTH,
            INTEGER_LENGTH,
            SIGNED,
            self.to_float(),
        ))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> std::fmt::Display
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.to_float(),))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> DatatypePacker
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
    const SIZE_IN_BITS: usize = WORD_LENGTH as usize;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        if WORD_LENGTH > 32 {
            u64::pack(fpga_bits, &(data.0))
        } else {
            u32::pack(fpga_bits, &(data.0 as u32))
        }
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(FXP({
            if WORD_LENGTH > 32 {
                u64::unpack(fpga_bits)?
            } else {
                u32::unpack(fpga_bits)? as u64
            }
        }))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool> Datatype
    for FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>
{
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool, const N: usize>
    DatatypePacker for [FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>; N]
{
    const SIZE_IN_BITS: usize =
        <FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED> as DatatypePacker>::SIZE_IN_BITS * N;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        data.iter()
            .zip(unsafe {
                fpga_bits
                    .chunks_mut(<bool as DatatypePacker>::SIZE_IN_BITS)
                    .remove_alias()
            })
            .for_each(|(src, bits)| {
                if WORD_LENGTH > 32 {
                    bits.store_be(src.0)
                } else {
                    bits.store_be(src.0 as u32)
                }
            });
        Ok(())
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        let mut data = [FXP(0); N];
        data.iter_mut()
            .zip(fpga_bits.chunks(<bool as DatatypePacker>::SIZE_IN_BITS))
            .for_each(|(dest, bits)| {
                if WORD_LENGTH > 32 {
                    dest.0 = bits.load_be();
                } else {
                    dest.0 = bits.load_be::<u32>() as u64
                }
            });
        Ok(data)
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8, const SIGNED: bool, const N: usize> Datatype
    for [FXP<WORD_LENGTH, INTEGER_LENGTH, SIGNED>; N]
{
}

#[cfg(test)]
mod tests {
    use super::*;

    // Binary constants use the following notation:
    // For FXPs with resolution less than 1, an underscore will follow the intergral part.
    // For FXPs with resolution greater than 1, (integer length - word length) underscores
    // will follow the integral part. For example, FXP<3, 6>(0b110___) == 0b110000.

    #[test]
    fn test_from_int() -> Result<(), Error> {
        for i in 0..=255 {
            assert_eq!(FXP::<8, 8, false>::from_int(i)?.0, i as u64);
        }
        for i in 0..=127 {
            assert_eq!(FXP::<8, 8, true>::from_int(i)?.0, i as u64);
        }
        Ok(())
    }

    #[test]
    fn test_to_int() -> Result<(), Error> {
        for i in 0..=255 {
            assert_eq!(FXP::<8, 8, false>::from_int(i)?.to_int(), i);
        }
        for i in -128..=127 {
            assert_eq!(FXP::<8, 8, true>::from_int(i)?.to_int(), i);
        }
        Ok(())
    }

    #[test]
    fn test_from_raw() -> Result<(), Error> {
        for i in 0..=0b11111111 {
            assert_eq!(FXP::<8, 8, false>::from_raw(i)?.0, i);
            assert_eq!(FXP::<8, 8, true>::from_raw(i)?.0, i);
        }
        Ok(())
    }

    #[test]
    fn test_from_raw_invalid_out_of_bounds() {
        assert_eq!(
            FXP::<8, 8, false>::from_raw(0b100000000),
            Err(Error::FixedPointRawOutOfBounds(0b100000000, 8, 8, false)),
        );
    }

    #[test]
    fn test_from_float() -> Result<(), Error> {
        // resolution = 1
        assert_eq!(
            FXP::<8, 8, false>::from_float(123.0)?,
            FXP::from_raw(0b01111011)?,
        );
        assert_eq!(
            FXP::<8, 8, true>::from_float(123.0)?,
            FXP::from_raw(0b0_1111011)?,
        );
        assert_eq!(
            FXP::<8, 8, true>::from_float(-123.0)?,
            FXP::from_raw(0b1_0000101)?,
        );
        // resolution = 0.5
        assert_eq!(
            FXP::<4, 3, false>::from_float(2.5)?,
            FXP::from_raw(0b010_1)?,
        );
        assert_eq!(
            FXP::<4, 3, true>::from_float(2.5)?,
            FXP::from_raw(0b0_10_1)?,
        );
        assert_eq!(
            FXP::<4, 3, true>::from_float(-2.5)?,
            FXP::from_raw(0b1_01_1)?,
        );
        // resolution = 8
        assert_eq!(
            FXP::<3, 6, true>::from_float(24.0)?,
            FXP::from_raw(0b011___)?,
        );
        assert_eq!(
            FXP::<3, 6, true>::from_float(24.0)?,
            FXP::from_raw(0b0_11___)?,
        );
        assert_eq!(
            FXP::<3, 6, true>::from_float(-24.0)?,
            FXP::from_raw(0b1_01___)?,
        );
        Ok(())
    }

    #[test]
    fn test_from_float_invalid_out_of_bounds() {
        assert_eq!(
            FXP::<8, 8, false>::from_float(-1.0),
            Err(Error::FixedPointOutOfBounds(-1.0, 8, 8, false)),
        );
        assert_eq!(
            FXP::<8, 8, false>::from_float(256.0),
            Err(Error::FixedPointOutOfBounds(256.0, 8, 8, false)),
        );
        assert_eq!(
            FXP::<8, 8, true>::from_float(-129.0),
            Err(Error::FixedPointOutOfBounds(-129.0, 8, 8, true)),
        );
        assert_eq!(
            FXP::<8, 8, true>::from_float(128.0),
            Err(Error::FixedPointOutOfBounds(128.0, 8, 8, true)),
        );
    }

    #[test]
    fn test_from_float_invalid_bad_fract() {
        // resolution = 1
        assert_eq!(
            FXP::<8, 8, false>::from_float(1.1),
            Err(Error::FixedPointPrecision(1.1, 8, 8, false)),
        );
        assert_eq!(
            FXP::<8, 8, true>::from_float(-1.1),
            Err(Error::FixedPointPrecision(-1.1, 8, 8, true)),
        );
        // resolution = 0.5
        assert_eq!(
            FXP::<4, 3, false>::from_float(2.25),
            Err(Error::FixedPointPrecision(2.25, 4, 3, false)),
        );
        assert_eq!(
            FXP::<4, 3, true>::from_float(-2.25),
            Err(Error::FixedPointPrecision(-2.25, 4, 3, true)),
        );
    }

    #[test]
    fn test_debug_impl() -> Result<(), Error> {
        // resolution = 1
        assert_eq!(
            format!("{:?}", FXP::<8, 8, false>::from_float(123.0)?),
            "FXP<word=8, integer=8, signed=false>(123.0)",
        );
        assert_eq!(
            format!("{:?}", FXP::<8, 8, true>::from_float(-123.0)?),
            "FXP<word=8, integer=8, signed=true>(-123.0)",
        );
        // resolution = 0.5
        assert_eq!(
            format!("{:?}", FXP::<4, 3, false>::from_float(2.5)?),
            "FXP<word=4, integer=3, signed=false>(2.5)",
        );
        assert_eq!(
            format!("{:?}", FXP::<4, 3, true>::from_float(-2.5)?),
            "FXP<word=4, integer=3, signed=true>(-2.5)",
        );
        // resolution = 8
        assert_eq!(
            format!("{:?}", FXP::<3, 6, false>::from_float(24.0)?),
            "FXP<word=3, integer=6, signed=false>(24.0)",
        );
        assert_eq!(
            format!("{:?}", FXP::<3, 6, true>::from_float(-24.0)?),
            "FXP<word=3, integer=6, signed=true>(-24.0)",
        );
        Ok(())
    }

    #[test]
    fn test_add() -> Result<(), Error> {
        assert_eq!(
            FXP::<8, 8, false>::from_float(1.0)? + FXP::<8, 8, false>::from_float(1.0)?,
            FXP::<8, 8, false>::from_float(2.0)?,
        );
        assert_eq!(
            FXP::<8, 8, true>::from_float(-1.0)? + FXP::<8, 8, true>::from_float(-1.0)?,
            FXP::<8, 8, true>::from_float(-2.0)?,
        );
        // overflows are masked
        assert_eq!(
            (FXP::<8, 8, false>::from_float(128.0)? + FXP::<8, 8, false>::from_float(128.0)?).0,
            0,
        );
        assert_eq!(
            (FXP::<8, 8, true>::from_float(-128.0)? + FXP::<8, 8, true>::from_float(-128.0)?).0,
            0,
        );
        Ok(())
    }

    #[test]
    fn test_bit_ops() -> Result<(), Error> {
        assert_eq!(
            FXP::<8, 8, false>::from_float(0.0)? & FXP::<8, 8, false>::from_float(1.0)?,
            FXP::<8, 8, false>::from_float(0.0)?,
        );
        assert_eq!(
            FXP::<8, 8, false>::from_float(0.0)? | FXP::<8, 8, false>::from_float(1.0)?,
            FXP::<8, 8, false>::from_float(1.0)?,
        );
        assert_eq!(
            FXP::<8, 8, false>::from_float(1.0)? ^ FXP::<8, 8, false>::from_float(1.0)?,
            FXP::<8, 8, false>::from_float(0.0)?,
        );
        assert_eq!(
            FXP::<8, 8, false>::from_float(1.0)? << 1,
            FXP::<8, 8, false>::from_float(2.0)?,
        );
        // overflows are masked
        assert_eq!((FXP::<8, 8, false>::from_raw(0b10000000)? << 1).0, 0);
        assert_eq!(
            FXP::<8, 8, false>::from_float(2.0)? >> 1,
            FXP::<8, 8, false>::from_float(1.0)?,
        );
        // signed right shifts are arithmetic
        assert_eq!(
            FXP::<8, 8, true>::from_float(-128.0)? >> 1,
            FXP::<8, 8, true>::from_float(-64.0)?,
        );
        Ok(())
    }

    // TODO: Implement Div and make tests pass.
    //
    // #[test]
    // fn test_div() -> Result<(), Error> {
    //     assert_eq!(
    //         FXP::<8, 7, false>::from_float(2.5)? / FXP::<8, 7, false>::from_float(0.5)?,
    //         FXP::<8, 7, false>::from_float(5.0)?,
    //     );
    //     assert_eq!(
    //         FXP::<8, 7, true>::from_float(2.5)? / FXP::<8, 7, true>::from_float(-0.5)?,
    //         FXP::<8, 7, true>::from_float(-5.0)?,
    //     );
    //     // overflows are masked
    //     assert_eq!(
    //         (FXP::<9, 8, false>::from_float(128.0)? / FXP::<9, 8, false>::from_float(0.5)?).0,
    //         0,
    //     );
    //     Ok(())
    // }

    #[test]
    fn test_mul() -> Result<(), Error> {
        assert_eq!(
            FXP::<8, 7, false>::from_float(5.0)? * FXP::<8, 7, false>::from_float(0.5)?,
            FXP::<8, 7, false>::from_float(2.5)?,
        );
        assert_eq!(
            FXP::<8, 7, true>::from_float(5.0)? * FXP::<8, 7, true>::from_float(-0.5)?,
            FXP::<8, 7, true>::from_float(-2.5)?,
        );
        assert_eq!(
            FXP::<8, 7, true>::from_float(-5.0)? * FXP::<8, 7, true>::from_float(-0.5)?,
            FXP::<8, 7, true>::from_float(2.5)?,
        );
        // overflows are masked
        assert_eq!(
            (FXP::<8, 8, false>::from_float(128.0)? * FXP::<8, 8, false>::from_float(2.0)?).0,
            0,
        );
        assert_eq!(
            (FXP::<8, 8, true>::from_float(-128.0)? * FXP::<8, 8, true>::from_float(2.0)?).0,
            0,
        );
        Ok(())
    }

    #[test]
    fn test_rem() -> Result<(), Error> {
        assert_eq!(
            FXP::<8, 6, false>::from_float(1.25)? % FXP::<8, 6, false>::from_float(0.5)?,
            FXP::<8, 6, false>::from_float(0.25)?,
        );
        assert_eq!(
            FXP::<8, 6, true>::from_float(-1.25)? % FXP::<8, 6, true>::from_float(0.5)?,
            FXP::<8, 6, true>::from_float(-0.25)?,
        );
        assert_eq!(
            FXP::<8, 6, true>::from_float(1.25)? % FXP::<8, 6, true>::from_float(-0.5)?,
            FXP::<8, 6, true>::from_float(0.25)?,
        );
        Ok(())
    }

    #[test]
    fn test_sub() -> Result<(), Error> {
        assert_eq!(
            FXP::<8, 8, false>::from_float(2.0)? - FXP::<8, 8, false>::from_float(1.0)?,
            FXP::<8, 8, false>::from_float(1.0)?,
        );
        assert_eq!(
            FXP::<8, 8, true>::from_float(-2.0)? - FXP::<8, 8, true>::from_float(-1.0)?,
            FXP::<8, 8, true>::from_float(-1.0)?,
        );
        Ok(())
    }

    #[test]
    fn test_cmp() -> Result<(), Error> {
        assert_eq!(
            FXP::<8, 8, false>::from_float(1.0)?,
            FXP::<8, 8, false>::from_float(1.0)?,
        );
        assert_eq!(
            FXP::<8, 8, true>::from_float(-1.0)?,
            FXP::<8, 8, true>::from_float(-1.0)?,
        );
        assert!(FXP::<8, 8, false>::from_float(1.0)? > FXP::<8, 8, false>::from_float(0.0)?);
        assert!(FXP::<8, 8, false>::from_float(0.0)? < FXP::<8, 8, false>::from_float(1.0)?);
        assert!(FXP::<8, 8, true>::from_float(-2.0)? < FXP::<8, 8, true>::from_float(-1.0)?);
        assert!(FXP::<8, 8, true>::from_float(-1.0)? < FXP::<8, 8, true>::from_float(0.0)?);
        assert!(FXP::<8, 8, true>::from_float(0.0)? < FXP::<8, 8, true>::from_float(1.0)?);
        assert!(FXP::<8, 8, true>::from_float(1.0)? > FXP::<8, 8, true>::from_float(0.0)?);
        assert!(FXP::<8, 8, true>::from_float(0.0)? > FXP::<8, 8, true>::from_float(-1.0)?);
        assert!(FXP::<8, 8, true>::from_float(-1.0)? > FXP::<8, 8, true>::from_float(-2.0)?);
        Ok(())
    }
}
