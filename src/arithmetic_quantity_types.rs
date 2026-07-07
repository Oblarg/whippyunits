#[macro_export]
#[doc(hidden)]
#[cfg(has_generic_const_exprs)]
macro_rules! inverse_quantity_type {
    ($T:ty) => {
        $crate::quantity::Quantity<
            $crate::quantity::Scale<
                $crate::quantity::_2<{ -SCALE_P2 }>,
                $crate::quantity::_3<{ -SCALE_P3 }>,
                $crate::quantity::_5<{ -SCALE_P5 }>,
                $crate::quantity::_Pi<{ -SCALE_PI }>,
            >,
            $crate::quantity::Dimension<
                $crate::quantity::_M<{ -MASS_EXPONENT }>,
                $crate::quantity::_L<{ -LENGTH_EXPONENT }>,
                $crate::quantity::_T<{ -TIME_EXPONENT }>,
                $crate::quantity::_I<{ -CURRENT_EXPONENT }>,
                $crate::quantity::_Θ<{ -TEMPERATURE_EXPONENT }>,
                $crate::quantity::_N<{ -AMOUNT_EXPONENT }>,
                $crate::quantity::_J<{ -LUMINOSITY_EXPONENT }>,
                $crate::quantity::_A<{ -ANGLE_EXPONENT }>,
            >,
            $T,
            Brand
        >
    };
}

