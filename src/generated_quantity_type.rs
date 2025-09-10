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
    T = f64
> {
    pub value: T,
}

impl<
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
    T
>
    Quantity<
        MASS_EXPONENT,
        MASS_SCALE_P10,
        LENGTH_EXPONENT,
        LENGTH_SCALE_P10,
        TIME_EXPONENT,
        TIME_SCALE_P2,
        TIME_SCALE_P3,
        TIME_SCALE_P5,
        CURRENT_EXPONENT,
        CURRENT_SCALE_P10,
        TEMPERATURE_EXPONENT,
        TEMPERATURE_SCALE_P10,
        AMOUNT_EXPONENT,
        AMOUNT_SCALE_P10,
        LUMINOSITY_EXPONENT,
        LUMINOSITY_SCALE_P10,
        ANGLE_EXPONENT,
        ANGLE_SCALE_P2,
        ANGLE_SCALE_P3,
        ANGLE_SCALE_P5,
        ANGLE_SCALE_PI,
        T
    >
{
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

// from/into for dimensionless quantities

// proper dimensionless quantities (all exponents are 0, scales irrelevant)
impl<
    const MASS_SCALE_P10: i8,
    const LENGTH_SCALE_P10: i8,
    const TIME_SCALE_P2: i8,
    const TIME_SCALE_P3: i8,
    const TIME_SCALE_P5: i8,
    const CURRENT_SCALE_P10: i8,
    const TEMPERATURE_SCALE_P10: i8,
    const AMOUNT_SCALE_P10: i8,
    const LUMINOSITY_SCALE_P10: i8,
    const ANGLE_SCALE_P2: i8,
    const ANGLE_SCALE_P3: i8,
    const ANGLE_SCALE_P5: i8,
    const ANGLE_SCALE_PI: i8,
> From<
    Quantity<
        0,
        MASS_SCALE_P10,
        0,
        LENGTH_SCALE_P10,
        0,
        TIME_SCALE_P2,
        TIME_SCALE_P3,
        TIME_SCALE_P5,
        0,
        CURRENT_SCALE_P10,
        0,
        TEMPERATURE_SCALE_P10,
        0,
        AMOUNT_SCALE_P10,
        0,
        LUMINOSITY_SCALE_P10,
        0,
        ANGLE_SCALE_P2,
        ANGLE_SCALE_P3,
        ANGLE_SCALE_P5,
        ANGLE_SCALE_PI,
        f64
    >
> for f64
{
    fn from(other: Quantity<
        0,
        MASS_SCALE_P10,
        0,
        LENGTH_SCALE_P10,
        0,
        TIME_SCALE_P2,
        TIME_SCALE_P3,
        TIME_SCALE_P5,
        0,
        CURRENT_SCALE_P10,
        0,
        TEMPERATURE_SCALE_P10,
        0,
        AMOUNT_SCALE_P10,
        0,
        LUMINOSITY_SCALE_P10,
        0,
        ANGLE_SCALE_P2,
        ANGLE_SCALE_P3,
        ANGLE_SCALE_P5,
        ANGLE_SCALE_PI,
        f64
    >
    ) -> f64 {
        other.value
    }
}

// radians can be identified as dimensionless (all exponents are 0 except angle, angle scale radians)
// trait resolution rules mean we have to manually template this out over different angle exponents...



macro_rules! define_from_for_radians {
    ($exponent:expr) => {
        impl<
            const MASS_SCALE_P10: i8,
            const LENGTH_SCALE_P10: i8,
            const TIME_SCALE_P2: i8,
            const TIME_SCALE_P3: i8,
            const TIME_SCALE_P5: i8,
            const CURRENT_SCALE_P10: i8,
            const TEMPERATURE_SCALE_P10: i8,
            const AMOUNT_SCALE_P10: i8,
            const LUMINOSITY_SCALE_P10: i8,
        > From<
            Quantity<
                0,
                MASS_SCALE_P10,
                0,
                LENGTH_SCALE_P10,
                0,
                TIME_SCALE_P2,
                TIME_SCALE_P3,
                TIME_SCALE_P5,
                0,
                CURRENT_SCALE_P10,
                0,
                TEMPERATURE_SCALE_P10,
                0,
                AMOUNT_SCALE_P10,
                0,
                LUMINOSITY_SCALE_P10,
                $exponent,
                0, 0, 0, 0,
                f64
            >
        > for f64
        {
            fn from(other: Quantity<
                0,
                MASS_SCALE_P10,
                0,
                LENGTH_SCALE_P10,
                0,
                TIME_SCALE_P2,
                TIME_SCALE_P3,
                TIME_SCALE_P5,
                0,
                CURRENT_SCALE_P10,
                0,
                TEMPERATURE_SCALE_P10,
                0,
                AMOUNT_SCALE_P10,
                0,
                LUMINOSITY_SCALE_P10,
                $exponent,
                0, 0, 0, 0,
                f64
            >) -> f64 {
                other.value
            }
        }

        impl<
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
            const ANGLE_SCALE_P2: i8,
            const ANGLE_SCALE_P3: i8,
            const ANGLE_SCALE_P5: i8,
            const ANGLE_SCALE_PI: i8,
        > From<
            Quantity<
                MASS_EXPONENT,
                MASS_SCALE_P10,
                LENGTH_EXPONENT,
                LENGTH_SCALE_P10,
                TIME_EXPONENT,
                TIME_SCALE_P2,
                TIME_SCALE_P3,
                TIME_SCALE_P5,
                CURRENT_EXPONENT,
                CURRENT_SCALE_P10,
                TEMPERATURE_EXPONENT,
                TEMPERATURE_SCALE_P10,
                AMOUNT_EXPONENT,
                AMOUNT_SCALE_P10,
                LUMINOSITY_EXPONENT,
                LUMINOSITY_SCALE_P10,
                $exponent,
                0, 0, 0, 0,
                f64
            >
        > for Quantity<
            MASS_EXPONENT,
            MASS_SCALE_P10,
            LENGTH_EXPONENT,
            LENGTH_SCALE_P10,
            TIME_EXPONENT,
            TIME_SCALE_P2,
            TIME_SCALE_P3,
            TIME_SCALE_P5,
            CURRENT_EXPONENT,
            CURRENT_SCALE_P10,
            TEMPERATURE_EXPONENT,
            TEMPERATURE_SCALE_P10,
            AMOUNT_EXPONENT,
            AMOUNT_SCALE_P10,
            LUMINOSITY_EXPONENT,
            LUMINOSITY_SCALE_P10,
            0,
            ANGLE_SCALE_P2,
            ANGLE_SCALE_P3,
            ANGLE_SCALE_P5,
            ANGLE_SCALE_PI,
            f64
        >
        {
            fn from(other: Quantity<
                MASS_EXPONENT,
                MASS_SCALE_P10,
                LENGTH_EXPONENT,
                LENGTH_SCALE_P10,
                TIME_EXPONENT,
                TIME_SCALE_P2,
                TIME_SCALE_P3,
                TIME_SCALE_P5,
                CURRENT_EXPONENT,
                CURRENT_SCALE_P10,
                TEMPERATURE_EXPONENT,
                TEMPERATURE_SCALE_P10,
                AMOUNT_EXPONENT,
                AMOUNT_SCALE_P10,
                LUMINOSITY_EXPONENT,
                LUMINOSITY_SCALE_P10,
                $exponent,
                0, 0, 0, 0,
                f64
            >) -> Self {
                Self { value: other.value }
            }
        }
    };
}

define_from_for_radians!(-9);
define_from_for_radians!(-8);
define_from_for_radians!(-7);
define_from_for_radians!(-6);
define_from_for_radians!(-5);
define_from_for_radians!(-4);
define_from_for_radians!(-3);
define_from_for_radians!(-2);
define_from_for_radians!(-1);
define_from_for_radians!(1);
define_from_for_radians!(2);
define_from_for_radians!(3);
define_from_for_radians!(4);
define_from_for_radians!(5);
define_from_for_radians!(6);
define_from_for_radians!(7);
define_from_for_radians!(8);
define_from_for_radians!(9);


#[macro_export]
macro_rules! quantity_type {
    () => {
        Quantity<
            MASS_EXPONENT,
            MASS_SCALE_P10,
            LENGTH_EXPONENT,
            LENGTH_SCALE_P10,
            TIME_EXPONENT,
            TIME_SCALE_P2,
            TIME_SCALE_P3,
            TIME_SCALE_P5,
            CURRENT_EXPONENT,
            CURRENT_SCALE_P10,
            TEMPERATURE_EXPONENT,
            TEMPERATURE_SCALE_P10,
            AMOUNT_EXPONENT,
            AMOUNT_SCALE_P10,
            LUMINOSITY_EXPONENT,
            LUMINOSITY_SCALE_P10,
            ANGLE_EXPONENT,
            ANGLE_SCALE_P2,
            ANGLE_SCALE_P3,
            ANGLE_SCALE_P5,
            ANGLE_SCALE_PI,
            T
        >
    };
    ($T:ty) => {
        Quantity<
            MASS_EXPONENT,
            MASS_SCALE_P10,
            LENGTH_EXPONENT,
            LENGTH_SCALE_P10,
            TIME_EXPONENT,
            TIME_SCALE_P2,
            TIME_SCALE_P3,
            TIME_SCALE_P5,
            CURRENT_EXPONENT,
            CURRENT_SCALE_P10,
            TEMPERATURE_EXPONENT,
            TEMPERATURE_SCALE_P10,
            AMOUNT_EXPONENT,
            AMOUNT_SCALE_P10,
            LUMINOSITY_EXPONENT,
            LUMINOSITY_SCALE_P10,
            ANGLE_EXPONENT,
            ANGLE_SCALE_P2,
            ANGLE_SCALE_P3,
            ANGLE_SCALE_P5,
            ANGLE_SCALE_PI,
            $T
        >
    };
}
