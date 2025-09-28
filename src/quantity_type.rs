//! Generated Quantity Type with Full Base Unit Dimensions
//!
//! This file is auto-generated from dimension_data.rs and includes support
//! for all base unit dimensions defined in the system.
//!
//! Base dimensions supported:
//! //! - mass (primes: [10])
//! - length (primes: [10])
//! - time (primes: [2, 3, 5])
//! - current (primes: [10])
//! - temperature (primes: [10])
//! - amount (primes: [10])
//! - luminosity (primes: [10])
//! - angle (primes: [2, 3, 5])

#[derive(Clone, Copy, PartialEq)]
pub struct Quantity<
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
    T = f64,
> {
    pub value: T,
}

impl<
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
        T,
    >
    Quantity<
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
        SCALE_PI,
        T,
    >
{
    pub const fn new(value: T) -> Self {
        Self { value }
    }
}

// from/into for dimensionless quantities

// proper dimensionless quantities (all exponents are 0, scales irrelevant)
macro_rules! define_from_dimensionless {
    ($type:ty, $rescale_fn:ident) => {
        // General case for all scales - rescale from current scale to 0
        impl<const SCALE_P2: i16, const SCALE_P3: i16, const SCALE_P5: i16, const SCALE_PI: i16>
            From<Quantity<0, 0, 0, 0, 0, 0, 0, 0, SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI, $type>> for $type
        {
            fn from(
                other: Quantity<0, 0, 0, 0, 0, 0, 0, 0, SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI, $type>,
            ) -> $type {
                // If all scales are zero, just return the raw value
                if SCALE_P2 == 0 && SCALE_P3 == 0 && SCALE_P5 == 0 && SCALE_PI == 0 {
                    other.value
                } else {
                    // Use the provided rescale function
                    crate::api::$rescale_fn::<
                        0, 0, 0, 0, 0, 0, 0, 0,
                        SCALE_P2, 0, SCALE_P3, 0, SCALE_P5, 0, SCALE_PI, 0
                    >(other).value
                }
            }
        }
    };
}

define_from_dimensionless!(f32, rescale_f32);
define_from_dimensionless!(f64, rescale_f64);
define_from_dimensionless!(i16, rescale_i16);
define_from_dimensionless!(i32, rescale_i32);
define_from_dimensionless!(i64, rescale_i64);
define_from_dimensionless!(i128, rescale_i128);

// Pure radian power to scalar with scale handling - handles both zero and non-zero scales
macro_rules! define_from_for_radians_with_scale {
    ($exponent:expr, $type:ty, $rescale_fn:ident) => {
        impl<const SCALE_P2: i16, const SCALE_P3: i16, const SCALE_P5: i16, const SCALE_PI: i16>
            From<Quantity<0, 0, 0, 0, 0, 0, 0, $exponent, SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI, $type>> for $type
        {
            fn from(
                other: Quantity<0, 0, 0, 0, 0, 0, 0, $exponent, SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI, $type>,
            ) -> $type {
                // If all scales are zero, just return the raw value
                if SCALE_P2 == 0 && SCALE_P3 == 0 && SCALE_P5 == 0 && SCALE_PI == 0 {
                    other.value
                } else {
                    // Use the provided rescale function
                    crate::api::$rescale_fn::<
                        0, 0, 0, 0, 0, 0, 0, $exponent,
                        SCALE_P2, 0, SCALE_P3, 0, SCALE_P5, 0, SCALE_PI, 0
                    >(other).value
                }
            }
        }
    };
}

// radians can be identified as dimensionless (all exponents are 0 except angle, angle scale radians)
// trait resolution rules mean we have to manually template this out over different angle exponents...

macro_rules! define_from_for_radians {
    ($exponent:expr, $($type:ty),+ $(,)?) => {
        $(
            // Removed direct-to-scalar implementation - now handled by define_from_for_radians_with_scale!

            // TODO: This second impl has unconstrained const parameters
            // Need to figure out the correct approach for angle conversions
            impl<
                    const MASS_EXPONENT: i16,
                    const LENGTH_EXPONENT: i16,
                    const TIME_EXPONENT: i16,
                    const CURRENT_EXPONENT: i16,
                    const TEMPERATURE_EXPONENT: i16,
                    const AMOUNT_EXPONENT: i16,
                    const LUMINOSITY_EXPONENT: i16,
                >
                From<
                    Quantity<
                        MASS_EXPONENT,
                        LENGTH_EXPONENT,
                        TIME_EXPONENT,
                        CURRENT_EXPONENT,
                        TEMPERATURE_EXPONENT,
                        AMOUNT_EXPONENT,
                        LUMINOSITY_EXPONENT,
                        $exponent,
                        0,
                        0,
                        0,
                        0,
                        $type,
                    >,
                >
                for Quantity<
                    MASS_EXPONENT,
                    LENGTH_EXPONENT,
                    TIME_EXPONENT,
                    CURRENT_EXPONENT,
                    TEMPERATURE_EXPONENT,
                    AMOUNT_EXPONENT,
                    LUMINOSITY_EXPONENT,
                    0,
                    0,
                    0,
                    0,
                    0,
                    $type,
                >
            {
                fn from(
                    other: Quantity<
                        MASS_EXPONENT,
                        LENGTH_EXPONENT,
                        TIME_EXPONENT,
                        CURRENT_EXPONENT,
                        TEMPERATURE_EXPONENT,
                        AMOUNT_EXPONENT,
                        LUMINOSITY_EXPONENT,
                        $exponent,
                        0,
                        0,
                        0,
                        0,
                        $type,
                    >,
                ) -> Self {
                    Self { value: other.value }
                }
            }
        )+
    };
}

// Generate all radian erasure implementations using unified proc macro
whippyunits_proc_macros::generate_all_radian_erasures!(9);

#[macro_export]
macro_rules! quantity_type {
    () => {
        Quantity<
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
            SCALE_PI,
            T
        >
    };
    ($T:ty) => {
        Quantity<
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
            SCALE_PI,
            $T
        >
    };
}
