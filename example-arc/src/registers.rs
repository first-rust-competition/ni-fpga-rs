use std::marker::PhantomData;
mod types {
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
    pub LocalTime: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98304>>>,
    pub Revision: Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98308>>>,
    pub Version: Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98314>>>,
    pub LocalTimeUpper:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98316>>>,
    pub LEDs:
        Option<ni_fpga::Register<types::LEDs, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98320>>>,
    pub UserButton: Option<ni_fpga::Register<bool, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98326>>>,
    pub InterruptForceOnce:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98330>>>,
    pub InterruptForceNumber:
        Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98334>>>,
    pub SysWatchdog_Status: Option<
        ni_fpga::Register<
            types::SysWatchdog_Status,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98336>,
        >,
    >,
    pub SysWatchdog_Command:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98340>>>,
    pub SysWatchdog_Challenge:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98344>>>,
    pub SysWatchdog_Timer:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98348>>>,
    pub SysWatchdog_Active:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98354>>>,
    pub SysWatchdog_ForcedKills: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<15, 15, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98358>,
        >,
    >,
    pub AI_ReadSelect: Option<
        ni_fpga::Register<types::AI_ReadSelect, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98362>>,
    >,
    pub AI_LatchOutput:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98366>>>,
    pub AI_Output: Option<ni_fpga::Register<i32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98368>>>,
    pub AI_Config: Option<
        ni_fpga::Register<types::AI_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98372>>,
    >,
    pub AI_ScanList: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<3, 3, false>; 8],
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98376>,
        >,
    >,
    pub AI_OversampleBits: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<4, 4, false>; 8],
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98380>,
        >,
    >,
    pub AI_AverageBits: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<4, 4, false>; 8],
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98384>,
        >,
    >,
    pub AI_LoopTiming:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98388>>>,
    pub Accumulator0_Center:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98392>>>,
    pub Accumulator0_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98398>>>,
    pub Accumulator0_Output: Option<
        ni_fpga::Register<
            types::Accumulator0_Output,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98400>,
        >,
    >,
    pub Accumulator0_Deadband:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98404>>>,
    pub Accumulator1_Center:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98408>>>,
    pub Accumulator1_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98414>>>,
    pub Accumulator1_Output: Option<
        ni_fpga::Register<
            types::Accumulator1_Output,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98416>,
        >,
    >,
    pub Accumulator1_Deadband:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98420>>>,
    pub AnalogTrigger_Output: Option<
        ni_fpga::Register<
            [types::AnalogTrigger_Output; 8],
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98424>,
        >,
    >,
    pub AnalogTrigger0_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger0_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98430>,
        >,
    >,
    pub AnalogTrigger0_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98432>>>,
    pub AnalogTrigger0_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98436>>>,
    pub AnalogTrigger1_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger1_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98442>,
        >,
    >,
    pub AnalogTrigger1_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98444>>>,
    pub AnalogTrigger1_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98448>>>,
    pub AnalogTrigger2_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger2_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98454>,
        >,
    >,
    pub AnalogTrigger2_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98456>>>,
    pub AnalogTrigger2_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98460>>>,
    pub AnalogTrigger3_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger3_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98466>,
        >,
    >,
    pub AnalogTrigger3_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98468>>>,
    pub AnalogTrigger3_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98472>>>,
    pub AnalogTrigger4_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger4_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98478>,
        >,
    >,
    pub AnalogTrigger4_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98480>>>,
    pub AnalogTrigger4_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98484>>>,
    pub AnalogTrigger5_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger5_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98490>,
        >,
    >,
    pub AnalogTrigger6_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98492>>>,
    pub AnalogTrigger6_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98496>>>,
    pub AnalogTrigger6_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger6_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98502>,
        >,
    >,
    pub AnalogTrigger5_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98504>>>,
    pub AnalogTrigger5_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98508>>>,
    pub AnalogTrigger7_SourceSelect: Option<
        ni_fpga::Register<
            types::AnalogTrigger7_SourceSelect,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98514>,
        >,
    >,
    pub AnalogTrigger7_UpperLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98516>>>,
    pub AnalogTrigger7_LowerLimit:
        Option<ni_fpga::Register<i32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98520>>>,
    pub PWM_LoopTiming:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98526>>>,
    pub PWM_CycleStartTimeUpper:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98528>>>,
    pub PWM_CycleStartTime:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98532>>>,
    pub PWM_Config: Option<
        ni_fpga::Register<types::PWM_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98536>>,
    >,
    pub PWM_PeriodScaleHdr: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<2, 2, false>; 10],
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98540>,
        >,
    >,
    pub PWM_PeriodScaleMXP: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<2, 2, false>; 10],
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98544>,
        >,
    >,
    pub PWM_ZeroLatch:
        Option<ni_fpga::Register<[bool; 20], ni_fpga::ReadWrite, ni_fpga::ConstOffset<98548>>>,
    pub PWM_Hdr0: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98554>,
        >,
    >,
    pub PWM_Hdr1: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98558>,
        >,
    >,
    pub PWM_Hdr2: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98562>,
        >,
    >,
    pub PWM_Hdr3: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98566>,
        >,
    >,
    pub PWM_Hdr4: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98570>,
        >,
    >,
    pub PWM_Hdr5: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98574>,
        >,
    >,
    pub PWM_Hdr6: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98578>,
        >,
    >,
    pub PWM_Hdr7: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98582>,
        >,
    >,
    pub PWM_Hdr8: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98586>,
        >,
    >,
    pub PWM_Hdr9: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98590>,
        >,
    >,
    pub PWM_MXP0: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98594>,
        >,
    >,
    pub PWM_MXP1: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98598>,
        >,
    >,
    pub PWM_MXP2: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98602>,
        >,
    >,
    pub PWM_MXP3: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98606>,
        >,
    >,
    pub PWM_MXP4: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98610>,
        >,
    >,
    pub PWM_MXP5: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98614>,
        >,
    >,
    pub PWM_MXP6: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98618>,
        >,
    >,
    pub PWM_MXP7: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98622>,
        >,
    >,
    pub PWM_MXP8: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98626>,
        >,
    >,
    pub PWM_MXP9: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<12, 12, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98630>,
        >,
    >,
    pub DIO_OutputEnable: Option<
        ni_fpga::Register<types::DIO_OutputEnable, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98632>>,
    >,
    pub DIO_DO:
        Option<ni_fpga::Register<types::DIO_DO, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98636>>>,
    pub DIO_DI:
        Option<ni_fpga::Register<types::DIO_DI, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98640>>>,
    pub DIO_FilterSelectHdr: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<2, 2, false>; 16],
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98644>,
        >,
    >,
    pub DIO_FilterPeriodHdr0: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<24, 24, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98648>,
        >,
    >,
    pub DIO_FilterPeriodHdr1: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<24, 24, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98652>,
        >,
    >,
    pub DIO_FilterPeriodHdr2: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<24, 24, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98656>,
        >,
    >,
    pub DIO_FilterSelectMXP: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<2, 2, false>; 16],
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98660>,
        >,
    >,
    pub DIO_FilterPeriodMXP0: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<24, 24, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98664>,
        >,
    >,
    pub DIO_FilterPeriodMXP1: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<24, 24, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98668>,
        >,
    >,
    pub DIO_FilterPeriodMXP2: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<24, 24, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98672>,
        >,
    >,
    pub DIO_EnableMXPSpecialFunction:
        Option<ni_fpga::Register<u16, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98678>>>,
    pub DIO_PulseLength:
        Option<ni_fpga::Register<u16, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98682>>>,
    pub DIO_Pulse: Option<
        ni_fpga::Register<types::DIO_Pulse, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98684>>,
    >,
    pub DIO_PWMDutyCycleA:
        Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::ConstOffset<98688>>>,
    pub DIO_PWMDutyCycleB:
        Option<ni_fpga::Register<[u8; 2], ni_fpga::ReadWrite, ni_fpga::ConstOffset<98694>>>,
    pub DIO_PWMOutputSelect: Option<
        ni_fpga::Register<
            [ni_fpga::fxp::FXP<5, 5, false>; 6],
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98696>,
        >,
    >,
    pub DIO_PWMPeriodPower:
        Option<ni_fpga::Register<u16, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98702>>>,
    pub Counter0_Config: Option<
        ni_fpga::Register<types::Counter0_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98704>>,
    >,
    pub Counter0_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98710>>>,
    pub Counter0_Output: Option<
        ni_fpga::Register<types::Counter0_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98712>>,
    >,
    pub Counter0_TimerConfig: Option<
        ni_fpga::Register<
            types::Counter0_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98716>,
        >,
    >,
    pub Counter0_TimerOutput: Option<
        ni_fpga::Register<
            types::Counter0_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98720>,
        >,
    >,
    pub Counter1_Config: Option<
        ni_fpga::Register<types::Counter1_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98724>>,
    >,
    pub Counter1_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98730>>>,
    pub Counter1_Output: Option<
        ni_fpga::Register<types::Counter1_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98732>>,
    >,
    pub Counter1_TimerConfig: Option<
        ni_fpga::Register<
            types::Counter1_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98736>,
        >,
    >,
    pub Counter1_TimerOutput: Option<
        ni_fpga::Register<
            types::Counter1_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98740>,
        >,
    >,
    pub Counter2_Config: Option<
        ni_fpga::Register<types::Counter2_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98744>>,
    >,
    pub Counter2_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98750>>>,
    pub Counter2_Output: Option<
        ni_fpga::Register<types::Counter2_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98752>>,
    >,
    pub Counter2_TimerConfig: Option<
        ni_fpga::Register<
            types::Counter2_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98756>,
        >,
    >,
    pub Counter2_TimerOutput: Option<
        ni_fpga::Register<
            types::Counter2_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98760>,
        >,
    >,
    pub Counter3_Config: Option<
        ni_fpga::Register<types::Counter3_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98764>>,
    >,
    pub Counter3_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98770>>>,
    pub Counter3_Output: Option<
        ni_fpga::Register<types::Counter3_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98772>>,
    >,
    pub Counter3_TimerConfig: Option<
        ni_fpga::Register<
            types::Counter3_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98776>,
        >,
    >,
    pub Counter3_TimerOutput: Option<
        ni_fpga::Register<
            types::Counter3_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98780>,
        >,
    >,
    pub Counter4_Config: Option<
        ni_fpga::Register<types::Counter4_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98784>>,
    >,
    pub Counter4_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98790>>>,
    pub Counter4_Output: Option<
        ni_fpga::Register<types::Counter4_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98792>>,
    >,
    pub Counter4_TimerConfig: Option<
        ni_fpga::Register<
            types::Counter4_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98796>,
        >,
    >,
    pub Counter4_TimerOutput: Option<
        ni_fpga::Register<
            types::Counter4_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98800>,
        >,
    >,
    pub Counter5_Config: Option<
        ni_fpga::Register<types::Counter5_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98804>>,
    >,
    pub Counter5_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98810>>>,
    pub Counter5_Output: Option<
        ni_fpga::Register<types::Counter5_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98812>>,
    >,
    pub Counter5_TimerConfig: Option<
        ni_fpga::Register<
            types::Counter5_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98816>,
        >,
    >,
    pub Counter5_TimerOutput: Option<
        ni_fpga::Register<
            types::Counter5_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98820>,
        >,
    >,
    pub Counter6_Config: Option<
        ni_fpga::Register<types::Counter6_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98824>>,
    >,
    pub Counter6_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98830>>>,
    pub Counter6_Output: Option<
        ni_fpga::Register<types::Counter6_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98832>>,
    >,
    pub Counter6_TimerConfig: Option<
        ni_fpga::Register<
            types::Counter6_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98836>,
        >,
    >,
    pub Counter6_TimerOutput: Option<
        ni_fpga::Register<
            types::Counter6_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98840>,
        >,
    >,
    pub Counter7_Config: Option<
        ni_fpga::Register<types::Counter7_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98844>>,
    >,
    pub Counter7_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98850>>>,
    pub Counter7_Output: Option<
        ni_fpga::Register<types::Counter7_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98852>>,
    >,
    pub Counter7_TimerConfig: Option<
        ni_fpga::Register<
            types::Counter7_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98856>,
        >,
    >,
    pub Counter7_TimerOutput: Option<
        ni_fpga::Register<
            types::Counter7_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98860>,
        >,
    >,
    pub Encoder0_Config: Option<
        ni_fpga::Register<types::Encoder0_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98864>>,
    >,
    pub Encoder0_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98870>>>,
    pub Encoder0_Output: Option<
        ni_fpga::Register<types::Encoder0_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98872>>,
    >,
    pub Encoder0_TimerConfig: Option<
        ni_fpga::Register<
            types::Encoder0_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98876>,
        >,
    >,
    pub Encoder0_TimerOutput: Option<
        ni_fpga::Register<
            types::Encoder0_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98880>,
        >,
    >,
    pub Encoder1_Config: Option<
        ni_fpga::Register<types::Encoder1_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98884>>,
    >,
    pub Encoder1_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98890>>>,
    pub Encoder1_Output: Option<
        ni_fpga::Register<types::Encoder1_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98892>>,
    >,
    pub Encoder1_TimerConfig: Option<
        ni_fpga::Register<
            types::Encoder1_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98896>,
        >,
    >,
    pub Encoder1_TimerOutput: Option<
        ni_fpga::Register<
            types::Encoder1_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98900>,
        >,
    >,
    pub Encoder2_Config: Option<
        ni_fpga::Register<types::Encoder2_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98904>>,
    >,
    pub Encoder2_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98910>>>,
    pub Encoder2_Output: Option<
        ni_fpga::Register<types::Encoder2_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98912>>,
    >,
    pub Encoder2_TimerConfig: Option<
        ni_fpga::Register<
            types::Encoder2_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98916>,
        >,
    >,
    pub Encoder2_TimerOutput: Option<
        ni_fpga::Register<
            types::Encoder2_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98920>,
        >,
    >,
    pub Encoder3_Config: Option<
        ni_fpga::Register<types::Encoder3_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98924>>,
    >,
    pub Encoder3_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98930>>>,
    pub Encoder3_Output: Option<
        ni_fpga::Register<types::Encoder3_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98932>>,
    >,
    pub Encoder3_TimerConfig: Option<
        ni_fpga::Register<
            types::Encoder3_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98936>,
        >,
    >,
    pub Encoder3_TimerOutput: Option<
        ni_fpga::Register<
            types::Encoder3_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98940>,
        >,
    >,
    pub Encoder4_Config: Option<
        ni_fpga::Register<types::Encoder4_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98944>>,
    >,
    pub Encoder4_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98950>>>,
    pub Encoder4_Output: Option<
        ni_fpga::Register<types::Encoder4_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98952>>,
    >,
    pub Encoder4_TimerConfig: Option<
        ni_fpga::Register<
            types::Encoder4_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98956>,
        >,
    >,
    pub Encoder4_TimerOutput: Option<
        ni_fpga::Register<
            types::Encoder4_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98960>,
        >,
    >,
    pub Encoder5_Config: Option<
        ni_fpga::Register<types::Encoder5_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98964>>,
    >,
    pub Encoder5_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98970>>>,
    pub Encoder5_Output: Option<
        ni_fpga::Register<types::Encoder5_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98972>>,
    >,
    pub Encoder5_TimerConfig: Option<
        ni_fpga::Register<
            types::Encoder5_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98976>,
        >,
    >,
    pub Encoder5_TimerOutput: Option<
        ni_fpga::Register<
            types::Encoder5_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<98980>,
        >,
    >,
    pub Encoder6_Config: Option<
        ni_fpga::Register<types::Encoder6_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98984>>,
    >,
    pub Encoder6_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<98990>>>,
    pub Encoder6_Output: Option<
        ni_fpga::Register<types::Encoder6_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<98992>>,
    >,
    pub Encoder6_TimerConfig: Option<
        ni_fpga::Register<
            types::Encoder6_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<98996>,
        >,
    >,
    pub Encoder6_TimerOutput: Option<
        ni_fpga::Register<
            types::Encoder6_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99000>,
        >,
    >,
    pub Encoder7_Config: Option<
        ni_fpga::Register<types::Encoder7_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99004>>,
    >,
    pub Encoder7_Reset:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99010>>>,
    pub Encoder7_Output: Option<
        ni_fpga::Register<types::Encoder7_Output, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99012>>,
    >,
    pub Encoder7_TimerConfig: Option<
        ni_fpga::Register<
            types::Encoder7_TimerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99016>,
        >,
    >,
    pub Encoder7_TimerOutput: Option<
        ni_fpga::Register<
            types::Encoder7_TimerOutput,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99020>,
        >,
    >,
    pub Interrupt0_Config: Option<
        ni_fpga::Register<
            types::Interrupt0_Config,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99026>,
        >,
    >,
    pub Interrupt0_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99028>>>,
    pub Interrupt0_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99032>>>,
    pub Interrupt1_Config: Option<
        ni_fpga::Register<
            types::Interrupt1_Config,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99038>,
        >,
    >,
    pub Interrupt1_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99040>>>,
    pub Interrupt1_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99044>>>,
    pub Interrupt2_Config: Option<
        ni_fpga::Register<
            types::Interrupt2_Config,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99050>,
        >,
    >,
    pub Interrupt2_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99052>>>,
    pub Interrupt2_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99056>>>,
    pub Interrupt3_Config: Option<
        ni_fpga::Register<
            types::Interrupt3_Config,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99062>,
        >,
    >,
    pub Interrupt3_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99064>>>,
    pub Interrupt3_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99068>>>,
    pub Interrupt4_Config: Option<
        ni_fpga::Register<
            types::Interrupt4_Config,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99074>,
        >,
    >,
    pub Interrupt4_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99076>>>,
    pub Interrupt4_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99080>>>,
    pub Interrupt5_Config: Option<
        ni_fpga::Register<
            types::Interrupt5_Config,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99086>,
        >,
    >,
    pub Interrupt5_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99088>>>,
    pub Interrupt5_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99092>>>,
    pub Interrupt6_Config: Option<
        ni_fpga::Register<
            types::Interrupt6_Config,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99098>,
        >,
    >,
    pub Interrupt6_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99100>>>,
    pub Interrupt6_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99104>>>,
    pub Interrupt7_Config: Option<
        ni_fpga::Register<
            types::Interrupt7_Config,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99110>,
        >,
    >,
    pub Interrupt7_RisingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99112>>>,
    pub Interrupt7_FallingTimeStamp:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99116>>>,
    pub DMA_Rate: Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99120>>>,
    pub DMA_Config: Option<
        ni_fpga::Register<types::DMA_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99124>>,
    >,
    pub DMA_ExternalTriggers0: Option<
        ni_fpga::Register<[types::Trigger; 4], ni_fpga::ReadWrite, ni_fpga::ConstOffset<99128>>,
    >,
    pub DMA_ExternalTriggers1: Option<
        ni_fpga::Register<[types::Trigger; 4], ni_fpga::ReadWrite, ni_fpga::ConstOffset<99132>>,
    >,
    pub Alarm_TriggerTime:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99136>>>,
    pub Alarm_Enable:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99142>>>,
    pub Relay_Value: Option<
        ni_fpga::Register<types::Relay_Value, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99146>>,
    >,
    pub Power_Status: Option<
        ni_fpga::Register<types::Power_Status, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99148>>,
    >,
    pub Power_Disable: Option<
        ni_fpga::Register<types::Power_Disable, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99154>>,
    >,
    pub Power_UserVoltage6V:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99158>>>,
    pub Power_UserCurrent6V:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99162>>>,
    pub Power_UserVoltage5V:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99166>>>,
    pub Power_UserCurrent5V:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99170>>>,
    pub Power_UserVoltage3V3:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99174>>>,
    pub Power_UserCurrent3V3:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99178>>>,
    pub Power_VinVoltage:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99182>>>,
    pub Power_VinCurrent:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99186>>>,
    pub Power_OnChipTemperature:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99190>>>,
    pub Power_MXP_DIOVoltage:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99194>>>,
    pub Power_IntegratedIO:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99198>>>,
    pub Power_AOVoltage:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99202>>>,
    pub Power_FaultCounts: Option<
        ni_fpga::Register<types::Power_FaultCounts, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99204>>,
    >,
    pub Power_ResetFaultCounts:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99210>>>,
    pub Power_BrownoutVoltage250mV:
        Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99214>>>,
    pub BIST_Enable:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99218>>>,
    pub BIST_DO0SquareEnable:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99222>>>,
    pub BIST_DO0SquareTicks:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99224>>>,
    pub BIST_DO0: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99230>>>,
    pub BIST_DO1SquareEnable:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99234>>>,
    pub BIST_DO1SquareTicks:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99236>>>,
    pub BIST_DO1: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99242>>>,
    pub AO_MXP0: Option<ni_fpga::Register<u16, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99246>>>,
    pub AO_MXP1: Option<ni_fpga::Register<u16, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99250>>>,
    pub SPI_ChipSelectActiveHigh: Option<
        ni_fpga::Register<
            types::SPI_ChipSelectActiveHigh,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99254>,
        >,
    >,
    pub SPI_EnableDIO: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<5, 5, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99258>,
        >,
    >,
    pub SPI_AutoSPI1Select:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99262>>>,
    pub SPI_AutoByteCount: Option<
        ni_fpga::Register<
            types::SPI_AutoByteCount,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99266>,
        >,
    >,
    pub SPI_AutoForceOne:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99270>>>,
    pub SPI_AutoRate:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99272>>>,
    pub SPI_AutoTriggerConfig: Option<
        ni_fpga::Register<
            types::SPI_AutoTriggerConfig,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99278>,
        >,
    >,
    pub SPI_AutoChipSelect:
        Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99282>>>,
    pub SPI_AutoTx0:
        Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::ConstOffset<99284>>>,
    pub SPI_AutoTx1:
        Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::ConstOffset<99288>>>,
    pub SPI_AutoTx2:
        Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::ConstOffset<99292>>>,
    pub SPI_AutoTx3:
        Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::ConstOffset<99296>>>,
    pub SPI_AutoTx4:
        Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::ConstOffset<99300>>>,
    pub SPI_AutoTx5:
        Option<ni_fpga::Register<[u8; 4], ni_fpga::ReadWrite, ni_fpga::ConstOffset<99304>>>,
    pub SPI_TransferSkippedFullCount:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99308>>>,
    pub SPI_StallConfig: Option<
        ni_fpga::Register<types::SPI_StallConfig, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99312>>,
    >,
    pub SPI_DebugState: Option<
        ni_fpga::Register<types::SPI_DebugState, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99318>>,
    >,
    pub SPI_DebugSubstate:
        Option<ni_fpga::Register<u8, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99322>>>,
    pub SPI_DebugRevision:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99324>>>,
    pub SPI_DebugEnabled:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99328>>>,
    pub SPI_DebugIntStat:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99332>>>,
    pub SPI_DebugIntStatReadCount:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99336>>>,
    pub Accel_ADDR: Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99342>>>,
    pub Accel_CNTR: Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99346>>>,
    pub Accel_DATO: Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99350>>>,
    pub Accel_DATI: Option<ni_fpga::Register<u8, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99354>>>,
    pub Accel_CNTL: Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99358>>>,
    pub Accel_STAT: Option<ni_fpga::Register<u8, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99362>>>,
    pub Accel_CNFG: Option<ni_fpga::Register<u8, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99366>>>,
    pub Accel_GO: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99370>>>,
    pub HMB_Config: Option<
        ni_fpga::Register<types::HMB_Config, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99372>>,
    >,
    pub HMB_ForceOnce:
        Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99378>>>,
    pub DutyCycle0_Source: Option<
        ni_fpga::Register<
            types::DutyCycle0_Source,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99382>,
        >,
    >,
    pub DutyCycle0_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99386>,
        >,
    >,
    pub DutyCycle0_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99388>,
        >,
    >,
    pub DutyCycle0_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99392>,
        >,
    >,
    pub DutyCycle1_Source: Option<
        ni_fpga::Register<
            types::DutyCycle1_Source,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99398>,
        >,
    >,
    pub DutyCycle1_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99402>,
        >,
    >,
    pub DutyCycle1_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99404>,
        >,
    >,
    pub DutyCycle1_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99408>,
        >,
    >,
    pub DutyCycle2_Source: Option<
        ni_fpga::Register<
            types::DutyCycle2_Source,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99414>,
        >,
    >,
    pub DutyCycle2_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99418>,
        >,
    >,
    pub DutyCycle2_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99420>,
        >,
    >,
    pub DutyCycle2_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99424>,
        >,
    >,
    pub DutyCycle3_Source: Option<
        ni_fpga::Register<
            types::DutyCycle3_Source,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99430>,
        >,
    >,
    pub DutyCycle3_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99434>,
        >,
    >,
    pub DutyCycle3_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99436>,
        >,
    >,
    pub DutyCycle3_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99440>,
        >,
    >,
    pub DutyCycle4_Source: Option<
        ni_fpga::Register<
            types::DutyCycle4_Source,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99446>,
        >,
    >,
    pub DutyCycle4_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99450>,
        >,
    >,
    pub DutyCycle4_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99452>,
        >,
    >,
    pub DutyCycle4_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99456>,
        >,
    >,
    pub DutyCycle5_Source: Option<
        ni_fpga::Register<
            types::DutyCycle5_Source,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99462>,
        >,
    >,
    pub DutyCycle5_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99466>,
        >,
    >,
    pub DutyCycle5_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99468>,
        >,
    >,
    pub DutyCycle5_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99472>,
        >,
    >,
    pub DutyCycle6_Source: Option<
        ni_fpga::Register<
            types::DutyCycle6_Source,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99478>,
        >,
    >,
    pub DutyCycle6_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99482>,
        >,
    >,
    pub DutyCycle6_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99484>,
        >,
    >,
    pub DutyCycle6_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99488>,
        >,
    >,
    pub DutyCycle7_Source: Option<
        ni_fpga::Register<
            types::DutyCycle7_Source,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99494>,
        >,
    >,
    pub DutyCycle7_Frequency: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<11, 11, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99498>,
        >,
    >,
    pub DutyCycle7_Output: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<31, 31, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99500>,
        >,
    >,
    pub DutyCycle7_HighTicks: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<20, 20, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99504>,
        >,
    >,
    pub LED_OutputSelect: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<4, 4, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99510>,
        >,
    >,
    pub LED_StringLength:
        Option<ni_fpga::Register<u16, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99514>>>,
    pub LED_Load: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99518>>>,
    pub LED_Reset: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99522>>>,
    pub LED_Start: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99526>>>,
    pub LED_Abort: Option<ni_fpga::Register<bool, ni_fpga::ReadWrite, ni_fpga::ConstOffset<99530>>>,
    pub LED_SyncTiming: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<16, 16, false>,
            ni_fpga::ReadWrite,
            ni_fpga::ConstOffset<99534>,
        >,
    >,
    pub LED_HighBitTickTiming:
        Option<ni_fpga::Register<[u8; 2], ni_fpga::ReadWrite, ni_fpga::ConstOffset<99538>>>,
    pub LED_LowBitTickTiming:
        Option<ni_fpga::Register<[u8; 2], ni_fpga::ReadWrite, ni_fpga::ConstOffset<99542>>>,
    pub LED_Active: Option<ni_fpga::Register<bool, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99546>>>,
    pub LED_PixelWriteIndex:
        Option<ni_fpga::Register<i16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99550>>>,
    pub LED_PixelOutputIndex:
        Option<ni_fpga::Register<u16, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99554>>>,
    pub HMB_ReadData:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99556>>>,
    pub HMB_WriteCount:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99560>>>,
    pub HMB_WriteAddress: Option<
        ni_fpga::Register<
            ni_fpga::fxp::FXP<9, 9, false>,
            ni_fpga::ReadOnly,
            ni_fpga::ConstOffset<99566>,
        >,
    >,
    pub HMB_WriteData:
        Option<ni_fpga::Register<u32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99568>>>,
    pub HMB_LoopCount:
        Option<ni_fpga::Register<i32, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99572>>>,
    pub HMB_WriteReadyForInput:
        Option<ni_fpga::Register<bool, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99578>>>,
    pub HMB_ReqReadyForInput:
        Option<ni_fpga::Register<bool, ni_fpga::ReadOnly, ni_fpga::ConstOffset<99582>>>,
    pub ViControl: Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<94208>>>,
    pub DiagramReset:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<94216>>>,
    pub InterruptEnable:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<90112>>>,
    pub InterruptMask:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<90120>>>,
    pub InterruptStatus:
        Option<ni_fpga::Register<u32, ni_fpga::ReadWrite, ni_fpga::ConstOffset<90124>>>,
}
impl FpgaBitfile {
    pub fn take() -> Option<Self> {
        static REGISTERS: std::sync::Mutex<Option<FpgaBitfile>> =
            std::sync::Mutex::new(Some(FpgaBitfile {
                LocalTime: Some(unsafe { ni_fpga::Register::new_const() }),
                Revision: Some(unsafe { ni_fpga::Register::new_const() }),
                Version: Some(unsafe { ni_fpga::Register::new_const() }),
                LocalTimeUpper: Some(unsafe { ni_fpga::Register::new_const() }),
                LEDs: Some(unsafe { ni_fpga::Register::new_const() }),
                UserButton: Some(unsafe { ni_fpga::Register::new_const() }),
                InterruptForceOnce: Some(unsafe { ni_fpga::Register::new_const() }),
                InterruptForceNumber: Some(unsafe { ni_fpga::Register::new_const() }),
                SysWatchdog_Status: Some(unsafe { ni_fpga::Register::new_const() }),
                SysWatchdog_Command: Some(unsafe { ni_fpga::Register::new_const() }),
                SysWatchdog_Challenge: Some(unsafe { ni_fpga::Register::new_const() }),
                SysWatchdog_Timer: Some(unsafe { ni_fpga::Register::new_const() }),
                SysWatchdog_Active: Some(unsafe { ni_fpga::Register::new_const() }),
                SysWatchdog_ForcedKills: Some(unsafe { ni_fpga::Register::new_const() }),
                AI_ReadSelect: Some(unsafe { ni_fpga::Register::new_const() }),
                AI_LatchOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                AI_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                AI_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                AI_ScanList: Some(unsafe { ni_fpga::Register::new_const() }),
                AI_OversampleBits: Some(unsafe { ni_fpga::Register::new_const() }),
                AI_AverageBits: Some(unsafe { ni_fpga::Register::new_const() }),
                AI_LoopTiming: Some(unsafe { ni_fpga::Register::new_const() }),
                Accumulator0_Center: Some(unsafe { ni_fpga::Register::new_const() }),
                Accumulator0_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Accumulator0_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Accumulator0_Deadband: Some(unsafe { ni_fpga::Register::new_const() }),
                Accumulator1_Center: Some(unsafe { ni_fpga::Register::new_const() }),
                Accumulator1_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Accumulator1_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Accumulator1_Deadband: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger0_SourceSelect: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger0_UpperLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger0_LowerLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger1_SourceSelect: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger1_UpperLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger1_LowerLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger2_SourceSelect: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger2_UpperLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger2_LowerLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger3_SourceSelect: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger3_UpperLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger3_LowerLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger4_SourceSelect: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger4_UpperLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger4_LowerLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger5_SourceSelect: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger6_UpperLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger6_LowerLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger6_SourceSelect: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger5_UpperLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger5_LowerLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger7_SourceSelect: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger7_UpperLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                AnalogTrigger7_LowerLimit: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_LoopTiming: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_CycleStartTimeUpper: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_CycleStartTime: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_PeriodScaleHdr: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_PeriodScaleMXP: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_ZeroLatch: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_Hdr0: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_Hdr1: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_Hdr2: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_Hdr3: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_Hdr4: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_Hdr5: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_Hdr6: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_Hdr7: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_Hdr8: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_Hdr9: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_MXP0: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_MXP1: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_MXP2: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_MXP3: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_MXP4: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_MXP5: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_MXP6: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_MXP7: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_MXP8: Some(unsafe { ni_fpga::Register::new_const() }),
                PWM_MXP9: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_OutputEnable: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_DO: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_DI: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_FilterSelectHdr: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_FilterPeriodHdr0: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_FilterPeriodHdr1: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_FilterPeriodHdr2: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_FilterSelectMXP: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_FilterPeriodMXP0: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_FilterPeriodMXP1: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_FilterPeriodMXP2: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_EnableMXPSpecialFunction: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_PulseLength: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_Pulse: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_PWMDutyCycleA: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_PWMDutyCycleB: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_PWMOutputSelect: Some(unsafe { ni_fpga::Register::new_const() }),
                DIO_PWMPeriodPower: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter0_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter0_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter0_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter0_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter0_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter1_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter1_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter1_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter1_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter1_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter2_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter2_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter2_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter2_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter2_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter3_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter3_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter3_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter3_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter3_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter4_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter4_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter4_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter4_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter4_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter5_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter5_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter5_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter5_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter5_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter6_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter6_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter6_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter6_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter6_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter7_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter7_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter7_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter7_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Counter7_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder0_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder0_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder0_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder0_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder0_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder1_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder1_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder1_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder1_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder1_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder2_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder2_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder2_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder2_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder2_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder3_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder3_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder3_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder3_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder3_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder4_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder4_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder4_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder4_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder4_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder5_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder5_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder5_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder5_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder5_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder6_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder6_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder6_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder6_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder6_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder7_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder7_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder7_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder7_TimerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                Encoder7_TimerOutput: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt0_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt0_RisingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt0_FallingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt1_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt1_RisingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt1_FallingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt2_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt2_RisingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt2_FallingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt3_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt3_RisingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt3_FallingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt4_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt4_RisingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt4_FallingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt5_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt5_RisingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt5_FallingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt6_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt6_RisingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt6_FallingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt7_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt7_RisingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                Interrupt7_FallingTimeStamp: Some(unsafe { ni_fpga::Register::new_const() }),
                DMA_Rate: Some(unsafe { ni_fpga::Register::new_const() }),
                DMA_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                DMA_ExternalTriggers0: Some(unsafe { ni_fpga::Register::new_const() }),
                DMA_ExternalTriggers1: Some(unsafe { ni_fpga::Register::new_const() }),
                Alarm_TriggerTime: Some(unsafe { ni_fpga::Register::new_const() }),
                Alarm_Enable: Some(unsafe { ni_fpga::Register::new_const() }),
                Relay_Value: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_Status: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_Disable: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_UserVoltage6V: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_UserCurrent6V: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_UserVoltage5V: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_UserCurrent5V: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_UserVoltage3V3: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_UserCurrent3V3: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_VinVoltage: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_VinCurrent: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_OnChipTemperature: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_MXP_DIOVoltage: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_IntegratedIO: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_AOVoltage: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_FaultCounts: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_ResetFaultCounts: Some(unsafe { ni_fpga::Register::new_const() }),
                Power_BrownoutVoltage250mV: Some(unsafe { ni_fpga::Register::new_const() }),
                BIST_Enable: Some(unsafe { ni_fpga::Register::new_const() }),
                BIST_DO0SquareEnable: Some(unsafe { ni_fpga::Register::new_const() }),
                BIST_DO0SquareTicks: Some(unsafe { ni_fpga::Register::new_const() }),
                BIST_DO0: Some(unsafe { ni_fpga::Register::new_const() }),
                BIST_DO1SquareEnable: Some(unsafe { ni_fpga::Register::new_const() }),
                BIST_DO1SquareTicks: Some(unsafe { ni_fpga::Register::new_const() }),
                BIST_DO1: Some(unsafe { ni_fpga::Register::new_const() }),
                AO_MXP0: Some(unsafe { ni_fpga::Register::new_const() }),
                AO_MXP1: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_ChipSelectActiveHigh: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_EnableDIO: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_AutoSPI1Select: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_AutoByteCount: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_AutoForceOne: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_AutoRate: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_AutoTriggerConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_AutoChipSelect: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_AutoTx0: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_AutoTx1: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_AutoTx2: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_AutoTx3: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_AutoTx4: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_AutoTx5: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_TransferSkippedFullCount: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_StallConfig: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_DebugState: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_DebugSubstate: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_DebugRevision: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_DebugEnabled: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_DebugIntStat: Some(unsafe { ni_fpga::Register::new_const() }),
                SPI_DebugIntStatReadCount: Some(unsafe { ni_fpga::Register::new_const() }),
                Accel_ADDR: Some(unsafe { ni_fpga::Register::new_const() }),
                Accel_CNTR: Some(unsafe { ni_fpga::Register::new_const() }),
                Accel_DATO: Some(unsafe { ni_fpga::Register::new_const() }),
                Accel_DATI: Some(unsafe { ni_fpga::Register::new_const() }),
                Accel_CNTL: Some(unsafe { ni_fpga::Register::new_const() }),
                Accel_STAT: Some(unsafe { ni_fpga::Register::new_const() }),
                Accel_CNFG: Some(unsafe { ni_fpga::Register::new_const() }),
                Accel_GO: Some(unsafe { ni_fpga::Register::new_const() }),
                HMB_Config: Some(unsafe { ni_fpga::Register::new_const() }),
                HMB_ForceOnce: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle0_Source: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle0_Frequency: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle0_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle0_HighTicks: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle1_Source: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle1_Frequency: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle1_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle1_HighTicks: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle2_Source: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle2_Frequency: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle2_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle2_HighTicks: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle3_Source: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle3_Frequency: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle3_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle3_HighTicks: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle4_Source: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle4_Frequency: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle4_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle4_HighTicks: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle5_Source: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle5_Frequency: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle5_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle5_HighTicks: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle6_Source: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle6_Frequency: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle6_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle6_HighTicks: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle7_Source: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle7_Frequency: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle7_Output: Some(unsafe { ni_fpga::Register::new_const() }),
                DutyCycle7_HighTicks: Some(unsafe { ni_fpga::Register::new_const() }),
                LED_OutputSelect: Some(unsafe { ni_fpga::Register::new_const() }),
                LED_StringLength: Some(unsafe { ni_fpga::Register::new_const() }),
                LED_Load: Some(unsafe { ni_fpga::Register::new_const() }),
                LED_Reset: Some(unsafe { ni_fpga::Register::new_const() }),
                LED_Start: Some(unsafe { ni_fpga::Register::new_const() }),
                LED_Abort: Some(unsafe { ni_fpga::Register::new_const() }),
                LED_SyncTiming: Some(unsafe { ni_fpga::Register::new_const() }),
                LED_HighBitTickTiming: Some(unsafe { ni_fpga::Register::new_const() }),
                LED_LowBitTickTiming: Some(unsafe { ni_fpga::Register::new_const() }),
                LED_Active: Some(unsafe { ni_fpga::Register::new_const() }),
                LED_PixelWriteIndex: Some(unsafe { ni_fpga::Register::new_const() }),
                LED_PixelOutputIndex: Some(unsafe { ni_fpga::Register::new_const() }),
                HMB_ReadData: Some(unsafe { ni_fpga::Register::new_const() }),
                HMB_WriteCount: Some(unsafe { ni_fpga::Register::new_const() }),
                HMB_WriteAddress: Some(unsafe { ni_fpga::Register::new_const() }),
                HMB_WriteData: Some(unsafe { ni_fpga::Register::new_const() }),
                HMB_LoopCount: Some(unsafe { ni_fpga::Register::new_const() }),
                HMB_WriteReadyForInput: Some(unsafe { ni_fpga::Register::new_const() }),
                HMB_ReqReadyForInput: Some(unsafe { ni_fpga::Register::new_const() }),
                ViControl: Some(unsafe { ni_fpga::Register::new_const() }),
                DiagramReset: Some(unsafe { ni_fpga::Register::new_const() }),
                InterruptEnable: Some(unsafe { ni_fpga::Register::new_const() }),
                InterruptMask: Some(unsafe { ni_fpga::Register::new_const() }),
                InterruptStatus: Some(unsafe { ni_fpga::Register::new_const() }),
            }));
        let mut lock = REGISTERS.lock().unwrap();
        lock.take()
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
