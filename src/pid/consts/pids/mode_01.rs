// Supported PIDs [01-20]
pub const PID_00: u8 = 0x00;

// Monitor status since DTCs cleared
pub const PID_01: u8 = 0x01;

// Freeze DTC (Mode 02 only)
pub const PID_02: u8 = 0x02;

// Fuel system status
pub const PID_03: u8 = 0x03;

// Calculated engine load
pub const PID_04: u8 = 0x04;

// Engine coolant temperature
pub const PID_05: u8 = 0x05;

// Short-term fuel trim — Bank 1
pub const PID_06: u8 = 0x06;

// Long-term fuel trim — Bank 1
pub const PID_07: u8 = 0x07;

// Short-term fuel trim — Bank 2
pub const PID_08: u8 = 0x08;

// Long-term fuel trim — Bank 2
pub const PID_09: u8 = 0x09;

// Fuel pressure
pub const PID_0A: u8 = 0x0A;

// Intake manifold absolute pressure
pub const PID_0B: u8 = 0x0B;

// Engine RPM
pub const PID_0C: u8 = 0x0C;

// Vehicle speed
pub const PID_0D: u8 = 0x0D;

// Timing advance
pub const PID_0E: u8 = 0x0E;

// Intake air temperature
pub const PID_0F: u8 = 0x0F;

// Mass air flow rate
pub const PID_10: u8 = 0x10;

// Throttle position
pub const PID_11: u8 = 0x11;

// Commanded secondary air status
pub const PID_12: u8 = 0x12;

// Oxygen sensors present (2 banks)
pub const PID_13: u8 = 0x13;

// Oxygen sensor 1 — voltage / short-term fuel trim
pub const PID_14: u8 = 0x14;

// Oxygen sensor 2 — voltage / short-term fuel trim
pub const PID_15: u8 = 0x15;

// Oxygen sensor 3 — voltage / short-term fuel trim
pub const PID_16: u8 = 0x16;

// Oxygen sensor 4 — voltage / short-term fuel trim
pub const PID_17: u8 = 0x17;

// Oxygen sensor 5 — voltage / short-term fuel trim
pub const PID_18: u8 = 0x18;

// Oxygen sensor 6 — voltage / short-term fuel trim
pub const PID_19: u8 = 0x19;

// Oxygen sensor 7 — voltage / short-term fuel trim
pub const PID_1A: u8 = 0x1A;

// Oxygen sensor 8 — voltage / short-term fuel trim
pub const PID_1B: u8 = 0x1B;

// OBD standards supported by the vehicle
pub const PID_1C: u8 = 0x1C;

// Oxygen sensors present (4 banks)
pub const PID_1D: u8 = 0x1D;

// Auxiliary input status
pub const PID_1E: u8 = 0x1E;

// Run time since engine start
pub const PID_1F: u8 = 0x1F;

// Supported PIDs [21-40]
pub const PID_20: u8 = 0x20;

// Distance traveled with MIL on
pub const PID_21: u8 = 0x21;

// Fuel rail pressure relative to manifold vacuum
pub const PID_22: u8 = 0x22;

// Fuel rail gauge pressure (diesel / direct injection)
pub const PID_23: u8 = 0x23;

// Oxygen sensor 1 — wide-range lambda / voltage
pub const PID_24: u8 = 0x24;

// Oxygen sensor 2 — wide-range lambda / voltage
pub const PID_25: u8 = 0x25;

// Oxygen sensor 3 — wide-range lambda / voltage
pub const PID_26: u8 = 0x26;

// Oxygen sensor 4 — wide-range lambda / voltage
pub const PID_27: u8 = 0x27;

// Oxygen sensor 5 — wide-range lambda / voltage
pub const PID_28: u8 = 0x28;

// Oxygen sensor 6 — wide-range lambda / voltage
pub const PID_29: u8 = 0x29;

// Oxygen sensor 7 — wide-range lambda / voltage
pub const PID_2A: u8 = 0x2A;

// Oxygen sensor 8 — wide-range lambda / voltage
pub const PID_2B: u8 = 0x2B;

// Commanded EGR
pub const PID_2C: u8 = 0x2C;

// EGR error
pub const PID_2D: u8 = 0x2D;

// Commanded evaporative purge
pub const PID_2E: u8 = 0x2E;

// Fuel tank level input
pub const PID_2F: u8 = 0x2F;

// Warm-ups since codes cleared
pub const PID_30: u8 = 0x30;

// Distance traveled since codes cleared
pub const PID_31: u8 = 0x31;

