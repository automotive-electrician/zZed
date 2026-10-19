// Fuel system status

use crate::pid::{
    consts::{modes::MODE_01, pids::mode_01::PID_03},
    core::{
        enums::{
            mode_01::pid_03::FuelSystemStatusEnum, parameter_value_enum::ParameterValue,
            raw_value_enum::RawValue,
        },
        errors::{paramter_value_error::ParameterValueError, raw_value_error::RawValueError},
        structs::mode_01::pid_03::Pid03Value,
        traits::parameter_trait::ParamterTrait,
        validations::raw_value_validation::raw_value_validation,
    },
};

pub struct FuelSystemStatus;
impl ParamterTrait for FuelSystemStatus {
    const MODE: u8 = MODE_01;
    const PID: u8 = PID_03;
    fn raw_value(value: &[u8]) -> Result<RawValue, RawValueError> {
        raw_value_validation(value, 4, MODE_01, PID_03)?;
        Ok(RawValue::TwoBytes([value[2], value[3]]))
    }
    fn parameter_value(value: RawValue) -> Result<ParameterValue, ParameterValueError> {
        let RawValue::TwoBytes([a, b]) = value else {
            return Err(ParameterValueError::InvalidData(""));
        };

        let system1 = match a {
            0x00 => FuelSystemStatusEnum::MotorOff,
            0x01 => FuelSystemStatusEnum::OpenLoopInsufficientTemperature,
            0x02 => FuelSystemStatusEnum::ClosedLoop,
            0x04 => FuelSystemStatusEnum::OpenLoopDrivingConditions,
            0x08 => FuelSystemStatusEnum::OpenLoopSystemFault,
            0x10 => FuelSystemStatusEnum::ClosedLoopSensorFault,
            _ => return Err(ParameterValueError::InvalidData("")),
        };

        let system2 = match b {
            0x00 => FuelSystemStatusEnum::MotorOff,
            0x01 => FuelSystemStatusEnum::OpenLoopInsufficientTemperature,
            0x02 => FuelSystemStatusEnum::ClosedLoop,
            0x04 => FuelSystemStatusEnum::OpenLoopDrivingConditions,
            0x08 => FuelSystemStatusEnum::OpenLoopSystemFault,
            0x10 => FuelSystemStatusEnum::ClosedLoopSensorFault,
            _ => return Err(ParameterValueError::InvalidData("")),
        };

        Ok(ParameterValue::Pid03(Pid03Value { system1, system2 }))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_raw_value() {
        let data = [0x41, 0x03, 0x02, 0x00];

        let raw = FuelSystemStatus::raw_value(&data).unwrap();
        println!("Raw: {:?}", raw);

        let parameter = FuelSystemStatus::parameter_value(raw).unwrap();
        println!("Parameter: {:?}", parameter)
    }
}
