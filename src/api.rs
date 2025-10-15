//! API for whippyunits quantities.
//! 
//! This module provides the API implementations for most operations on the 
//! whippyunits [Quantity] type.

use crate::define_aggregate_scale_factor_float;
use crate::define_aggregate_scale_factor_rational;
use crate::define_display_traits;
use crate::print::prettyprint::*;
use crate::quantity_type::*;
use crate::scale_conversion::*;
use crate::IsI16;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

define_aggregate_scale_factor_rational!(
    // params
    (
        scale_p2_from: i16, scale_p3_from: i16, scale_p5_from: i16, scale_pi_from: i16,
        scale_p2_to: i16, scale_p3_to: i16, scale_p5_to: i16, scale_pi_to: i16,
    ),
    // diff expressions
    (
        let diff_scale_p2 = scale_p2_from - scale_p2_to;
        let diff_scale_p3 = scale_p3_from - scale_p3_to;
        let diff_scale_p5 = scale_p5_from - scale_p5_to;
        let diff_scale_pi = scale_pi_from - scale_pi_to;
    ),
    // pow expressions
    (
        let (num2, den2) = pow2(diff_scale_p2 as i32);
        let (num3, den3) = pow3(diff_scale_p3 as i32);
        let (num5, den5) = pow5(diff_scale_p5 as i32);
        let (num_pi, den_pi) = pow_pi(diff_scale_pi as i32);
    ),
    // num and den expressions
    (num2 * num3 * num5 * num_pi),
    (den2 * den3 * den5 * den_pi),
);

define_aggregate_scale_factor_float!(
    // params
    (
        scale_p2_from: i16, scale_p3_from: i16, scale_p5_from: i16, scale_pi_from: i16,
        scale_p2_to: i16, scale_p3_to: i16, scale_p5_to: i16, scale_pi_to: i16,
    ),
    // diff expressions
    (
        let diff_scale_p2 = scale_p2_from - scale_p2_to;
        let diff_scale_p3 = scale_p3_from - scale_p3_to;
        let diff_scale_p5 = scale_p5_from - scale_p5_to;
        let diff_scale_pi = scale_pi_from - scale_pi_to;
    ),
    // pow expressions
    (
        let pow_2 = crate::scale_conversion::pow2_float(diff_scale_p2 as i32);
        let pow_3 = crate::scale_conversion::pow3_float(diff_scale_p3 as i32);
        let pow_5 = crate::scale_conversion::pow5_float(diff_scale_p5 as i32);
        let pow_pi = crate::scale_conversion::pow_pi_float(diff_scale_pi as i32);
    ),
    // final expression
    (pow_2 * pow_3 * pow_5 * pow_pi),
);

#[doc(hidden)]
macro_rules! define_float_rescale {
    ($rescale_fn:ident, $T:ty) => {
        $crate::_define_float_rescale!(
            (
                const MASS_EXPONENT: i16,
                const LENGTH_EXPONENT: i16,
                const TIME_EXPONENT: i16,
                const CURRENT_EXPONENT: i16,
                const TEMPERATURE_EXPONENT: i16,
                const AMOUNT_EXPONENT: i16,
                const LUMINOSITY_EXPONENT: i16,
                const ANGLE_EXPONENT: i16,
                const SCALE_P2_FROM: i16, const SCALE_P2_TO: i16,
                const SCALE_P3_FROM: i16, const SCALE_P3_TO: i16,
                const SCALE_P5_FROM: i16, const SCALE_P5_TO: i16,
                const SCALE_PI_FROM: i16, const SCALE_PI_TO: i16,
            ),
            (
                Quantity<
                    Scale<_2<SCALE_P2_FROM>, _3<SCALE_P3_FROM>, _5<SCALE_P5_FROM>, _Pi<SCALE_PI_FROM>>,
                    Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
                    $T,
                >
            ),
            (
                Quantity<
                    Scale<_2<SCALE_P2_TO>, _3<SCALE_P3_TO>, _5<SCALE_P5_TO>, _Pi<SCALE_PI_TO>>,
                    Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
                    $T,
                >
            ),
            (
                SCALE_P2_FROM, SCALE_P3_FROM, SCALE_P5_FROM, SCALE_PI_FROM,
                SCALE_P2_TO, SCALE_P3_TO, SCALE_P5_TO, SCALE_PI_TO,
            ),
            $rescale_fn, $T,
        );
    };
}