// Evaporative system vapor pressure
pub const PID_32: u8 = 0x32;

// Absolute barometric pressure
pub const PID_33: u8 = 0x33;

// Oxygen sensor 1 — wide-range lambda / current
pub const PID_34: u8 = 0x34;

// Oxygen sensor 2 — wide-range lambda / current
pub const PID_35: u8 = 0x35;

// Oxygen sensor 3 — wide-range lambda / current
pub const PID_36: u8 = 0x36;

// Oxygen sensor 4 — wide-range lambda / current
pub const PID_37: u8 = 0x37;

// Oxygen sensor 5 — wide-range lambda / current
pub const PID_38: u8 = 0x38;

// Oxygen sensor 6 — wide-range lambda / current
pub const PID_39: u8 = 0x39;

// Oxygen sensor 7 — wide-range lambda / current
pub const PID_3A: u8 = 0x3A;

// Oxygen sensor 8 — wide-range lambda / current
pub const PID_3B: u8 = 0x3B;

// Catalyst temperature — Bank 1, Sensor 1
pub const PID_3C: u8 = 0x3C;

// Catalyst temperature — Bank 2, Sensor 1
pub const PID_3D: u8 = 0x3D;

// Catalyst temperature — Bank 1, Sensor 2
pub const PID_3E: u8 = 0x3E;

// Catalyst temperature — Bank 2, Sensor 2
pub const PID_3F: u8 = 0x3F;

// Supported PIDs [41-60]
pub const PID_40: u8 = 0x40;

// Monitor status this drive cycle
pub const PID_41: u8 = 0x41;

// Control module voltage
pub const PID_42: u8 = 0x42;

// Absolute load value
pub const PID_43: u8 = 0x43;

// Commanded air-fuel equivalence ratio
pub const PID_44: u8 = 0x44;

// Relative throttle position
pub const PID_45: u8 = 0x45;

// Ambient air temperature
pub const PID_46: u8 = 0x46;

// Absolute throttle position B
pub const PID_47: u8 = 0x47;

// Absolute throttle position C
pub const PID_48: u8 = 0x48;

// Accelerator pedal position D
pub const PID_49: u8 = 0x49;

// Accelerator pedal position E
pub const PID_4A: u8 = 0x4A;

// Accelerator pedal position F
pub const PID_4B: u8 = 0x4B;

// Commanded throttle actuator
pub const PID_4C: u8 = 0x4C;

// Time run with MIL on
pub const PID_4D: u8 = 0x4D;

// Time since trouble codes cleared
pub const PID_4E: u8 = 0x4E;

// Maximum values: lambda, O2 voltage/current, MAP
pub const PID_4F: u8 = 0x4F;

// Maximum MAF air flow rate
pub const PID_50: u8 = 0x50;

// Fuel type
pub const PID_51: u8 = 0x51;

// Ethanol fuel percentage
pub const PID_52: u8 = 0x52;

// Absolute evaporative system vapor pressure
pub const PID_53: u8 = 0x53;

// Evaporative system vapor pressure
pub const PID_54: u8 = 0x54;

// Short-term secondary oxygen sensor trim — Banks 1 / 3
pub const PID_55: u8 = 0x55;

// Long-term secondary oxygen sensor trim — Banks 1 / 3
pub const PID_56: u8 = 0x56;

// Short-term secondary oxygen sensor trim — Banks 2 / 4
pub const PID_57: u8 = 0x57;

// Long-term secondary oxygen sensor trim — Banks 2 / 4
pub const PID_58: u8 = 0x58;

// Fuel rail absolute pressure
pub const PID_59: u8 = 0x59;

// Relative accelerator pedal position
pub const PID_5A: u8 = 0x5A;

// Hybrid / EV battery pack remaining life
pub const PID_5B: u8 = 0x5B;

// Engine oil temperature
pub const PID_5C: u8 = 0x5C;

// Fuel injection timing
pub const PID_5D: u8 = 0x5D;

// Engine fuel rate
pub const PID_5E: u8 = 0x5E;

// Emission requirements to which vehicle is designed
pub const PID_5F: u8 = 0x5F;

// Supported PIDs [61-80]
pub const PID_60: u8 = 0x60;

// Driver's demand engine torque — percentage
pub const PID_61: u8 = 0x61;

// Actual engine torque — percentage
pub const PID_62: u8 = 0x62;

// Engine reference torque
pub const PID_63: u8 = 0x63;

// Engine percent torque data
pub const PID_64: u8 = 0x64;

// Auxiliary input / output supported
pub const PID_65: u8 = 0x65;

