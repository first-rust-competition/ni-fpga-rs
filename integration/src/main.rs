use ni_fpga::{RegisterAccess, SessionAccess};
use std::io::Write;

use colored::*;
use ni_fpga::fxp::{SignedFXP, UnsignedFXP};
use ni_fpga::{Datatype, Session};
use ni_fpga_macros::Cluster;
use tempfile::NamedTempFile;

#[derive(Cluster, Debug, PartialEq, Clone, Copy)]
struct TestCluster {
    b: bool,
    u: u16,
}

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

fn full_test_case<T: PartialEq + std::fmt::Debug + Datatype + Copy, const N: u32>(
    session: &impl SessionAccess,
    test_case_name: &str,
    expected: T,
) -> Result<(), ni_fpga::Error> {
    test_case(test_case_name, session.read::<T>(N)?, expected);
    let reg = session.open_register::<T>(N);
    test_case(
        &format!("{} ref", test_case_name),
        reg.read(session)?,
        expected,
    );
    let reg = session.open_const_register::<T, N>();
    test_case(
        &format!("{} const ref", test_case_name),
        reg.read(session)?,
        expected,
    );

    Ok(())
}

#[allow(overflowing_literals)]
fn main() -> Result<(), ni_fpga::Error> {
    let mut tmp_bitfile = NamedTempFile::new().unwrap();
    write!(tmp_bitfile, include_str!("integration.lvbitx")).unwrap();

    let session = Session::open(
        tmp_bitfile.path().to_str().unwrap(),
        "D08F17F77A45A5692FA2342C6B86E0EE",
        "rio://172.22.11.2/RIO0",
    )?;

    full_test_case::<u8, 98306>(&session, "read plain U8", 0b00000001)?;
    full_test_case::<u16, 98310>(&session, "read plain U16", 0b0000001100000001)?;
    full_test_case::<u32, 98312>(
        &session,
        "read plain U32",
        0b00001111000001110000001100000001,
    )?;
    full_test_case::<u64, 98316>(
        &session,
        "read plain U64",
        0b1111111101111111001111110001111100001111000001110000001100000001,
    )?;

    full_test_case::<i8, 98322>(&session, "read plain I8", 0b10000000)?;
    full_test_case::<i16, 98326>(&session, "read plain I16", 0b1100000010000000)?;
    full_test_case::<i32, 98328>(
        &session,
        "read plain I32",
        0b11110000111000001100000010000000,
    )?;
    full_test_case::<i64, 98332>(
        &session,
        "read plain I64",
        0b1111111111111110111111001111100011110000111000001100000010000000,
    )?;

    #[allow(clippy::approx_constant)]
    full_test_case::<f32, 98336>(&session, "read SGL", 3.14)?;

    full_test_case::<UnsignedFXP<4, 3>, 98342>(
        &session,
        "read unsigned FXP",
        UnsignedFXP::from_float(4.5)?,
    )?;
    full_test_case::<SignedFXP<4, 3>, 98346>(
        &session,
        "read unsigned FXP",
        SignedFXP::from_float(-1.5)?,
    )?;

    full_test_case::<bool, 98350>(&session, "read true bool", true)?;
    full_test_case::<bool, 98354>(&session, "read false bool", false)?;
    full_test_case::<[bool; 8], 98358>(
        &session,
        "read bool array",
        [true, false, true, false, true, false, true, false],
    )?;

    full_test_case::<TestCluster, 98360>(
        &session,
        "read cluster",
        TestCluster { b: false, u: 1337 },
    )?;
    full_test_case::<[TestCluster; 2], 98364>(
        &session,
        "read cluster array",
        [
            TestCluster { b: true, u: 1234 },
            TestCluster { b: false, u: 5678 },
        ],
    )?;

    Ok(())
}
