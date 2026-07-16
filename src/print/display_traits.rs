#[macro_export]
#[doc(hidden)]
macro_rules! define_display_traits {
    (($($dimension_signature_params:tt)*), ($($dimension_args:tt)*), ($($scale_args:tt)*)) => {
        impl<
            $($dimension_signature_params)*
            T,
            Brand,
        >
            ::core::fmt::Display
            for $crate::quantity::Quantity<
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
                T,
                Brand,
            >
        where
            T: Copy + $crate::__private_num_traits::NumCast,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let value_f64 = <f64 as $crate::__private_num_traits::NumCast>::from(self.unsafe_value)
                    .expect("unable to convert numeric value to f64 for display");
                #[cfg(feature = "std")]
                let brand_name = ::std::any::type_name::<Brand>();
                #[cfg(not(feature = "std"))]
                let brand_name = "<unknown>";
                #[cfg(feature = "std")]
                let type_name = ::std::any::type_name::<T>();
                #[cfg(not(feature = "std"))]
                let type_name = "<T>";
                let pretty = $crate::print::prettyprint::pretty_print_quantity_value(
                    value_f64,
                    $crate::__private_whippyunits_core::dimension_exponents::DynDimensionExponents([$($dimension_args)*]),
                    $crate::__private_whippyunits_core::scale_exponents::ScaleExponents([$($scale_args)*]),
                    type_name,
                    false, // Non-verbose mode for Display
                    true, // Show type in brackets for Display (now unified)
                    Some(brand_name),
                );
                write!(f, "{}", pretty)
            }
        }

        impl<
            $($dimension_signature_params)*
            T,
            Brand,
        >
            ::core::fmt::Debug
            for $crate::quantity::Quantity<
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
                T,
                Brand,
            >
        where
            T: Copy + $crate::__private_num_traits::NumCast,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let value_f64 = <f64 as $crate::__private_num_traits::NumCast>::from(self.unsafe_value)
                    .expect("unable to convert numeric value to f64 for debug");
                #[cfg(feature = "std")]
                let brand_name = ::std::any::type_name::<Brand>();
                #[cfg(not(feature = "std"))]
                let brand_name = "<unknown>";
                #[cfg(feature = "std")]
                let type_name = ::std::any::type_name::<T>();
                #[cfg(not(feature = "std"))]
                let type_name = "<T>";
                let pretty = $crate::print::prettyprint::pretty_print_quantity_value(
                    value_f64,
                    $crate::__private_whippyunits_core::dimension_exponents::DynDimensionExponents([$($dimension_args)*]),
                    $crate::__private_whippyunits_core::scale_exponents::ScaleExponents([$($scale_args)*]),
                    type_name,
                    true, // Verbose mode for Debug
                    true, // Show type in brackets for Debug (now unified)
                    Some(brand_name),
                );
                write!(f, "{}", pretty)
            }
        }
    };
}