// Mass air flow sensor data
pub const PID_66: u8 = 0x66;

// Engine coolant temperature — two sensors
pub const PID_67: u8 = 0x67;

// Intake air temperature — two sensors
pub const PID_68: u8 = 0x68;

// Actual EGR, commanded EGR, and EGR error
pub const PID_69: u8 = 0x69;

// Commanded diesel intake air flow control and position
pub const PID_6A: u8 = 0x6A;

// Exhaust gas recirculation temperature
pub const PID_6B: u8 = 0x6B;

// Commanded throttle actuator control and relative position
pub const PID_6C: u8 = 0x6C;

// Fuel pressure control system
pub const PID_6D: u8 = 0x6D;

// Injection pressure control system
pub const PID_6E: u8 = 0x6E;

// Turbocharger compressor inlet pressure
pub const PID_6F: u8 = 0x6F;

// Boost pressure control
pub const PID_70: u8 = 0x70;

// Variable geometry turbocharger control
pub const PID_71: u8 = 0x71;

// Wastegate control
pub const PID_72: u8 = 0x72;

// Exhaust pressure
pub const PID_73: u8 = 0x73;

// Turbocharger RPM
pub const PID_74: u8 = 0x74;

// Turbocharger temperature — first set
pub const PID_75: u8 = 0x75;

// Turbocharger temperature — second set
pub const PID_76: u8 = 0x76;

// Charge air cooler temperature
pub const PID_77: u8 = 0x77;

// Exhaust gas temperature — Bank 1
pub const PID_78: u8 = 0x78;

// Exhaust gas temperature — Bank 2
pub const PID_79: u8 = 0x79;

// Diesel particulate filter differential pressure
pub const PID_7A: u8 = 0x7A;

// Diesel particulate filter status
pub const PID_7B: u8 = 0x7B;

// Diesel particulate filter temperature
pub const PID_7C: u8 = 0x7C;

// NOx NTE control area status
pub const PID_7D: u8 = 0x7D;

// PM NTE control area status
pub const PID_7E: u8 = 0x7E;

// Engine run time
pub const PID_7F: u8 = 0x7F;

// Supported PIDs [81-A0]
pub const PID_80: u8 = 0x80;

// Engine run time for AECD #1-#5
pub const PID_81: u8 = 0x81;

// Engine run time for AECD #6-#10
pub const PID_82: u8 = 0x82;

// NOx sensor
pub const PID_83: u8 = 0x83;

// Manifold surface temperature
pub const PID_84: u8 = 0x84;

// NOx reagent system
pub const PID_85: u8 = 0x85;

// Particulate matter sensor
pub const PID_86: u8 = 0x86;

// Intake manifold absolute pressure — extended
pub const PID_87: u8 = 0x87;

// SCR inducement system
pub const PID_88: u8 = 0x88;

// Engine run time for AECD #11-#15
pub const PID_89: u8 = 0x89;

// Engine run time for AECD #16-#20
pub const PID_8A: u8 = 0x8A;

// Diesel aftertreatment status
pub const PID_8B: u8 = 0x8B;

// Wide-range oxygen sensor data
pub const PID_8C: u8 = 0x8C;

// Throttle position G
pub const PID_8D: u8 = 0x8D;

// Engine friction — percentage torque
pub const PID_8E: u8 = 0x8E;

// PM sensor — Banks 1 and 2
pub const PID_8F: u8 = 0x8F;

// WWH-OBD vehicle OBD system information
pub const PID_90: u8 = 0x90;

// WWH-OBD vehicle OBD system information — extended
pub const PID_91: u8 = 0x91;

// Fuel system control
pub const PID_92: u8 = 0x92;

// WWH-OBD vehicle OBD counters support
pub const PID_93: u8 = 0x93;

// NOx warning and inducement system
pub const PID_94: u8 = 0x94;

// Reserved / no commonly defined generic PID
pub const PID_95: u8 = 0x95;

// Reserved / no commonly defined generic PID
pub const PID_96: u8 = 0x96;

// Reserved / no commonly defined generic PID
pub const PID_97: u8 = 0x97;

// Exhaust gas temperature sensor
pub const PID_98: u8 = 0x98;

// Exhaust gas temperature sensor — second set
pub const PID_99: u8 = 0x99;

// Hybrid / EV vehicle system data — battery voltage
pub const PID_9A: u8 = 0x9A;

// Diesel exhaust fluid sensor data
pub const PID_9B: u8 = 0x9B;

// Oxygen sensor data — extended
pub const PID_9C: u8 = 0x9C;

