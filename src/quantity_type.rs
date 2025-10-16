// No longer need api_helpers imports - this file doesn't use them

/// The base-2 scale exponent of a quantity.
pub struct _2<const EXP: i16 = 0>;
/// The base-3 scale exponent of a quantity.
pub struct _3<const EXP: i16 = 0>;
/// The base-5 scale exponent of a quantity.
pub struct _5<const EXP: i16 = 0>;
/// The base-π scale exponent of a quantity - used for angular units.
pub struct _Pi<const EXP: i16 = 0>;

/// The mass dimension exponent of a quantity.
pub struct _M<const EXP: i16 = 0>;
/// The length dimension exponent of a quantity.
pub struct _L<const EXP: i16 = 0>;
/// The time dimension exponent of a quantity.
pub struct _T<const EXP: i16 = 0>;
/// The current dimension exponent of a quantity.
pub struct _I<const EXP: i16 = 0>;
/// The temperature dimension exponent of a quantity.
pub struct _Θ<const EXP: i16 = 0>;
/// The amount dimension exponent of a quantity.
pub struct _N<const EXP: i16 = 0>;
/// The luminosity dimension exponent of a quantity.
pub struct _J<const EXP: i16 = 0>;
/// The angle dimension exponent of a quantity.
pub struct _A<const EXP: i16 = 0>;

/// The scale of a quantity
///
/// If all scale exponents are zero, the quantity is in SI base units
/// (kilogram, meter, second, ampere, kelvin, mole, candela, radian, or
/// some combination thereof).
///
/// SI prefixes indicate scales of 10^n = _2<n>, _3<0>, _5<n>, _Pi<0>, e.g.
///  - milli: 10^-3 = _2<-3>, _3<0>, _5<-3>, _Pi<0>
///  - kilo: 10^3 = _2<3>, _3<0>, _5<3>, _Pi<0>
///
/// Certain time units involve factors of 60^n = _2<2>, _3<1>, _5<1>, _Pi<0>, e.g.
///  - minute: 60 = _2<2>, _3<1>, _5<1>, _Pi<0>
///  - hour: 3600 = _2<4>, _3<2>, _5<2>, _Pi<0>
///
/// Angular units can involve "all of the above", plus a possible factor of π:
///  - revolution: 2π = _2<1>, _3<0>, _5<0>, _Pi<1>
///  - degree: π/180 = _2<-2>, _3<-2>, _5<-1>, _Pi<1>
///  - arcminute: π/10800 = _2<-4>, _3<-2>, _5<-2>, _Pi<1>
#[allow(dead_code)]
pub struct Scale<P2 = _2<0>, P3 = _3<0>, P5 = _5<0>, PI = _Pi<0>> {
    _phantom: std::marker::PhantomData<(P2, P3, P5, PI)>,
}

/// The dimension of a quantity
///
/// If all dimension exponents are zero, the quantity is dimensionless.
///
/// Atomic dimensions have a single dimension exponent of 1:
///  - length: _L<1>
///  - mass: _M<1>
///  - time: _T<1>
///  - current: _I<1>
///  - temperature: _Θ<1>
///  - amount: _N<1>
///  - luminosity: _J<1>
///  - angle: _A<1>
///
/// Derived dimensions have a mixture of dimension exponents:
///  - velocity: _L<1>, _T<-1>
///  - acceleration: _L<1>, _T<-2>
///  - force: _M<1>, _L<1>, _T<-2>
///  - energy: _M<1>, _L<2>, _T<-2>
///  - power: _M<1>, _L<2>, _T<-3>
///  - pressure: _M<1>, _L<-1>, _T<-2>
///  - frequency: _T<-1>
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
    _phantom: std::marker::PhantomData<(
        MASS,
        LENGTH,
        TIME,
        CURRENT,
        TEMPERATURE,
        AMOUNT,
        LUMINOSITY,
        ANGLE,
    )>,
}

