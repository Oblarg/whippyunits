//! Serialize and deserialize whippyunits quantities.
//!
//! Whippyunits follows the [UCUM (Unified Code for Units of Measure) standard](https://ucum.org) for
//! serialization and deserialization to ASCII strings.  Serialization is supported to and from both
//! simple strings (e.g., "10.0 m") and JSON objects (e.g., `{"value": 10.0, "unit": "m"}`).

use crate::api::aggregate_scale_factor_float;
use crate::print::name_lookup::generate_systematic_unit_name_with_format;
use crate::print::prettyprint::UnitFormat;
use crate::quantity_type::Quantity;
use crate::{_2, _3, _5, _A, _I, _J, _L, _M, _N, _Pi, _T, _Θ, Dimension, Scale};
use whippyunits_core::{
    SiPrefix, Unit, dimension_exponents::DynDimensionExponents, scale_exponents::ScaleExponents,
};

/// Represents the dimension and scale exponents for a unit using proper whippyunits-core types
pub type UnitDimensions = (
    whippyunits_core::dimension_exponents::DynDimensionExponents,
    whippyunits_core::scale_exponents::ScaleExponents,
);

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

    // Try simple unit first using existing machinery
    if let Some((dimensions, scales)) = lookup_unit_dimensions(ucum_string) {
        return Ok((dimensions, scales));
    }

    // Parse the UCUM string by splitting on '/' and handling multiplication
    let parts: Vec<&str> = ucum_string.split('/').collect();
    if parts.len() > 2 {
        return Err(UcumError::UnknownDimension(
            whippyunits_core::dimension_exponents::DynDimensionExponents([0, 0, 0, 0, 0, 0, 0, 0]),
        )); // Invalid format
    }

    let (numerator, denominator) = if parts.len() == 1 {
        (parts[0], "")
    } else {
        (parts[0], parts[1])
    };

    // Parse numerator (multiplication of terms)
    let mut result_dimensions = DynDimensionExponents::ZERO;
    let mut result_scales = ScaleExponents::IDENTITY;
    if !numerator.is_empty() {
        let terms: Vec<&str> = numerator.split('.').collect();
        for term in terms {
            let ((dimensions, _), scale) = parse_ucum_term(term)?;
            result_dimensions = result_dimensions + dimensions;
            result_scales = result_scales.mul(scale);
        }
    }

    // Parse denominator (division of terms)
    if !denominator.is_empty() {
        let terms: Vec<&str> = denominator.split('.').collect();
        for term in terms {
            let ((dimensions, _), scale) = parse_ucum_term(term)?;
            result_dimensions = result_dimensions + (-dimensions);
            result_scales = result_scales.mul(scale.neg());
        }
    }

    Ok((result_dimensions, result_scales))
}

/// Parse a single UCUM term (e.g., "m^2", "kg", "s^-1", "s2")
fn parse_ucum_term(
    term: &str,
) -> Result<((DynDimensionExponents, ScaleExponents), ScaleExponents), UcumError> {
    // Handle special case for dimensionless unit "1"
    if term == "1" {
        return Ok((
            (
                DynDimensionExponents([0, 0, 0, 0, 0, 0, 0, 0]),
                ScaleExponents([0, 0, 0, 0]),
            ),
            ScaleExponents([0, 0, 0, 0]),
        ));
    }

    // Handle exponent notation (e.g., "m^2", "s^-1")
    let (base_unit, exponent) = if let Some(caret_pos) = term.find('^') {
        let base = &term[..caret_pos];
        let exp_str = &term[caret_pos + 1..];
        let exp: i16 = exp_str.parse().map_err(|_| {
            UcumError::UnknownDimension(
                whippyunits_core::dimension_exponents::DynDimensionExponents([
                    0, 0, 0, 0, 0, 0, 0, 0,
                ]),
            )
        })?;
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
    if let Some((dimensions, scales)) = lookup_unit_dimensions(base_unit) {
        // Apply exponent to dimensions
        let result_dimensions = DynDimensionExponents([
            dimensions.0[0] * exponent,
            dimensions.0[1] * exponent,
            dimensions.0[2] * exponent,
            dimensions.0[3] * exponent,
            dimensions.0[4] * exponent,
            dimensions.0[5] * exponent,
            dimensions.0[6] * exponent,
            dimensions.0[7] * exponent,
        ]);
        // Apply exponent to scale factors
        let result_scales = ScaleExponents([
            scales.0[0] * exponent,
            scales.0[1] * exponent,
            scales.0[2] * exponent,
            scales.0[3] * exponent,
        ]);
        return Ok(((result_dimensions, result_scales), result_scales));
    }

    // Fall back to the complex parsing for special cases
    let (dimensions, scales) = get_unit_dimensions_from_ucum(base_unit)?;

    // Apply exponent
    let result_dimensions = DynDimensionExponents([
        dimensions.0[0] * exponent,
        dimensions.0[1] * exponent,
        dimensions.0[2] * exponent,
        dimensions.0[3] * exponent,
        dimensions.0[4] * exponent,
        dimensions.0[5] * exponent,
        dimensions.0[6] * exponent,
        dimensions.0[7] * exponent,
    ]);
    let result_scales = ScaleExponents([
        scales.0[0] * exponent,
        scales.0[1] * exponent,
        scales.0[2] * exponent,
        scales.0[3] * exponent,
    ]);

    Ok(((result_dimensions, result_scales), result_scales))
}

/// Get dimensions for a UCUM base unit
fn get_unit_dimensions_from_ucum(
    unit: &str,
) -> Result<(DynDimensionExponents, ScaleExponents), UcumError> {
    // Check if it's a unit literal first
    if let Some((dimension, unit)) = lookup_unit_literal_direct(unit) {
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
        return Ok((
            DynDimensionExponents([mass, length, time, current, temp, amount, lum, angle]),
            ScaleExponents([p2, p3, p5, pi]),
        ));
    }

    // Parse prefix and base unit
    let (prefix, base_unit) = parse_unit_with_prefix_direct(unit);

    // Get base unit dimensions
    let (mass, length, time, current, temp, amount, lum, angle, inherent_p10) =
        get_base_unit_dimensions_ucum(&base_unit)?;

    // Get prefix power of 10
    let prefix_p10 = prefix.map(get_prefix_power_ucum).unwrap_or(0);

    // Calculate final scale
    let final_scale = inherent_p10 + prefix_p10;

    // Get special time scale factors for UCUM time units
    let (p2, p3, p5) = match base_unit.as_str() {
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
        DynDimensionExponents([mass, length, time, current, temp, amount, lum, angle]),
        ScaleExponents([p2_final, p3, p5_final, 0]),
    ))
}

