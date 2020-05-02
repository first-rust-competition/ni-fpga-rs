use ni_fpga::Session;
use ni_fpga_macros::Cluster;

#[derive(Cluster, Debug)]
struct PWMConfig {
    period: u16,
    min_high: u16,
}

#[derive(Cluster, Debug)]
struct LEDs {
    comm: u8,
    mode: u8,
    rsl: bool,
}

#[derive(Cluster, Debug)]
struct AnalogTriggerOutput {
    in_hysteresis: bool,
    over_limit: bool,
    rising: bool,
    falling: bool,
}

fn main() {
    let session = Session::open(
        "/boot/user.lvbitx",
        "264D0BA312FF00B741D4742415E1D470",
        "RIO0",
    )
    .unwrap();

    println!("{:#?}", session.read::<LEDs>(98320).unwrap());
    println!("{:#?}", session.read::<PWMConfig>(98536).unwrap());
    println!(
        "{:#?}",
        session.read::<[AnalogTriggerOutput; 8]>(98424).unwrap()
    );
}
