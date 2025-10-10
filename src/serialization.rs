//! Serialize and deserialize whippyunits quantities.
//! 
//! Whippyunits follows the [UCUM (Unified Code for Units of Measure) standard](https://ucum.org) for 
//! serialization and deserialization to ASCII strings.  Serialization is supported to and from both
//! simple strings (e.g., "10.0 m") and JSON objects (e.g., `{"value": 10.0, "unit": "m"}`).

use crate::api::aggregate_scale_factor_float;
use crate::print::name_lookup::generate_systematic_unit_name_with_format;
use crate::print::prettyprint::UnitFormat;
use crate::quantity_type::Quantity;
use crate::{Scale, Dimension, _2, _3, _5, _Pi, _M, _L, _T, _I, _Θ, _N, _J, _A};
use whippyunits_default_dimensions::{
    get_unit_dimensions, is_prefixed_base_unit, lookup_unit_literal, DimensionExponents, BASE_UNITS,
    SI_PREFIXES,
    // Use centralized parsing functions
    parse_unit_with_prefix, get_prefix_scale_factor,
};

/// Represents the dimension and scale exponents for a unit
/// Vector format: [mass, length, time, current, temperature, amount, luminosity, angle, scale_p2, scale_p3, scale_p5, scale_pi]
pub type UnitDimensions = Vec<i16>;

