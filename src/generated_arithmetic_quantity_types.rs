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
            MASS_SCALE_P10,
            { -LENGTH_EXPONENT },
            LENGTH_SCALE_P10,
            { -TIME_EXPONENT },
            TIME_SCALE_P2,
            TIME_SCALE_P3,
            TIME_SCALE_P5,
            { -CURRENT_EXPONENT },
            CURRENT_SCALE_P10,
            { -TEMPERATURE_EXPONENT },
            TEMPERATURE_SCALE_P10,
            { -AMOUNT_EXPONENT },
            AMOUNT_SCALE_P10,
            { -LUMINOSITY_EXPONENT },
            LUMINOSITY_SCALE_P10,
            { -ANGLE_EXPONENT },
            ANGLE_SCALE_P2,
            ANGLE_SCALE_P3,
            ANGLE_SCALE_P5,
            ANGLE_SCALE_PI,
            $T
        >
    };
}

#[macro_export]
macro_rules! addition_input {
    (Strict, $T:ty) => {
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
    (LeftHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT,
            MASS_SCALE_P10_1,
            LENGTH_EXPONENT,
            LENGTH_SCALE_P10_1,
            TIME_EXPONENT,
            TIME_SCALE_P2_1,
            TIME_SCALE_P3_1,
            TIME_SCALE_P5_1,
            CURRENT_EXPONENT,
            CURRENT_SCALE_P10_1,
            TEMPERATURE_EXPONENT,
            TEMPERATURE_SCALE_P10_1,
            AMOUNT_EXPONENT,
            AMOUNT_SCALE_P10_1,
            LUMINOSITY_EXPONENT,
            LUMINOSITY_SCALE_P10_1,
            ANGLE_EXPONENT,
            ANGLE_SCALE_P2_1,
            ANGLE_SCALE_P3_1,
            ANGLE_SCALE_P5_1,
            ANGLE_SCALE_PI_1,
            $T
        >
    };
    (RightHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT,
            MASS_SCALE_P10_2,
            LENGTH_EXPONENT,
            LENGTH_SCALE_P10_2,
            TIME_EXPONENT,
            TIME_SCALE_P2_2,
            TIME_SCALE_P3_2,
            TIME_SCALE_P5_2,
            CURRENT_EXPONENT,
            CURRENT_SCALE_P10_2,
            TEMPERATURE_EXPONENT,
            TEMPERATURE_SCALE_P10_2,
            AMOUNT_EXPONENT,
            AMOUNT_SCALE_P10_2,
            LUMINOSITY_EXPONENT,
            LUMINOSITY_SCALE_P10_2,
            ANGLE_EXPONENT,
            ANGLE_SCALE_P2_2,
            ANGLE_SCALE_P3_2,
            ANGLE_SCALE_P5_2,
            ANGLE_SCALE_PI_2,
            $T
        >
    };
}

#[macro_export]
macro_rules! multiplication_input {
    (Strict, LeftHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_1,
            MASS_SCALE_P10,
            LENGTH_EXPONENT_1,
            LENGTH_SCALE_P10,
            TIME_EXPONENT_1,
            TIME_SCALE_P2,
            TIME_SCALE_P3,
            TIME_SCALE_P5,
            CURRENT_EXPONENT_1,
            CURRENT_SCALE_P10,
            TEMPERATURE_EXPONENT_1,
            TEMPERATURE_SCALE_P10,
            AMOUNT_EXPONENT_1,
            AMOUNT_SCALE_P10,
            LUMINOSITY_EXPONENT_1,
            LUMINOSITY_SCALE_P10,
            ANGLE_EXPONENT_1,
            ANGLE_SCALE_P2,
            ANGLE_SCALE_P3,
            ANGLE_SCALE_P5,
            ANGLE_SCALE_PI,
            $T
        >
    };
    (Strict, RightHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_2,
            MASS_SCALE_P10,
            LENGTH_EXPONENT_2,
            LENGTH_SCALE_P10,
            TIME_EXPONENT_2,
            TIME_SCALE_P2,
            TIME_SCALE_P3,
            TIME_SCALE_P5,
            CURRENT_EXPONENT_2,
            CURRENT_SCALE_P10,
            TEMPERATURE_EXPONENT_2,
            TEMPERATURE_SCALE_P10,
            AMOUNT_EXPONENT_2,
            AMOUNT_SCALE_P10,
            LUMINOSITY_EXPONENT_2,
            LUMINOSITY_SCALE_P10,
            ANGLE_EXPONENT_2,
            ANGLE_SCALE_P2,
            ANGLE_SCALE_P3,
            ANGLE_SCALE_P5,
            ANGLE_SCALE_PI,
            $T
        >
    };
    ($rescale_behavior:ident, LeftHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_1,
            MASS_SCALE_P10_1,
            LENGTH_EXPONENT_1,
            LENGTH_SCALE_P10_1,
            TIME_EXPONENT_1,
            TIME_SCALE_P2_1,
            TIME_SCALE_P3_1,
            TIME_SCALE_P5_1,
            CURRENT_EXPONENT_1,
            CURRENT_SCALE_P10_1,
            TEMPERATURE_EXPONENT_1,
            TEMPERATURE_SCALE_P10_1,
            AMOUNT_EXPONENT_1,
            AMOUNT_SCALE_P10_1,
            LUMINOSITY_EXPONENT_1,
            LUMINOSITY_SCALE_P10_1,
            ANGLE_EXPONENT_1,
            ANGLE_SCALE_P2_1,
            ANGLE_SCALE_P3_1,
            ANGLE_SCALE_P5_1,
            ANGLE_SCALE_PI_1,
            $T
        >
    };
    ($rescale_behavior:ident, RightHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_2,
            MASS_SCALE_P10_2,
            LENGTH_EXPONENT_2,
            LENGTH_SCALE_P10_2,
            TIME_EXPONENT_2,
            TIME_SCALE_P2_2,
            TIME_SCALE_P3_2,
            TIME_SCALE_P5_2,
            CURRENT_EXPONENT_2,
            CURRENT_SCALE_P10_2,
            TEMPERATURE_EXPONENT_2,
            TEMPERATURE_SCALE_P10_2,
            AMOUNT_EXPONENT_2,
            AMOUNT_SCALE_P10_2,
            LUMINOSITY_EXPONENT_2,
            LUMINOSITY_SCALE_P10_2,
            ANGLE_EXPONENT_2,
            ANGLE_SCALE_P2_2,
            ANGLE_SCALE_P3_2,
            ANGLE_SCALE_P5_2,
            ANGLE_SCALE_PI_2,
            $T
        >
    };
}

macro_rules! multiplication_output_scale_input {
    (LeftHandwins, LeftHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_1,
            MASS_SCALE_P10_1,
            LENGTH_EXPONENT_1,
            LENGTH_SCALE_P10_1,
            TIME_EXPONENT_1,
            TIME_SCALE_P2_1,
            TIME_SCALE_P3_1,
            TIME_SCALE_P5_1,
            CURRENT_EXPONENT_1,
            CURRENT_SCALE_P10_1,
            TEMPERATURE_EXPONENT_1,
            TEMPERATURE_SCALE_P10_1,
            AMOUNT_EXPONENT_1,
            AMOUNT_SCALE_P10_1,
            LUMINOSITY_EXPONENT_1,
            LUMINOSITY_SCALE_P10_1,
            ANGLE_EXPONENT_1,
            ANGLE_SCALE_P2_1,
            ANGLE_SCALE_P3_1,
            ANGLE_SCALE_P5_1,
            ANGLE_SCALE_PI_1,
            $T
        >
    };
    (LeftHandwins, RightHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_2,
            MASS_SCALE_P10_1,
            LENGTH_EXPONENT_2,
            LENGTH_SCALE_P10_1,
            TIME_EXPONENT_2,
            TIME_SCALE_P2_1,
            TIME_SCALE_P3_1,
            TIME_SCALE_P5_1,
            CURRENT_EXPONENT_2,
            CURRENT_SCALE_P10_1,
            TEMPERATURE_EXPONENT_2,
            TEMPERATURE_SCALE_P10_1,
            AMOUNT_EXPONENT_2,
            AMOUNT_SCALE_P10_1,
            LUMINOSITY_EXPONENT_2,
            LUMINOSITY_SCALE_P10_1,
            ANGLE_EXPONENT_2,
            ANGLE_SCALE_P2_1,
            ANGLE_SCALE_P3_1,
            ANGLE_SCALE_P5_1,
            ANGLE_SCALE_PI_1,
            $T
        >
    };
    (SmallerWins, LeftHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_1,
            { min_mass_scale(MASS_EXPONENT_1, MASS_SCALE_P10_1, MASS_SCALE_P10_2, MASS_EXPONENT_2) },
            LENGTH_EXPONENT_1,
            { min_length_scale(LENGTH_EXPONENT_1, LENGTH_SCALE_P10_1, LENGTH_SCALE_P10_2, LENGTH_EXPONENT_2) },
            TIME_EXPONENT_1,
            { min_time_scale(2, 
                TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
            { min_time_scale(3, 
                TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
            { min_time_scale(5, 
                TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
            CURRENT_EXPONENT_1,
            { min_current_scale(CURRENT_EXPONENT_1, CURRENT_SCALE_P10_1, CURRENT_SCALE_P10_2, CURRENT_EXPONENT_2) },
            TEMPERATURE_EXPONENT_1,
            { min_temperature_scale(TEMPERATURE_EXPONENT_1, TEMPERATURE_SCALE_P10_1, TEMPERATURE_SCALE_P10_2, TEMPERATURE_EXPONENT_2) },
            AMOUNT_EXPONENT_1,
            { min_amount_scale(AMOUNT_EXPONENT_1, AMOUNT_SCALE_P10_1, AMOUNT_SCALE_P10_2, AMOUNT_EXPONENT_2) },
            LUMINOSITY_EXPONENT_1,
            { min_luminosity_scale(LUMINOSITY_EXPONENT_1, LUMINOSITY_SCALE_P10_1, LUMINOSITY_SCALE_P10_2, LUMINOSITY_EXPONENT_2) },
            ANGLE_EXPONENT_1,
            { min_angle_scale(2, 
                ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_SCALE_PI_1,
                ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2, ANGLE_SCALE_PI_2
            ) },
            { min_angle_scale(3, 
                ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_SCALE_PI_1,
                ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2, ANGLE_SCALE_PI_2
            ) },
            { min_angle_scale(5, 
                ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_SCALE_PI_1,
                ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2, ANGLE_SCALE_PI_2
            ) },
            { min_angle_scale(i8::MAX,
                ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_SCALE_PI_1,
                ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2, ANGLE_SCALE_PI_2
            ) },
            $T
        >
    };
    (SmallerWins, RightHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_2,
            { min_mass_scale(MASS_EXPONENT_1, MASS_SCALE_P10_1, MASS_SCALE_P10_2, MASS_EXPONENT_2) },
            LENGTH_EXPONENT_2,
            { min_length_scale(LENGTH_EXPONENT_1, LENGTH_SCALE_P10_1, LENGTH_SCALE_P10_2, LENGTH_EXPONENT_2) },
            TIME_EXPONENT_2,
            { min_time_scale(2, 
                TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
            { min_time_scale(3, 
                TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
            { min_time_scale(5, 
                TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
            CURRENT_EXPONENT_2,
            { min_current_scale(CURRENT_EXPONENT_1, CURRENT_SCALE_P10_1, CURRENT_SCALE_P10_2, CURRENT_EXPONENT_2) },
            TEMPERATURE_EXPONENT_2,
            { min_temperature_scale(TEMPERATURE_EXPONENT_1, TEMPERATURE_SCALE_P10_1, TEMPERATURE_SCALE_P10_2, TEMPERATURE_EXPONENT_2) },
            AMOUNT_EXPONENT_2,
            { min_amount_scale(AMOUNT_EXPONENT_1, AMOUNT_SCALE_P10_1, AMOUNT_SCALE_P10_2, AMOUNT_EXPONENT_2) },
            LUMINOSITY_EXPONENT_2,
            { min_luminosity_scale(LUMINOSITY_EXPONENT_1, LUMINOSITY_SCALE_P10_1, LUMINOSITY_SCALE_P10_2, LUMINOSITY_EXPONENT_2) },
            ANGLE_EXPONENT_2,
            { min_angle_scale(2, 
                ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_SCALE_PI_1,
                ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2, ANGLE_SCALE_PI_2
            ) },
            { min_angle_scale(3, 
                ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_SCALE_PI_1,
                ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2, ANGLE_SCALE_PI_2
            ) },
            { min_angle_scale(5, 
                ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_SCALE_PI_1,
                ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2, ANGLE_SCALE_PI_2
            ) },
            { min_angle_scale(i8::MAX,
                ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_SCALE_PI_1,
                ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2, ANGLE_SCALE_PI_2
            ) },
            $T
        >
    };
}

#[macro_export]
macro_rules! multiplication_output {
    (Strict, $T:ty, $log_op:tt) => {
        Quantity<
            { MASS_EXPONENT_1 $log_op MASS_EXPONENT_2 },
            MASS_SCALE_P10,
            { LENGTH_EXPONENT_1 $log_op LENGTH_EXPONENT_2 },
            LENGTH_SCALE_P10,
            { TIME_EXPONENT_1 $log_op TIME_EXPONENT_2 },
            TIME_SCALE_P2,
            TIME_SCALE_P3,
            TIME_SCALE_P5,
            { CURRENT_EXPONENT_1 $log_op CURRENT_EXPONENT_2 },
            CURRENT_SCALE_P10,
            { TEMPERATURE_EXPONENT_1 $log_op TEMPERATURE_EXPONENT_2 },
            TEMPERATURE_SCALE_P10,
            { AMOUNT_EXPONENT_1 $log_op AMOUNT_EXPONENT_2 },
            AMOUNT_SCALE_P10,
            { LUMINOSITY_EXPONENT_1 $log_op LUMINOSITY_EXPONENT_2 },
            LUMINOSITY_SCALE_P10,
            { ANGLE_EXPONENT_1 $log_op ANGLE_EXPONENT_2 },
            ANGLE_SCALE_P2,
            ANGLE_SCALE_P3,
            ANGLE_SCALE_P5,
            ANGLE_SCALE_PI,
            $T
        >
    };

    (LeftHandWins, $T:ty, $log_op:tt) => {
        Quantity<
            { MASS_EXPONENT_1 $log_op MASS_EXPONENT_2 },
            MASS_SCALE_P10_1,
            { LENGTH_EXPONENT_1 $log_op LENGTH_EXPONENT_2 },
            LENGTH_SCALE_P10_1,
            { TIME_EXPONENT_1 $log_op TIME_EXPONENT_2 },
            TIME_SCALE_P2_1,
            TIME_SCALE_P3_1,
            TIME_SCALE_P5_1,
            { CURRENT_EXPONENT_1 $log_op CURRENT_EXPONENT_2 },
            CURRENT_SCALE_P10_1,
            { TEMPERATURE_EXPONENT_1 $log_op TEMPERATURE_EXPONENT_2 },
            TEMPERATURE_SCALE_P10_1,
            { AMOUNT_EXPONENT_1 $log_op AMOUNT_EXPONENT_2 },
            AMOUNT_SCALE_P10_1,
            { LUMINOSITY_EXPONENT_1 $log_op LUMINOSITY_EXPONENT_2 },
            LUMINOSITY_SCALE_P10_1,
            { ANGLE_EXPONENT_1 $log_op ANGLE_EXPONENT_2 },
            ANGLE_SCALE_P2_1,
            ANGLE_SCALE_P3_1,
            ANGLE_SCALE_P5_1,
            ANGLE_SCALE_PI_1,
            $T
        >
    };

    (SmallerWins, $T:ty, $log_op:tt) => {
        Quantity<
            { MASS_EXPONENT_1 $log_op MASS_EXPONENT_2 },
            { min_mass_scale(MASS_EXPONENT_1, MASS_SCALE_P10_1, MASS_SCALE_P10_2, MASS_EXPONENT_2) },
            { LENGTH_EXPONENT_1 $log_op LENGTH_EXPONENT_2 },
            { min_length_scale(LENGTH_EXPONENT_1, LENGTH_SCALE_P10_1, LENGTH_SCALE_P10_2, LENGTH_EXPONENT_2) },
            { TIME_EXPONENT_1 $log_op TIME_EXPONENT_2 },
            { min_time_scale(2, 
                TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
            { min_time_scale(3, 
                TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
            { min_time_scale(5, 
                TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
            { CURRENT_EXPONENT_1 $log_op CURRENT_EXPONENT_2 },
            { min_current_scale(CURRENT_EXPONENT_1, CURRENT_SCALE_P10_1, CURRENT_SCALE_P10_2, CURRENT_EXPONENT_2) },
            { TEMPERATURE_EXPONENT_1 $log_op TEMPERATURE_EXPONENT_2 },
            { min_temperature_scale(TEMPERATURE_EXPONENT_1, TEMPERATURE_SCALE_P10_1, TEMPERATURE_SCALE_P10_2, TEMPERATURE_EXPONENT_2) },
            { AMOUNT_EXPONENT_1 $log_op AMOUNT_EXPONENT_2 },
            { min_amount_scale(AMOUNT_EXPONENT_1, AMOUNT_SCALE_P10_1, AMOUNT_SCALE_P10_2, AMOUNT_EXPONENT_2) },
            { LUMINOSITY_EXPONENT_1 $log_op LUMINOSITY_EXPONENT_2 },
            { min_luminosity_scale(LUMINOSITY_EXPONENT_1, LUMINOSITY_SCALE_P10_1, LUMINOSITY_SCALE_P10_2, LUMINOSITY_EXPONENT_2) },
            { ANGLE_EXPONENT_1 $log_op ANGLE_EXPONENT_2 },
            { min_angle_scale(2, 
                ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_SCALE_PI_1,
                ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2, ANGLE_SCALE_PI_2
            ) },
            { min_angle_scale(3, 
                ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_SCALE_PI_1,
                ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2, ANGLE_SCALE_PI_2
            ) },
            { min_angle_scale(5, 
                ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_SCALE_PI_1,
                ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2, ANGLE_SCALE_PI_2
            ) },
            { min_angle_scale(i8::MAX,
                 ANGLE_EXPONENT_1, ANGLE_SCALE_P2_1, ANGLE_SCALE_P3_1, ANGLE_SCALE_P5_1, ANGLE_SCALE_PI_1,
                ANGLE_EXPONENT_2, ANGLE_SCALE_P2_2, ANGLE_SCALE_P3_2, ANGLE_SCALE_P5_2, ANGLE_SCALE_PI_2
            ) },
            $T
        >
    };
}
