//! Serialize and deserialize [Quantity] instances to and from strings and JSON objects.
//!
//! [`from_json`]
//! [`from_string`]
//!
//! ## Format
//!
//! ```rust,ignore
//! "<value><unit expression>"
//! "<value> <unit expression>"
//! ```
//! ```json
//! {
//!     "value": "<value>",
//!     "unit": "<unit expression>"
//! }
//! ```
//!
//! where:
//!
//! - `<value>` is a numeric value (integer or floating point)
//! - `<unit expression>` is a unit literal expression
//!     - A "unit literal expression" is either:
//!         - An atomic unit (may include prefix):
//!             - `m`, `kg`, `s`, `A`, `K`, `mol`, `cd`, `rad`
//!         - An exponentiation of an atomic unit:
//!             - `m2`, `m^2`
//!         - A multiplication of two or more (possibly exponentiated) atomic units:
//!             - `kg.m2`, `kg * m2`
//!         - A division of two such product expressions:
//!             - `kg.m2/s2`, `kg * m2 / s^2`
//!             - There may be at most one division expression in a unit literal expression
//!             - All terms trailing the division symbol are considered to be in the denominator
//!
//! Runtime parsing of unit literal expressions leverages the same parser as the [unit!](crate::unit!) macro;
//! using `from_string` or `from_json` means your binary will include the `syn` crate as a dependency.  
//! This is technically no-std, but it is fairly heavyweight for a no-std library, and is not appropriate
//! for sufficiently resource-constrained environments.

use crate::api::aggregate_scale_factor_float;
use crate::print::name_lookup::generate_systematic_unit_name_with_format;
use crate::print::prettyprint::UnitFormat;
use crate::quantity::Quantity;
use crate::quantity::{_2, _3, _5, _A, _I, _J, _L, _M, _N, _Pi, _T, _Θ, Dimension, Scale};
use whippyunits_core::{
    SiPrefix, Unit, UnitEvaluationResult, UnitExpr, dimension_exponents::DynDimensionExponents,
    scale_exponents::ScaleExponents,
};

/// Represents the dimension and scale exponents for a unit using proper whippyunits-core types
pub type UnitDimensions = (
    whippyunits_core::dimension_exponents::DynDimensionExponents,
    whippyunits_core::scale_exponents::ScaleExponents,
);

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};
#[cfg(feature = "std")]
use std::{string::String, vec::Vec};

use proc_macro2::TokenStream;
use syn::parse_str;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

/// Convert a whippyunits quantity to UCUM unit string
pub fn to_ucum_unit<
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
>(
    _quantity: &Quantity<
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
    >,
) -> String
where
    T: Into<f64> + Copy,
{
    let exponents = vec![
        MASS_EXPONENT,
        LENGTH_EXPONENT,
        TIME_EXPONENT,
        CURRENT_EXPONENT,
        TEMPERATURE_EXPONENT,
        AMOUNT_EXPONENT,
        LUMINOSITY_EXPONENT,
        ANGLE_EXPONENT,
    ];
    generate_systematic_unit_name_with_format(exponents, false, UnitFormat::Ucum)
}

// Note: FromUcum trait removed because whippyunits is a compile-time only library.
// The dimension exponents are const generic parameters that must be known at compile time.
// Runtime deserialization from UCUM strings is not possible with this architecture.

/// Errors that can occur during UCUM serialization/deserialization
#[derive(Debug, Clone, PartialEq)]
pub enum UcumError {
    /// The dimension exponents don't match any known dimension
    UnknownDimension(whippyunits_core::dimension_exponents::DynDimensionExponents),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SerializationError {
    DimensionMismatch {
        expected: UnitDimensions,
        actual: UnitDimensions,
    },
    InvalidFormat(String),
    ParseError(String),
    UnknownUnit(String),
    UnknownBaseUnit(String),
    UnknownUnitLiteral(String),
}

impl core::fmt::Display for UcumError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            UcumError::UnknownDimension(exponents) => {
                write!(f, "Unknown dimension: {:?}", exponents)
            }
        }
    }
}

/// Convert UnitDimensions to a human-readable unit name
fn format_unit_dimensions(dims: &UnitDimensions) -> String {
    use crate::print::name_lookup::generate_systematic_unit_name_with_format;
    use crate::print::prettyprint::UnitFormat;
    
    let exponents: Vec<i16> = dims.0.0.iter().copied().collect();
    generate_systematic_unit_name_with_format(exponents, false, UnitFormat::Ucum)
}