#[cfg(not(feature = "std"))]
use alloc::{String, Vec};
#[cfg(feature = "std")]
use std::{string::String, vec::Vec};

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
        Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
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
    UnknownDimension(DimensionExponents),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SerializationError {
    DimensionMismatch {
        expected: UnitDimensions,
        actual: UnitDimensions,
    },
    ScaleIncoherence {
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

impl core::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SerializationError::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "Dimension mismatch: expected {:?}, got {:?}",
                    expected, actual
                )
            }
            SerializationError::ScaleIncoherence { expected, actual } => {
                write!(f, "Scale incoherence: expected {:?}, got {:?}. Use non-strict macros for automatic rescaling.", expected, actual)
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

// Trait implementations removed - using pure functions instead

/// Serialize a quantity to JSON using UCUM format
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
        Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
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
        return Ok(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    // Try simple unit first using existing machinery
    if let Some(dims) = get_unit_dimensions(ucum_string) {
        return Ok(vec![
            dims.0, dims.1, dims.2, dims.3, dims.4, dims.5, dims.6, dims.7, 0, 0, 0, 0,
        ]);
    }

    // Parse the UCUM string by splitting on '/' and handling multiplication
    let parts: Vec<&str> = ucum_string.split('/').collect();
    if parts.len() > 2 {
        return Err(UcumError::UnknownDimension((0, 0, 0, 0, 0, 0, 0, 0))); // Invalid format
    }

    let (numerator, denominator) = if parts.len() == 1 {
        (parts[0], "")
    } else {
        (parts[0], parts[1])
    };

    // Parse numerator (multiplication of terms)
    let mut num_dims = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    if !numerator.is_empty() {
        let terms: Vec<&str> = numerator.split('.').collect();
        for term in terms {
            let (dims, scale) = parse_ucum_term(term)?;
            num_dims = (
                num_dims.0 + dims.0,
                num_dims.1 + dims.1,
                num_dims.2 + dims.2,
                num_dims.3 + dims.3,
                num_dims.4 + dims.4,
                num_dims.5 + dims.5,
                num_dims.6 + dims.6,
                num_dims.7 + dims.7,
                num_dims.8 + dims.8,
                num_dims.9 + dims.9,
                num_dims.10 + dims.10,
                num_dims.11 + dims.11,
            );
            num_dims = (
                num_dims.0,
                num_dims.1,
                num_dims.2,
                num_dims.3,
                num_dims.4,
                num_dims.5,
                num_dims.6,
                num_dims.7,
                num_dims.8 + scale.0,
                num_dims.9 + scale.1,
                num_dims.10 + scale.2,
                num_dims.11 + scale.3,
            );
        }
    }

    // Parse denominator (multiplication of terms)
    let mut denom_dims = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    if !denominator.is_empty() {
        let terms: Vec<&str> = denominator.split('.').collect();
        for term in terms {
            let (dims, scale) = parse_ucum_term(term)?;
            denom_dims = (
                denom_dims.0 + dims.0,
                denom_dims.1 + dims.1,
                denom_dims.2 + dims.2,
                denom_dims.3 + dims.3,
                denom_dims.4 + dims.4,
                denom_dims.5 + dims.5,
                denom_dims.6 + dims.6,
                denom_dims.7 + dims.7,
                denom_dims.8 + dims.8,
                denom_dims.9 + dims.9,
                denom_dims.10 + dims.10,
                denom_dims.11 + dims.11,
            );
            denom_dims = (
                denom_dims.0,
                denom_dims.1,
                denom_dims.2,
                denom_dims.3,
                denom_dims.4,
                denom_dims.5,
                denom_dims.6,
                denom_dims.7,
                denom_dims.8 + scale.0,
                denom_dims.9 + scale.1,
                denom_dims.10 + scale.2,
                denom_dims.11 + scale.3,
            );
        }
    }

    // Subtract denominator from numerator
    let result_dims = vec![
        num_dims.0 - denom_dims.0,
        num_dims.1 - denom_dims.1,
        num_dims.2 - denom_dims.2,
        num_dims.3 - denom_dims.3,
        num_dims.4 - denom_dims.4,
        num_dims.5 - denom_dims.5,
        num_dims.6 - denom_dims.6,
        num_dims.7 - denom_dims.7,
        num_dims.8 - denom_dims.8,
        num_dims.9 - denom_dims.9,
        num_dims.10 - denom_dims.10,
        num_dims.11 - denom_dims.11,
    ];
    Ok(result_dims)
}

/// Parse a single UCUM term (e.g., "m^2", "kg", "s^-1", "s2")
fn parse_ucum_term(
    term: &str,
) -> Result<
    (
        (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16),
        (i16, i16, i16, i16),
    ),
    UcumError,
> {
    // Handle exponent notation (e.g., "m^2", "s^-1")
    let (base_unit, exponent) = if let Some(caret_pos) = term.find('^') {
        let base = &term[..caret_pos];
        let exp_str = &term[caret_pos + 1..];
        let exp: i16 = exp_str
            .parse()
            .map_err(|_| UcumError::UnknownDimension((0, 0, 0, 0, 0, 0, 0, 0)))?;
        (base, exp)
    } else {
        // Handle implicit exponent notation (e.g., "s2" -> "s" with exponent 2)
        if let Some(pos) = term.chars().position(|c| c.is_ascii_digit()) {
            let base = &term[..pos];
            let exp_str = &term[pos..];
            if let Ok(exp) = exp_str.parse::<i16>() {
                (base, exp)
            } else {
                (term, 1)
            }
        } else {
            (term, 1)
        }
    };

    // Get dimensions for the base unit using existing machinery
    if let Some(dims) = get_unit_dimensions(base_unit) {
        // Apply exponent
        let result_dims = (
            dims.0 * exponent,
            dims.1 * exponent,
            dims.2 * exponent,
            dims.3 * exponent,
            dims.4 * exponent,
            dims.5 * exponent,
            dims.6 * exponent,
            dims.7 * exponent,
            0,
            0,
            0,
            0, // No scale factors for simple units
        );
        return Ok((result_dims, (0, 0, 0, 0)));
    }

    // Fall back to the complex parsing for special cases
    let (mass, length, time, current, temp, amount, lum, angle, p2, p3, p5, pi) =
        get_unit_dimensions_from_ucum(base_unit)?;

    // Apply exponent
    let dims = (
        mass * exponent,
        length * exponent,
        time * exponent,
        current * exponent,
        temp * exponent,
        amount * exponent,
        lum * exponent,
        angle * exponent,
        p2 * exponent,
        p3 * exponent,
        p5 * exponent,
        pi * exponent,
    );

    Ok((dims, (p2, p3, p5, pi)))
}

/// Get dimensions for a UCUM base unit
fn get_unit_dimensions_from_ucum(
    unit: &str,
) -> Result<(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16), UcumError> {
    // Check if it's a unit literal first
    if let Some((dimension, unit)) = lookup_unit_literal(unit) {
        let (mass, length, time, current, temp, amount, lum, angle) = dimension.exponents;
        let (p2, p3, p5, pi) = unit.scale_factors.unwrap_or((0, 0, 0, 0));
        return Ok((
            mass, length, time, current, temp, amount, lum, angle, p2, p3, p5, pi,
        ));
    }

    // Parse prefix and base unit
    let (prefix, base_unit) = parse_unit_name_ucum(unit);

    // Get base unit dimensions
    let (mass, length, time, current, temp, amount, lum, angle, inherent_p10) =
        get_base_unit_dimensions_ucum(base_unit)?;

    // Get prefix power of 10
    let prefix_p10 = prefix.map(get_prefix_power_ucum).unwrap_or(0);

    // Calculate final scale
    let final_scale = inherent_p10 + prefix_p10;

    // Get special time scale factors for UCUM time units
    let (p2, p3, p5) = match base_unit {
        "s" => (0, 0, 0),
        "min" => (2, 1, 1),
        "h" | "hr" => (4, 2, 2),
        "d" | "yr" => (7, 3, 2),
        _ => (0, 0, 0),
    };

    // Convert p10 to p2 and p5 representation
    let (p2_final, p5_final) = if final_scale >= 0 {
        (p2, p5 + final_scale)
    } else {
        (p2 + final_scale.abs(), p5)
    };

    Ok((
        mass, length, time, current, temp, amount, lum, angle, p2_final, p3, p5_final, 0,
    ))
}

/// Parse unit name to extract prefix and base unit for UCUM
fn parse_unit_name_ucum(unit_name: &str) -> (Option<&str>, &str) {
    // Use the centralized parsing logic from default-dimensions
    parse_unit_with_prefix(unit_name)
}

// Removed is_valid_base_unit_ucum - now using centralized parsing from default-dimensions

/// Get prefix power of 10 for UCUM
/// 
/// This function now uses the centralized parsing logic from default-dimensions.
fn get_prefix_power_ucum(prefix: &str) -> i16 {
    get_prefix_scale_factor(prefix)
}

/// Get base unit dimensions for UCUM parsing
fn get_base_unit_dimensions_ucum(
    base_unit: &str,
) -> Result<(i16, i16, i16, i16, i16, i16, i16, i16, i16), UcumError> {
    if let Some(base_unit_info) = BASE_UNITS.iter().find(|info| info.symbol == base_unit) {
        let (m, l, t, c, temp, a, lum, ang) = base_unit_info.dimension_exponents;
        return Ok((
            m,
            l,
            t,
            c,
            temp,
            a,
            lum,
            ang,
            base_unit_info.prefix_scale_offset,
        ));
    }

    if let Some((dimension, _)) = lookup_unit_literal(base_unit) {
        let (m, l, t, c, temp, a, lum, ang) = dimension.exponents;
        return Ok((m, l, t, c, temp, a, lum, ang, 0));
    }

    Err(UcumError::UnknownDimension((0, 0, 0, 0, 0, 0, 0, 0)))
}

/// Check if two dimension vectors match (ignoring scale factors)
pub fn dimensions_match(a: &UnitDimensions, b: &UnitDimensions) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).take(8).all(|(x, y)| x == y)
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

