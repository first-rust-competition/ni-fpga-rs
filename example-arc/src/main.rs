use std::{ffi::CString, thread};

use ni_fpga::fxp::UnsignedPackedNumber;
use ni_fpga::{Register, RegisterAccess, SessionAccess, SessionBuilder, StoredOffset};
use ni_fpga_macros::{Cluster, Enum};

#[derive(Cluster, Debug)]
struct DigitalSource {
    channel: UnsignedPackedNumber<4>,
    module: UnsignedPackedNumber<1>,
    analog_trigger: bool,
}

#[derive(Cluster, Debug)]
struct DutyCycleFrequency {
    pub overflow: bool,
    pub frequency: UnsignedPackedNumber<11>,
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

#[derive(Enum, Debug, Copy, Clone)]
enum SPIDebugState {
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

const BITFILE_CONTENTS: &str = include_str!("roboRIO_FPGA_2023_23.0.0.lvbitx");

fn main() -> Result<(), ni_fpga::Error> {
    let session = SessionBuilder::new()
        .bitfile_contents(BITFILE_CONTENTS)?
        .ignore_signature()
        .resource("rio://172.22.11.2/RIO0")?
        .build_arc()?;

    let mut dc_offset: u32 = 0;
    let c = CString::new("DutyCycle0.Frequency").unwrap();
    let dc0 = unsafe {
        session
            .fpga()
            .ffi()
            .api21
            .as_ref()
            .unwrap()
            .NiFpgaDll_FindRegister(session.fpga().session(), c.as_ptr(), &mut dc_offset)
    };

    println!("{} {}", dc0, dc_offset);

    let dc = session.open_register::<DutyCycleFrequency>(dc_offset);
    let r = dc.read(&session)?;

    println!("{:?} {}", r, r.frequency);

    let c = CString::new("DutyCycle1.Frequency").unwrap();
    let dc0 = unsafe {
        session
            .fpga()
            .ffi()
            .api21
            .as_ref()
            .unwrap()
            .NiFpgaDll_FindRegister(session.fpga().session(), c.as_ptr(), &mut dc_offset)
    };

    println!("{} {}", dc0, dc_offset);

    let dc = session.open_register::<DutyCycleFrequency>(dc_offset);
    let r = dc.read(&session)?;

    println!("{:?} {}", r, r.frequency);

    let dc0s = session.open_const_register::<DigitalSource, 99398>();
    let config = dc0s.read(&session);

    println!("{:?}", config);

    let session_2 = session.clone();

    let voltage_register = session.open_register::<u16>(99174);
    let voltage_register_2 = session.open_const_register::<u16, 99174>();

    let voltage_register_3: Register<u16, StoredOffset> =
        session.open_const_register::<u16, 99174>().into();

    let read_pwm_thread = thread::spawn(move || voltage_register_2.read(&session_2));

    println!("Input voltage: {:?}", voltage_register.read(&session)?);
    println!("Input voltage: {:?}", voltage_register_3.read(&session)?);
    println!("Input voltage: {:?}", read_pwm_thread.join().unwrap()?);
    Ok(())
}
