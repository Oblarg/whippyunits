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

pub const MILLIMETER_SCALE: isize = -1;
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
    Strict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum CancelledScaleBehavior {
    Retain, // Keep the storage scales even when dimensions are cancelled
    Forget, // Automatically convert to Unused when exponent becomes 0
}

// ============================================================================
// Quantity Type
// ============================================================================
#[derive(Clone, Copy)]
pub struct Quantity<
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE: isize,
    const MASS_EXPONENT: isize, const MASS_SCALE: isize,
    const TIME_EXPONENT: isize, const TIME_P2: isize, const TIME_P3: isize, const TIME_P5: isize, const TIME_SCALE_ORDER: isize,
> {
    pub value: f64,
}

impl<
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE: isize,
    const MASS_EXPONENT: isize, const MASS_SCALE: isize,
    const TIME_EXPONENT: isize, const TIME_P2: isize, const TIME_P3: isize, const TIME_P5: isize, const TIME_SCALE_ORDER: isize,
>
    Quantity<
        LENGTH_EXPONENT, LENGTH_SCALE,
        MASS_EXPONENT, MASS_SCALE,
        TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
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
> fmt::Display
    for Quantity<
        LENGTH_EXPONENT, LENGTH_SCALE,
        MASS_EXPONENT, MASS_SCALE,
        TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
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
> fmt::Debug
    for Quantity<
        LENGTH_EXPONENT, LENGTH_SCALE,
        MASS_EXPONENT, MASS_SCALE,
        TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
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
        let length_scale_str = if LENGTH_SCALE == isize::MAX {
            "MAX".to_string()
        } else {
            LENGTH_SCALE.to_string()
        };
        write!(
            f,
            " Length: Exponent {} [Scale Index {}; {}], ",
            LENGTH_EXPONENT,
            length_scale_str,
            match LENGTH_SCALE {
                MILLIMETER_SCALE => "millimeter",
                METER_SCALE => "meter",
                KILOMETER_SCALE => "kilometer",
                LENGTH_UNUSED => "unused",
                _ => "unknown",
            }
        )?;
        let mass_scale_str = if MASS_SCALE == isize::MAX {
            "MAX".to_string()
        } else {
            MASS_SCALE.to_string()
        };
        write!(
            f,
            "Mass: Exponent {} [Scale Index {}; {}], ",
            MASS_EXPONENT,
            mass_scale_str,
            match MASS_SCALE {
                MILLIGRAM_SCALE => "milligram",
                GRAM_SCALE => "gram",
                KILOGRAM_SCALE => "kilogram",
                MASS_UNUSED => "unused",
                _ => "unknown",
            }
        )?;
        let time_p2_str = if TIME_P2 == isize::MAX {
            "MAX".to_string()
        } else {
            TIME_P2.to_string()
        };
        let time_p3_str = if TIME_P3 == isize::MAX {
            "MAX".to_string()
        } else {
            TIME_P3.to_string()
        };
        let time_p5_str = if TIME_P5 == isize::MAX {
            "MAX".to_string()
        } else {
            TIME_P5.to_string()
        };
        write!(
            f,
            "Time: Exponent {} [Prime Factors p2:{}, p3:{}, p5:{}; {}], ",
            TIME_EXPONENT,
            time_p2_str,
            time_p3_str,
            time_p5_str,
            match TIME_SCALE_ORDER {
                MILLISECOND_SCALE_ORDER => "millisecond",
                SECOND_SCALE_ORDER => "second",
                MINUTE_SCALE_ORDER => "minute",
                TIME_UNUSED => "unused",
                _ => "unknown",
            }
        )?;
        Ok(())
    }
}

// ============================================================================
// Const Functions for Scale Operations
// ============================================================================

