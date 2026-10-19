#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuelSystemStatusEnum {
    MotorOff,
    OpenLoopInsufficientTemperature,
    ClosedLoop,
    OpenLoopDrivingConditions,
    OpenLoopSystemFault,
    ClosedLoopSensorFault,
}
