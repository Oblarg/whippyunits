use crate::quantity_type::Quantity;
use crate::constants::*;

#[macro_export]
macro_rules! define_dimension_traits {
    (
        $mass_unused:expr,
        $length_unused:expr,
        $time_unused_p2:expr, $time_unused_p3:expr, $time_unused_p5:expr
    ) => {
        // ===========================================================================
        // MASS-like units
        // ===========================================================================

        pub trait Mass {
            type Unit;
        }

        #[rustfmt::skip]
        impl<
            const MASS_SCALE_P10: isize>
        Mass
        for Quantity<
            1, MASS_SCALE_P10, 
            0, $length_unused, 
            0, $time_unused_p2, $time_unused_p3, $time_unused_p5> {
            type Unit = Self;
        }

        // ===========================================================================
        // LENGTH-like units
        // ===========================================================================

        pub trait Length {
            type Unit;
        }

        #[rustfmt::skip]
        impl<
            const LENGTH_SCALE_P10: isize>
        Length
        for Quantity<
            0, $mass_unused, 
            1, LENGTH_SCALE_P10, 
            0, $time_unused_p2, $time_unused_p3, $time_unused_p5> {
            type Unit = Self;
        }

        pub trait Area {
            type Unit;
        }

        #[rustfmt::skip]
        impl<
            const LENGTH_SCALE_P10: isize>
        Area
        for Quantity<
            0, $mass_unused, 
            2, LENGTH_SCALE_P10, 
            0, $time_unused_p2, $time_unused_p3, $time_unused_p5> {
            type Unit = Self;
        }

        // ===========================================================================
        // TIME-like units
        // ===========================================================================

        pub trait Time {
            type Unit;
        }

        #[rustfmt::skip]
        impl<
            const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize>
        Time
        for Quantity<
            0, $mass_unused, 
            0, $length_unused, 
            1, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5> {
            type Unit = Self;
        }

        pub trait Frequency {
            type Unit;
        }

        #[rustfmt::skip]
        impl<
            const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize>
        Frequency
        for Quantity<
            0, $mass_unused, 
            0, $length_unused, 
            -1, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5> {
            type Unit = Self;
        }
    };
}

#[cfg(feature = "strict")]
// Define the traits with specific scale values for strict mode
define_dimension_traits!(
    KILOGRAM_SCALE_P10,
    METER_SCALE_P10,
    SECOND_SCALE_P2, SECOND_SCALE_P3, SECOND_SCALE_P5
);

#[cfg(not(feature = "strict"))]
// Define the traits with 0 values for all non-strict modes (same as strict now)
define_dimension_traits!(
    0,
    0,
    0, 0, 0
);

