// ============================================================================
// Scoped Preferences Macros
// ============================================================================

use crate::{
    length_conversion_factor, mass_conversion_factor, time_conversion_factor, Quantity, GRAM_SCALE,
    KILOGRAM_SCALE, KILOMETER_SCALE, LENGTH_UNUSED, MASS_UNUSED, METER_SCALE, MILLIGRAM_SCALE,
    MILLIMETER_SCALE, MILLISECOND_SCALE_ORDER, MILLISECOND_SCALE_P2, MILLISECOND_SCALE_P3,
    MILLISECOND_SCALE_P5, MINUTE_SCALE_ORDER, MINUTE_SCALE_P2, MINUTE_SCALE_P3, MINUTE_SCALE_P5,
    SECOND_SCALE_ORDER, SECOND_SCALE_P2, SECOND_SCALE_P3, SECOND_SCALE_P5, TIME_UNUSED,
    time_scale_2, time_scale_3, time_scale_5,
};

#[rustfmt::skip]
#[macro_export]
macro_rules! set_unit_preferences {
    ($length_scale:path, $mass_scale:path, $time_scale_order:path, $rescale_behavior:expr, $cancelled_scale_behavior:expr) => {
        // Generate type aliases
        type Length = Quantity<
            1, $length_scale,
            0, MASS_UNUSED,
            0, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED,
            $rescale_behavior, $cancelled_scale_behavior
        >;
        type Mass = Quantity<
            0, LENGTH_UNUSED, 
            1, $mass_scale,
            0, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED,
            $rescale_behavior, $cancelled_scale_behavior
        >;
        type Time = Quantity<
            0, LENGTH_UNUSED, 
            0, MASS_UNUSED, 
            1, { time_scale_2($time_scale_order) }, { time_scale_3($time_scale_order) }, { time_scale_5($time_scale_order) }, $time_scale_order, 
            $rescale_behavior, $cancelled_scale_behavior
        >;

        // Generate local extension traits
        trait ScopedExtensions {
            fn meters(self) -> Length;
            fn kilograms(self) -> Mass;
            fn seconds(self) -> Time;
        }

        impl ScopedExtensions for f64 {
            fn meters(self) -> Length {
                const factor: f64 = length_conversion_factor($length_scale, METER_SCALE, 1);
                Length::new(self * factor)
            }
            fn kilograms(self) -> Mass {
                const factor: f64 = mass_conversion_factor($mass_scale, KILOGRAM_SCALE, 1);
                Mass::new(self * factor)
            }
            fn seconds(self) -> Time {
                const factor: f64 = time_conversion_factor(
                    time_scale_2($time_scale_order), time_scale_3($time_scale_order), time_scale_5($time_scale_order),
                    SECOND_SCALE_P2, SECOND_SCALE_P3, SECOND_SCALE_P5, 1
                );
                Time::new(self * factor)
            }
        }

        impl ScopedExtensions for i32 {
            fn meters(self) -> Length {
                const factor: f64 = length_conversion_factor($length_scale, METER_SCALE, 1);
                Length::new((self as f64) * factor)
            }
            fn kilograms(self) -> Mass {
                const factor: f64 = mass_conversion_factor($mass_scale, KILOGRAM_SCALE, 1);
                Mass::new((self as f64) * factor)
            }
            fn seconds(self) -> Time {
                const factor: f64 = time_conversion_factor(
                    time_scale_2($time_scale_order), time_scale_3($time_scale_order), time_scale_5($time_scale_order),
                    SECOND_SCALE_P2, SECOND_SCALE_P3, SECOND_SCALE_P5, 1
                );
                Time::new((self as f64) * factor)
            }
        }
    };
}

// // ============================================================================
// // Attribute-Style Unit Preferences Macro
// // ============================================================================

// #[macro_export]
// macro_rules! unit_preferences {
//     (
//         $length_scale:path, $mass_scale:path, $time_p2:expr, $time_p3:expr, $time_p5:expr,
//         $rescale_behavior:path, $cancelled_scale_behavior:path,
//         fn $fn_name:ident($($fn_args:tt)*) -> $ret_type:ty {
//             $($fn_body:tt)*
//         }
//     ) => {
//         fn $fn_name($($fn_args)*) -> $ret_type {
//             // Generate type aliases for this function scope
//             type Length = Quantity<1, { $length_scale }, 0, { MassScale::Unused }, 0, 0, 0, 0, { $rescale_behavior }, { $cancelled_scale_behavior }>;
//             type Mass = Quantity<0, { LengthScale::Unused }, 1, { $mass_scale }, 0, 0, 0, 0, { $rescale_behavior }, { $cancelled_scale_behavior }>;
//             type Time = Quantity<0, { LengthScale::Unused }, 0, { MassScale::Unused }, 1, $time_p2, $time_p3, $time_p5, { $rescale_behavior }, { $cancelled_scale_behavior }>;

//             // Generate scoped extension traits
//             trait ScopedExtensions {
//                 fn meters(self) -> Length;
//                 fn kilograms(self) -> Mass;
//                 fn seconds(self) -> Time;
//             }

