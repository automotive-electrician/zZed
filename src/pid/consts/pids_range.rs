// PID_04 — Calculated engine load (%)
pub const PID_04_MIN: f32 = 0.0;
pub const PID_04_MAX: f32 = 100.0;

// PID_05 — Engine coolant temperature (°C)
pub const PID_05_MIN: i16 = -40;
pub const PID_05_MAX: i16 = 215;

// PID_06 — Short-term fuel trim, Bank 1 (%)
pub const PID_06_MIN: f32 = -100.0;
pub const PID_06_MAX: f32 = 99.21875;

// PID_07 — Long-term fuel trim, Bank 1 (%)
pub const PID_07_MIN: f32 = -100.0;
pub const PID_07_MAX: f32 = 99.21875;

// PID_08 — Short-term fuel trim, Bank 2 (%)
pub const PID_08_MIN: f32 = -100.0;
pub const PID_08_MAX: f32 = 99.21875;

// PID_09 — Long-term fuel trim, Bank 2 (%)
pub const PID_09_MIN: f32 = -100.0;
pub const PID_09_MAX: f32 = 99.21875;

// PID_0A — Fuel pressure (kPa)
pub const PID_0A_MIN: u16 = 0;
pub const PID_0A_MAX: u16 = 765;

// PID_0B — Intake manifold absolute pressure (kPa)
pub const PID_0B_MIN: f32 = 0.0;
pub const PID_0B_MAX: f32 = 255.0;

// PID_0C — Engine RPM
pub const PID_0C_MIN: f32 = 0.0;
pub const PID_0C_MAX: f32 = 16383.75;

// PID_0D — Vehicle speed (km/h)
pub const PID_0D_MIN: f32 = 0.0;
pub const PID_0D_MAX: f32 = 255.0;

// PID_0E — Timing advance (degrees)
pub const PID_0E_MIN: f32 = -64.0;
pub const PID_0E_MAX: f32 = 63.5;

// PID_0F — Intake air temperature (°C)
pub const PID_0F_MIN: f32 = -40.0;
pub const PID_0F_MAX: f32 = 215.0;

// PID_10 — Mass air flow rate (g/s)
pub const PID_10_MIN: f32 = 0.0;
pub const PID_10_MAX: f32 = 655.35;

// PID_11 — Throttle position (%)
pub const PID_11_MIN: f32 = 0.0;
pub const PID_11_MAX: f32 = 100.0;

// PID_14..PID_1B — Oxygen sensor voltage (V)
pub const OXYGEN_SENSOR_VOLTAGE_MIN: f32 = 0.0;
pub const OXYGEN_SENSOR_VOLTAGE_MAX: f32 = 1.275;

// PID_14..PID_1B — Oxygen sensor fuel trim (%)
pub const OXYGEN_SENSOR_TRIM_MIN: f32 = -100.0;
pub const OXYGEN_SENSOR_TRIM_MAX: f32 = 99.21875;

// PID_1F — Run time since engine start (s)
pub const PID_1F_MIN: f32 = 0.0;
pub const PID_1F_MAX: f32 = 65535.0;

// PID_21 — Distance traveled with MIL on (km)
pub const PID_21_MIN: f32 = 0.0;
pub const PID_21_MAX: f32 = 65535.0;

// PID_22 — Fuel rail pressure relative to manifold vacuum (kPa)
pub const PID_22_MIN: f32 = 0.0;
pub const PID_22_MAX: f32 = 5177.265;

// PID_23 — Fuel rail gauge pressure (kPa)
pub const PID_23_MIN: f32 = 0.0;
pub const PID_23_MAX: f32 = 655350.0;

// PID_2C — Commanded EGR (%)
pub const PID_2C_MIN: f32 = 0.0;
pub const PID_2C_MAX: f32 = 100.0;

// PID_2D — EGR error (%)
pub const PID_2D_MIN: f32 = -100.0;
pub const PID_2D_MAX: f32 = 99.21875;

// PID_2E — Commanded evaporative purge (%)
pub const PID_2E_MIN: f32 = 0.0;
pub const PID_2E_MAX: f32 = 100.0;

