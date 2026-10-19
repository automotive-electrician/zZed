// Monitor status since DTCs cleared

use crate::pid::{
    consts::{modes::MODE_01, pids::mode_01::PID_01},
    core::{
        enums::{parameter_value_enum::ParameterValue, raw_value_enum::RawValue},
        errors::{paramter_value_error::ParameterValueError, raw_value_error::RawValueError},
        structs::mode_01::pid_01::Pid01Value,
        traits::parameter_trait::ParamterTrait,
        validations::raw_value_validation::raw_value_validation,
    },
};

pub struct MonitorStatusSinceDTCsCleared;
impl ParamterTrait for MonitorStatusSinceDTCsCleared {
    const MODE: u8 = MODE_01;
    const PID: u8 = PID_01;

    fn raw_value(value: &[u8]) -> Result<RawValue, RawValueError> {
        raw_value_validation(value, 6, MODE_01, PID_01)?;
        Ok(RawValue::ForBytes([value[2], value[3], value[4], value[5]]))
    }

    fn parameter_value(value: RawValue) -> Result<ParameterValue, ParameterValueError> {
        let raw = match value {
            RawValue::ForBytes(x) => x,
            _ => return Err(ParameterValueError::InvalidData("")),
        };

        let a = raw[0];
        let b = raw[1];
        let c = raw[2];
        let d = raw[3];

        Ok(ParameterValue::Pid01(Pid01Value {
            mil_on: (a & 0x80) != 0,
            dtc_count: a & 0x7F,

            comprehensive_supported: (b & 0x04) != 0,
            fuel_system_supported: (b & 0x02) != 0,
            misfire_supported: (b & 0x01) != 0,

            comprehensive_incomplete: (b & 0x40) != 0,
            fuel_system_incomplete: (b & 0x20) != 0,
            misfire_incomplete: (b & 0x10) != 0,

            compression_ignition: (b & 0x08) != 0,

            byte_c: c,
            byte_d: d,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monitor_status_since_dtcs_cleared_test() {
        let data = [0x41, 0x01, 0xF1, 0x0F, 0x00, 0x00];

        let raw = MonitorStatusSinceDTCsCleared::raw_value(&data).unwrap();
        println!("Raw: {:?}", raw);

        let parameter = MonitorStatusSinceDTCsCleared::parameter_value(raw).unwrap();
        println!("Parameter: {:?}", parameter)
    }
}
