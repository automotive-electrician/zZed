use crate::pid::core::enums::mode_01::pid_03::FuelSystemStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pid03Value {
    pub system1: FuelSystemStatus,
    pub system2: FuelSystemStatus,
}
