#![allow(warnings)]

pub mod types {
    use super::types;
    use ni_fpga_macros::{Cluster, Enum};
    #[derive(Cluster, Debug)]
    pub struct AI_Config {
        pub ScanSize: ni_fpga::fxp::FXP<3, 3, false>,
        pub ConvertRate: ni_fpga::fxp::FXP<26, 26, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct AI_ReadSelect {
        pub Channel: ni_fpga::fxp::FXP<3, 3, false>,
        pub Averaged: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct ASource {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Accumulator0_Output {
        pub Value: i64,
        pub Count: u32,
    }
    #[derive(Cluster, Debug)]
    pub struct Accumulator1_Output {
        pub Value: i64,
        pub Count: u32,
    }
    #[derive(Cluster, Debug)]
    pub struct AnalogTrigger0_SourceSelect {
        pub Channel: ni_fpga::fxp::FXP<3, 3, false>,
        pub Averaged: bool,
        pub DutyCycle: bool,
        pub Filter: bool,
        pub FloatingRollover: bool,
        pub RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct AnalogTrigger1_SourceSelect {
        pub Channel: ni_fpga::fxp::FXP<3, 3, false>,
        pub Averaged: bool,
        pub DutyCycle: bool,
        pub Filter: bool,
        pub FloatingRollover: bool,
        pub RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct AnalogTrigger2_SourceSelect {
        pub Channel: ni_fpga::fxp::FXP<3, 3, false>,
        pub Averaged: bool,
        pub DutyCycle: bool,
        pub Filter: bool,
        pub FloatingRollover: bool,
        pub RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct AnalogTrigger3_SourceSelect {
        pub Channel: ni_fpga::fxp::FXP<3, 3, false>,
        pub Averaged: bool,
        pub DutyCycle: bool,
        pub Filter: bool,
        pub FloatingRollover: bool,
        pub RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct AnalogTrigger4_SourceSelect {
        pub Channel: ni_fpga::fxp::FXP<3, 3, false>,
        pub Averaged: bool,
        pub DutyCycle: bool,
        pub Filter: bool,
        pub FloatingRollover: bool,
        pub RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct AnalogTrigger5_SourceSelect {
        pub Channel: ni_fpga::fxp::FXP<3, 3, false>,
        pub Averaged: bool,
        pub DutyCycle: bool,
        pub Filter: bool,
        pub FloatingRollover: bool,
        pub RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct AnalogTrigger6_SourceSelect {
        pub Channel: ni_fpga::fxp::FXP<3, 3, false>,
        pub Averaged: bool,
        pub DutyCycle: bool,
        pub Filter: bool,
        pub FloatingRollover: bool,
        pub RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct AnalogTrigger7_SourceSelect {
        pub Channel: ni_fpga::fxp::FXP<3, 3, false>,
        pub Averaged: bool,
        pub DutyCycle: bool,
        pub Filter: bool,
        pub FloatingRollover: bool,
        pub RolloverLimit: ni_fpga::fxp::FXP<8, 12, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct AnalogTrigger_Output {
        pub InHysteresis: bool,
        pub OverLimit: bool,
        pub Rising: bool,
        pub Falling: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct BSource {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter0_Config {
        pub UpSource: types::UpSource,
        pub DownSource: types::DownSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub UpRisingEdge: bool,
        pub UpFallingEdge: bool,
        pub DownRisingEdge: bool,
        pub DownFallingEdge: bool,
        pub Mode: ni_fpga::fxp::FXP<2, 2, false>,
        pub PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter0_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter0_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter0_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter1_Config {
        pub UpSource: types::UpSource,
        pub DownSource: types::DownSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub UpRisingEdge: bool,
        pub UpFallingEdge: bool,
        pub DownRisingEdge: bool,
        pub DownFallingEdge: bool,
        pub Mode: ni_fpga::fxp::FXP<2, 2, false>,
        pub PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter1_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter1_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter1_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter2_Config {
        pub UpSource: types::UpSource,
        pub DownSource: types::DownSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub UpRisingEdge: bool,
        pub UpFallingEdge: bool,
        pub DownRisingEdge: bool,
        pub DownFallingEdge: bool,
        pub Mode: ni_fpga::fxp::FXP<2, 2, false>,
        pub PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter2_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter2_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter2_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter3_Config {
        pub UpSource: types::UpSource,
        pub DownSource: types::DownSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub UpRisingEdge: bool,
        pub UpFallingEdge: bool,
        pub DownRisingEdge: bool,
        pub DownFallingEdge: bool,
        pub Mode: ni_fpga::fxp::FXP<2, 2, false>,
        pub PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter3_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter3_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter3_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter4_Config {
        pub UpSource: types::UpSource,
        pub DownSource: types::DownSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub UpRisingEdge: bool,
        pub UpFallingEdge: bool,
        pub DownRisingEdge: bool,
        pub DownFallingEdge: bool,
        pub Mode: ni_fpga::fxp::FXP<2, 2, false>,
        pub PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter4_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter4_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter4_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter5_Config {
        pub UpSource: types::UpSource,
        pub DownSource: types::DownSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub UpRisingEdge: bool,
        pub UpFallingEdge: bool,
        pub DownRisingEdge: bool,
        pub DownFallingEdge: bool,
        pub Mode: ni_fpga::fxp::FXP<2, 2, false>,
        pub PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter5_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter5_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter5_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter6_Config {
        pub UpSource: types::UpSource,
        pub DownSource: types::DownSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub UpRisingEdge: bool,
        pub UpFallingEdge: bool,
        pub DownRisingEdge: bool,
        pub DownFallingEdge: bool,
        pub Mode: ni_fpga::fxp::FXP<2, 2, false>,
        pub PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter6_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter6_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter6_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter7_Config {
        pub UpSource: types::UpSource,
        pub DownSource: types::DownSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub UpRisingEdge: bool,
        pub UpFallingEdge: bool,
        pub DownRisingEdge: bool,
        pub DownFallingEdge: bool,
        pub Mode: ni_fpga::fxp::FXP<2, 2, false>,
        pub PulseLengthThreshold: ni_fpga::fxp::FXP<6, 14, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter7_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter7_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Counter7_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct DIO_DI {
        pub Headers: ni_fpga::fxp::FXP<10, 10, false>,
        pub SPIPort: ni_fpga::fxp::FXP<5, 5, false>,
        pub Reserved: ni_fpga::fxp::FXP<1, 1, false>,
        pub MXP: u16,
    }
    #[derive(Cluster, Debug)]
    pub struct DIO_DO {
        pub Headers: ni_fpga::fxp::FXP<10, 10, false>,
        pub SPIPort: ni_fpga::fxp::FXP<5, 5, false>,
        pub Reserved: ni_fpga::fxp::FXP<1, 1, false>,
        pub MXP: u16,
    }
    #[derive(Cluster, Debug)]
    pub struct DIO_OutputEnable {
        pub Headers: ni_fpga::fxp::FXP<10, 10, false>,
        pub SPIPort: ni_fpga::fxp::FXP<5, 5, false>,
        pub Reserved: ni_fpga::fxp::FXP<1, 1, false>,
        pub MXP: u16,
    }
    #[derive(Cluster, Debug)]
    pub struct DIO_Pulse {
        pub Headers: ni_fpga::fxp::FXP<10, 10, false>,
        pub SPIPort: ni_fpga::fxp::FXP<5, 5, false>,
        pub Reserved: ni_fpga::fxp::FXP<1, 1, false>,
        pub MXP: u16,
    }
    #[derive(Cluster, Debug)]
    pub struct DMA_Config {
        pub Pause: bool,
        pub Enable: types::Enable,
        pub ExternalClock: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct DownSource {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct DutyCycle0_Source {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct DutyCycle1_Source {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct DutyCycle2_Source {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct DutyCycle3_Source {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct DutyCycle4_Source {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct DutyCycle5_Source {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct DutyCycle6_Source {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct DutyCycle7_Source {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Enable {
        pub AI0_Low: bool,
        pub AI0_High: bool,
        pub AIAveraged0_Low: bool,
        pub AIAveraged0_High: bool,
        pub AI1_Low: bool,
        pub AI1_High: bool,
        pub AIAveraged1_Low: bool,
        pub AIAveraged1_High: bool,
        pub Accumulator0: bool,
        pub Accumulator1: bool,
        pub DI: bool,
        pub AnalogTriggers: bool,
        pub Counters_Low: bool,
        pub Counters_High: bool,
        pub CounterTimers_Low: bool,
        pub CounterTimers_High: bool,
        pub Encoders_Low: bool,
        pub Encoders_High: bool,
        pub EncoderTimers_Low: bool,
        pub EncoderTimers_High: bool,
        pub DutyCycle_Low: bool,
        pub DutyCycle_High: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Enables {
        pub AI0_Low: bool,
        pub AI0_High: bool,
        pub AIAveraged0_Low: bool,
        pub AIAveraged0_High: bool,
        pub AI1_Low: bool,
        pub AI1_High: bool,
        pub AIAveraged1_Low: bool,
        pub AIAveraged1_High: bool,
        pub Accumulator0: bool,
        pub Accumulator1: bool,
        pub DI: bool,
        pub AnalogTriggers: bool,
        pub Counters_Low: bool,
        pub Counters_High: bool,
        pub CounterTimers_Low: bool,
        pub CounterTimers_High: bool,
        pub Encoders_Low: bool,
        pub Encoders_High: bool,
        pub EncoderTimers_Low: bool,
        pub EncoderTimers_High: bool,
        pub DutyCycle_Low: bool,
        pub DutyCycle_High: bool,
        pub Interrupts: bool,
        pub PWM: bool,
        pub PWM_MXP: bool,
        pub Relay_DO_AO: bool,
        pub Timestamp: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder0_Config {
        pub ASource: types::ASource,
        pub BSource: types::BSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub Reverse: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder0_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder0_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder0_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder1_Config {
        pub ASource: types::ASource,
        pub BSource: types::BSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub Reverse: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder1_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder1_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder1_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder2_Config {
        pub ASource: types::ASource,
        pub BSource: types::BSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub Reverse: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder2_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder2_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder2_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder3_Config {
        pub ASource: types::ASource,
        pub BSource: types::BSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub Reverse: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder3_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder3_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder3_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder4_Config {
        pub ASource: types::ASource,
        pub BSource: types::BSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub Reverse: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder4_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder4_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder4_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder5_Config {
        pub ASource: types::ASource,
        pub BSource: types::BSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub Reverse: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder5_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder5_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder5_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder6_Config {
        pub ASource: types::ASource,
        pub BSource: types::BSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub Reverse: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder6_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder6_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder6_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder7_Config {
        pub ASource: types::ASource,
        pub BSource: types::BSource,
        pub IndexSource: types::IndexSource,
        pub IndexActiveHigh: bool,
        pub IndexEdgeSensitive: bool,
        pub Reverse: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder7_Output {
        pub Direction: bool,
        pub Value: ni_fpga::fxp::FXP<31, 31, true>,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder7_TimerConfig {
        pub StallPeriod: ni_fpga::fxp::FXP<24, 25, false>,
        pub AverageSize: ni_fpga::fxp::FXP<7, 7, false>,
        pub UpdateWhenEmpty: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Encoder7_TimerOutput {
        pub Period: ni_fpga::fxp::FXP<23, 24, false>,
        pub Count: ni_fpga::fxp::FXP<8, 8, true>,
        pub Stalled: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct ExternalClockSource {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct HMB_Config {
        pub Enables: types::Enables,
    }
    #[derive(Cluster, Debug)]
    pub struct IndexSource {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Interrupt0_Config {
        pub Source: types::Source,
        pub RisingEdge: bool,
        pub FallingEdge: bool,
        pub WaitForAck: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Interrupt1_Config {
        pub Source: types::Source,
        pub RisingEdge: bool,
        pub FallingEdge: bool,
        pub WaitForAck: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Interrupt2_Config {
        pub Source: types::Source,
        pub RisingEdge: bool,
        pub FallingEdge: bool,
        pub WaitForAck: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Interrupt3_Config {
        pub Source: types::Source,
        pub RisingEdge: bool,
        pub FallingEdge: bool,
        pub WaitForAck: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Interrupt4_Config {
        pub Source: types::Source,
        pub RisingEdge: bool,
        pub FallingEdge: bool,
        pub WaitForAck: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Interrupt5_Config {
        pub Source: types::Source,
        pub RisingEdge: bool,
        pub FallingEdge: bool,
        pub WaitForAck: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Interrupt6_Config {
        pub Source: types::Source,
        pub RisingEdge: bool,
        pub FallingEdge: bool,
        pub WaitForAck: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Interrupt7_Config {
        pub Source: types::Source,
        pub RisingEdge: bool,
        pub FallingEdge: bool,
        pub WaitForAck: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct LEDs {
        pub Comm: u8,
        pub Mode: u8,
        pub RSL: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct PWM_Config {
        pub Period: u16,
        pub MinHigh: u16,
    }
    #[derive(Cluster, Debug)]
    pub struct Power_Disable {
        pub User3V3: bool,
        pub User5V: bool,
        pub User6V: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct Power_FaultCounts {
        pub OverCurrentFaultCount3V3: u8,
        pub OverCurrentFaultCount5V: u8,
        pub OverCurrentFaultCount6V: u8,
        pub UnderVoltageFaultCount5V: u8,
    }
    #[derive(Cluster, Debug)]
    pub struct Power_Status {
        pub User3V3: u8,
        pub User5V: u8,
        pub User6V: u8,
    }
    #[derive(Cluster, Debug)]
    pub struct Relay_Value {
        pub Forward: ni_fpga::fxp::FXP<4, 4, false>,
        pub Reverse: ni_fpga::fxp::FXP<4, 4, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct SPI_AutoByteCount {
        pub TxByteCount: ni_fpga::fxp::FXP<5, 5, false>,
        pub ZeroByteCount: ni_fpga::fxp::FXP<7, 7, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct SPI_AutoTriggerConfig {
        pub ExternalClockSource: types::ExternalClockSource,
        pub RisingEdge: bool,
        pub FallingEdge: bool,
        pub ExternalClock: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct SPI_ChipSelectActiveHigh {
        pub Hdr: ni_fpga::fxp::FXP<4, 4, false>,
        pub MXP: ni_fpga::fxp::FXP<1, 1, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct SPI_StallConfig {
        pub Pow2BytesPerRead: u8,
        pub StallTicks: u16,
        pub CsToSclkTicks: u8,
    }
    #[derive(Cluster, Debug)]
    pub struct Source {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct SysWatchdog_Status {
        pub SystemActive: bool,
        pub PowerAlive: bool,
        pub SysDisableCount: ni_fpga::fxp::FXP<15, 15, false>,
        pub PowerDisableCount: ni_fpga::fxp::FXP<15, 15, false>,
    }
    #[derive(Cluster, Debug)]
    pub struct Trigger {
        pub ExternalClockSource: types::ExternalClockSource,
        pub RisingEdge: bool,
        pub FallingEdge: bool,
    }
    #[derive(Cluster, Debug)]
    pub struct UpSource {
        pub Channel: ni_fpga::fxp::FXP<4, 4, false>,
        pub Module: ni_fpga::fxp::FXP<1, 1, false>,
        pub AnalogTrigger: bool,
    }
    #[derive(Debug, Enum)]
    pub enum SPI_DebugState {
        Idle,
        Check_Window,
        Check_Available,
        Set_Fifo_Mark,
        Enable_SPI,
        Stuff_Fifo,
        Check_Mark,
        Shuffle_Data,
        Disable,
    }
}
pub struct FpgaBitfile {
    pub LocalTime: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Revision: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Version: Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub LocalTimeUpper: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub LEDs: Option<ni_fpga::Register<types::LEDs, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub UserButton: Option<ni_fpga::Register<bool, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub InterruptForceOnce:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub InterruptForceNumber:
        Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SysWatchdog_Status: Option<
        ni_fpga::Register<types::SysWatchdog_Status, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub SysWatchdog_Command:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SysWatchdog_Challenge:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub SysWatchdog_Timer: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub SysWatchdog_Active:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SysWatchdog_ForcedKills: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<15, 15, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AI_ReadSelect:
        Option<ni_fpga::Register<types::AI_ReadSelect, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AI_LatchOutput: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AI_Output: Option<ni_fpga::Register<i32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub AI_Config:
        Option<ni_fpga::Register<types::AI_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AI_ScanList: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<3, 3, false>; 8],
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AI_OversampleBits: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<4, 4, false>; 8],
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AI_AverageBits: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<4, 4, false>; 8],
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AI_LoopTiming: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Accumulator0_Center:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Accumulator0_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Accumulator0_Output: Option<
        ni_fpga::Register<types::Accumulator0_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Accumulator0_Deadband:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Accumulator1_Center:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Accumulator1_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Accumulator1_Output: Option<
        ni_fpga::Register<types::Accumulator1_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Accumulator1_Deadband:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger_Output: Option<
        ni_fpga::Register<
            [types::AnalogTrigger_Output; 8],
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AnalogTrigger0_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger0_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AnalogTrigger0_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger0_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger1_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger1_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AnalogTrigger1_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger1_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger2_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger2_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AnalogTrigger2_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger2_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger3_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger3_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AnalogTrigger3_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger3_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger4_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger4_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AnalogTrigger4_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger4_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger5_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger5_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AnalogTrigger6_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger6_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger6_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger6_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AnalogTrigger5_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger5_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger7_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger7_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub AnalogTrigger7_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AnalogTrigger7_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub PWM_LoopTiming: Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub PWM_CycleStartTimeUpper:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub PWM_CycleStartTime:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub PWM_Config:
        Option<ni_fpga::Register<types::PWM_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub PWM_PeriodScaleHdr: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<2, 2, false>; 10],
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_PeriodScaleMXP: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<2, 2, false>; 10],
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_ZeroLatch:
        Option<ni_fpga::Register<[bool; 20], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub PWM_Hdr0: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_Hdr1: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_Hdr2: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_Hdr3: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_Hdr4: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_Hdr5: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_Hdr6: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_Hdr7: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_Hdr8: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_Hdr9: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_MXP0: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_MXP1: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_MXP2: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_MXP3: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_MXP4: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_MXP5: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_MXP6: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_MXP7: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_MXP8: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub PWM_MXP9: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DIO_OutputEnable: Option<
        ni_fpga::Register<types::DIO_OutputEnable, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub DIO_DO: Option<ni_fpga::Register<types::DIO_DO, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub DIO_DI: Option<ni_fpga::Register<types::DIO_DI, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub DIO_FilterSelectHdr: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<2, 2, false>; 16],
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DIO_FilterPeriodHdr0: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<24, 24, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DIO_FilterPeriodHdr1: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<24, 24, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DIO_FilterPeriodHdr2: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<24, 24, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DIO_FilterSelectMXP: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<2, 2, false>; 16],
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DIO_FilterPeriodMXP0: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<24, 24, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DIO_FilterPeriodMXP1: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<24, 24, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DIO_FilterPeriodMXP2: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<24, 24, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DIO_EnableMXPSpecialFunction:
        Option<ni_fpga::Register<u16, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub DIO_PulseLength: Option<ni_fpga::Register<u16, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub DIO_Pulse:
        Option<ni_fpga::Register<types::DIO_Pulse, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub DIO_PWMDutyCycleA:
        Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub DIO_PWMDutyCycleB:
        Option<ni_fpga::Register<[u8; 2], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub DIO_PWMOutputSelect: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<5, 5, false>; 6],
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DIO_PWMPeriodPower:
        Option<ni_fpga::Register<u16, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Counter0_Config: Option<
        ni_fpga::Register<types::Counter0_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter0_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Counter0_Output:
        Option<ni_fpga::Register<types::Counter0_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Counter0_TimerConfig: Option<
        ni_fpga::Register<types::Counter0_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter0_TimerOutput: Option<
        ni_fpga::Register<types::Counter0_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Counter1_Config: Option<
        ni_fpga::Register<types::Counter1_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter1_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Counter1_Output:
        Option<ni_fpga::Register<types::Counter1_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Counter1_TimerConfig: Option<
        ni_fpga::Register<types::Counter1_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter1_TimerOutput: Option<
        ni_fpga::Register<types::Counter1_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Counter2_Config: Option<
        ni_fpga::Register<types::Counter2_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter2_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Counter2_Output:
        Option<ni_fpga::Register<types::Counter2_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Counter2_TimerConfig: Option<
        ni_fpga::Register<types::Counter2_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter2_TimerOutput: Option<
        ni_fpga::Register<types::Counter2_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Counter3_Config: Option<
        ni_fpga::Register<types::Counter3_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter3_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Counter3_Output:
        Option<ni_fpga::Register<types::Counter3_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Counter3_TimerConfig: Option<
        ni_fpga::Register<types::Counter3_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter3_TimerOutput: Option<
        ni_fpga::Register<types::Counter3_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Counter4_Config: Option<
        ni_fpga::Register<types::Counter4_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter4_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Counter4_Output:
        Option<ni_fpga::Register<types::Counter4_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Counter4_TimerConfig: Option<
        ni_fpga::Register<types::Counter4_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter4_TimerOutput: Option<
        ni_fpga::Register<types::Counter4_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Counter5_Config: Option<
        ni_fpga::Register<types::Counter5_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter5_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Counter5_Output:
        Option<ni_fpga::Register<types::Counter5_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Counter5_TimerConfig: Option<
        ni_fpga::Register<types::Counter5_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter5_TimerOutput: Option<
        ni_fpga::Register<types::Counter5_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Counter6_Config: Option<
        ni_fpga::Register<types::Counter6_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter6_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Counter6_Output:
        Option<ni_fpga::Register<types::Counter6_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Counter6_TimerConfig: Option<
        ni_fpga::Register<types::Counter6_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter6_TimerOutput: Option<
        ni_fpga::Register<types::Counter6_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Counter7_Config: Option<
        ni_fpga::Register<types::Counter7_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter7_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Counter7_Output:
        Option<ni_fpga::Register<types::Counter7_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Counter7_TimerConfig: Option<
        ni_fpga::Register<types::Counter7_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Counter7_TimerOutput: Option<
        ni_fpga::Register<types::Counter7_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Encoder0_Config: Option<
        ni_fpga::Register<types::Encoder0_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder0_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Encoder0_Output:
        Option<ni_fpga::Register<types::Encoder0_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Encoder0_TimerConfig: Option<
        ni_fpga::Register<types::Encoder0_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder0_TimerOutput: Option<
        ni_fpga::Register<types::Encoder0_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Encoder1_Config: Option<
        ni_fpga::Register<types::Encoder1_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder1_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Encoder1_Output:
        Option<ni_fpga::Register<types::Encoder1_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Encoder1_TimerConfig: Option<
        ni_fpga::Register<types::Encoder1_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder1_TimerOutput: Option<
        ni_fpga::Register<types::Encoder1_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Encoder2_Config: Option<
        ni_fpga::Register<types::Encoder2_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder2_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Encoder2_Output:
        Option<ni_fpga::Register<types::Encoder2_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Encoder2_TimerConfig: Option<
        ni_fpga::Register<types::Encoder2_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder2_TimerOutput: Option<
        ni_fpga::Register<types::Encoder2_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Encoder3_Config: Option<
        ni_fpga::Register<types::Encoder3_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder3_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Encoder3_Output:
        Option<ni_fpga::Register<types::Encoder3_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Encoder3_TimerConfig: Option<
        ni_fpga::Register<types::Encoder3_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder3_TimerOutput: Option<
        ni_fpga::Register<types::Encoder3_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Encoder4_Config: Option<
        ni_fpga::Register<types::Encoder4_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder4_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Encoder4_Output:
        Option<ni_fpga::Register<types::Encoder4_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Encoder4_TimerConfig: Option<
        ni_fpga::Register<types::Encoder4_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder4_TimerOutput: Option<
        ni_fpga::Register<types::Encoder4_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Encoder5_Config: Option<
        ni_fpga::Register<types::Encoder5_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder5_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Encoder5_Output:
        Option<ni_fpga::Register<types::Encoder5_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Encoder5_TimerConfig: Option<
        ni_fpga::Register<types::Encoder5_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder5_TimerOutput: Option<
        ni_fpga::Register<types::Encoder5_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Encoder6_Config: Option<
        ni_fpga::Register<types::Encoder6_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder6_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Encoder6_Output:
        Option<ni_fpga::Register<types::Encoder6_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Encoder6_TimerConfig: Option<
        ni_fpga::Register<types::Encoder6_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder6_TimerOutput: Option<
        ni_fpga::Register<types::Encoder6_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Encoder7_Config: Option<
        ni_fpga::Register<types::Encoder7_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder7_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Encoder7_Output:
        Option<ni_fpga::Register<types::Encoder7_Output, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Encoder7_TimerConfig: Option<
        ni_fpga::Register<types::Encoder7_TimerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Encoder7_TimerOutput: Option<
        ni_fpga::Register<types::Encoder7_TimerOutput, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Interrupt0_Config: Option<
        ni_fpga::Register<types::Interrupt0_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Interrupt0_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt0_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt1_Config: Option<
        ni_fpga::Register<types::Interrupt1_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Interrupt1_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt1_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt2_Config: Option<
        ni_fpga::Register<types::Interrupt2_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Interrupt2_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt2_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt3_Config: Option<
        ni_fpga::Register<types::Interrupt3_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Interrupt3_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt3_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt4_Config: Option<
        ni_fpga::Register<types::Interrupt4_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Interrupt4_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt4_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt5_Config: Option<
        ni_fpga::Register<types::Interrupt5_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Interrupt5_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt5_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt6_Config: Option<
        ni_fpga::Register<types::Interrupt6_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Interrupt6_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt6_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt7_Config: Option<
        ni_fpga::Register<types::Interrupt7_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub Interrupt7_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Interrupt7_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub DMA_Rate: Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub DMA_Config:
        Option<ni_fpga::Register<types::DMA_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub DMA_ExternalTriggers0:
        Option<ni_fpga::Register<[types::Trigger; 4], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub DMA_ExternalTriggers1:
        Option<ni_fpga::Register<[types::Trigger; 4], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Alarm_TriggerTime:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Alarm_Enable: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Relay_Value:
        Option<ni_fpga::Register<types::Relay_Value, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Power_Status:
        Option<ni_fpga::Register<types::Power_Status, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_Disable:
        Option<ni_fpga::Register<types::Power_Disable, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Power_UserVoltage6V:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_UserCurrent6V:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_UserVoltage5V:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_UserCurrent5V:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_UserVoltage3V3:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_UserCurrent3V3:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_VinVoltage: Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_VinCurrent: Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_OnChipTemperature:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_MXP_DIOVoltage:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_IntegratedIO:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_AOVoltage: Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Power_FaultCounts: Option<
        ni_fpga::Register<types::Power_FaultCounts, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub Power_ResetFaultCounts:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Power_BrownoutVoltage250mV:
        Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub BIST_Enable: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub BIST_DO0SquareEnable:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub BIST_DO0SquareTicks:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub BIST_DO0: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub BIST_DO1SquareEnable:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub BIST_DO1SquareTicks:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub BIST_DO1: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AO_MXP0: Option<ni_fpga::Register<u16, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub AO_MXP1: Option<ni_fpga::Register<u16, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SPI_ChipSelectActiveHigh: Option<
        ni_fpga::Register<
            types::SPI_ChipSelectActiveHigh,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub SPI_EnableDIO: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<5, 5, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub SPI_AutoSPI1Select:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SPI_AutoByteCount: Option<
        ni_fpga::Register<types::SPI_AutoByteCount, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub SPI_AutoForceOne:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SPI_AutoRate: Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SPI_AutoTriggerConfig: Option<
        ni_fpga::Register<types::SPI_AutoTriggerConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub SPI_AutoChipSelect:
        Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SPI_AutoTx0: Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SPI_AutoTx1: Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SPI_AutoTx2: Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SPI_AutoTx3: Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SPI_AutoTx4: Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SPI_AutoTx5: Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub SPI_TransferSkippedFullCount:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub SPI_StallConfig: Option<
        ni_fpga::Register<types::SPI_StallConfig, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub SPI_DebugState:
        Option<ni_fpga::Register<types::SPI_DebugState, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub SPI_DebugSubstate: Option<ni_fpga::Register<u8, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub SPI_DebugRevision: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub SPI_DebugEnabled: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub SPI_DebugIntStat: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub SPI_DebugIntStatReadCount:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Accel_ADDR: Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Accel_CNTR: Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Accel_DATO: Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Accel_DATI: Option<ni_fpga::Register<u8, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Accel_CNTL: Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Accel_STAT: Option<ni_fpga::Register<u8, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub Accel_CNFG: Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub Accel_GO: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub HMB_Config:
        Option<ni_fpga::Register<types::HMB_Config, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub HMB_ForceOnce: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub DutyCycle0_Source: Option<
        ni_fpga::Register<types::DutyCycle0_Source, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub DutyCycle0_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle0_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle0_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle1_Source: Option<
        ni_fpga::Register<types::DutyCycle1_Source, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub DutyCycle1_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle1_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle1_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle2_Source: Option<
        ni_fpga::Register<types::DutyCycle2_Source, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub DutyCycle2_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle2_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle2_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle3_Source: Option<
        ni_fpga::Register<types::DutyCycle3_Source, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub DutyCycle3_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle3_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle3_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle4_Source: Option<
        ni_fpga::Register<types::DutyCycle4_Source, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub DutyCycle4_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle4_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle4_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle5_Source: Option<
        ni_fpga::Register<types::DutyCycle5_Source, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub DutyCycle5_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle5_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle5_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle6_Source: Option<
        ni_fpga::Register<types::DutyCycle6_Source, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub DutyCycle6_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle6_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle6_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle7_Source: Option<
        ni_fpga::Register<types::DutyCycle7_Source, ni_fpga::ReadWrite, ni_fpga::StoredOffset>,
    >,
    pub DutyCycle7_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle7_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub DutyCycle7_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::StoredOffset,
        >,
    >,
    pub LED_OutputSelect: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<4, 4, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub LED_StringLength: Option<ni_fpga::Register<u16, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub LED_Load: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub LED_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub LED_Start: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub LED_Abort: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub LED_SyncTiming: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<16, 16, false>,
            ni_fpga::ReadWrite,
            ni_fpga::StoredOffset,
        >,
    >,
    pub LED_HighBitTickTiming:
        Option<ni_fpga::Register<[u8; 2], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub LED_LowBitTickTiming:
        Option<ni_fpga::Register<[u8; 2], ni_fpga::ReadWrite, ni_fpga::StoredOffset>>,
    pub LED_Active: Option<ni_fpga::Register<bool, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub LED_PixelWriteIndex:
        Option<ni_fpga::Register<i16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub LED_PixelOutputIndex:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub HMB_ReadData: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub HMB_WriteCount: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub HMB_WriteAddress: Option<
        ni_fpga::Register<ni_fpga::fxp::FXP<9, 9, false>, ni_fpga::ReadOnly, ni_fpga::StoredOffset>,
    >,
    pub HMB_WriteData: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub HMB_LoopCount: Option<ni_fpga::Register<i32, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub HMB_WriteReadyForInput:
        Option<ni_fpga::Register<bool, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
    pub HMB_ReqReadyForInput:
        Option<ni_fpga::Register<bool, ni_fpga::ReadOnly, ni_fpga::StoredOffset>>,
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
            LocalTime: Some(unsafe { ni_fpga::Register::new(session.find_offset("LocalTime")?) }),
            Revision: Some(unsafe { ni_fpga::Register::new(session.find_offset("Revision")?) }),
            Version: Some(unsafe { ni_fpga::Register::new(session.find_offset("Version")?) }),
            LocalTimeUpper: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("LocalTimeUpper")?)
            }),
            LEDs: Some(unsafe { ni_fpga::Register::new(session.find_offset("LEDs")?) }),
            UserButton: Some(unsafe { ni_fpga::Register::new(session.find_offset("UserButton")?) }),
            InterruptForceOnce: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("InterruptForceOnce")?)
            }),
            InterruptForceNumber: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("InterruptForceNumber")?)
            }),
            SysWatchdog_Status: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SysWatchdog_Status")?)
            }),
            SysWatchdog_Command: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SysWatchdog_Command")?)
            }),
            SysWatchdog_Challenge: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SysWatchdog_Challenge")?)
            }),
            SysWatchdog_Timer: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SysWatchdog_Timer")?)
            }),
            SysWatchdog_Active: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SysWatchdog_Active")?)
            }),
            SysWatchdog_ForcedKills: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SysWatchdog_ForcedKills")?)
            }),
            AI_ReadSelect: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AI_ReadSelect")?)
            }),
            AI_LatchOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AI_LatchOutput")?)
            }),
            AI_Output: Some(unsafe { ni_fpga::Register::new(session.find_offset("AI_Output")?) }),
            AI_Config: Some(unsafe { ni_fpga::Register::new(session.find_offset("AI_Config")?) }),
            AI_ScanList: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AI_ScanList")?)
            }),
            AI_OversampleBits: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AI_OversampleBits")?)
            }),
            AI_AverageBits: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AI_AverageBits")?)
            }),
            AI_LoopTiming: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AI_LoopTiming")?)
            }),
            Accumulator0_Center: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Accumulator0_Center")?)
            }),
            Accumulator0_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Accumulator0_Reset")?)
            }),
            Accumulator0_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Accumulator0_Output")?)
            }),
            Accumulator0_Deadband: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Accumulator0_Deadband")?)
            }),
            Accumulator1_Center: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Accumulator1_Center")?)
            }),
            Accumulator1_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Accumulator1_Reset")?)
            }),
            Accumulator1_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Accumulator1_Output")?)
            }),
            Accumulator1_Deadband: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Accumulator1_Deadband")?)
            }),
            AnalogTrigger_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger_Output")?)
            }),
            AnalogTrigger0_SourceSelect: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger0_SourceSelect")?)
            }),
            AnalogTrigger0_UpperLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger0_UpperLimit")?)
            }),
            AnalogTrigger0_LowerLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger0_LowerLimit")?)
            }),
            AnalogTrigger1_SourceSelect: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger1_SourceSelect")?)
            }),
            AnalogTrigger1_UpperLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger1_UpperLimit")?)
            }),
            AnalogTrigger1_LowerLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger1_LowerLimit")?)
            }),
            AnalogTrigger2_SourceSelect: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger2_SourceSelect")?)
            }),
            AnalogTrigger2_UpperLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger2_UpperLimit")?)
            }),
            AnalogTrigger2_LowerLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger2_LowerLimit")?)
            }),
            AnalogTrigger3_SourceSelect: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger3_SourceSelect")?)
            }),
            AnalogTrigger3_UpperLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger3_UpperLimit")?)
            }),
            AnalogTrigger3_LowerLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger3_LowerLimit")?)
            }),
            AnalogTrigger4_SourceSelect: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger4_SourceSelect")?)
            }),
            AnalogTrigger4_UpperLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger4_UpperLimit")?)
            }),
            AnalogTrigger4_LowerLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger4_LowerLimit")?)
            }),
            AnalogTrigger5_SourceSelect: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger5_SourceSelect")?)
            }),
            AnalogTrigger6_UpperLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger6_UpperLimit")?)
            }),
            AnalogTrigger6_LowerLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger6_LowerLimit")?)
            }),
            AnalogTrigger6_SourceSelect: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger6_SourceSelect")?)
            }),
            AnalogTrigger5_UpperLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger5_UpperLimit")?)
            }),
            AnalogTrigger5_LowerLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger5_LowerLimit")?)
            }),
            AnalogTrigger7_SourceSelect: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger7_SourceSelect")?)
            }),
            AnalogTrigger7_UpperLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger7_UpperLimit")?)
            }),
            AnalogTrigger7_LowerLimit: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("AnalogTrigger7_LowerLimit")?)
            }),
            PWM_LoopTiming: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("PWM_LoopTiming")?)
            }),
            PWM_CycleStartTimeUpper: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("PWM_CycleStartTimeUpper")?)
            }),
            PWM_CycleStartTime: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("PWM_CycleStartTime")?)
            }),
            PWM_Config: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_Config")?) }),
            PWM_PeriodScaleHdr: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("PWM_PeriodScaleHdr")?)
            }),
            PWM_PeriodScaleMXP: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("PWM_PeriodScaleMXP")?)
            }),
            PWM_ZeroLatch: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("PWM_ZeroLatch")?)
            }),
            PWM_Hdr0: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_Hdr0")?) }),
            PWM_Hdr1: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_Hdr1")?) }),
            PWM_Hdr2: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_Hdr2")?) }),
            PWM_Hdr3: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_Hdr3")?) }),
            PWM_Hdr4: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_Hdr4")?) }),
            PWM_Hdr5: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_Hdr5")?) }),
            PWM_Hdr6: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_Hdr6")?) }),
            PWM_Hdr7: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_Hdr7")?) }),
            PWM_Hdr8: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_Hdr8")?) }),
            PWM_Hdr9: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_Hdr9")?) }),
            PWM_MXP0: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_MXP0")?) }),
            PWM_MXP1: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_MXP1")?) }),
            PWM_MXP2: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_MXP2")?) }),
            PWM_MXP3: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_MXP3")?) }),
            PWM_MXP4: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_MXP4")?) }),
            PWM_MXP5: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_MXP5")?) }),
            PWM_MXP6: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_MXP6")?) }),
            PWM_MXP7: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_MXP7")?) }),
            PWM_MXP8: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_MXP8")?) }),
            PWM_MXP9: Some(unsafe { ni_fpga::Register::new(session.find_offset("PWM_MXP9")?) }),
            DIO_OutputEnable: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_OutputEnable")?)
            }),
            DIO_DO: Some(unsafe { ni_fpga::Register::new(session.find_offset("DIO_DO")?) }),
            DIO_DI: Some(unsafe { ni_fpga::Register::new(session.find_offset("DIO_DI")?) }),
            DIO_FilterSelectHdr: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_FilterSelectHdr")?)
            }),
            DIO_FilterPeriodHdr0: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_FilterPeriodHdr0")?)
            }),
            DIO_FilterPeriodHdr1: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_FilterPeriodHdr1")?)
            }),
            DIO_FilterPeriodHdr2: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_FilterPeriodHdr2")?)
            }),
            DIO_FilterSelectMXP: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_FilterSelectMXP")?)
            }),
            DIO_FilterPeriodMXP0: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_FilterPeriodMXP0")?)
            }),
            DIO_FilterPeriodMXP1: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_FilterPeriodMXP1")?)
            }),
            DIO_FilterPeriodMXP2: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_FilterPeriodMXP2")?)
            }),
            DIO_EnableMXPSpecialFunction: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_EnableMXPSpecialFunction")?)
            }),
            DIO_PulseLength: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_PulseLength")?)
            }),
            DIO_Pulse: Some(unsafe { ni_fpga::Register::new(session.find_offset("DIO_Pulse")?) }),
            DIO_PWMDutyCycleA: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_PWMDutyCycleA")?)
            }),
            DIO_PWMDutyCycleB: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_PWMDutyCycleB")?)
            }),
            DIO_PWMOutputSelect: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_PWMOutputSelect")?)
            }),
            DIO_PWMPeriodPower: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DIO_PWMPeriodPower")?)
            }),
            Counter0_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter0_Config")?)
            }),
            Counter0_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter0_Reset")?)
            }),
            Counter0_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter0_Output")?)
            }),
            Counter0_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter0_TimerConfig")?)
            }),
            Counter0_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter0_TimerOutput")?)
            }),
            Counter1_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter1_Config")?)
            }),
            Counter1_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter1_Reset")?)
            }),
            Counter1_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter1_Output")?)
            }),
            Counter1_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter1_TimerConfig")?)
            }),
            Counter1_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter1_TimerOutput")?)
            }),
            Counter2_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter2_Config")?)
            }),
            Counter2_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter2_Reset")?)
            }),
            Counter2_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter2_Output")?)
            }),
            Counter2_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter2_TimerConfig")?)
            }),
            Counter2_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter2_TimerOutput")?)
            }),
            Counter3_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter3_Config")?)
            }),
            Counter3_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter3_Reset")?)
            }),
            Counter3_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter3_Output")?)
            }),
            Counter3_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter3_TimerConfig")?)
            }),
            Counter3_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter3_TimerOutput")?)
            }),
            Counter4_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter4_Config")?)
            }),
            Counter4_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter4_Reset")?)
            }),
            Counter4_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter4_Output")?)
            }),
            Counter4_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter4_TimerConfig")?)
            }),
            Counter4_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter4_TimerOutput")?)
            }),
            Counter5_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter5_Config")?)
            }),
            Counter5_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter5_Reset")?)
            }),
            Counter5_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter5_Output")?)
            }),
            Counter5_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter5_TimerConfig")?)
            }),
            Counter5_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter5_TimerOutput")?)
            }),
            Counter6_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter6_Config")?)
            }),
            Counter6_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter6_Reset")?)
            }),
            Counter6_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter6_Output")?)
            }),
            Counter6_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter6_TimerConfig")?)
            }),
            Counter6_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter6_TimerOutput")?)
            }),
            Counter7_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter7_Config")?)
            }),
            Counter7_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter7_Reset")?)
            }),
            Counter7_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter7_Output")?)
            }),
            Counter7_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter7_TimerConfig")?)
            }),
            Counter7_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Counter7_TimerOutput")?)
            }),
            Encoder0_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder0_Config")?)
            }),
            Encoder0_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder0_Reset")?)
            }),
            Encoder0_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder0_Output")?)
            }),
            Encoder0_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder0_TimerConfig")?)
            }),
            Encoder0_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder0_TimerOutput")?)
            }),
            Encoder1_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder1_Config")?)
            }),
            Encoder1_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder1_Reset")?)
            }),
            Encoder1_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder1_Output")?)
            }),
            Encoder1_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder1_TimerConfig")?)
            }),
            Encoder1_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder1_TimerOutput")?)
            }),
            Encoder2_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder2_Config")?)
            }),
            Encoder2_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder2_Reset")?)
            }),
            Encoder2_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder2_Output")?)
            }),
            Encoder2_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder2_TimerConfig")?)
            }),
            Encoder2_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder2_TimerOutput")?)
            }),
            Encoder3_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder3_Config")?)
            }),
            Encoder3_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder3_Reset")?)
            }),
            Encoder3_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder3_Output")?)
            }),
            Encoder3_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder3_TimerConfig")?)
            }),
            Encoder3_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder3_TimerOutput")?)
            }),
            Encoder4_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder4_Config")?)
            }),
            Encoder4_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder4_Reset")?)
            }),
            Encoder4_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder4_Output")?)
            }),
            Encoder4_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder4_TimerConfig")?)
            }),
            Encoder4_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder4_TimerOutput")?)
            }),
            Encoder5_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder5_Config")?)
            }),
            Encoder5_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder5_Reset")?)
            }),
            Encoder5_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder5_Output")?)
            }),
            Encoder5_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder5_TimerConfig")?)
            }),
            Encoder5_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder5_TimerOutput")?)
            }),
            Encoder6_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder6_Config")?)
            }),
            Encoder6_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder6_Reset")?)
            }),
            Encoder6_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder6_Output")?)
            }),
            Encoder6_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder6_TimerConfig")?)
            }),
            Encoder6_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder6_TimerOutput")?)
            }),
            Encoder7_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder7_Config")?)
            }),
            Encoder7_Reset: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder7_Reset")?)
            }),
            Encoder7_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder7_Output")?)
            }),
            Encoder7_TimerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder7_TimerConfig")?)
            }),
            Encoder7_TimerOutput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Encoder7_TimerOutput")?)
            }),
            Interrupt0_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt0_Config")?)
            }),
            Interrupt0_RisingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt0_RisingTimeStamp")?)
            }),
            Interrupt0_FallingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt0_FallingTimeStamp")?)
            }),
            Interrupt1_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt1_Config")?)
            }),
            Interrupt1_RisingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt1_RisingTimeStamp")?)
            }),
            Interrupt1_FallingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt1_FallingTimeStamp")?)
            }),
            Interrupt2_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt2_Config")?)
            }),
            Interrupt2_RisingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt2_RisingTimeStamp")?)
            }),
            Interrupt2_FallingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt2_FallingTimeStamp")?)
            }),
            Interrupt3_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt3_Config")?)
            }),
            Interrupt3_RisingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt3_RisingTimeStamp")?)
            }),
            Interrupt3_FallingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt3_FallingTimeStamp")?)
            }),
            Interrupt4_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt4_Config")?)
            }),
            Interrupt4_RisingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt4_RisingTimeStamp")?)
            }),
            Interrupt4_FallingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt4_FallingTimeStamp")?)
            }),
            Interrupt5_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt5_Config")?)
            }),
            Interrupt5_RisingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt5_RisingTimeStamp")?)
            }),
            Interrupt5_FallingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt5_FallingTimeStamp")?)
            }),
            Interrupt6_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt6_Config")?)
            }),
            Interrupt6_RisingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt6_RisingTimeStamp")?)
            }),
            Interrupt6_FallingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt6_FallingTimeStamp")?)
            }),
            Interrupt7_Config: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt7_Config")?)
            }),
            Interrupt7_RisingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt7_RisingTimeStamp")?)
            }),
            Interrupt7_FallingTimeStamp: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Interrupt7_FallingTimeStamp")?)
            }),
            DMA_Rate: Some(unsafe { ni_fpga::Register::new(session.find_offset("DMA_Rate")?) }),
            DMA_Config: Some(unsafe { ni_fpga::Register::new(session.find_offset("DMA_Config")?) }),
            DMA_ExternalTriggers0: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DMA_ExternalTriggers0")?)
            }),
            DMA_ExternalTriggers1: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DMA_ExternalTriggers1")?)
            }),
            Alarm_TriggerTime: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Alarm_TriggerTime")?)
            }),
            Alarm_Enable: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Alarm_Enable")?)
            }),
            Relay_Value: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Relay_Value")?)
            }),
            Power_Status: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_Status")?)
            }),
            Power_Disable: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_Disable")?)
            }),
            Power_UserVoltage6V: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_UserVoltage6V")?)
            }),
            Power_UserCurrent6V: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_UserCurrent6V")?)
            }),
            Power_UserVoltage5V: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_UserVoltage5V")?)
            }),
            Power_UserCurrent5V: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_UserCurrent5V")?)
            }),
            Power_UserVoltage3V3: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_UserVoltage3V3")?)
            }),
            Power_UserCurrent3V3: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_UserCurrent3V3")?)
            }),
            Power_VinVoltage: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_VinVoltage")?)
            }),
            Power_VinCurrent: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_VinCurrent")?)
            }),
            Power_OnChipTemperature: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_OnChipTemperature")?)
            }),
            Power_MXP_DIOVoltage: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_MXP_DIOVoltage")?)
            }),
            Power_IntegratedIO: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_IntegratedIO")?)
            }),
            Power_AOVoltage: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_AOVoltage")?)
            }),
            Power_FaultCounts: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_FaultCounts")?)
            }),
            Power_ResetFaultCounts: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_ResetFaultCounts")?)
            }),
            Power_BrownoutVoltage250mV: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("Power_BrownoutVoltage250mV")?)
            }),
            BIST_Enable: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("BIST_Enable")?)
            }),
            BIST_DO0SquareEnable: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("BIST_DO0SquareEnable")?)
            }),
            BIST_DO0SquareTicks: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("BIST_DO0SquareTicks")?)
            }),
            BIST_DO0: Some(unsafe { ni_fpga::Register::new(session.find_offset("BIST_DO0")?) }),
            BIST_DO1SquareEnable: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("BIST_DO1SquareEnable")?)
            }),
            BIST_DO1SquareTicks: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("BIST_DO1SquareTicks")?)
            }),
            BIST_DO1: Some(unsafe { ni_fpga::Register::new(session.find_offset("BIST_DO1")?) }),
            AO_MXP0: Some(unsafe { ni_fpga::Register::new(session.find_offset("AO_MXP0")?) }),
            AO_MXP1: Some(unsafe { ni_fpga::Register::new(session.find_offset("AO_MXP1")?) }),
            SPI_ChipSelectActiveHigh: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_ChipSelectActiveHigh")?)
            }),
            SPI_EnableDIO: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_EnableDIO")?)
            }),
            SPI_AutoSPI1Select: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_AutoSPI1Select")?)
            }),
            SPI_AutoByteCount: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_AutoByteCount")?)
            }),
            SPI_AutoForceOne: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_AutoForceOne")?)
            }),
            SPI_AutoRate: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_AutoRate")?)
            }),
            SPI_AutoTriggerConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_AutoTriggerConfig")?)
            }),
            SPI_AutoChipSelect: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_AutoChipSelect")?)
            }),
            SPI_AutoTx0: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_AutoTx0")?)
            }),
            SPI_AutoTx1: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_AutoTx1")?)
            }),
            SPI_AutoTx2: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_AutoTx2")?)
            }),
            SPI_AutoTx3: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_AutoTx3")?)
            }),
            SPI_AutoTx4: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_AutoTx4")?)
            }),
            SPI_AutoTx5: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_AutoTx5")?)
            }),
            SPI_TransferSkippedFullCount: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_TransferSkippedFullCount")?)
            }),
            SPI_StallConfig: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_StallConfig")?)
            }),
            SPI_DebugState: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_DebugState")?)
            }),
            SPI_DebugSubstate: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_DebugSubstate")?)
            }),
            SPI_DebugRevision: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_DebugRevision")?)
            }),
            SPI_DebugEnabled: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_DebugEnabled")?)
            }),
            SPI_DebugIntStat: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_DebugIntStat")?)
            }),
            SPI_DebugIntStatReadCount: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("SPI_DebugIntStatReadCount")?)
            }),
            Accel_ADDR: Some(unsafe { ni_fpga::Register::new(session.find_offset("Accel_ADDR")?) }),
            Accel_CNTR: Some(unsafe { ni_fpga::Register::new(session.find_offset("Accel_CNTR")?) }),
            Accel_DATO: Some(unsafe { ni_fpga::Register::new(session.find_offset("Accel_DATO")?) }),
            Accel_DATI: Some(unsafe { ni_fpga::Register::new(session.find_offset("Accel_DATI")?) }),
            Accel_CNTL: Some(unsafe { ni_fpga::Register::new(session.find_offset("Accel_CNTL")?) }),
            Accel_STAT: Some(unsafe { ni_fpga::Register::new(session.find_offset("Accel_STAT")?) }),
            Accel_CNFG: Some(unsafe { ni_fpga::Register::new(session.find_offset("Accel_CNFG")?) }),
            Accel_GO: Some(unsafe { ni_fpga::Register::new(session.find_offset("Accel_GO")?) }),
            HMB_Config: Some(unsafe { ni_fpga::Register::new(session.find_offset("HMB_Config")?) }),
            HMB_ForceOnce: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("HMB_ForceOnce")?)
            }),
            DutyCycle0_Source: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle0_Source")?)
            }),
            DutyCycle0_Frequency: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle0_Frequency")?)
            }),
            DutyCycle0_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle0_Output")?)
            }),
            DutyCycle0_HighTicks: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle0_HighTicks")?)
            }),
            DutyCycle1_Source: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle1_Source")?)
            }),
            DutyCycle1_Frequency: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle1_Frequency")?)
            }),
            DutyCycle1_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle1_Output")?)
            }),
            DutyCycle1_HighTicks: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle1_HighTicks")?)
            }),
            DutyCycle2_Source: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle2_Source")?)
            }),
            DutyCycle2_Frequency: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle2_Frequency")?)
            }),
            DutyCycle2_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle2_Output")?)
            }),
            DutyCycle2_HighTicks: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle2_HighTicks")?)
            }),
            DutyCycle3_Source: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle3_Source")?)
            }),
            DutyCycle3_Frequency: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle3_Frequency")?)
            }),
            DutyCycle3_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle3_Output")?)
            }),
            DutyCycle3_HighTicks: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle3_HighTicks")?)
            }),
            DutyCycle4_Source: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle4_Source")?)
            }),
            DutyCycle4_Frequency: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle4_Frequency")?)
            }),
            DutyCycle4_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle4_Output")?)
            }),
            DutyCycle4_HighTicks: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle4_HighTicks")?)
            }),
            DutyCycle5_Source: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle5_Source")?)
            }),
            DutyCycle5_Frequency: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle5_Frequency")?)
            }),
            DutyCycle5_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle5_Output")?)
            }),
            DutyCycle5_HighTicks: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle5_HighTicks")?)
            }),
            DutyCycle6_Source: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle6_Source")?)
            }),
            DutyCycle6_Frequency: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle6_Frequency")?)
            }),
            DutyCycle6_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle6_Output")?)
            }),
            DutyCycle6_HighTicks: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle6_HighTicks")?)
            }),
            DutyCycle7_Source: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle7_Source")?)
            }),
            DutyCycle7_Frequency: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle7_Frequency")?)
            }),
            DutyCycle7_Output: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle7_Output")?)
            }),
            DutyCycle7_HighTicks: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("DutyCycle7_HighTicks")?)
            }),
            LED_OutputSelect: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("LED_OutputSelect")?)
            }),
            LED_StringLength: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("LED_StringLength")?)
            }),
            LED_Load: Some(unsafe { ni_fpga::Register::new(session.find_offset("LED_Load")?) }),
            LED_Reset: Some(unsafe { ni_fpga::Register::new(session.find_offset("LED_Reset")?) }),
            LED_Start: Some(unsafe { ni_fpga::Register::new(session.find_offset("LED_Start")?) }),
            LED_Abort: Some(unsafe { ni_fpga::Register::new(session.find_offset("LED_Abort")?) }),
            LED_SyncTiming: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("LED_SyncTiming")?)
            }),
            LED_HighBitTickTiming: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("LED_HighBitTickTiming")?)
            }),
            LED_LowBitTickTiming: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("LED_LowBitTickTiming")?)
            }),
            LED_Active: Some(unsafe { ni_fpga::Register::new(session.find_offset("LED_Active")?) }),
            LED_PixelWriteIndex: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("LED_PixelWriteIndex")?)
            }),
            LED_PixelOutputIndex: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("LED_PixelOutputIndex")?)
            }),
            HMB_ReadData: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("HMB_ReadData")?)
            }),
            HMB_WriteCount: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("HMB_WriteCount")?)
            }),
            HMB_WriteAddress: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("HMB_WriteAddress")?)
            }),
            HMB_WriteData: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("HMB_WriteData")?)
            }),
            HMB_LoopCount: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("HMB_LoopCount")?)
            }),
            HMB_WriteReadyForInput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("HMB_WriteReadyForInput")?)
            }),
            HMB_ReqReadyForInput: Some(unsafe {
                ni_fpga::Register::new(session.find_offset("HMB_ReqReadyForInput")?)
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
        include_str!("roboRIO_FPGA_2023_23.0.0.lvbitx")
    }
    pub const fn signature() -> &'static str {
        "2A5927EB7178780081872E6823E32FFC"
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