impl core::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SerializationError::DimensionMismatch { expected, actual } => {
                let expected_unit = format_unit_dimensions(expected);
                let actual_unit = format_unit_dimensions(actual);
                write!(
                    f,
                    "Dimension mismatch: expected {}, got {}",
                    expected_unit, actual_unit
                )
            }
            SerializationError::InvalidFormat(msg) => {
                write!(f, "Invalid format: {}", msg)
            }
            SerializationError::ParseError(msg) => {
                write!(f, "Parse error: {}", msg)
            }
            SerializationError::UnknownUnit(unit) => {
                write!(f, "Unknown unit: {}", unit)
            }
            SerializationError::UnknownBaseUnit(unit) => {
                write!(f, "Unknown base unit: {}", unit)
            }
            SerializationError::UnknownUnitLiteral(unit) => {
                write!(f, "Unknown unit literal: {}", unit)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UcumError {}

#[cfg(feature = "std")]
impl std::error::Error for SerializationError {}

// parse_ucum_unit function removed - not compatible with compile-time only units library

// Serde trait implementations for [Quantity]
//
// The `Serialize` and `Deserialize` traits allow quantities to be serialized/deserialized
// using any serde-compatible serializer (e.g., serde_json, serde_yaml, etc.).
//
// ## Examples
//
// ```rust
// use serde::{Serialize, Deserialize};
// use serde_json;
// use whippyunits::quantity;
// use whippyunits::unit;
//
// // Serialize to JSON string
// let length = quantity!(5.0, m);
// let json: String = serde_json::to_string(&length)?;
// // json = r#"{"value":5.0,"unit":"m"}"#
//
// // Deserialize from JSON string
// let json_str = r#"{"value": 5.0, "unit": "m"}"#;
// let length: unit!(m) = serde_json::from_str(json_str)?;
// ```
//
// Both serialization and deserialization are no-std compatible (require `alloc` feature).

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
    Brand,
> Serialize
    for Quantity<
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
        Brand,
    >
where
    T: Into<f64> + Copy,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let value = self.unsafe_value.into();
        // Create a temporary quantity with Brand = () to pass to to_ucum_unit
        // (Brand is a phantom type and doesn't affect unit calculation)
        let temp_value = self.unsafe_value; // T: Copy from trait bounds
        let quantity_for_unit = Quantity::<
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
            (),
        >::new(temp_value);
        let unit = to_ucum_unit::<
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
        >(&quantity_for_unit);

        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("value", &value)?;
        map.serialize_entry("unit", &unit)?;
        map.end()
    }
}

/// Visitor for deserializing Quantity types
struct QuantityVisitor<
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
    Brand,
> {
    _phantom: core::marker::PhantomData<(
        fn() -> Quantity<
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
            Brand,
        >,
    )>,
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
    Brand,
>
    QuantityVisitor<
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
        Brand,
    >
where
    T: From<f64> + Copy,
{
    fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<
    'de,
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
    Brand,
> Visitor<'de>
    for QuantityVisitor<
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
        Brand,
    >
where
    T: From<f64> + Copy,
{
    type Value = Quantity<
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
        Brand,
    >;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a map with 'value' (number) and 'unit' (string) fields")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error;

        let mut value: Option<f64> = None;
        let mut unit: Option<String> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "value" => {
                    if value.is_some() {
                        return Err(Error::duplicate_field("value"));
                    }
                    value = Some(map.next_value()?);
                }
                "unit" => {
                    if unit.is_some() {
                        return Err(Error::duplicate_field("unit"));
                    }
                    unit = Some(map.next_value()?);
                }
                _ => {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }

        let value = value.ok_or_else(|| Error::missing_field("value"))?;
        let unit_str = unit.ok_or_else(|| Error::missing_field("unit"))?;

        // Use the existing deserialize_core_quantity function, then convert to the correct Brand
        let quantity_no_brand = deserialize_core_quantity::<
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
        >(value, &unit_str)
        .map_err(|e| {
            #[cfg(not(feature = "std"))]
            use alloc::format;
            #[cfg(feature = "std")]
            use std::format;
            Error::custom(format!("{}", e))
        })?;

        // Convert to the correct Brand type
        Ok(Quantity::<
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
            Brand,
        >::new(quantity_no_brand.unsafe_value))
    }
}

impl<
    'de,
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
    Brand,
> Deserialize<'de>
    for Quantity<
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
        Brand,
    >
where
    T: From<f64> + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(QuantityVisitor::<
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
            Brand,
        >::new())
    }
}

/// Serialize a quantity to JSON using UCUM format
///
/// **Note**: This function requires the `std` feature. For no-std compatibility,
/// use the `Serialize` trait implementation instead.
#[cfg(feature = "std")]
pub fn serialize_to_json<
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
>(
    quantity: &Quantity<
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
    >,
) -> Result<String, serde_json::Error>
where
    T: Into<f64> + Copy,
{
    let value = quantity.unsafe_value.into();
    let unit = to_ucum_unit(quantity);
    serde_json::to_string(&serde_json::json!({
        "value": value,
        "unit": unit
    }))
}

// deserialize_from_json removed - not compatible with compile-time only units library

/// Parse a UCUM unit string to extract dimension exponents and scale factors
/// Returns (mass, length, time, current, temperature, amount, luminosity, angle, p2, p3, p5, pi)
pub fn parse_ucum_unit(ucum_string: &str) -> Result<UnitDimensions, UcumError> {
    // Handle dimensionless case
    if ucum_string == "1" {
        return Ok((
            DynDimensionExponents([0, 0, 0, 0, 0, 0, 0, 0]),
            ScaleExponents([0, 0, 0, 0]),
        ));
    }

    // Convert the string to a TokenStream for parsing
    let token_stream: TokenStream = parse_str(ucum_string).map_err(|_| {
        UcumError::UnknownDimension(DynDimensionExponents([0, 0, 0, 0, 0, 0, 0, 0]))
    })?;

    // Parse the TokenStream into a UnitExpr
    let unit_expr: UnitExpr = syn::parse2(token_stream).map_err(|_| {
        UcumError::UnknownDimension(DynDimensionExponents([0, 0, 0, 0, 0, 0, 0, 0]))
    })?;

    // Evaluate the unit expression to get dimensions and scales (use tolerant mode for serialization)
    let result: UnitEvaluationResult = unit_expr.evaluate_with_mode(whippyunits_core::EvaluationMode::Tolerant);

    Ok((result.dimension_exponents, result.scale_exponents))
}

