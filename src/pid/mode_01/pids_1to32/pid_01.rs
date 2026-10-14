use crate::pid::{
    consts::{modes::mode_01::MODE01, pids::mode_01::PID_01},
    core::{
        enums::{parameter_value_enum::ParameterValue, raw_value_enum::RawValue},
        errors::{paramter_value_error::ParameterValueError, raw_value_error::RawValueError},
        structs::mode_01::pid_01::Pid01Value,
        traits::parameter_trait::ParamterTrait,
    },
};

pub struct MonitorStatusSinceDTCsCleared;
impl ParamterTrait for MonitorStatusSinceDTCsCleared {
    const MODE: u8 = MODE01;
    const PID: u8 = PID_01;
    fn raw_value(value: &[u8]) -> Result<RawValue, RawValueError> {
        Ok(RawValue::U32(u32::from_be_bytes([
            value[2], value[3], value[4], value[5],
        ])))
    }
    fn parameter_value(value: RawValue) -> Result<ParameterValue, ParameterValueError> {
        let raw = match value {
            RawValue::U32(x) => x,
            _ => return Err(ParameterValueError::InvalidData("")),
        };

        let bytes = raw.to_be_bytes();
        let a = bytes[0];
        let b = bytes[1];
        let c = bytes[2];
        let d = bytes[3];

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
