#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pid01Value {
    pub mil_on: bool,
    pub dtc_count: u8,

    pub comprehensive_supported: bool,
    pub fuel_system_supported: bool,
    pub misfire_supported: bool,

    pub comprehensive_incomplete: bool,
    pub fuel_system_incomplete: bool,
    pub misfire_incomplete: bool,

    pub compression_ignition: bool,

    pub byte_c: u8,
    pub byte_d: u8,
}
