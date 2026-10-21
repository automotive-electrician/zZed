use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum RawValue {
    U8(u8),
    U16(u16),
    I8(i8),
    I16(i16),
    U32(u32),
    F32(f32),
    TwoBytes([u8; 2]),
    FourBytes([u8; 4]),
}

#[derive(Serialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum DecodedValue {
    U8(u8),
    U16(u16),
    I8(i8),
    I16(i16),
    U32(u32),
    F32(f32),
    SupportedPids([bool; 32]),
}

#[derive(Debug, Error)]
pub enum PidError {
    #[error("mode mismatch")]
    MismatchMode,
    #[error("pid mismatch")]
    MismatchPid,
    #[error("invalid length: expected {expected}, got {actual}")]
    InvalidLength { expected: usize, actual: usize },
    #[error("invalid data: {0}")]
    InvalidData(&'static str),
    #[error("out of range")]
    OutOfRange,
}

pub trait Pid {
    const MODE: u8;
    const PID: u8;
    const NAME: &'static str;
    const UNIT: Option<&'static str>;
    const MIN: f64;
    const MAX: f64;
    fn raw_value(b: &[u8]) -> Result<RawValue, PidError>;
    fn decoder(value: RawValue) -> Result<DecodedValue, PidError>;
}

pub fn raw_value_validation(value: &[u8], len: usize, mode: u8, pid: u8) -> Result<(), PidError> {
    if value.len() != len {
        return Err(PidError::InvalidLength {
            expected: len,
            actual: value.len(),
        });
    }

    if value.len() < 2 {
        return Err(PidError::InvalidData("Expected length must be at least 2"));
    }

    if value[0] != 0x40 + mode {
        return Err(PidError::MismatchMode);
    }

    if value[1] != pid {
        return Err(PidError::MismatchPid);
    }

    Ok(())
}

pub fn pid_range_validation<T>(value: T, min: T, max: T) -> Result<(), PidError>
where
    T: PartialOrd,
{
    if value >= min && value <= max {
        Ok(())
    } else {
        Err(PidError::OutOfRange)
    }
}
