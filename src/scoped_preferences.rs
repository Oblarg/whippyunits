// ============================================================================
// Scoped Preferences Macros
// ============================================================================

use crate::quantity_type::Quantity;
use crate::constants::*;
use crate::scale_conversion::*;

#[macro_export]
macro_rules! get_mass_scale {
    (Milligram) => {
        MILLIGRAM_SCALE_P10
    };
    (Gram) => {
        GRAM_SCALE_P10
    };
    (Kilogram) => {
        KILOGRAM_SCALE_P10
    };
    ($unknown:tt) => {
        compile_error!(concat!("Unknown mass unit: ", stringify!($unknown), ". Use Milligram, Gram, or Kilogram."))
    };
}

#[macro_export]
macro_rules! get_length_scale {
    (Millimeter) => {
        MILLIMETER_SCALE_P10
    };
    (Meter) => {
        METER_SCALE_P10
    };
    (Kilometer) => {
        KILOMETER_SCALE_P10
    };
    ($unknown:tt) => {
        compile_error!(concat!("Unknown length unit: ", stringify!($unknown), ". Use Millimeter, Meter, or Kilometer."))
    };
}

#[macro_export]
macro_rules! get_time_scale_p2 {
    (Millisecond) => {
        MILLISECOND_SCALE_P2
    };
    (Second) => {
        SECOND_SCALE_P2
    };
    (Minute) => {
        MINUTE_SCALE_P2
    };
    ($unknown:tt) => {
        compile_error!(concat!("Unknown time unit: ", stringify!($unknown), ". Use Millisecond, Second, or Minute."))
    };
}

#[macro_export]
macro_rules! get_time_scale_p3 {
    (Millisecond) => {
        MILLISECOND_SCALE_P3
    };
    (Second) => {
        SECOND_SCALE_P3
    };
    (Minute) => {
        MINUTE_SCALE_P3
    };
    ($unknown:tt) => {
        compile_error!(concat!("Unknown time unit: ", stringify!($unknown), ". Use Millisecond, Second, or Minute."))
    };
}

#[macro_export]
macro_rules! get_time_scale_p5 {
    (Millisecond) => {
        MILLISECOND_SCALE_P5
    };
    (Second) => {
        SECOND_SCALE_P5
    };
    (Minute) => {
        MINUTE_SCALE_P5
    };
    ($unknown:tt) => {
        compile_error!(concat!("Unknown time unit: ", stringify!($unknown), ". Use Millisecond, Second, or Minute."))
    };
}

#[rustfmt::skip]
#[macro_export]
macro_rules! set_unit_preferences {
    ($mass_scale:ident, $length_scale:ident, $time_scale:ident) => {
        // Generate type aliases
        type LocalMass = Quantity<
            1, { get_mass_scale!($mass_scale) },
            0, { get_length_scale!($length_scale) },
            0, { get_time_scale_p2!($time_scale) }, { get_time_scale_p3!($time_scale) }, { get_time_scale_p5!($time_scale) },
        >;
        type LocalLength = Quantity<
            0, { get_mass_scale!($mass_scale) },
            1, { get_length_scale!($length_scale) },
            0, { get_time_scale_p2!($time_scale) }, { get_time_scale_p3!($time_scale) }, { get_time_scale_p5!($time_scale) },
        >;
        type LocalTime = Quantity<
            0, { get_mass_scale!($mass_scale) }, 
            0, { get_length_scale!($length_scale) },
            1, { get_time_scale_p2!($time_scale) }, { get_time_scale_p3!($time_scale) }, { get_time_scale_p5!($time_scale) },
        >;

        // Generate local extension traits
        trait ScopedExtensions {
            fn meters(self) -> LocalLength;
            fn kilograms(self) -> LocalMass;
            fn seconds(self) -> LocalTime;
        }

        impl ScopedExtensions for f64 {
            fn meters(self) -> LocalLength {
                const factor: f64 = length_conversion_factor(METER_SCALE_P10, { get_length_scale!($length_scale) }, 1);
                LocalLength::new(self * factor)
            }
            fn kilograms(self) -> LocalMass {
                const factor: f64 = mass_conversion_factor(KILOGRAM_SCALE_P10, { get_mass_scale!($mass_scale) }, 1);
                LocalMass::new(self * factor)
            }
            fn seconds(self) -> LocalTime {
                const factor: f64 = time_conversion_factor(
                    SECOND_SCALE_P2, SECOND_SCALE_P3, SECOND_SCALE_P5,
                    get_time_scale_p2!($time_scale), get_time_scale_p3!($time_scale), get_time_scale_p5!($time_scale), 1
                );
                LocalTime::new(self * factor)
            }
        }

        impl ScopedExtensions for i32 {
            fn meters(self) -> LocalLength {
                const factor: f64 = length_conversion_factor(METER_SCALE_P10, { get_length_scale!($length_scale) }, 1);
                LocalLength::new((self as f64) * factor)
            }
            fn kilograms(self) -> LocalMass {
                const factor: f64 = mass_conversion_factor(KILOGRAM_SCALE_P10, { get_mass_scale!($mass_scale) }, 1);
                LocalMass::new((self as f64) * factor)
            }
            fn seconds(self) -> LocalTime {
                const factor: f64 = time_conversion_factor(
                    SECOND_SCALE_P2, SECOND_SCALE_P3, SECOND_SCALE_P5,
                    get_time_scale_p2!($time_scale), get_time_scale_p3!($time_scale), get_time_scale_p5!($time_scale), 1
                );
                LocalTime::new((self as f64) * factor)
            }
        }

        // Define scoped dimension traits
        #[cfg(feature = "strict")]
        define_dimension_traits!(
            { get_mass_scale!($mass_scale) },
            { get_length_scale!($length_scale) },
            { get_time_scale_p2!($time_scale) }, { get_time_scale_p3!($time_scale) }, { get_time_scale_p5!($time_scale) }
        );

        // Define scoped unit macro
        #[cfg(feature = "strict")]
        define_unit_macro!(
            { get_mass_scale!($mass_scale) },
            { get_length_scale!($length_scale) },
            { get_time_scale_p2!($time_scale) }, { get_time_scale_p3!($time_scale) }, { get_time_scale_p5!($time_scale) }
        );
    };
}
