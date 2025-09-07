#[macro_export]
macro_rules! quantity_type {
    () => {
        Quantity<
            MASS_EXPONENT, MASS_SCALE_P10,
            LENGTH_EXPONENT, LENGTH_SCALE_P10,
            TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            T,
        >
    };
    ($T:ty) => {
        Quantity<
            MASS_EXPONENT, MASS_SCALE_P10,
            LENGTH_EXPONENT, LENGTH_SCALE_P10,
            TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            $T,
        >
    };
}

#[macro_export]
macro_rules! inverse_quantity_type {
    ($T:ty) => {
        Quantity<
            { -MASS_EXPONENT }, MASS_SCALE_P10,
            { -LENGTH_EXPONENT }, LENGTH_SCALE_P10,
            { -TIME_EXPONENT }, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            $T,
        >
    };
}

#[macro_export]
macro_rules! addition_input {
    (Strict, $T:ty) => {
        Quantity<
            MASS_EXPONENT, MASS_SCALE_P10,
            LENGTH_EXPONENT, LENGTH_SCALE_P10,
            TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            $T,
        >
    };
    (LeftHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT, MASS_SCALE_P10_1,
            LENGTH_EXPONENT, LENGTH_SCALE_P10_1,
            TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
            $T,
        >
    };
    (RightHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT, MASS_SCALE_P10_2,
            LENGTH_EXPONENT, LENGTH_SCALE_P10_2,
            TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
            $T,
        >
    };
}

#[macro_export]
macro_rules! multiplication_input {
    (Strict, LeftHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_1, MASS_SCALE_P10,
            LENGTH_EXPONENT_1, LENGTH_SCALE_P10,
            TIME_EXPONENT_1, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
        $T,
    >
    };
    (Strict, RightHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_2, MASS_SCALE_P10,
            LENGTH_EXPONENT_2, LENGTH_SCALE_P10,
            TIME_EXPONENT_2, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            $T,
        >
    };
    ($rescale_behavior:ident, LeftHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_1, MASS_SCALE_P10_1,
            LENGTH_EXPONENT_1, LENGTH_SCALE_P10_1,
            TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
            $T,
        >
    };
    ($rescale_behavior:ident, RightHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_2, MASS_SCALE_P10_2,
            LENGTH_EXPONENT_2, LENGTH_SCALE_P10_2,
            TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
            $T,
        >
    };
}

macro_rules! multiplication_output_scale_input {
    (LeftHandwins, LeftHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_1, MASS_SCALE_P10_1,
            LENGTH_EXPONENT_1, LENGTH_SCALE_P10_1,
            TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
            $T,
        >
    };
    (LeftHandwins, RightHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_2, MASS_SCALE_P10_2,
            LENGTH_EXPONENT_2, LENGTH_SCALE_P10_2,
            TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
            $T,
        >
    };
    (SmallerWins, LeftHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_1, { min_mass_scale(MASS_EXPONENT_1, MASS_SCALE_P10_1, MASS_EXPONENT_2, MASS_SCALE_P10_2) },
            LENGTH_EXPONENT_1, { min_length_scale(LENGTH_EXPONENT_1, LENGTH_SCALE_P10_1, LENGTH_EXPONENT_2, LENGTH_SCALE_P10_2) },
            TIME_EXPONENT_1, { min_time_scale(2, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                 TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                             { min_time_scale(3, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                 TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                             { min_time_scale(5, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                 TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
            $T,
        >
    };
    (SmallerWins, RightHand, $T:ty) => {
        Quantity<
            MASS_EXPONENT_2, { min_mass_scale(MASS_EXPONENT_1, MASS_SCALE_P10_1, MASS_EXPONENT_2, MASS_SCALE_P10_2) },
            LENGTH_EXPONENT_2, { min_length_scale(LENGTH_EXPONENT_1, LENGTH_SCALE_P10_1, LENGTH_EXPONENT_2, LENGTH_SCALE_P10_2) },
            TIME_EXPONENT_2, { min_time_scale(2, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                 TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                             { min_time_scale(3, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                 TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                             { min_time_scale(5, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                 TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
            $T,
        >
    };
}

#[macro_export]
macro_rules! multiplication_output {
    (Strict, $T:ty, $log_op:tt) => {
        Quantity<
            { MASS_EXPONENT_1 $log_op MASS_EXPONENT_2 }, MASS_SCALE_P10,
            { LENGTH_EXPONENT_1 $log_op LENGTH_EXPONENT_2 }, LENGTH_SCALE_P10,
            { TIME_EXPONENT_1 $log_op TIME_EXPONENT_2 }, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            $T,
        >
    };

    (LeftHandWins, $T:ty, $log_op:tt) => {
        Quantity<
            { MASS_EXPONENT_1 $log_op MASS_EXPONENT_2 }, MASS_SCALE_P10_1,
            { LENGTH_EXPONENT_1 $log_op LENGTH_EXPONENT_2 }, LENGTH_SCALE_P10_1,
            { TIME_EXPONENT_1 $log_op TIME_EXPONENT_2 }, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
            $T,
        >
    };

    (SmallerWins, $T:ty, $log_op:tt) => {
        Quantity<
            { MASS_EXPONENT_1 $log_op MASS_EXPONENT_2 }, { min_mass_scale(MASS_EXPONENT_1, MASS_SCALE_P10_1, MASS_EXPONENT_2, MASS_SCALE_P10_2) },
            { LENGTH_EXPONENT_1 $log_op LENGTH_EXPONENT_2 }, { min_length_scale(LENGTH_EXPONENT_1, LENGTH_SCALE_P10_1, LENGTH_EXPONENT_2, LENGTH_SCALE_P10_2) },
            { TIME_EXPONENT_1 $log_op TIME_EXPONENT_2 }, { min_time_scale(2, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                                             TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                                                         { min_time_scale(3, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                                             TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                                                         { min_time_scale(5, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                                             TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
            $T,
        >
    };
}


