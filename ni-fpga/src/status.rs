use std::fmt;

use crate::ffi;

#[derive(PartialEq, Eq)]
#[repr(C)]
pub struct Status(ffi::Status);

#[allow(non_upper_case_globals)]
impl Status {
    pub const Success: Status = Status(0);
    pub const FifoTimeout: Status = Status(-50400);
    pub const MemoryFull: Status = Status(-52000);
    pub const SoftwareFault: Status = Status(-52003);
    pub const InvalidParameter: Status = Status(-52005);
    pub const ResourceNotFound: Status = Status(-52006);
    pub const ResourceNotInitialized: Status = Status(-52010);
    pub const FpgaAlreadyRunning: Status = Status(-61003);
    pub const DownloadError: Status = Status(-61018);
    pub const DeviceTypeMismatch: Status = Status(-61024);
    pub const CommunicationTimeout: Status = Status(-61046);
    pub const IrqTimeout: Status = Status(-61060);
    pub const CorruptBitfile: Status = Status(-61070);
    pub const BadDepth: Status = Status(-61072);
    pub const BadReadWriteCount: Status = Status(-61073);
    pub const ClockLostLock: Status = Status(-61083);
    pub const FpgaBusy: Status = Status(-61141);
    pub const FpgaBusyFpgaInterfaceCApi: Status = Status(-61200);
    pub const FpgaBusyScanInterface: Status = Status(-61201);
    pub const FpgaBusyFpgaInterface: Status = Status(-61202);
    pub const FpgaBusyInteractive: Status = Status(-61203);
    pub const FpgaBusyEmulation: Status = Status(-61204);
    pub const ResetCalledWithImplicitEnableRemoval: Status = Status(-61211);
    pub const AbortCalledWithImplicitEnableRemoval: Status = Status(-61212);
    pub const CloseAndResetCalledWithImplicitEnableRemoval: Status = Status(-61213);
    pub const ImplicitEnableRemovalButNotYetRun: Status = Status(-61214);
    pub const RunAfterStoppedCalledWithImplicitEnableRemoval: Status = Status(-61215);
    pub const GatedClockHandshakingViolation: Status = Status(-61216);
    pub const ElementsNotPermissibleToBeAcquired: Status = Status(-61219);
    pub const InternalError: Status = Status(-61499);
    pub const TotalDmaFifoDepthExceeded: Status = Status(-63003);
    pub const AccessDenied: Status = Status(-63033);
    pub const HostVersionMismatch: Status = Status(-63038);
    pub const RpcConnectionError: Status = Status(-63040);
    pub const RpcSessionError: Status = Status(-63043);
    pub const FifoReserved: Status = Status(-63082);
    pub const FifoElementsCurrentlyAcquired: Status = Status(-63083);
    pub const MisalignedAccess: Status = Status(-63084);
    pub const ControlOrIndicatorTooLarge: Status = Status(-63085);
    pub const InvalidUsage: Status = Status(-63100);
    pub const BitfileReadError: Status = Status(-63101);
    pub const SignatureMismatch: Status = Status(-63106);
    pub const IncompatibleBitfile: Status = Status(-63107);
    pub const InvalidResourceName: Status = Status(-63192);
    pub const FeatureNotSupported: Status = Status(-63193);
    pub const VersionMismatch: Status = Status(-63194);
    pub const InvalidSession: Status = Status(-63195);
    pub const OutOfHandles: Status = Status(-63198);
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            match *self {
                Status::Success => "No errors or warnings.",
                Status::FifoTimeout => "The timeout expired before the FIFO operation could complete.",
                Status::MemoryFull => "A memory allocation failed. Try again after rebooting.",
                Status::SoftwareFault => "An unexpected software error occurred.",
                Status::InvalidParameter => "A parameter to a function was not valid. This could be a NULL pointer, a bad value, etc.",
                Status::ResourceNotFound => "A required resource was not found. The NiFpga.* library, the RIO resource, or some other resource may be missing.",
                Status::ResourceNotInitialized => "A required resource was not properly initialized. This could occur if NiFpga_Initialize was not called or a required NiFpga_IrqContext was not reserved.",
                Status::FpgaAlreadyRunning => "The FPGA is already running.",
                Status::DownloadError => "An error occurred downloading the VI to the FPGA device. Verify that the target is connected and powered and that the resource of the target is properly configured.",
                Status::DeviceTypeMismatch => "The bitfile was not compiled for the specified resource's device type.",
                Status::CommunicationTimeout => "An error was detected in the communication between the host computer and the FPGA target.",
                Status::IrqTimeout => "The timeout expired before any of the IRQs were asserted.",
                Status::CorruptBitfile => "The specified bitfile is invalid or corrupt.",
                Status::BadDepth => "The requested FIFO depth is invalid. It is either 0 or an amount not supported by the hardware.",
                Status::BadReadWriteCount => "The number of FIFO elements is invalid. Either the number is greater than the depth of the host memory DMA FIFO, or more elements were requested for release than had been acquired.",
                Status::ClockLostLock => "A hardware clocking error occurred. A derived clock lost lock with its base clock during the execution of the LabVIEW FPGA VI. If any base clocks with derived clocks are referencing an external source, make sure that the external source is connected and within the supported frequency, jitter, accuracy, duty cycle, and voltage specifications. Also verify that the characteristics of the base clock match the configuration specified in the FPGA Base Clock Properties. If all base clocks with derived clocks are generated from free-running, on-board sources, please contact National Instruments technical support at ni.com/support.",
                Status::FpgaBusy => "The operation could not be performed because the FPGA is busy. Stop all activities on the FPGA before requesting this operation. If the target is in Scan Interface programming mode, put it in FPGA Interface programming mode.",
                Status::FpgaBusyFpgaInterfaceCApi => "The operation could not be performed because the FPGA is busy operating in FPGA Interface C API mode. Stop all activities on the FPGA before requesting this operation.",
                Status::FpgaBusyScanInterface => "The chassis is in Scan Interface programming mode. In order to run FPGA VIs, you must go to the chassis properties page, select FPGA programming mode, and deploy settings.",
                Status::FpgaBusyFpgaInterface => "The operation could not be performed because the FPGA is busy operating in FPGA Interface mode. Stop all activities on the FPGA before requesting this operation.",
                Status::FpgaBusyInteractive => "The operation could not be performed because the FPGA is busy operating in Interactive mode. Stop all activities on the FPGA before requesting this operation.",
                Status::FpgaBusyEmulation => "The operation could not be performed because the FPGA is busy operating in Emulation mode. Stop all activities on the FPGA before requesting this operation.",
                Status::ResetCalledWithImplicitEnableRemoval => "LabVIEW FPGA does not support the Reset method for bitfiles that allow removal of implicit enable signals in single-cycle Timed Loops.",
                Status::AbortCalledWithImplicitEnableRemoval => "LabVIEW FPGA does not support the Abort method for bitfiles that allow removal of implicit enable signals in single-cycle Timed Loops.",
                Status::CloseAndResetCalledWithImplicitEnableRemoval => "LabVIEW FPGA does not support Close and Reset if Last Reference for bitfiles that allow removal of implicit enable signals in single-cycle Timed Loops. Pass NiFpga_CloseAttribute_NoResetIflastSession to NiFpga_Close instead.",
                Status::ImplicitEnableRemovalButNotYetRun => "For bitfiles that allow removal of implicit enable signals in single-cycle Timed Loops, LabVIEW FPGA does not support this method prior to running the bitfile.",
                Status::RunAfterStoppedCalledWithImplicitEnableRemoval => "Bitfiles that allow removal of implicit enable signals in single-cycle Timed Loops can run only once. Download the bitfile again before re-running the VI.",
                Status::GatedClockHandshakingViolation => "A gated clock has violated the handshaking protocol. If you are using external gated clocks, ensure that they follow the required clock gating protocol. If you are generating your clocks internally, please contact National Instruments Technical Support.",
                Status::ElementsNotPermissibleToBeAcquired => "The number of elements requested must be less than or equal to the number of unacquired elements left in the host memory DMA FIFO. There are currently fewer unacquired elements left in the FIFO than are being requested. Release some acquired elements before acquiring more elements.",
                Status::InternalError => "An unexpected internal error occurred.",
                Status::TotalDmaFifoDepthExceeded => "The NI-RIO driver was unable to allocate memory for a FIFO. This can happen when the combined depth of all DMA FIFOs exceeds the maximum depth for the controller, or when the controller runs out of system memory. You may be able to reconfigure the controller with a greater maximum FIFO depth. For more information, refer to NI KnowledgeBase article 65OF2ERQ.",
                Status::AccessDenied => "Access to the remote system was denied. Use MAX to check the Remote Device Access settings under Software»NI-RIO»NI-RIO Settings on the remote system.",
                Status::HostVersionMismatch => "The NI-RIO software on the host is not compatible with the software on the target. Upgrade the NI-RIO software on the host in order to connect to this target.",
                Status::RpcConnectionError => "A connection could not be established to the specified remote device. Ensure that the device is on and accessible over the network, that NI-RIO software is installed, and that the RIO server is running and properly configured.",
                Status::RpcSessionError => "The RPC session is invalid. The target may have reset or been rebooted. Check the network connection and retry the operation.",
                Status::FifoReserved => "The operation could not complete because another session is accessing the FIFO. Close the other session and retry.",
                Status::FifoElementsCurrentlyAcquired => "A Read FIFO or Write FIFO function was called while the host had acquired elements of the FIFO. Release all acquired elements before reading or writing.",
                Status::MisalignedAccess => "A function was called using a misaligned address. The address must be a multiple of the size of the datatype.",
                Status::ControlOrIndicatorTooLarge => "The FPGA Read/Write Control Function is accessing a control or indicator with data that exceeds the maximum size supported on the current target. Refer to the hardware documentation for the limitations on data types for this target.",
                Status::InvalidUsage => "Invalid usage. The syntax is as follows: capigen.exe [-e] [<bitfile> [<output_directory> [<prefix_override>]]]",
                Status::BitfileReadError => "A valid .lvbitx bitfile is required. If you are using a vaild .lvbitx bitfile, the bitfile may not be compatible with the software you are using. Determine which version of LabVIEW was used to make the bitfile, update your software to that version or later, and try again.",
                Status::SignatureMismatch => "The specified signature does not match the signature of the bitfile. If the bitfile has been recompiled, regenerate the C API and rebuild the application.",
                Status::IncompatibleBitfile => "The bitfile you are trying to use is not compatible with the version of NI-RIO installed on the target and/or the host. Determine which versions of NI-RIO and LabVIEW were used to make the bitfile, update the software on the target and host to that version or later, and try again.",
                Status::InvalidResourceName => "Either the supplied resource name is invalid as a RIO resource name, or the device was not found. Use MAX to find the proper resource name for the intended device.",
                Status::FeatureNotSupported => "The requested feature is not supported.",
                Status::VersionMismatch => "The NI-RIO software on the target system is not compatible with this software. Upgrade the NI-RIO software on the target system.",
                Status::InvalidSession => "The session is invalid or has been closed.",
                Status::OutOfHandles => "The maximum number of open FPGA sessions has been reached. Close some open sessions.",
                _ => "Unknown.",
            },
            self.0,
        )
    }
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl From<ffi::Status> for Status {
    fn from(status: ffi::Status) -> Self {
        Status(status)
    }
}

impl From<Status> for ffi::Status {
    fn from(status: Status) -> Self {
        status.0
    }
}
