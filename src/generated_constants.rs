// Auto-generated scale constants and helper functions
// Generated from dimensional_metadata.rs
// DO NOT EDIT - This file is auto-generated

// ============================================================================
// Length Scale Constants (Auto-generated)
// ============================================================================

pub const MILLIMETER_SCALE: isize = -1;
pub const METER_SCALE: isize = 0;
pub const KILOMETER_SCALE: isize = 1;
pub const LENGTH_UNUSED: isize = isize::MAX;

// ============================================================================
// Mass Scale Constants (Auto-generated)
// ============================================================================

pub const MILLIGRAM_SCALE: isize = -1;
pub const GRAM_SCALE: isize = 0;
pub const KILOGRAM_SCALE: isize = 1;
pub const MASS_UNUSED: isize = isize::MAX;

// ============================================================================
// Time Scale Constants (Auto-generated)
// ============================================================================

pub const MILLISECOND_SCALE_ORDER: isize = -1;
pub const MILLISECOND_SCALE_P2: isize = -3;
pub const MILLISECOND_SCALE_P3: isize = 0;
pub const MILLISECOND_SCALE_P5: isize = -3;

pub const SECOND_SCALE_ORDER: isize = 0;
pub const SECOND_SCALE_P2: isize = 0;
pub const SECOND_SCALE_P3: isize = 0;
pub const SECOND_SCALE_P5: isize = 0;

pub const MINUTE_SCALE_ORDER: isize = 1;
pub const MINUTE_SCALE_P2: isize = 2;
pub const MINUTE_SCALE_P3: isize = 1;
pub const MINUTE_SCALE_P5: isize = 1;

pub const TIME_UNUSED: isize = isize::MAX;

// ============================================================================
// Time Scale Helper Functions (Auto-generated)
// ============================================================================

pub const fn time_scale_2(scale_order: isize) -> isize {
    match scale_order {
        -1 => -3,
        0 => 0,
        1 => 2,
        _ => isize::MAX,
    }
}

pub const fn time_scale_3(scale_order: isize) -> isize {
    match scale_order {
        -1 => 0,
        0 => 0,
        1 => 1,
        _ => isize::MAX,
    }
}

pub const fn time_scale_5(scale_order: isize) -> isize {
    match scale_order {
        -1 => -3,
        0 => 0,
        1 => 1,
        _ => isize::MAX,
    }
}

// ============================================================================
// Power Functions (Auto-generated)
// ============================================================================

const fn pow1000(exp: isize) -> f64 {
    match exp {
        -3 => 0.000000001_f64,
        -2 => 0.000001_f64,
        -1 => 0.001_f64,
        0 => 1.0_f64,
        1 => 1000.0_f64,
        2 => 1000000.0_f64,
        3 => 1000000000.0_f64,
        _ => 1.0_f64, // we'll only test small values during prototyping
    }
}

pub const fn pow2(exp: isize) -> f64 {
    match exp {
        -3 => 0.125_f64,
        -2 => 0.25_f64,
        -1 => 0.5_f64,
        0 => 1.0_f64,
        1 => 2.0_f64,
        2 => 4.0_f64,
        3 => 8.0_f64,
        _ => 1.0_f64, // we'll only test small values during prototyping
    }
}

pub const fn pow3(exp: isize) -> f64 {
    match exp {
        -3 => 0.037037037037037035_f64,
        -2 => 0.1111111111111111_f64,
        -1 => 0.3333333333333333_f64,
        0 => 1.0_f64,
        1 => 3.0_f64,
        2 => 9.0_f64,
        3 => 27.0_f64,
        _ => 1.0_f64, // we'll only test small values during prototyping
    }
}

pub const fn pow5(exp: isize) -> f64 {
    match exp {
        -3 => 0.008_f64,
        -2 => 0.04_f64,
        -1 => 0.2_f64,
        0 => 1.0_f64,
        1 => 5.0_f64,
        2 => 25.0_f64,
        3 => 125.0_f64,
        _ => 1.0_f64, // we'll only test small values during prototyping
    }
}

// ============================================================================
// Generic Conversion Functions (Auto-generated)
// ============================================================================

// ============================================================================
// Dimension-Specific Conversion Functions (Auto-generated)
// ============================================================================

/// Convert between Length units
pub const fn length_conversion_factor(from: isize, to: isize, exponent: isize) -> f64 {
    let diff: isize = (from - to) * exponent;
    const UNUSED: isize = LENGTH_UNUSED;
    match (from, to) {
        (UNUSED, _) | (_, UNUSED) => 1.0_f64,
        _ => pow1000(diff),
    }
}

/// Convert between Mass units
pub const fn mass_conversion_factor(from: isize, to: isize, exponent: isize) -> f64 {
    let diff: isize = (from - to) * exponent;
    const UNUSED: isize = MASS_UNUSED;
    match (from, to) {
        (UNUSED, _) | (_, UNUSED) => 1.0_f64,
        _ => pow1000(diff),
    }
}

/// Convert between Time units
pub const fn time_conversion_factor(
    from_p2: isize,
    from_p3: isize,
    from_p5: isize,
    to_p2: isize,
    to_p3: isize,
    to_p5: isize,
    exponent: isize,
) -> f64 {
    let diff_p2: isize = (from_p2 - to_p2) * exponent;
    let diff_p3: isize = (from_p3 - to_p3) * exponent;
    let diff_p5: isize = (from_p5 - to_p5) * exponent;
    const UNUSED: isize = TIME_UNUSED;
    match (from_p2, to_p2, from_p3, to_p3, from_p5, to_p5) {
        (UNUSED, _, _, _, _, _) => 1.0_f64,
        (_, UNUSED, _, _, _, _) => 1.0_f64,
        (_, _, UNUSED, _, _, _) => 1.0_f64,
        (_, _, _, UNUSED, _, _) => 1.0_f64,
        (_, _, _, _, UNUSED, _) => 1.0_f64,
        (_, _, _, _, _, UNUSED) => 1.0_f64,
        _ => pow2(diff_p2) * pow3(diff_p3) * pow5(diff_p5),
    }
}

// ============================================================================
// Display Helper Functions (Auto-generated)
// ============================================================================

pub const fn length_short_name(scale: isize) -> &'static str {
    match scale {
        -1 => "mm",
        0 => "m",
        1 => "km",
        _ => "unknown",
    }
}

pub const fn length_long_name(scale: isize) -> &'static str {
    match scale {
        -1 => "millimeter",
        0 => "meter",
        1 => "kilometer",
        _ => "unknown",
    }
}

pub const fn mass_short_name(scale: isize) -> &'static str {
    match scale {
        -1 => "mg",
        0 => "g",
        1 => "kg",
        _ => "unknown",
    }
}

pub const fn mass_long_name(scale: isize) -> &'static str {
    match scale {
        -1 => "milligram",
        0 => "gram",
        1 => "kilogram",
        _ => "unknown",
    }
}

pub const fn time_short_name(scale_order: isize) -> &'static str {
    match scale_order {
        -1 => "ms",
        0 => "s",
        1 => "min",
        _ => "unknown",
    }
}

pub const fn time_long_name(scale_order: isize) -> &'static str {
    match scale_order {
        -1 => "millisecond",
        0 => "second",
        1 => "minute",
        _ => "unknown",
    }
}

