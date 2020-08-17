use crate::Datatype;
use crate::Error;
use crate::FpgaBits;

fn fxp_to_float<const WORD_LENGTH: usize, const INTEGER_WORD_LENGTH: usize>(data: u64) -> f64 {
    if WORD_LENGTH > INTEGER_WORD_LENGTH {
        // FXP has a fractional part
        let fractional_length = WORD_LENGTH - INTEGER_WORD_LENGTH;
        let integer_part = (data >> fractional_length) as f64;
        let fractional_mask = (1 << fractional_length) - 1;
        let fractional_part = (data & fractional_mask) as f64 / (1 << fractional_length) as f64;
        integer_part + fractional_part
    } else {
        // FXP may have a delta > 1
        data as f64 * (1 << (INTEGER_WORD_LENGTH - WORD_LENGTH)) as f64
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UnsignedFXP<const WORD_LENGTH: usize, const INTEGER_WORD_LENGTH: usize>(pub u64);

impl<const WORD_LENGTH: usize, const INTEGER_WORD_LENGTH: usize> Datatype
    for UnsignedFXP<WORD_LENGTH, INTEGER_WORD_LENGTH>
{
    const SIZE_IN_BITS: usize = WORD_LENGTH;

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

impl<const WORD_LENGTH: usize, const INTEGER_WORD_LENGTH: usize>
    From<UnsignedFXP<WORD_LENGTH, INTEGER_WORD_LENGTH>> for f64
{
    fn from(fxp: UnsignedFXP<WORD_LENGTH, INTEGER_WORD_LENGTH>) -> Self {
        fxp_to_float::<WORD_LENGTH, INTEGER_WORD_LENGTH>(fxp.0)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SignedFXP<const WORD_LENGTH: usize, const INTEGER_WORD_LENGTH: usize>(pub u64);

impl<const WORD_LENGTH: usize, const INTEGER_WORD_LENGTH: usize> Datatype
    for SignedFXP<WORD_LENGTH, INTEGER_WORD_LENGTH>
{
    const SIZE_IN_BITS: usize = WORD_LENGTH;

    fn pack(fpga_bits: &mut FpgaBits, data: &Self) -> Result<(), Error> {
        if WORD_LENGTH > 32 {
            u64::pack(fpga_bits, &(*data).0)
        } else {
            u32::pack(fpga_bits, &((*data).0 as u32))
        }
    }

    fn unpack(fpga_bits: &FpgaBits) -> Result<Self, Error> {
        Ok(SignedFXP({
            if WORD_LENGTH > 32 {
                u64::unpack(fpga_bits)?
            } else {
                u32::unpack(fpga_bits)? as u64
            }
        }))
    }
}

impl<const WORD_LENGTH: usize, const INTEGER_WORD_LENGTH: usize>
    From<SignedFXP<WORD_LENGTH, INTEGER_WORD_LENGTH>> for f64
{
    fn from(fxp: SignedFXP<WORD_LENGTH, INTEGER_WORD_LENGTH>) -> Self {
        let sign_coefficient = {
            if fxp.0 & (1 << (WORD_LENGTH - 1)) != 0 {
                -1.0
            } else {
                1.0
            }
        };
        sign_coefficient
            * fxp_to_float::<WORD_LENGTH, INTEGER_WORD_LENGTH>(
                // Mask out sign bit
                fxp.0 & ((1 << (WORD_LENGTH - 1)) - 1),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_unsigned_fxp() {
        assert_eq!(f64::from(UnsignedFXP::<8, 8> { 0: 0b01111011 }), 123.0);
    }

    #[test]
    fn test_fractional_unsigned_fxp() {
        assert_eq!(f64::from(UnsignedFXP::<4, 2> { 0: 0b10_01 }), 2.25);
    }

    #[test]
    fn test_scaled_unsigned_fxp() {
        // 4x scaling factor
        assert_eq!(f64::from(UnsignedFXP::<4, 6> { 0: 0b1000 }), 32.0);
    }

    #[test]
    fn test_basic_signed_fxp() {
        assert_eq!(f64::from(SignedFXP::<8, 8> { 0: 0b0_1111011 }), 123.0);
        assert_eq!(f64::from(SignedFXP::<8, 8> { 0: 0b1_1111011 }), -123.0);
    }

    #[test]
    fn test_fractional_signed_fxp() {
        assert_eq!(f64::from(SignedFXP::<5, 3> { 0: 0b0_10_01 }), 2.25);
        assert_eq!(f64::from(SignedFXP::<5, 3> { 0: 0b1_10_01 }), -2.25);
    }

    #[test]
    fn test_scaled_signed_fxp() {
        // 4x scaling factor
        assert_eq!(f64::from(SignedFXP::<4, 7> { 0: 0b0_100 }), 32.0);
        assert_eq!(f64::from(SignedFXP::<4, 7> { 0: 0b1_100 }), -32.0);
    }
}
