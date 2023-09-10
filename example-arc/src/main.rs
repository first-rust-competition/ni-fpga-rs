use std::{thread, time::Duration};

use ni_fpga::RegisterRead;

use crate::registers::FpgaBitfile;

mod registers;

fn main() -> Result<(), ni_fpga::Error> {
    let session = FpgaBitfile::session_builder("rio://172.22.11.2/RIO0")?.build_arc()?;
    let mut regs = FpgaBitfile::take(&session).unwrap();

    let frequency = regs.DutyCycle0_Frequency.take().unwrap();
    let output = regs.DutyCycle0_HighTicks.take().unwrap();

    let frequency1 = regs.DutyCycle1_Frequency.take().unwrap();

    loop {
        let o = output
            .read(&session)?
            .to_fxp()
            .map(|f| f.to_int())
            .unwrap_or(0);
        let f = frequency
            .read(&session)?
            .to_fxp()
            .map(|f| f.to_int())
            .unwrap_or(0);
        let f1 = frequency1
            .read(&session)?
            .to_fxp()
            .map(|f| f.to_int())
            .unwrap_or(-42);

        println!("F: {f} O: {o} f1: {f1}");
        thread::sleep(Duration::from_secs(1));
    }
}