/// A quantity with a specified scale, dimension, and numeric type.
///
/// Since the type is highly-parameterized, direct usage is discouraged.  Interaction with the
/// Quantity type should generally be done through an API method, the [quantity!](crate::quantity!) macro,
/// or a [literal macro](crate::define_literals!), which will handle the const generic parameters for you:
///
/// ```rust
/// // declarator method
/// let distance = 1.0.meters();
///
/// // quantity! macro
/// let distance = quantity!(1.0, m);
///
/// // literal (only in scopes tagged with #[culit::culit]):
/// let distance = 1.0m;
/// ```
///
/// If you want a concrete Quantity type *as a type*, use the [unit!](crate::unit!) macro:
///
/// ```rust
/// // explicit type assertion provides additional unit safety
/// let area: unit!(m^2) = distance * distance;
/// ```
///
/// Because quantity scale is represented at compile-time, the runtime value of a quantity
/// may differ from its "semantic" value in code by a factor of the scale, and it is generally advised
/// to avoid accessing the underlying value directly.  Access to the underlying
/// value via the [value!](crate::value!) macro is unit-safe, as is "erasure" of dimensionless or angular
/// quantities via `from/into`.
///
/// Quantity is a zero-cost wrapper type - at runtime, your binary will only contain
/// the underlying numeric type.  Accordingly, the dimensionality of any quantity represented by
/// a Quantity type must be known at compile time.  Whippyunits does *not* support unit-safe operations
/// on values whose dimensionality is only known at runtime, e.g. as deserialized from a JSON string,
/// unless all possible runtime dimensionalities of the quantity are each given their own statically-declared
/// code branch.
#[derive(Clone, PartialEq)]
pub struct Quantity<Scale, Dimension, T = f64> {
    /// The raw numeric value of this quantity.
    ///
    /// **⚠️ WARNING: This property is NOT unit-safe!**
    ///
    /// Direct access to `.unsafe_value` bypasses the type system's unit safety guarantees.
    /// This should only be used when interacting with non-unit-safe APIs that you don't control,
    /// and only if the unit-safe methods outlined below are not viable.
    ///
    /// ## Example
    /// ```rust
    /// use whippyunits::*;
    ///
    /// let angle = 90.0.degrees(); // erasable unit
    /// let distance = quantity!(1.0, m); // non-erasable unit
    ///
    /// // ✅ CORRECT: .into() for erasable units (dimensionless/angular)
    /// let val: f64 = f64::sin(angle.into()); // sin(π/2) ≈ 1.0
    ///
    /// // ✅ CORRECT: value! macro or division by reference quantity + .into() for
    /// // non-erasable units (anything else)
    /// let millimeters: f64 = value!(distance, mm); // 1000.0 (1m = 1000mm);
    /// let millimeters: f64 = (distance / quantity!(1.0, mm)).into();
    ///
    /// // ❌ BUG: .unsafe_value bypasses unit conversion for erasable units,
    /// //         returns in degrees (not radians)
    /// let val: f64 = f64::sin(90.0.degrees().unsafe_value); // BUG: sin(90.0) ≈ 0.89 (wrong!)
    ///
    /// // ❌ BUG: .unsafe_value bypasses dimensional/scale safety for non-erasable units,
    /// //         returns in meters (not millimeters)
    /// let millimeters: f64 = distance.unsafe_value;
    /// ```
    pub unsafe_value: T,
    _phantom: std::marker::PhantomData<fn() -> (Scale, Dimension)>,
}

impl<Scale, Dimension, T> Copy for Quantity<Scale, Dimension, T>
where
    Scale: Clone,
    Dimension: Clone,
    T: Copy,
{
}