// Engine fuel rate — mass
pub const PID_9D: u8 = 0x9D;

// Engine exhaust flow rate
pub const PID_9E: u8 = 0x9E;

// Fuel system percentage use
pub const PID_9F: u8 = 0x9F;

// Supported PIDs [A1-C0]
pub const PID_A0: u8 = 0xA0;

// NOx sensor corrected data
pub const PID_A1: u8 = 0xA1;

// Cylinder fuel rate
pub const PID_A2: u8 = 0xA2;

// Evaporative system vapor pressure — extended
pub const PID_A3: u8 = 0xA3;

// Transmission actual gear
pub const PID_A4: u8 = 0xA4;

// Commanded diesel exhaust fluid dosing
pub const PID_A5: u8 = 0xA5;

// Odometer
pub const PID_A6: u8 = 0xA6;

// NOx sensor concentration — sensors 3 and 4
pub const PID_A7: u8 = 0xA7;

// NOx sensor corrected concentration — sensors 3 and 4
pub const PID_A8: u8 = 0xA8;

// ABS disable switch state
pub const PID_A9: u8 = 0xA9;

// Reserved / no commonly defined generic PID
pub const PID_AA: u8 = 0xAA;

// Reserved / no commonly defined generic PID
pub const PID_AB: u8 = 0xAB;

// Reserved / no commonly defined generic PID
pub const PID_AC: u8 = 0xAC;

// Reserved / no commonly defined generic PID
pub const PID_AD: u8 = 0xAD;

// Reserved / no commonly defined generic PID
pub const PID_AE: u8 = 0xAE;

// Reserved / no commonly defined generic PID
pub const PID_AF: u8 = 0xAF;

// Reserved / no commonly defined generic PID
pub const PID_B0: u8 = 0xB0;

// Reserved / no commonly defined generic PID
pub const PID_B1: u8 = 0xB1;

// Reserved / no commonly defined generic PID
pub const PID_B2: u8 = 0xB2;

// Reserved / no commonly defined generic PID
pub const PID_B3: u8 = 0xB3;

// Reserved / no commonly defined generic PID
pub const PID_B4: u8 = 0xB4;

// Reserved / no commonly defined generic PID
pub const PID_B5: u8 = 0xB5;

// Reserved / no commonly defined generic PID
pub const PID_B6: u8 = 0xB6;

// Reserved / no commonly defined generic PID
pub const PID_B7: u8 = 0xB7;

// Reserved / no commonly defined generic PID
pub const PID_B8: u8 = 0xB8;

// Reserved / no commonly defined generic PID
pub const PID_B9: u8 = 0xB9;

// Reserved / no commonly defined generic PID
pub const PID_BA: u8 = 0xBA;

// Reserved / no commonly defined generic PID
pub const PID_BB: u8 = 0xBB;

// Reserved / no commonly defined generic PID
pub const PID_BC: u8 = 0xBC;

// Reserved / no commonly defined generic PID
pub const PID_BD: u8 = 0xBD;

// Reserved / no commonly defined generic PID
pub const PID_BE: u8 = 0xBE;

// Reserved / no commonly defined generic PID
pub const PID_BF: u8 = 0xBF;

// Supported PIDs [C1-E0]
pub const PID_C0: u8 = 0xC0;

// Reserved / no commonly defined generic PID
pub const PID_C1: u8 = 0xC1;

// Reserved / no commonly defined generic PID
pub const PID_C2: u8 = 0xC2;

// Fuel level input A/B
pub const PID_C3: u8 = 0xC3;

// Exhaust particulate control system diagnostic time/count
pub const PID_C4: u8 = 0xC4;

// Fuel pressure A and B
pub const PID_C5: u8 = 0xC5;

// Particulate control driver inducement counters
pub const PID_C6: u8 = 0xC6;

// Distance since reflash or module replacement
pub const PID_C7: u8 = 0xC7;

// NOx / particulate control diagnostic warning lamp status
pub const PID_C8: u8 = 0xC8;

// Reserved / no commonly defined generic PID
pub const PID_C9: u8 = 0xC9;

// Reserved / no commonly defined generic PID
pub const PID_CA: u8 = 0xCA;

// Reserved / no commonly defined generic PID
pub const PID_CB: u8 = 0xCB;

// Reserved / no commonly defined generic PID
pub const PID_CC: u8 = 0xCC;

// Reserved / no commonly defined generic PID
pub const PID_CD: u8 = 0xCD;

// Reserved / no commonly defined generic PID
pub const PID_CE: u8 = 0xCE;

