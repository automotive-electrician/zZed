#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawValueError {
    MismatchPID(&'static str),
    MismatchMode(&'static str),
    InvalidData(&'static str),
    InvalidLength {
        text: &'static str,
        expected: usize,
        actual: usize,
    },
}
