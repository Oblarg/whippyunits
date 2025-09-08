use crate::IsI8;
use crate::scale_conversion::*;
use crate::quantity_type::*;
use crate::print::prettyprint::*;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt;

define_composite_scale_factor!(
    // params
    (
        p2_from: i8, p3_from: i8, p5_from: i8,
        p2_to: i8, p3_to: i8, p5_to: i8,
    ),
    // since a later clause depends on exponent, we need to pass it in as a parameter
    exponent,
    // one branch pow expressions
    (
        let (num2, den2) = pow2(p2_from as i16 - p2_to as i16);
        let (num3, den3) = pow3(p3_from as i16 - p3_to as i16);
        let (num5, den5) = pow5(p5_from as i16 - p5_to as i16);
    ),
    // default branch pow expressions
    (
        let (num2, den2) = pow2((p2_from as i16 - p2_to as i16) * exponent as i16);
        let (num3, den3) = pow3((p3_from as i16 - p3_to as i16) * exponent as i16);
        let (num5, den5) = pow5((p5_from as i16 - p5_to as i16) * exponent as i16);
    ),
    // numerator expression
    (
        num2 as i128 * num3 as i128 * num5 as i128
    ),
    // denominator expression
    (
        den2 as i128 * den3 as i128 * den5 as i128
    ),
    time_scale_factor,
);

define_aggregate_scale_factor!(
    // params
    (
        mass_exponent: i8, mass_scale_p10_from: i8, mass_scale_p10_to: i8,
        length_exponent: i8, length_scale_p10_from: i8, length_scale_p10_to: i8,
        time_exponent: i8, time_scale_p2_from: i8, time_scale_p3_from: i8, time_scale_p5_from: i8,
                           time_scale_p2_to: i8, time_scale_p3_to: i8, time_scale_p5_to: i8
    ),
    // diff expressions
    (
        let diff_length_p10 = (length_scale_p10_from as i16 - length_scale_p10_to as i16) * length_exponent as i16;
        let diff_mass_p10 = (mass_scale_p10_from as i16 - mass_scale_p10_to as i16) * mass_exponent as i16;
        let diff_time_p2 = (time_scale_p2_from as i16 - time_scale_p2_to as i16) * time_exponent as i16;
        let diff_time_p3 = (time_scale_p3_from as i16 - time_scale_p3_to as i16) * time_exponent as i16;
        let diff_time_p5 = (time_scale_p5_from as i16 - time_scale_p5_to as i16) * time_exponent as i16;
    ),
    // pow expressions
    (
        let (num10, den10) = pow10(diff_length_p10 + diff_mass_p10);
        let (num2, den2) = pow2(diff_time_p2);
        let (num3, den3) = pow3(diff_time_p3);
        let (num5, den5) = pow5(diff_time_p5);
    ),
    // num and den expressions
    (num10 * num2 * num3 * num5),
    (den10 * den2 * den3 * den5),
);

macro_rules! define_float_rescale {
    ($rescale_fn:ident, $T:ty) => {
        _define_float_rescale!(
            (
                const MASS_EXPONENT: i8, const MASS_SCALE_P10_FROM: i8, const MASS_SCALE_P10_TO: i8,
                const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10_FROM: i8, const LENGTH_SCALE_P10_TO: i8,
                const TIME_EXPONENT: i8, const TIME_SCALE_P2_FROM: i8, const TIME_SCALE_P3_FROM: i8, const TIME_SCALE_P5_FROM: i8,
                                         const TIME_SCALE_P2_TO: i8, const TIME_SCALE_P3_TO: i8, const TIME_SCALE_P5_TO: i8,
            ),
            (
                Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_FROM,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_FROM,
                    TIME_EXPONENT, TIME_SCALE_P2_FROM, TIME_SCALE_P3_FROM, TIME_SCALE_P5_FROM,
                    $T,
                >
            ),
            (
                Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_TO,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_TO,
                    TIME_EXPONENT, TIME_SCALE_P2_TO, TIME_SCALE_P3_TO, TIME_SCALE_P5_TO,
                    $T,
                >
            ),
            (
                MASS_EXPONENT, MASS_SCALE_P10_FROM, MASS_SCALE_P10_TO,
                LENGTH_EXPONENT, LENGTH_SCALE_P10_FROM, LENGTH_SCALE_P10_TO,
                TIME_EXPONENT, TIME_SCALE_P2_FROM, TIME_SCALE_P3_FROM, TIME_SCALE_P5_FROM, 
                               TIME_SCALE_P2_TO, TIME_SCALE_P3_TO, TIME_SCALE_P5_TO,
            ),
            ($rescale_fn, $T),
        );
    };
    
}

