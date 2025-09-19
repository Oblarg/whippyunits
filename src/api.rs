use crate::IsI16;
use crate::scale_conversion::*;
use crate::quantity_type::*;
use crate::print::prettyprint::*;
use crate::define_aggregate_scale_factor_rational;
use crate::define_aggregate_scale_factor_float;
use crate::define_display_traits;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt;

define_aggregate_scale_factor_rational!(
    // params
    (
        scale_p2_from: i16, scale_p3_from: i16, scale_p5_from: i16, scale_p10_from: i16, scale_pi_from: i16,
        scale_p2_to: i16, scale_p3_to: i16, scale_p5_to: i16, scale_p10_to: i16, scale_pi_to: i16,
    ),
    // diff expressions
    (
        let diff_scale_p2 = scale_p2_from - scale_p2_to;
        let diff_scale_p3 = scale_p3_from - scale_p3_to;
        let diff_scale_p5 = scale_p5_from - scale_p5_to;
        let diff_scale_p10 = scale_p10_from - scale_p10_to;
        let diff_scale_pi = scale_pi_from - scale_pi_to;
    ),
    // pow expressions
    (
        let (num2, den2) = pow2(diff_scale_p2 as i32);
        let (num3, den3) = pow3(diff_scale_p3 as i32);
        let (num5, den5) = pow5(diff_scale_p5 as i32);
        let (num10, den10) = pow10(diff_scale_p10 as i32);
        let (num_pi, den_pi) = powPi(diff_scale_pi as i32);
    ),
    // num and den expressions
    (num2 * num3 * num5 * num10 * num_pi),
    (den2 * den3 * den5 * den10 * den_pi),
);

define_aggregate_scale_factor_float!(
    // params
    (
        scale_p2_from: i16, scale_p3_from: i16, scale_p5_from: i16, scale_p10_from: i16, scale_pi_from: i16,
        scale_p2_to: i16, scale_p3_to: i16, scale_p5_to: i16, scale_p10_to: i16, scale_pi_to: i16,
    ),
    // diff expressions
    (
        let diff_scale_p2 = scale_p2_from - scale_p2_to;
        let diff_scale_p3 = scale_p3_from - scale_p3_to;
        let diff_scale_p5 = scale_p5_from - scale_p5_to;
        let diff_scale_p10 = scale_p10_from - scale_p10_to;
        let diff_scale_pi = scale_pi_from - scale_pi_to;
    ),
    // pow expressions
    (
        let pow_2 = (2 as f64).powi(diff_scale_p2 as i32);
        let pow_3 = (3 as f64).powi(diff_scale_p3 as i32);
        let pow_5 = (5 as f64).powi(diff_scale_p5 as i32);
        let pow_10 = (10 as f64).powi(diff_scale_p10 as i32);
        let pow_pi = (std::f64::consts::PI).powi(diff_scale_pi as i32);
    ),
    // final expression
    (pow_2 * pow_3 * pow_5 * pow_10 * pow_pi),
);

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
                const SCALE_P10_FROM: i16, const SCALE_P10_TO: i16,
                const SCALE_PI_FROM: i16, const SCALE_PI_TO: i16,
            ),
            (
                Quantity<
                    MASS_EXPONENT,
                    LENGTH_EXPONENT,
                    TIME_EXPONENT,
                    CURRENT_EXPONENT,
                    TEMPERATURE_EXPONENT,
                    AMOUNT_EXPONENT,
                    LUMINOSITY_EXPONENT,
                    ANGLE_EXPONENT,
                    SCALE_P2_FROM,
                    SCALE_P3_FROM,
                    SCALE_P5_FROM,
                    SCALE_P10_FROM,
                    SCALE_PI_FROM,
                    $T,
                >
            ),
            (
                Quantity<
                    MASS_EXPONENT,
                    LENGTH_EXPONENT,
                    TIME_EXPONENT,
                    CURRENT_EXPONENT,
                    TEMPERATURE_EXPONENT,
                    AMOUNT_EXPONENT,
                    LUMINOSITY_EXPONENT,
                    ANGLE_EXPONENT,
                    SCALE_P2_TO,
                    SCALE_P3_TO,
                    SCALE_P5_TO,
                    SCALE_P10_TO,
                    SCALE_PI_TO,
                    $T,
                >
            ),
            (
                SCALE_P2_FROM, SCALE_P3_FROM, SCALE_P5_FROM, SCALE_P10_FROM, SCALE_PI_FROM,
                SCALE_P2_TO, SCALE_P3_TO, SCALE_P5_TO, SCALE_P10_TO, SCALE_PI_TO,
            ),
            $rescale_fn, $T,
        );
    };
    
}
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
                const SCALE_P10_FROM: i16, const SCALE_P10_TO: i16,
                const SCALE_PI_FROM: i16, const SCALE_PI_TO: i16,
            ),
            (
                Quantity<
                    MASS_EXPONENT,
                    LENGTH_EXPONENT,
                    TIME_EXPONENT,
                    CURRENT_EXPONENT,
                    TEMPERATURE_EXPONENT,
                    AMOUNT_EXPONENT,
                    LUMINOSITY_EXPONENT,
                    ANGLE_EXPONENT,
                    SCALE_P2_FROM,
                    SCALE_P3_FROM,
                    SCALE_P5_FROM,
                    SCALE_P10_FROM,
                    SCALE_PI_FROM,
                    $T,
                >
            ),
            (
                Quantity<
                    MASS_EXPONENT,
                    LENGTH_EXPONENT,
                    TIME_EXPONENT,
                    CURRENT_EXPONENT,
                    TEMPERATURE_EXPONENT,
                    AMOUNT_EXPONENT,
                    LUMINOSITY_EXPONENT,
                    ANGLE_EXPONENT,
                    SCALE_P2_TO,
                    SCALE_P3_TO,
                    SCALE_P5_TO,
                    SCALE_P10_TO,
                    SCALE_PI_TO,
                    $T,
                >
            ),
            (
                SCALE_P2_FROM, SCALE_P2_TO,
                SCALE_P3_FROM, SCALE_P3_TO,
                SCALE_P5_FROM, SCALE_P5_TO,
                SCALE_P10_FROM, SCALE_P10_TO,
                SCALE_PI_FROM, SCALE_PI_TO,
            ),
            $rescale_fn, $T,
        );
    };
}
define_float_rescale!(rescale, f64);
define_float_rescale!(rescale_f64, f64);

