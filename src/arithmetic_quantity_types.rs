//! Generated Arithmetic Quantity Types with Full Base Unit Dimensions
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

#[macro_export]
macro_rules! inverse_quantity_type {
    ($T:ty) => {
        Quantity<
            { -MASS_EXPONENT },
            { -LENGTH_EXPONENT },
            { -TIME_EXPONENT },
            { -CURRENT_EXPONENT },
            { -TEMPERATURE_EXPONENT },
            { -AMOUNT_EXPONENT },
            { -LUMINOSITY_EXPONENT },
            { -ANGLE_EXPONENT },
            SCALE_P2,
            SCALE_P3,
            SCALE_P5,
            SCALE_PI,
            $T
        >
    };
}

#[macro_export]
macro_rules! addition_input {
    (Strict, $T:ty) => {
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
    (LeftHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT,
            LENGTH_EXPONENT,
            TIME_EXPONENT,
            CURRENT_EXPONENT,
            TEMPERATURE_EXPONENT,
            AMOUNT_EXPONENT,
            LUMINOSITY_EXPONENT,
            ANGLE_EXPONENT,
            SCALE_P2_1,
            SCALE_P3_1,
            SCALE_P5_1,
            SCALE_PI_1,
            $T
        >
    };
    (RightHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT,
            LENGTH_EXPONENT,
            TIME_EXPONENT,
            CURRENT_EXPONENT,
            TEMPERATURE_EXPONENT,
            AMOUNT_EXPONENT,
            LUMINOSITY_EXPONENT,
            ANGLE_EXPONENT,
            SCALE_P2_2,
            SCALE_P3_2,
            SCALE_P5_2,
            SCALE_PI_2,
            $T
        >
    };
}

#[macro_export]
macro_rules! multiplication_input {
    (LeftHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_1,
            LENGTH_EXPONENT_1,
            TIME_EXPONENT_1,
            CURRENT_EXPONENT_1,
            TEMPERATURE_EXPONENT_1,
            AMOUNT_EXPONENT_1,
            LUMINOSITY_EXPONENT_1,
            ANGLE_EXPONENT_1,
            SCALE_P2_1,
            SCALE_P3_1,
            SCALE_P5_1,
            SCALE_PI_1,
            $T
        >
    };
    (RightHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_2,
            LENGTH_EXPONENT_2,
            TIME_EXPONENT_2,
            CURRENT_EXPONENT_2,
            TEMPERATURE_EXPONENT_2,
            AMOUNT_EXPONENT_2,
            LUMINOSITY_EXPONENT_2,
            ANGLE_EXPONENT_2,
            SCALE_P2_2,
            SCALE_P3_2,
            SCALE_P5_2,
            SCALE_PI_2,
            $T
        >
    };
}

#[macro_export]
macro_rules! multiplication_output {
    ($T:ty, $log_op:tt) => {
        Quantity<
            { MASS_EXPONENT_1 $log_op MASS_EXPONENT_2 },
            { LENGTH_EXPONENT_1 $log_op LENGTH_EXPONENT_2 },
            { TIME_EXPONENT_1 $log_op TIME_EXPONENT_2 },
            { CURRENT_EXPONENT_1 $log_op CURRENT_EXPONENT_2 },
            { TEMPERATURE_EXPONENT_1 $log_op TEMPERATURE_EXPONENT_2 },
            { AMOUNT_EXPONENT_1 $log_op AMOUNT_EXPONENT_2 },
            { LUMINOSITY_EXPONENT_1 $log_op LUMINOSITY_EXPONENT_2 },
            { ANGLE_EXPONENT_1 $log_op ANGLE_EXPONENT_2 },
            { SCALE_P2_1 $log_op SCALE_P2_2 },
            { SCALE_P3_1 $log_op SCALE_P3_2 },
            { SCALE_P5_1 $log_op SCALE_P5_2 },
            { SCALE_PI_1 $log_op SCALE_PI_2 },
            $T
        >
    };
}
