// Short term fuel trim (STFT)—Bank 2

use crate::pid::{
    consts::{
        modes::MODE_01,
        pids::mode_01::PID_08,
        pids_range::{PID_08_MAX, PID_08_MIN},
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

pub struct ShortTermFuelTrimBank2;
impl ParamterTrait for ShortTermFuelTrimBank2 {
    const MODE: u8 = MODE_01;
    const PID: u8 = PID_08;

    fn raw_value(value: &[u8]) -> Result<RawValue, RawValueError> {
        raw_value_validation(value, 3, MODE_01, PID_08)?;
        Ok(RawValue::U8(value[2]))
    }

    fn parameter_value(value: RawValue) -> Result<ParameterValue, ParameterValueError> {
        let RawValue::U8(x) = value else {
            return Err(ParameterValueError::InvalidData("()"));
        };

        let stftb1 = (f32::from(x) * 0.78125) - 100.00;
        parameter_range_validation(stftb1, PID_08_MIN, PID_08_MAX)?;
        Ok(ParameterValue::F32(stftb1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stftb1_test() {
        let data = [0x41, 0x06, 0xF1];

        let raw = ShortTermFuelTrimBank2::raw_value(&data).unwrap();
        println!("Raw: {:?}", raw);

        let parameter = ShortTermFuelTrimBank2::parameter_value(raw).unwrap();
        println!("Parameter: {:?}", parameter);
    }
}
