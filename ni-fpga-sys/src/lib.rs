#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

pub type Offset = u32;
pub type Session = u32;
pub type Status = i32;
pub type Bool = u8;
pub type IrqContext = *const c_void;

mod enums;

pub use dlopen::Error;
pub use enums::*;

use std::ffi::{c_char, c_void};

use dlopen::{
    utils::platform_file_name,
    wrapper::{Container, WrapperApi, WrapperMultiApi},
};

use dlopen_derive::{WrapperApi, WrapperMultiApi};

pub type NiFpgaApiContainer = Container<NiFpgaApi>;

#[derive(WrapperApi)]
pub struct NiFpgaHmbApi {
    NiFpgaDll_OpenHmb: unsafe extern "C" fn(
        session: Session,
        memory_name: *const c_char,
        memory_size: *mut usize,
        virual_address: *mut *mut c_void,
    ) -> Status,
    NiFpgaDll_CloseHmb:
        unsafe extern "C" fn(session: Session, memory_name: *const c_char) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaLlbApi {
    NiFpgaDll_OpenLlb: unsafe extern "C" fn(
        session: Session,
        memory_name: *const c_char,
        memory_size_to_host: *mut usize,
        virual_address_to_host: *mut *mut c_void,
        memory_size_to_fpga: *mut usize,
        virual_address_to_fpga: *mut *mut c_void,
    ) -> Status,
    NiFpgaDll_CloseLlb:
        unsafe extern "C" fn(session: Session, memory_name: *const c_char) -> Status,
}

// These are new API's for each LabVIEW year.
#[derive(WrapperApi)]
pub struct NiFpga19 {
    NiFpgaDll_CommitFifoConfiguration: unsafe extern "C" fn(session: Session, fifo: u32) -> Status,
    NiFpgaDll_ReadFifoComposite: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut c_void,
        bytesPerElement: u32,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoComposite: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const c_void,
        bytesPerElement: u32,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpga20 {
    NiFpgaDll_UnreserveFifo: unsafe extern "C" fn(session: Session, fifo: u32) -> Status,
    NiFpgaDll_ClientFunctionCall: unsafe extern "C" fn(
        session: Session,
        group: u32,
        functionId: u32,
        inBuffer: *const c_void,
        inBufferSize: usize,
        outBuffer: *mut c_void,
        outBufferSize: usize,
    ) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpga21 {
    NiFpgaDll_MapP2PSinkFifo: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        size: *mut usize,
        virtualAddress: *mut *mut c_void,
    ) -> Status,
    NiFpgaDll_UnmapP2PSinkFifo: unsafe extern "C" fn(session: Session, fifo: u32) -> Status,
    NiFpgaDll_FindRegister: unsafe extern "C" fn(
        session: Session,
        registerName: *const c_char,
        registerOffset: *mut u32,
    ) -> Status,
    NiFpgaDll_FindFifo: unsafe extern "C" fn(
        session: Session,
        fifoName: *const c_char,
        fifoNumber: *mut u32,
    ) -> Status,
    NiFpgaDll_GetFpgaViState: unsafe extern "C" fn(session: Session, state: *mut u32) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaBaseApi {
    NiFpgaDll_Open: unsafe extern "C" fn(
        path: *const c_char,
        signature: *const c_char,
        resource: *const c_char,
        attribute: u32,
        session: *mut Session,
    ) -> Status,
    NiFpgaDll_Close: unsafe extern "C" fn(session: Session, attribute: u32) -> Status,
    NiFpgaDll_Run: unsafe extern "C" fn(session: Session, attribute: u32) -> Status,
    NiFpgaDll_Abort: unsafe extern "C" fn(session: Session) -> Status,
    NiFpgaDll_Reset: unsafe extern "C" fn(session: Session) -> Status,
    NiFpgaDll_Download: unsafe extern "C" fn(session: Session) -> Status,
    NiFpgaDll_ReadBool:
        unsafe extern "C" fn(session: Session, indicator: u32, value: *mut Bool) -> Status,
    NiFpgaDll_ReadI8:
        unsafe extern "C" fn(session: Session, indicator: u32, value: *mut i8) -> Status,
    NiFpgaDll_ReadU8:
        unsafe extern "C" fn(session: Session, indicator: u32, value: *mut u8) -> Status,
    NiFpgaDll_ReadI16:
        unsafe extern "C" fn(session: Session, indicator: u32, value: *mut i16) -> Status,
    NiFpgaDll_ReadU16:
        unsafe extern "C" fn(session: Session, indicator: u32, value: *mut u16) -> Status,
    NiFpgaDll_ReadI32:
        unsafe extern "C" fn(session: Session, indicator: u32, value: *mut i32) -> Status,
    NiFpgaDll_ReadU32:
        unsafe extern "C" fn(session: Session, indicator: u32, value: *mut u32) -> Status,
    NiFpgaDll_ReadI64:
        unsafe extern "C" fn(session: Session, indicator: u32, value: *mut i64) -> Status,
    NiFpgaDll_ReadU64:
        unsafe extern "C" fn(session: Session, indicator: u32, value: *mut u64) -> Status,
    NiFpgaDll_ReadSgl:
        unsafe extern "C" fn(session: Session, indicator: u32, value: *mut f32) -> Status,
    NiFpgaDll_ReadDbl:
        unsafe extern "C" fn(session: Session, indicator: u32, value: *mut f64) -> Status,
    NiFpgaDll_WriteBool:
        unsafe extern "C" fn(session: Session, control: u32, value: Bool) -> Status,
    NiFpgaDll_WriteI8: unsafe extern "C" fn(session: Session, control: u32, value: i8) -> Status,
    NiFpgaDll_WriteU8: unsafe extern "C" fn(session: Session, control: u32, value: u8) -> Status,
    NiFpgaDll_WriteI16: unsafe extern "C" fn(session: Session, control: u32, value: i16) -> Status,
    NiFpgaDll_WriteU16: unsafe extern "C" fn(session: Session, control: u32, value: u16) -> Status,
    NiFpgaDll_WriteI32: unsafe extern "C" fn(session: Session, control: u32, value: i32) -> Status,
    NiFpgaDll_WriteU32: unsafe extern "C" fn(session: Session, control: u32, value: u32) -> Status,
    NiFpgaDll_WriteI64: unsafe extern "C" fn(session: Session, control: u32, value: i64) -> Status,
    NiFpgaDll_WriteU64: unsafe extern "C" fn(session: Session, control: u32, value: u64) -> Status,
    NiFpgaDll_WriteSgl: unsafe extern "C" fn(session: Session, control: u32, value: f32) -> Status,
    NiFpgaDll_WriteDbl: unsafe extern "C" fn(session: Session, control: u32, value: f64) -> Status,
    NiFpgaDll_ReadArrayBool: unsafe extern "C" fn(
        session: Session,
        indicator: u32,
        array: *mut Bool,
        size: usize,
    ) -> Status,
    NiFpgaDll_ReadArrayI8: unsafe extern "C" fn(
        session: Session,
        indicator: u32,
        array: *mut i8,
        size: usize,
    ) -> Status,
    NiFpgaDll_ReadArrayU8: unsafe extern "C" fn(
        session: Session,
        indicator: u32,
        array: *mut u8,
        size: usize,
    ) -> Status,
    NiFpgaDll_ReadArrayI16: unsafe extern "C" fn(
        session: Session,
        indicator: u32,
        array: *mut i16,
        size: usize,
    ) -> Status,
    NiFpgaDll_ReadArrayU16: unsafe extern "C" fn(
        session: Session,
        indicator: u32,
        array: *mut u16,
        size: usize,
    ) -> Status,
    NiFpgaDll_ReadArrayI32: unsafe extern "C" fn(
        session: Session,
        indicator: u32,
        array: *mut i32,
        size: usize,
    ) -> Status,
    NiFpgaDll_ReadArrayU32: unsafe extern "C" fn(
        session: Session,
        indicator: u32,
        array: *mut u32,
        size: usize,
    ) -> Status,
    NiFpgaDll_ReadArrayI64: unsafe extern "C" fn(
        session: Session,
        indicator: u32,
        array: *mut i64,
        size: usize,
    ) -> Status,
    NiFpgaDll_ReadArrayU64: unsafe extern "C" fn(
        session: Session,
        indicator: u32,
        array: *mut u64,
        size: usize,
    ) -> Status,
    NiFpgaDll_ReadArraySgl: unsafe extern "C" fn(
        session: Session,
        indicator: u32,
        array: *mut f32,
        size: usize,
    ) -> Status,
    NiFpgaDll_ReadArrayDbl: unsafe extern "C" fn(
        session: Session,
        indicator: u32,
        array: *mut f64,
        size: usize,
    ) -> Status,
    NiFpgaDll_WriteArrayBool: unsafe extern "C" fn(
        session: Session,
        control: u32,
        array: *const Bool,
        size: usize,
    ) -> Status,
    NiFpgaDll_WriteArrayI8: unsafe extern "C" fn(
        session: Session,
        control: u32,
        array: *const i8,
        size: usize,
    ) -> Status,
    NiFpgaDll_WriteArrayU8: unsafe extern "C" fn(
        session: Session,
        control: u32,
        array: *const u8,
        size: usize,
    ) -> Status,
    NiFpgaDll_WriteArrayI16: unsafe extern "C" fn(
        session: Session,
        control: u32,
        array: *const i16,
        size: usize,
    ) -> Status,
    NiFpgaDll_WriteArrayU16: unsafe extern "C" fn(
        session: Session,
        control: u32,
        array: *const u16,
        size: usize,
    ) -> Status,
    NiFpgaDll_WriteArrayI32: unsafe extern "C" fn(
        session: Session,
        control: u32,
        array: *const i32,
        size: usize,
    ) -> Status,
    NiFpgaDll_WriteArrayU32: unsafe extern "C" fn(
        session: Session,
        control: u32,
        array: *const u32,
        size: usize,
    ) -> Status,
    NiFpgaDll_WriteArrayI64: unsafe extern "C" fn(
        session: Session,
        control: u32,
        array: *const i64,
        size: usize,
    ) -> Status,
    NiFpgaDll_WriteArrayU64: unsafe extern "C" fn(
        session: Session,
        control: u32,
        array: *const u64,
        size: usize,
    ) -> Status,
    NiFpgaDll_WriteArraySgl: unsafe extern "C" fn(
        session: Session,
        control: u32,
        array: *const f32,
        size: usize,
    ) -> Status,
    NiFpgaDll_WriteArrayDbl: unsafe extern "C" fn(
        session: Session,
        control: u32,
        array: *const f64,
        size: usize,
    ) -> Status,
    NiFpgaDll_ReserveIrqContext:
        unsafe extern "C" fn(session: Session, context: *mut IrqContext) -> Status,
    NiFpgaDll_UnreserveIrqContext:
        unsafe extern "C" fn(session: Session, context: IrqContext) -> Status,
    NiFpgaDll_WaitOnIrqs: unsafe extern "C" fn(
        session: Session,
        context: IrqContext,
        irqs: u32,
        timeout: u32,
        irqsAsserted: *mut u32,
        timedOut: *mut Bool,
    ) -> Status,
    NiFpgaDll_AcknowledgeIrqs: unsafe extern "C" fn(session: Session, irqs: u32) -> Status,
    NiFpgaDll_ConfigureFifo:
        unsafe extern "C" fn(session: Session, fifo: u32, depth: usize) -> Status,
    NiFpgaDll_ConfigureFifo2: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        requestedDepth: usize,
        actualDepth: *mut usize,
    ) -> Status,
    NiFpgaDll_StartFifo: unsafe extern "C" fn(session: Session, fifo: u32) -> Status,
    NiFpgaDll_StopFifo: unsafe extern "C" fn(session: Session, fifo: u32) -> Status,
    NiFpgaDll_ReadFifoBool: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut Bool,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoI8: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut i8,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoU8: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut u8,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoI16: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut i16,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoU16: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut u16,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoI32: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut i32,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoU32: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut u32,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoI64: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut i64,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoU64: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut u64,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoSgl: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut f32,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoDbl: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut f64,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoBool: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const Bool,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoI8: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const i8,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoU8: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const u8,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoI16: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const i16,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoU16: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const u16,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoI32: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const i32,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoU32: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const u32,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoI64: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const i64,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoU64: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const u64,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoSgl: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const f32,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoDbl: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const f64,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsBool: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const Bool,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsI8: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const i8,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsU8: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const u8,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsI16: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const i16,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsU16: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const u16,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsI32: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const i32,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsU32: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const u32,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsI64: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const i64,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsU64: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const u64,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsSgl: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const f32,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsDbl: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const f64,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsBool: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut Bool,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsI8: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut i8,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsU8: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut u8,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsI16: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut i16,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsU16: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut u16,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsI32: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut i32,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsU32: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut u32,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsI64: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut i64,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsU64: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut u64,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsSgl: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut f32,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsDbl: unsafe extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut f64,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReleaseFifoElements:
        unsafe extern "C" fn(session: Session, fifo: u32, elements: usize) -> Status,
    NiFpgaDll_GetPeerToPeerFifoEndpoint:
        unsafe extern "C" fn(session: Session, fifo: u32, endpoint: *mut u32) -> Status,
    NiFpgaDll_GetBitfileContents:
        unsafe extern "C" fn(session: Session, contents: *mut *const c_char) -> Status,
}

#[derive(WrapperMultiApi)]
pub struct NiFpgaApi {
    pub base: NiFpgaBaseApi,
    pub hmb: Option<NiFpgaHmbApi>,
    pub llb: Option<NiFpgaLlbApi>,

    pub api19: Option<NiFpga19>,
    pub api20: Option<NiFpga20>,
    pub api21: Option<NiFpga21>,
}

impl NiFpgaApi {
    pub fn load() -> Result<NiFpgaApiContainer, Error> {
        match unsafe { Container::load(platform_file_name("NiFpga")) } {
            Ok(api) => Ok(api),
            Err(err) => Err(err),
        }
    }
}
