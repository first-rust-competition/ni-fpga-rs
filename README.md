# ni-fpga-rs
[![crates.io](https://img.shields.io/crates/v/ni-fpga.svg)](https://crates.io/crates/ni-fpga)
[![docs.rs](https://docs.rs/ni-fpga/badge.svg)](https://docs.rs/ni-fpga)
[![CI](https://github.com/first-rust-competition/ni-fpga-rs/workflows/CI/badge.svg)](https://github.com/first-rust-competition/ni-fpga-rs/actions?query=workflow%3ACI)

Use this Rust interface to interact with NI FPGAs! See NI's [documentation](http://zone.ni.com/reference/en-XX/help/372928G-01/) about the FPGA C interface for more information.

## Supported types
This interface supports reading and writing the following types, both indvidually and in fixed-sized arrays:

### Primitive types
* bool
* u8
* u16
* u32
* ~u64~
* i8
* i16
* i32
* ~i64~

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
Enums are supported via a derive macro. Arrays of Enums are also supported. One of `u8`, `u16`, `u32`, or `u64` will be chosen as a backing type depending on the number of variants.
```rust
#[derive(Enum)]
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
```

### Fixed-point number types
FXP types are currently unsupported.

## Locating register offsets
Register offset can be found by introspecting `/boot/user.lvbitx` on a roboRIO. This file is also present in [first-rust-competition/cross-images](https://github.com/first-rust-competition/cross-images) images.

## Full example
```rust
use ni_fpga::Session;
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
    let session = Session::open(
        "/boot/user.lvbitx",
        "264D0BA312FF00B741D4742415E1D470",
        "RIO0",
    )?;

    println!("Input voltage: {:?}", session.read::<u16>(99174)?);
    println!("{:#?}", session.read::<PWMConfig>(98536)?);
    println!("{:#?}", session.read::<[AnalogTriggerOutput; 8]>(98424)?);
    println!("{:#?}", session.read::<SPIDebugState>(99314)?);
    Ok(())
}
```
