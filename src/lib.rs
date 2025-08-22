#![no_std]
#![feature(custom_inner_attributes)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![rustfmt::skip]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::f64;
use core::marker::ConstParamTy;
use core::ops::{Add, Div, Mul, Sub};

// ============================================================================
// Constants
// ============================================================================

// Bias constant to make all exponents non-negative for storage
pub const MILLIMETER_SCALE: isize = 1;
pub const METER_SCALE: isize = 0;
pub const KILOMETER_SCALE: isize = 1;
pub const LENGTH_UNUSED: isize = isize::MAX;

pub const MILLIGRAM_SCALE: isize = -1;
pub const GRAM_SCALE: isize = 0;
pub const KILOGRAM_SCALE: isize = 1;
pub const MASS_UNUSED: isize = isize::MAX;

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
// Core Types and Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum RescaleBehavior {
    SmallerWins,
    LeftHandWins,
    LargerWins,
    Strict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum CancelledScaleBehavior {
    Retain, // Keep the storage scales even when dimensions are cancelled
    Forget, // Automatically convert to Unused when exponent becomes 0
}

// ============================================================================
// Main Quantity Type
// ============================================================================

#[derive(Clone, Copy)]
pub struct Quantity<
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE: isize,
    const MASS_EXPONENT: isize, const MASS_SCALE: isize,
    const TIME_EXPONENT: isize, const TIME_P2: isize, const TIME_P3: isize, const TIME_P5: isize, const TIME_SCALE_ORDER: isize,
    const RESCALE_BEHAVIOR: RescaleBehavior, const CANCELLED_SCALE_BEHAVIOR: CancelledScaleBehavior,
> {
    pub value: f64,
}

impl<
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE: isize,
    const MASS_EXPONENT: isize, const MASS_SCALE: isize,
    const TIME_EXPONENT: isize, const TIME_P2: isize, const TIME_P3: isize, const TIME_P5: isize, const TIME_SCALE_ORDER: isize,
    const RESCALE_BEHAVIOR: RescaleBehavior, const CANCELLED_SCALE_BEHAVIOR: CancelledScaleBehavior,
>
    Quantity<
        LENGTH_EXPONENT, LENGTH_SCALE,
        MASS_EXPONENT, MASS_SCALE,
        TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
        RESCALE_BEHAVIOR, CANCELLED_SCALE_BEHAVIOR,
    >
{
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

// ============================================================================
// Display and Debug Implementations
// ============================================================================

use core::fmt;

// Helper function to build unit strings for both Display and Debug
fn build_unit_strings<
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE: isize,
    const MASS_EXPONENT: isize, const MASS_SCALE: isize,
    const TIME_EXPONENT: isize, const TIME_P2: isize, const TIME_P3: isize, const TIME_P5: isize, const TIME_SCALE_ORDER: isize,
>(
    use_long_names: bool,
    separate_numerator_denominator: bool,
) -> (Vec<String>, Vec<String>) {
    let mut numerator_units: Vec<String> = Vec::new();
    let mut denominator_units: Vec<String> = Vec::new();

    // Helper to push with exponent formatting
    let push_unit = |vec: &mut Vec<String>, name: &str, exp: i32| {
        if exp == 1 {
            vec.push(name.to_string());
        } else {
            vec.push(format!("{}^{}", name, exp));
        }
    };

    // Length
    if LENGTH_EXPONENT != 0 {
        let lname = if use_long_names {
            match LENGTH_SCALE {
                MILLIMETER_SCALE => "millimeter",
                METER_SCALE => "meter",
                KILOMETER_SCALE => "kilometer",
                LENGTH_UNUSED => "",
                _ => "unknown",
            }
        } else {
            match LENGTH_SCALE {
                MILLIMETER_SCALE => "mm",
                METER_SCALE => "m",
                KILOMETER_SCALE => "km",
                LENGTH_UNUSED => "",
                _ => "unknown",
            }
        };
        if !lname.is_empty() {
            if separate_numerator_denominator {
                let actual_exponent = LENGTH_EXPONENT as i32;
                if actual_exponent > 0 {
                    push_unit(&mut numerator_units, lname, actual_exponent);
                } else {
                    push_unit(&mut denominator_units, lname, -actual_exponent);
                }
            } else {
                push_unit(&mut numerator_units, lname, LENGTH_EXPONENT as i32);
            }
        }
    }

    // Mass
    if MASS_EXPONENT != 0 {
        let mname = if use_long_names {
            match MASS_SCALE {
                MILLIGRAM_SCALE => "milligram",
                GRAM_SCALE => "gram",
                KILOGRAM_SCALE => "kilogram",
                MASS_UNUSED => "",
                _ => "unknown",
            }
        } else {
            match MASS_SCALE {
                MILLIGRAM_SCALE => "mg",
                GRAM_SCALE => "g",
                KILOGRAM_SCALE => "kg",
                MASS_UNUSED => "",
                _ => "unknown",
            }
        };
        if !mname.is_empty() {
            if separate_numerator_denominator {
                let actual_exponent = MASS_EXPONENT as i32;
                if actual_exponent > 0 {
                    push_unit(&mut numerator_units, mname, actual_exponent);
                } else {
                    push_unit(&mut denominator_units, mname, -actual_exponent);
                }
            } else {
                push_unit(&mut numerator_units, mname, MASS_EXPONENT as i32);
            }
        }
    }

    // Time
    if TIME_EXPONENT != 0 {
        let tname = if use_long_names {
            match TIME_SCALE_ORDER {
                MILLISECOND_SCALE_ORDER => "millisecond".to_string(),
                SECOND_SCALE_ORDER => "second".to_string(),
                MINUTE_SCALE_ORDER => "minute".to_string(),
                TIME_UNUSED => "".to_string(),
                _ => "unknown".to_string(), // unrecognized unit
            }
        } else {
            match TIME_SCALE_ORDER {
                MILLISECOND_SCALE_ORDER => "ms",
                SECOND_SCALE_ORDER => "s",
                MINUTE_SCALE_ORDER => "min",
                TIME_UNUSED => "",
                _ => "unknown", // unrecognized unit
            }
            .to_string()
        };
        if separate_numerator_denominator {
            let actual_exponent = TIME_EXPONENT as i32;
            if actual_exponent > 0 {
                push_unit(&mut numerator_units, &tname, actual_exponent);
            } else {
                push_unit(&mut denominator_units, &tname, -actual_exponent);
            }
        } else {
            push_unit(&mut numerator_units, &tname, TIME_EXPONENT as i32);
        }
    }

    (numerator_units, denominator_units)
}

impl<
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE: isize,
    const MASS_EXPONENT: isize, const MASS_SCALE: isize,
    const TIME_EXPONENT: isize, const TIME_P2: isize, const TIME_P3: isize, const TIME_P5: isize, const TIME_SCALE_ORDER: isize,
    const RESCALE_BEHAVIOR: RescaleBehavior, const CANCELLED_SCALE_BEHAVIOR: CancelledScaleBehavior,
> fmt::Display
    for Quantity<
        LENGTH_EXPONENT, LENGTH_SCALE,
        MASS_EXPONENT, MASS_SCALE,
        TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
        RESCALE_BEHAVIOR, CANCELLED_SCALE_BEHAVIOR,
    >
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the value
        write!(f, "{}", self.value)?;

        // Add units using the helper function
        let (numerator_units, _) = build_unit_strings::<
            LENGTH_EXPONENT, LENGTH_SCALE,
            MASS_EXPONENT, MASS_SCALE,
            TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
        >(false, false);

        // If we have units, add them
        if !numerator_units.is_empty() {
            write!(f, " {}", numerator_units.join("⋅"))?;
        }

        Ok(())
    }
}