impl<P2, P3, P5, PI> Clone for Scale<P2, P3, P5, PI> {
    fn clone(&self) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<MASS, LENGTH, TIME, CURRENT, TEMPERATURE, AMOUNT, LUMINOSITY, ANGLE> Clone
    for Dimension<MASS, LENGTH, TIME, CURRENT, TEMPERATURE, AMOUNT, LUMINOSITY, ANGLE>
{
    fn clone(&self) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
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
        Dimension<
            _M<MASS_EXPONENT>,
            _L<LENGTH_EXPONENT>,
            _T<TIME_EXPONENT>,
            _I<CURRENT_EXPONENT>,
            _Θ<TEMPERATURE_EXPONENT>,
            _N<AMOUNT_EXPONENT>,
            _J<LUMINOSITY_EXPONENT>,
            _A<ANGLE_EXPONENT>,
        >,
        T,
    >
{
    pub const fn new(unsafe_value: T) -> Self {
        Self {
            unsafe_value,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Format this quantity in the specified unit
    ///
    /// Returns a formatter that implements Display, allowing use with println! macros:
    /// ```rust
    /// println!("{}", value.fmt("ft")); // "1000.0 ft"
    /// ```
    ///
    /// Dimensionally-incompatible units will print an error message, but will *not* panic:
    ///
    /// ```rust
    /// println!("{}", value.fmt("kg")); // "Error: Dimension mismatch: cannot convert from m to kg"
    /// ```
    ///
    /// if a panic is desired, use a type assertion instead.
    pub fn fmt(&self, unit: &str) -> impl std::fmt::Display + '_
    where
        T: Copy + Into<f64>,
    {
        use crate::api::aggregate_scale_factor_float;
        use whippyunits_core::{
            Dimension, dimension_exponents::DynDimensionExponents, scale_exponents::ScaleExponents,
        };

        // Look up the target unit (handles prefixed units like "km")
        if let Some((target_unit, target_dimension, prefix)) = Dimension::find_by_literal(unit) {
            // Check if dimensions match
            let source_dimensions = DynDimensionExponents([
                MASS_EXPONENT,
                LENGTH_EXPONENT,
                TIME_EXPONENT,
                CURRENT_EXPONENT,
                TEMPERATURE_EXPONENT,
                AMOUNT_EXPONENT,
                LUMINOSITY_EXPONENT,
                ANGLE_EXPONENT,
            ]);

            if source_dimensions == target_dimension.exponents {
                // Dimensions match, calculate conversion factor
                let source_scales = ScaleExponents([SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI]);

                // Calculate target scales: base unit scales + prefix scales
                let mut target_scales = target_unit.scale;
                if let Some(prefix) = prefix {
                    let prefix_scale = prefix.factor_log10();
                    // For SI prefixes, add the scale to both p2 and p5 (since 10^n = 2^n * 5^n)
                    target_scales = ScaleExponents([
                        target_scales.0[0] + prefix_scale, // p2
                        target_scales.0[1],                // p3
                        target_scales.0[2] + prefix_scale, // p5
                        target_scales.0[3],                // pi
                    ]);
                }

                let conversion_factor = aggregate_scale_factor_float(
                    source_scales.0[0],
                    source_scales.0[1],
                    source_scales.0[2],
                    source_scales.0[3],
                    target_scales.0[0],
                    target_scales.0[1],
                    target_scales.0[2],
                    target_scales.0[3],
                );

                // Apply unit conversion factor (inverted for imperial units)
                let unit_conversion_factor = if target_unit.conversion_factor != 1.0 {
                    1.0 / target_unit.conversion_factor
                } else {
                    1.0
                };

                let converted_value: f64 =
                    self.unsafe_value.into() * conversion_factor * unit_conversion_factor;

                // Return a formatter that displays the converted value with the unit
                QuantityFormatter {
                    value: converted_value,
                    unit: unit.to_string(),
                    is_error: false,
                }
            } else {
                // Dimension mismatch - return error formatter
                QuantityFormatter {
                    value: 0.0,
                    unit: format!(
                        "Error: Dimension mismatch: cannot convert from {} to {}",
                        self.get_source_unit_symbol(),
                        unit
                    ),
                    is_error: true,
                }
            }
        } else {
            // Unit not found - return error formatter
            QuantityFormatter {
                value: 0.0,
                unit: format!("Error: Unknown unit: {}", unit),
                is_error: true,
            }
        }
    }

    /// Get the source unit symbol for error messages
    fn get_source_unit_symbol(&self) -> String {
        use whippyunits_core::{
            Dimension, dimension_exponents::DynDimensionExponents, scale_exponents::ScaleExponents,
        };

        // Create the source dimensions and scales
        let source_dimensions = DynDimensionExponents([
            MASS_EXPONENT,
            LENGTH_EXPONENT,
            TIME_EXPONENT,
            CURRENT_EXPONENT,
            TEMPERATURE_EXPONENT,
            AMOUNT_EXPONENT,
            LUMINOSITY_EXPONENT,
            ANGLE_EXPONENT,
        ]);
        let source_scales = ScaleExponents([SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI]);

        // Try to find a matching unit
        if let Some(dimension) = Dimension::find_dimension_by_exponents(source_dimensions) {
            // Look for a unit with matching scales and conversion_factor = 1.0 (SI base units)
            if let Some(unit) = dimension
                .units
                .iter()
                .find(|unit| unit.scale == source_scales && unit.conversion_factor == 1.0)
            {
                return unit.symbols[0].to_string();
            }

            // If no exact match, try to find any unit in this dimension
            if let Some(unit) = dimension.units.first() {
                return unit.symbols[0].to_string();
            }
        }

        // Fallback to dimension symbol if no unit found
        if let Some(dimension) = Dimension::find_dimension_by_exponents(source_dimensions) {
            return dimension.symbol.to_string();
        }

        // Final fallback
        "unknown unit".to_string()
    }
}

/// A formatter for displaying quantities with unit conversion
struct QuantityFormatter {
    value: f64,
    unit: String,
    is_error: bool,
}

impl std::fmt::Display for QuantityFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_error {
            write!(f, "{}", self.unit)
        } else {
            // Use the formatter's precision if specified, otherwise use default formatting
            if let Some(precision) = f.precision() {
                write!(
                    f,
                    "{:.precision$} {}",
                    self.value,
                    self.unit,
                    precision = precision
                )
            } else {
                write!(f, "{} {}", self.value, self.unit)
            }
        }
    }
}

// from/into for dimensionless quantities

// proper dimensionless quantities (all exponents are 0, scales irrelevant)
#[doc(hidden)]
macro_rules! define_from_dimensionless_cross_type {
    ($source_type:ty, $target_type:ty, $rescale_fn:ident) => {
        /// Converts dimensionless quantities between different numeric types with proper scaling.
        ///
        /// Performs de-scaling before type conversion to ensure unit-safe numeric extraction.
        ///
        /// ## Examples
        /// ```rust
        /// use whippyunits::*;
        ///
        /// // Cross-type conversion from f32 to f64
        /// let dimensionless_f32 = (1.0f32.meters() / 1.0f32.meters());
        /// let result_f64: f64 = dimensionless_f32.into();
        /// assert_eq!(result_f64, 1.0);
        ///
        /// // Cross-type conversion with scale handling
        /// let ratio_f32 = (1.0f32.meters() / 1.0f32.millimeters());
        /// let result_f64: f64 = ratio_f32.into();
        /// assert_eq!(result_f64, 1000.0);
        /// ```
        impl<const SCALE_P2: i16, const SCALE_P3: i16, const SCALE_P5: i16, const SCALE_PI: i16>
            From<
                Quantity<
                    Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                    Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>,
                    $source_type,
                >,
            > for $target_type
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
                    (other.unsafe_value as f64) as $target_type
                } else {
                    // Convert to f64 quantity first, then apply rescale logic
                    let f64_quantity = Quantity::<
                        Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                        Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>,
                        f64,
                    >::new(other.unsafe_value as f64);
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
                    .unsafe_value) as $target_type
                }
            }
        }
    };
}