/// Check if two dimension vectors match (comparing both dimensions and scales)
pub fn dimensions_match(a: &UnitDimensions, b: &UnitDimensions) -> bool {
    a.0 == b.0
}

/// Validate dimensions and appropriate error if they don't match
pub fn validate_dimensions(
    expected: &UnitDimensions,
    actual: &UnitDimensions,
) -> Result<(), SerializationError> {
    if !dimensions_match(expected, actual) {
        return Err(SerializationError::DimensionMismatch {
            expected: expected.clone(),
            actual: actual.clone(),
        });
    }
    Ok(())
}

/// Calculate conversion factor and affine offset from a parsed unit string
/// Returns (conversion_factor, affine_offset) for nonstorage units
/// For storage units, returns (1.0, 0.0)
fn calculate_unit_conversion_factors(unit_str: &str) -> Result<(f64, f64), SerializationError> {
    use proc_macro2::TokenStream;
    use syn::parse_str;
    use whippyunits_core::{UnitExpr, calculate_unit_conversion_factors as calc_cf};

    // Handle dimensionless case
    if unit_str == "1" {
        return Ok((1.0, 0.0));
    }

    // Parse the unit string into a UnitExpr
    let token_stream: TokenStream = parse_str(unit_str).map_err(|_| {
        SerializationError::ParseError(format!("Failed to parse unit string: {}", unit_str))
    })?;

    let unit_expr: UnitExpr = syn::parse2(token_stream).map_err(|_| {
        SerializationError::ParseError(format!("Failed to parse unit expression: {}", unit_str))
    })?;

    // Use the shared logic from whippyunits-core
    Ok(calc_cf(&unit_expr))
}

/// Calculate conversion factor between two units with matching dimensions
/// Uses the existing scale_conversion logic for consistency
pub fn calculate_conversion_factor(from_dims: &UnitDimensions, to_dims: &UnitDimensions) -> f64 {
    // Use the existing scale conversion logic
    aggregate_scale_factor_float(
        from_dims.1.0[0],
        from_dims.1.0[1],
        from_dims.1.0[2],
        from_dims.1.0[3], // from scales
        to_dims.1.0[0],
        to_dims.1.0[1],
        to_dims.1.0[2],
        to_dims.1.0[3], // to scales
    )
}