#[macro_export]
#[doc(hidden)]
#[cfg(not(has_generic_const_exprs))]
macro_rules! inverse_quantity_type {
    ($T:ty) => {
        $crate::quantity::Quantity<
            $crate::quantity::Scale<
                $crate::quantity::_2<INVERSE_SCALE_P2>,
                $crate::quantity::_3<INVERSE_SCALE_P3>,
                $crate::quantity::_5<INVERSE_SCALE_P5>,
                $crate::quantity::_Pi<INVERSE_SCALE_PI>,
            >,
            $crate::quantity::Dimension<
                $crate::quantity::_M<INVERSE_MASS_EXPONENT>,
                $crate::quantity::_L<INVERSE_LENGTH_EXPONENT>,
                $crate::quantity::_T<INVERSE_TIME_EXPONENT>,
                $crate::quantity::_I<INVERSE_CURRENT_EXPONENT>,
                $crate::quantity::_Θ<INVERSE_TEMPERATURE_EXPONENT>,
                $crate::quantity::_N<INVERSE_AMOUNT_EXPONENT>,
                $crate::quantity::_J<INVERSE_LUMINOSITY_EXPONENT>,
                $crate::quantity::_A<INVERSE_ANGLE_EXPONENT>,
            >,
            $T,
            Brand
        >
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! addition_input {
    (Strict, $T:ty) => {
        $crate::quantity::Quantity<
            $crate::quantity::Scale<
                $crate::quantity::_2<SCALE_P2>,
                $crate::quantity::_3<SCALE_P3>,
                $crate::quantity::_5<SCALE_P5>,
                $crate::quantity::_Pi<SCALE_PI>,
            >,
            $crate::quantity::Dimension<
                $crate::quantity::_M<MASS_EXPONENT>,
                $crate::quantity::_L<LENGTH_EXPONENT>,
                $crate::quantity::_T<TIME_EXPONENT>,
                $crate::quantity::_I<CURRENT_EXPONENT>,
                $crate::quantity::_Θ<TEMPERATURE_EXPONENT>,
                $crate::quantity::_N<AMOUNT_EXPONENT>,
                $crate::quantity::_J<LUMINOSITY_EXPONENT>,
                $crate::quantity::_A<ANGLE_EXPONENT>,
            >,
            $T,
            Brand
        >
    };
    (LeftHand, $T:ty) => {
        $crate::quantity::Quantity<
            $crate::quantity::Scale<
                $crate::quantity::_2<SCALE_P2_1>,
                $crate::quantity::_3<SCALE_P3_1>,
                $crate::quantity::_5<SCALE_P5_1>,
                $crate::quantity::_Pi<SCALE_PI_1>,
            >,
            $crate::quantity::Dimension<
                $crate::quantity::_M<MASS_EXPONENT>,
                $crate::quantity::_L<LENGTH_EXPONENT>,
                $crate::quantity::_T<TIME_EXPONENT>,
                $crate::quantity::_I<CURRENT_EXPONENT>,
                $crate::quantity::_Θ<TEMPERATURE_EXPONENT>,
                $crate::quantity::_N<AMOUNT_EXPONENT>,
                $crate::quantity::_J<LUMINOSITY_EXPONENT>,
                $crate::quantity::_A<ANGLE_EXPONENT>,
            >,
            $T,
            Brand
        >
    };
    (RightHand, $T:ty) => {
        $crate::quantity::Quantity<
            $crate::quantity::Scale<
                $crate::quantity::_2<SCALE_P2_2>,
                $crate::quantity::_3<SCALE_P3_2>,
                $crate::quantity::_5<SCALE_P5_2>,
                $crate::quantity::_Pi<SCALE_PI_2>,
            >,
            $crate::quantity::Dimension<
                $crate::quantity::_M<MASS_EXPONENT>,
                $crate::quantity::_L<LENGTH_EXPONENT>,
                $crate::quantity::_T<TIME_EXPONENT>,
                $crate::quantity::_I<CURRENT_EXPONENT>,
                $crate::quantity::_Θ<TEMPERATURE_EXPONENT>,
                $crate::quantity::_N<AMOUNT_EXPONENT>,
                $crate::quantity::_J<LUMINOSITY_EXPONENT>,
                $crate::quantity::_A<ANGLE_EXPONENT>,
            >,
            $T,
            Brand
        >
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! multiplication_input {
    (LeftHand, $T:ty) => {
        $crate::quantity::Quantity<
            $crate::quantity::Scale<
                $crate::quantity::_2<SCALE_P2_1>,
                $crate::quantity::_3<SCALE_P3_1>,
                $crate::quantity::_5<SCALE_P5_1>,
                $crate::quantity::_Pi<SCALE_PI_1>,
            >,
            $crate::quantity::Dimension<
                $crate::quantity::_M<MASS_EXPONENT_1>,
                $crate::quantity::_L<LENGTH_EXPONENT_1>,
                $crate::quantity::_T<TIME_EXPONENT_1>,
                $crate::quantity::_I<CURRENT_EXPONENT_1>,
                $crate::quantity::_Θ<TEMPERATURE_EXPONENT_1>,
                $crate::quantity::_N<AMOUNT_EXPONENT_1>,
                $crate::quantity::_J<LUMINOSITY_EXPONENT_1>,
                $crate::quantity::_A<ANGLE_EXPONENT_1>,
            >,
            $T,
            Brand
        >
    };
    (RightHand, $T:ty) => {
        $crate::quantity::Quantity<
            $crate::quantity::Scale<
                $crate::quantity::_2<SCALE_P2_2>,
                $crate::quantity::_3<SCALE_P3_2>,
                $crate::quantity::_5<SCALE_P5_2>,
                $crate::quantity::_Pi<SCALE_PI_2>,
            >,
            $crate::quantity::Dimension<
                $crate::quantity::_M<MASS_EXPONENT_2>,
                $crate::quantity::_L<LENGTH_EXPONENT_2>,
                $crate::quantity::_T<TIME_EXPONENT_2>,
                $crate::quantity::_I<CURRENT_EXPONENT_2>,
                $crate::quantity::_Θ<TEMPERATURE_EXPONENT_2>,
                $crate::quantity::_N<AMOUNT_EXPONENT_2>,
                $crate::quantity::_J<LUMINOSITY_EXPONENT_2>,
                $crate::quantity::_A<ANGLE_EXPONENT_2>,
            >,
            $T,
            Brand
        >
    };
}

#[macro_export]
#[doc(hidden)]
#[cfg(has_generic_const_exprs)]
macro_rules! multiplication_output {
    ($T:ty, $log_op:tt) => {
        $crate::quantity::Quantity<
            $crate::quantity::Scale<
                $crate::quantity::_2<{ SCALE_P2_1 $log_op SCALE_P2_2 }>,
                $crate::quantity::_3<{ SCALE_P3_1 $log_op SCALE_P3_2 }>,
                $crate::quantity::_5<{ SCALE_P5_1 $log_op SCALE_P5_2 }>,
                $crate::quantity::_Pi<{ SCALE_PI_1 $log_op SCALE_PI_2 }>,
            >,
            $crate::quantity::Dimension<
                $crate::quantity::_M<{ MASS_EXPONENT_1 $log_op MASS_EXPONENT_2 }>,
                $crate::quantity::_L<{ LENGTH_EXPONENT_1 $log_op LENGTH_EXPONENT_2 }>,
                $crate::quantity::_T<{ TIME_EXPONENT_1 $log_op TIME_EXPONENT_2 }>,
                $crate::quantity::_I<{ CURRENT_EXPONENT_1 $log_op CURRENT_EXPONENT_2 }>,
                $crate::quantity::_Θ<{ TEMPERATURE_EXPONENT_1 $log_op TEMPERATURE_EXPONENT_2 }>,
                $crate::quantity::_N<{ AMOUNT_EXPONENT_1 $log_op AMOUNT_EXPONENT_2 }>,
                $crate::quantity::_J<{ LUMINOSITY_EXPONENT_1 $log_op LUMINOSITY_EXPONENT_2 }>,
                $crate::quantity::_A<{ ANGLE_EXPONENT_1 $log_op ANGLE_EXPONENT_2 }>,
            >,
            $T,
            Brand
        >
    };
}

#[macro_export]
#[doc(hidden)]
#[cfg(not(has_generic_const_exprs))]
macro_rules! multiplication_output {
    ($T:ty, $log_op:tt) => {
        $crate::quantity::Quantity<
            $crate::quantity::Scale<
                $crate::quantity::_2<SCALE_P2>,
                $crate::quantity::_3<SCALE_P3>,
                $crate::quantity::_5<SCALE_P5>,
                $crate::quantity::_Pi<SCALE_PI>,
            >,
            $crate::quantity::Dimension<
                $crate::quantity::_M<MASS_EXPONENT>,
                $crate::quantity::_L<LENGTH_EXPONENT>,
                $crate::quantity::_T<TIME_EXPONENT>,
                $crate::quantity::_I<CURRENT_EXPONENT>,
                $crate::quantity::_Θ<TEMPERATURE_EXPONENT>,
                $crate::quantity::_N<AMOUNT_EXPONENT>,
                $crate::quantity::_J<LUMINOSITY_EXPONENT>,
                $crate::quantity::_A<ANGLE_EXPONENT>,
            >,
            $T,
            Brand
        >
    };
}
