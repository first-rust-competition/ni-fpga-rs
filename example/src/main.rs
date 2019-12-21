#![feature(const_generics)]

use std::{thread, time};

use ni_fpga::Session;

#[macro_use] extern crate ni_fpga_macros;
use ni_fpga_macros::prelude::*;

cluster! {
    LEDs {
        comm: u8,
        mode: u8,
        rsl: bool,
    }
}

fn main() {
    assert_eq!(LEDs::packed_bits(), 17);
    let session = Session::open(
        "/boot/user.lvbitx",
        "C571384F0C3E586B64ADFE11551DAAD0",
        "RIO0",
    ).unwrap();
    loop {
        let leds: LEDs = session.read(98320).unwrap();
        println!("LED state: {}", leds);
        thread::sleep(time::Duration::from_secs(1));
    }
}
