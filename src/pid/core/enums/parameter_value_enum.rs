use crate::pid::core::structs::mode_01::pid_01::Pid01Value;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterValue {
    U32(u32),
    F32(f32),
    PidsSupported([bool; 32]),
    Pid01(Pid01Value),
}