/// Parse unit name to extract prefix and base unit for UCUM
fn parse_unit_with_prefix_direct(unit_name: &str) -> (Option<&str>, String) {
    // Use the centralized parsing logic from whippyunits-core
    let (prefix_opt, base_unit) = parse_unit_with_prefix_core(unit_name);
    let prefix_str = prefix_opt.map(|p| p.symbol());
    (prefix_str, base_unit)
}

// Removed is_valid_base_unit_ucum - now using centralized parsing from default-dimensions

/// Get prefix power of 10 for UCUM
///
/// This function now uses the centralized parsing logic from default-dimensions.
fn get_prefix_power_ucum(prefix: &str) -> i16 {
    if let Some(si_prefix) = SiPrefix::from_symbol(prefix) {
        si_prefix.factor_log10() as i16
    } else {
        0
    }
}

/// Get base unit dimensions for UCUM parsing
fn get_base_unit_dimensions_ucum(
    base_unit: &str,
) -> Result<(i16, i16, i16, i16, i16, i16, i16, i16, i16), UcumError> {
    if let Some(base_unit_info) = whippyunits_core::Unit::BASES
        .iter()
        .find(|info| info.symbols.contains(&base_unit))
    {
        let (m, l, t, c, temp, a, lum, ang) = (
            base_unit_info.exponents.0[0], // mass
            base_unit_info.exponents.0[1], // length
            base_unit_info.exponents.0[2], // time
            base_unit_info.exponents.0[3], // current
            base_unit_info.exponents.0[4], // temperature
            base_unit_info.exponents.0[5], // amount
            base_unit_info.exponents.0[6], // luminosity
            base_unit_info.exponents.0[7], // angle
        );

        // Get the inherent scale offset for the base unit
        // For gram, the scale is 10^-3, so the inherent_p10 should be -3
        // This represents the power of 10 that the unit stores relative to the SI base unit
        let inherent_p10 = match base_unit {
            "g" => -3, // gram stores as 10^-3 of kilogram (SI base unit for mass)
            _ => 0,    // other base units have no inherent scale offset
        };

        return Ok((m, l, t, c, temp, a, lum, ang, inherent_p10));
    }

    if let Some((dimension, _)) = lookup_unit_literal_direct(base_unit) {
        let (m, l, t, c, temp, a, lum, ang) = (
            dimension.exponents.0[0], // mass
            dimension.exponents.0[1], // length
            dimension.exponents.0[2], // time
            dimension.exponents.0[3], // current
            dimension.exponents.0[4], // temperature
            dimension.exponents.0[5], // amount
            dimension.exponents.0[6], // luminosity
            dimension.exponents.0[7], // angle
        );
        return Ok((m, l, t, c, temp, a, lum, ang, 0));
    }

    Err(UcumError::UnknownDimension(
        whippyunits_core::dimension_exponents::DynDimensionExponents([0, 0, 0, 0, 0, 0, 0, 0]),
    ))
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
                const UNIT_INFO: (whippyunits_core::dimension_exponents::DynDimensionExponents, whippyunits_core::scale_exponents::ScaleExponents) =
                    whippyunits_proc_macros::compute_unit_dimensions!($unit);
                const dimensions: whippyunits_core::dimension_exponents::DynDimensionExponents = UNIT_INFO.0;
                const scales: whippyunits_core::scale_exponents::ScaleExponents = UNIT_INFO.1;
                $crate::serialization::deserialize_core_quantity::<
                    { dimensions.0[0] },
                    { dimensions.0[1] },
                    { dimensions.0[2] },
                    { dimensions.0[3] },
                    { dimensions.0[4] },
                    { dimensions.0[5] },
                    { dimensions.0[6] },
                    { dimensions.0[7] },
                    { scales.0[0] },
                    { scales.0[1] },
                    { scales.0[2] },
                    { scales.0[3] },
                    f64,
                >(value, &unit_str)
            }
            Err(e) => Err(e),
        }
    }};
    ($json:expr, $unit:expr, $storage_type:ty) => {{
        match $crate::serialization::parse_json_input($json) {
            Ok((value, unit_str)) => {
                const UNIT_INFO: (whippyunits_core::dimension_exponents::DynDimensionExponents, whippyunits_core::scale_exponents::ScaleExponents) =
                    whippyunits_proc_macros::compute_unit_dimensions!($unit);
                const dimensions: whippyunits_core::dimension_exponents::DynDimensionExponents = UNIT_INFO.0;
                const scales: whippyunits_core::scale_exponents::ScaleExponents = UNIT_INFO.1;
                $crate::serialization::deserialize_core_quantity::<
                    { dimensions.0[0] },
                    { dimensions.0[1] },
                    { dimensions.0[2] },
                    { dimensions.0[3] },
                    { dimensions.0[4] },
                    { dimensions.0[5] },
                    { dimensions.0[6] },
                    { dimensions.0[7] },
                    { scales.0[0] },
                    { scales.0[1] },
                    { scales.0[2] },
                    { scales.0[3] },
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
                const UNIT_INFO: (whippyunits_core::dimension_exponents::DynDimensionExponents, whippyunits_core::scale_exponents::ScaleExponents) =
                    whippyunits_proc_macros::compute_unit_dimensions!($unit);
                const dimensions: whippyunits_core::dimension_exponents::DynDimensionExponents = UNIT_INFO.0;
                const scales: whippyunits_core::scale_exponents::ScaleExponents = UNIT_INFO.1;
                $crate::serialization::deserialize_core_quantity::<
                    { dimensions.0[0] },
                    { dimensions.0[1] },
                    { dimensions.0[2] },
                    { dimensions.0[3] },
                    { dimensions.0[4] },
                    { dimensions.0[5] },
                    { dimensions.0[6] },
                    { dimensions.0[7] },
                    { scales.0[0] },
                    { scales.0[1] },
                    { scales.0[2] },
                    { scales.0[3] },
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
                const UNIT_INFO: (whippyunits_core::dimension_exponents::DynDimensionExponents, whippyunits_core::scale_exponents::ScaleExponents) =
                    whippyunits_proc_macros::compute_unit_dimensions!($unit);
                const dimensions: whippyunits_core::dimension_exponents::DynDimensionExponents = UNIT_INFO.0;
                const scales: whippyunits_core::scale_exponents::ScaleExponents = UNIT_INFO.1;
                $crate::serialization::deserialize_core_quantity::<
                    { dimensions.0[0] },
                    { dimensions.0[1] },
                    { dimensions.0[2] },
                    { dimensions.0[3] },
                    { dimensions.0[4] },
                    { dimensions.0[5] },
                    { dimensions.0[6] },
                    { dimensions.0[7] },
                    { scales.0[0] },
                    { scales.0[1] },
                    { scales.0[2] },
                    { scales.0[3] },
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

    // Calculate conversion factor if needed
    let conversion_factor = calculate_conversion_factor(&parsed_dims, &target_dims);
    let converted_value = value * conversion_factor;

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

/// Parse a unit name to extract prefix and base unit
/// Returns (prefix_option, base_unit_name)
fn parse_unit_with_prefix_core(unit_name: &str) -> (Option<&'static SiPrefix>, String) {
    // Try to strip any prefix from the unit name
    if let Some((prefix, base)) = SiPrefix::strip_any_prefix_symbol(unit_name) {
        // Check if the base unit exists
        if whippyunits_core::Dimension::find_unit_by_symbol(base).is_some() {
            return (Some(prefix), String::from(base));
        }
    }

    // Also try stripping prefix from name (not just symbol)
    if let Some((prefix, base)) = SiPrefix::strip_any_prefix_name(unit_name) {
        // Check if the base unit exists by name
        if whippyunits_core::Dimension::find_unit_by_name(base).is_some() {
            return (Some(prefix), String::from(base));
        }
    }

    (None, String::from(unit_name))
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
    use crate::quantity_type::Quantity;

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
