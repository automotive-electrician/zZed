use crate::pid::{
    consts::{modes::mode_01::MODE01, pids::mode_01::PID_00},
    core::{
        enums::{parameter_value_enum::ParameterValue, raw_value_enum::RawValue},
        errors::{paramter_value_error::ParameterValueError, raw_value_error::RawValueError},
        traits::parameter_trait::ParamterTrait,
        validations::raw_value_validation::raw_value_validation,
    },
};

pub struct PIDsSupported_1To32;
impl ParamterTrait for PIDsSupported_1To32 {
    const MODE: u8 = MODE01;
    const PID: u8 = PID_00;
    fn raw_value(value: &[u8]) -> Result<RawValue, RawValueError> {
        raw_value_validation(value, 6, MODE01, PID_00)?;
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

        let supported = [
            a & 0x80 != 0, // PID 01
            a & 0x40 != 0, // PID 02
            a & 0x20 != 0, // PID 03
            a & 0x10 != 0, // PID 04
            a & 0x08 != 0, // PID 05
            a & 0x04 != 0, // PID 06
            a & 0x02 != 0, // PID 07
            a & 0x01 != 0, // PID 08
            b & 0x80 != 0, // PID 09
            b & 0x40 != 0, // PID 10
            b & 0x20 != 0, // PID 11
            b & 0x10 != 0, // PID 12
            b & 0x08 != 0, // PID 13
            b & 0x04 != 0, // PID 14
            b & 0x02 != 0, // PID 15
            b & 0x01 != 0, // PID 16
            c & 0x80 != 0, // PID 17
            c & 0x40 != 0, // PID 18
            c & 0x20 != 0, // PID 19
            c & 0x10 != 0, // PID 20
            c & 0x08 != 0, // PID 21
            c & 0x04 != 0, // PID 22
            c & 0x02 != 0, // PID 23
            c & 0x01 != 0, // PID 24
            d & 0x80 != 0, // PID 25
            d & 0x40 != 0, // PID 26
            d & 0x20 != 0, // PID 27
            d & 0x10 != 0, // PID 28
            d & 0x08 != 0, // PID 29
            d & 0x04 != 0, // PID 30
            d & 0x02 != 0, // PID 31
            d & 0x01 != 0, // PID 32
        ];

        Ok(ParameterValue::PidsSupported(supported))
    }
}
