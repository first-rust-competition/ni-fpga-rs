# ni-fpga-rs
Use this Rust interface to interact with NI FPGAs! See NI's [documentation](http://zone.ni.com/reference/en-XX/help/372928G-01/) about the FPGA C interface for more information.

## Supported types
This interface supports reading and writing the following types, both indvidually and in fixed-sized arrays:

### Primitive types
* bool
* u8
* u16
* u32
* u64
* i8
* i16
* i32
* i64

### Clusters
Clusters are supported via a derive macro. Arrays of Clusters are also supported.
```rust
#[derive(Cluster)]
struct PWMConfig {
    period: u16,
    min_high: u16,
}
```

### Enums
Enums are currently unsupported.

### Fixed-point number types
FXP types are currently unsupported.

## Locating register offsets
Register offset can be found by introspecting `/boot/user.lvbitx` on a roboRIO. This file is also present in [first-rust-competition/cross-images](https://github.com/first-rust-competition/cross-images) images.

## Full example
```rust
use std::{thread, time};

use ni_fpga::Session;
use ni_fpga_macros::Cluster;

#[derive(Cluster, Debug)]
struct PWMConfig {
    period: u16,
    min_high: u16,
}

fn main(){
    let session = Session::open(
        "/boot/user.lvbitx",
        "C571384F0C3E586B64ADFE11551DAAD0",
        "RIO0",
    ).unwrap();
    
    // Offsets will vary between FPGA images
    let voltage: u16 = session.read(99174).unwrap();
    println!("Input voltage: {:?}", voltage);
    
    println!("PWM configuration: {:?}", session.read::<PWMConfig>(98536).unwrap());
}
```
