use crate::pid::core::enums::mode_01::pid_03::FuelSystemStatusEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pid03Value {
    pub system1: FuelSystemStatusEnum,
    pub system2: FuelSystemStatusEnum,
}
