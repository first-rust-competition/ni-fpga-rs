#![feature(const_generics)]

use ni_fpga::Session;

#[macro_use] extern crate ni_fpga_macros;
use ni_fpga_macros::prelude::*;

cluster! {
    struct PWMConfig {
        period: u16,
        min_high: u16,
    }
}

fn main() {
    let session = Session::open(
        "/boot/user.lvbitx",
        "C571384F0C3E586B64ADFE11551DAAD0",
        "RIO0",
    ).unwrap();

    println!("{}", session.read::<PWMConfig>(98528).unwrap());
}