impl<
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE: isize,
    const MASS_EXPONENT: isize, const MASS_SCALE: isize,
    const TIME_EXPONENT: isize, const TIME_P2: isize, const TIME_P3: isize, const TIME_P5: isize, const TIME_SCALE_ORDER: isize,
    const RESCALE_BEHAVIOR: RescaleBehavior, const CANCELLED_SCALE_BEHAVIOR: CancelledScaleBehavior,
> fmt::Debug
    for Quantity<
        LENGTH_EXPONENT, LENGTH_SCALE,
        MASS_EXPONENT, MASS_SCALE,
        TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
        RESCALE_BEHAVIOR, CANCELLED_SCALE_BEHAVIOR,
    >
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Build human-readable unit literal expression using the helper function
        let (numerator_units, denominator_units) = build_unit_strings::<
            LENGTH_EXPONENT, LENGTH_SCALE,
            MASS_EXPONENT, MASS_SCALE,
            TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
        >(true, true);

        // Build unit string
        let unit_str = if !numerator_units.is_empty() || !denominator_units.is_empty() {
            let mut s = String::new();
            if !numerator_units.is_empty() {
                s.push_str(&format!("({})", numerator_units.join(" ⋅ ")));
            } else {
                s.push_str("(1)");
            }
            if !denominator_units.is_empty() {
                s.push_str(&format!(" / ({})", denominator_units.join(" ⋅ ")));
            }
            s
        } else {
            "(1)".to_string()
        };

        // Write debug output with value up front
        write!(f, "({}) ", self.value)?;
        write!(f, "Quantity<{}>", unit_str)?;
        write!(
            f,
            " Length: Exponent {} [Scale Index {}; {}], ",
            LENGTH_EXPONENT,
            match LENGTH_SCALE {
                isize::MAX => "MAX",
                _ => &LENGTH_SCALE.to_string(),
            },
            match LENGTH_SCALE {
                MILLIMETER_SCALE => "millimeter",
                METER_SCALE => "meter",
                KILOMETER_SCALE => "kilometer",
                LENGTH_UNUSED => "unused",
                _ => "unknown",
            }
        )?;
        write!(
            f,
            "Mass: Exponent {} [Scale Index {}; {}], ",
            MASS_EXPONENT,
            match MASS_SCALE {
                isize::MAX => "MAX",
                _ => &MASS_SCALE.to_string(),
            },
            match MASS_SCALE {
                MILLIGRAM_SCALE => "milligram",
                GRAM_SCALE => "gram",
                KILOGRAM_SCALE => "kilogram",
                MASS_UNUSED => "unused",
                _ => "unknown",
            }
        )?;
        write!(
            f,
            "Time: Exponent {} [Prime Factors p2:{}, p3:{}, p5:{}; {}], ",
            TIME_EXPONENT,
            match TIME_P2 {
                isize::MAX => "MAX",
                _ => &TIME_P2.to_string(),
            },
            match TIME_P3 {
                isize::MAX => "MAX",
                _ => &TIME_P3.to_string(),
            },
            match TIME_P5 {
                isize::MAX => "MAX",
                _ => &TIME_P5.to_string(),
            },
            match TIME_SCALE_ORDER {
                MILLISECOND_SCALE_ORDER => "millisecond",
                SECOND_SCALE_ORDER => "second",
                MINUTE_SCALE_ORDER => "minute",
                TIME_UNUSED => "unused",
                _ => "unknown",
            }
        )?;
        write!(
            f,
            "RescaleBehavior: {:?}, CancelledScaleBehavior: {:?}",
            RESCALE_BEHAVIOR, CANCELLED_SCALE_BEHAVIOR
        )?;
        Ok(())
    }
}

