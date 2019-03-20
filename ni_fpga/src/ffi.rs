#![allow(dead_code)]
use std::os::raw::c_char;

pub type Offset = u32;
pub type Session = u32;
pub type Status = i32;

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
    #[link_name = "NiFpgaDll_ReadU8"]
    pub fn ReadU8(
        session: Session,
        indicator: Offset,
        value: *mut u8,
    ) -> Status;
    #[link_name = "NiFpgaDll_ReadU16"]
    pub fn ReadU16(
        session: Session,
        indicator: Offset,
        value: *mut u16,
    ) -> Status;
    #[link_name = "NiFpgaDll_ReadU32"]
    pub fn ReadU32(
        session: Session,
        indicator: Offset,
        value: *mut u32,
    ) -> Status;
    #[link_name = "NiFpgaDll_ReadU64"]
    pub fn ReadU64(
        session: Session,
        indicator: Offset,
        value: *mut u64,
    ) -> Status;
    #[link_name = "NiFpgaDll_ReadI8"]
    pub fn ReadI8(
        session: Session,
        indicator: Offset,
        value: *mut i8,
    ) -> Status;
    #[link_name = "NiFpgaDll_ReadI16"]
    pub fn ReadI16(
        session: Session,
        indicator: Offset,
        value: *mut i16,
    ) -> Status;
    #[link_name = "NiFpgaDll_ReadI324"]
    pub fn ReadI32(
        session: Session,
        indicator: Offset,
        value: *mut i32,
    ) -> Status;
    #[link_name = "NiFpgaDll_ReadI64"]
    pub fn ReadI64(
        session: Session,
        indicator: Offset,
        value: *mut i64,
    ) -> Status;

    #[link_name = "NiFpgaDll_WriteBool"]
    pub fn WriteBool(
        session: Session,
        indicator: Offset,
        value: bool,
    ) -> Status;
}
