use std::{
    ffi::{c_void, CString},
    mem::size_of,
    ptr,
    sync::Arc,
};

use crate::{
    nifpga::{NiFpga, StatusHelper},
    Error, Status,
};

pub struct Hmb {
    fpga: Arc<Box<NiFpga>>,
    memory_name: CString,
    memory_size: usize,
    virtual_address: *mut c_void,
}

impl Hmb {
    pub fn new(fpga: Arc<Box<NiFpga>>, memory_name: &CString) -> Result<Hmb, Error> {
        match &fpga.api.hmb {
            Some(hmb) => {
                let mut memory_size: usize = 0;
                let mut virtual_address: *mut c_void = ptr::null_mut();
                match hmb
                    .NiFpgaDll_OpenHmb(
                        fpga.session,
                        memory_name.as_ptr(),
                        &mut memory_size,
                        &mut virtual_address,
                    )
                    .to_result()
                {
                    Ok(_) => Ok({
                        Self {
                            fpga,
                            memory_name: memory_name.clone(),
                            memory_size,
                            virtual_address,
                        }
                    }),
                    Err(err) => Err(err),
                }
            }
            None => Err(Error::FPGA(Status::ResourceNotInitialized)),
        }
    }

    pub fn read<T>(&self, offset: usize) -> T {
        unsafe {
            assert!(size_of::<T>() + offset <= self.memory_size);
            let base: *const u8 = self.virtual_address as *const u8;
            let address = base.add(offset);
            let typed_address = address as *const T;
            ptr::read_volatile(typed_address)
        }
    }

    pub fn write<T>(&mut self, offset: usize, value: T) {
        unsafe {
            assert!(size_of::<T>() + offset <= self.memory_size);
            let base: *mut u8 = self.virtual_address as *mut u8;
            let address = base.add(offset);
            let typed_address = address as *mut T;
            ptr::write_volatile(typed_address, value);
        }
    }
}

impl Drop for Hmb {
    fn drop(&mut self) {
        // Unwrap is safe here, as the only way this can get constructed is
        // if its possible to unwrap it at construction
        // TODO figure out what to do here with the return value
        self.fpga
            .api
            .hmb
            .as_ref()
            .unwrap()
            .NiFpgaDll_CloseHmb(self.fpga.session, self.memory_name.as_ptr());
    }
}