// ============================================================================
// Const Functions for Scale Operations
// ============================================================================

const fn aggregate_conversion_factor(
    length_exponent: isize, length_scale: isize, to_length_scale: isize,
    mass_exponent: isize, mass_scale: isize, to_mass_scale: isize,
    time_exponent: isize, time_p2: isize, time_p3: isize, time_p5: isize, to_time_p2: isize, to_time_p3: isize, to_time_p5: isize,
) -> f64 {
    length_conversion_factor(
        length_scale * length_exponent,
        to_length_scale * length_exponent,
        length_exponent,
    ) * mass_conversion_factor(
        mass_scale * mass_exponent,
        to_mass_scale * mass_exponent,
        mass_exponent,
    ) * time_conversion_factor(
        time_p2 * time_exponent, time_p3 * time_exponent, time_p5 * time_exponent,
        to_time_p2 * time_exponent, to_time_p3 * time_exponent, to_time_p5 * time_exponent,
        time_exponent,
    )
}

const fn pow1000(exp: isize) -> f64 {
    match exp {
        0 => 1.0,
        1 => 1000.0,
        2 => 1000000.0,
        3 => 1000000000.0,
        -1 => 0.001,
        -2 => 0.000001,
        -3 => 0.000000001,
        _ => 1.0, // we'll only test small values during prototyping
    }
}

pub const fn pow2(exp: isize) -> f64 {
    match exp {
        0 => 1.0,
        1 => 2.0,
        2 => 4.0,
        3 => 8.0,
        -1 => 0.5,
        -2 => 0.25,
        -3 => 0.125,
        _ => 1.0, // we'll only test small values during prototyping
    }
}

pub const fn pow3(exp: isize) -> f64 {
    match exp {
        0 => 1.0,
        1 => 3.0,
        2 => 9.0,
        3 => 27.0,
        -1 => 1.0 / 3.0,
        -2 => 1.0 / 9.0,
        -3 => 1.0 / 27.0,
        _ => 1.0, // we'll only test small values during prototyping
    }
}

pub const fn pow5(exp: isize) -> f64 {
    match exp {
        0 => 1.0,
        1 => 5.0,
        2 => 25.0,
        3 => 125.0,
        -1 => 0.2,
        -2 => 0.04,
        -3 => 0.008,
        _ => 1.0, // we'll only test small values during prototyping
    }
}

pub const fn length_conversion_factor(from: isize, to: isize, exponent: isize) -> f64 {
    let diff: isize = (from - to) * exponent;
    const UNUSED: isize = LENGTH_UNUSED;
    match (from, to) {
        (UNUSED, _) | (_, UNUSED) => 1.0, // unused scales are represented by 0; should never happen
        _ => pow1000(diff),
    }
}

pub const fn mass_conversion_factor(from: isize, to: isize, exponent: isize) -> f64 {
    let diff: isize = (from - to) * exponent;
    const UNUSED: isize = MASS_UNUSED;
    match (from, to) {
        (UNUSED, _) | (_, UNUSED) => 1.0, // unused scales are represented by 0; should never happen
        _ => pow1000(diff),
    }
}

pub const fn time_conversion_factor(
    from_p2: isize, from_p3: isize, from_p5: isize,
    to_p2: isize, to_p3: isize, to_p5: isize,
    exponent: isize,
) -> f64 {
    let diff_p2: isize = (from_p2 - to_p2) * exponent;
    let diff_p3: isize = (from_p3 - to_p3) * exponent;
    let diff_p5: isize = (from_p5 - to_p5) * exponent;
    const UNUSED: isize = TIME_UNUSED;
    match (from_p2, from_p3, from_p5, to_p2, to_p3, to_p5) {
        (UNUSED, _, _, _, _, _)
        | (_, UNUSED, _, _, _, _)
        | (_, _, UNUSED, _, _, _)
        | (_, _, _, UNUSED, _, _)
        | (_, _, _, _, UNUSED, _)
        | (_, _, _, _, _, UNUSED) => 1.0, // should never happen
        _ => pow2(diff_p2) * pow3(diff_p3) * pow5(diff_p5),
    }
}

