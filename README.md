# ni_fpga-rs
Use this Rust interface to interact with NI FPGAs! See NI's [documentation](http://zone.ni.com/reference/en-XX/help/372928G-01/) about the FPGA C interface for more information.

## Supported types
This interface supports reading and writing the following types, both indvidually and in fixed-sized arrays:
* bool
* u8
* u16
* u32
* u64
* i8
* i16
* i32
* i64

## Locating register offsets
Register offset can be found by introspecting `/boot/user.lvbitx` on a roboRIO. This file is also present in [first-rust-competition/cross-images](https://github.com/first-rust-competition/cross-images) images.

## Example
```rust
use std::{thread, time};

use ni_fpga::Session;

fn main() {
    let session = Session::open(
        "/boot/user.lvbitx",
        "C571384F0C3E586B64ADFE11551DAAD0",
        "RIO0",
    ).unwrap();
    loop {
        // The VinVoltage register lives at offset 99174 
        let voltage: u16 = session.read(99174).unwrap();
        println!("Input voltage: {}", voltage);
        thread::sleep(time::Duration::from_secs(1));
    }
}
```
