//! Generated Quantity Type with Full Base Unit Dimensions
//!
//! This file is auto-generated from dimension_data.rs and includes support
//! for all base unit dimensions defined in the system.
//!
//! Base dimensions supported:
//! //! - mass (primes: [10])
//! - length (primes: [10])
//! - time (primes: [2, 3, 5])
//! - current (primes: [10])
//! - temperature (primes: [10])
//! - amount (primes: [10])
//! - luminosity (primes: [10])
//! - angle (primes: [2, 3, 5])

use crate::api::aggregate_scale_factor_float;
use crate::print::format_specifiers::{format_with_unit, UnitFormatSpecifier};
use whippyunits_default_dimensions::lookup_unit_literal;

// Scale exponent structs - these are zero-sized types used for const generic parameters
pub struct _2<const EXP: i16>;
pub struct _3<const EXP: i16>;
pub struct _5<const EXP: i16>;
pub struct _Pi<const EXP: i16>;


// Dimension exponent structs - these are zero-sized types used for const generic parameters
pub struct _M<const EXP: i16>;
pub struct _L<const EXP: i16>;
pub struct _T<const EXP: i16>;
pub struct _I<const EXP: i16>;  // Current
pub struct _Θ<const EXP: i16>;  // Temperature (Unicode theta)
pub struct _N<const EXP: i16>;  // Amount
pub struct _J<const EXP: i16>;  // Luminosity
pub struct _A<const EXP: i16>;  // Angle


// Scale representation - groups scale exponents using wrapper structs
#[allow(dead_code)]
pub struct Scale<
    P2 = _2<0>,
    P3 = _3<0>,
    P5 = _5<0>,
    PI = _Pi<0>,
> {
    _phantom: std::marker::PhantomData<(P2, P3, P5, PI)>,
}

// Dimension representation - groups dimension exponents using wrapper structs
#[allow(dead_code)]
pub struct Dimension<
    MASS = _M<0>,
    LENGTH = _L<0>,
    TIME = _T<0>,
    CURRENT = _I<0>,
    TEMPERATURE = _Θ<0>,
    AMOUNT = _N<0>,
    LUMINOSITY = _J<0>,
    ANGLE = _A<0>,
> {
    _phantom: std::marker::PhantomData<(MASS, LENGTH, TIME, CURRENT, TEMPERATURE, AMOUNT, LUMINOSITY, ANGLE)>,
}



#[derive(Clone, PartialEq)]
pub struct Quantity<
    Scale,
    Dimension,
    T = f64,
> {
    /// The raw numeric value of this quantity.
    ///
    /// **⚠️ WARNING: This property is NOT unit-safe!**
    ///
    /// Direct access to `.value` bypasses the type system's unit safety guarantees.
    /// This should only be used when interacting with non-unit-safe APIs that you don't control, and
    /// only if `.into()` is not available.
    /// 
    /// For unit-safe conversion to underlying numeric types, prefer using `.into()`,
    /// which performs appropriate de-scaling before erasure to the underlying numeric type.
    ///
    /// # Example
    /// ```rust
    /// use whippyunits::*;
    /// 
    /// // ✅ CORRECT: .into() performs proper unit conversion
    /// let angle = 90.0.degrees();
    /// let result = f64::sin(angle.into()); // Works as expected: sin(π/2) ≈ 1.0
    /// 
    /// // ❌ BUG: .value bypasses unit conversion
    /// let result = f64::sin(angle.value); // BUG: sin(90.0) ≈ 0.89 (wrong!)
    /// ```
    pub value: T,
    _phantom: std::marker::PhantomData<fn() -> (Scale, Dimension)>,
}

impl<Scale, Dimension, T> Copy for Quantity<Scale, Dimension, T> 
where 
    Scale: Clone,
    Dimension: Clone,
    T: Copy 
{}

impl<P2, P3, P5, PI> Clone for Scale<P2, P3, P5, PI> {
    fn clone(&self) -> Self {
        Self { _phantom: std::marker::PhantomData }
    }
}

impl<MASS, LENGTH, TIME, CURRENT, TEMPERATURE, AMOUNT, LUMINOSITY, ANGLE> Clone for Dimension<MASS, LENGTH, TIME, CURRENT, TEMPERATURE, AMOUNT, LUMINOSITY, ANGLE> {
    fn clone(&self) -> Self {
        Self { _phantom: std::marker::PhantomData }
    }
}

