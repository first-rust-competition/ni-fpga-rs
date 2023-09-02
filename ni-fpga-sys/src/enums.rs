bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct OpenAttribute: u32 {
        const NoRun = 1;
        const BitfilePathIsUTF8 = 2;
        const BitfileContentsNotPath = 1u32 << 30;
        const IgnoreSignatureArgument = 1u32 << 31;
        const _ = !0;
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct CloseAttribute: u32 {
        const NoResetIfLastSession = 1;
        const _ = !0;
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RunAttribute: u32 {
        const WaitUntilDone = 1;
        const _ = !0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FpgaViState {
    NotRunning = 0,
    Invalid = 1,
    Running = 2,
    NaturallyStopped = 3,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Irq: u32 {
        const Irq0  = 1u32 << 0;
        const Irq1  = 1u32 << 1;
        const Irq2  = 1u32 << 2;
        const Irq3  = 1u32 << 3;
        const Irq4  = 1u32 << 4;
        const Irq5  = 1u32 << 5;
        const Irq6  = 1u32 << 6;
        const Irq7  = 1u32 << 7;
        const Irq8  = 1u32 << 8;
        const Irq9  = 1u32 << 9;
        const Irq10 = 1u32 << 10;
        const Irq11 = 1u32 << 11;
        const Irq12 = 1u32 << 12;
        const Irq13 = 1u32 << 13;
        const Irq14 = 1u32 << 14;
        const Irq15 = 1u32 << 15;
        const Irq16 = 1u32 << 16;
        const Irq17 = 1u32 << 17;
        const Irq18 = 1u32 << 18;
        const Irq19 = 1u32 << 19;
        const Irq20 = 1u32 << 20;
        const Irq21 = 1u32 << 21;
        const Irq22 = 1u32 << 22;
        const Irq23 = 1u32 << 23;
        const Irq24 = 1u32 << 24;
        const Irq25 = 1u32 << 25;
        const Irq26 = 1u32 << 26;
        const Irq27 = 1u32 << 27;
        const Irq28 = 1u32 << 28;
        const Irq29 = 1u32 << 29;
        const Irq30 = 1u32 << 30;
        const Irq31 = 1u32 << 31;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FifoProperty {
    BytesPerElement = 1,
    HostBufferAllocationGranularity = 2,
    HostBufferSize = 3,
    HostBufferMirrorSize = 4,
    HostBufferType = 5,
    HostBuffer = 6,
    FlowControl = 7,
    ElementsCurrentlyAcquired = 8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HostBufferType {
    AllocatedByRIO = 1,
    AllocatedByUser = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FifoFlowControl {
    Disabled = 1,
    Enabled = 2,
}