define_int_rescale!(rescale_i64, i64);

#[macro_export]
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
            const SCALE_P10: i16,
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
            const SCALE_P2_1: i16, const SCALE_P3_1: i16, const SCALE_P5_1: i16, const SCALE_P10_1: i16, const SCALE_PI_1: i16,
            const SCALE_P2_2: i16, const SCALE_P3_2: i16, const SCALE_P5_2: i16, const SCALE_P10_2: i16, const SCALE_PI_2: i16
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
            const SCALE_P2_1: i16, const SCALE_P3_1: i16, const SCALE_P5_1: i16, const SCALE_P10_1: i16, const SCALE_PI_1: i16,
            const SCALE_P2_2: i16, const SCALE_P3_2: i16, const SCALE_P5_2: i16, const SCALE_P10_2: i16, const SCALE_PI_2: i16
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
            (): IsI16<{ -SCALE_P10 }>,
            (): IsI16<{ -SCALE_PI }>
        ),
        // add min scale where clauses
        (
            (): IsI16<{ min_scale(
                2, 
                SCALE_P2_1, SCALE_P3_1, SCALE_P5_1, SCALE_P10_1, SCALE_PI_1,
                SCALE_P2_2, SCALE_P3_2, SCALE_P5_2, SCALE_P10_2, SCALE_PI_2,
            )}>,
            (): IsI16<{ min_scale(
                3, 
                SCALE_P2_1, SCALE_P3_1, SCALE_P5_1, SCALE_P10_1, SCALE_PI_1,
                SCALE_P2_2, SCALE_P3_2, SCALE_P5_2, SCALE_P10_2, SCALE_PI_2,
            )}>,
            (): IsI16<{ min_scale(
                5, 
                SCALE_P2_1, SCALE_P3_1, SCALE_P5_1, SCALE_P10_1, SCALE_PI_1,
                SCALE_P2_2, SCALE_P3_2, SCALE_P5_2, SCALE_P10_2, SCALE_PI_2,
            )}>,
            (): IsI16<{ min_scale(
                10, 
                SCALE_P2_1, SCALE_P3_1, SCALE_P5_1, SCALE_P10_1, SCALE_PI_1,
                SCALE_P2_2, SCALE_P3_2, SCALE_P5_2, SCALE_P10_2, SCALE_PI_2,
            )}>,
            (): IsI16<{ min_scale(
                i16::Max, 
                SCALE_P2_1, SCALE_P3_1, SCALE_P5_1, SCALE_P10_1, SCALE_PI_1,
                SCALE_P2_2, SCALE_P3_2, SCALE_P5_2, SCALE_P10_2, SCALE_PI_2,
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
            (): IsI16<{ SCALE_P10_1 + SCALE_P10_2 }>,
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
            (): IsI16<{ SCALE_P10_1 - SCALE_P10_2 }>,
            (): IsI16<{ SCALE_PI_1 - SCALE_PI_2 }>
        ),
            // other parameters
            $T, rescale_fn
        );
    }
}
define_arithmetic!(f64, rescale_f64);


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
        const SCALE_P10: i16,
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
        SCALE_P2,
        SCALE_P3,
        SCALE_P5,
        SCALE_P10,
        SCALE_PI,
    )
);