#[doc(hidden)]
macro_rules! define_from_dimensionless {
    ($type:ty, $rescale_fn:ident) => {
        /// Converts dimensionless quantities to underlying numeric types with proper scaling.
        ///
        /// Performs de-scaling before erasure to ensure unit-safe numeric extraction.
        /// Dimensionless quantities with non-unity storage scales are rescaled to unity.
        ///
        /// ## Examples
        /// ```rust
        /// use whippyunits::*;
        ///
        /// // Division yields dimensionless quantity
        /// let dimensionless: f64 = (1.0.meters() / 1.0.meters()).into();
        /// assert_eq!(dimensionless, 1.0);
        ///
        /// // Non-unity scales are rescaled before erasure
        /// let ratio: f64 = (1.0.meters() / 1.0.millimeters()).into();
        /// assert_eq!(ratio, 1000.0);
        /// ```
        impl<const SCALE_P2: i16, const SCALE_P3: i16, const SCALE_P5: i16, const SCALE_PI: i16>
            From<
                Quantity<
                    Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                    Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>,
                    $type,
                >,
            > for $type
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
                    other.unsafe_value
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
                    .unsafe_value
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
#[doc(hidden)]
macro_rules! define_from_for_radians_with_scale_cross_type {
    ($exponent:expr, $source_type:ty, $target_type:ty, $rescale_fn:ident) => {
        /// Converts angular quantities between different numeric types in radian scale.
        ///
        /// Performs de-scaling before type conversion, ensuring all angular values are converted to radians.
        ///
        /// ## Examples
        /// ```rust
        /// use whippyunits::*;
        ///
        /// // Cross-type conversion from f32 to f64
        /// let angle_f32 = 90.0f32.degrees();
        /// let result_f64: f64 = angle_f32.into();
        /// assert_eq!(result_f64, std::f64::consts::PI / 2.0);
        ///
        /// // Enables unit-safe trigonometric functions with cross-type conversion
        /// let sin_value: f64 = f64::sin(90.0f32.degrees().into());
        /// assert_eq!(sin_value, 1.0);
        /// ```
        impl<const SCALE_P2: i16, const SCALE_P3: i16, const SCALE_P5: i16, const SCALE_PI: i16>
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
                    (other.unsafe_value as f64) as $target_type
                } else {
                    // Convert to f64 quantity first, then apply rescale logic
                    let f64_quantity = Quantity::<
                        Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                        Dimension<_M<0>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<$exponent>>,
                        f64,
                    >::new(other.unsafe_value as f64);
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
                    .unsafe_value) as $target_type
                }
            }
        }
    };
}

