use std::{thread, time::Duration};

use ni_fpga::{
    ReadOnly, ReadWrite, Register, RegisterRead, RegisterWrite, SessionAccess, StoredOffset,
};

use crate::registers::FpgaBitfile;

mod registers;

fn main() -> Result<(), ni_fpga::Error> {
    let session = FpgaBitfile::session_builder("rio://172.22.11.2/RIO0")?.build_arc()?;
    let mut regs = FpgaBitfile::take(&session).unwrap();

    let frequency = regs.DutyCycle0_Frequency.take().unwrap();
    let output = regs.DutyCycle0_HighTicks.take().unwrap();

    loop {
        let o = output.read(&session)?.to_int();
        let f = frequency.read(&session)?.to_int();

        println!("F: {f} O: {o}");
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
