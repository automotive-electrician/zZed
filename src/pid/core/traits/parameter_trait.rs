use crate::pid::core::{
    enums::{parameter_value_enum::ParameterValue, raw_value_enum::RawValue},
    errors::{paramter_value_error::ParameterValueError, raw_value_error::RawValueError},
};

pub trait ParamterTrait {
    const MODE: u8;
    const PID: u8;
    fn raw_value(value: &[u8]) -> Result<RawValue, RawValueError>;
    fn parameter_value(value: RawValue) -> Result<ParameterValue, ParameterValueError>;
}
