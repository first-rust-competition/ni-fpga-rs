#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

pub type Offset = u32;
pub type Session = u32;
pub type Status = i32;
pub type Bool = u8;
pub type IrqContext = *const c_void;

mod dlopenerror;
mod enums;

pub use dlopenerror::DlOpenError;
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
    NiFpgaDll_OpenHmb: extern "C" fn(
        session: Session,
        memory_name: *const c_char,
        memory_size: *mut usize,
        virual_address: *mut *mut c_void,
    ) -> Status,
    NiFpgaDll_CloseHmb: extern "C" fn(session: Session, memory_name: *const c_char) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaLlbApi {
    NiFpgaDll_OpenLlb: extern "C" fn(
        session: Session,
        memory_name: *const c_char,
        memory_size_to_host: *mut usize,
        virual_address_to_host: *mut *mut c_void,
        memory_size_to_fpga: *mut usize,
        virual_address_to_fpga: *mut *mut c_void,
    ) -> Status,
    NiFpgaDll_CloseLlb: extern "C" fn(session: Session, memory_name: *const c_char) -> Status,
}

// These are new API's for each LabVIEW year. Right now which ones go with equivelant years are unknown
// So each is their own optional.
#[derive(WrapperApi)]
pub struct NiFpgaCommitFifoConfigurationApi {
    NiFpgaDll_CommitFifoConfiguration: extern "C" fn(session: Session, fifo: u32) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaUnreserveFifoApi {
    NiFpgaDll_UnreserveFifo: extern "C" fn(session: Session, fifo: u32) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaReadFifoCompositeApi {
    NiFpgaDll_ReadFifoComposite: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut c_void,
        bytesPerElement: u32,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaWriteFifoCompositeApi {
    NiFpgaDll_WriteFifoComposite: extern "C" fn(
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
pub struct NiFpgaMapP2PSinkFifoApi {
    NiFpgaDll_MapP2PSinkFifo: extern "C" fn(
        session: Session,
        fifo: u32,
        size: *mut usize,
        virtualAddress: *mut *mut c_void,
    ) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaUnmapP2PSinkFifoApi {
    NiFpgaDll_UnmapP2PSinkFifo: extern "C" fn(session: Session, fifo: u32) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaFindRegisterPrivateApi {
    NiFpgaDll_FindRegisterPrivate: extern "C" fn(
        session: Session,
        resourceName: *const c_char,
        expectedResourceType: u32,
        resourceOffset: *mut u32,
    ) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaFindFifoPrivateApi {
    NiFpgaDll_FindFifoPrivate: extern "C" fn(
        session: Session,
        resourceName: *const c_char,
        expectedResourceType: u32,
        resourceOffset: *mut u32,
    ) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaClientFunctionCallApi {
    NiFpgaDll_ClientFunctionCall: extern "C" fn(
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
pub struct NiFpgaFindRegisterApi {
    NiFpgaDll_FindRegister: extern "C" fn(
        session: Session,
        registerName: *const c_char,
        registerOffset: *mut u32,
    ) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaFindFifoApi {
    NiFpgaDll_FindFifo:
        extern "C" fn(session: Session, fifoName: *const c_char, fifoNumber: *mut u32) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaGetFpgaViStateApi {
    NiFpgaDll_GetFpgaViState: extern "C" fn(session: Session, state: *mut u32) -> Status,
}

#[derive(WrapperApi)]
pub struct NiFpgaBaseApi {
    NiFpgaDll_Open: extern "C" fn(
        path: *const c_char,
        signature: *const c_char,
        resource: *const c_char,
        attribute: u32,
        session: *mut Session,
    ) -> Status,
    NiFpgaDll_Close: extern "C" fn(session: Session, attribute: u32) -> Status,
    NiFpgaDll_Run: extern "C" fn(session: Session, attribute: u32) -> Status,
    NiFpgaDll_Abort: extern "C" fn(session: Session) -> Status,
    NiFpgaDll_Reset: extern "C" fn(session: Session) -> Status,
    NiFpgaDll_Download: extern "C" fn(session: Session) -> Status,
    NiFpgaDll_ReadBool: extern "C" fn(session: Session, indicator: u32, value: *mut Bool) -> Status,
    NiFpgaDll_ReadI8: extern "C" fn(session: Session, indicator: u32, value: *mut i8) -> Status,
    NiFpgaDll_ReadU8: extern "C" fn(session: Session, indicator: u32, value: *mut u8) -> Status,
    NiFpgaDll_ReadI16: extern "C" fn(session: Session, indicator: u32, value: *mut i16) -> Status,
    NiFpgaDll_ReadU16: extern "C" fn(session: Session, indicator: u32, value: *mut u16) -> Status,
    NiFpgaDll_ReadI32: extern "C" fn(session: Session, indicator: u32, value: *mut i32) -> Status,
    NiFpgaDll_ReadU32: extern "C" fn(session: Session, indicator: u32, value: *mut u32) -> Status,
    NiFpgaDll_ReadI64: extern "C" fn(session: Session, indicator: u32, value: *mut i64) -> Status,
    NiFpgaDll_ReadU64: extern "C" fn(session: Session, indicator: u32, value: *mut u64) -> Status,
    NiFpgaDll_ReadSgl: extern "C" fn(session: Session, indicator: u32, value: *mut f32) -> Status,
    NiFpgaDll_ReadDbl: extern "C" fn(session: Session, indicator: u32, value: *mut f64) -> Status,
    NiFpgaDll_WriteBool: extern "C" fn(session: Session, control: u32, value: Bool) -> Status,
    NiFpgaDll_WriteI8: extern "C" fn(session: Session, control: u32, value: i8) -> Status,
    NiFpgaDll_WriteU8: extern "C" fn(session: Session, control: u32, value: u8) -> Status,
    NiFpgaDll_WriteI16: extern "C" fn(session: Session, control: u32, value: i16) -> Status,
    NiFpgaDll_WriteU16: extern "C" fn(session: Session, control: u32, value: u16) -> Status,
    NiFpgaDll_WriteI32: extern "C" fn(session: Session, control: u32, value: i32) -> Status,
    NiFpgaDll_WriteU32: extern "C" fn(session: Session, control: u32, value: u32) -> Status,
    NiFpgaDll_WriteI64: extern "C" fn(session: Session, control: u32, value: i64) -> Status,
    NiFpgaDll_WriteU64: extern "C" fn(session: Session, control: u32, value: u64) -> Status,
    NiFpgaDll_WriteSgl: extern "C" fn(session: Session, control: u32, value: f32) -> Status,
    NiFpgaDll_WriteDbl: extern "C" fn(session: Session, control: u32, value: f64) -> Status,
    NiFpgaDll_ReadArrayBool:
        extern "C" fn(session: Session, indicator: u32, array: *mut Bool, size: usize) -> Status,
    NiFpgaDll_ReadArrayI8:
        extern "C" fn(session: Session, indicator: u32, array: *mut i8, size: usize) -> Status,
    NiFpgaDll_ReadArrayU8:
        extern "C" fn(session: Session, indicator: u32, array: *mut u8, size: usize) -> Status,
    NiFpgaDll_ReadArrayI16:
        extern "C" fn(session: Session, indicator: u32, array: *mut i16, size: usize) -> Status,
    NiFpgaDll_ReadArrayU16:
        extern "C" fn(session: Session, indicator: u32, array: *mut u16, size: usize) -> Status,
    NiFpgaDll_ReadArrayI32:
        extern "C" fn(session: Session, indicator: u32, array: *mut i32, size: usize) -> Status,
    NiFpgaDll_ReadArrayU32:
        extern "C" fn(session: Session, indicator: u32, array: *mut u32, size: usize) -> Status,
    NiFpgaDll_ReadArrayI64:
        extern "C" fn(session: Session, indicator: u32, array: *mut i64, size: usize) -> Status,
    NiFpgaDll_ReadArrayU64:
        extern "C" fn(session: Session, indicator: u32, array: *mut u64, size: usize) -> Status,
    NiFpgaDll_ReadArraySgl:
        extern "C" fn(session: Session, indicator: u32, array: *mut f32, size: usize) -> Status,
    NiFpgaDll_ReadArrayDbl:
        extern "C" fn(session: Session, indicator: u32, array: *mut f64, size: usize) -> Status,
    NiFpgaDll_WriteArrayBool:
        extern "C" fn(session: Session, control: u32, array: *const Bool, size: usize) -> Status,
    NiFpgaDll_WriteArrayI8:
        extern "C" fn(session: Session, control: u32, array: *const i8, size: usize) -> Status,
    NiFpgaDll_WriteArrayU8:
        extern "C" fn(session: Session, control: u32, array: *const u8, size: usize) -> Status,
    NiFpgaDll_WriteArrayI16:
        extern "C" fn(session: Session, control: u32, array: *const i16, size: usize) -> Status,
    NiFpgaDll_WriteArrayU16:
        extern "C" fn(session: Session, control: u32, array: *const u16, size: usize) -> Status,
    NiFpgaDll_WriteArrayI32:
        extern "C" fn(session: Session, control: u32, array: *const i32, size: usize) -> Status,
    NiFpgaDll_WriteArrayU32:
        extern "C" fn(session: Session, control: u32, array: *const u32, size: usize) -> Status,
    NiFpgaDll_WriteArrayI64:
        extern "C" fn(session: Session, control: u32, array: *const i64, size: usize) -> Status,
    NiFpgaDll_WriteArrayU64:
        extern "C" fn(session: Session, control: u32, array: *const u64, size: usize) -> Status,
    NiFpgaDll_WriteArraySgl:
        extern "C" fn(session: Session, control: u32, array: *const f32, size: usize) -> Status,
    NiFpgaDll_WriteArrayDbl:
        extern "C" fn(session: Session, control: u32, array: *const f64, size: usize) -> Status,
    NiFpgaDll_ReserveIrqContext:
        extern "C" fn(session: Session, context: *mut IrqContext) -> Status,
    NiFpgaDll_UnreserveIrqContext: extern "C" fn(session: Session, context: IrqContext) -> Status,
    NiFpgaDll_WaitOnIrqs: extern "C" fn(
        session: Session,
        context: IrqContext,
        irqs: u32,
        timeout: u32,
        irqsAsserted: *mut u32,
        timedOut: *mut Bool,
    ) -> Status,
    NiFpgaDll_AcknowledgeIrqs: extern "C" fn(session: Session, irqs: u32) -> Status,
    NiFpgaDll_ConfigureFifo: extern "C" fn(session: Session, fifo: u32, depth: usize) -> Status,
    NiFpgaDll_ConfigureFifo2: extern "C" fn(
        session: Session,
        fifo: u32,
        requestedDepth: usize,
        actualDepth: *mut usize,
    ) -> Status,
    NiFpgaDll_StartFifo: extern "C" fn(session: Session, fifo: u32) -> Status,
    NiFpgaDll_StopFifo: extern "C" fn(session: Session, fifo: u32) -> Status,
    NiFpgaDll_ReadFifoBool: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut Bool,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoI8: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut i8,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoU8: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut u8,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoI16: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut i16,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoU16: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut u16,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoI32: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut i32,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoU32: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut u32,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoI64: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut i64,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoU64: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut u64,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoSgl: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut f32,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReadFifoDbl: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *mut f64,
        numberOfElements: usize,
        timeout: u32,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoBool: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const Bool,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoI8: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const i8,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoU8: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const u8,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoI16: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const i16,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoU16: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const u16,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoI32: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const i32,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoU32: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const u32,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoI64: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const i64,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoU64: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const u64,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoSgl: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const f32,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_WriteFifoDbl: extern "C" fn(
        session: Session,
        fifo: u32,
        data: *const f64,
        numberOfElements: usize,
        timeout: u32,
        emptyElementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsBool: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const Bool,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsI8: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const i8,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsU8: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const u8,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsI16: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const i16,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsU16: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const u16,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsI32: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const i32,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsU32: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const u32,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsI64: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const i64,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsU64: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const u64,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsSgl: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const f32,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoReadElementsDbl: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *const f64,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsBool: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut Bool,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsI8: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut i8,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsU8: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut u8,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsI16: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut i16,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsU16: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut u16,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsI32: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut i32,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsU32: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut u32,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsI64: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut i64,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsU64: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut u64,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsSgl: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut f32,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_AcquireFifoWriteElementsDbl: extern "C" fn(
        session: Session,
        fifo: u32,
        elements: *mut *mut f64,
        elementsRequested: usize,
        timeout: u32,
        elementsAcquired: *mut usize,
        elementsRemaining: *mut usize,
    ) -> Status,
    NiFpgaDll_ReleaseFifoElements:
        extern "C" fn(session: Session, fifo: u32, elements: usize) -> Status,
    NiFpgaDll_GetPeerToPeerFifoEndpoint:
        extern "C" fn(session: Session, fifo: u32, endpoint: *mut u32) -> Status,
    NiFpgaDll_GetBitfileContents:
        extern "C" fn(session: Session, contents: *mut *const c_char) -> Status,
}

#[derive(WrapperMultiApi)]
pub struct NiFpgaApi {
    pub base: NiFpgaBaseApi,
    pub hmb: Option<NiFpgaHmbApi>,

    pub commit_fifo_configuration: Option<NiFpgaCommitFifoConfigurationApi>,
    pub unreserve_fifo: Option<NiFpgaUnreserveFifoApi>,
    pub read_fifo_composite: Option<NiFpgaReadFifoCompositeApi>,
    pub write_fifo_composite: Option<NiFpgaWriteFifoCompositeApi>,
    pub map_p_2_p_sink_fifo: Option<NiFpgaMapP2PSinkFifoApi>,
    pub unmap_p_2_p_sink_fifo: Option<NiFpgaUnmapP2PSinkFifoApi>,
    pub find_register_private: Option<NiFpgaFindRegisterPrivateApi>,
    pub find_fifo_private: Option<NiFpgaFindFifoPrivateApi>,
    pub client_function_call: Option<NiFpgaClientFunctionCallApi>,
    pub find_register: Option<NiFpgaFindRegisterApi>,
    pub find_fifo: Option<NiFpgaFindFifoApi>,
    pub get_fpga_vi_state: Option<NiFpgaGetFpgaViStateApi>,
}

impl NiFpgaApi {
    pub fn load() -> Result<NiFpgaApiContainer, DlOpenError> {
        match unsafe { Container::load(platform_file_name("NiFpga")) } {
            Ok(api) => Ok(api),
            Err(err) => Err(DlOpenError(err)),
        }
    }
}
