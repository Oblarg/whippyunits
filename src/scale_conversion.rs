use crate::quantity_type::Quantity;
use crate::constants::*;

// ============================================================================
// Individual Scale Conversion Factor Functions
// ============================================================================

/// Convert between Length units
pub const fn length_conversion_factor(p10_from: isize, p10_to: isize, exponent: isize) -> f64 {
    match exponent {
        0 => 1.0_f64,  // dimension exponent is 0, no conversion needed
        1 => pow10(p10_from - p10_to),
        _ => pow10((p10_from - p10_to) * exponent),
    }
}

/// Convert between Mass units
pub const fn mass_conversion_factor(p10_from: isize, p10_to: isize, exponent: isize) -> f64 {
    match exponent {
        0 => 1.0_f64,  // dimension exponent is 0, no conversion needed
        1 => pow10(p10_from - p10_to),
        _ => pow10((p10_from - p10_to) * exponent),
    }
}

/// Convert between Time units
pub const fn time_conversion_factor(
    p2_from: isize,
    p3_from: isize,
    p5_from: isize,
    p2_to: isize,
    p3_to: isize,
    p5_to: isize,
    exponent: isize,
) -> f64 {
    match exponent {
        0 => 1.0_f64,  // dimension exponent is 0, no conversion needed
        1 => pow2(p2_from - p2_to) * pow3(p3_from - p3_to) * pow5(p5_from - p5_to),
        _ => {
            let diff_p2: isize = (p2_from - p2_to) * exponent;
            let diff_p3: isize = (p3_from - p3_to) * exponent;
            let diff_p5: isize = (p5_from - p5_to) * exponent;
            pow2(diff_p2) * pow3(diff_p3) * pow5(diff_p5)
        }
    }
}

// ============================================================================
// Aggregate Scale Conversion Factor Function
// ============================================================================

// Calculate the aggregate conversion factor multiplied over all logarithmic scales
// Like-base logarithms are added together prior to LUT exponentiation for accuracy/efficiency
const fn aggregate_conversion_factor(
    mass_exponent: isize, mass_scale_p10_from: isize, mass_scale_p10_to: isize,
    length_exponent: isize, length_scale_p10_from: isize, length_scale_p10_to: isize,
    time_exponent: isize, time_scale_p2_from: isize, time_scale_p3_from: isize, time_scale_p5_from: isize, time_scale_p2_to: isize, time_scale_p3_to: isize, time_scale_p5_to: isize,
) -> f64 {
    let diff_length_p10: isize = (length_scale_p10_from - length_scale_p10_to) * length_exponent;
    let diff_mass_p10: isize = (mass_scale_p10_from - mass_scale_p10_to) * mass_exponent;
    let diff_time_p2: isize = (time_scale_p2_from - time_scale_p2_to) * time_exponent;
    let diff_time_p3: isize = (time_scale_p3_from - time_scale_p3_to) * time_exponent;
    let diff_time_p5: isize = (time_scale_p5_from - time_scale_p5_to) * time_exponent;
    pow10(diff_length_p10 + diff_mass_p10) * pow2(diff_time_p2) * pow3(diff_time_p3) * pow5(diff_time_p5)
}


// ============================================================================
// Rescale Function
// ============================================================================

pub fn rescale<
    const MASS_EXPONENT: isize, const MASS_SCALE_P10_FROM: isize, const MASS_SCALE_P10_TO: isize,
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10_FROM: isize, const LENGTH_SCALE_P10_TO: isize,
    const TIME_EXPONENT: isize, const TIME_SCALE_P2_FROM: isize, const TIME_SCALE_P3_FROM: isize, const TIME_SCALE_P5_FROM: isize,
                                const TIME_SCALE_P2_TO: isize, const TIME_SCALE_P3_TO: isize, const TIME_SCALE_P5_TO: isize,
> (
    quantity: Quantity<
        MASS_EXPONENT, MASS_SCALE_P10_FROM,
        LENGTH_EXPONENT, LENGTH_SCALE_P10_FROM,
        TIME_EXPONENT, TIME_SCALE_P2_FROM, TIME_SCALE_P3_FROM, TIME_SCALE_P5_FROM,
    >,
) -> Quantity<
    MASS_EXPONENT, MASS_SCALE_P10_TO,
    LENGTH_EXPONENT, LENGTH_SCALE_P10_TO,
    TIME_EXPONENT, TIME_SCALE_P2_TO, TIME_SCALE_P3_TO, TIME_SCALE_P5_TO,
> {
    Quantity::new(
        aggregate_conversion_factor(
            MASS_EXPONENT, MASS_SCALE_P10_FROM, MASS_SCALE_P10_TO,
            LENGTH_EXPONENT, LENGTH_SCALE_P10_FROM, LENGTH_SCALE_P10_TO,
            TIME_EXPONENT, TIME_SCALE_P2_FROM, TIME_SCALE_P3_FROM, TIME_SCALE_P5_FROM, 
                            TIME_SCALE_P2_TO, TIME_SCALE_P3_TO, TIME_SCALE_P5_TO,
        ) * quantity.value
    )
}
