#![feature(const_generics)]

use std::{thread, time};

use ni_fpga::Session;

#[macro_use] extern crate ni_fpga_macros;
use ni_fpga_macros::prelude::*;

cluster! {
    Cluster {
        period: u16,
        min_high: u16,
    }
}

fn main() {
    println!("{}, {}", Cluster::packed_bits(), std::mem::size_of::<Cluster>());
    let session = Session::open(
        "/boot/user.lvbitx",
        "C571384F0C3E586B64ADFE11551DAAD0",
        "RIO0",
    ).unwrap();
    loop {
        let val: Cluster = session.read(98528).unwrap();
        println!("{}", val);
        thread::sleep(time::Duration::from_secs(1));
    }
}