impl<const EXP: i16> Clone for _2<EXP> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<const EXP: i16> Clone for _3<EXP> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<const EXP: i16> Clone for _5<EXP> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<const EXP: i16> Clone for _Pi<EXP> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<const EXP: i16> Clone for _M<EXP> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<const EXP: i16> Clone for _L<EXP> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<const EXP: i16> Clone for _T<EXP> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<const EXP: i16> Clone for _I<EXP> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<const EXP: i16> Clone for _Θ<EXP> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<const EXP: i16> Clone for _N<EXP> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<const EXP: i16> Clone for _J<EXP> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<const EXP: i16> Clone for _A<EXP> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<
        const MASS_EXPONENT: i16,
        const LENGTH_EXPONENT: i16,
        const TIME_EXPONENT: i16,
        const CURRENT_EXPONENT: i16,
        const TEMPERATURE_EXPONENT: i16,
        const AMOUNT_EXPONENT: i16,
        const LUMINOSITY_EXPONENT: i16,
        const ANGLE_EXPONENT: i16,
        const SCALE_P2: i16,
        const SCALE_P3: i16,
        const SCALE_P5: i16,
        const SCALE_PI: i16,
        T,
    >
    Quantity<
        Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
        Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
        T,
    >
{
    pub const fn new(value: T) -> Self {
        Self { 
            value,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Format this quantity in the specified unit
    ///
    /// Returns a formatter that implements Display, allowing use with println! macros:
    /// ```rust
    /// println!("{}", value.fmt("ft"));
    /// ```
    pub fn fmt(&self, unit: &str) -> impl std::fmt::Display + '_
    where
        T: Copy + Into<f64>,
    {
        let unit = unit.to_string();
        let quantity = self;

        struct Formatter<
            'a,
            const MASS_EXPONENT: i16,
            const LENGTH_EXPONENT: i16,
            const TIME_EXPONENT: i16,
            const CURRENT_EXPONENT: i16,
            const TEMPERATURE_EXPONENT: i16,
            const AMOUNT_EXPONENT: i16,
            const LUMINOSITY_EXPONENT: i16,
            const ANGLE_EXPONENT: i16,
            const SCALE_P2: i16,
            const SCALE_P3: i16,
            const SCALE_P5: i16,
            const SCALE_PI: i16,
            T,
        > {
            quantity: &'a Quantity<
                Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
                T,
            >,
            unit: String,
            precision: Option<usize>,
        }

        impl<
                'a,
                const MASS_EXPONENT: i16,
                const LENGTH_EXPONENT: i16,
                const TIME_EXPONENT: i16,
                const CURRENT_EXPONENT: i16,
                const TEMPERATURE_EXPONENT: i16,
                const AMOUNT_EXPONENT: i16,
                const LUMINOSITY_EXPONENT: i16,
                const ANGLE_EXPONENT: i16,
                const SCALE_P2: i16,
                const SCALE_P3: i16,
                const SCALE_P5: i16,
                const SCALE_PI: i16,
                T,
            > std::fmt::Display
            for Formatter<
                'a,
                MASS_EXPONENT,
                LENGTH_EXPONENT,
                TIME_EXPONENT,
                CURRENT_EXPONENT,
                TEMPERATURE_EXPONENT,
                AMOUNT_EXPONENT,
                LUMINOSITY_EXPONENT,
                ANGLE_EXPONENT,
                SCALE_P2,
                SCALE_P3,
                SCALE_P5,
                SCALE_PI,
                T,
            >
        where
            T: Copy + Into<f64>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // Get target unit info from centralized data
                let target_unit_info = match lookup_unit_literal(&self.unit) {
                    Some(info) => info,
                    None => return write!(f, "Error: Unknown unit: {}", self.unit),
                };

                // Check dimension compatibility
                let source_dims = (
                    MASS_EXPONENT,
                    LENGTH_EXPONENT,
                    TIME_EXPONENT,
                    CURRENT_EXPONENT,
                    TEMPERATURE_EXPONENT,
                    AMOUNT_EXPONENT,
                    LUMINOSITY_EXPONENT,
                    ANGLE_EXPONENT,
                );

                if source_dims != target_unit_info.dimension_exponents {
                    let source_unit_name = crate::print::name_lookup::generate_systematic_unit_name(vec![
                        source_dims.0, source_dims.1, source_dims.2, source_dims.3,
                        source_dims.4, source_dims.5, source_dims.6, source_dims.7,
                    ], false);
                    
                    return write!(
                        f,
                        "Error: Dimension mismatch: cannot convert from {} to {}",
                        source_unit_name, self.unit
                    );
                }

                // Calculate conversion factor
                let conversion_factor = self
                    .quantity
                    .calculate_conversion_factor(&self.unit, &target_unit_info);

                // Convert and format
                let original_value: f64 = self.quantity.value.into();
                let converted_value = original_value * conversion_factor;

                // Use precision from format specifier if available, otherwise use the stored precision
                let precision = f.precision().or(self.precision);

                let spec = UnitFormatSpecifier {
                    target_unit: self.unit.clone(),
                    precision,
                    width: None,
                    alignment: None,
                };

                match format_with_unit(converted_value, &spec) {
                    Ok(formatted) => write!(f, "{}", formatted),
                    Err(e) => write!(f, "Error: {}", e),
                }
            }
        }

        Formatter {
            quantity,
            unit,
            precision: None,
        }
    }


    /// Calculate the conversion factor from the source unit to the target unit
    fn calculate_conversion_factor(
        &self,
        unit: &str,
        target_unit_info: &whippyunits_default_dimensions::UnitLiteralInfo,
    ) -> f64 {
        // For all cases, we need to calculate scale factors first
        let prefix_scale = if let Some(prefix_info) = whippyunits_default_dimensions::lookup_si_prefix(
            &unit[..unit.len() - target_unit_info.symbol.len()],
        ) {
            // Short name prefixed unit (like "km", "cm")
            prefix_info.scale_factor
        } else {
            // Try to find long name prefixed unit (like "kilometer", "centimeter")
            let mut found_prefix_scale = 0;
            for prefix in whippyunits_default_dimensions::SI_PREFIXES {
                for base_unit in whippyunits_default_dimensions::BASE_UNITS {
                    let base_singular = base_unit.long_name;
                    let base_plural = base_unit.long_name.to_string() + "s";

                    if unit.starts_with(prefix.long_name)
                        && (unit.ends_with(base_singular) || unit.ends_with(&base_plural))
                    {
                        let expected_length_singular = prefix.long_name.len() + base_singular.len();
                        let expected_length_plural = prefix.long_name.len() + base_plural.len();

                        if unit.len() == expected_length_singular
                            || unit.len() == expected_length_plural
                        {
                            found_prefix_scale = prefix.scale_factor;
                            break;
                        }
                    }
                }
                if found_prefix_scale != 0 {
                    break;
                }
            }
            found_prefix_scale
        };

        // Calculate target scale factors (all cases use the same logic)
        let (target_p2, target_p3, target_p5, target_pi) = (
            target_unit_info.scale_factors.0 + prefix_scale, // p2 gets prefix
            target_unit_info.scale_factors.1,                // p3 unchanged
            target_unit_info.scale_factors.2 + prefix_scale, // p5 gets prefix
            target_unit_info.scale_factors.3,                // pi unchanged
        );

        // Calculate conversion factor from source to base unit (e.g., meters)
        let scale_factor = aggregate_scale_factor_float(
            SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI, target_p2, target_p3, target_p5, target_pi,
        );

        // If this unit has a bespoke conversion factor (imperial units, time units, etc.),
        // we need to apply it on top of the scale factor
        if let Some(unit_conversion_factor) = target_unit_info.conversion_factor {
            // The scale factor converts from source unit to target unit's base unit
            // The conversion factor converts from target unit's base unit to target unit
            // So the final conversion is: scale_factor / unit_conversion_factor
            scale_factor / unit_conversion_factor
        } else {
            scale_factor
        }
    }
}