define_float_rescale!(rescale_f64, f64);

macro_rules! define_int_rescale {
    ($rescale_fn:ident, $T:ty) => {
        _define_int_rescale!(
            (
                const MASS_EXPONENT: i8, const MASS_SCALE_P10_FROM: i8, const MASS_SCALE_P10_TO: i8,
                const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10_FROM: i8, const LENGTH_SCALE_P10_TO: i8,
                const TIME_EXPONENT: i8, const TIME_SCALE_P2_FROM: i8, const TIME_SCALE_P3_FROM: i8, const TIME_SCALE_P5_FROM: i8,
                                         const TIME_SCALE_P2_TO: i8, const TIME_SCALE_P3_TO: i8, const TIME_SCALE_P5_TO: i8,
            ),
            (
                Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_FROM,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_FROM,
                    TIME_EXPONENT, TIME_SCALE_P2_FROM, TIME_SCALE_P3_FROM, TIME_SCALE_P5_FROM,
                    $T,
                >
            ),
            (
                Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_TO,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_TO,
                    TIME_EXPONENT, TIME_SCALE_P2_TO, TIME_SCALE_P3_TO, TIME_SCALE_P5_TO,
                    $T,
                >
            ),
            (
                MASS_EXPONENT, MASS_SCALE_P10_FROM, MASS_SCALE_P10_TO,
                LENGTH_EXPONENT, LENGTH_SCALE_P10_FROM, LENGTH_SCALE_P10_TO,
                TIME_EXPONENT, TIME_SCALE_P2_FROM, TIME_SCALE_P3_FROM, TIME_SCALE_P5_FROM, 
                               TIME_SCALE_P2_TO, TIME_SCALE_P3_TO, TIME_SCALE_P5_TO,
            ),
            ($rescale_fn, $T),
        );
    };
}

define_int_rescale!(rescale_i64, i64);

define_min_max_scale!(min_mass_scale, <);
define_min_max_scale!(max_mass_scale, >);
define_min_max_scale!(min_length_scale, <);
define_min_max_scale!(max_length_scale, >);

#[macro_export]
macro_rules! define_min_max_composite_scale {
    (2, 3, 5, $fn:ident, $factor_fn:ident, $exponent1:ident, $exponent2:ident, $op:tt) => {
        _define_min_max_composite_scale!(
            // variadic template parameters (prime scales)
            (p2_1: i8, p3_1: i8, p5_1: i8, p2_2: i8, p3_2: i8, p5_2: i8),
            // variadic block for defer_to_second (time_exponent_1 = 0) - entire match arms
            (2 => p2_2, 3 => p3_2, 5 => p5_2, _ => 0),
            // variadic block for defer_to_first (time_exponent_2 = 0) - entire match arms
            (2 => p2_1, 3 => p3_1, 5 => p5_1, _ => 0),
            // variadic block for compare_scales (both non-zero) - let statements and match arms
            (let (num1, den1) = $factor_fn(0, 0, 0, p2_1, p3_1, p5_1, 1); let (num2, den2) = $factor_fn(0, 0, 0, p2_2, p3_2, p5_2, 1);),
            (if num1 * den2 $op num2 * den1),
            (2 => p2_1, 3 => p3_1, 5 => p5_1, _ => 0),
            (2 => p2_2, 3 => p3_2, 5 => p5_2, _ => 0),
            // other compile-time parameters
            $fn, $factor_fn, $exponent1, $exponent2, $op
        );
    }
}

define_min_max_composite_scale!(2, 3, 5, min_time_scale, time_scale_factor, time_exponent1, time_exponent2, <);
define_min_max_composite_scale!(2, 3, 5, max_time_scale, time_scale_factor, time_exponent1, time_exponent2, >);

