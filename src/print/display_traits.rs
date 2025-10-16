use crate::print::prettyprint::pretty_print_quantity_value;
use crate::quantity_type::Quantity;
use crate::{_2, _3, _5, _A, _I, _J, _L, _M, _N, _Pi, _T, _Θ, Dimension, Scale};
use std::fmt;

#[macro_export]
#[doc(hidden)]
macro_rules! define_display_traits {
    (($($dimension_signature_params:tt)*), ($($dimension_args:tt)*), ($($scale_args:tt)*)) => {
        impl<
            $($dimension_signature_params)*
            T,
        >
            fmt::Display
            for Quantity<
                Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
                T,
            >
        where
            T: Copy + Into<f64>,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let pretty = pretty_print_quantity_value(
                    self.unsafe_value.into(),
                    whippyunits_core::dimension_exponents::DynDimensionExponents([$($dimension_args)*]),
                    whippyunits_core::scale_exponents::ScaleExponents([$($scale_args)*]),
                    std::any::type_name::<T>(),
                    false, // Non-verbose mode for Display
                    true, // Show type in brackets for Display (now unified)
                );
                write!(f, "{}", pretty)
            }
        }

        impl<
            $($dimension_signature_params)*
            T,
        >
            fmt::Debug
            for Quantity<
                Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
                T,
            >
        where
            T: Copy + Into<f64>,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let pretty = pretty_print_quantity_value(
                    self.unsafe_value.into(),
                    whippyunits_core::dimension_exponents::DynDimensionExponents([$($dimension_args)*]),
                    whippyunits_core::scale_exponents::ScaleExponents([$($scale_args)*]),
                    std::any::type_name::<T>(),
                    true, // Verbose mode for Debug
                    true, // Show type in brackets for Debug (now unified)
                );
                write!(f, "{}", pretty)
            }
        }
    };
}
