#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RawValue {
    U8(u8),
    U16(u16),
    I8(i8),
    I16(i16),
    U32(u32),
    F32(f32),
    TwoBytes([u8; 2]),
}