// from/into for dimensionless quantities

// proper dimensionless quantities (all exponents are 0, scales irrelevant)
macro_rules! define_from_dimensionless_cross_type {
    ($source_type:ty, $target_type:ty, $rescale_fn:ident) => {
        // Cross-type conversion for dimensionless quantities
        impl<
                const SCALE_P2: i16,
                const SCALE_P3: i16,
                const SCALE_P5: i16,
                const SCALE_PI: i16,
            > From<Quantity<Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>, Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, $source_type>>
            for $target_type
        {
            fn from(
                other: Quantity<
                    Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                    Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>,
                    $source_type,
                >,
            ) -> $target_type {
                // Convert to float first, then apply rescale logic, then convert to target type
                if SCALE_P2 == 0 && SCALE_P3 == 0 && SCALE_P5 == 0 && SCALE_PI == 0 {
                    (other.value as f64) as $target_type
                } else {
                    // Convert to f64 quantity first, then apply rescale logic
                    let f64_quantity = Quantity::<Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>, Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64>::new(other.value as f64);
                    (crate::api::rescale_f64::<
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        SCALE_P2,
                        0,
                        SCALE_P3,
                        0,
                        SCALE_P5,
                        0,
                        SCALE_PI,
                        0,
                    >(f64_quantity)
                    .value) as $target_type
                }
            }
        }
    };
}