// Pure radian power to scalar with scale handling - handles both zero and non-zero scales
#[doc(hidden)]
macro_rules! define_from_for_radians_with_scale {
    ($exponent:expr, $type:ty, $rescale_fn:ident) => {
        /// Converts angular quantities to underlying numeric types in radian scale.
        ///
        /// Performs de-scaling before erasure, ensuring all angular values are converted to radians.
        /// Non-radian angular quantities are rescaled to radian scale before erasure.
        ///
        /// ## Examples
        /// ```rust
        /// use whippyunits::*;
        ///
        /// // Pure radian quantities erase directly
        /// let radians: f64 = 1.0.radians().into();
        /// assert_eq!(radians, 1.0);
        ///
        /// // Non-radian quantities rescale to radian scale
        /// let degrees: f64 = 90.0.degrees().into();
        /// assert_eq!(degrees, std::f64::consts::PI / 2.0);
        ///
        /// // Enables unit-safe trigonometric functions
        /// let sin_value: f64 = f64::sin(90.0.degrees().into());
        /// assert_eq!(sin_value, 1.0);
        /// ```
        impl<const SCALE_P2: i16, const SCALE_P3: i16, const SCALE_P5: i16, const SCALE_PI: i16>
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
                    other.unsafe_value
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
                    .unsafe_value
                }
            }
        }
    };
}

// radians can be identified as dimensionless (all exponents are 0 except angle, angle scale radians)
// trait resolution rules mean we have to manually template this out over different angle exponents...

#[doc(hidden)]
macro_rules! define_from_for_radians {
    ($exponent:expr, $($type:ty),+ $(,)?) => {
        $(
            /// Erases radian components from compound units, retaining other dimensional components.
            /// Only pure radian powers are erased; residual scales of non-radian units are retained.
            ///
            /// ## Examples
            /// ```rust
            /// use whippyunits::*;
            ///
            /// // Curvature in radians per meter
            /// let curvature = quantity!(1.0, rad / m);
            /// let velocity = quantity!(1.0, m / s);
            ///
            /// // Erase radian component for centripetal acceleration calculation
            /// let centripetal_acceleration: unit!(m / s^2) = (curvature * velocity * velocity).into();
            /// assert_eq!(value!(centripetal_acceleration, m / s^2), 1.0);
            /// ```
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
                        unsafe_value: other.unsafe_value,
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
#[doc(hidden)]
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
