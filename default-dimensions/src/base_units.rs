//! Base unit definitions for whippyunits
//!
//! This module contains the canonical base unit information that defines
//! the fundamental units for each dimension.

use crate::DimensionExponents;

/// Base unit information including dimension exponents and inherent scale factor
#[derive(Debug, Clone)]
pub struct BaseUnitInfo {
    pub symbol: &'static str,
    pub dimension_exponents: DimensionExponents,
    pub inherent_scale_factor: i16, // p10_offset
    pub long_name: &'static str,
    pub conversion_factor: Option<f64>, // Conversion factor to base SI unit (e.g., 0.3048 for feet to meters)
}

/// Base unit definitions
/// Each base unit maps to its dimension exponents and inherent scale factor
pub const BASE_UNITS: &[BaseUnitInfo] = &[
    // Mass - all mass units are relative to kilogram (scale factor 0)
    BaseUnitInfo {
        symbol: "g",
        dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0),
        inherent_scale_factor: -3,
        long_name: "gram",
        conversion_factor: None,
    },
    // Length - all length units are relative to meter (scale factor 0)
    BaseUnitInfo {
        symbol: "m",
        dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0),
        inherent_scale_factor: 0,
        long_name: "meter",
        conversion_factor: None,
    },
    // Time - all time units are relative to second (scale factor 0)
    BaseUnitInfo {
        symbol: "s",
        dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0),
        inherent_scale_factor: 0,
        long_name: "second",
        conversion_factor: None,
    },
    // Current - all current units are relative to ampere (scale factor 0)
    BaseUnitInfo {
        symbol: "A",
        dimension_exponents: (0, 0, 0, 1, 0, 0, 0, 0),
        inherent_scale_factor: 0,
        long_name: "ampere",
        conversion_factor: None,
    },
    // Temperature - all temperature units are relative to kelvin (scale factor 0)
    BaseUnitInfo {
        symbol: "K",
        dimension_exponents: (0, 0, 0, 0, 1, 0, 0, 0),
        inherent_scale_factor: 0,
        long_name: "kelvin",
        conversion_factor: None,
    },
    // Amount - all amount units are relative to mole (scale factor 0)
    BaseUnitInfo {
        symbol: "mol",
        dimension_exponents: (0, 0, 0, 0, 0, 1, 0, 0),
        inherent_scale_factor: 0,
        long_name: "mole",
        conversion_factor: None,
    },
    // Luminosity - all luminosity units are relative to candela (scale factor 0)
    BaseUnitInfo {
        symbol: "cd",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 1, 0),
        inherent_scale_factor: 0,
        long_name: "candela",
        conversion_factor: None,
    },
    // Angle - all angle units are relative to radian (scale factor 0)
    BaseUnitInfo {
        symbol: "rad",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        inherent_scale_factor: 0,
        long_name: "radian",
        conversion_factor: None,
    },
    // Special cases
    BaseUnitInfo {
        symbol: "dimensionless",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 0),
        inherent_scale_factor: 0,
        long_name: "dimensionless",
        conversion_factor: None,
    },
];

/// Look up base unit information by symbol
pub fn lookup_base_unit(unit_symbol: &str) -> Option<&'static BaseUnitInfo> {
    BASE_UNITS.iter().find(|unit| unit.symbol == unit_symbol)
}
