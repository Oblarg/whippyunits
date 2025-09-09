//! Dimension type definitions and data for WhippyUnits
//!
//! This module contains the dimension type definitions and dimensional data
//! that defines what physical quantities are supported.

// ============================================================================
// Core Types
// ============================================================================

use std::collections::HashMap;
use std::vec::Vec;

type DimensionalExponents = HashMap<&'static str, i8>;

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantDefinition {
    pub name: &'static str,
    pub float_value: f64,
    pub rational_numerator: i128,
    pub rational_denominator: i128,
}

impl Eq for ConstantDefinition {}

impl std::hash::Hash for ConstantDefinition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScaleFactor {
    Prime(i8, i8), // (prime, exponent)
    Constant(&'static ConstantDefinition, i8), // (constant_definition, exponent)
}

pub type CompositeScaleLevels = HashMap<&'static str, Vec<ScaleFactor>>;

// ============================================================================
// Constants
// ============================================================================

/// Mathematical constants with both float and rational approximations
pub const PI: ConstantDefinition = ConstantDefinition {
    name: "pi",
    float_value: std::f64::consts::PI,
    rational_numerator: 710,
    rational_denominator: 113,
};

/// Get constant definition by name
pub fn get_constant_definition(constant_name: &str) -> Option<&'static ConstantDefinition> {
    match constant_name {
        "pi" => Some(&PI),
        _ => None,
    }
}

/// Get rational approximation for a constant (for backward compatibility)
pub fn get_constant_approximation(constant_name: &str) -> (i128, i128) {
    match get_constant_definition(constant_name) {
        Some(constant) => (constant.rational_numerator, constant.rational_denominator),
        None => (1, 1), // Unknown constant, return 1
    }
}

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
    pub composite_prime_levels: Option<CompositeScaleLevels>,
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
            composite_prime_levels: Some(HashMap::from([
                ("minute", vec![
                    ScaleFactor::Prime(2, 2), 
                    ScaleFactor::Prime(3, 1), 
                    ScaleFactor::Prime(5, 1)
                ]), // 1 minute (60 = 2^2 * 3 * 5)
                ("hour", vec![
                    ScaleFactor::Prime(2, 4), 
                    ScaleFactor::Prime(3, 2), 
                    ScaleFactor::Prime(5, 2)
                ]), // 1 hour (3600 = 2^4 * 3^2 * 5^2)
                ("day", vec![
                    ScaleFactor::Prime(2, 7), 
                    ScaleFactor::Prime(3, 3), 
                    ScaleFactor::Prime(5, 2)
                ]), // 1 day (86,400 = 2^7 * 3^3 * 5^2)
            ])),
        },
        UnitScale {
            name: "current",
            base_name: "ampere",
            base_symbol: "A",
            primes: vec![10],
            base_scale_name_offset: 0,
            composite_prime_levels: None,
        },
        UnitScale {
            name: "temperature",
            base_name: "kelvin",
            base_symbol: "K",
            primes: vec![10],
            base_scale_name_offset: 0,
            composite_prime_levels: None,
        },
        UnitScale {
            name: "amount",
            base_name: "mole",
            base_symbol: "mol",
            primes: vec![10],
            base_scale_name_offset: 0,
            composite_prime_levels: None,
        },
        UnitScale {
            name: "luminosity",
            base_name: "candela",
            base_symbol: "cd",
            primes: vec![10],
            base_scale_name_offset: 0,
            composite_prime_levels: None,
        },
        UnitScale {
            name: "angle",
            base_name: "radian",
            base_symbol: "rad",
            primes: vec![2, 3, 5],
            base_scale_name_offset: 0,
            composite_prime_levels: Some(HashMap::from([
                ("rotation", vec![
                    ScaleFactor::Prime(2, 1), 
                    ScaleFactor::Prime(3, 0), 
                    ScaleFactor::Prime(5, 0), 
                    ScaleFactor::Constant(&PI, 1)
                ]), // 1 rotation = 2π radians
                ("degree", vec![
                    ScaleFactor::Prime(2, -2), 
                    ScaleFactor::Prime(3, -2), 
                    ScaleFactor::Prime(5, -1), 
                    ScaleFactor::Constant(&PI, 1)
                ]), // 1 degree = π/180 radians
                ("gradian", vec![
                    ScaleFactor::Prime(2, -3), 
                    ScaleFactor::Prime(3, 0), 
                    ScaleFactor::Prime(5, -2), 
                    ScaleFactor::Constant(&PI, 1)
                ]), // 1 gradian = π/200 radians
                ("arcminute", vec![
                    ScaleFactor::Prime(2, -4), 
                    ScaleFactor::Prime(3, -3), 
                    ScaleFactor::Prime(5, -2), 
                    ScaleFactor::Constant(&PI, 1)
                ]), // 1 arcminute = π/10800 radians
                ("arcsecond", vec![
                    ScaleFactor::Prime(2, -5), 
                    ScaleFactor::Prime(3, -4), 
                    ScaleFactor::Prime(5, -2), 
                    ScaleFactor::Constant(&PI, 1)
                ]), // 1 arcsecond = π/648000 radians
            ])),
        },
    ]
}


