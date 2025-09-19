use crate::quantity_type::Quantity;

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
            const SCALE_P2: i16,
            const SCALE_P3: i16,
            const SCALE_P5: i16,
            const SCALE_P10: i16,
            const SCALE_PI: i16,
            T>
        Mass
        for Quantity<
            1, 0, 0, 0, 0, 0, 0, 0,
            SCALE_P2, SCALE_P3, SCALE_P5, SCALE_P10, SCALE_PI,
            T> {
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
            const SCALE_P2: i16,
            const SCALE_P3: i16,
            const SCALE_P5: i16,
            const SCALE_P10: i16,
            const SCALE_PI: i16,
            T>
        Length
        for Quantity<
            0, 1, 0, 0, 0, 0, 0, 0,
            SCALE_P2, SCALE_P3, SCALE_P5, SCALE_P10, SCALE_PI,
            T> {
            type Unit = Self;
        }

        pub trait Area {
            type Unit;
        }

        #[rustfmt::skip]
        impl<
            const SCALE_P2: i16,
            const SCALE_P3: i16,
            const SCALE_P5: i16,
            const SCALE_P10: i16,
            const SCALE_PI: i16,
            T>
        Area
        for Quantity<
            0, 2, 0, 0, 0, 0, 0, 0,
            SCALE_P2, SCALE_P3, SCALE_P5, SCALE_P10, SCALE_PI,
            T> {
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
            const SCALE_P2: i16,
            const SCALE_P3: i16,
            const SCALE_P5: i16,
            const SCALE_P10: i16,
            const SCALE_PI: i16,
            T>
        Time
        for Quantity<
            0, 0, 1, 0, 0, 0, 0, 0,
            SCALE_P2, SCALE_P3, SCALE_P5, SCALE_P10, SCALE_PI,
            T> {
            type Unit = Self;
        }

        pub trait Frequency {
            type Unit;
        }

        #[rustfmt::skip]
        impl<
            const SCALE_P2: i16,
            const SCALE_P3: i16,
            const SCALE_P5: i16,
            const SCALE_P10: i16,
            const SCALE_PI: i16,
            T>
        Frequency
        for Quantity<
            0, 0, -1, 0, 0, 0, 0, 0,
            SCALE_P2, SCALE_P3, SCALE_P5, SCALE_P10, SCALE_PI,
            T> {
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