/// Deserializes a [Quantity] from JSON representation.
///
/// Parses a JSON object in the format `{"value": number, "unit": "unit_string"}`
/// (e.g., `{"value": 5.0, "unit": "m"}`, `{"value": 2.5, "unit": "kg"}`)
/// and returns a `Quantity` with the specified unit type. It performs dimension
/// validation and automatic unit conversion.
///
/// # Syntax
///
/// ```rust, ignore
/// from_json!(json_string, target_unit)
/// from_json!(json_string, target_unit, storage_type)
/// ```
///
/// where
///  - `json_string`: A JSON string containing:
///     - `"value"`: A numeric value (integer or floating point)
///     - `"unit"`: A unit literal expression
///        - A "unit literal expression" is either:
///            - An atomic unit (may include prefix):
///                - `m`, `kg`, `s`, `A`, `K`, `mol`, `cd`, `rad`
///            - An exponentiation of an atomic unit:
///                - `m2`, `m^2`
///            - A multiplication of two or more (possibly exponentiated) atomic units:
///                - `kg.m2`, `kg * m2`
///            - A division of two such product expressions:
///                - `kg.m2/s2`, `kg * m2 / s^2`
///                - There may be at most one division expression in a unit literal expression
///                - All terms trailing the division symbol are considered to be in the denominator
///  - `target_unit`: A unit literal expression
///  - `storage_type`: (optional) The storage type for the quantity (defaults to f64)
///
/// # Examples
///
/// ```rust
/// # use whippyunits::from_json;
/// # use whippyunits::value;
/// # use whippyunits::unit;
/// # fn main() {
/// let length: unit!(m) = from_json!(r#"{"value": 5.0, "unit": "m"}"#, m).unwrap();
/// assert_eq!(value!(length, m), 5.0);
/// let length: unit!(km) = from_json!(r#"{"value": 5.0, "unit": "m"}"#, km).unwrap();
/// assert_eq!(value!(length, km), 0.005);
/// let error = from_json!(r#"{"value": 5.0, "unit": "m"}"#, kg);
/// assert!(error.is_err());
/// # }
/// ```
///
/// # Error Handling
///
/// The macro returns a `Result<Quantity, SerializationError>`:
/// - `Ok(quantity)`: Successfully parsed and converted quantity
/// - `Err(SerializationError::DimensionMismatch)`: Unit dimension doesn't match target
/// - `Err(SerializationError::InvalidFormat)`: JSON format is invalid or missing required fields
/// - `Err(SerializationError::ParseError)`: JSON parsing failed or unit string couldn't be parsed
#[macro_export]
macro_rules! from_json {
    ($json:expr, $unit:expr) => {{
        match $crate::serialization::parse_json_input($json) {
            Ok((value, unit_str)) => {
                // Use deserialize_core_quantity to handle dimension checking and rescaling
                // Returns Quantity directly - no need for quantity! macro
                const UNIT_INFO: (
                    whippyunits_core::dimension_exponents::DynDimensionExponents,
                    whippyunits_core::scale_exponents::ScaleExponents,
                ) = whippyunits_proc_macros::compute_unit_dimensions!($unit);
                const DIMENSIONS: whippyunits_core::dimension_exponents::DynDimensionExponents =
                    UNIT_INFO.0;
                const SCALES: whippyunits_core::scale_exponents::ScaleExponents = UNIT_INFO.1;
                $crate::serialization::deserialize_core_quantity::<
                    { DIMENSIONS.0[0] },
                    { DIMENSIONS.0[1] },
                    { DIMENSIONS.0[2] },
                    { DIMENSIONS.0[3] },
                    { DIMENSIONS.0[4] },
                    { DIMENSIONS.0[5] },
                    { DIMENSIONS.0[6] },
                    { DIMENSIONS.0[7] },
                    { SCALES.0[0] },
                    { SCALES.0[1] },
                    { SCALES.0[2] },
                    { SCALES.0[3] },
                    f64,
                >(value, &unit_str)
                    as Result<
                        whippyunits::unit!($unit, f64),
                        $crate::serialization::SerializationError,
                    >
            }
            Err(e) => Err(e),
        }
    }};
    ($json:expr, $unit:expr, $storage_type:ty) => {{
        match $crate::serialization::parse_json_input($json) {
            Ok((value, unit_str)) => {
                const UNIT_INFO: (
                    whippyunits_core::dimension_exponents::DynDimensionExponents,
                    whippyunits_core::scale_exponents::ScaleExponents,
                ) = whippyunits_proc_macros::compute_unit_dimensions!($unit);
                const DIMENSIONS: whippyunits_core::dimension_exponents::DynDimensionExponents =
                    UNIT_INFO.0;
                const SCALES: whippyunits_core::scale_exponents::ScaleExponents = UNIT_INFO.1;
                $crate::serialization::deserialize_core_quantity::<
                    { DIMENSIONS.0[0] },
                    { DIMENSIONS.0[1] },
                    { DIMENSIONS.0[2] },
                    { DIMENSIONS.0[3] },
                    { DIMENSIONS.0[4] },
                    { DIMENSIONS.0[5] },
                    { DIMENSIONS.0[6] },
                    { DIMENSIONS.0[7] },
                    { SCALES.0[0] },
                    { SCALES.0[1] },
                    { SCALES.0[2] },
                    { SCALES.0[3] },
                    $storage_type,
                >(value, &unit_str)
                    as Result<
                        whippyunits::unit!($unit, $storage_type),
                        $crate::serialization::SerializationError,
                    >
            }
            Err(e) => Err(e),
        }
    }};
}

