#![allow(dead_code)]
use std::os::raw::c_char;

pub type Offset = u32;
pub type Session = u32;
pub type Status = i32;

pub type Func<T> = unsafe fn(Session, Offset, T) -> Status;

#[link(name = "NiFpga")]
extern "C" {
    #[link_name = "NiFpgaDll_Open"]
   pub fn Open(
        bitfile: *const c_char,
        signature: *const c_char,
        resource: *const c_char,
        attribute: u32,
        session: *mut Session,
    ) -> Status;
    #[link_name = "NiFpgaDll_Close"]
    pub fn Close(
        session: Session,
        attribute: u32,
    ) -> Status;

    #[link_name = "NiFpgaDll_ReadBool"]
    pub fn ReadBool(
        session: Session,
        indicator: Offset,
        value: *mut bool,
    ) -> Status;
    #[link_name = "NiFpgaDll_ReadArrayBool"]
    pub fn ReadArrayBool(
        session: Session,
        indicator: Offset,
        array: *mut bool,
        size: usize,
    ) -> Status;
    #[link_name = "NiFpgaDll_WriteBool"]
    pub fn WriteBool(
        session: Session,
        indicator: Offset,
        value: bool,
    ) -> Status;
    #[link_name = "NiFpgaDll_WriteArrayBool"]
    pub fn WriteArrayBool(
        session: Session,
        indicator: Offset,
        array: *const bool,
        size: usize,
    ) -> Status;
}
