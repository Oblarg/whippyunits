//! Base unit definitions for whippyunits
//!
//! This module contains the canonical base unit information that defines
//! the fundamental units for each dimension.

use crate::DimensionExponents;

#[derive(Debug, Clone)]
pub struct BaseUnitInfo {
    pub symbol: &'static str,
    pub dimension_exponents: DimensionExponents,
    pub prefix_scale_offset: i16,
    pub long_name: &'static str,
}

pub const BASE_UNITS: &[BaseUnitInfo] = &[
    BaseUnitInfo {
        symbol: "g",
        dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0),
        prefix_scale_offset: -3, // kilogram is base unit for mass
        long_name: "gram",
    },
    BaseUnitInfo {
        symbol: "m",
        dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0),
        prefix_scale_offset: 0,
        long_name: "meter",
    },
    BaseUnitInfo {
        symbol: "s",
        dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0),
        prefix_scale_offset: 0,
        long_name: "second",
    },
    BaseUnitInfo {
        symbol: "A",
        dimension_exponents: (0, 0, 0, 1, 0, 0, 0, 0),
        prefix_scale_offset: 0,
        long_name: "ampere",
    },
    BaseUnitInfo {
        symbol: "K",
        dimension_exponents: (0, 0, 0, 0, 1, 0, 0, 0),
        prefix_scale_offset: 0,
        long_name: "kelvin",
    },
    BaseUnitInfo {
        symbol: "mol",
        dimension_exponents: (0, 0, 0, 0, 0, 1, 0, 0),
        prefix_scale_offset: 0,
        long_name: "mole",
    },
    BaseUnitInfo {
        symbol: "cd",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 1, 0),
        prefix_scale_offset: 0,
        long_name: "candela",
    },
    BaseUnitInfo {
        symbol: "rad",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        prefix_scale_offset: 0,
        long_name: "radian",
    },
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
