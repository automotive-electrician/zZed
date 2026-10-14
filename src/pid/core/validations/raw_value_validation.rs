use crate::pid::core::errors::raw_value_error::RawValueError;

pub fn raw_value_validation(
    value: &[u8],
    min_len: usize,
    mode: u8,
    pid: u8,
) -> Result<(), RawValueError> {
    debug_assert!(
        min_len >= 2,
        "min_len must be >= 2 to access mode+pid bytes"
    );

    if value.len() < min_len {
        return Err(RawValueError::InvalidData(""));
    }

    if value[0] != 0x40 + mode {
        return Err(RawValueError::InvalidData(""));
    }

    if value[1] != pid {
        return Err(RawValueError::InvalidData(""));
    }

    Ok(())
}
