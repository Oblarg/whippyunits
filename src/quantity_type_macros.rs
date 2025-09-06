#[macro_export]
macro_rules! single_quantity_type {
    () => {
        Quantity<
            MASS_EXPONENT, MASS_SCALE_P10,
            LENGTH_EXPONENT, LENGTH_SCALE_P10,
            TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            T,
        >
    };
}

#[macro_export]
macro_rules! left_hand_quantity_type {
    () => {
        Quantity<
            MASS_EXPONENT, MASS_SCALE_P10_1,
            LENGTH_EXPONENT, LENGTH_SCALE_P10_1,
            TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
            T,
        >
    };
}

#[macro_export]
macro_rules! right_hand_quantity_type {
    () => {
        Quantity<
            MASS_EXPONENT, MASS_SCALE_P10_2,
            LENGTH_EXPONENT, LENGTH_SCALE_P10_2,
            TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
            T,
        >
    };
}