/// Deserializes a [Quantity] from a string representation.
///
/// Parses a string in the format "value unit" or "valueunit" (e.g., "5.0 m", "5.0m", "2.5 kg", "2.5kg")
/// and returns a `Quantity` with the specified unit type. It performs dimension
/// validation and automatic unit conversion. The space between value and unit is optional.
///
/// # Syntax
///
/// ```rust, ignore
/// from_string!(string_literal, target_unit)
/// from_string!(string_literal, target_unit, storage_type)
/// ```
///
/// where
///  - `string_literal`: A string literal containing:
///     - A numeric value (integer or floating point)
///     - A unit literal expression
///        - A "unit literal expression" is either:
///            - An atomic unit (may include prefix):
///                - `m`, `kg`, `s`, `A`, `K`, `mol`, `cd`, `rad`
///            - An exponentiation of an atomic unit:
///                - `m2`, `m^2`
///            - A multiplication of two or more (possibly exponentiated) atomic units:
///                - `kg.m2`, `kg * m2`
///            - A division of two such product expressions:
///                - `kg.m2/s2`, `kg * m2 / s^2`
///                - There may be at most one division expression in a unit literal expression
///                - All terms trailing the division symbol are considered to be in the denominator
///  - `target_unit`: A unit literal expression
///  - `storage_type`: (optional) The storage type for the quantity (defaults to f64)
///
/// ## Examples
///
/// ```rust
/// # use whippyunits::from_string;
/// # use whippyunits::value;
/// # use whippyunits::unit;
/// # fn main() {
/// let length: unit!(m) = from_string!("5.0 m", m).unwrap();
/// assert_eq!(value!(length, m), 5.0);
/// let length: unit!(m) = from_string!("5.0m", m).unwrap(); // Space is optional
/// assert_eq!(value!(length, m), 5.0);
/// let length: unit!(km) = from_string!("5.0 m", km).unwrap();
/// assert_eq!(value!(length, km), 0.005);
/// let acceleration: unit!(m/s2) = from_string!("9.81 m/s2", m/s2).unwrap();
/// assert_eq!(value!(acceleration, m/s2), 9.81);
/// let acceleration: unit!(m/s2) = from_string!("9.81m/s2", m/s2).unwrap(); // Space is optional
/// assert_eq!(value!(acceleration, m/s2), 9.81);
/// let error = from_string!("5.0 m/s2", m/s);
/// assert!(error.is_err());
/// # }
/// ```
///
/// # Error Handling
///
/// The macro returns a `Result<Quantity, SerializationError>`:
/// - `Ok(quantity)`: Successfully parsed and converted quantity
/// - `Err(SerializationError::DimensionMismatch)`: Unit dimension doesn't match target
/// - `Err(SerializationError::InvalidFormat)`: String format is invalid
/// - `Err(SerializationError::ParseError)`: Numeric value couldn't be parsed
#[macro_export]
macro_rules! from_string {
    ($string:expr, $unit:expr) => {{
        match $crate::serialization::parse_string_input($string) {
            Ok((value, unit_str)) => {
                // Use deserialize_core_quantity to handle dimension checking and rescaling
                // Returns Quantity directly - no need for quantity! macro
                const UNIT_INFO: (
                    whippyunits_core::dimension_exponents::DynDimensionExponents,
                    whippyunits_core::scale_exponents::ScaleExponents,
                ) = whippyunits_proc_macros::compute_unit_dimensions!($unit);
                const DIMENSIONS: whippyunits_core::dimension_exponents::DynDimensionExponents =
                    UNIT_INFO.0;
                const SCALES: whippyunits_core::scale_exponents::ScaleExponents = UNIT_INFO.1;
                $crate::serialization::deserialize_core_quantity::<
                    { DIMENSIONS.0[0] },
                    { DIMENSIONS.0[1] },
                    { DIMENSIONS.0[2] },
                    { DIMENSIONS.0[3] },
                    { DIMENSIONS.0[4] },
                    { DIMENSIONS.0[5] },
                    { DIMENSIONS.0[6] },
                    { DIMENSIONS.0[7] },
                    { SCALES.0[0] },
                    { SCALES.0[1] },
                    { SCALES.0[2] },
                    { SCALES.0[3] },
                    f64,
                >(value, &unit_str)
                    as Result<
                        whippyunits::unit!($unit, f64),
                        $crate::serialization::SerializationError,
                    >
            }
            Err(e) => Err(e),
        }
    }};
    ($string:expr, $unit:expr, $storage_type:ty) => {{
        match $crate::serialization::parse_string_input($string) {
            Ok((value, unit_str)) => {
                // Use deserialize_core_quantity to handle dimension checking and rescaling
                // Returns Quantity directly - no need for quantity! macro
                const UNIT_INFO: (
                    whippyunits_core::dimension_exponents::DynDimensionExponents,
                    whippyunits_core::scale_exponents::ScaleExponents,
                ) = whippyunits_proc_macros::compute_unit_dimensions!($unit);
                const DIMENSIONS: whippyunits_core::dimension_exponents::DynDimensionExponents =
                    UNIT_INFO.0;
                const SCALES: whippyunits_core::scale_exponents::ScaleExponents = UNIT_INFO.1;
                $crate::serialization::deserialize_core_quantity::<
                    { DIMENSIONS.0[0] },
                    { DIMENSIONS.0[1] },
                    { DIMENSIONS.0[2] },
                    { DIMENSIONS.0[3] },
                    { DIMENSIONS.0[4] },
                    { DIMENSIONS.0[5] },
                    { DIMENSIONS.0[6] },
                    { DIMENSIONS.0[7] },
                    { SCALES.0[0] },
                    { SCALES.0[1] },
                    { SCALES.0[2] },
                    { SCALES.0[3] },
                    $storage_type,
                >(value, &unit_str)
                    as Result<
                        whippyunits::unit!($unit, $storage_type),
                        $crate::serialization::SerializationError,
                    >
            }
            Err(e) => Err(e),
        }
    }};
}

/// Parse JSON to extract value and unit string
pub fn parse_json_input(json: &str) -> Result<(f64, String), SerializationError> {
    use serde_json;

    let json_value: serde_json::Value = serde_json::from_str(json)
        .map_err(|e| SerializationError::ParseError(format!("Invalid JSON format: {}", e)))?;

    let value: f64 = json_value["value"].as_f64().ok_or_else(|| {
        SerializationError::InvalidFormat("Missing or invalid 'value' field".to_string())
    })?;
    let unit_str: String = json_value["unit"]
        .as_str()
        .ok_or_else(|| {
            SerializationError::InvalidFormat("Missing or invalid 'unit' field".to_string())
        })?
        .to_string();

    Ok((value, unit_str))
}

