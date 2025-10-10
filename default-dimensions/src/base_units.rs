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
    pub prefix_scale_offset: i16, // p10_offset
    pub long_name: &'static str,
}

/// Base unit definitions
/// Each base unit maps to its dimension exponents and inherent scale factor
pub const BASE_UNITS: &[BaseUnitInfo] = &[
    // Mass - all mass units are relative to kilogram (scale factor 0)
    BaseUnitInfo {
        symbol: "g",
        dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0),
        prefix_scale_offset: -3,
        long_name: "gram",
    },
    // Length - all length units are relative to meter (scale factor 0)
    BaseUnitInfo {
        symbol: "m",
        dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0),
        prefix_scale_offset: 0,
        long_name: "meter",
    },
    // Time - all time units are relative to second (scale factor 0)
    BaseUnitInfo {
        symbol: "s",
        dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0),
        prefix_scale_offset: 0,
        long_name: "second",
    },
    // Current - all current units are relative to ampere (scale factor 0)
    BaseUnitInfo {
        symbol: "A",
        dimension_exponents: (0, 0, 0, 1, 0, 0, 0, 0),
        prefix_scale_offset: 0,
        long_name: "ampere",
    },
    // Temperature - all temperature units are relative to kelvin (scale factor 0)
    BaseUnitInfo {
        symbol: "K",
        dimension_exponents: (0, 0, 0, 0, 1, 0, 0, 0),
        prefix_scale_offset: 0,
        long_name: "kelvin",
    },
    // Amount - all amount units are relative to mole (scale factor 0)
    BaseUnitInfo {
        symbol: "mol",
        dimension_exponents: (0, 0, 0, 0, 0, 1, 0, 0),
        prefix_scale_offset: 0,
        long_name: "mole",
    },
    // Luminosity - all luminosity units are relative to candela (scale factor 0)
    BaseUnitInfo {
        symbol: "cd",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 1, 0),
        prefix_scale_offset: 0,
        long_name: "candela",
    },
    // Angle - all angle units are relative to radian (scale factor 0)
    BaseUnitInfo {
        symbol: "rad",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        prefix_scale_offset: 0,
        long_name: "radian",
    },
    // Special cases
    BaseUnitInfo {
        symbol: "dimensionless",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 0),
        prefix_scale_offset: 0,
        long_name: "dimensionless",
    },
];

/// Look up base unit information by symbol
pub fn lookup_base_unit(unit_symbol: &str) -> Option<&'static BaseUnitInfo> {
    BASE_UNITS.iter().find(|unit| unit.symbol == unit_symbol)
}