// ============================================================================
// Rescale Behavior Functions
// ============================================================================

const fn min_length_scale(a: isize, b: isize) -> isize {
    match (a, b) {
        // unused scales have no opinion on scale selection
        (LENGTH_UNUSED, _) => b,
        (_, LENGTH_UNUSED) => a,
        _ => {
            if a < b {
                a
            } else {
                b
            }
        }
    }
}

const fn max_length_scale(a: isize, b: isize) -> isize {
    match (a, b) {
        // unused scales have no opinion on scale selection
        (LENGTH_UNUSED, _) => b,
        (_, LENGTH_UNUSED) => a,
        _ => {
            if a > b {
                a
            } else {
                b
            }
        }
    }
}

const fn min_mass_scale(a: isize, b: isize) -> isize {
    match (a, b) {
        // unused scales have no opinion on scale selection
        (MASS_UNUSED, _) => b,
        (_, MASS_UNUSED) => a,
        _ => {
            if a < b {
                a
            } else {
                b
            }
        }
    }
}

const fn max_mass_scale(a: isize, b: isize) -> isize {
    match (a, b) {
        // unused scales have no opinion on scale selection
        (MASS_UNUSED, _) => b,
        (_, MASS_UNUSED) => a,
        _ => {
            if a > b {
                a
            } else {
                b
            }
        }
    }
}

const fn min_time_scale(
    which_prime: isize,
    p2_1: isize, p3_1: isize, p5_1: isize, order_1: isize,
    p2_2: isize, p3_2: isize, p5_2: isize, order_2: isize,
) -> isize {
    // time scales are aggregate across primes, and we can't just mix-and-match or we end up with nonstandard scale values
    // so, we need to parse the two sets of primes to legal values, and *then* compare, and then return coherently from the result combination
    match (order_1, order_2) {
        (TIME_UNUSED, _) => match which_prime {
            0 => order_2,
            2 => p2_2,
            3 => p3_2,
            5 => p5_2,
            _ => TIME_UNUSED, // should never happen
        },
        (_, TIME_UNUSED) => match which_prime {
            0 => order_1,
            2 => p2_1,
            3 => p3_1,
            5 => p5_1,
            _ => TIME_UNUSED, // should never happen
        },
        _ => {
            if order_1 < order_2 {
                match which_prime {
                    0 => order_1,
                    2 => p2_1,
                    3 => p3_1,
                    5 => p5_1,
                    _ => TIME_UNUSED, // should never happen
                }
            } else {
                match which_prime {
                    0 => order_2,
                    2 => p2_2,
                    3 => p3_2,
                    5 => p5_2,
                    _ => TIME_UNUSED, // should never happen
                }
            }
        }
    }
}

const fn max_time_scale(
    which_prime: isize,
    p2_1: isize, p3_1: isize, p5_1: isize, order_1: isize,
    p2_2: isize, p3_2: isize, p5_2: isize, order_2: isize,
) -> isize {
    // time scales are aggregate across primes, and we can't just mix-and-match or we end up with nonstandard scale values
    // so, we need to parse the two sets of primes to legal values, and *then* compare, and then return coherently from the result combination
    match (order_1, order_2) {
        (TIME_UNUSED, _) => match which_prime {
            2 => p2_2,
            3 => p3_2,
            5 => p5_2,
            _ => TIME_UNUSED, // should never happen
        },
        (_, TIME_UNUSED) => match which_prime {
            2 => p2_1,
            3 => p3_1,
            5 => p5_1,
            _ => TIME_UNUSED, // should never happen
        },
        _ => {
            if order_1 > order_2 {
                match which_prime {
                    2 => p2_1,
                    3 => p3_1,
                    5 => p5_1,
                    _ => TIME_UNUSED, // should never happen
                }
            } else {
                match which_prime {
                    2 => p2_2,
                    3 => p3_2,
                    5 => p5_2,
                    _ => TIME_UNUSED, // should never happen
                }
            }
        }
    }
}

// Helper functions to determine result scales based on rescale behavior and cancellation behavior
pub const fn result_length_scale(
    scale1: isize, scale2: isize,
    rescale_behavior: RescaleBehavior, cancelled_scale_behavior: CancelledScaleBehavior,
    exponent: isize,
) -> isize {
    match cancelled_scale_behavior {
        CancelledScaleBehavior::Forget if exponent == 0 => LENGTH_UNUSED, // revert to unused scale sentinal value
        _ => match rescale_behavior {
            RescaleBehavior::SmallerWins => min_length_scale(scale1, scale2),
            RescaleBehavior::LargerWins => max_length_scale(scale1, scale2),
            RescaleBehavior::LeftHandWins => scale1,
            RescaleBehavior::Strict => scale1, // strict rescale behavior means this should never be reached
        },
    }
}