// PID_2F — Fuel tank level (%)
pub const PID_2F_MIN: f32 = 0.0;
pub const PID_2F_MAX: f32 = 100.0;

// PID_30 — Warm-ups since codes cleared
pub const PID_30_MIN: f32 = 0.0;
pub const PID_30_MAX: f32 = 255.0;

// PID_31 — Distance since codes cleared (km)
pub const PID_31_MIN: f32 = 0.0;
pub const PID_31_MAX: f32 = 65535.0;

// PID_32 — Evaporative system vapor pressure (Pa)
pub const PID_32_MIN: f32 = -8192.0;
pub const PID_32_MAX: f32 = 8191.75;

// PID_33 — Absolute barometric pressure (kPa)
pub const PID_33_MIN: f32 = 0.0;
pub const PID_33_MAX: f32 = 255.0;

// PID_3C..PID_3F — Catalyst temperature (°C)
pub const CATALYST_TEMP_MIN: f32 = -40.0;
pub const CATALYST_TEMP_MAX: f32 = 6513.5;

// PID_42 — Control module voltage (V)
pub const PID_42_MIN: f32 = 0.0;
pub const PID_42_MAX: f32 = 65.535;

// PID_43 — Absolute load (%)
pub const PID_43_MIN: f32 = 0.0;
pub const PID_43_MAX: f32 = 25700.0;

// PID_44 — Commanded air-fuel equivalence ratio
pub const PID_44_MIN: f32 = 0.0;
pub const PID_44_MAX: f32 = 1.9999695;

// PID_45 — Relative throttle position (%)
pub const PID_45_MIN: f32 = 0.0;
pub const PID_45_MAX: f32 = 100.0;

// PID_46 — Ambient air temperature (°C)
pub const PID_46_MIN: f32 = -40.0;
pub const PID_46_MAX: f32 = 215.0;

// PID_47..PID_4C — Throttle / pedal position (%)
pub const THROTTLE_POSITION_MIN: f32 = 0.0;
pub const THROTTLE_POSITION_MAX: f32 = 100.0;

// PID_4D — Time with MIL on (min)
pub const PID_4D_MIN: f32 = 0.0;
pub const PID_4D_MAX: f32 = 65535.0;

// PID_4E — Time since trouble codes cleared (min)
pub const PID_4E_MIN: f32 = 0.0;
pub const PID_4E_MAX: f32 = 65535.0;

// PID_52 — Ethanol fuel percentage (%)
pub const PID_52_MIN: f32 = 0.0;
pub const PID_52_MAX: f32 = 100.0;

// PID_53 — Absolute evaporative system vapor pressure (kPa)
pub const PID_53_MIN: f32 = 0.0;
pub const PID_53_MAX: f32 = 327.675;

// PID_54 — Evaporative system vapor pressure (Pa)
pub const PID_54_MIN: f32 = -32768.0;
pub const PID_54_MAX: f32 = 32767.0;

// PID_55..PID_58 — Secondary oxygen sensor trim (%)
pub const SECONDARY_OXYGEN_TRIM_MIN: f32 = -100.0;
pub const SECONDARY_OXYGEN_TRIM_MAX: f32 = 99.21875;

// PID_59 — Fuel rail absolute pressure (kPa)
pub const PID_59_MIN: f32 = 0.0;
pub const PID_59_MAX: f32 = 655350.0;

// PID_5A — Relative accelerator pedal position (%)
pub const PID_5A_MIN: f32 = 0.0;
pub const PID_5A_MAX: f32 = 100.0;

// PID_5B — Hybrid battery remaining life (%)
pub const PID_5B_MIN: f32 = 0.0;
pub const PID_5B_MAX: f32 = 100.0;

// PID_5C — Engine oil temperature (°C)
pub const PID_5C_MIN: f32 = -40.0;
pub const PID_5C_MAX: f32 = 215.0;

// PID_5D — Fuel injection timing (degrees)
pub const PID_5D_MIN: f32 = -210.0;
pub const PID_5D_MAX: f32 = 301.9921875;

// PID_5E — Engine fuel rate (L/h)
pub const PID_5E_MIN: f32 = 0.0;
pub const PID_5E_MAX: f32 = 3276.75;
