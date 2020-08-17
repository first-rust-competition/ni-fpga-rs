use crate::Datatype;
use crate::Error;
use crate::FpgaBits;

pub struct UnsignedFXP<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8>(pub(crate) u64);

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> Datatype
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    const SIZE_IN_BITS: usize = WORD_LENGTH as usize;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        if WORD_LENGTH > 32 {
            u64::pack(fpga_bits, &(*data).0)
        } else {
            u32::pack(fpga_bits, &((*data).0 as u32))
        }
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(UnsignedFXP({
            if WORD_LENGTH > 32 {
                u64::unpack(fpga_bits)?
            } else {
                u32::unpack(fpga_bits)? as u64
            }
        }))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH> {
    const SCALING_FACTOR: i16 = WORD_LENGTH as i16 - INTEGER_LENGTH as i16;

    pub fn from_raw(num: u64) -> Result<Self, Error> {
        if num >= (1 << WORD_LENGTH) {
            Err(Error::InvalidUnsignedFXPError(
                num as f64,
                WORD_LENGTH,
                INTEGER_LENGTH,
            ))
        } else {
            Ok(Self(num))
        }
    }

    pub fn from_float(num: f64) -> Result<Self, Error> {
        if num >= (1 << INTEGER_LENGTH) as f64 || num < 0.0 {
            Err(Error::InvalidUnsignedFXPError(
                num,
                WORD_LENGTH,
                INTEGER_LENGTH,
            ))
        } else {
            let scaled = num * 2.0f64.powi(Self::SCALING_FACTOR as i32);
            if scaled.fract() != 0.0 {
                Err(Error::InvalidUnsignedFXPError(
                    num,
                    WORD_LENGTH,
                    INTEGER_LENGTH,
                ))
            } else {
                Ok(Self(scaled as u64))
            }
        }
    }

    fn mask(bits: u64) -> u64 {
        bits & ((1 << WORD_LENGTH) - 1)
    }

    fn scale(bits: u128) -> u128 {
        if Self::SCALING_FACTOR >= 0 {
            bits << Self::SCALING_FACTOR as u8
        } else {
            bits >> -Self::SCALING_FACTOR as u8
        }
    }

    fn unscale(bits: u128) -> u128 {
        if Self::SCALING_FACTOR >= 0 {
            bits >> Self::SCALING_FACTOR as u8
        } else {
            bits << -Self::SCALING_FACTOR as u8
        }
    }

    fn mul(lhs: u64, rhs: u64) -> u64 {
        Self::unscale(lhs as u128 * rhs as u128) as u64
    }

    fn div(lhs: u64, rhs: u64) -> u64 {
        (Self::scale(lhs as u128) / rhs as u128) as u64
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::ops::Add
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(Self::mask(self.0.add(other.0)))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::ops::BitAnd
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self(self.0.bitand(other.0))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::ops::BitOr
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0.bitor(other.0))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::ops::BitXor
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self(self.0.bitxor(other.0))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::ops::Div
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(Self::mask(Self::div(self.0, other.0)))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::ops::Mul
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(Self::mask(Self::mul(self.0, other.0)))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::ops::Rem
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self(self.0.rem(other.0))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::ops::Shl<u8>
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    type Output = Self;

    fn shl(self, other: u8) -> Self {
        Self(Self::mask(self.0.shl(other)))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::ops::Shr<u8>
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    type Output = Self;

    fn shr(self, other: u8) -> Self {
        Self(self.0.shr(other))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::ops::Sub
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::cmp::PartialEq
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::cmp::Eq
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::cmp::PartialOrd
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::cmp::Ord
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<const WORD_LENGTH: u8, const INTEGER_LENGTH: u8> std::fmt::Debug
    for UnsignedFXP<WORD_LENGTH, INTEGER_LENGTH>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut component = 2.0f64.powf(INTEGER_LENGTH as f64 - WORD_LENGTH as f64);
        let mut result = 0.0;
        for i in 0..WORD_LENGTH {
            if self.0 & (1 << i) != 0 {
                result += component;
            }
            component *= 2.0;
        }
        f.write_fmt(format_args!(
            "UnsignedFXP<{:?}, {:?}>({:?})",
            WORD_LENGTH, INTEGER_LENGTH, result
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Binary constants use the following notation:
    // For FXPs with resolution less than 1, an underscore will follow the intergral part.
    // For FXPs with resolution greater than 1, (integer length - word length) underscores
    // will follow the integral part. For example, FXP<3, 6>(0b110___) == 0b110000.

    #[test]
    fn test_from_raw() -> Result<(), Error> {
        assert_eq!(UnsignedFXP::<8, 8>::from_raw(0b01111011)?.0, 0b01111011,);
        Ok(())
    }

    #[test]
    fn test_from_raw_invalid_out_of_bounds() {
        assert_eq!(
            UnsignedFXP::<8, 8>::from_raw(0b100000000),
            Err(Error::InvalidUnsignedFXPError(256.0, 8, 8)),
        );
    }

    #[test]
    fn test_from_float() -> Result<(), Error> {
        // resolution = 1
        assert_eq!(
            UnsignedFXP::<8, 8>::from_float(123.0)?,
            UnsignedFXP::from_raw(0b01111011)?,
        );
        // resolution = 0.5
        assert_eq!(
            UnsignedFXP::<4, 3>::from_float(2.5)?,
            UnsignedFXP::from_raw(0b010_1)?,
        );
        // resolution = 8
        assert_eq!(
            UnsignedFXP::<3, 6>::from_float(48.0)?,
            UnsignedFXP::from_raw(0b110___)?,
        );

        Ok(())
    }

    #[test]
    fn test_from_float_invalid_out_of_bounds() {
        assert_eq!(
            UnsignedFXP::<8, 8>::from_float(-1.0),
            Err(Error::InvalidUnsignedFXPError(-1.0, 8, 8)),
        );
        assert_eq!(
            UnsignedFXP::<3, 6>::from_float(64.0),
            Err(Error::InvalidUnsignedFXPError(64.0, 3, 6)),
        );
    }

    #[test]
    fn test_from_float_invalid_bad_fract() {
        // resolution = 1
        assert_eq!(
            UnsignedFXP::<8, 8>::from_float(1.1),
            Err(Error::InvalidUnsignedFXPError(1.1, 8, 8)),
        );
        // resolution = 0.5
        assert_eq!(
            UnsignedFXP::<4, 3>::from_float(2.25),
            Err(Error::InvalidUnsignedFXPError(2.25, 4, 3)),
        );
    }

    #[test]
    fn test_debug_impl() -> Result<(), Error> {
        assert_eq!(
            format!("{:?}", UnsignedFXP::<8, 8>::from_float(123.0)?),
            "UnsignedFXP<8, 8>(123.0)",
        );
        assert_eq!(
            format!("{:?}", UnsignedFXP::<4, 3>::from_float(2.5)?),
            "UnsignedFXP<4, 3>(2.5)",
        );
        assert_eq!(
            format!("{:?}", UnsignedFXP::<3, 6>::from_float(48.0)?),
            "UnsignedFXP<3, 6>(48.0)",
        );
        Ok(())
    }

    #[test]
    fn test_add() -> Result<(), Error> {
        assert_eq!(
            UnsignedFXP::<8, 8>::from_float(1.0)? + UnsignedFXP::<8, 8>::from_float(1.0)?,
            UnsignedFXP::<8, 8>::from_float(2.0)?,
        );
        // overflows are masked
        assert_eq!(
            (UnsignedFXP::<8, 8>::from_float(128.0)? + UnsignedFXP::<8, 8>::from_float(128.0)?).0,
            0,
        );
        Ok(())
    }

    #[test]
    fn test_bit_ops() -> Result<(), Error> {
        assert_eq!(
            UnsignedFXP::<8, 8>::from_float(1.0)? & UnsignedFXP::<8, 8>::from_float(1.0)?,
            UnsignedFXP::<8, 8>::from_float(1.0)?,
        );
        assert_eq!(
            UnsignedFXP::<8, 8>::from_float(0.0)? | UnsignedFXP::<8, 8>::from_float(1.0)?,
            UnsignedFXP::<8, 8>::from_float(1.0)?,
        );
        assert_eq!(
            UnsignedFXP::<8, 8>::from_float(1.0)? ^ UnsignedFXP::<8, 8>::from_float(1.0)?,
            UnsignedFXP::<8, 8>::from_float(0.0)?,
        );
        assert_eq!(
            UnsignedFXP::<8, 8>::from_float(1.0)? << 1,
            UnsignedFXP::<8, 8>::from_float(2.0)?,
        );
        // overflows are masked
        assert_eq!((UnsignedFXP::<8, 8>::from_raw(0b10000000)? << 1).0, 0,);
        assert_eq!(
            UnsignedFXP::<8, 8>::from_float(2.0)? >> 1,
            UnsignedFXP::<8, 8>::from_float(1.0)?,
        );
        Ok(())
    }

    #[test]
    fn test_div() -> Result<(), Error> {
        assert_eq!(
            UnsignedFXP::<8, 7>::from_float(2.5)? / UnsignedFXP::<8, 7>::from_float(0.5)?,
            UnsignedFXP::<8, 7>::from_float(5.0)?,
        );
        // overflows are masked
        assert_eq!(
            (UnsignedFXP::<9, 8>::from_float(128.0)? / UnsignedFXP::<9, 8>::from_float(0.5)?).0,
            0,
        );
        Ok(())
    }

    #[test]
    fn test_mul() -> Result<(), Error> {
        assert_eq!(
            UnsignedFXP::<8, 7>::from_float(5.0)? * UnsignedFXP::<8, 7>::from_float(0.5)?,
            UnsignedFXP::<8, 7>::from_float(2.5)?,
        );
        // overflows are masked
        assert_eq!(
            (UnsignedFXP::<8, 8>::from_float(128.0)? * UnsignedFXP::<8, 8>::from_float(2.0)?).0,
            0,
        );
        Ok(())
    }

    #[test]
    fn test_rem() -> Result<(), Error> {
        assert_eq!(
            UnsignedFXP::<8, 6>::from_float(1.25)? % UnsignedFXP::<8, 6>::from_float(0.5)?,
            UnsignedFXP::<8, 6>::from_float(0.25)?,
        );
        Ok(())
    }

    #[test]
    fn test_sub() -> Result<(), Error> {
        assert_eq!(
            UnsignedFXP::<8, 8>::from_float(1.0)? - UnsignedFXP::<8, 8>::from_float(1.0)?,
            UnsignedFXP::<8, 8>::from_float(0.0)?,
        );
        Ok(())
    }

    #[test]
    fn test_cmp() -> Result<(), Error> {
        assert_eq!(
            UnsignedFXP::<8, 8>::from_float(1.0)?,
            UnsignedFXP::<8, 8>::from_float(1.0)?,
        );
        assert!(UnsignedFXP::<8, 8>::from_float(1.0)? > UnsignedFXP::<8, 8>::from_float(0.0)?);
        assert!(UnsignedFXP::<8, 8>::from_float(0.0)? < UnsignedFXP::<8, 8>::from_float(1.0)?);
        Ok(())
    }
}