pub const fn result_mass_scale(
    scale1: isize, scale2: isize,
    rescale_behavior: RescaleBehavior, cancelled_scale_behavior: CancelledScaleBehavior,
    exponent: isize,
) -> isize {
    match cancelled_scale_behavior {
        CancelledScaleBehavior::Forget if exponent == 0 => MASS_UNUSED, // revert to unused scale sentinal value
        _ => match rescale_behavior {
            RescaleBehavior::SmallerWins => min_mass_scale(scale1, scale2),
            RescaleBehavior::LargerWins => max_mass_scale(scale1, scale2),
            RescaleBehavior::LeftHandWins => scale1,
            RescaleBehavior::Strict => scale1, // strict rescale behavior means this should never be reached
        },
    }
}

pub const fn result_time_scale(
    which_prime: isize,
    p2_1: isize, p3_1: isize, p5_1: isize, order_1: isize,
    p2_2: isize, p3_2: isize, p5_2: isize, order_2: isize,
    rescale_behavior: RescaleBehavior, cancelled_scale_behavior: CancelledScaleBehavior,
    exponent: isize,
) -> isize {
    match cancelled_scale_behavior {
        CancelledScaleBehavior::Forget if exponent == 0 => TIME_UNUSED, // revert to unused scale sentinal value
        _ => match rescale_behavior {
            RescaleBehavior::SmallerWins => min_time_scale(
                which_prime,
                p2_1, p3_1, p5_1, order_1,
                p2_2, p3_2, p5_2, order_2,
            ),
            RescaleBehavior::LargerWins => max_time_scale(
                which_prime,
                p2_1, p3_1, p5_1, order_1,
                p2_2, p3_2, p5_2, order_2,
            ),
            RescaleBehavior::LeftHandWins => match which_prime {
                0 => order_1,
                2 => p2_1,
                3 => p3_1,
                5 => p5_1,
                _ => TIME_UNUSED, // should never happen
            },
            RescaleBehavior::Strict => TIME_UNUSED, // strict rescale behavior means this should never be reached
        },
    }
}

pub const fn time_scale_2(order: isize) -> isize {
    match order {
        MILLISECOND_SCALE_ORDER => MILLISECOND_SCALE_P2,
        SECOND_SCALE_ORDER => SECOND_SCALE_P2,
        MINUTE_SCALE_ORDER => MINUTE_SCALE_P2,
        _ => 0, // TODO: handle this better
    }
}

pub const fn time_scale_3(order: isize) -> isize {
    match order {
        MILLISECOND_SCALE_ORDER => MILLISECOND_SCALE_P3,
        SECOND_SCALE_ORDER => SECOND_SCALE_P3,
        MINUTE_SCALE_ORDER => MINUTE_SCALE_P3,
        _ => 0, // TODO: handle this better
    }
}

pub const fn time_scale_5(order: isize) -> isize {
    match order {
        MILLISECOND_SCALE_ORDER => MILLISECOND_SCALE_P5,
        SECOND_SCALE_ORDER => SECOND_SCALE_P5,
        MINUTE_SCALE_ORDER => MINUTE_SCALE_P5,
        _ => 0, // TODO: handle this better
    }
}

// ============================================================================
// Dimension constraint checking trait
// ============================================================================

trait DimensionsMatch<
    const L1: isize,
    const L2: isize,
    const M1: isize,
    const M2: isize,
    const T1: isize,
    const T2: isize,
>
{
}
impl<const L: isize, const M: isize, const T: isize> DimensionsMatch<L, L, M, M, T, T> for () {}

trait RescaleBehaviorsMatch<const B1: RescaleBehavior, const B2: RescaleBehavior> {}
impl<const B: RescaleBehavior> RescaleBehaviorsMatch<B, B> for () {}

trait CancelledScaleBehaviorsMatch<
    const B1: CancelledScaleBehavior,
    const B2: CancelledScaleBehavior,
>
{
}
impl<const B: CancelledScaleBehavior> CancelledScaleBehaviorsMatch<B, B> for () {}

trait ValidScale<const S: isize> {}
impl<const S: isize> ValidScale<S> for () {}

// ============================================================================
// Arithmetic Operations
// ============================================================================

impl<
    const LENGTH_EXPONENT1: isize, const LENGTH_SCALE1: isize,
    const MASS_EXPONENT1: isize, const MASS_SCALE1: isize,
    const TIME_EXPONENT1: isize, const TIME_P2_1: isize, const TIME_P3_1: isize, const TIME_P5_1: isize, const TIME_SCALE_ORDER1: isize,
    const RESCALE_BEHAVIOR1: RescaleBehavior, const CANCELLED_SCALE_BEHAVIOR1: CancelledScaleBehavior,
    const LENGTH_EXPONENT2: isize, const LENGTH_SCALE2: isize,
    const MASS_EXPONENT2: isize, const MASS_SCALE2: isize,
    const TIME_EXPONENT2: isize, const TIME_P2_2: isize, const TIME_P3_2: isize, const TIME_P5_2: isize, const TIME_SCALE_ORDER2: isize,
    const RESCALE_BEHAVIOR2: RescaleBehavior, const CANCELLED_SCALE_BEHAVIOR2: CancelledScaleBehavior,
