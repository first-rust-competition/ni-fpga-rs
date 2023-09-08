use std::{thread, ffi::CString};
use std::io::Write;

use ni_fpga::fxp::{FXP, UnsignedFXP};
use ni_fpga::{Register, RegisterAccess, Session, StoredOffset, SessionAccess};
use ni_fpga_macros::{Cluster, Enum};
use tempfile::NamedTempFile;

#[derive(Cluster, Debug)]
struct DutyCycleFrequency {
    pub overflow: bool,
    pub frequency: UnsignedFXP<11, 11>,

}

#[derive(Cluster, Debug)]
struct PWMConfig {
    period: u16,
    min_high: u16,
}
#[derive(Cluster, Debug)]
struct AnalogTriggerOutput {
    in_hysteresis: bool,
    over_limit: bool,
    rising: bool,
    falling: bool,
}

#[derive(Enum, Debug, Copy, Clone, Default)]
enum SPIDebugState {
    #[default]
    Idle,
    CheckWindow,
    CheckAvailable,
    SetFIFOMark,
    EnableSPI,
    StuffFIFO,
    CheckMark,
    ShuffleData,
    Disable,
}

fn main() -> Result<(), ni_fpga::Error> {
    let mut tmp_bitfile = NamedTempFile::new().unwrap();
    write!(tmp_bitfile, include_str!("roboRIO_FPGA_2023_23.0.0.lvbitx")).unwrap();

    let session = Session::open_arc(
        tmp_bitfile.path().to_str().unwrap(),
        "2A5927EB7178780081872E6823E32FFC",
        "rio://172.22.11.2/RIO0",
    )?;


    let mut dc_offset: u32 = 0;
    let c = CString::new("DutyCycle0.Frequency").unwrap();
    let dc0 = unsafe {session.fpga().ffi().api21.as_ref().unwrap().NiFpgaDll_FindRegister(session.fpga().session(), c.as_ptr(), &mut dc_offset) };

    println!("{} {}", dc0, dc_offset);


    let dc = session.open_register::<DutyCycleFrequency>(dc_offset);
    let r = dc.read(&session)?;

    println!("{:?} {}", r, r.frequency);

    let c = CString::new("DutyCycle1.Frequency").unwrap();
    let dc0 = unsafe {session.fpga().ffi().api21.as_ref().unwrap().NiFpgaDll_FindRegister(session.fpga().session(), c.as_ptr(), &mut dc_offset) };

    println!("{} {}", dc0, dc_offset);


    let dc = session.open_register::<DutyCycleFrequency>(dc_offset);
    let r = dc.read(&session)?;

    println!("{:?} {}", r, r.frequency);

    let session_2 = session.clone();

    let voltage_register = session.open_register::<u16>(99174);
    let voltage_register_2 = session.open_const_register::<u16, 99174>();

    let voltage_register_3: Register<StoredOffset<u16>> =
        session.open_const_register::<u16, 99174>().into();

    let read_pwm_thread = thread::spawn(move || voltage_register_2.read(&session_2));

    println!("Input voltage: {:?}", voltage_register.read(&session)?);
    println!("Input voltage: {:?}", voltage_register_3.read(&session)?);
    println!("Input voltage: {:?}", read_pwm_thread.join().unwrap()?);
    Ok(())
}
