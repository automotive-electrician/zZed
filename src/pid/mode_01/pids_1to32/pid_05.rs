// Engine coolant temperature

use crate::pid::{
    consts::{
        modes::MODE_01,
        pids::mode_01::PID_05,
        pids_range::{PID_05_MAX, PID_05_MIN},
    },
    core::{
        enums::{parameter_value_enum::ParameterValue, raw_value_enum::RawValue},
        errors::{paramter_value_error::ParameterValueError, raw_value_error::RawValueError},
        traits::parameter_trait::ParamterTrait,
        validations::{
            pid_range_validation::parameter_range_validation,
            raw_value_validation::raw_value_validation,
        },
    },
};

pub struct EngineCoolantTemperature;
impl ParamterTrait for EngineCoolantTemperature {
    const MODE: u8 = MODE_01;
    const PID: u8 = PID_05;

    fn raw_value(value: &[u8]) -> Result<RawValue, RawValueError> {
        raw_value_validation(value, 3, MODE_01, PID_05)?;
        Ok(RawValue::U8(value[2]))
    }

    fn parameter_value(value: RawValue) -> Result<ParameterValue, ParameterValueError> {
        let RawValue::U8(x) = value else {
            return Err(ParameterValueError::InvalidData("Error"));
        };

        let engine_coolant_temperature = i16::from(x) - 40;
        parameter_range_validation(engine_coolant_temperature, PID_05_MIN, PID_05_MAX)?;
        Ok(ParameterValue::I16(engine_coolant_temperature))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value_test() {}

    #[test]
    fn engine_coolant_temperature_test() {
        let data = [0x41, 0x05, 0xCD];

        let raw = EngineCoolantTemperature::raw_value(&data).unwrap();
        println!("Raw: {:?}", raw);

        let parameter = EngineCoolantTemperature::parameter_value(raw).unwrap();
        println!("Parameter: {:?}", parameter);
    }
}