>
    Add<
        Quantity<
            LENGTH_EXPONENT2, LENGTH_SCALE2,
            MASS_EXPONENT2, MASS_SCALE2,
            TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2,
            RESCALE_BEHAVIOR2, CANCELLED_SCALE_BEHAVIOR2,
        >,
    >
    for Quantity<
        LENGTH_EXPONENT1, LENGTH_SCALE1,
        MASS_EXPONENT1, MASS_SCALE1,
        TIME_EXPONENT1, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1,
        RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1,
    >
where
    (): DimensionsMatch<LENGTH_EXPONENT1, LENGTH_EXPONENT2, MASS_EXPONENT1, MASS_EXPONENT2, TIME_EXPONENT1, TIME_EXPONENT2>,
    (): RescaleBehaviorsMatch<RESCALE_BEHAVIOR1, RESCALE_BEHAVIOR2>,
    (): CancelledScaleBehaviorsMatch<CANCELLED_SCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR2>,
    (): ValidScale<{ result_length_scale(LENGTH_SCALE1, LENGTH_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, LENGTH_EXPONENT1) }>,
    (): ValidScale<{ result_mass_scale(MASS_SCALE1, MASS_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, MASS_EXPONENT1) }>,
    (): ValidScale<{ result_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1) }>,
    (): ValidScale<{ result_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1) }>,
    (): ValidScale<{ result_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1) }>,
    (): ValidScale<{ result_time_scale(0, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1) }>,
{
    type Output = Quantity<
        LENGTH_EXPONENT1,
        { result_length_scale(LENGTH_SCALE1, LENGTH_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, LENGTH_EXPONENT1) },
        MASS_EXPONENT1,
        { result_mass_scale(MASS_SCALE1, MASS_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, MASS_EXPONENT1) },
        TIME_EXPONENT1,
        { result_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1) },
        { result_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1) },
        { result_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1) },
        { result_time_scale(0, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1) },
        RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1,
    >;

    fn add(
        self,
        other: Quantity<
            LENGTH_EXPONENT2, LENGTH_SCALE2,
            MASS_EXPONENT2, MASS_SCALE2,
            TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2,
            RESCALE_BEHAVIOR2, CANCELLED_SCALE_BEHAVIOR2,
        >,
    ) -> Self::Output {
        let result_length_scale = result_length_scale(LENGTH_SCALE1, LENGTH_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, LENGTH_EXPONENT1);
        let result_mass_scale = result_mass_scale(MASS_SCALE1, MASS_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, MASS_EXPONENT1);
        let result_time_p2 = result_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1);
        let result_time_p3 = result_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1);
        let result_time_p5 = result_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1);
       
        let factor1 = aggregate_conversion_factor(
            LENGTH_EXPONENT1,LENGTH_SCALE1, result_length_scale,
            MASS_EXPONENT1,MASS_SCALE1, result_mass_scale,
            TIME_EXPONENT1,TIME_P2_1, TIME_P3_1, TIME_P5_1, result_time_p2, result_time_p3, result_time_p5,
        );
        let factor2 = aggregate_conversion_factor(
            LENGTH_EXPONENT2,LENGTH_SCALE2, result_length_scale,
            MASS_EXPONENT2,MASS_SCALE2, result_mass_scale,
            TIME_EXPONENT2,TIME_P2_2, TIME_P3_2, TIME_P5_2, result_time_p2, result_time_p3, result_time_p5,
        );

        let result_value = self.value * factor1 + other.value * factor2;
        Quantity::new(result_value)
    }
}

impl<
    const LENGTH_EXPONENT1: isize, const LENGTH_SCALE1: isize,
    const MASS_EXPONENT1: isize, const MASS_SCALE1: isize,
    const TIME_EXPONENT1: isize, const TIME_P2_1: isize, const TIME_P3_1: isize, const TIME_P5_1: isize, const TIME_SCALE_ORDER1: isize,
    const RESCALE_BEHAVIOR1: RescaleBehavior, const CANCELLED_SCALE_BEHAVIOR1: CancelledScaleBehavior,
    const LENGTH_EXPONENT2: isize, const LENGTH_SCALE2: isize,
    const MASS_EXPONENT2: isize, const MASS_SCALE2: isize,
    const TIME_EXPONENT2: isize, const TIME_P2_2: isize, const TIME_P3_2: isize, const TIME_P5_2: isize, const TIME_SCALE_ORDER2: isize,
    const RESCALE_BEHAVIOR2: RescaleBehavior, const CANCELLED_SCALE_BEHAVIOR2: CancelledScaleBehavior,
