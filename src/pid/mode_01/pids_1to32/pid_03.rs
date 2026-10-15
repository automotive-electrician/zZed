// Fuel system status

use crate::pid::{
    consts::{modes::mode_01::MODE01, pids::mode_01::PID_03},
    core::{
        enums::{
            mode_01::pid_03::FuelSystemStatus, parameter_value_enum::ParameterValue,
            raw_value_enum::RawValue,
        },
        errors::{paramter_value_error::ParameterValueError, raw_value_error::RawValueError},
        structs::mode_01::pid_03::Pid03Value,
        traits::parameter_trait::ParamterTrait,
        validations::raw_value_validation::raw_value_validation,
    },
};

pub struct FuelSystemStatusPid03;
impl ParamterTrait for FuelSystemStatusPid03 {
    const MODE: u8 = MODE01;
    const PID: u8 = PID_03;
    fn raw_value(value: &[u8]) -> Result<RawValue, RawValueError> {
        raw_value_validation(value, 4, MODE01, PID_03)?;
        Ok(RawValue::TwoBytes([value[2], value[3]]))
    }
    fn parameter_value(value: RawValue) -> Result<ParameterValue, ParameterValueError> {
        let RawValue::TwoBytes([a, b]) = value else {
            return Err(ParameterValueError::InvalidData(""));
        };

        let system1 = match a {
            0x00 => FuelSystemStatus::MotorOff,
            0x01 => FuelSystemStatus::OpenLoopInsufficientTemperature,
            0x02 => FuelSystemStatus::ClosedLoop,
            0x04 => FuelSystemStatus::OpenLoopDrivingConditions,
            0x08 => FuelSystemStatus::OpenLoopSystemFault,
            0x10 => FuelSystemStatus::ClosedLoopSensorFault,
            _ => return Err(ParameterValueError::InvalidData("")),
        };

        let system2 = match b {
            0x00 => FuelSystemStatus::MotorOff,
            0x01 => FuelSystemStatus::OpenLoopInsufficientTemperature,
            0x02 => FuelSystemStatus::ClosedLoop,
            0x04 => FuelSystemStatus::OpenLoopDrivingConditions,
            0x08 => FuelSystemStatus::OpenLoopSystemFault,
            0x10 => FuelSystemStatus::ClosedLoopSensorFault,
            _ => return Err(ParameterValueError::InvalidData("")),
        };

        Ok(ParameterValue::Pid03(Pid03Value { system1, system2 }))
    }
}