#[doc(hidden)]
macro_rules! define_int_rescale {
    ($rescale_fn:ident, $T:ty) => {
        $crate::_define_int_rescale!(
            (
                const MASS_EXPONENT: i16,
                const LENGTH_EXPONENT: i16,
                const TIME_EXPONENT: i16,
                const CURRENT_EXPONENT: i16,
                const TEMPERATURE_EXPONENT: i16,
                const AMOUNT_EXPONENT: i16,
                const LUMINOSITY_EXPONENT: i16,
                const ANGLE_EXPONENT: i16,
                const SCALE_P2_FROM: i16, const SCALE_P2_TO: i16,
                const SCALE_P3_FROM: i16, const SCALE_P3_TO: i16,
                const SCALE_P5_FROM: i16, const SCALE_P5_TO: i16,
                const SCALE_PI_FROM: i16, const SCALE_PI_TO: i16,
            ),
            (
                Quantity<
                    Scale<_2<SCALE_P2_FROM>, _3<SCALE_P3_FROM>, _5<SCALE_P5_FROM>, _Pi<SCALE_PI_FROM>>,
                    Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
                    $T,
                >
            ),
            (
                Quantity<
                    Scale<_2<SCALE_P2_TO>, _3<SCALE_P3_TO>, _5<SCALE_P5_TO>, _Pi<SCALE_PI_TO>>,
                    Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
                    $T,
                >
            ),
            (
                SCALE_P2_FROM, SCALE_P3_FROM, SCALE_P5_FROM, SCALE_PI_FROM,
                SCALE_P2_TO, SCALE_P3_TO, SCALE_P5_TO, SCALE_PI_TO,
            ),
            $rescale_fn, $T,
        );
    };
}
// Float rescale functions
define_float_rescale!(rescale, f64);
define_float_rescale!(rescale_f64, f64);
define_float_rescale!(rescale_f32, f32);

// Integer rescale functions
define_int_rescale!(rescale_i8, i8);
define_int_rescale!(rescale_i16, i16);
define_int_rescale!(rescale_i32, i32);
define_int_rescale!(rescale_i64, i64);
define_int_rescale!(rescale_i128, i128);

// Unsigned integer rescale functions
define_int_rescale!(rescale_u8, u8);
define_int_rescale!(rescale_u16, u16);
define_int_rescale!(rescale_u32, u32);
define_int_rescale!(rescale_u64, u64);
define_int_rescale!(rescale_u128, u128);

