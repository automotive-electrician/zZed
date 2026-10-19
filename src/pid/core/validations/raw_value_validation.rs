use crate::pid::core::errors::raw_value_error::RawValueError;

pub fn raw_value_validation(
    value: &[u8],
    len: usize,
    mode: u8,
    pid: u8,
) -> Result<(), RawValueError> {
    if len < 2 {
        return Err(RawValueError::InvalidData(
            "Expected length must be at least 2",
        ));
    }

    if value.len() != len {
        return Err(RawValueError::InvalidLength {
            text: "Data length does not match",
            expected: len,
            actual: value.len(),
        });
    }

    if value[0] != 0x40 + mode {
        return Err(RawValueError::MismatchMode(
            "The selected MODE is incorrect",
        ));
    }

    if value[1] != pid {
        return Err(RawValueError::MismatchPID("The selected PID is incorrect"));
    }

    Ok(())
}
