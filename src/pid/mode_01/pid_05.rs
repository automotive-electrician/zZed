// Engine coolant temperature

use crate::{
    consts::{
        modes::MODE_01,
        pids::{
            mode_01::PID_05,
            mode_01_pid_names::PID_05_NAME,
            mode_01_pids_range::{PID_05_MAX, PID_05_MIN},
        },
        units::CELSIUS,
    },
    pid::core::{
        DecodedValue, Pid, PidError, RawValue, pid_range_validation, raw_value_validation,
    },
};

pub struct EngineCoolantTemperature;
impl Pid for EngineCoolantTemperature {
    const MODE: u8 = MODE_01;
    const PID: u8 = PID_05;
    const NAME: &'static str = PID_05_NAME;
    const UNIT: Option<&'static str> = CELSIUS;
    const MIN: f64 = PID_05_MIN as f64;
    const MAX: f64 = PID_05_MAX as f64;
    fn raw_value(b: &[u8]) -> Result<RawValue, PidError> {
        raw_value_validation(b, 3, MODE_01, PID_05)?;
        Ok(RawValue::U8(b[2]))
    }
    fn decoder(value: RawValue) -> Result<DecodedValue, PidError> {
        let RawValue::U8(x) = value else {
            return Err(PidError::InvalidData(
                "PID 05: expected RawValue::U8, got a different variant",
            ));
        };

        let engine_coolant_temperature = i16::from(x) - 40;
        pid_range_validation(engine_coolant_temperature, PID_05_MIN, PID_05_MAX)?;
        Ok(DecodedValue::I16(engine_coolant_temperature))
    }
}
