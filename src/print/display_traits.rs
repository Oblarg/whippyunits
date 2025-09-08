use std::fmt;
use crate::print::prettyprint::pretty_print_quantity_value;
use crate::quantity_type::Quantity;

#[macro_export]
macro_rules! define_display_traits {
    (($($const_dimension_params:tt)*), ($($pretty_print_dimension_params:tt)*)) => {
        impl<
            $($const_dimension_params)*,
            T,
        >
            fmt::Display
            for quantity_type!()
        where
            T: Copy + Into<f64>,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let pretty = pretty_print_quantity_value(
                    self.value.into(),
                    $($pretty_print_dimension_params)*,
                    false, // Non-verbose mode for Display
                );
                write!(f, "{}", pretty)
            }
        }

        impl<
            $($const_dimension_params)*,
            T,
        >
            fmt::Debug
            for quantity_type!()
        where
            T: Copy + Into<f64>,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let pretty = pretty_print_quantity_value(
                    self.value.into(),
                    $($pretty_print_dimension_params)*,
                    true, // Verbose mode for Debug
                );
                write!(f, "{}", pretty)
            }
        }
    };
}