/// Validate scale coherence and appropriate error if they don't match
pub fn validate_scale_coherence(
    expected: &UnitDimensions,
    actual: &UnitDimensions,
) -> Result<(), SerializationError> {
    if expected != actual {
        return Err(SerializationError::ScaleIncoherence {
            expected: expected.clone(),
            actual: actual.clone(),
        });
    }
    Ok(())
}

/// Calculate conversion factor between two units with matching dimensions
/// Uses the existing scale_conversion logic for consistency
pub fn calculate_conversion_factor(from_dims: &UnitDimensions, to_dims: &UnitDimensions) -> f64 {
    // Use the existing scale conversion logic
    aggregate_scale_factor_float(
        from_dims[8],
        from_dims[9],
        from_dims[10],
        from_dims[11], // from scales
        to_dims[8],
        to_dims[9],
        to_dims[10],
        to_dims[11], // to scales
    )
}

/// Deserializes a quantity from JSON representation.
///
/// Usage: `from_json!(json_string, <unit_literal>)`
///
/// 1. Parses the JSON to extract value and unit
/// 2. Uses deserialize_core_quantity to validate dimensions and rescale if needed
/// 3. Returns a Quantity directly (optimized - no quantity! macro needed)
#[macro_export]
macro_rules! from_json {
    ($json:expr, $unit:expr) => {{
        match $crate::serialization::parse_json_input($json) {
            Ok((value, unit_str)) => {
                // Use deserialize_core_quantity to handle dimension checking and rescaling
                // Returns Quantity directly - no need for quantity! macro
                const dimensions: (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) =
                    whippyunits_proc_macros::compute_unit_dimensions!($unit);
                $crate::serialization::deserialize_core_quantity::<
                    { dimensions.0 },
                    { dimensions.1 },
                    { dimensions.2 },
                    { dimensions.3 },
                    { dimensions.4 },
                    { dimensions.5 },
                    { dimensions.6 },
                    { dimensions.7 },
                    { dimensions.8 },
                    { dimensions.9 },
                    { dimensions.10 },
                    { dimensions.11 },
                    f64,
                >(value, &unit_str)
            }
            Err(e) => Err(e),
        }
    }};
    ($json:expr, $unit:expr, $storage_type:ty) => {{
        match $crate::serialization::parse_json_input($json) {
            Ok((value, unit_str)) => {
                // Use deserialize_core_quantity to handle dimension checking and rescaling
                // Returns Quantity directly - no need for quantity! macro
                const dimensions: (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) =
                    whippyunits_proc_macros::compute_unit_dimensions!($unit);
                $crate::serialization::deserialize_core_quantity::<
                    { dimensions.0 },
                    { dimensions.1 },
                    { dimensions.2 },
                    { dimensions.3 },
                    { dimensions.4 },
                    { dimensions.5 },
                    { dimensions.6 },
                    { dimensions.7 },
                    { dimensions.8 },
                    { dimensions.9 },
                    { dimensions.10 },
                    { dimensions.11 },
                    $storage_type,
                >(value, &unit_str)
            }
            Err(e) => Err(e),
        }
    }};
}

