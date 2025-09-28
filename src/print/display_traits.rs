use crate::print::prettyprint::pretty_print_quantity_value;
use crate::quantity_type::Quantity;
use std::fmt;

#[macro_export]
macro_rules! define_display_traits {
    (($($dimension_signature_params:tt)*), ($($dimension_args:tt)*)) => {
        impl<
            $($dimension_signature_params)*
            T,
        >
            fmt::Display
            for Quantity<
                $($dimension_args)*
                T,
            >
        where
            T: Copy + Into<f64>,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let pretty = pretty_print_quantity_value(
                    self.value.into(),
                    $($dimension_args)*
                    std::any::type_name::<T>(),
                    false, // Non-verbose mode for Display
                    false, // Don't show type in brackets for Display
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
                $($dimension_args)*
                T,
            >
        where
            T: Copy + Into<f64>,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let pretty = pretty_print_quantity_value(
                    self.value.into(),
                    $($dimension_args)*
                    std::any::type_name::<T>(),
                    true, // Verbose mode for Debug
                    false, // Don't show type in brackets for Debug (show as value suffix)
                );
                write!(f, "{}", pretty)
            }
        }
    };
}