macro_rules! define_from_dimensionless {
    ($type:ty, $rescale_fn:ident) => {
        // General case for all scales - rescale from current scale to 0
        /// Unit-safe conversion from dimensionless quantities to underlying numeric types.
        ///
        /// This implementation provides the `.into()` method for dimensionless quantities,
        /// which performs appropriate de-scaling before erasure to the underlying numeric type.
        /// This is the preferred way to extract numeric values from dimensionless quantities
        /// as it maintains unit safety by handling scale conversions correctly.
        impl<
                const SCALE_P2: i16,
                const SCALE_P3: i16,
                const SCALE_P5: i16,
                const SCALE_PI: i16,
            > From<Quantity<Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>, Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, $type>>
            for $type
        {
            fn from(
                other: Quantity<
                    Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                    Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>,
                    $type,
                >,
            ) -> $type {
                // If all scales are zero, just return the raw value
                if SCALE_P2 == 0 && SCALE_P3 == 0 && SCALE_P5 == 0 && SCALE_PI == 0 {
                    other.value
                } else {
                    // Use the provided rescale function
                    crate::api::$rescale_fn::<
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        SCALE_P2,
                        0,
                        SCALE_P3,
                        0,
                        SCALE_P5,
                        0,
                        SCALE_PI,
                        0,
                    >(other)
                    .value
                }
            }
        }
    };
}

define_from_dimensionless!(f32, rescale_f32);
define_from_dimensionless!(f64, rescale_f64);
define_from_dimensionless!(i16, rescale_i16);
define_from_dimensionless!(i32, rescale_i32);
define_from_dimensionless!(i64, rescale_i64);
define_from_dimensionless!(i128, rescale_i128);

// Cross-type conversions for dimensionless quantities
define_from_dimensionless_cross_type!(f32, f64, rescale_f32);
define_from_dimensionless_cross_type!(f64, f32, rescale_f64);
define_from_dimensionless_cross_type!(i16, f32, rescale_i16);
define_from_dimensionless_cross_type!(i16, f64, rescale_i16);
define_from_dimensionless_cross_type!(i32, f32, rescale_i32);
define_from_dimensionless_cross_type!(i32, f64, rescale_i32);
define_from_dimensionless_cross_type!(i64, f32, rescale_i64);
define_from_dimensionless_cross_type!(i64, f64, rescale_i64);
define_from_dimensionless_cross_type!(i128, f32, rescale_i128);
define_from_dimensionless_cross_type!(i128, f64, rescale_i128);
define_from_dimensionless_cross_type!(f32, i32, rescale_f32);
define_from_dimensionless_cross_type!(f64, i32, rescale_f64);
define_from_dimensionless_cross_type!(f32, i64, rescale_f32);
define_from_dimensionless_cross_type!(f64, i64, rescale_f64);

