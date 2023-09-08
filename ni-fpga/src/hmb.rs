use std::{
    ffi::{c_void, CString},
    mem::size_of,
    ops::Deref,
    ptr,
};

use crate::{
    nifpga::{NiFpga, StatusHelper},
    session::SessionAccess,
    Error, Session, Status,
};

struct VirtualAddressHandle(*mut c_void);

unsafe impl Send for VirtualAddressHandle {}
unsafe impl Sync for VirtualAddressHandle {}

pub struct Hmb<Fpga>
where
    Fpga: Deref<Target = NiFpga>,
{
    session: Session<Fpga>,
    memory_name: CString,
    memory_size: usize,
    virtual_address: VirtualAddressHandle,
}

impl<Fpga> Hmb<Fpga>
where
    Fpga: Deref<Target = NiFpga>,
{
    pub fn new(session: Session<Fpga>, memory_name: &CString) -> Result<Hmb<Fpga>, Error> {
        let fpga = session.fpga();
        match &fpga.api.hmb {
            Some(hmb) => {
                let mut memory_size: usize = 0;
                let mut virtual_address: *mut c_void = ptr::null_mut();
                match unsafe {
                    hmb.NiFpgaDll_OpenHmb(
                        fpga.session,
                        memory_name.as_ptr(),
                        &mut memory_size,
                        &mut virtual_address,
                    )
                    .to_result()
                } {
                    Ok(_) => Ok({
                        Self {
                            session,
                            memory_name: memory_name.clone(),
                            memory_size,
                            virtual_address: VirtualAddressHandle(virtual_address),
                        }
                    }),
                    Err(err) => Err(err),
                }
            }
            None => Err(Error::FPGA(Status::ResourceNotInitialized)),
        }
    }
}

pub trait HmbAccess {
    fn read<T>(&self, offset: usize) -> T;
    fn write<T>(&mut self, offset: usize, value: T);
}

impl<Fpga> HmbAccess for Hmb<Fpga>
where
    Fpga: Deref<Target = NiFpga>,
{
    fn read<T>(&self, offset: usize) -> T {
        unsafe {
            assert!(size_of::<T>() + offset <= self.memory_size);
            let base: *const u8 = self.virtual_address.0 as *const u8;
            let address = base.add(offset);
            let typed_address = address as *const T;
            ptr::read_volatile(typed_address)
        }
    }

    fn write<T>(&mut self, offset: usize, value: T) {
        unsafe {
            assert!(size_of::<T>() + offset <= self.memory_size);
            let base: *mut u8 = self.virtual_address.0 as *mut u8;
            let address = base.add(offset);
            let typed_address = address as *mut T;
            ptr::write_volatile(typed_address, value);
        }
    }
}

impl<Fpga> Drop for Hmb<Fpga>
where
    Fpga: Deref<Target = NiFpga>,
{
    fn drop(&mut self) {
        // Unwrap is safe here, as the only way this can get constructed is
        // if its possible to unwrap it at construction
        // TODO figure out what to do here with the return value
        unsafe {
            self.session
                .fpga()
                .api
                .hmb
                .as_ref()
                .unwrap()
                .NiFpgaDll_CloseHmb(self.session.fpga().session, self.memory_name.as_ptr());
        }
    }
}