//             impl ScopedExtensions for f64 {
//                 fn meters(self) -> Length {
//                     let factor = match $length_scale {
//                         LengthScale::Millimeter => 1000.0,
//                         LengthScale::Meter => 1.0,
//                         LengthScale::Kilometer => 0.001,
//                         LengthScale::Unused => 1.0,
//                     };
//                     Length::new(self * factor)
//                 }
//                 fn kilograms(self) -> Mass {
//                     let factor = match $mass_scale {
//                         MassScale::Milligram => 1_000_000.0,
//                         MassScale::Gram => 1000.0,
//                         MassScale::Kilogram => 1.0,
//                         MassScale::Unused => 1.0,
//                     };
//                     Mass::new(self * factor)
//                 }
//                 fn seconds(self) -> Time {
//                     let factor = whippyunits::pow2($time_p2) * whippyunits::pow3($time_p3) * whippyunits::pow5($time_p5);
//                     Time::new(self * factor)
//                 }
//             }

//             impl ScopedExtensions for i32 {
//                 fn meters(self) -> Length {
//                     let factor = match $length_scale {
//                         LengthScale::Millimeter => 1000.0,
//                         LengthScale::Meter => 1.0,
//                         LengthScale::Kilometer => 0.001,
//                         LengthScale::Unused => 1.0,
//                     };
//                     Length::new((self as f64) * factor)
//                 }
//                 fn kilograms(self) -> Mass {
//                     let factor = match $mass_scale {
//                         MassScale::Milligram => 1_000_000.0,
//                         MassScale::Gram => 1000.0,
//                         MassScale::Kilogram => 1.0,
//                         MassScale::Unused => 1.0,
//                     };
//                     Mass::new((self as f64) * factor)
//                 }
//                 fn seconds(self) -> Time {
//                     let factor = whippyunits::pow2($time_p2) * whippyunits::pow3($time_p3) * whippyunits::pow5($time_p5);
//                     Time::new((self as f64) * factor)
//                 }
//             }

//             // Execute the function body
//             $($fn_body)*
//         }
//     };
// }

// // ============================================================================
// // Function-Scoped Preferences Macro
// // ============================================================================

// #[macro_export]
// macro_rules! with_unit_preferences {
//     (
//         $length_scale:path, $mass_scale:path, $time_p2:expr, $time_p3:expr, $time_p5:expr,
//         $rescale_behavior:path, $cancelled_scale_behavior:path,
//         $($body:tt)*
//     ) => {
//         {
//             // Generate type aliases for this scope
//             type Length = whippyunits::Quantity<1, { $length_scale }, 0, { whippyunits::MassScale::Unused }, 0, 0, 0, 0, { $rescale_behavior }, { $cancelled_scale_behavior }>;
//             type Mass = whippyunits::Quantity<0, { whippyunits::LengthScale::Unused }, 1, { $mass_scale }, 0, 0, 0, 0, { $rescale_behavior }, { $cancelled_scale_behavior }>;
//             type Time = whippyunits::Quantity<0, { whippyunits::LengthScale::Unused }, 0, { whippyunits::MassScale::Unused }, 1, $time_p2, $time_p3, $time_p5, { $rescale_behavior }, { $cancelled_scale_behavior }>;

//             // Generate scoped extension traits
//             trait ScopedExtensions {
//                 fn meters(self) -> Length;
//                 fn kilograms(self) -> Mass;
//                 fn seconds(self) -> Time;
//             }

//             impl ScopedExtensions for f64 {
//                 fn meters(self) -> Length {
//                     let factor = match $length_scale {
//                         LengthScale::Millimeter => 1000.0,
//                         LengthScale::Meter => 1.0,
//                         LengthScale::Kilometer => 0.001,
//                         LengthScale::Unused => 1.0,
//                     };
//                     Length::new(self * factor)
//                 }
//                 fn kilograms(self) -> Mass {
//                     let factor = match $mass_scale {
//                         MassScale::Milligram => 1_000_000.0,
//                         MassScale::Gram => 1000.0,
//                         MassScale::Kilogram => 1.0,
//                         MassScale::Unused => 1.0,
//                     };
//                     Mass::new(self * factor)
//                 }
//                 fn seconds(self) -> Time {
//                     let factor = whippyunits::pow2($time_p2) * whippyunits::pow3($time_p3) * whippyunits::pow5($time_p5);
//                     Time::new(self * factor)
//                 }
//             }

//             impl ScopedExtensions for i32 {
//                 fn meters(self) -> Length {
//                     let factor = match $length_scale {
//                         LengthScale::Millimeter => 1000.0,
//                         LengthScale::Meter => 1.0,
//                         LengthScale::Kilometer => 0.001,
//                         LengthScale::Unused => 1.0,
//                     };
//                     Length::new((self as f64) * factor)
//                 }
//                 fn kilograms(self) -> Mass {
//                     let factor = match $mass_scale {
//                         MassScale::Milligram => 1_000_000.0,
//                         MassScale::Gram => 1000.0,
//                         MassScale::Kilogram => 1.0,
//                         MassScale::Unused => 1.0,
//                     };
//                     Mass::new((self as f64) * factor)
//                 }
//                 fn seconds(self) -> Time {
//                     let factor = whippyunits::pow2($time_p2) * whippyunits::pow3($time_p3) * whippyunits::pow5($time_p5);
//                     Time::new((self as f64) * factor)
//                 }
//             }

//             // Execute the body
//             $($body)*
//         }
//     };
// }
