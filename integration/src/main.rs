use std::io::Write;

use colored::*;
use ni_fpga::fxp::{SignedFXP, UnsignedFXP};
use ni_fpga::Session;
use ni_fpga_macros::Cluster;
use tempfile::NamedTempFile;

#[derive(Cluster, Debug, PartialEq)]
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

#[allow(overflowing_literals)]
fn main() -> Result<(), ni_fpga::Error> {
    let mut tmp_bitfile = NamedTempFile::new().unwrap();
    write!(tmp_bitfile, include_str!("integration.lvbitx")).unwrap();

    let session = Session::open(
        tmp_bitfile.path().to_str().unwrap(),
        "D08F17F77A45A5692FA2342C6B86E0EE",
        "RIO0",
    )?;

    test_case("read plain U8", session.read::<u8>(98306)?, 0b00000001);
    test_case(
        "read plain U16",
        session.read::<u16>(98310)?,
        0b0000001100000001,
    );
    test_case(
        "read plain U32",
        session.read::<u32>(98312)?,
        0b00001111000001110000001100000001,
    );
    test_case(
        "read plain U64",
        session.read::<u64>(98316)?,
        0b1111111101111111001111110001111100001111000001110000001100000001,
    );

    test_case("read plain I8", session.read::<i8>(98322)?, 0b10000000);
    test_case(
        "read plain I16",
        session.read::<i16>(98326)?,
        0b1100000010000000,
    );
    test_case(
        "read plain I32",
        session.read::<i32>(98328)?,
        0b11110000111000001100000010000000,
    );
    test_case(
        "read plain I64",
        session.read::<i64>(98332)?,
        0b1111111111111110111111001111100011110000111000001100000010000000,
    );

    #[allow(clippy::approx_constant)]
    test_case("read SGL", session.read::<f32>(98336)?, 3.14);

    test_case(
        "read unsigned FXP",
        session.read::<UnsignedFXP<4, 3>>(98342)?,
        UnsignedFXP::from_float(4.5)?,
    );
    test_case(
        "read unsigned FXP",
        session.read::<SignedFXP<4, 3>>(98346)?,
        SignedFXP::from_float(-1.5)?,
    );

    test_case("read true bool", session.read::<bool>(98350)?, true);
    test_case("read false bool", session.read::<bool>(98354)?, false);
    test_case(
        "read bool array",
        session.read::<[bool; 8]>(98358)?,
        [true, false, true, false, true, false, true, false],
    );

    test_case(
        "read cluster",
        session.read::<TestCluster>(98360)?,
        TestCluster { b: false, u: 1337 },
    );
    // TODO: Investigate cluster array memory layout in order to fix this test.
    // The expected array may be incorrect here, I don't exactly remember what I used for the
    // fixture bitfile before my LabView FPGA trial expired.
    test_case(
        "read cluster array",
        session.read::<[TestCluster; 2]>(98360)?,
        [
            TestCluster { b: true, u: 255 },
            TestCluster { b: false, u: 1337 },
        ],
    );

    Ok(())
}