// Reserved / no commonly defined generic PID
pub const PID_CF: u8 = 0xCF;

// Reserved / no commonly defined generic PID
pub const PID_D0: u8 = 0xD0;

// Reserved / no commonly defined generic PID
pub const PID_D1: u8 = 0xD1;

// Reserved / no commonly defined generic PID
pub const PID_D2: u8 = 0xD2;

// Reserved / no commonly defined generic PID
pub const PID_D3: u8 = 0xD3;

// Reserved / no commonly defined generic PID
pub const PID_D4: u8 = 0xD4;

// Reserved / no commonly defined generic PID
pub const PID_D5: u8 = 0xD5;

// Reserved / no commonly defined generic PID
pub const PID_D6: u8 = 0xD6;

// Reserved / no commonly defined generic PID
pub const PID_D7: u8 = 0xD7;

// Reserved / no commonly defined generic PID
pub const PID_D8: u8 = 0xD8;

// Reserved / no commonly defined generic PID
pub const PID_D9: u8 = 0xD9;

// Reserved / no commonly defined generic PID
pub const PID_DA: u8 = 0xDA;

// Reserved / no commonly defined generic PID
pub const PID_DB: u8 = 0xDB;

// Reserved / no commonly defined generic PID
pub const PID_DC: u8 = 0xDC;

// Reserved / no commonly defined generic PID
pub const PID_DD: u8 = 0xDD;

// Reserved / no commonly defined generic PID
pub const PID_DE: u8 = 0xDE;

// Reserved / no commonly defined generic PID
pub const PID_DF: u8 = 0xDF;

// Supported PIDs [E1-FF]
pub const PID_E0: u8 = 0xE0;

// Reserved / no commonly defined generic PID
pub const PID_E1: u8 = 0xE1;

// Reserved / no commonly defined generic PID
pub const PID_E2: u8 = 0xE2;

// Reserved / no commonly defined generic PID
pub const PID_E3: u8 = 0xE3;

// Reserved / no commonly defined generic PID
pub const PID_E4: u8 = 0xE4;

// Reserved / no commonly defined generic PID
pub const PID_E5: u8 = 0xE5;

// Reserved / no commonly defined generic PID
pub const PID_E6: u8 = 0xE6;

// Reserved / no commonly defined generic PID
pub const PID_E7: u8 = 0xE7;

// Reserved / no commonly defined generic PID
pub const PID_E8: u8 = 0xE8;

// Reserved / no commonly defined generic PID
pub const PID_E9: u8 = 0xE9;

// Reserved / no commonly defined generic PID
pub const PID_EA: u8 = 0xEA;

// Reserved / no commonly defined generic PID
pub const PID_EB: u8 = 0xEB;

// Reserved / no commonly defined generic PID
pub const PID_EC: u8 = 0xEC;

// Reserved / no commonly defined generic PID
pub const PID_ED: u8 = 0xED;

// Reserved / no commonly defined generic PID
pub const PID_EE: u8 = 0xEE;

// Reserved / no commonly defined generic PID
pub const PID_EF: u8 = 0xEF;

// Reserved / no commonly defined generic PID
pub const PID_F0: u8 = 0xF0;

// Reserved / no commonly defined generic PID
pub const PID_F1: u8 = 0xF1;

// Reserved / no commonly defined generic PID
pub const PID_F2: u8 = 0xF2;

// Reserved / no commonly defined generic PID
pub const PID_F3: u8 = 0xF3;

// Reserved / no commonly defined generic PID
pub const PID_F4: u8 = 0xF4;

// Reserved / no commonly defined generic PID
pub const PID_F5: u8 = 0xF5;

// Reserved / no commonly defined generic PID
pub const PID_F6: u8 = 0xF6;

// Reserved / no commonly defined generic PID
pub const PID_F7: u8 = 0xF7;

// Reserved / no commonly defined generic PID
pub const PID_F8: u8 = 0xF8;

// Reserved / no commonly defined generic PID
pub const PID_F9: u8 = 0xF9;

// Reserved / no commonly defined generic PID
pub const PID_FA: u8 = 0xFA;

// Reserved / no commonly defined generic PID
pub const PID_FB: u8 = 0xFB;

// Reserved / no commonly defined generic PID
pub const PID_FC: u8 = 0xFC;

// Reserved / no commonly defined generic PID
pub const PID_FD: u8 = 0xFD;

// Reserved / no commonly defined generic PID
pub const PID_FE: u8 = 0xFE;

// Reserved / no commonly defined generic PID
pub const PID_FF: u8 = 0xFF;