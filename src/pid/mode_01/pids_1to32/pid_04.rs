use crate::pid::{
    consts::{modes::mode_01::MODE01, pids::mode_01::PID_04},
    core::{
        enums::{parameter_value_enum::ParameterValue, raw_value_enum::RawValue},
        errors::{paramter_value_error::ParameterValueError, raw_value_error::RawValueError},
        traits::parameter_trait::ParamterTrait,
        validations::raw_value_validation::raw_value_validation,
    },
};

// Calculated engine load
pub struct CalculatedEngineLoad;
impl ParamterTrait for CalculatedEngineLoad {
    const MODE: u8 = MODE01;
    const PID: u8 = PID_04;
    fn raw_value(value: &[u8]) -> Result<RawValue, RawValueError> {
        raw_value_validation(value, 4, MODE01, PID_04)?;
        Ok(RawValue::U8(value[2]))
    }
    fn parameter_value(value: RawValue) -> Result<ParameterValue, ParameterValueError> {
        let RawValue::U8(x) = value else {
            return Err(ParameterValueError::InvalidData(""));
        };

        Ok(ParameterValue::F32(f32::from(x) * 0.39215687))
    }
}
