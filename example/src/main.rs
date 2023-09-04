use ni_fpga::{RegisterAccess, Session};
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
    let session = Session::open(
        "/boot/user.lvbitx",
        "264D0BA312FF00B741D4742415E1D470",
        "RIO0",
    )?;

    let spi_debug_state = session.open_const_register::<SPIDebugState, 99314>();

    let pwm_config_register = session.open_const_register::<PWMConfig, 98536>();

    spi_debug_state.read(&session)?;

    println!("Input voltage: {:?}", session.read::<u16>(99174)?);
    println!(
        "{:#?}",
        session.read::<PWMConfig>(pwm_config_register.offset())?
    );
    println!("{:#?}", session.read::<PWMConfig>(98536)?);
    println!("{:#?}", session.read::<[AnalogTriggerOutput; 8]>(98424)?);
    println!("{:#?}", session.read::<SPIDebugState>(99314)?);
    println!("{:#?}", spi_debug_state.read(&session));
    Ok(())
}
