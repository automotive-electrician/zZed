use crate::pid::core::structs::mode_01::{pid_01::Pid01Value, pid_03::Pid03Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterValue {
    U8(u8),
    U32(u32),
    F32(f32),
    I16(i16),
    U16(u16),
    PidsSupported([bool; 32]),
    Pid01(Pid01Value),
    Pid03(Pid03Value),
}
