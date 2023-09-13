#![allow(warnings)]

pub mod types {
    use super::types;
    use ni_fpga_macros::{Cluster, Enum};
    #[derive(Cluster, Debug, PartialEq, Clone, Copy)]
    pub struct TestCluster {
        pub b: bool,
        pub u: u16,
    }
    #[derive(Cluster, Debug, PartialEq, Clone, Copy)]
    pub struct TestClusterArray {
        pub b: bool,
        pub u: u16,
    }
}
pub struct FpgaBitfile {
    pub U8: Option<ni_fpga::Register<u8, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub U16: Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub U32: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub U64: Option<ni_fpga::Register<u64, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub I8: Option<ni_fpga::Register<i8, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub I16: Option<ni_fpga::Register<i16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub I32: Option<ni_fpga::Register<i32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub I64: Option<ni_fpga::Register<i64, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub SGL: Option<ni_fpga::Register<f32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub UnsignedFXP: Option<
        ni_fpga::Register<ni_fpga::fxp::FXP<4, 3, false>, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub SignedFXP: Option<
        ni_fpga::Register<ni_fpga::fxp::FXP<4, 3, true>, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub TrueBool: Option<ni_fpga::Register<bool, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub FalseBool: Option<ni_fpga::Register<bool, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub BoolArray: Option<ni_fpga::Register<[bool; 8], ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub TestCluster:
        Option<ni_fpga::Register<types::TestCluster, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub TestClusterArray: Option<
        ni_fpga::Register<[types::TestClusterArray; 2], ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub ViControl: Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub DiagramReset: Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub InterruptEnable: Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub InterruptMask: Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub InterruptStatus: Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
}
impl FpgaBitfile {
    pub fn take(session: &impl ni_fpga::SessionAccess) -> Result<Self, ni_fpga::Error> {
        static REGISTERS: std::sync::Mutex<Option<()>> = std::sync::Mutex::new(Some(()));
        let mut lock = REGISTERS.lock().unwrap();
        let contents = lock.take();
        drop(lock);
        if contents.is_none() {
            return Err(ni_fpga::Error::ResourceAlreadyTaken);
        }
        Ok(Self {
            U8: Some(unsafe { ni_fpga::Register::new(session.find_offset("U8")?) }),
            U16: Some(unsafe { ni_fpga::Register::new(session.find_offset("U16")?) }),
            U32: Some(unsafe { ni_fpga::Register::new(session.find_offset("U32")?) }),
            U64: Some(unsafe { ni_fpga::Register::new(session.find_offset("U64")?) }),
            I8: Some(unsafe { ni_fpga::Register::new(session.find_offset("I8")?) }),
            I16: Some(unsafe { ni_fpga::Register::new(session.find_offset("I16")?) }),
            I32: Some(unsafe { ni_fpga::Register::new(session.find_offset("I32")?) }),
            I64: Some(unsafe { ni_fpga::Register::new(session.find_offset("I64")?) }),
            SGL: Some(unsafe { ni_fpga::Register::new(session.find_offset("SGL")?) }),
            UnsignedFXP: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("UnsignedFXP")?)
            }),
            SignedFXP: Some(unsafe { ni_fpga::Register::new(session.find_offset("SignedFXP")?) }),
            TrueBool: Some(unsafe { ni_fpga::Register::new(session.find_offset("TrueBool")?) }),
            FalseBool: Some(unsafe { ni_fpga::Register::new(session.find_offset("FalseBool")?) }),
            BoolArray: Some(unsafe { ni_fpga::Register::new(session.find_offset("BoolArray")?) }),
            TestCluster: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("TestCluster")?)
            }),
            TestClusterArray: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("TestClusterArray")?)
            }),
            ViControl: Some(unsafe { ni_fpga::Register::new(session.find_offset("ViControl")?) }),
            DiagramReset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DiagramReset")?)
            }),
            InterruptEnable: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("InterruptEnable")?)
            }),
            InterruptMask: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("InterruptMask")?)
            }),
            InterruptStatus: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("InterruptStatus")?)
            }),
        })
    }
    pub const fn contents() -> &'static str {
        include_str!("integration.lvbitx")
    }
    pub const fn signature() -> &'static str {
        "D08F17F77A45A5692FA2342C6B86E0EE"
    }
    pub fn session_builder(
        resource: impl AsRef<str>,
    ) -> Result<ni_fpga::SessionBuilder, ni_fpga::Error> {
        ni_fpga::SessionBuilder::new()
            .bitfile_contents(Self::contents())?
            .signature(Self::signature())?
            .resource(resource)
    }
}
