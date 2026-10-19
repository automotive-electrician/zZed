// Fuel pressure (gauge pressure)

use crate::pid::{
    consts::{
        modes::MODE_01,
        pids::mode_01::PID_0A,
        pids_range::{PID_0A_MAX, PID_0A_MIN},
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

pub struct FuelPressure;
impl ParamterTrait for FuelPressure {
    const MODE: u8 = MODE_01;
    const PID: u8 = PID_0A;

    fn raw_value(value: &[u8]) -> Result<RawValue, RawValueError> {
        raw_value_validation(value, 3, MODE_01, PID_0A)?;
        Ok(RawValue::U8(value[2]))
    }

    fn parameter_value(value: RawValue) -> Result<ParameterValue, ParameterValueError> {
        let RawValue::U8(x) = value else {
            return Err(ParameterValueError::InvalidData("()"));
        };

        let fuel_pressure = u16::from(x) * 3;
        parameter_range_validation(fuel_pressure, PID_0A_MIN, PID_0A_MAX)?;
        Ok(ParameterValue::U16(fuel_pressure))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_pressure() {
        let data = [0x41, 0x0A, 0xC9];

        let raw = FuelPressure::raw_value(&data).unwrap();
        println!("Raw: {:?}", raw);

        let parameter = FuelPressure::parameter_value(raw).unwrap();
        println!("Parameter: {:?}", parameter)
    }
}
