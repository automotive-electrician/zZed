// Calculated engine load

use crate::pid::{
    consts::{
        modes::MODE_01,
        pids::mode_01::PID_04,
        pids_range::{PID_04_MAX, PID_04_MIN},
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

// Calculated engine load
pub struct CalculatedEngineLoad;
impl ParamterTrait for CalculatedEngineLoad {
    const MODE: u8 = MODE_01;
    const PID: u8 = PID_04;

    fn raw_value(value: &[u8]) -> Result<RawValue, RawValueError> {
        raw_value_validation(value, 3, MODE_01, PID_04)?;
        Ok(RawValue::U8(value[2]))
    }

    fn parameter_value(value: RawValue) -> Result<ParameterValue, ParameterValueError> {
        let RawValue::U8(x) = value else {
            return Err(ParameterValueError::InvalidData("Error"));
        };

        let calculated_engine_load = f32::from(x) * 0.39215687;
        parameter_range_validation(calculated_engine_load, PID_04_MIN, PID_04_MAX)?;
        Ok(ParameterValue::F32(calculated_engine_load))
    }
}

#[cfg(test)]
mod tests {
    use crate::pid::mode_01::pids_1to32::pid_04::CalculatedEngineLoad;

    use super::*;

    #[test]
    fn test_engine_load() {
        let data = [0x41, 0x04, 0xFD];

        let raw = CalculatedEngineLoad::raw_value(&data).unwrap();
        println!("Raw value: {:?}", raw);

        let parameter = CalculatedEngineLoad::parameter_value(raw).unwrap();
        println!("Real value: {:?}", parameter);
    }
}