#[macro_export]
macro_rules! define_arithmetic {
    ($rescale_behavior:ident, $T:ty, $rescale_fn:ident) => {
        _define_arithmetic!(
            // single dimension, single scale
            (const MASS_EXPONENT: i8, const MASS_SCALE_P10: i8,
            const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10: i8,
            const TIME_EXPONENT: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8),
            // single dimension, multiple scales
            (const MASS_EXPONENT: i8, const MASS_SCALE_P10_1: i8, const MASS_SCALE_P10_2: i8,
            const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10_1: i8, const LENGTH_SCALE_P10_2: i8,
            const TIME_EXPONENT: i8, const TIME_SCALE_P2_1: i8, const TIME_SCALE_P3_1: i8, const TIME_SCALE_P5_1: i8,
                                     const TIME_SCALE_P2_2: i8, const TIME_SCALE_P3_2: i8, const TIME_SCALE_P5_2: i8),
            // multiple dimension, single scale
            (const MASS_EXPONENT_1: i8, const MASS_EXPONENT_2: i8, const MASS_SCALE_P10: i8,
            const LENGTH_EXPONENT_1: i8, const LENGTH_EXPONENT_2: i8, const LENGTH_SCALE_P10: i8,
            const TIME_EXPONENT_1: i8, const TIME_EXPONENT_2: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8),
            // multiple dimension, multiple scales
            (const MASS_EXPONENT_1: i8, const MASS_SCALE_P10_1: i8,
            const LENGTH_EXPONENT_1: i8, const LENGTH_SCALE_P10_1: i8,
            const TIME_EXPONENT_1: i8, const TIME_SCALE_P2_1: i8, const TIME_SCALE_P3_1: i8, const TIME_SCALE_P5_1: i8,
            const MASS_EXPONENT_2: i8, const MASS_SCALE_P10_2: i8,
            const LENGTH_EXPONENT_2: i8, const LENGTH_SCALE_P10_2: i8,
            const TIME_EXPONENT_2: i8, const TIME_SCALE_P2_2: i8, const TIME_SCALE_P3_2: i8, const TIME_SCALE_P5_2: i8,),
            // inversion where clauses
            ((): IsI8<{ -MASS_EXPONENT }>,
            (): IsI8<{ -LENGTH_EXPONENT }>,
            (): IsI8<{ -TIME_EXPONENT }>),
            // add min scale where clauses
            ((): IsI8<{ min_mass_scale(MASS_EXPONENT, MASS_SCALE_P10_1, MASS_EXPONENT, MASS_SCALE_P10_2) }>,
            (): IsI8<{ min_length_scale(LENGTH_EXPONENT, LENGTH_SCALE_P10_1, LENGTH_EXPONENT, LENGTH_SCALE_P10_2) }>,
            (): IsI8<{ min_time_scale(2, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                            TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsI8<{ min_time_scale(3, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                            TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsI8<{ min_time_scale(5, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                            TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>),
            // mul min scale where clauses
            ((): IsI8<{ min_mass_scale(MASS_EXPONENT_1, MASS_SCALE_P10_1, MASS_EXPONENT_2, MASS_SCALE_P10_2) }>,
            (): IsI8<{ min_length_scale(LENGTH_EXPONENT_1, LENGTH_SCALE_P10_1, LENGTH_EXPONENT_2, LENGTH_SCALE_P10_2) }>,
            (): IsI8<{ min_time_scale(2, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                            TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsI8<{ min_time_scale(3, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                            TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsI8<{ min_time_scale(5, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                            TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>),
            // mul output dimension where clauses
            ((): IsI8<{ MASS_EXPONENT_1 + MASS_EXPONENT_2 }>,
            (): IsI8<{ LENGTH_EXPONENT_1 + LENGTH_EXPONENT_2 }>,
            (): IsI8<{ TIME_EXPONENT_1 + TIME_EXPONENT_2 }>),
            // div output dimension where clauses
            ((): IsI8<{ MASS_EXPONENT_1 - MASS_EXPONENT_2 }>,
            (): IsI8<{ LENGTH_EXPONENT_1 - LENGTH_EXPONENT_2 }>,
            (): IsI8<{ TIME_EXPONENT_1 - TIME_EXPONENT_2 }>),
            // other parameters
            $rescale_behavior, $T, rescale_fn
        );
    }
}

#[cfg(feature = "strict")]
define_arithmetic!(Strict, f64, rescale_f64);

// Default if no feature is specified
#[cfg(not(any(
    feature = "strict",
    feature = "smaller_wins",
    feature = "left_hand_wins"
)))]
define_arithmetic!(Strict, f64, rescale_f64);

define_display_traits!(
    (const MASS_EXPONENT: i8, const MASS_SCALE_P10: i8,
    const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10: i8,
    const TIME_EXPONENT: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8),
    (MASS_EXPONENT, MASS_SCALE_P10,
    LENGTH_EXPONENT, LENGTH_SCALE_P10,
    TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5)
);