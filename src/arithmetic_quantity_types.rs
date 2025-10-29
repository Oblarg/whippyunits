#[macro_export]
#[doc(hidden)]
macro_rules! inverse_quantity_type {
    ($T:ty) => {
        Quantity<
            Scale<_2<{ -SCALE_P2 }>, _3<{ -SCALE_P3 }>, _5<{ -SCALE_P5 }>, _Pi<{ -SCALE_PI }>>,
            Dimension<_M<{ -MASS_EXPONENT }>, _L<{ -LENGTH_EXPONENT }>, _T<{ -TIME_EXPONENT }>, _I<{ -CURRENT_EXPONENT }>, _Θ<{ -TEMPERATURE_EXPONENT }>, _N<{ -AMOUNT_EXPONENT }>, _J<{ -LUMINOSITY_EXPONENT }>, _A<{ -ANGLE_EXPONENT }>>,
            $T,
            Brand
        >
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! addition_input {
    (Strict, $T:ty) => {
        Quantity<
            Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
            Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
            $T,
            Brand
        >
    };
    (LeftHand, $T:ty) => {
        Quantity<
            Scale<_2<SCALE_P2_1>, _3<SCALE_P3_1>, _5<SCALE_P5_1>, _Pi<SCALE_PI_1>>,
            Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
            $T,
            Brand
        >
    };
    (RightHand, $T:ty) => {
        Quantity<
            Scale<_2<SCALE_P2_2>, _3<SCALE_P3_2>, _5<SCALE_P5_2>, _Pi<SCALE_PI_2>>,
            Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
            $T,
            Brand
        >
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! multiplication_input {
    (LeftHand, $T:ty) => {
        Quantity<
            Scale<_2<SCALE_P2_1>, _3<SCALE_P3_1>, _5<SCALE_P5_1>, _Pi<SCALE_PI_1>>,
            Dimension<_M<MASS_EXPONENT_1>, _L<LENGTH_EXPONENT_1>, _T<TIME_EXPONENT_1>, _I<CURRENT_EXPONENT_1>, _Θ<TEMPERATURE_EXPONENT_1>, _N<AMOUNT_EXPONENT_1>, _J<LUMINOSITY_EXPONENT_1>, _A<ANGLE_EXPONENT_1>>,
            $T,
            Brand
        >
    };
    (RightHand, $T:ty) => {
        Quantity<
            Scale<_2<SCALE_P2_2>, _3<SCALE_P3_2>, _5<SCALE_P5_2>, _Pi<SCALE_PI_2>>,
            Dimension<_M<MASS_EXPONENT_2>, _L<LENGTH_EXPONENT_2>, _T<TIME_EXPONENT_2>, _I<CURRENT_EXPONENT_2>, _Θ<TEMPERATURE_EXPONENT_2>, _N<AMOUNT_EXPONENT_2>, _J<LUMINOSITY_EXPONENT_2>, _A<ANGLE_EXPONENT_2>>,
            $T,
            Brand
        >
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! multiplication_output {
    ($T:ty, $log_op:tt) => {
        Quantity<
            Scale<
                _2<{ SCALE_P2_1 $log_op SCALE_P2_2 }>,
                _3<{ SCALE_P3_1 $log_op SCALE_P3_2 }>,
                _5<{ SCALE_P5_1 $log_op SCALE_P5_2 }>,
                _Pi<{ SCALE_PI_1 $log_op SCALE_PI_2 }>
            >,
            Dimension<
                _M<{ MASS_EXPONENT_1 $log_op MASS_EXPONENT_2 }>,
                _L<{ LENGTH_EXPONENT_1 $log_op LENGTH_EXPONENT_2 }>,
                _T<{ TIME_EXPONENT_1 $log_op TIME_EXPONENT_2 }>,
                _I<{ CURRENT_EXPONENT_1 $log_op CURRENT_EXPONENT_2 }>,
                _Θ<{ TEMPERATURE_EXPONENT_1 $log_op TEMPERATURE_EXPONENT_2 }>,
                _N<{ AMOUNT_EXPONENT_1 $log_op AMOUNT_EXPONENT_2 }>,
                _J<{ LUMINOSITY_EXPONENT_1 $log_op LUMINOSITY_EXPONENT_2 }>,
                _A<{ ANGLE_EXPONENT_1 $log_op ANGLE_EXPONENT_2 }>
            >,
            $T,
            Brand
        >
    };
}