>
    Mul<
        Quantity<
            LENGTH_EXPONENT2, LENGTH_SCALE2,
            MASS_EXPONENT2, MASS_SCALE2,
            TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2,
            RESCALE_BEHAVIOR2, CANCELLED_SCALE_BEHAVIOR2,
        >,
    >
    for Quantity<
        LENGTH_EXPONENT1, LENGTH_SCALE1,
        MASS_EXPONENT1, MASS_SCALE1,
        TIME_EXPONENT1, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1,
        RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1,
    >
where
    (): RescaleBehaviorsMatch<RESCALE_BEHAVIOR1, RESCALE_BEHAVIOR2>,
    (): CancelledScaleBehaviorsMatch<CANCELLED_SCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR2>,
    (): ValidScale<{ LENGTH_EXPONENT1 + LENGTH_EXPONENT2 }>,
    (): ValidScale<{ MASS_EXPONENT1 + MASS_EXPONENT2 }>,
    (): ValidScale<{ TIME_EXPONENT1 + TIME_EXPONENT2 }>,
    (): ValidScale<{ result_length_scale(LENGTH_SCALE1, LENGTH_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, LENGTH_EXPONENT1 + LENGTH_EXPONENT2) }>,
    (): ValidScale<{ result_mass_scale(MASS_SCALE1, MASS_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, MASS_EXPONENT1 + MASS_EXPONENT2) }>,
    (): ValidScale<{ result_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 + TIME_EXPONENT2) }>,
    (): ValidScale<{ result_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 + TIME_EXPONENT2) }>,
    (): ValidScale<{ result_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 + TIME_EXPONENT2) }>,
    (): ValidScale<{ result_time_scale(0, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 + TIME_EXPONENT2) }>,
{
    type Output = Quantity<
        { LENGTH_EXPONENT1 + LENGTH_EXPONENT2 },
        { result_length_scale(LENGTH_SCALE1, LENGTH_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, LENGTH_EXPONENT1 + LENGTH_EXPONENT2) },
        { MASS_EXPONENT1 + MASS_EXPONENT2 },
        { result_mass_scale(MASS_SCALE1, MASS_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, MASS_EXPONENT1 + MASS_EXPONENT2) },
        { TIME_EXPONENT1 + TIME_EXPONENT2 },
        { result_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 + TIME_EXPONENT2) },
        { result_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 + TIME_EXPONENT2) },
        { result_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 + TIME_EXPONENT2) },
        { result_time_scale(0, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 + TIME_EXPONENT2) },
        RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1,
    >;

    fn mul(
        self,
        other: Quantity<
            LENGTH_EXPONENT2, LENGTH_SCALE2,
            MASS_EXPONENT2, MASS_SCALE2,
            TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2,
            RESCALE_BEHAVIOR2, CANCELLED_SCALE_BEHAVIOR2,
        >,
    ) -> Self::Output {
        let result_length_scale = result_length_scale(LENGTH_SCALE1, LENGTH_SCALE2, RESCALE_BEHAVIOR1, CancelledScaleBehavior::Retain, LENGTH_EXPONENT1 + LENGTH_EXPONENT2);
        let result_mass_scale = result_mass_scale(MASS_SCALE1, MASS_SCALE2, RESCALE_BEHAVIOR1, CancelledScaleBehavior::Retain, MASS_EXPONENT1 + MASS_EXPONENT2);
        let result_time_p2 = result_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CancelledScaleBehavior::Retain, TIME_EXPONENT1 + TIME_EXPONENT2);
        let result_time_p3 = result_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CancelledScaleBehavior::Retain, TIME_EXPONENT1 + TIME_EXPONENT2);
        let result_time_p5 = result_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CancelledScaleBehavior::Retain, TIME_EXPONENT1 + TIME_EXPONENT2);

        let conversion_factor = aggregate_conversion_factor(
            LENGTH_EXPONENT1, LENGTH_SCALE1, result_length_scale,
            MASS_EXPONENT1, MASS_SCALE1, result_mass_scale,
            TIME_EXPONENT1, TIME_P2_1, TIME_P3_1, TIME_P5_1, result_time_p2, result_time_p3, result_time_p5,
        ) * aggregate_conversion_factor(
            LENGTH_EXPONENT2, LENGTH_SCALE2, result_length_scale,
            MASS_EXPONENT2, MASS_SCALE2, result_mass_scale,
            TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, result_time_p2, result_time_p3, result_time_p5,
        );

        let result_value = self.value * other.value * conversion_factor;
        Quantity::new(result_value)
    }
}

// ============================================================================
// Division Implementation
// ============================================================================

impl<
    const LENGTH_EXPONENT1: isize, const LENGTH_SCALE1: isize,
    const MASS_EXPONENT1: isize, const MASS_SCALE1: isize,
    const TIME_EXPONENT1: isize, const TIME_P2_1: isize, const TIME_P3_1: isize, const TIME_P5_1: isize, const TIME_SCALE_ORDER1: isize,
    const RESCALE_BEHAVIOR1: RescaleBehavior, const CANCELLED_SCALE_BEHAVIOR1: CancelledScaleBehavior,
    const LENGTH_EXPONENT2: isize, const LENGTH_SCALE2: isize,
    const MASS_EXPONENT2: isize, const MASS_SCALE2: isize,
    const TIME_EXPONENT2: isize, const TIME_P2_2: isize, const TIME_P3_2: isize, const TIME_P5_2: isize, const TIME_SCALE_ORDER2: isize,
    const RESCALE_BEHAVIOR2: RescaleBehavior, const CANCELLED_SCALE_BEHAVIOR2: CancelledScaleBehavior,
