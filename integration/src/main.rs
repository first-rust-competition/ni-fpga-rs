use bitvec::vec::BitVec;
use ni_fpga::{Error, RegisterRead, SessionAccess};

use colored::*;
use ni_fpga::fxp::{SignedFXP, UnsignedFXP};
use ni_fpga::Datatype;
use registers::FpgaBitfile;

mod registers;

fn test_case<T: PartialEq + std::fmt::Debug>(test_case_name: &str, actual: T, expected: T) {
    eprint!("{}...", test_case_name);
    if expected != actual {
        eprintln!(
            "{}: {:?} (actual) != {:?} (expected)",
            "failed".red(),
            actual,
            expected,
        );
    } else {
        eprintln!("{}", "passed".green());
    }
}

fn full_test_case<T: PartialEq + std::fmt::Debug + Datatype + Copy>(
    session: &impl SessionAccess,
    test_case_name: &str,
    reg: impl RegisterRead<T>,
    expected: T,
) -> Result<(), ni_fpga::Error> {
    test_case(test_case_name, reg.read(session)?, expected);
    round_trip_test(&expected)?;

    Ok(())
}

fn round_trip_test<T: Datatype + PartialEq + std::fmt::Debug>(data: &T) -> Result<(), Error> {
    let mut bv = BitVec::with_capacity(T::SIZE_IN_BITS);
    unsafe { bv.set_len(T::SIZE_IN_BITS) }

    let fpga_bits = bv.as_mut_bitslice();
    T::pack(fpga_bits, data)?;
    assert_eq!(T::unpack(fpga_bits)?, *data);
    Ok(())
}

#[allow(overflowing_literals)]
fn main() -> Result<(), ni_fpga::Error> {
    let session = FpgaBitfile::session_builder("rio://172.22.11.2/RIO0")?.build()?;
    let registers = FpgaBitfile::take(&session)?;

    full_test_case(&session, "read plain U8", registers.U8.unwrap(), 0b00000001)?;
    full_test_case(
        &session,
        "read plain U16",
        registers.U16.unwrap(),
        0b0000001100000001,
    )?;
    full_test_case(
        &session,
        "read plain U32",
        registers.U32.unwrap(),
        0b00001111000001110000001100000001,
    )?;
    full_test_case(
        &session,
        "read plain U64",
        registers.U64.unwrap(),
        0b1111111101111111001111110001111100001111000001110000001100000001,
    )?;

    full_test_case(&session, "read plain I8", registers.I8.unwrap(), 0b10000000)?;
    full_test_case(
        &session,
        "read plain I16",
        registers.I16.unwrap(),
        0b1100000010000000,
    )?;
    full_test_case(
        &session,
        "read plain I32",
        registers.I32.unwrap(),
        0b11110000111000001100000010000000,
    )?;
    full_test_case(
        &session,
        "read plain I64",
        registers.I64.unwrap(),
        0b1111111111111110111111001111100011110000111000001100000010000000,
    )?;

    #[allow(clippy::approx_constant)]
    full_test_case(&session, "read SGL", registers.SGL.unwrap(), 3.14)?;

    full_test_case(
        &session,
        "read unsigned FXP",
        registers.UnsignedFXP.unwrap(),
        UnsignedFXP::from_float(4.5)?,
    )?;
    full_test_case(
        &session,
        "read unsigned FXP",
        registers.SignedFXP.unwrap(),
        SignedFXP::from_float(-1.5)?,
    )?;

    full_test_case(
        &session,
        "read true bool",
        registers.TrueBool.unwrap(),
        true,
    )?;
    full_test_case(
        &session,
        "read false bool",
        registers.FalseBool.unwrap(),
        false,
    )?;
    full_test_case(
        &session,
        "read bool array",
        registers.BoolArray.unwrap(),
        [true, false, true, false, true, false, true, false],
    )?;

    full_test_case(
        &session,
        "read cluster",
        registers.TestCluster.unwrap(),
        registers::types::TestCluster { b: false, u: 1337 },
    )?;
    full_test_case(
        &session,
        "read cluster array",
        registers.TestClusterArray.unwrap(),
        [
            registers::types::TestClusterArray { b: true, u: 1234 },
            registers::types::TestClusterArray { b: false, u: 5678 },
        ],
    )?;

    Ok(())
}
