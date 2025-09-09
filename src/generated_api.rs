use crate::IsI8;
use crate::scale_conversion::*;
use crate::generated_quantity_type::*;
use crate::print::prettyprint::*;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt;

define_composite_scale_factor!(
    // params
    (p2_from: i8, p3_from: i8, p5_from: i8, p2_to: i8, p3_to: i8, p5_to: i8),
    // since a later clause depends on exponent, we need to pass it in as a parameter
    exponent,
    // one branch pow expressions
    (
        let (num2, den2) = pow2(p2_from as i32 - p2_to as i32);
        let (num3, den3) = pow3(p3_from as i32 - p3_to as i32);
        let (num5, den5) = pow5(p5_from as i32 - p5_to as i32);
    ),
    // default branch pow expressions
    (
        let (num2, den2) = pow2((p2_from as i32 - p2_to as i32) * exponent as i32);
        let (num3, den3) = pow3((p3_from as i32 - p3_to as i32) * exponent as i32);
        let (num5, den5) = pow5((p5_from as i32 - p5_to as i32) * exponent as i32);
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
define_composite_scale_factor!(
    // params
    (p2_from: i8, p3_from: i8, p5_from: i8, pi_to: i8, p2_to: i8, p3_to: i8, p5_to: i8, pi_from: i8),
    // since a later clause depends on exponent, we need to pass it in as a parameter
    exponent,
    // one branch pow expressions
    (
        let (num2, den2) = pow2(p2_from as i32 - p2_to as i32);
        let (num3, den3) = pow3(p3_from as i32 - p3_to as i32);
        let (num5, den5) = pow5(p5_from as i32 - p5_to as i32);
        let (num_pi, den_pi) = powPi(pi_from as i32 - pi_to as i32);
    ),
    // default branch pow expressions
    (
        let (num2, den2) = pow2((p2_from as i32 - p2_to as i32) * exponent as i32);
        let (num3, den3) = pow3((p3_from as i32 - p3_to as i32) * exponent as i32);
        let (num5, den5) = pow5((p5_from as i32 - p5_to as i32) * exponent as i32);
        let (num_pi, den_pi) = powPi((pi_from as i32 - pi_to as i32) * exponent as i32);
    ),
    // numerator expression
    (
        num2 as i128 * num3 as i128 * num5 as i128 * num_pi as i128
    ),
    // denominator expression
    (
        den2 as i128 * den3 as i128 * den5 as i128 * den_pi as i128
    ),
    angle_scale_factor,
);
define_aggregate_scale_factor!(
    // params
    (
        mass_exponent: i8,
        mass_scale_p10_from: i8, mass_scale_p10_to: i8,
        length_exponent: i8,
        length_scale_p10_from: i8, length_scale_p10_to: i8,
        time_exponent: i8,
        time_scale_p2_from: i8, time_scale_p2_to: i8,
        time_scale_p3_from: i8, time_scale_p3_to: i8,
        time_scale_p5_from: i8, time_scale_p5_to: i8,
        current_exponent: i8,
        current_scale_p10_from: i8, current_scale_p10_to: i8,
        temperature_exponent: i8,
        temperature_scale_p10_from: i8, temperature_scale_p10_to: i8,
        amount_exponent: i8,
        amount_scale_p10_from: i8, amount_scale_p10_to: i8,
        luminosity_exponent: i8,
        luminosity_scale_p10_from: i8, luminosity_scale_p10_to: i8,
        angle_exponent: i8,
        angle_scale_p2_from: i8, angle_scale_p2_to: i8,
        angle_scale_p3_from: i8, angle_scale_p3_to: i8,
        angle_scale_p5_from: i8, angle_scale_p5_to: i8,
        angle_scale_pi_from: i8, angle_scale_pi_to: i8
    ),
    // diff expressions
    (
        let diff_mass_p10 = (mass_scale_p10_from as i32 - mass_scale_p10_to as i32) * mass_exponent as i32;
        let diff_length_p10 = (length_scale_p10_from as i32 - length_scale_p10_to as i32) * length_exponent as i32;
        let diff_time_p2 = (time_scale_p2_from as i32 - time_scale_p2_to as i32) * time_exponent as i32;
        let diff_time_p3 = (time_scale_p3_from as i32 - time_scale_p3_to as i32) * time_exponent as i32;
        let diff_time_p5 = (time_scale_p5_from as i32 - time_scale_p5_to as i32) * time_exponent as i32;
        let diff_current_p10 = (current_scale_p10_from as i32 - current_scale_p10_to as i32) * current_exponent as i32;
        let diff_temperature_p10 = (temperature_scale_p10_from as i32 - temperature_scale_p10_to as i32) * temperature_exponent as i32;
        let diff_amount_p10 = (amount_scale_p10_from as i32 - amount_scale_p10_to as i32) * amount_exponent as i32;
        let diff_luminosity_p10 = (luminosity_scale_p10_from as i32 - luminosity_scale_p10_to as i32) * luminosity_exponent as i32;
        let diff_angle_p2 = (angle_scale_p2_from as i32 - angle_scale_p2_to as i32) * angle_exponent as i32;
        let diff_angle_p3 = (angle_scale_p3_from as i32 - angle_scale_p3_to as i32) * angle_exponent as i32;
        let diff_angle_p5 = (angle_scale_p5_from as i32 - angle_scale_p5_to as i32) * angle_exponent as i32;
        let diff_angle_pi = (angle_scale_pi_from as i32 - angle_scale_pi_to as i32) * angle_exponent as i32;
    ),
    // pow expressions
    (
        let (num2, den2) = pow2(diff_time_p2 + diff_angle_p2);
        let (num3, den3) = pow3(diff_time_p3 + diff_angle_p3);
        let (num5, den5) = pow5(diff_time_p5 + diff_angle_p5);
        let (num10, den10) = pow10(diff_mass_p10 + diff_length_p10 + diff_current_p10 + diff_temperature_p10 + diff_amount_p10 + diff_luminosity_p10);
        let (num_pi, den_pi) = powPi(diff_angle_pi + diff_angle_pi + diff_angle_pi + diff_angle_pi + diff_angle_pi);
    ),
    // num and den expressions
    (num2 * num3 * num5 * num10 * num_pi),
    (den2 * den3 * den5 * den10 * den_pi),
);

define_aggregate_scale_factor_float!(
    // params
    (
        mass_exponent: i8,
        mass_scale_p10_from: i8, mass_scale_p10_to: i8,
        length_exponent: i8,
        length_scale_p10_from: i8, length_scale_p10_to: i8,
        time_exponent: i8,
        time_scale_p2_from: i8, time_scale_p2_to: i8,
        time_scale_p3_from: i8, time_scale_p3_to: i8,
        time_scale_p5_from: i8, time_scale_p5_to: i8,
        current_exponent: i8,
        current_scale_p10_from: i8, current_scale_p10_to: i8,
        temperature_exponent: i8,
        temperature_scale_p10_from: i8, temperature_scale_p10_to: i8,
        amount_exponent: i8,
        amount_scale_p10_from: i8, amount_scale_p10_to: i8,
        luminosity_exponent: i8,
        luminosity_scale_p10_from: i8, luminosity_scale_p10_to: i8,
        angle_exponent: i8,
        angle_scale_p2_from: i8, angle_scale_p2_to: i8,
        angle_scale_p3_from: i8, angle_scale_p3_to: i8,
        angle_scale_p5_from: i8, angle_scale_p5_to: i8,
        angle_scale_pi_from: i8, angle_scale_pi_to: i8
    ),
    // diff expressions
    (
        let diff_mass_p10 = (mass_scale_p10_from as i32 - mass_scale_p10_to as i32) * mass_exponent as i32;
        let diff_length_p10 = (length_scale_p10_from as i32 - length_scale_p10_to as i32) * length_exponent as i32;
        let diff_time_p2 = (time_scale_p2_from as i32 - time_scale_p2_to as i32) * time_exponent as i32;
        let diff_time_p3 = (time_scale_p3_from as i32 - time_scale_p3_to as i32) * time_exponent as i32;
        let diff_time_p5 = (time_scale_p5_from as i32 - time_scale_p5_to as i32) * time_exponent as i32;
        let diff_current_p10 = (current_scale_p10_from as i32 - current_scale_p10_to as i32) * current_exponent as i32;
        let diff_temperature_p10 = (temperature_scale_p10_from as i32 - temperature_scale_p10_to as i32) * temperature_exponent as i32;
        let diff_amount_p10 = (amount_scale_p10_from as i32 - amount_scale_p10_to as i32) * amount_exponent as i32;
        let diff_luminosity_p10 = (luminosity_scale_p10_from as i32 - luminosity_scale_p10_to as i32) * luminosity_exponent as i32;
        let diff_angle_p2 = (angle_scale_p2_from as i32 - angle_scale_p2_to as i32) * angle_exponent as i32;
        let diff_angle_p3 = (angle_scale_p3_from as i32 - angle_scale_p3_to as i32) * angle_exponent as i32;
        let diff_angle_p5 = (angle_scale_p5_from as i32 - angle_scale_p5_to as i32) * angle_exponent as i32;
        let diff_angle_pi = (angle_scale_pi_from as i32 - angle_scale_pi_to as i32) * angle_exponent as i32;
    ),
    // pow expressions
    (
        let pow_2 = (2 as f64).powi(diff_time_p2 + diff_angle_p2);
        let pow_3 = (3 as f64).powi(diff_time_p3 + diff_angle_p3);
        let pow_5 = (5 as f64).powi(diff_time_p5 + diff_angle_p5);
        let pow_10 = (10 as f64).powi(diff_mass_p10 + diff_length_p10 + diff_current_p10 + diff_temperature_p10 + diff_amount_p10 + diff_luminosity_p10);
        let pow_pi = (std::f64::consts::PI).powi(diff_angle_pi);
    ),
    // final expression
    (pow_2 * pow_3 * pow_5 * pow_10 * pow_pi),
);

macro_rules! define_float_rescale {
    ($rescale_fn:ident, $T:ty) => {
        _define_float_rescale!(
            (
                const MASS_EXPONENT: i8,
                const MASS_SCALE_P10_FROM: i8, const MASS_SCALE_P10_TO: i8,
                const LENGTH_EXPONENT: i8,
                const LENGTH_SCALE_P10_FROM: i8, const LENGTH_SCALE_P10_TO: i8,
                const TIME_EXPONENT: i8,
                const TIME_SCALE_P2_FROM: i8, const TIME_SCALE_P2_TO: i8,
                const TIME_SCALE_P3_FROM: i8, const TIME_SCALE_P3_TO: i8,
                const TIME_SCALE_P5_FROM: i8, const TIME_SCALE_P5_TO: i8,
                const CURRENT_EXPONENT: i8,
                const CURRENT_SCALE_P10_FROM: i8, const CURRENT_SCALE_P10_TO: i8,
                const TEMPERATURE_EXPONENT: i8,
                const TEMPERATURE_SCALE_P10_FROM: i8, const TEMPERATURE_SCALE_P10_TO: i8,
                const AMOUNT_EXPONENT: i8,
                const AMOUNT_SCALE_P10_FROM: i8, const AMOUNT_SCALE_P10_TO: i8,
                const LUMINOSITY_EXPONENT: i8,
                const LUMINOSITY_SCALE_P10_FROM: i8, const LUMINOSITY_SCALE_P10_TO: i8,
                const ANGLE_EXPONENT: i8,
                const ANGLE_SCALE_P2_FROM: i8, const ANGLE_SCALE_P2_TO: i8,
                const ANGLE_SCALE_P3_FROM: i8, const ANGLE_SCALE_P3_TO: i8,
                const ANGLE_SCALE_P5_FROM: i8, const ANGLE_SCALE_P5_TO: i8,
                const ANGLE_SCALE_PI_FROM: i8, const ANGLE_SCALE_PI_TO: i8
            ),
            (
                Quantity<
                    MASS_EXPONENT,
                    MASS_SCALE_P10_FROM,
                    LENGTH_EXPONENT,
                    LENGTH_SCALE_P10_FROM,
                    TIME_EXPONENT,
                    TIME_SCALE_P2_FROM,
                    TIME_SCALE_P3_FROM,
                    TIME_SCALE_P5_FROM,
                    CURRENT_EXPONENT,
                    CURRENT_SCALE_P10_FROM,
                    TEMPERATURE_EXPONENT,
                    TEMPERATURE_SCALE_P10_FROM,
                    AMOUNT_EXPONENT,
                    AMOUNT_SCALE_P10_FROM,
                    LUMINOSITY_EXPONENT,
                    LUMINOSITY_SCALE_P10_FROM,
                    ANGLE_EXPONENT,
                    ANGLE_SCALE_P2_FROM,
                    ANGLE_SCALE_P3_FROM,
                    ANGLE_SCALE_P5_FROM,
                    ANGLE_SCALE_PI_FROM,
                    $T,
                >
            ),
            (
                Quantity<
                    MASS_EXPONENT,
                    MASS_SCALE_P10_TO,
                    LENGTH_EXPONENT,
                    LENGTH_SCALE_P10_TO,
                    TIME_EXPONENT,
                    TIME_SCALE_P2_TO,
                    TIME_SCALE_P3_TO,
                    TIME_SCALE_P5_TO,
                    CURRENT_EXPONENT,
                    CURRENT_SCALE_P10_TO,
                    TEMPERATURE_EXPONENT,
                    TEMPERATURE_SCALE_P10_TO,
                    AMOUNT_EXPONENT,
                    AMOUNT_SCALE_P10_TO,
                    LUMINOSITY_EXPONENT,
                    LUMINOSITY_SCALE_P10_TO,
                    ANGLE_EXPONENT,
                    ANGLE_SCALE_P2_TO,
                    ANGLE_SCALE_P3_TO,
                    ANGLE_SCALE_P5_TO,
                    ANGLE_SCALE_PI_TO,
                    $T,
                >
            ),
            (
                MASS_EXPONENT,
                MASS_SCALE_P10_FROM, MASS_SCALE_P10_TO,
                LENGTH_EXPONENT,
                LENGTH_SCALE_P10_FROM, LENGTH_SCALE_P10_TO,
                TIME_EXPONENT,
                TIME_SCALE_P2_FROM, TIME_SCALE_P2_TO,
                TIME_SCALE_P3_FROM, TIME_SCALE_P3_TO,
                TIME_SCALE_P5_FROM, TIME_SCALE_P5_TO,
                CURRENT_EXPONENT,
                CURRENT_SCALE_P10_FROM, CURRENT_SCALE_P10_TO,
                TEMPERATURE_EXPONENT,
                TEMPERATURE_SCALE_P10_FROM, TEMPERATURE_SCALE_P10_TO,
                AMOUNT_EXPONENT,
                AMOUNT_SCALE_P10_FROM, AMOUNT_SCALE_P10_TO,
                LUMINOSITY_EXPONENT,
                LUMINOSITY_SCALE_P10_FROM, LUMINOSITY_SCALE_P10_TO,
                ANGLE_EXPONENT,
                ANGLE_SCALE_P2_FROM, ANGLE_SCALE_P2_TO,
                ANGLE_SCALE_P3_FROM, ANGLE_SCALE_P3_TO,
                ANGLE_SCALE_P5_FROM, ANGLE_SCALE_P5_TO,
                ANGLE_SCALE_PI_FROM, ANGLE_SCALE_PI_TO
            ),
            $rescale_fn, $T,
        );
    };
    
}
macro_rules! define_int_rescale {
    ($rescale_fn:ident, $T:ty) => {
        _define_int_rescale!(
            (
                const MASS_EXPONENT: i8,
                const MASS_SCALE_P10_FROM: i8, const MASS_SCALE_P10_TO: i8,
                const LENGTH_EXPONENT: i8,
                const LENGTH_SCALE_P10_FROM: i8, const LENGTH_SCALE_P10_TO: i8,
                const TIME_EXPONENT: i8,
                const TIME_SCALE_P2_FROM: i8, const TIME_SCALE_P2_TO: i8,
                const TIME_SCALE_P3_FROM: i8, const TIME_SCALE_P3_TO: i8,
                const TIME_SCALE_P5_FROM: i8, const TIME_SCALE_P5_TO: i8,
                const CURRENT_EXPONENT: i8,
                const CURRENT_SCALE_P10_FROM: i8, const CURRENT_SCALE_P10_TO: i8,
                const TEMPERATURE_EXPONENT: i8,
                const TEMPERATURE_SCALE_P10_FROM: i8, const TEMPERATURE_SCALE_P10_TO: i8,
                const AMOUNT_EXPONENT: i8,
                const AMOUNT_SCALE_P10_FROM: i8, const AMOUNT_SCALE_P10_TO: i8,
                const LUMINOSITY_EXPONENT: i8,
                const LUMINOSITY_SCALE_P10_FROM: i8, const LUMINOSITY_SCALE_P10_TO: i8,
                const ANGLE_EXPONENT: i8,
                const ANGLE_SCALE_P2_FROM: i8, const ANGLE_SCALE_P2_TO: i8,
                const ANGLE_SCALE_P3_FROM: i8, const ANGLE_SCALE_P3_TO: i8,
                const ANGLE_SCALE_P5_FROM: i8, const ANGLE_SCALE_P5_TO: i8,
                const ANGLE_SCALE_PI_FROM: i8, const ANGLE_SCALE_PI_TO: i8
            ),
            (
                Quantity<
                    MASS_EXPONENT,
                    MASS_SCALE_P10_FROM,
                    LENGTH_EXPONENT,
                    LENGTH_SCALE_P10_FROM,
                    TIME_EXPONENT,
                    TIME_SCALE_P2_FROM,
                    TIME_SCALE_P3_FROM,
                    TIME_SCALE_P5_FROM,
                    CURRENT_EXPONENT,
                    CURRENT_SCALE_P10_FROM,
                    TEMPERATURE_EXPONENT,
                    TEMPERATURE_SCALE_P10_FROM,
                    AMOUNT_EXPONENT,
                    AMOUNT_SCALE_P10_FROM,
                    LUMINOSITY_EXPONENT,
                    LUMINOSITY_SCALE_P10_FROM,
                    ANGLE_EXPONENT,
                    ANGLE_SCALE_P2_FROM,
                    ANGLE_SCALE_P3_FROM,
                    ANGLE_SCALE_P5_FROM,
                    ANGLE_SCALE_PI_FROM,
                    $T,
                >
            ),
            (
                Quantity<
                    MASS_EXPONENT,
                    MASS_SCALE_P10_TO,
                    LENGTH_EXPONENT,
                    LENGTH_SCALE_P10_TO,
                    TIME_EXPONENT,
                    TIME_SCALE_P2_TO,
                    TIME_SCALE_P3_TO,
                    TIME_SCALE_P5_TO,
                    CURRENT_EXPONENT,
                    CURRENT_SCALE_P10_TO,
                    TEMPERATURE_EXPONENT,
                    TEMPERATURE_SCALE_P10_TO,
                    AMOUNT_EXPONENT,
                    AMOUNT_SCALE_P10_TO,
                    LUMINOSITY_EXPONENT,
                    LUMINOSITY_SCALE_P10_TO,
                    ANGLE_EXPONENT,
                    ANGLE_SCALE_P2_TO,
                    ANGLE_SCALE_P3_TO,
                    ANGLE_SCALE_P5_TO,
                    ANGLE_SCALE_PI_TO,
                    $T,
                >
            ),
            (
                MASS_EXPONENT,
                MASS_SCALE_P10_FROM, MASS_SCALE_P10_TO,
                LENGTH_EXPONENT,
                LENGTH_SCALE_P10_FROM, LENGTH_SCALE_P10_TO,
                TIME_EXPONENT,
                TIME_SCALE_P2_FROM, TIME_SCALE_P2_TO,
                TIME_SCALE_P3_FROM, TIME_SCALE_P3_TO,
                TIME_SCALE_P5_FROM, TIME_SCALE_P5_TO,
                CURRENT_EXPONENT,
                CURRENT_SCALE_P10_FROM, CURRENT_SCALE_P10_TO,
                TEMPERATURE_EXPONENT,
                TEMPERATURE_SCALE_P10_FROM, TEMPERATURE_SCALE_P10_TO,
                AMOUNT_EXPONENT,
                AMOUNT_SCALE_P10_FROM, AMOUNT_SCALE_P10_TO,
                LUMINOSITY_EXPONENT,
                LUMINOSITY_SCALE_P10_FROM, LUMINOSITY_SCALE_P10_TO,
                ANGLE_EXPONENT,
                ANGLE_SCALE_P2_FROM, ANGLE_SCALE_P2_TO,
                ANGLE_SCALE_P3_FROM, ANGLE_SCALE_P3_TO,
                ANGLE_SCALE_P5_FROM, ANGLE_SCALE_P5_TO,
                ANGLE_SCALE_PI_FROM, ANGLE_SCALE_PI_TO
            ),
            $rescale_fn, $T,
        );
    };
}
define_float_rescale!(rescale_f64, f64);

define_int_rescale!(rescale_i64, i64);

define_min_max_scale!(min_mass_scale, <);
define_min_max_scale!(max_mass_scale, >);
define_min_max_scale!(min_length_scale, <);
define_min_max_scale!(max_length_scale, >);
define_min_max_scale!(min_current_scale, <);
define_min_max_scale!(max_current_scale, >);
define_min_max_scale!(min_temperature_scale, <);
define_min_max_scale!(max_temperature_scale, >);
define_min_max_scale!(min_amount_scale, <);
define_min_max_scale!(max_amount_scale, >);
define_min_max_scale!(min_luminosity_scale, <);
define_min_max_scale!(max_luminosity_scale, >);

#[macro_export]
macro_rules! define_min_max_time_scale {
    ($fn:ident, $factor_fn:ident, $op:tt) => {
        _define_min_max_composite_scale!(
            // variadic template parameters (prime scales)
            (p2_1: i8, p3_1: i8, p5_1: i8),
            (p2_2: i8, p3_2: i8, p5_2: i8),
            // variadic block for defer_to_second (exponent_1 = 0) - entire match arms
            (2 => p2_2, 3 => p3_2, 5 => p5_2, _ => 0),
            // variadic block for defer_to_first (exponent_2 = 0) - entire match arms
            (2 => p2_1, 3 => p3_1, 5 => p5_1, _ => 0),
            // variadic block for compare_scales (both non-zero) - let statements and match arms
            $factor_fn(0, 0, 0, p2_1, p3_1, p5_1, 1),
            $factor_fn(0, 0, 0, p2_2, p3_2, p5_2, 1),
            // other compile-time parameters
            $fn, exponent1, exponent2, $op
        );
    }
}

#[macro_export]
macro_rules! define_min_max_angle_scale {
    ($fn:ident, $factor_fn:ident, $op:tt) => {
        _define_min_max_composite_scale!(
            // variadic template parameters (prime scales)
            (p2_1: i8, p3_1: i8, p5_1: i8, pi_1: i8),
            (p2_2: i8, p3_2: i8, p5_2: i8, pi_2: i8),
            // variadic block for defer_to_second (exponent_1 = 0) - entire match arms
            (2 => p2_2, 3 => p3_2, 5 => p5_2, i8::MAX => pi_2, _ => 0),
            // variadic block for defer_to_first (exponent_2 = 0) - entire match arms
            (2 => p2_1, 3 => p3_1, 5 => p5_1, i8::MAX => pi_1, _ => 0),
            // variadic block for compare_scales (both non-zero) - let statements and match arms
            $factor_fn(0, 0, 0, 0, p2_1, p3_1, p5_1, pi_1, 1),
            $factor_fn(0, 0, 0, 0, p2_2, p3_2, p5_2, pi_2, 1),
            // other compile-time parameters
            $fn, exponent1, exponent2, $op
        );
    }
}

define_min_max_time_scale!(min_time_scale, time_scale_factor, <);
define_min_max_time_scale!(max_time_scale, time_scale_factor, >);
define_min_max_angle_scale!(min_angle_scale, angle_scale_factor, <);
define_min_max_angle_scale!(max_angle_scale, angle_scale_factor, >);

#[macro_export]
macro_rules! define_arithmetic {
    ($rescale_behavior:ident, $T:ty, $rescale_fn:ident) => {
        _define_arithmetic!(
            // single dimension, single scale
            (
            const MASS_EXPONENT: i8,
            const MASS_SCALE_P10: i8,
            const LENGTH_EXPONENT: i8,
            const LENGTH_SCALE_P10: i8,
            const TIME_EXPONENT: i8,
            const TIME_SCALE_P2: i8,
            const TIME_SCALE_P3: i8,
            const TIME_SCALE_P5: i8,
            const CURRENT_EXPONENT: i8,
            const CURRENT_SCALE_P10: i8,
            const TEMPERATURE_EXPONENT: i8,
            const TEMPERATURE_SCALE_P10: i8,
            const AMOUNT_EXPONENT: i8,
            const AMOUNT_SCALE_P10: i8,
            const LUMINOSITY_EXPONENT: i8,
            const LUMINOSITY_SCALE_P10: i8,
            const ANGLE_EXPONENT: i8,
            const ANGLE_SCALE_P2: i8,
            const ANGLE_SCALE_P3: i8,
            const ANGLE_SCALE_P5: i8,
            const ANGLE_SCALE_PI: i8
        ),
            // single dimension, multiple scales
            (
            const MASS_EXPONENT: i8,
            const MASS_SCALE_P10_1: i8, const MASS_SCALE_P10_2: i8,
            const LENGTH_EXPONENT: i8,
            const LENGTH_SCALE_P10_1: i8, const LENGTH_SCALE_P10_2: i8,
            const TIME_EXPONENT: i8,
            const TIME_SCALE_P2_1: i8, const TIME_SCALE_P2_2: i8,
            const TIME_SCALE_P3_1: i8, const TIME_SCALE_P3_2: i8,
            const TIME_SCALE_P5_1: i8, const TIME_SCALE_P5_2: i8,
            const CURRENT_EXPONENT: i8,
            const CURRENT_SCALE_P10_1: i8, const CURRENT_SCALE_P10_2: i8,
            const TEMPERATURE_EXPONENT: i8,
            const TEMPERATURE_SCALE_P10_1: i8, const TEMPERATURE_SCALE_P10_2: i8,
            const AMOUNT_EXPONENT: i8,
            const AMOUNT_SCALE_P10_1: i8, const AMOUNT_SCALE_P10_2: i8,
            const LUMINOSITY_EXPONENT: i8,
            const LUMINOSITY_SCALE_P10_1: i8, const LUMINOSITY_SCALE_P10_2: i8,
            const ANGLE_EXPONENT: i8,
            const ANGLE_SCALE_P2_1: i8, const ANGLE_SCALE_P2_2: i8,
            const ANGLE_SCALE_P3_1: i8, const ANGLE_SCALE_P3_2: i8,
            const ANGLE_SCALE_P5_1: i8, const ANGLE_SCALE_P5_2: i8,
            const ANGLE_SCALE_PI_1: i8, const ANGLE_SCALE_PI_2: i8
        ),
            // multiple dimension, single scale
            (
            const MASS_EXPONENT_1: i8, const MASS_EXPONENT_2: i8,
            const MASS_SCALE_P10: i8,
            const LENGTH_EXPONENT_1: i8, const LENGTH_EXPONENT_2: i8,
            const LENGTH_SCALE_P10: i8,
            const TIME_EXPONENT_1: i8, const TIME_EXPONENT_2: i8,
            const TIME_SCALE_P2: i8,
            const TIME_SCALE_P3: i8,
            const TIME_SCALE_P5: i8,
            const CURRENT_EXPONENT_1: i8, const CURRENT_EXPONENT_2: i8,
            const CURRENT_SCALE_P10: i8,
            const TEMPERATURE_EXPONENT_1: i8, const TEMPERATURE_EXPONENT_2: i8,
            const TEMPERATURE_SCALE_P10: i8,
            const AMOUNT_EXPONENT_1: i8, const AMOUNT_EXPONENT_2: i8,
            const AMOUNT_SCALE_P10: i8,
            const LUMINOSITY_EXPONENT_1: i8, const LUMINOSITY_EXPONENT_2: i8,
            const LUMINOSITY_SCALE_P10: i8,
            const ANGLE_EXPONENT_1: i8, const ANGLE_EXPONENT_2: i8,
            const ANGLE_SCALE_P2: i8,
            const ANGLE_SCALE_P3: i8,
            const ANGLE_SCALE_P5: i8,
            const ANGLE_SCALE_PI: i8
        ),
            // multiple dimension, multiple scales
            (
            const MASS_EXPONENT_1: i8, const MASS_EXPONENT_2: i8,
            const MASS_SCALE_P10_1: i8, const MASS_SCALE_P10_2: i8,
            const LENGTH_EXPONENT_1: i8, const LENGTH_EXPONENT_2: i8,
            const LENGTH_SCALE_P10_1: i8, const LENGTH_SCALE_P10_2: i8,
            const TIME_EXPONENT_1: i8, const TIME_EXPONENT_2: i8,
            const TIME_SCALE_P2_1: i8, const TIME_SCALE_P2_2: i8,
            const TIME_SCALE_P3_1: i8, const TIME_SCALE_P3_2: i8,
            const TIME_SCALE_P5_1: i8, const TIME_SCALE_P5_2: i8,
            const CURRENT_EXPONENT_1: i8, const CURRENT_EXPONENT_2: i8,
            const CURRENT_SCALE_P10_1: i8, const CURRENT_SCALE_P10_2: i8,
            const TEMPERATURE_EXPONENT_1: i8, const TEMPERATURE_EXPONENT_2: i8,
            const TEMPERATURE_SCALE_P10_1: i8, const TEMPERATURE_SCALE_P10_2: i8,
            const AMOUNT_EXPONENT_1: i8, const AMOUNT_EXPONENT_2: i8,
            const AMOUNT_SCALE_P10_1: i8, const AMOUNT_SCALE_P10_2: i8,
            const LUMINOSITY_EXPONENT_1: i8, const LUMINOSITY_EXPONENT_2: i8,
            const LUMINOSITY_SCALE_P10_1: i8, const LUMINOSITY_SCALE_P10_2: i8,
            const ANGLE_EXPONENT_1: i8, const ANGLE_EXPONENT_2: i8,
            const ANGLE_SCALE_P2_1: i8, const ANGLE_SCALE_P2_2: i8,
            const ANGLE_SCALE_P3_1: i8, const ANGLE_SCALE_P3_2: i8,
            const ANGLE_SCALE_P5_1: i8, const ANGLE_SCALE_P5_2: i8,
            const ANGLE_SCALE_PI_1: i8, const ANGLE_SCALE_PI_2: i8
        ),
            // inversion where clauses
            (
            (): IsI8<{ -MASS_EXPONENT }>,
            (): IsI8<{ -LENGTH_EXPONENT }>,
            (): IsI8<{ -TIME_EXPONENT }>,
            (): IsI8<{ -CURRENT_EXPONENT }>,
            (): IsI8<{ -TEMPERATURE_EXPONENT }>,
            (): IsI8<{ -AMOUNT_EXPONENT }>,
            (): IsI8<{ -LUMINOSITY_EXPONENT }>,
            (): IsI8<{ -ANGLE_EXPONENT }>
        ),
            // add min scale where clauses
            (
            (): IsI8<{ min_mass_scale(MASS_EXPONENT, MASS_SCALE_P10_1, MASS_EXPONENT, MASS_SCALE_P10_2) }>,
            (): IsI8<{ min_length_scale(LENGTH_EXPONENT, LENGTH_SCALE_P10_1, LENGTH_EXPONENT, LENGTH_SCALE_P10_2) }>,
            (): IsI8<{ min_time_scale(2, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsI8<{ min_time_scale(3, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsI8<{ min_time_scale(5, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsI8<{ min_current_scale(CURRENT_EXPONENT, CURRENT_SCALE_P10_1, CURRENT_EXPONENT, CURRENT_SCALE_P10_2) }>,
            (): IsI8<{ min_temperature_scale(TEMPERATURE_EXPONENT, TEMPERATURE_SCALE_P10_1, TEMPERATURE_EXPONENT, TEMPERATURE_SCALE_P10_2) }>,
            (): IsI8<{ min_amount_scale(AMOUNT_EXPONENT, AMOUNT_SCALE_P10_1, AMOUNT_EXPONENT, AMOUNT_SCALE_P10_2) }>,
            (): IsI8<{ min_luminosity_scale(LUMINOSITY_EXPONENT, LUMINOSITY_SCALE_P10_1, LUMINOSITY_EXPONENT, LUMINOSITY_SCALE_P10_2) }>,
            (): IsI8<{ min_angle_scale(2, ANGLE_EXPONENT, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_EXPONENT, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2) }>,
            (): IsI8<{ min_angle_scale(3, ANGLE_EXPONENT, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_EXPONENT, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2) }>,
            (): IsI8<{ min_angle_scale(5, ANGLE_EXPONENT, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_EXPONENT, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2) }>,
            (): IsI8<{ min_angle_scale(0, ANGLE_EXPONENT, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_EXPONENT, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2) }>
        ),
            // mul min scale where clauses
            (
            (): IsI8<{ min_mass_scale(MASS_EXPONENT_1, MASS_SCALE_P10_1, MASS_EXPONENT_2, MASS_SCALE_P10_2) }>,
            (): IsI8<{ min_length_scale(LENGTH_EXPONENT_1, LENGTH_SCALE_P10_1, LENGTH_EXPONENT_2, LENGTH_SCALE_P10_2) }>,
            (): IsI8<{ min_time_scale(2, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsI8<{ min_time_scale(3, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsI8<{ min_time_scale(5, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsI8<{ min_current_scale(CURRENT_EXPONENT_1, CURRENT_SCALE_P10_1, CURRENT_EXPONENT_2, CURRENT_SCALE_P10_2) }>,
            (): IsI8<{ min_temperature_scale(TEMPERATURE_EXPONENT_1, TEMPERATURE_SCALE_P10_1, TEMPERATURE_EXPONENT_2, TEMPERATURE_SCALE_P10_2) }>,
            (): IsI8<{ min_amount_scale(AMOUNT_EXPONENT_1, AMOUNT_SCALE_P10_1, AMOUNT_EXPONENT_2, AMOUNT_SCALE_P10_2) }>,
            (): IsI8<{ min_luminosity_scale(LUMINOSITY_EXPONENT_1, LUMINOSITY_SCALE_P10_1, LUMINOSITY_EXPONENT_2, LUMINOSITY_SCALE_P10_2) }>,
            (): IsI8<{ min_angle_scale(2, ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2) }>,
            (): IsI8<{ min_angle_scale(3, ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2) }>,
            (): IsI8<{ min_angle_scale(5, ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2) }>,
            (): IsI8<{ min_angle_scale(0, ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2) }>
        ),
            // mul output dimension where clauses
            (
            (): IsI8<{ MASS_EXPONENT_1 + MASS_EXPONENT_2 }>,
            (): IsI8<{ LENGTH_EXPONENT_1 + LENGTH_EXPONENT_2 }>,
            (): IsI8<{ TIME_EXPONENT_1 + TIME_EXPONENT_2 }>,
            (): IsI8<{ CURRENT_EXPONENT_1 + CURRENT_EXPONENT_2 }>,
            (): IsI8<{ TEMPERATURE_EXPONENT_1 + TEMPERATURE_EXPONENT_2 }>,
            (): IsI8<{ AMOUNT_EXPONENT_1 + AMOUNT_EXPONENT_2 }>,
            (): IsI8<{ LUMINOSITY_EXPONENT_1 + LUMINOSITY_EXPONENT_2 }>,
            (): IsI8<{ ANGLE_EXPONENT_1 + ANGLE_EXPONENT_2 }>
        ),
            // div output dimension where clauses
            (
            (): IsI8<{ MASS_EXPONENT_1 - MASS_EXPONENT_2 }>,
            (): IsI8<{ LENGTH_EXPONENT_1 - LENGTH_EXPONENT_2 }>,
            (): IsI8<{ TIME_EXPONENT_1 - TIME_EXPONENT_2 }>,
            (): IsI8<{ CURRENT_EXPONENT_1 - CURRENT_EXPONENT_2 }>,
            (): IsI8<{ TEMPERATURE_EXPONENT_1 - TEMPERATURE_EXPONENT_2 }>,
            (): IsI8<{ AMOUNT_EXPONENT_1 - AMOUNT_EXPONENT_2 }>,
            (): IsI8<{ LUMINOSITY_EXPONENT_1 - LUMINOSITY_EXPONENT_2 }>,
            (): IsI8<{ ANGLE_EXPONENT_1 - ANGLE_EXPONENT_2 }>
        ),
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
    (
        const MASS_EXPONENT: i8,
        const MASS_SCALE_P10: i8,
        const LENGTH_EXPONENT: i8,
        const LENGTH_SCALE_P10: i8,
        const TIME_EXPONENT: i8,
        const TIME_SCALE_P2: i8,
        const TIME_SCALE_P3: i8,
        const TIME_SCALE_P5: i8,
        const CURRENT_EXPONENT: i8,
        const CURRENT_SCALE_P10: i8,
        const TEMPERATURE_EXPONENT: i8,
        const TEMPERATURE_SCALE_P10: i8,
        const AMOUNT_EXPONENT: i8,
        const AMOUNT_SCALE_P10: i8,
        const LUMINOSITY_EXPONENT: i8,
        const LUMINOSITY_SCALE_P10: i8,
        const ANGLE_EXPONENT: i8,
        const ANGLE_SCALE_P2: i8,
        const ANGLE_SCALE_P3: i8,
        const ANGLE_SCALE_P5: i8,
        const ANGLE_SCALE_PI: i8,
    ),
    (
        MASS_EXPONENT, MASS_SCALE_P10,
        LENGTH_EXPONENT, LENGTH_SCALE_P10,
        TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
        CURRENT_EXPONENT, CURRENT_SCALE_P10,
        TEMPERATURE_EXPONENT, TEMPERATURE_SCALE_P10,
        AMOUNT_EXPONENT, AMOUNT_SCALE_P10,
        LUMINOSITY_EXPONENT, LUMINOSITY_SCALE_P10,
        ANGLE_EXPONENT, ANGLE_SCALE_P2, ANGLE_SCALE_P3, ANGLE_SCALE_P5, ANGLE_SCALE_PI,
    )
);