/// Deserializes a quantity from a string representation.
///
/// Parses a string in the format "value unit" (e.g., "5.0 m", "2.5 kg")
/// and returns a `Quantity` with the specified unit type. It performs dimension
/// validation and automatic unit conversion.
///
/// # Syntax
///
/// ```rust
/// from_string!(string_literal, target_unit)
/// ```
///
/// # Examples
///
/// ```rust
/// use whippyunits::from_string;
///
/// // Basic units
/// let length = from_string!("5.0 m", m).unwrap();
/// let mass = from_string!("2.5 kg", kg).unwrap();
/// let time = from_string!("10.0 s", s).unwrap();
///
/// // Unit conversions
/// let km_from_m = from_string!("1000.0 m", km).unwrap();
/// let cm_from_m = from_string!("1.0 m", cm).unwrap();
/// let kg_from_g = from_string!("1000.0 g", kg).unwrap();
///
/// // Compound units
/// let velocity = from_string!("10.0 m/s", m / s).unwrap();
/// let acceleration = from_string!("9.81 m/s2", m / s^2).unwrap();
/// let force = from_string!("100.0 kg.m/s2", kg * m / s^2).unwrap();
/// ```
///
/// # String Format
///
/// The input string must be in the format "value unit" where:
/// - `value`: A numeric value (integer or floating point)
/// - `unit`: A unit symbol or expression (e.g., `m`, `kg`, `m/s`, `kg.m/s2`)
///
/// # Error Handling
///
/// The macro returns a `Result<Quantity, SerializationError>`:
/// - `Ok(quantity)`: Successfully parsed and converted quantity
/// - `Err(SerializationError::DimensionMismatch)`: Unit dimension doesn't match target
/// - `Err(SerializationError::InvalidFormat)`: String format is invalid
/// - `Err(SerializationError::ParseError)`: Numeric value couldn't be parsed
///
/// # Dimension Validation
///
/// The macro ensures that the parsed unit has the same dimensions as the target unit.
/// For example, `from_string!("5.0 m", kg)` will return a dimension mismatch error.
///
/// # Unit Conversion
///
/// If the parsed unit has the same dimensions but different scale than the target unit,
/// automatic conversion is performed. For example, `from_string!("1000.0 m", km)` will
/// return a quantity representing 1.0 km.
#[macro_export]
macro_rules! from_string {
    ($string:expr, $unit:expr) => {{
        match $crate::serialization::parse_string_input($string) {
            Ok((value, unit_str)) => {
                // Use deserialize_core_quantity to handle dimension checking and rescaling
                // Returns Quantity directly - no need for quantity! macro
                const dimensions: (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) =
                    whippyunits_proc_macros::compute_unit_dimensions!($unit);
                $crate::serialization::deserialize_core_quantity::<
                    { dimensions.0 },
                    { dimensions.1 },
                    { dimensions.2 },
                    { dimensions.3 },
                    { dimensions.4 },
                    { dimensions.5 },
                    { dimensions.6 },
                    { dimensions.7 },
                    { dimensions.8 },
                    { dimensions.9 },
                    { dimensions.10 },
                    { dimensions.11 },
                    f64,
                >(value, &unit_str)
            }
            Err(e) => Err(e),
        }
    }};
    ($string:expr, $unit:expr, $storage_type:ty) => {{
        match $crate::serialization::parse_string_input($string) {
            Ok((value, unit_str)) => {
                // Use deserialize_core_quantity to handle dimension checking and rescaling
                // Returns Quantity directly - no need for quantity! macro
                const dimensions: (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) =
                    whippyunits_proc_macros::compute_unit_dimensions!($unit);
                $crate::serialization::deserialize_core_quantity::<
                    { dimensions.0 },
                    { dimensions.1 },
                    { dimensions.2 },
                    { dimensions.3 },
                    { dimensions.4 },
                    { dimensions.5 },
                    { dimensions.6 },
                    { dimensions.7 },
                    { dimensions.8 },
                    { dimensions.9 },
                    { dimensions.10 },
                    { dimensions.11 },
                    $storage_type,
                >(value, &unit_str)
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
pub fn parse_string_input(string: &str) -> Result<(f64, String), SerializationError> {
    let trimmed = string.trim();
    let parts: Vec<&str> = trimmed.split_whitespace().collect();

    if parts.len() < 2 {
        return Err(SerializationError::InvalidFormat(format!(
            "Expected 'value unit', got '{}'",
            trimmed
        )));
    }

    let value: f64 = parts[0].parse().map_err(|e| {
        SerializationError::ParseError(format!("Failed to parse value as f64: {}", e))
    })?;
    let unit_str = parts[1..].join(" "); // Join remaining parts in case unit has spaces

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
    let target_dims = vec![
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
    ];

    // Check if dimensions match
    if !dimensions_match(&parsed_dims, &target_dims) {
        return Err(SerializationError::DimensionMismatch {
            expected: target_dims,
            actual: parsed_dims,
        });
    }

    // Calculate conversion factor if needed
    let conversion_factor = calculate_conversion_factor(&parsed_dims, &target_dims);
    Ok(value * conversion_factor)
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
        Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
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
    let target_dims = vec![
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
    ];

    // Check if dimensions match
    if !dimensions_match(&parsed_dims, &target_dims) {
        return Err(SerializationError::DimensionMismatch {
            expected: target_dims,
            actual: parsed_dims,
        });
    }

    // Calculate conversion factor if needed
    let conversion_factor = calculate_conversion_factor(&parsed_dims, &target_dims);
    let converted_value = value * conversion_factor;

    // Construct Quantity directly using const parameters - no need for quantity! macro
    Ok(Quantity::<Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>, Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>, T>::new(converted_value.into()))
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
        Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>,
        T,
    >,
) -> (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) {
    (
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
    )
}

/// Get target unit dimensions for a unit literal using default_dimensions
pub fn get_target_unit_dimensions(
    unit_literal: &str,
) -> (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) {
    // First try to find in unit literals
    if let Some((dimension, unit)) = lookup_unit_literal(unit_literal) {
        let (mass, length, time, current, temp, amount, lum, angle) = dimension.exponents;
        let (p2, p3, p5, pi) = unit.scale_factors.unwrap_or((0, 0, 0, 0));
        (
            mass, length, time, current, temp, amount, lum, angle, p2, p3, p5, pi,
        )
    } else {
        // Try to parse as a prefixed unit (e.g., "cm", "km", "mm")
        if let Some((base_symbol, prefix)) = is_prefixed_base_unit(unit_literal) {
            // Get the base unit dimensions
            if let Some(base_unit) = BASE_UNITS.iter().find(|unit| unit.symbol == base_symbol) {
                let (mass, length, time, current, temp, amount, lum, angle) =
                    base_unit.dimension_exponents;
                let inherent_scale = base_unit.prefix_scale_offset;

                // Get the prefix scale factor
                let prefix_scale =
                    if let Some(prefix_info) = SI_PREFIXES.iter().find(|p| p.symbol == prefix) {
                        prefix_info.scale_factor
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
                    mass, length, time, current, temp, amount, lum, angle, p2, p3, p5, pi,
                )
            } else {
                panic!("Unknown base unit: {}", base_symbol);
            }
        } else {
            // Try the existing logic from default_dimensions
            if let Some(dimensions) = get_unit_dimensions(unit_literal) {
                let (mass, length, time, current, temp, amount, lum, angle) = dimensions;
                (
                    mass, length, time, current, temp, amount, lum, angle, 0, 0, 0, 0,
                )
            } else {
                panic!("Unknown unit literal: {}", unit_literal);
            }
        }
    }
}

// create_quantity_from_value_and_unit function removed - now using quantity! macro directly

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantity_type::Quantity;

    #[cfg(feature = "std")]
    #[test]
    fn test_json_serialization() {
        let q: Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64> = Quantity::new(5.0);
        let json = serialize_to_json(&q).unwrap();
        assert!(json.contains("\"value\":5.0"));
        assert!(json.contains("\"unit\":\"m\""));
    }

    #[test]
    fn test_parse_ucum_unit_basic() {
        // Test basic units
        let result = parse_ucum_unit("m").unwrap();
        assert_eq!(result, vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let result = parse_ucum_unit("kg").unwrap();
        assert_eq!(result, vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let result = parse_ucum_unit("s").unwrap();
        assert_eq!(result, vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_parse_ucum_unit_dimensionless() {
        let result = parse_ucum_unit("1").unwrap();
        assert_eq!(result, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_parse_ucum_unit_with_exponents() {
        let result = parse_ucum_unit("m^2").unwrap();
        assert_eq!(result, vec![0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let result = parse_ucum_unit("s^-1").unwrap();
        assert_eq!(result, vec![0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_parse_ucum_unit_compound() {
        let result = parse_ucum_unit("kg.m/s^2").unwrap();
        assert_eq!(result, vec![1, 1, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_dimensions_match() {
        let dims1 = vec![1, 1, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let dims2 = vec![1, 1, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let dims3 = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert!(dimensions_match(&dims1, &dims2));
        assert!(!dimensions_match(&dims1, &dims3));
    }

    #[test]
    fn test_calculate_conversion_factor() {
        let from_dims = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let to_dims = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        let factor = calculate_conversion_factor(&from_dims, &to_dims);
        assert!((factor - 1.0).abs() < 1e-10);
    }
}
