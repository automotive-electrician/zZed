use crate::pid::core::errors::paramter_value_error::ParameterValueError;

pub fn parameter_range_validation<T>(value: T, min: T, max: T) -> Result<(), ParameterValueError>
where
    T: PartialOrd,
{
    if value >= min && value <= max {
        Ok(())
    } else {
        return Err(ParameterValueError::OutOfRange(
            "The value is outside the standard range.",
        ));
    }
}