/// Parse string to extract value and unit string
/// Supports both formats: "5.0 m" (with space) and "5.0m" (without space)
pub fn parse_string_input(string: &str) -> Result<(f64, String), SerializationError> {
    let trimmed = string.trim();

    // First, try parsing with whitespace separation (for backward compatibility)
    let parts: Vec<&str> = trimmed.split_whitespace().collect();

    if parts.len() >= 2 {
        // Space-separated format: "5.0 m"
        let value: f64 = parts[0].parse().map_err(|e| {
            SerializationError::ParseError(format!("Failed to parse value as f64: {}", e))
        })?;
        let unit_str = parts[1..].join(" "); // Join remaining parts in case unit has spaces
        return Ok((value, unit_str));
    }

    // No space found - try to parse by finding the boundary between number and unit
    // Find the longest valid numeric prefix by trying all possible prefixes
    let chars: Vec<char> = trimmed.chars().collect();
    let mut end_of_number = 0;

    // Try all possible prefixes and find the longest one that parses as a valid f64
    for i in 1..=chars.len() {
        let candidate: String = chars[..i].iter().collect();
        if candidate.parse::<f64>().is_ok() {
            end_of_number = i;
        }
        // Continue trying longer prefixes even if this one fails
        // (e.g., "5.0e" fails but "5.0e-3" succeeds)
    }

    if end_of_number == 0 || end_of_number >= chars.len() {
        return Err(SerializationError::InvalidFormat(format!(
            "Could not parse value and unit from '{}'",
            trimmed
        )));
    }

    let value_str: String = chars[..end_of_number].iter().collect();
    let unit_str: String = chars[end_of_number..].iter().collect();

    let value: f64 = value_str.parse().map_err(|e| {
        SerializationError::ParseError(format!("Failed to parse value as f64: {}", e))
    })?;

    if unit_str.trim().is_empty() {
        return Err(SerializationError::InvalidFormat(format!(
            "Missing unit in '{}'",
            trimmed
        )));
    }

    Ok((value, unit_str))
}

/// Core deserialization logic that handles parsing and validation
pub fn deserialize_core<
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
>(
    value: f64,
    unit_str: &str,
) -> Result<f64, SerializationError> {
    // Parse UCUM unit to get dimensions
    let parsed_dims = parse_ucum_unit(unit_str)
        .map_err(|e| SerializationError::ParseError(format!("Failed to parse UCUM unit: {}", e)))?;

    // Get target dimensions from const generics
    let target_dims = (
        DynDimensionExponents([
            MASS_EXPONENT,
            LENGTH_EXPONENT,
            TIME_EXPONENT,
            CURRENT_EXPONENT,
            TEMPERATURE_EXPONENT,
            AMOUNT_EXPONENT,
            LUMINOSITY_EXPONENT,
            ANGLE_EXPONENT,
        ]),
        ScaleExponents([SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI]),
    );

    // Check if dimensions match
    if !dimensions_match(&parsed_dims, &target_dims) {
        return Err(SerializationError::DimensionMismatch {
            expected: target_dims,
            actual: parsed_dims,
        });
    }

    // Calculate nonstorage unit conversion factors (if any)
    let (unit_cf, unit_af) = calculate_unit_conversion_factors(unit_str)?;
    
    // Apply nonstorage unit conversion factor and affine offset
    let value_with_unit_conversion = (value * unit_cf) + unit_af;

    // Calculate scale factor conversion (for storage unit scaling)
    let scale_conversion_factor = calculate_conversion_factor(&parsed_dims, &target_dims);
    Ok(value_with_unit_conversion * scale_conversion_factor)
}

/// Core deserialization logic that returns a Quantity directly (optimized version)
/// This single function handles both f64 and custom storage types
pub fn deserialize_core_quantity<
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
>(
    value: f64,
    unit_str: &str,
) -> Result<
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
    >,
    SerializationError,
>
where
    T: From<f64> + Copy,
{
    // Parse UCUM unit to get dimensions
    let parsed_dims = parse_ucum_unit(unit_str)
        .map_err(|e| SerializationError::ParseError(format!("Failed to parse UCUM unit: {}", e)))?;

    // Get target dimensions from const generics
    let target_dims = (
        DynDimensionExponents([
            MASS_EXPONENT,
            LENGTH_EXPONENT,
            TIME_EXPONENT,
            CURRENT_EXPONENT,
            TEMPERATURE_EXPONENT,
            AMOUNT_EXPONENT,
            LUMINOSITY_EXPONENT,
            ANGLE_EXPONENT,
        ]),
        ScaleExponents([SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI]),
    );

    // Check if dimensions match
    if !dimensions_match(&parsed_dims, &target_dims) {
        return Err(SerializationError::DimensionMismatch {
            expected: target_dims,
            actual: parsed_dims,
        });
    }

    // Calculate nonstorage unit conversion factors (if any)
    let (unit_cf, unit_af) = calculate_unit_conversion_factors(unit_str)?;
    
    // Apply nonstorage unit conversion factor and affine offset
    let value_with_unit_conversion = (value * unit_cf) + unit_af;

    // Calculate scale factor conversion (for storage unit scaling)
    let scale_conversion_factor = calculate_conversion_factor(&parsed_dims, &target_dims);
    let converted_value = value_with_unit_conversion * scale_conversion_factor;

    // Construct Quantity directly using const parameters - no need for quantity! macro
    Ok(Quantity::<
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
    >::new(converted_value.into()))
}