// ============================================================================
// Dimension Data
// ============================================================================

pub fn get_dimensional_data() -> Vec<Dimension> {    
    vec![
        // Atomic dimensions (SI base quantities)
        Dimension {
            names: vec!["Mass"],
            exponents: HashMap::from([("mass", 1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Length", "Distance"],
            exponents: HashMap::from([("length", 1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Time"],
            exponents: HashMap::from([("time", 1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["ElectricCurrent"],
            exponents: HashMap::from([("current", 1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Temperature"],
            exponents: HashMap::from([("temperature", 1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["AmountOfSubstance"],
            exponents: HashMap::from([("amount", 1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["LuminousIntensity"],
            exponents: HashMap::from([("luminosity", 1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Angle"],
            exponents: HashMap::from([("angle", 1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        
        // Length-like dimensions
        Dimension {
            names: vec!["Area"],
            exponents: HashMap::from([("length", 2)]),
            si_shortname: None, // none value means the dimension is purely systematic from the atomic dimensions
            bespoke_units: None,    
        },
        Dimension {
            names: vec!["Volume"],
            exponents: HashMap::from([("length", 3)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec![
                "LinearFrequency",
                "LinearCountDensity",
                "WaveNumber",
                "PerLength",
            ],
            exponents: HashMap::from([("length", -1)]),
            si_shortname: None,
            bespoke_units: None
        },
        Dimension {
            names: vec!["SurfaceFrequency", "SurfaceCountDensity", "PerArea"],
            exponents: HashMap::from([("length", -2)]),
            si_shortname: None,
            bespoke_units: None
        },
        Dimension {
            names: vec!["VolumeFrequency", "VolumeCountDensity", "PerVolume"],
            exponents: HashMap::from([("length", -3)]),
            si_shortname: None,
            bespoke_units: None
        },

        // Time-like dimensions
        Dimension {
            names: vec!["SpecificImpulse"],
            exponents: HashMap::from([("time", 1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Frequency", "PerTime"],
            exponents: HashMap::from([("time", -1)]),
            si_shortname: Some(SiUnit {
                name: "Hertz",
                symbol: "Hz",
            }),
            bespoke_units: None,
        },
        // Velocity-like dimensions
        Dimension {
            names: vec!["Velocity"],
            exponents: HashMap::from([("length", 1), ("time", -1)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Acceleration"],
            exponents: HashMap::from([("length", 1), ("time", -2)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Jerk"],
            exponents: HashMap::from([("length", 1), ("time", -3)]),
            si_shortname: None,
            bespoke_units: None
        },

        // Force-like dimensions
        Dimension {
            names: vec!["Momentum", "Impulse"],
            exponents: HashMap::from([("mass", 1), ("length", 1), ("time", -1)]),
            si_shortname: Some(SiUnit {
                name: "Newton Second",
                symbol: "N⋅s",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Force"],
            exponents: HashMap::from([("mass", 1), ("length", 1), ("time", -2)]),
            si_shortname: Some(SiUnit {
                name: "Newton",
                symbol: "N",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Energy", "Work", "Heat"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -2)]),
            si_shortname: Some(SiUnit {
                name: "Joule",
                symbol: "J",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Power", "HeatTransferRate"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -3)]),
            si_shortname: Some(SiUnit {
                name: "Watt",
                symbol: "W",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Action"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -1)]),
            si_shortname: Some(SiUnit {
                name: "Joule Second",
                symbol: "J⋅s",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Pressure", "Stress"],
            exponents: HashMap::from([("mass", 1), ("length", -1), ("time", -2)]),
            si_shortname: Some(SiUnit {
                name: "Pascal",
                symbol: "Pa",
            }),
            bespoke_units: None,
        },
        // Density-like dimensions
        Dimension {
            names: vec!["LinearMassDensity", "MassPerLength"],
            exponents: HashMap::from([("mass", 1), ("length", -1)]),
            si_shortname: None,
            bespoke_units: None
        },
        Dimension {
            names: vec!["SurfaceMassDensity", "MassPerArea"],
            exponents: HashMap::from([("mass", 1), ("length", -2)]),
            si_shortname: None,
            bespoke_units: None
        },
        Dimension {
            names: vec!["VolumeMassDensity", "MassPerVolume"],
            exponents: HashMap::from([("mass", 1), ("length", -3)]),
            si_shortname: None,
            bespoke_units: None,
        },
        // Viscosity-like dimensions
        Dimension {
            names: vec!["Viscosity", "DynamicViscosity"],
            exponents: HashMap::from([("mass", 1), ("length", -1), ("time", -1)]),
            si_shortname: Some(SiUnit {
                name: "Pascal Second",
                symbol: "Pa⋅s",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["KinematicViscosity"],
            exponents: HashMap::from([("length", 2), ("time", -1)]),
            si_shortname: None,
            bespoke_units: None
        },
        // Surface tension-like dimensions
        Dimension {
            names: vec!["SurfaceTension"],
            exponents: HashMap::from([("mass", 1), ("time", -2)]),
            si_shortname: Some(SiUnit {
                name: "Newton per Meter",
                symbol: "N/m",
            }),
            bespoke_units: None,
        },
        // Specific energy-like dimensions
        Dimension {
            names: vec!["SpecificEnergy"],
            exponents: HashMap::from([("length", 2), ("time", -2)]),
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["SpecificPower"],
            exponents: HashMap::from([("length", 2), ("time", -3)]),
            si_shortname: None,
            bespoke_units: None
        },
        // Flow rate-like dimensions
        Dimension {
            names: vec!["MassFlowRate"],
            exponents: HashMap::from([("mass", 1), ("time", -1)]),
            si_shortname: None,
            bespoke_units: None
        },
        Dimension {
            names: vec!["VolumeFlowRate"],
            exponents: HashMap::from([("length", 3), ("time", -1)]),
            si_shortname: None,
            bespoke_units: None
        },
        // Density-like dimensions (continued)
        Dimension {
            names: vec!["LinearMomentumDensity", "MassFlux"],
            exponents: HashMap::from([("mass", 1), ("length", -2), ("time", -1)]),
            si_shortname: None,
            bespoke_units: None
        },
        Dimension {
            names: vec!["PowerDensity"],
            exponents: HashMap::from([("mass", 1), ("length", -1), ("time", -3)]),
            si_shortname: Some(SiUnit {
                name: "Watt per Meter",
                symbol: "W/m",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["ForceDensity"],
            exponents: HashMap::from([("mass", 1), ("length", -2), ("time", -2)]),
            si_shortname: Some(SiUnit {
                name: "Newton per Square Meter",
                symbol: "N/m²",
            }),
            bespoke_units: None,
        },
        // Heat flux-like dimensions
        Dimension {
            names: vec!["HeatFlux"],
            exponents: HashMap::from([("mass", 1), ("time", -3)]),
            si_shortname: Some(SiUnit {
                name: "Watt per Square Meter",
                symbol: "W/m²",
            }),
            bespoke_units: None,
        },
        
        // Electrical dimensions
        Dimension {
            names: vec!["ElectricCharge"],
            exponents: HashMap::from([("current", 1), ("time", 1)]),
            si_shortname: Some(SiUnit {
                name: "Coulomb",
                symbol: "C",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["ElectricPotential", "Voltage"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -3), ("current", -1)]),
            si_shortname: Some(SiUnit {
                name: "Volt",
                symbol: "V",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["ElectricResistance"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -3), ("current", -2)]),
            si_shortname: Some(SiUnit {
                name: "Ohm",
                symbol: "Ω",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["ElectricConductance"],
            exponents: HashMap::from([("mass", -1), ("length", -2), ("time", 3), ("current", 2)]),
            si_shortname: Some(SiUnit {
                name: "Siemens",
                symbol: "S",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Capacitance"],
            exponents: HashMap::from([("mass", -1), ("length", -2), ("time", 4), ("current", 2)]),
            si_shortname: Some(SiUnit {
                name: "Farad",
                symbol: "F",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Inductance"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -2), ("current", -2)]),
            si_shortname: Some(SiUnit {
                name: "Henry",
                symbol: "H",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["ElectricField"],
            exponents: HashMap::from([("mass", 1), ("length", 1), ("time", -3), ("current", -1)]),
            si_shortname: Some(SiUnit {
                name: "Volt per Meter",
                symbol: "V/m",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MagneticField"],
            exponents: HashMap::from([("mass", 1), ("time", -2), ("current", -1)]),
            si_shortname: Some(SiUnit {
                name: "Tesla",
                symbol: "T",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MagneticFlux"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -2), ("current", -1)]),
            si_shortname: Some(SiUnit {
                name: "Weber",
                symbol: "Wb",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["ElectricDisplacement"],
            exponents: HashMap::from([("length", -2), ("time", 1), ("current", 1)]),
            si_shortname: Some(SiUnit {
                name: "Coulomb per Square Meter",
                symbol: "C/m²",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MagnetizingField"],
            exponents: HashMap::from([("length", -1), ("current", 1)]),
            si_shortname: Some(SiUnit {
                name: "Ampere per Meter",
                symbol: "A/m",
            }),
            bespoke_units: None,
        },
        
        // Thermodynamic dimensions
        Dimension {
            names: vec!["Entropy"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -2), ("temperature", -1)]),
            si_shortname: Some(SiUnit {
                name: "Joule per Kelvin",
                symbol: "J/K",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["HeatCapacity", "ThermalCapacity"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -2), ("temperature", -1)]),
            si_shortname: Some(SiUnit {
                name: "Joule per Kelvin",
                symbol: "J/K",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["SpecificHeatCapacity", "SpecificHeat"],
            exponents: HashMap::from([("length", 2), ("time", -2), ("temperature", -1)]),
            si_shortname: Some(SiUnit {
                name: "Joule per Kilogram Kelvin",
                symbol: "J/(kg⋅K)",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MolarHeatCapacity"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -2), ("temperature", -1), ("amount", -1)]),
            si_shortname: Some(SiUnit {
                name: "Joule per Mole Kelvin",
                symbol: "J/(mol⋅K)",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["ThermalConductivity"],
            exponents: HashMap::from([("mass", 1), ("length", 1), ("time", -3), ("temperature", -1)]),
            si_shortname: Some(SiUnit {
                name: "Watt per Meter Kelvin",
                symbol: "W/(m⋅K)",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["ThermalDiffusivity"],
            exponents: HashMap::from([("length", 2), ("time", -1)]),
            si_shortname: Some(SiUnit {
                name: "Square Meter per Second",
                symbol: "m²/s",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["ThermalResistance"],
            exponents: HashMap::from([("mass", -1), ("length", -2), ("time", 3), ("temperature", 1)]),
            si_shortname: Some(SiUnit {
                name: "Kelvin per Watt",
                symbol: "K/W",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["ThermalExpansion"],
            exponents: HashMap::from([("temperature", -1)]),
            si_shortname: Some(SiUnit {
                name: "Per Kelvin",
                symbol: "K⁻¹",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["ThermalFlux"],
            exponents: HashMap::from([("mass", 1), ("time", -3)]),
            si_shortname: Some(SiUnit {
                name: "Watt per Square Meter",
                symbol: "W/m²",
            }),
            bespoke_units: None,
        },
        
        // Chemical dimensions
        Dimension {
            names: vec!["MolarMass"],
            exponents: HashMap::from([("mass", 1), ("amount", -1)]),
            si_shortname: Some(SiUnit {
                name: "Kilogram per Mole",
                symbol: "kg/mol",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MolarVolume"],
            exponents: HashMap::from([("length", 3), ("amount", -1)]),
            si_shortname: Some(SiUnit {
                name: "Cubic Meter per Mole",
                symbol: "m³/mol",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MolarConcentration", "Molarity"],
            exponents: HashMap::from([("length", -3), ("amount", 1)]),
            si_shortname: Some(SiUnit {
                name: "Mole per Cubic Meter",
                symbol: "mol/m³",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MolalConcentration", "Molality"],
            exponents: HashMap::from([("mass", -1), ("amount", 1)]),
            si_shortname: Some(SiUnit {
                name: "Mole per Kilogram",
                symbol: "mol/kg",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MoleFraction"],
            exponents: HashMap::from([]), // dimensionless
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MassFraction"],
            exponents: HashMap::from([]), // dimensionless
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["VolumeFraction"],
            exponents: HashMap::from([]), // dimensionless
            si_shortname: None,
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MolarFlowRate"],
            exponents: HashMap::from([("amount", 1), ("time", -1)]),
            si_shortname: Some(SiUnit {
                name: "Mole per Second",
                symbol: "mol/s",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MolarFlux"],
            exponents: HashMap::from([("length", -2), ("time", -1), ("amount", 1)]),
            si_shortname: Some(SiUnit {
                name: "Mole per Square Meter Second",
                symbol: "mol/(m²⋅s)",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MolarDensity"],
            exponents: HashMap::from([("length", -3), ("amount", 1)]),
            si_shortname: Some(SiUnit {
                name: "Mole per Cubic Meter",
                symbol: "mol/m³",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["MolarEnergy"],
            exponents: HashMap::from([("mass", 1), ("length", 2), ("time", -2), ("amount", -1)]),
            si_shortname: Some(SiUnit {
                name: "Joule per Mole",
                symbol: "J/mol",
            }),
            bespoke_units: None,
        },
        
        // Photometric dimensions
        Dimension {
            names: vec!["LuminousFlux"],
            exponents: HashMap::from([("luminosity", 1)]),
            si_shortname: Some(SiUnit {
                name: "Lumen",
                symbol: "lm",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Illuminance"],
            exponents: HashMap::from([("length", -2), ("luminosity", 1)]),
            si_shortname: Some(SiUnit {
                name: "Lux",
                symbol: "lx",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["Luminance"],
            exponents: HashMap::from([("length", -2), ("luminosity", 1)]),
            si_shortname: Some(SiUnit {
                name: "Candela per Square Meter",
                symbol: "cd/m²",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["LuminousEnergy"],
            exponents: HashMap::from([("time", 1), ("luminosity", 1)]),
            si_shortname: Some(SiUnit {
                name: "Lumen Second",
                symbol: "lm⋅s",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["LuminousExposure"],
            exponents: HashMap::from([("length", -2), ("time", 1), ("luminosity", 1)]),
            si_shortname: Some(SiUnit {
                name: "Lux Second",
                symbol: "lx⋅s",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["LuminousEfficacy"],
            exponents: HashMap::from([("mass", -1), ("length", -2), ("time", 3), ("luminosity", 1)]),
            si_shortname: Some(SiUnit {
                name: "Lumen per Watt",
                symbol: "lm/W",
            }),
            bespoke_units: None,
        },
        Dimension {
            names: vec!["LuminousEfficiency"],
            exponents: HashMap::from([]), // dimensionless
            si_shortname: None,
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