// Cross-type conversion for radian quantities
macro_rules! define_from_for_radians_with_scale_cross_type {
    ($exponent:expr, $source_type:ty, $target_type:ty, $rescale_fn:ident) => {
        /// Unit-safe cross-type conversion from angular quantities to underlying numeric types.
        ///
        /// This implementation provides the `.into()` method for cross-type conversions
        /// from angular quantities, which performs appropriate de-scaling before erasure
        /// to the underlying numeric type. This is the preferred way to extract numeric
        /// values from angular quantities as it maintains unit safety by handling scale
        /// conversions correctly.
        impl<
                const SCALE_P2: i16,
                const SCALE_P3: i16,
                const SCALE_P5: i16,
                const SCALE_PI: i16,
            >
            From<
                Quantity<
                    Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                    Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<$exponent>>,
                    $source_type,
                >,
            > for $target_type
        {
            fn from(
                other: Quantity<
                    Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                    Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<$exponent>>,
                    $source_type,
                >,
            ) -> $target_type {
                // Convert to float first, then apply rescale logic, then convert to target type
                if SCALE_P2 == 0 && SCALE_P3 == 0 && SCALE_P5 == 0 && SCALE_PI == 0 {
                    (other.value as f64) as $target_type
                } else {
                    // Convert to f64 quantity first, then apply rescale logic
                    let f64_quantity = Quantity::<Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>, Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<$exponent>>, f64>::new(other.value as f64);
                    (crate::api::rescale_f64::<
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        $exponent,
                        SCALE_P2,
                        0,
                        SCALE_P3,
                        0,
                        SCALE_P5,
                        0,
                        SCALE_PI,
                        0,
                    >(f64_quantity)
                    .value) as $target_type
                }
            }
        }
    };
}

// Pure radian power to scalar with scale handling - handles both zero and non-zero scales
macro_rules! define_from_for_radians_with_scale {
    ($exponent:expr, $type:ty, $rescale_fn:ident) => {
        /// Unit-safe conversion from angular quantities to underlying numeric types.
        ///
        /// This implementation provides the `.into()` method for angular quantities,
        /// which performs appropriate de-scaling before erasure to the underlying numeric type.
        /// This is the preferred way to extract numeric values from angular quantities
        /// as it maintains unit safety by handling scale conversions correctly.
        impl<
                const SCALE_P2: i16,
                const SCALE_P3: i16,
                const SCALE_P5: i16,
                const SCALE_PI: i16,
            >
            From<
                Quantity<
                    Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                    Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<$exponent>>,
                    $type,
                >,
            > for $type
        {
            fn from(
                other: Quantity<
                    Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                    Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<$exponent>>,
                    $type,
                >,
            ) -> $type {
                // If all scales are zero, just return the raw value
                if SCALE_P2 == 0 && SCALE_P3 == 0 && SCALE_P5 == 0 && SCALE_PI == 0 {
                    other.value
                } else {
                    // Use the provided rescale function
                    crate::api::$rescale_fn::<
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        $exponent,
                        SCALE_P2,
                        0,
                        SCALE_P3,
                        0,
                        SCALE_P5,
                        0,
                        SCALE_PI,
                        0,
                    >(other)
                    .value
                }
            }
        }
    };
}

// radians can be identified as dimensionless (all exponents are 0 except angle, angle scale radians)
// trait resolution rules mean we have to manually template this out over different angle exponents...

macro_rules! define_from_for_radians {
    ($exponent:expr, $($type:ty),+ $(,)?) => {
        $(
            // Removed direct-to-scalar implementation - now handled by define_from_for_radians_with_scale!

            // TODO: This second impl has unconstrained const parameters
            // Need to figure out the correct approach for angle conversions
            impl<
                    const MASS_EXPONENT: i16,
                    const LENGTH_EXPONENT: i16,
                    const TIME_EXPONENT: i16,
                    const CURRENT_EXPONENT: i16,
                    const TEMPERATURE_EXPONENT: i16,
                    const AMOUNT_EXPONENT: i16,
                    const LUMINOSITY_EXPONENT: i16,
                >
                From<
                    Quantity<
                        Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>,
                        Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<$exponent>>,
                        $type,
                    >,
                >
                for Quantity<
                    Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>,
                    Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<0>>,
                    $type,
                >
            {
                fn from(
                    other: Quantity<
                        Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>,
                        Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<$exponent>>,
                        $type,
                    >,
                ) -> Self {
                    Self { 
                        value: other.value,
                        _phantom: std::marker::PhantomData,
                    }
                }
            }
        )+
    };
}

// Generate all radian erasure implementations using unified proc macro
whippyunits_proc_macros::generate_all_radian_erasures!(9);

#[macro_export]
macro_rules! quantity_type {
    () => {
        Quantity<
            Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
            Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
            T
        >
    };
    ($T:ty) => {
        Quantity<
            Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
            Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
            $T
        >
    };
}
