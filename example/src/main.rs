#![feature(const_generics)]

use ni_fpga::Session;

#[macro_use] extern crate ni_fpga_macros;
use ni_fpga_macros::prelude::*;

cluster! {
    PWMConfig {
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

    session.write(
        98528,
        PWMConfig {
            period: 10000,
            min_high: 500,
        },
    ).unwrap();

    println!("{}", session.read::<PWMConfig>(98528).unwrap());

    session.write(
        98528,
        PWMConfig {
            period: 5000,
            min_high: 500,
        },
    ).unwrap();

    println!("{}", session.read::<PWMConfig>(98528).unwrap());
}