/// Get dimensions from a quantity by creating a temporary quantity and extracting its dimensions
pub fn get_quantity_dimensions<
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
>(
    _quantity: &Quantity<
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
    >,
) -> UnitDimensions {
    (
        DynDimensionExponents([
            MASS_EXPONENT,
            LENGTH_EXPONENT,
            TIME_EXPONENT,
            CURRENT_EXPONENT,
            TEMPERATURE_EXPONENT,
            AMOUNT_EXPONENT,
            LUMINOSITY_EXPONENT,
            ANGLE_EXPONENT,
        ]),
        ScaleExponents([SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI]),
    )
}

/// Get target unit dimensions for a unit literal using proper core types
pub fn get_target_unit_dimensions(unit_literal: &str) -> UnitDimensions {
    // First try to find in unit literals
    if let Some((dimension, unit)) = lookup_unit_literal_direct(unit_literal) {
        let (mass, length, time, current, temp, amount, lum, angle) = (
            dimension.exponents.0[0], // mass
            dimension.exponents.0[1], // length
            dimension.exponents.0[2], // time
            dimension.exponents.0[3], // current
            dimension.exponents.0[4], // temperature
            dimension.exponents.0[5], // amount
            dimension.exponents.0[6], // luminosity
            dimension.exponents.0[7], // angle
        );
        let (p2, p3, p5, pi) = (
            unit.scale.0[0],
            unit.scale.0[1],
            unit.scale.0[2],
            unit.scale.0[3],
        );
        (
            DynDimensionExponents([mass, length, time, current, temp, amount, lum, angle]),
            ScaleExponents([p2, p3, p5, pi]),
        )
    } else {
        // Try to parse as a prefixed unit (e.g., "cm", "km", "mm")
        if let Some((base_symbol, prefix)) = is_prefixed_base_unit_direct(unit_literal) {
            // Get the base unit dimensions
            if let Some(base_unit) = whippyunits_core::Unit::BASES
                .iter()
                .find(|unit| unit.symbols.contains(&base_symbol.as_str()))
            {
                let (mass, length, time, current, temp, amount, lum, angle) = (
                    base_unit.exponents.0[0], // mass
                    base_unit.exponents.0[1], // length
                    base_unit.exponents.0[2], // time
                    base_unit.exponents.0[3], // current
                    base_unit.exponents.0[4], // temperature
                    base_unit.exponents.0[5], // amount
                    base_unit.exponents.0[6], // luminosity
                    base_unit.exponents.0[7], // angle
                );
                let inherent_scale = 0; // No inherent scale offset in the new system

                // Get the prefix scale factor
                let prefix_scale = if let Some(prefix_info) = whippyunits_core::SiPrefix::ALL
                    .iter()
                    .find(|p| p.symbol() == prefix)
                {
                    prefix_info.factor_log10()
                } else {
                    0
                };

                // Calculate the total scale factor
                let total_scale = inherent_scale + prefix_scale;

                // Convert to p2, p3, p5, pi format
                // The scale factors represent powers of 10, so we need to factor them properly
                let (p2, p3, p5, pi) = if total_scale == 0 {
                    (0, 0, 0, 0)
                } else {
                    // For SI prefixes, the scale is a power of 10
                    // Factor 10^n into 2^n * 5^n since 10 = 2 * 5
                    (total_scale, 0, total_scale, 0)
                };

                (
                    DynDimensionExponents([mass, length, time, current, temp, amount, lum, angle]),
                    ScaleExponents([p2, p3, p5, pi]),
                )
            } else {
                panic!("Unknown base unit: {}", base_symbol);
            }
        } else {
            // Try the existing logic from whippyunits-core
            if let Some((dimensions, scales)) = lookup_unit_dimensions(unit_literal) {
                (dimensions, scales)
            } else {
                panic!("Unknown unit literal: {}", unit_literal);
            }
        }
    }
}

// create_quantity_from_value_and_unit function removed - now using quantity! macro directly

// Helper functions that replace api_helpers functions with direct whippyunits-core calls

/// Look up a unit literal (like "min", "h", "g", "m", "s", etc.) in the dimensions data
fn lookup_unit_literal_direct(
    unit_name: &str,
) -> Option<(&'static whippyunits_core::Dimension, &'static Unit)> {
    // First try to find by symbol
    if let Some((unit, dimension)) = whippyunits_core::Dimension::find_unit_by_symbol(unit_name) {
        return Some((dimension, unit));
    }

    // Then try to find by name
    if let Some((unit, dimension)) = whippyunits_core::Dimension::find_unit_by_name(unit_name) {
        return Some((dimension, unit));
    }

    None
}

/// Check if a unit name is a prefixed base unit (like kg, kW, mm, etc.)
/// Returns Some((base_unit, prefix)) if it is, None otherwise
fn is_prefixed_base_unit_direct(unit_name: &str) -> Option<(String, String)> {
    // Try to strip any prefix from the unit name
    if let Some((prefix, base)) = SiPrefix::strip_any_prefix_symbol(unit_name) {
        // Check if the base unit exists
        if whippyunits_core::Dimension::find_unit_by_symbol(base).is_some() {
            return Some((String::from(base), String::from(prefix.symbol())));
        }
    }

    // Also try stripping prefix from name (not just symbol)
    if let Some((prefix, base)) = SiPrefix::strip_any_prefix_name(unit_name) {
        // Check if the base unit exists by name
        if whippyunits_core::Dimension::find_unit_by_name(base).is_some() {
            return Some((String::from(base), String::from(prefix.symbol())));
        }
    }

    None
}

