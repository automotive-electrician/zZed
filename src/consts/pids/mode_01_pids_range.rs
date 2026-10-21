// PID 4 – Calculated engine load (%)
pub const PID_04_MIN: f32 = 0.0;
pub const PID_04_MAX: f32 = 100.0;

// PID 5 – Engine coolant temperature (°C)
pub const PID_05_MIN: i16 = -40;
pub const PID_05_MAX: i16 = 215;

// PID 6 – Short term fuel trim Bank 1 (%)
pub const PID_06_MIN: f32 = -100.0;
pub const PID_06_MAX: f32 = 99.2;

// PID 7 – Long term fuel trim Bank 1 (%)
pub const PID_07_MIN: f32 = -100.0;
pub const PID_07_MAX: f32 = 99.2;

// PID 8 – Short term fuel trim Bank 2 (%)
pub const PID_08_MIN: f32 = -100.0;
pub const PID_08_MAX: f32 = 99.2;

// PID 9 – Long term fuel trim Bank 2 (%)
pub const PID_09_MIN: f32 = -100.0;
pub const PID_09_MAX: f32 = 99.2;

// PID 10 – Fuel pressure, gauge (kPa)
pub const PID_10_MIN: f32 = 0.0;
pub const PID_10_MAX: f32 = 765.0;

// PID 11 – Intake manifold absolute pressure (kPa)
pub const PID_11_MIN: f32 = 0.0;
pub const PID_11_MAX: f32 = 255.0;

// PID 12 – Engine RPM (rpm)
pub const PID_12_MIN: f32 = 0.0;
pub const PID_12_MAX: f32 = 16_383.75;

// PID 13 – Vehicle speed (km/h)
pub const PID_13_MIN: u8 = 0;
pub const PID_13_MAX: u8 = 255;

// PID 14 – Timing advance (° before TDC)
pub const PID_14_MIN: f32 = -64.0;
pub const PID_14_MAX: f32 = 63.5;

// PID 15 – Intake air temperature (°C)
pub const PID_15_MIN: f32 = -40.0;
pub const PID_15_MAX: f32 = 215.0;

// PID 16 – MAF air flow rate (g/s)
pub const PID_16_MIN: f32 = 0.0;
pub const PID_16_MAX: f32 = 655.35;

// PID 17 – Throttle position (%)
pub const PID_17_MIN: f32 = 0.0;
pub const PID_17_MAX: f32 = 100.0;

// PID 20 – Oxygen Sensor 1, Bank 1, Sensor 1 – voltage (V)
pub const PID_20_MIN: f32 = 0.0;
pub const PID_20_MAX: f32 = 1.275;

// PID 21 – Oxygen Sensor 2, Bank 1, Sensor 2 – voltage (V)
pub const PID_21_MIN: f32 = 0.0;
pub const PID_21_MAX: f32 = 1.275;

// PID 31 – Run time since engine start (s)
pub const PID_31_MIN: f32 = 0.0;
pub const PID_31_MAX: f32 = 65_535.0;

// PID 47 – Fuel tank level input (%)
pub const PID_47_MIN: f32 = 0.0;
pub const PID_47_MAX: f32 = 100.0;

// PID 51 – Absolute barometric pressure (kPa)
pub const PID_51_MIN: f32 = 0.0;
pub const PID_51_MAX: f32 = 255.0;

// PID 66 – Control module voltage (V)
pub const PID_66_MIN: f32 = 0.0;
pub const PID_66_MAX: f32 = 65.535;

// PID 92 – Engine oil temperature (°C)
pub const PID_92_MIN: f32 = -40.0;
pub const PID_92_MAX: f32 = 215.0;
