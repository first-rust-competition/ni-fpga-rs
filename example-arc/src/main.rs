use std::thread;

use ni_fpga::{RegisterAccess, Session, SessionAccess};
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

    let pwm_config_register = session.open_register::<PWMConfig, 98536>();

    let read_pwm_thread = thread::spawn(move || pwm_config_register.read());

    println!("Input voltage: {:?}", session.read::<u16>(99174)?);
    //println!("{:#?}", pwm_config_register.read()?);
    println!("{:#?}", session.read::<PWMConfig>(98536)?);
    println!("{:#?}", session.read::<[AnalogTriggerOutput; 8]>(98424)?);
    println!("{:#?}", session.read::<SPIDebugState>(99314)?);
    println!("{:#?}", read_pwm_thread.join().unwrap()?);
    Ok(())
}