/// Get unit dimensions for a unit literal using proper whippyunits-core types
fn lookup_unit_dimensions(unit_literal: &str) -> Option<UnitDimensions> {
    // This is a simplified version that just returns the base dimensions
    // The full implementation would need to handle prefixes and conversion factors
    if let Some((_dimension, unit)) = lookup_unit_literal_direct(unit_literal) {
        Some((unit.exponents, unit.scale))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantity::Quantity;

    #[cfg(feature = "std")]
    #[test]
    fn test_json_serialization() {
        let q: Quantity<
            Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>,
            Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>,
            f64,
        > = Quantity::new(5.0);
        let json = serialize_to_json(&q).unwrap();
        assert!(json.contains("\"value\":5.0"));
        assert!(json.contains("\"unit\":\"m\""));
    }

    #[test]
    fn test_parse_ucum_unit_basic() {
        // Test basic units
        let result = parse_ucum_unit("m").unwrap();
        assert_eq!(
            result,
            (
                DynDimensionExponents([0, 1, 0, 0, 0, 0, 0, 0]),
                ScaleExponents([0, 0, 0, 0])
            )
        );

        let result = parse_ucum_unit("kg").unwrap();
        assert_eq!(
            result,
            (
                DynDimensionExponents([1, 0, 0, 0, 0, 0, 0, 0]),
                ScaleExponents([0, 0, 0, 0])
            )
        );

        let result = parse_ucum_unit("s").unwrap();
        assert_eq!(
            result,
            (
                DynDimensionExponents([0, 0, 1, 0, 0, 0, 0, 0]),
                ScaleExponents([0, 0, 0, 0])
            )
        );
    }

    #[test]
    fn test_parse_ucum_unit_dimensionless() {
        let result = parse_ucum_unit("1").unwrap();
        assert_eq!(
            result,
            (
                DynDimensionExponents([0, 0, 0, 0, 0, 0, 0, 0]),
                ScaleExponents([0, 0, 0, 0])
            )
        );
    }

    #[test]
    fn test_parse_ucum_unit_with_exponents() {
        let result = parse_ucum_unit("m^2").unwrap();
        assert_eq!(
            result,
            (
                DynDimensionExponents([0, 2, 0, 0, 0, 0, 0, 0]),
                ScaleExponents([0, 0, 0, 0])
            )
        );

        let result = parse_ucum_unit("s^-1").unwrap();
        assert_eq!(
            result,
            (
                DynDimensionExponents([0, 0, -1, 0, 0, 0, 0, 0]),
                ScaleExponents([0, 0, 0, 0])
            )
        );
    }

    #[test]
    fn test_parse_ucum_unit_implicit_exponents() {
        // Test implicit exponent notation (UCUM standard)
        let result = parse_ucum_unit("m2").unwrap();
        assert_eq!(
            result,
            (
                DynDimensionExponents([0, 2, 0, 0, 0, 0, 0, 0]),
                ScaleExponents([0, 0, 0, 0])
            )
        );

        let result = parse_ucum_unit("1/s").unwrap();
        assert_eq!(
            result,
            (
                DynDimensionExponents([0, 0, -1, 0, 0, 0, 0, 0]),
                ScaleExponents([0, 0, 0, 0])
            )
        );

        let result = parse_ucum_unit("kg.m/s2").unwrap();
        assert_eq!(
            result,
            (
                DynDimensionExponents([1, 1, -2, 0, 0, 0, 0, 0]),
                ScaleExponents([0, 0, 0, 0])
            )
        );
    }

    #[test]
    fn test_parse_ucum_unit_compound() {
        let result = parse_ucum_unit("kg.m/s^2").unwrap();
        assert_eq!(
            result,
            (
                DynDimensionExponents([1, 1, -2, 0, 0, 0, 0, 0]),
                ScaleExponents([0, 0, 0, 0])
            )
        );
    }

    #[test]
    fn test_dimensions_match() {
        let dims1 = (
            DynDimensionExponents([1, 1, -2, 0, 0, 0, 0, 0]),
            ScaleExponents([0, 0, 0, 0]),
        );
        let dims2 = (
            DynDimensionExponents([1, 1, -2, 0, 0, 0, 0, 0]),
            ScaleExponents([0, 0, 0, 0]),
        );
        let dims3 = (
            DynDimensionExponents([1, 0, 0, 0, 0, 0, 0, 0]),
            ScaleExponents([0, 0, 0, 0]),
        );

        assert!(dimensions_match(&dims1, &dims2));
        assert!(!dimensions_match(&dims1, &dims3));
    }

    #[test]
    fn test_calculate_conversion_factor() {
        let from_dims = (
            DynDimensionExponents([0, 0, 0, 0, 0, 0, 0, 0]),
            ScaleExponents([0, 0, 0, 0]),
        );
        let to_dims = (
            DynDimensionExponents([0, 0, 0, 0, 0, 0, 0, 0]),
            ScaleExponents([0, 0, 0, 0]),
        );

        let factor = calculate_conversion_factor(&from_dims, &to_dims);
        assert!((factor - 1.0).abs() < 1e-10);
    }
}
