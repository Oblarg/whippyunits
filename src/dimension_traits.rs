use crate::generated_quantity_type::Quantity;
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
            const MASS_SCALE_P10: i8>
        Mass
        for Quantity<
            1, MASS_SCALE_P10, 
            0, $length_unused, 
            0, $time_unused_p2, $time_unused_p3, $time_unused_p5,
            0, 0,
            0, 0,
            0, 0,
            0, 0,
            0, 0, 0, 0, 0> {
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
            const LENGTH_SCALE_P10: i8>
        Length
        for Quantity<
            0, $mass_unused, 
            1, LENGTH_SCALE_P10, 
            0, $time_unused_p2, $time_unused_p3, $time_unused_p5,
            0, 0,
            0, 0,
            0, 0,
            0, 0,
            0, 0, 0, 0, 0> {
            type Unit = Self;
        }

        pub trait Area {
            type Unit;
        }

        #[rustfmt::skip]
        impl<
            const LENGTH_SCALE_P10: i8>
        Area
        for Quantity<
            0, $mass_unused, 
            2, LENGTH_SCALE_P10, 
            0, $time_unused_p2, $time_unused_p3, $time_unused_p5,
            0, 0,
            0, 0,
            0, 0,
            0, 0,
            0, 0, 0, 0, 0> {
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
            const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8>
        Time
        for Quantity<
            0, $mass_unused, 
            0, $length_unused, 
            1, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            0, 0,
            0, 0,
            0, 0,
            0, 0,
            0, 0, 0, 0, 0> {
            type Unit = Self;
        }

        pub trait Frequency {
            type Unit;
        }

        #[rustfmt::skip]
        impl<
            const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8>
        Frequency
        for Quantity<
            0, $mass_unused, 
            0, $length_unused, 
            -1, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            0, 0,
            0, 0,
            0, 0,
            0, 0,
            0, 0, 0, 0, 0> {
            type Unit = Self;
        }
    };
}

// Define the traits with 0 values for all non-strict modes (same as strict now)
define_dimension_traits!(
    0,
    0,
    0, 0, 0
);