const fn aggregate_conversion_factor(
    length_exponent: isize, from_length_scale: isize, to_length_scale: isize,
    mass_exponent: isize, from_mass_scale: isize, to_mass_scale: isize,
    time_exponent: isize, from_time_p2: isize, from_time_p3: isize, from_time_p5: isize, to_time_p2: isize, to_time_p3: isize, to_time_p5: isize,
) -> f64 {
    length_conversion_factor(
        from_length_scale * length_exponent,
        to_length_scale * length_exponent,
        length_exponent,
    ) * mass_conversion_factor(
        from_mass_scale * mass_exponent,
        to_mass_scale * mass_exponent,
        mass_exponent,
    ) * time_conversion_factor(
        from_time_p2 * time_exponent, from_time_p3 * time_exponent, from_time_p5 * time_exponent,
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

pub const fn left_hand_wins_scale(a: isize, b: isize) -> isize {
    match (a, b) {
        (LENGTH_UNUSED, _) => b,
        _ => a,
    }
}

pub const fn min_length_scale(a: isize, b: isize) -> isize {
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

pub const fn max_length_scale(a: isize, b: isize) -> isize {
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

pub const fn min_mass_scale(a: isize, b: isize) -> isize {
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

pub const fn max_mass_scale(a: isize, b: isize) -> isize {
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

pub const fn min_time_scale(
    which_prime: isize,
    p2_1: isize, p3_1: isize, p5_1: isize, order_1: isize,
    p2_2: isize, p3_2: isize, p5_2: isize, order_2: isize,
) -> isize {
    // time scales are aggregate across primes, and we can't just mix-and-match or we end up with nonstandard scale values
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

pub const fn max_time_scale(
    which_prime: isize,
    p2_1: isize, p3_1: isize, p5_1: isize, order_1: isize,
    p2_2: isize, p3_2: isize, p5_2: isize, order_2: isize,
) -> isize {
    // time scales are aggregate across primes, and we can't just mix-and-match or we end up with nonstandard scale values
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

trait IsIsize<const S: isize> {}
impl<const S: isize> IsIsize<S> for () {}

// ============================================================================
// Conversions
// ============================================================================

const fn rescale_value<
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE_FROM: isize, const LENGTH_SCALE_TO: isize,
    const MASS_EXPONENT: isize, const MASS_SCALE_FROM: isize, const MASS_SCALE_TO: isize,
    const TIME_EXPONENT: isize, const TIME_P2_FROM: isize, const TIME_P3_FROM: isize, const TIME_P5_FROM: isize,
                                const TIME_P2_TO: isize, const TIME_P3_TO: isize, const TIME_P5_TO: isize,
>(quantity: f64) -> f64 {
    let factor = aggregate_conversion_factor(
        LENGTH_EXPONENT, LENGTH_SCALE_FROM, LENGTH_SCALE_TO,
        MASS_EXPONENT, MASS_SCALE_FROM, MASS_SCALE_TO,
        TIME_EXPONENT, TIME_P2_FROM, TIME_P3_FROM, TIME_P5_FROM, TIME_P2_TO, TIME_P3_TO, TIME_P5_TO,
    );
    quantity * factor
}

pub fn rescale<
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE_FROM: isize, const LENGTH_SCALE_TO: isize,
    const MASS_EXPONENT: isize, const MASS_SCALE_FROM: isize, const MASS_SCALE_TO: isize,
    const TIME_EXPONENT: isize, const TIME_P2_FROM: isize, const TIME_P3_FROM: isize, const TIME_P5_FROM: isize, const TIME_SCALE_ORDER_FROM: isize,
                                const TIME_P2_TO: isize, const TIME_P3_TO: isize, const TIME_P5_TO: isize, const TIME_SCALE_ORDER_TO: isize,
> (
    quantity: Quantity<
        LENGTH_EXPONENT, LENGTH_SCALE_FROM,
        MASS_EXPONENT, MASS_SCALE_FROM,
        TIME_EXPONENT, TIME_P2_FROM, TIME_P3_FROM, TIME_P5_FROM, TIME_SCALE_ORDER_FROM,
    >,
) -> Quantity<
    LENGTH_EXPONENT, LENGTH_SCALE_TO,
    MASS_EXPONENT, MASS_SCALE_TO,
    TIME_EXPONENT, TIME_P2_TO, TIME_P3_TO, TIME_P5_TO, TIME_SCALE_ORDER_TO,
> {
    Quantity::new(rescale_value::<
        LENGTH_EXPONENT, LENGTH_SCALE_FROM, LENGTH_SCALE_TO,
        MASS_EXPONENT, MASS_SCALE_FROM, MASS_SCALE_TO,
        TIME_EXPONENT, TIME_P2_FROM, TIME_P3_FROM, TIME_P5_FROM, TIME_P2_TO, TIME_P3_TO, TIME_P5_TO,
    >(quantity.value))
}

pub mod default_declarators;
pub mod scoped_preferences;
#[macro_use]
pub mod arithmetic;
pub mod api;

// Re-export the proc macro
pub use whippyunits_unit_macro::proc_unit;