#[macro_export]
#[doc(hidden)]
macro_rules! define_arithmetic_signed {
    ($T:ty, $rescale_fn:ident) => {
        $crate::_define_arithmetic_signed!(
        // single dimension, single scale
        (
            const MASS_EXPONENT: i16,
            const LENGTH_EXPONENT: i16,
            const TIME_EXPONENT: i16,
            const CURRENT_EXPONENT: i16,
            const TEMPERATURE_EXPONENT: i16,
            const AMOUNT_EXPONENT: i16,
            const LUMINOSITY_EXPONENT: i16,
            const ANGLE_EXPONENT: i16,
            const SCALE_P2: i16,
            const SCALE_P3: i16,
            const SCALE_P5: i16,
            const SCALE_PI: i16,
        ),
        // single dimension, multiple scales
        (
            const MASS_EXPONENT: i16,
            const LENGTH_EXPONENT: i16,
            const TIME_EXPONENT: i16,
            const CURRENT_EXPONENT: i16,
            const TEMPERATURE_EXPONENT: i16,
            const AMOUNT_EXPONENT: i16,
            const LUMINOSITY_EXPONENT: i16,
            const ANGLE_EXPONENT: i16,
            const SCALE_P2_1: i16, const SCALE_P3_1: i16, const SCALE_P5_1: i16, const SCALE_PI_1: i16,
            const SCALE_P2_2: i16, const SCALE_P3_2: i16, const SCALE_P5_2: i16, const SCALE_PI_2: i16
        ),

        // multiple dimension, multiple scales
        (
            const MASS_EXPONENT_1: i16, const MASS_EXPONENT_2: i16,
            const LENGTH_EXPONENT_1: i16, const LENGTH_EXPONENT_2: i16,
            const TIME_EXPONENT_1: i16, const TIME_EXPONENT_2: i16,
            const CURRENT_EXPONENT_1: i16, const CURRENT_EXPONENT_2: i16,
            const TEMPERATURE_EXPONENT_1: i16, const TEMPERATURE_EXPONENT_2: i16,
            const AMOUNT_EXPONENT_1: i16, const AMOUNT_EXPONENT_2: i16,
            const LUMINOSITY_EXPONENT_1: i16, const LUMINOSITY_EXPONENT_2: i16,
            const ANGLE_EXPONENT_1: i16, const ANGLE_EXPONENT_2: i16,
            const SCALE_P2_1: i16, const SCALE_P3_1: i16, const SCALE_P5_1: i16, const SCALE_PI_1: i16,
            const SCALE_P2_2: i16, const SCALE_P3_2: i16, const SCALE_P5_2: i16, const SCALE_PI_2: i16
        ),
        // inversion where clauses
        (
            (): IsI16<{ -MASS_EXPONENT }>,
            (): IsI16<{ -LENGTH_EXPONENT }>,
            (): IsI16<{ -TIME_EXPONENT }>,
            (): IsI16<{ -CURRENT_EXPONENT }>,
            (): IsI16<{ -TEMPERATURE_EXPONENT }>,
            (): IsI16<{ -AMOUNT_EXPONENT }>,
            (): IsI16<{ -LUMINOSITY_EXPONENT }>,
            (): IsI16<{ -ANGLE_EXPONENT }>,
            (): IsI16<{ -SCALE_P2 }>,
            (): IsI16<{ -SCALE_P3 }>,
            (): IsI16<{ -SCALE_P5 }>,
            (): IsI16<{ -SCALE_PI }>
        ),
        // add min scale where clauses
        (
            (): IsI16<{ SCALE_P2_1.min(SCALE_P2_2) }>,
            (): IsI16<{ SCALE_P3_1.min(SCALE_P3_2) }>,
            (): IsI16<{ SCALE_P5_1.min(SCALE_P5_2) }>,
            (): IsI16<{ SCALE_PI_1.min(SCALE_PI_2) }>
        ),
        // mul output dimension where clauses
        (
            (): IsI16<{ MASS_EXPONENT_1 + MASS_EXPONENT_2 }>,
            (): IsI16<{ LENGTH_EXPONENT_1 + LENGTH_EXPONENT_2 }>,
            (): IsI16<{ TIME_EXPONENT_1 + TIME_EXPONENT_2 }>,
            (): IsI16<{ CURRENT_EXPONENT_1 + CURRENT_EXPONENT_2 }>,
            (): IsI16<{ TEMPERATURE_EXPONENT_1 + TEMPERATURE_EXPONENT_2 }>,
            (): IsI16<{ AMOUNT_EXPONENT_1 + AMOUNT_EXPONENT_2 }>,
            (): IsI16<{ LUMINOSITY_EXPONENT_1 + LUMINOSITY_EXPONENT_2 }>,
            (): IsI16<{ ANGLE_EXPONENT_1 + ANGLE_EXPONENT_2 }>,
            (): IsI16<{ SCALE_P2_1 + SCALE_P2_2 }>,
            (): IsI16<{ SCALE_P3_1 + SCALE_P3_2 }>,
            (): IsI16<{ SCALE_P5_1 + SCALE_P5_2 }>,
            (): IsI16<{ SCALE_PI_1 + SCALE_PI_2 }>
        ),
        // div output dimension where clauses
        (
            (): IsI16<{ MASS_EXPONENT_1 - MASS_EXPONENT_2 }>,
            (): IsI16<{ LENGTH_EXPONENT_1 - LENGTH_EXPONENT_2 }>,
            (): IsI16<{ TIME_EXPONENT_1 - TIME_EXPONENT_2 }>,
            (): IsI16<{ CURRENT_EXPONENT_1 - CURRENT_EXPONENT_2 }>,
            (): IsI16<{ TEMPERATURE_EXPONENT_1 - TEMPERATURE_EXPONENT_2 }>,
            (): IsI16<{ AMOUNT_EXPONENT_1 - AMOUNT_EXPONENT_2 }>,
            (): IsI16<{ LUMINOSITY_EXPONENT_1 - LUMINOSITY_EXPONENT_2 }>,
            (): IsI16<{ ANGLE_EXPONENT_1 - ANGLE_EXPONENT_2 }>,
            (): IsI16<{ SCALE_P2_1 - SCALE_P2_2 }>,
            (): IsI16<{ SCALE_P3_1 - SCALE_P3_2 }>,
            (): IsI16<{ SCALE_P5_1 - SCALE_P5_2 }>,
            (): IsI16<{ SCALE_PI_1 - SCALE_PI_2 }>
        ),
            // other parameters
            $T, rescale_fn
        );
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_arithmetic {
    ($T:ty, $rescale_fn:ident) => {
        $crate::_define_arithmetic!(
        // single dimension, single scale
        (
            const MASS_EXPONENT: i16,
            const LENGTH_EXPONENT: i16,
            const TIME_EXPONENT: i16,
            const CURRENT_EXPONENT: i16,
            const TEMPERATURE_EXPONENT: i16,
            const AMOUNT_EXPONENT: i16,
            const LUMINOSITY_EXPONENT: i16,
            const ANGLE_EXPONENT: i16,
            const SCALE_P2: i16,
            const SCALE_P3: i16,
            const SCALE_P5: i16,
            const SCALE_PI: i16,
        ),
        // single dimension, multiple scales
        (
            const MASS_EXPONENT: i16,
            const LENGTH_EXPONENT: i16,
            const TIME_EXPONENT: i16,
            const CURRENT_EXPONENT: i16,
            const TEMPERATURE_EXPONENT: i16,
            const AMOUNT_EXPONENT: i16,
            const LUMINOSITY_EXPONENT: i16,
            const ANGLE_EXPONENT: i16,
            const SCALE_P2_1: i16, const SCALE_P3_1: i16, const SCALE_P5_1: i16, const SCALE_PI_1: i16,
            const SCALE_P2_2: i16, const SCALE_P3_2: i16, const SCALE_P5_2: i16, const SCALE_PI_2: i16
        ),

        // multiple dimension, multiple scales
        (
            const MASS_EXPONENT_1: i16, const MASS_EXPONENT_2: i16,
            const LENGTH_EXPONENT_1: i16, const LENGTH_EXPONENT_2: i16,
            const TIME_EXPONENT_1: i16, const TIME_EXPONENT_2: i16,
            const CURRENT_EXPONENT_1: i16, const CURRENT_EXPONENT_2: i16,
            const TEMPERATURE_EXPONENT_1: i16, const TEMPERATURE_EXPONENT_2: i16,
            const AMOUNT_EXPONENT_1: i16, const AMOUNT_EXPONENT_2: i16,
            const LUMINOSITY_EXPONENT_1: i16, const LUMINOSITY_EXPONENT_2: i16,
            const ANGLE_EXPONENT_1: i16, const ANGLE_EXPONENT_2: i16,
            const SCALE_P2_1: i16, const SCALE_P3_1: i16, const SCALE_P5_1: i16, const SCALE_PI_1: i16,
            const SCALE_P2_2: i16, const SCALE_P3_2: i16, const SCALE_P5_2: i16, const SCALE_PI_2: i16
        ),
        // inversion where clauses
        (
            (): IsI16<{ -MASS_EXPONENT }>,
            (): IsI16<{ -LENGTH_EXPONENT }>,
            (): IsI16<{ -TIME_EXPONENT }>,
            (): IsI16<{ -CURRENT_EXPONENT }>,
            (): IsI16<{ -TEMPERATURE_EXPONENT }>,
            (): IsI16<{ -AMOUNT_EXPONENT }>,
            (): IsI16<{ -LUMINOSITY_EXPONENT }>,
            (): IsI16<{ -ANGLE_EXPONENT }>,
            (): IsI16<{ -SCALE_P2 }>,
            (): IsI16<{ -SCALE_P3 }>,
            (): IsI16<{ -SCALE_P5 }>,
            (): IsI16<{ -SCALE_PI }>
        ),
        // add min scale where clauses
        (
            (): IsI16<{ min_scale(
                2,
                SCALE_P2_1, SCALE_P3_1, SCALE_P5_1, SCALE_PI_1,
                SCALE_P2_2, SCALE_P3_2, SCALE_P5_2, SCALE_PI_2,
            )}>,
            (): IsI16<{ min_scale(
                3,
                SCALE_P2_1, SCALE_P3_1, SCALE_P5_1, SCALE_PI_1,
                SCALE_P2_2, SCALE_P3_2, SCALE_P5_2, SCALE_PI_2,
            )}>,
            (): IsI16<{ min_scale(
                5,
                SCALE_P2_1, SCALE_P3_1, SCALE_P5_1, SCALE_PI_1,
                SCALE_P2_2, SCALE_P3_2, SCALE_P5_2, SCALE_PI_2,
            )}>,
            (): IsI16<{ min_scale(
                i16::Max,
                SCALE_P2_1, SCALE_P3_1, SCALE_P5_1, SCALE_PI_1,
                SCALE_P2_2, SCALE_P3_2, SCALE_P5_2, SCALE_PI_2,
            )}>
        ),
        // mul output dimension where clauses
        (
            (): IsI16<{ MASS_EXPONENT_1 + MASS_EXPONENT_2 }>,
            (): IsI16<{ LENGTH_EXPONENT_1 + LENGTH_EXPONENT_2 }>,
            (): IsI16<{ TIME_EXPONENT_1 + TIME_EXPONENT_2 }>,
            (): IsI16<{ CURRENT_EXPONENT_1 + CURRENT_EXPONENT_2 }>,
            (): IsI16<{ TEMPERATURE_EXPONENT_1 + TEMPERATURE_EXPONENT_2 }>,
            (): IsI16<{ AMOUNT_EXPONENT_1 + AMOUNT_EXPONENT_2 }>,
            (): IsI16<{ LUMINOSITY_EXPONENT_1 + LUMINOSITY_EXPONENT_2 }>,
            (): IsI16<{ ANGLE_EXPONENT_1 + ANGLE_EXPONENT_2 }>,
            (): IsI16<{ SCALE_P2_1 + SCALE_P2_2 }>,
            (): IsI16<{ SCALE_P3_1 + SCALE_P3_2 }>,
            (): IsI16<{ SCALE_P5_1 + SCALE_P5_2 }>,
            (): IsI16<{ SCALE_PI_1 + SCALE_PI_2 }>
        ),
        // div output dimension where clauses
        (
            (): IsI16<{ MASS_EXPONENT_1 - MASS_EXPONENT_2 }>,
            (): IsI16<{ LENGTH_EXPONENT_1 - LENGTH_EXPONENT_2 }>,
            (): IsI16<{ TIME_EXPONENT_1 - TIME_EXPONENT_2 }>,
            (): IsI16<{ CURRENT_EXPONENT_1 - CURRENT_EXPONENT_2 }>,
            (): IsI16<{ TEMPERATURE_EXPONENT_1 - TEMPERATURE_EXPONENT_2 }>,
            (): IsI16<{ AMOUNT_EXPONENT_1 - AMOUNT_EXPONENT_2 }>,
            (): IsI16<{ LUMINOSITY_EXPONENT_1 - LUMINOSITY_EXPONENT_2 }>,
            (): IsI16<{ ANGLE_EXPONENT_1 - ANGLE_EXPONENT_2 }>,
            (): IsI16<{ SCALE_P2_1 - SCALE_P2_2 }>,
            (): IsI16<{ SCALE_P3_1 - SCALE_P3_2 }>,
            (): IsI16<{ SCALE_P5_1 - SCALE_P5_2 }>,
            (): IsI16<{ SCALE_PI_1 - SCALE_PI_2 }>
        ),
            // other parameters
            $T, rescale_fn
        );
    }
}
// Float arithmetic implementations
define_arithmetic_signed!(f32, rescale_f32);
define_arithmetic_signed!(f64, rescale_f64);

// Integer arithmetic implementations
define_arithmetic_signed!(i8, rescale_i8);
define_arithmetic_signed!(i16, rescale_i16);
define_arithmetic_signed!(i32, rescale_i32);
define_arithmetic_signed!(i64, rescale_i64);
define_arithmetic_signed!(i128, rescale_i128);

// Unsigned integer arithmetic implementations
define_arithmetic!(u8, rescale_u8);
define_arithmetic!(u16, rescale_u16);
define_arithmetic!(u32, rescale_u32);
define_arithmetic!(u64, rescale_u64);
define_arithmetic!(u128, rescale_u128);

// Display traits for all supported types
define_display_traits!(
    (
        const MASS_EXPONENT: i16,
        const LENGTH_EXPONENT: i16,
        const TIME_EXPONENT: i16,
        const CURRENT_EXPONENT: i16,
        const TEMPERATURE_EXPONENT: i16,
        const AMOUNT_EXPONENT: i16,
        const LUMINOSITY_EXPONENT: i16,
        const ANGLE_EXPONENT: i16,
        const SCALE_P2: i16,
        const SCALE_P3: i16,
        const SCALE_P5: i16,
        const SCALE_PI: i16,
    ),
    (
        MASS_EXPONENT,
        LENGTH_EXPONENT,
        TIME_EXPONENT,
        CURRENT_EXPONENT,
        TEMPERATURE_EXPONENT,
        AMOUNT_EXPONENT,
        LUMINOSITY_EXPONENT,
        ANGLE_EXPONENT,
    ),
    (
        SCALE_P2,
        SCALE_P3,
        SCALE_P5,
        SCALE_PI,
    )
);

