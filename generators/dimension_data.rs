//! Dimension type definitions and data for WhippyUnits
//!
//! This module contains the dimension type definitions and dimensional data
//! that defines what physical quantities are supported.

// ============================================================================
// Core Types
// ============================================================================

use crate::unit_data;
use std::collections::HashMap;
use std::vec::Vec;


type DimensionalExponents = HashMap<&'static str, i8>;

#[derive(Debug, Clone, PartialEq)]
pub struct SiUnit {
    pub name: &'static str,
    pub symbol: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub name: &'static str,
    pub symbol: &'static str,
    pub scales: DimensionalExponents,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dimension {
    pub names: Vec<&'static str>,
    pub exponents: DimensionalExponents,
    pub si_shortname: Option<SiUnit>,
    pub bespoke_units: Option<Vec<Unit>>,
}

// ============================================================================
// Scale Data
// ============================================================================


pub struct UnitScale {
    pub name: &'static str,
    pub base_name: &'static str,
    pub base_symbol: &'static str,
    pub base_scale_name_offset: i8,
    pub primes: Vec<i8>,
    pub composite_prime_levels: Option<Vec<HashMap<i8, i8>>>,
}

pub fn get_unit_scales() -> Vec<UnitScale> {
    vec![
        UnitScale {
            name: "mass",
            // "base name" in a linguistic sense; see: base_scale_name_offset
            base_name: "gram",
            base_symbol: "g",
            primes: vec![10],
            // alas, kilograms... (10^3)
            base_scale_name_offset: 3,
            composite_prime_levels: None,
        },
        UnitScale {
            name: "length",
            base_name: "meter",
            base_symbol: "m",
            primes: vec![10],
            base_scale_name_offset: 0,
            composite_prime_levels: None,
        },
        UnitScale {
            name: "time",
            base_name: "second",
            base_symbol: "s",
            primes: vec![2, 3, 5],
            base_scale_name_offset: 0,
            composite_prime_levels: Some(vec![
                HashMap::from([(2, 2), (3, 1), (5, 1)]), // 1 minute (60 = 2 * 2 * 3 * 5)
                HashMap::from([(2, 4), (3, 2), (5, 2)]), // 1 hour
                HashMap::from([(2, 0), (3, 0), (5, 1)]), // 1 day
            ]),
        }
    ]
}


// ============================================================================
// Dimension Data
// ============================================================================

pub fn get_dimensional_data() -> Vec<Dimension> {    
    vec![
        // Length-like dimensions
        Dimension {
            names: vec!["Length", "Distance"],
            exponents: HashMap::from([("length", 1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: &["Area"],
            exponents: HashMap::from([("length", 2)]),
            si_shortname: None, // none value means the dimension is purely systematic from the atomic dimensions
            bespoke_units: None,    
        },
        Dimension {
            names: &["Volume"],
            exponents: HashMap::from([("mass", 0), ("length", 3), ("time", 0)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: &[
                "LinearFrequency",
                "LinearCountDensity",
                "WaveNumber",
                "PerLength",
            ],
            exponents: HashMap::from([("mass", 0), ("length", -1), ("time", 0)]),
            si_shortname: None,
            bespoke_units: None
        },
        Dimension {
            names: &["SurfaceFrequency", "SurfaceCountDensity", "PerArea"],
            exponents: HashMap::from([("mass", 0), ("length", -2), ("time", 0)]),
            si_shortname: None,
            bespoke_units: None
        },
        Dimension {
            names: &["VolumeFrequency", "VolumeCountDensity", "PerVolume"],
            exponents: HashMap::from([("mass", 0), ("length", -3), ("time", 0)]),
            si_shortname: None,
            bespoke_units: None
        },
        // Mass-like dimensions
        Dimension {
            names: &["Mass"],
            exponents: HashMap::from([("mass", 1), ("length", 0), ("time", 0)]),
            si_shortname: None,
            bespoke_units: None,
        },
        // Time-like dimensions
        Dimension {
            names: &["Time", "SpecificImpulse"],
            exponents: HashMap::from([("mass", 0), ("length", 0), ("time", 1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: &["Frequency", "PerTime"],
            exponents: HashMap::from([("mass", 0), ("length", 0), ("time", -1)]),
            si_shortname: Some(SiUnit {
                name: "Hertz",
                symbol: "Hz",
            }),
            bespoke_units: None,
        },

        // Velocity-like dimensions
        Dimension {
            names: &["Velocity"],
            exponents: HashMap::from([("mass", 0), ("length", 1), ("time", -1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: &["Acceleration"],
            exponents: HashMap::from([("mass", 0), ("length", 1), ("time", -2)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: &["Jerk"],
            exponents: HashMap::from([("mass", 0), ("length", 1), ("time", -3)]),
            si_shortname: None,
            bespoke_units: None
        },

        // Force-like dimensions
        Dimension {
            names: &["Momentum", "Impulse"],
            exponents: HashMap::from([("mass", 1), ("length", 1), ("time", -1)]),
            si_shortname: Some(SiUnit {
                name: "Newton Second",
                symbol: "N⋅s",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: &["Force"],
            exponents: HashMap::from([("mass", 1), ("length", 1), ("time", -2)]),
            si_shortname: Some(SiUnit {
                name: "Newton",
                symbol: "N",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: &["Energy", "Work", "Heat"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -2)]),
            si_shortname: Some(SiUnit {
                name: "Joule",
                symbol: "J",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: &["Power", "HeatTransferRate"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -3)]),
            si_shortname: Some(SiUnit {
                name: "Watt",
                symbol: "W",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: &["Action"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -1)]),
            si_shortname: Some(SiUnit {
                name: "Joule Second",
                symbol: "J⋅s",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: &["Pressure", "Stress"],
            exponents: HashMap::from([("mass", 1), ("length", -1), ("time", -2)]),
            si_shortname: Some(SiUnit {
                name: "Pascal",
                symbol: "Pa",
            }),
            bespoke_units: None,
        },
        // Density-like dimensions
        Dimension {
            names: &["LinearMassDensity", "MassPerLength"],
            exponents: HashMap::from([("mass", 1), ("length", -1), ("time", 0)]),
            si_shortname: None,
            bespoke_units: None
        },
        Dimension {
            names: &["SurfaceMassDensity", "MassPerArea"],
            exponents: HashMap::from([("mass", 1), ("length", -2), ("time", 0)]),
            si_shortname: None,
            bespoke_units: None
        },
        Dimension {
            names: &["VolumeMassDensity", "MassPerVolume"],
            exponents: HashMap::from([("mass", 1), ("length", -3), ("time", 0)]),
            si_shortname: None,
            bespoke_units: None,
        },
        // Viscosity-like dimensions
        Dimension {
            names: &["Viscosity", "DynamicViscosity"],
            exponents: HashMap::from([("mass", 1), ("length", -1), ("time", -1)]),
            si_shortname: Some(SiUnit {
                name: "Pascal Second",
                symbol: "Pa⋅s",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: &["KinematicViscosity"],
            exponents: HashMap::from([("mass", 0), ("length", 2), ("time", -1)]),
            si_shortname: None,
            bespoke_units: None
        },
        // Surface tension-like dimensions
        Dimension {
            names: &["SurfaceTension"],
            exponents: HashMap::from([("mass", 1), ("length", 0), ("time", -2)]),
            si_shortname: Some(SiUnit {
                name: "Newton per Meter",
                symbol: "N/m",
            }),
            bespoke_units: None,
        },
        // Specific energy-like dimensions
        Dimension {
            names: &["SpecificEnergy"],
            exponents: HashMap::from([("mass", 0), ("length", 2), ("time", -2)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: &["SpecificPower"],
            exponents: HashMap::from([("mass", 0), ("length", 2), ("time", -3)]),
            si_shortname: None,
            bespoke_units: None
        },
        // Flow rate-like dimensions
        Dimension {
            names: &["MassFlowRate"],
            exponents: HashMap::from([("mass", 1), ("length", 0), ("time", -1)]),
            si_shortname: None,
            bespoke_units: None
        },
        Dimension {
            names: &["VolumeFlowRate"],
            exponents: HashMap::from([("mass", 0), ("length", 3), ("time", -1)]),
            si_shortname: None,
            bespoke_units: None
        },
        // Density-like dimensions (continued)
        Dimension {
            names: &["LinearMomentumDensity", "MassFlux"],
            exponents: HashMap::from([("mass", 1), ("length", -2), ("time", -1)]),
            si_shortname: None,
            bespoke_units: None
        },
        Dimension {
            names: &["PowerDensity"],
            exponents: HashMap::from([("mass", 1), ("length", -1), ("time", -3)]),
            si_shortname: Some(SiUnit {
                name: "Watt per Meter",
                symbol: "W/m",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: &["ForceDensity"],
            exponents: HashMap::from([("mass", 1), ("length", -2), ("time", -2)]),
            si_shortname: Some(SiUnit {
                name: "Newton per Square Meter",
                symbol: "N/m²",
            }),
            bespoke_units: None,
        },
        // Heat flux-like dimensions
        Dimension {
            names: &["HeatFlux"],
            exponents: HashMap::from([("mass", 1), ("length", 0), ("time", -3)]),
            si_shortname: Some(SiUnit {
                name: "Watt per Square Meter",
                symbol: "W/m²",
            }),
            bespoke_units: None,
        },
    ]
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Get canonical name for a dimension
pub fn get_canonical_name(dimension: &Dimension) -> &str {
    dimension.names.first().unwrap()
}

/// Find dimension by name
pub fn find_dimension_by_name(name: &str) -> Option<Dimension> {
    get_dimensional_data()
        .into_iter()
        .find(|d| d.names.contains(&name))
}

/// Get dimensions by exponents
pub fn get_dimensions_by_exponents(exponents: &DimensionalExponents) -> Vec<Dimension> {
    get_dimensional_data()
        .into_iter()
        .filter(|d| d.exponents == *exponents)
        .collect()
}
