use crate::quantity_type::Quantity;

/// Macro to define dimension traits for atomic dimensions
///
/// This macro generates a trait and its implementation for a specific atomic dimension.
/// It follows the same pattern as the default declarators but focuses only on the
/// trait definition and implementation for scale-generic quantities.
#[macro_export]
macro_rules! define_atomic_dimension_trait {
    (
        $mass_exp:expr, $length_exp:expr, $time_exp:expr, $current_exp:expr,
        $temperature_exp:expr, $amount_exp:expr, $luminosity_exp:expr, $angle_exp:expr,
        $trait_name:ident
    ) => {
        /// Trait for quantities with the specified atomic dimension
        pub trait $trait_name {
            type Unit;
        }

#[rustfmt::skip]
        impl<
            const SCALE_P2: i16,
            const SCALE_P3: i16,
            const SCALE_P5: i16,
            const SCALE_PI: i16,
            T>
        $trait_name
        for Quantity<
            $mass_exp, $length_exp, $time_exp, $current_exp,
            $temperature_exp, $amount_exp, $luminosity_exp, $angle_exp,
            SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI,
            T> {
            type Unit = Self;
        }
    };
}

// Define traits for all 8 atomic dimensions (SI base quantities)
define_atomic_dimension_trait!(1, 0, 0, 0, 0, 0, 0, 0, Mass);
define_atomic_dimension_trait!(0, 1, 0, 0, 0, 0, 0, 0, Length);
define_atomic_dimension_trait!(0, 0, 1, 0, 0, 0, 0, 0, Time);
define_atomic_dimension_trait!(0, 0, 0, 1, 0, 0, 0, 0, Current);
define_atomic_dimension_trait!(0, 0, 0, 0, 1, 0, 0, 0, Temperature);
define_atomic_dimension_trait!(0, 0, 0, 0, 0, 1, 0, 0, Amount);
define_atomic_dimension_trait!(0, 0, 0, 0, 0, 0, 1, 0, Luminosity);
define_atomic_dimension_trait!(0, 0, 0, 0, 0, 0, 0, 1, Angle);