>
    Div<
        Quantity<
            LENGTH_EXPONENT2, LENGTH_SCALE2,
            MASS_EXPONENT2, MASS_SCALE2,
            TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2,
            RESCALE_BEHAVIOR2, CANCELLED_SCALE_BEHAVIOR2,
        >,
    >
    for Quantity<
        LENGTH_EXPONENT1, LENGTH_SCALE1,
        MASS_EXPONENT1, MASS_SCALE1,
        TIME_EXPONENT1, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1,
        RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1,
    >
where
    (): RescaleBehaviorsMatch<RESCALE_BEHAVIOR1, RESCALE_BEHAVIOR2>,
    (): CancelledScaleBehaviorsMatch<CANCELLED_SCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR2>,
    (): ValidScale<{ LENGTH_EXPONENT1 - LENGTH_EXPONENT2 }>,
    (): ValidScale<{ MASS_EXPONENT1 - MASS_EXPONENT2 }>,
    (): ValidScale<{ TIME_EXPONENT1 - TIME_EXPONENT2 }>,
    (): ValidScale<{ result_length_scale(LENGTH_SCALE1, LENGTH_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, LENGTH_EXPONENT1 - LENGTH_EXPONENT2) }>,
    (): ValidScale<{ result_mass_scale(MASS_SCALE1, MASS_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, MASS_EXPONENT1 - MASS_EXPONENT2) }>,
    (): ValidScale<{ result_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 - TIME_EXPONENT2) }>,
    (): ValidScale<{ result_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 - TIME_EXPONENT2) }>,
    (): ValidScale<{ result_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 - TIME_EXPONENT2) }>,
    (): ValidScale<{ result_time_scale(0, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 - TIME_EXPONENT2) }>,
{
    type Output = Quantity<
        { LENGTH_EXPONENT1 - LENGTH_EXPONENT2 },
        { result_length_scale(LENGTH_SCALE1, LENGTH_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, LENGTH_EXPONENT1 - LENGTH_EXPONENT2) },
        { MASS_EXPONENT1 - MASS_EXPONENT2 },
        { result_mass_scale(MASS_SCALE1, MASS_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, MASS_EXPONENT1 - MASS_EXPONENT2) },
        { TIME_EXPONENT1 - TIME_EXPONENT2 },
        { result_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 - TIME_EXPONENT2) },
        { result_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 - TIME_EXPONENT2) },
        { result_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 - TIME_EXPONENT2) },
        { result_time_scale(0, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, TIME_EXPONENT1 - TIME_EXPONENT2) },
        RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1,
    >;

    fn div(
        self,
        other: Quantity<
            LENGTH_EXPONENT2, LENGTH_SCALE2,
            MASS_EXPONENT2, MASS_SCALE2,
            TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2,
            RESCALE_BEHAVIOR2, CANCELLED_SCALE_BEHAVIOR2,
        >,
    ) -> Self::Output {
        let result_length_scale = result_length_scale(LENGTH_SCALE1, LENGTH_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, LENGTH_EXPONENT1 - LENGTH_EXPONENT2);
        let result_mass_scale = result_mass_scale(MASS_SCALE1, MASS_SCALE2, RESCALE_BEHAVIOR1, CANCELLED_SCALE_BEHAVIOR1, MASS_EXPONENT1 - MASS_EXPONENT2);
        let result_time_p2 = result_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CancelledScaleBehavior::Retain, TIME_EXPONENT1 - TIME_EXPONENT2);
        let result_time_p3 = result_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CancelledScaleBehavior::Retain, TIME_EXPONENT1 - TIME_EXPONENT2);
        let result_time_p5 = result_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CancelledScaleBehavior::Retain, TIME_EXPONENT1 - TIME_EXPONENT2);
        let result_time_0 = result_time_scale(0, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2, RESCALE_BEHAVIOR1, CancelledScaleBehavior::Retain, TIME_EXPONENT1 - TIME_EXPONENT2);

        let factor = aggregate_conversion_factor(
            LENGTH_EXPONENT1, LENGTH_SCALE1, result_length_scale,
            MASS_EXPONENT1, MASS_SCALE1, result_mass_scale,
            TIME_EXPONENT1, TIME_P2_1, TIME_P3_1, TIME_P5_1, result_time_p2, result_time_p3, result_time_p5,
        ) / aggregate_conversion_factor(
            LENGTH_EXPONENT2, LENGTH_SCALE2, result_length_scale,
            MASS_EXPONENT2, MASS_SCALE2, result_mass_scale,
            TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, result_time_p2, result_time_p3, result_time_p5,
        );

        let result_value = self.value / other.value * factor;
        Quantity::new(result_value)
    }
}


pub mod default_declarators;
pub mod scoped_preferences;
