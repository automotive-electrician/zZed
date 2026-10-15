#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuelSystemStatus {
    MotorOff,
    OpenLoopInsufficientTemperature,
    ClosedLoop,
    OpenLoopDrivingConditions,
    OpenLoopSystemFault,
    ClosedLoopSensorFault,
}
