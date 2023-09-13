use std::{ops::Deref, sync::Mutex, time::Duration};

pub use ni_fpga_sys::Irq;

use crate::{nifpga::IrqContext, Error, NiFpga, Session, SessionAccess, Status};

pub struct InterruptContext<Fpga>
where
    Fpga: Deref<Target = NiFpga>,
{
    manager: &'static Mutex<InterruptManager>,
    context: IrqContext,
    session: Session<Fpga>,
}

pub trait InterruptWaiter {
    fn wait_for_interrupt(
        &self,
        mask: Irq,
        ignore_previous: bool,
        timeout: Duration,
    ) -> Result<Irq, Error>;
}

impl<Fpga> InterruptContext<Fpga>
where
    Fpga: Deref<Target = NiFpga>,
{
    pub fn new(session: Session<Fpga>) -> Result<Self, Error> {
        static MANAGER: Mutex<InterruptManager> = Mutex::new(InterruptManager {
            current_mask: Irq::empty(),
        });

        let context = unsafe { session.fpga() }.reserve_irq_context()?;
        Ok(Self {
            session,
            context,
            manager: &MANAGER,
        })
    }
}


impl<Fpga> InterruptWaiter for InterruptContext<Fpga>
where
    Fpga: Deref<Target = NiFpga>,
{
    fn wait_for_interrupt(
        &self,
        mask: Irq,
        ignore_previous: bool,
        timeout: Duration,
    ) -> Result<Irq, Error> {
        let _reservation = MaskReservation::reserve(self.manager, mask)?;

        if ignore_previous {
            unsafe { self.session.fpga().acknowledge_irqs(mask)? };
        }

        unsafe {
            let result = self
                .session
                .fpga()
                .wait_on_irqs(self.context, mask, timeout)?;
            self.session.fpga().acknowledge_irqs(mask)?;
            Ok(result)
        }
    }
}

impl<Fpga> Drop for InterruptContext<Fpga>
where
    Fpga: Deref<Target = NiFpga>,
{
    fn drop(&mut self) {
        // TODO Handle result
        let _ = unsafe { self.session.fpga().unreserve_irq_context(self.context) };
    }
}

struct MaskReservation {
    manager: &'static Mutex<InterruptManager>,
    mask: Irq,
}

impl MaskReservation {
    pub fn reserve(manager: &'static Mutex<InterruptManager>, mask: Irq) -> Result<Self, Error> {
        let mut lock = manager.lock()?;
        lock.reserve(mask)?;
        Ok(Self { manager, mask })
    }
}

impl Drop for MaskReservation {
    fn drop(&mut self) {
        // TODO make this unwrap a bit safer
        let mut lock = self.manager.lock().unwrap();
        lock.release(self.mask);
    }
}

struct InterruptManager {
    pub current_mask: Irq,
}

impl InterruptManager {
    pub fn reserve(&mut self, mask: Irq) -> Result<(), Error> {
        if self.current_mask.intersects(mask) {
            return Err(Error::FPGA(Status::InvalidParameter));
        }
        self.current_mask |= mask;
        Ok(())
    }

    pub fn release(&mut self, mask: Irq) {
        self.current_mask &= !mask;
    }
}
