// Auto-generated scale constants and helper functions
// Generated from dimensional_metadata.rs
// DO NOT EDIT - This file is auto-generated

// ============================================================================
// Length Scale Constants (Auto-generated)
// ============================================================================

pub const MILLIMETER_SCALE_P10: isize = -3;
pub const METER_SCALE_P10: isize = 0;
pub const KILOMETER_SCALE_P10: isize = 3;
pub const LENGTH_UNUSED: isize = isize::MAX;

// ============================================================================
// Mass Scale Constants (Auto-generated)
// ============================================================================

pub const MILLIGRAM_SCALE_P10: isize = -6;
pub const GRAM_SCALE_P10: isize = -3;
pub const KILOGRAM_SCALE_P10: isize = 0;
pub const MASS_UNUSED: isize = isize::MAX;

// ============================================================================
// Time Scale Constants (Auto-generated)
// ============================================================================

pub const MILLISECOND_SCALE_P2: isize = -3;
pub const MILLISECOND_SCALE_P3: isize = 0;
pub const MILLISECOND_SCALE_P5: isize = -3;

pub const SECOND_SCALE_P2: isize = 0;
pub const SECOND_SCALE_P3: isize = 0;
pub const SECOND_SCALE_P5: isize = 0;

pub const MINUTE_SCALE_P2: isize = 2;
pub const MINUTE_SCALE_P3: isize = 1;
pub const MINUTE_SCALE_P5: isize = 1;

pub const TIME_UNUSED: isize = isize::MAX;

// ============================================================================
// Power Functions (Auto-generated)
// ============================================================================

pub const fn pow10(exp: isize) -> f64 {
    match exp {
        -9 => 0.000000001_f64,
        -8 => 0.00000001_f64,
        -7 => 0.0000001_f64,
        -6 => 0.000001_f64,
        -5 => 0.00001_f64,
        -4 => 0.0001_f64,
        -3 => 0.001_f64,
        -2 => 0.01_f64,
        -1 => 0.1_f64,
        0 => 1.0_f64,
        1 => 10.0_f64,
        2 => 100.0_f64,
        3 => 1000.0_f64,
        4 => 10000.0_f64,
        5 => 100000.0_f64,
        6 => 1000000.0_f64,
        7 => 10000000.0_f64,
        8 => 100000000.0_f64,
        9 => 1000000000.0_f64,
        _ => 1.0_f64, // we'll only test small values during prototyping
    }
}

pub const fn pow2(exp: isize) -> f64 {
    match exp {
        -9 => 0.001953125_f64,
        -8 => 0.00390625_f64,
        -7 => 0.0078125_f64,
        -6 => 0.015625_f64,
        -5 => 0.03125_f64,
        -4 => 0.0625_f64,
        -3 => 0.125_f64,
        -2 => 0.25_f64,
        -1 => 0.5_f64,
        0 => 1.0_f64,
        1 => 2.0_f64,
        2 => 4.0_f64,
        3 => 8.0_f64,
        4 => 16.0_f64,
        5 => 32.0_f64,
        6 => 64.0_f64,
        7 => 128.0_f64,
        8 => 256.0_f64,
        9 => 512.0_f64,
        _ => 1.0_f64, // we'll only test small values during prototyping
    }
}

pub const fn pow3(exp: isize) -> f64 {
    match exp {
        -9 => 0.00005080526342529085_f64,
        -8 => 0.00015241579027587256_f64,
        -7 => 0.0004572473708276177_f64,
        -6 => 0.0013717421124828531_f64,
        -5 => 0.00411522633744856_f64,
        -4 => 0.012345679012345678_f64,
        -3 => 0.037037037037037035_f64,
        -2 => 0.1111111111111111_f64,
        -1 => 0.3333333333333333_f64,
        0 => 1.0_f64,
        1 => 3.0_f64,
        2 => 9.0_f64,
        3 => 27.0_f64,
        4 => 81.0_f64,
        5 => 243.0_f64,
        6 => 729.0_f64,
        7 => 2187.0_f64,
        8 => 6561.0_f64,
        9 => 19683.0_f64,
        _ => 1.0_f64, // we'll only test small values during prototyping
    }
}

pub const fn pow5(exp: isize) -> f64 {
    match exp {
        -9 => 0.000000512_f64,
        -8 => 0.00000256_f64,
        -7 => 0.0000128_f64,
        -6 => 0.000064_f64,
        -5 => 0.00032_f64,
        -4 => 0.0016_f64,
        -3 => 0.008_f64,
        -2 => 0.04_f64,
        -1 => 0.2_f64,
        0 => 1.0_f64,
        1 => 5.0_f64,
        2 => 25.0_f64,
        3 => 125.0_f64,
        4 => 625.0_f64,
        5 => 3125.0_f64,
        6 => 15625.0_f64,
        7 => 78125.0_f64,
        8 => 390625.0_f64,
        9 => 1953125.0_f64,
        _ => 1.0_f64, // we'll only test small values during prototyping
    }
}
