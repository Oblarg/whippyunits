#[macro_export]
#[doc(hidden)]
macro_rules! define_display_traits {
    (($($dimension_signature_params:tt)*), ($($dimension_args:tt)*), ($($scale_args:tt)*)) => {
        impl<
            $($dimension_signature_params)*
            T,
            Brand,
        >
            fmt::Display
            for Quantity<
                Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
                T,
                Brand,
            >
        where
            T: Copy + num_traits::NumCast,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let value_f64 = <f64 as num_traits::NumCast>::from(self.unsafe_value)
                    .expect("unable to convert numeric value to f64 for display");
                #[cfg(feature = "std")]
                let brand_name = std::any::type_name::<Brand>();
                #[cfg(not(feature = "std"))]
                let brand_name = "<unknown>";
                #[cfg(feature = "std")]
                let type_name = std::any::type_name::<T>();
                #[cfg(not(feature = "std"))]
                let type_name = "<T>";
                let pretty = pretty_print_quantity_value(
                    value_f64,
                    whippyunits_core::dimension_exponents::DynDimensionExponents([$($dimension_args)*]),
                    whippyunits_core::scale_exponents::ScaleExponents([$($scale_args)*]),
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
            fmt::Debug
            for Quantity<
                Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
                T,
                Brand,
            >
        where
            T: Copy + num_traits::NumCast,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let value_f64 = <f64 as num_traits::NumCast>::from(self.unsafe_value)
                    .expect("unable to convert numeric value to f64 for debug");
                #[cfg(feature = "std")]
                let brand_name = std::any::type_name::<Brand>();
                #[cfg(not(feature = "std"))]
                let brand_name = "<unknown>";
                #[cfg(feature = "std")]
                let type_name = std::any::type_name::<T>();
                #[cfg(not(feature = "std"))]
                let type_name = "<T>";
                let pretty = pretty_print_quantity_value(
                    value_f64,
                    whippyunits_core::dimension_exponents::DynDimensionExponents([$($dimension_args)*]),
                    whippyunits_core::scale_exponents::ScaleExponents([$($scale_args)*]),
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
