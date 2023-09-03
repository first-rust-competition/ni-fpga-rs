use std::thread;

use ni_fpga::{FixedRegisterAccess, Session};
use ni_fpga_macros::{Cluster, Enum};

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

#[derive(Enum, Debug)]
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

fn main() -> Result<(), ni_fpga::Error> {
    let session = Session::open_arc(
        "/boot/user.lvbitx",
        "264D0BA312FF00B741D4742415E1D470",
        "RIO0",
    )?;

    let voltage_register = session.open_register::<u16, 99174>();
    let voltage_register_2 = session.open_register::<u16, 99174>();

    let read_pwm_thread = thread::spawn(move || voltage_register_2.read_direct());

    println!("Input voltage: {:?}", voltage_register.read_direct()?);
    println!("Input voltage: {:?}", read_pwm_thread.join().unwrap()?);
    Ok(())
}
