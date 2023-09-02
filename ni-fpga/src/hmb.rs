use std::{
    ffi::{c_void, CString},
    mem::size_of,
    ptr,
};

use ni_fpga_sys::{NiFpgaHmbApi, Session};

pub struct Hmb<'a> {
    api: &'a NiFpgaHmbApi,
    session: Session,
    memory_name: CString,
    memory_size: usize,
    virtual_address: *mut c_void,
}

impl<'a> Hmb<'a> {
    pub fn new(
        api: &'a NiFpgaHmbApi,
        session: Session,
        memory_name: CString,
        memory_size: usize,
        virtual_address: *mut c_void,
    ) -> Hmb<'a> {
        Self {
            api,
            session,
            memory_name,
            memory_size,
            virtual_address,
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

impl<'a> Drop for Hmb<'a> {
    fn drop(&mut self) {
        // TODO figure out what to do here with the return value
        self.api
            .NiFpgaDll_CloseHmb(self.session, self.memory_name.as_ptr());
    }
}
