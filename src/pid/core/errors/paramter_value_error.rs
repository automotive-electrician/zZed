#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterValueError {
    InvalidData(&'static str),
    OutOfRange(&'static str),
}
