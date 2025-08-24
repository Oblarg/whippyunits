// ============================================================================
// Scoped Preferences Macros
// ============================================================================

use crate::{
    length_conversion_factor, mass_conversion_factor, time_conversion_factor, Quantity, GRAM_SCALE,
    KILOGRAM_SCALE, KILOMETER_SCALE, LENGTH_UNUSED, MASS_UNUSED, METER_SCALE, MILLIGRAM_SCALE,
    MILLIMETER_SCALE, MILLISECOND_SCALE_ORDER, MILLISECOND_SCALE_P2, MILLISECOND_SCALE_P3,
    MILLISECOND_SCALE_P5, MINUTE_SCALE_ORDER, MINUTE_SCALE_P2, MINUTE_SCALE_P3, MINUTE_SCALE_P5,
    SECOND_SCALE_ORDER, SECOND_SCALE_P2, SECOND_SCALE_P3, SECOND_SCALE_P5, TIME_UNUSED,
    time_scale_2, time_scale_3, time_scale_5, IsIsize
};

#[rustfmt::skip]
#[macro_export]
macro_rules! set_unit_preferences {
    ($length_scale:path, $mass_scale:path, $time_scale_order:path) => {
        // Generate type aliases
        type Length = Quantity<
            1, $length_scale,
            0, MASS_UNUSED,
            0, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED,
        >;
        type Mass = Quantity<
            0, LENGTH_UNUSED, 
            1, $mass_scale,
            0, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED,
        >;
        type Time = Quantity<
            0, LENGTH_UNUSED,
            0, MASS_UNUSED, 
            1, { time_scale_2($time_scale_order) }, { time_scale_3($time_scale_order) }, { time_scale_5($time_scale_order) }, $time_scale_order, 
        >;

        // Generate local extension traits
        trait ScopedExtensions {
            fn meters(self) -> Length;
            fn kilograms(self) -> Mass;
            fn seconds(self) -> Time;
        }

        impl ScopedExtensions for f64 {
            fn meters(self) -> Length {
                const factor: f64 = length_conversion_factor(METER_SCALE, $length_scale, 1);
                Length::new(self * factor)
            }
            fn kilograms(self) -> Mass {
                const factor: f64 = mass_conversion_factor(KILOGRAM_SCALE, $mass_scale, 1);
                Mass::new(self * factor)
            }
            fn seconds(self) -> Time {
                const factor: f64 = time_conversion_factor(
                    SECOND_SCALE_P2, SECOND_SCALE_P3, SECOND_SCALE_P5,
                    time_scale_2($time_scale_order), time_scale_3($time_scale_order), time_scale_5($time_scale_order), 1
                );
                Time::new(self * factor)
            }
        }

        impl ScopedExtensions for i32 {
            fn meters(self) -> Length {
                const factor: f64 = length_conversion_factor(METER_SCALE, $length_scale, 1);
                Length::new((self as f64) * factor)
            }
            fn kilograms(self) -> Mass {
                const factor: f64 = mass_conversion_factor(KILOGRAM_SCALE, $mass_scale, 1);
                Mass::new((self as f64) * factor)
            }
            fn seconds(self) -> Time {
                const factor: f64 = time_conversion_factor(
                    SECOND_SCALE_P2, SECOND_SCALE_P3, SECOND_SCALE_P5,
                    time_scale_2($time_scale_order), time_scale_3($time_scale_order), time_scale_5($time_scale_order), 1
                );
                Time::new((self as f64) * factor)
            }
        }
    };
}
