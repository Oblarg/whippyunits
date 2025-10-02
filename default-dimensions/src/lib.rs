//! Default dimension data for whippyunits
//!
//! This crate provides canonical dimension data that can be shared between
//! the main whippyunits library and the proc macro crate without circular dependencies.

/// Dimension exponents tuple: (mass, length, time, current, temperature, amount, luminosity, angle)
pub type DimensionExponents = (i16, i16, i16, i16, i16, i16, i16, i16);

/// Dimension information including name, symbol, and optional SI unit symbols
#[derive(Debug, Clone)]
pub struct DimensionInfo {
    pub exponents: DimensionExponents,
    pub name: &'static str,
    pub symbol: Option<&'static str>,
    pub si_symbol: Option<&'static str>,
    pub si_long_name: Option<&'static str>,
}

/// SI prefix information
#[derive(Debug, Clone)]
pub struct PrefixInfo {
    pub symbol: &'static str,
    pub scale_factor: i16,
    pub long_name: &'static str,
}

/// Base unit information including dimension exponents and inherent scale factor
#[derive(Debug, Clone)]
pub struct BaseUnitInfo {
    pub symbol: &'static str,
    pub dimension_exponents: DimensionExponents,
    pub inherent_scale_factor: i16, // p10_offset
    pub long_name: &'static str,
    pub conversion_factor: Option<f64>, // Conversion factor to base SI unit (e.g., 0.3048 for feet to meters)
}

/// Unit literal information for formatting and conversion
#[derive(Debug, Clone)]
pub struct UnitLiteralInfo {
    pub symbol: &'static str,
    pub long_name: &'static str,
    pub type_name: &'static str, // The actual type name (e.g., "Kilometer", "Meter")
    pub dimension_exponents: DimensionExponents,
    pub scale_factors: (i16, i16, i16, i16), // p2, p3, p5, pi
    pub conversion_factor: Option<f64>, // Conversion factor to base SI unit (None for SI base units)
}

/// Canonical lookup table for all supported dimensions
///
/// This is the single source of truth for dimension data, shared between
/// the prettyprint logic and the proc macro DSL.
pub const DIMENSION_LOOKUP: &[DimensionInfo] = &[
    // Atomic dimensions (SI base quantities)
    DimensionInfo {
        exponents: (1, 0, 0, 0, 0, 0, 0, 0),
        name: "Mass",
        symbol: Some("M"),
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 1, 0, 0, 0, 0, 0, 0),
        name: "Length",
        symbol: Some("L"),
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, 1, 0, 0, 0, 0, 0),
        name: "Time",
        symbol: Some("T"),
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, 0, 1, 0, 0, 0, 0),
        name: "Current",
        symbol: Some("I"),
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, 0, 0, 1, 0, 0, 0),
        name: "Temperature",
        symbol: Some("θ"),
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, 0, 0, 0, 1, 0, 0),
        name: "Amount",
        symbol: Some("N"),
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, 0, 0, 0, 0, 1, 0),
        name: "Luminosity",
        symbol: Some("Cd"),
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        name: "Angle",
        symbol: Some("A"),
        si_symbol: None,
        si_long_name: None,
    },
    // Length-like dimensions
    DimensionInfo {
        exponents: (0, 2, 0, 0, 0, 0, 0, 0),
        name: "Area",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 3, 0, 0, 0, 0, 0, 0),
        name: "Volume",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, -1, 0, 0, 0, 0, 0, 0),
        name: "Wave Number",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    // Time-like dimensions
    DimensionInfo {
        exponents: (0, 0, -1, 0, 0, 0, 0, 0),
        name: "Frequency",
        symbol: None,
        si_symbol: Some("Hz"),
        si_long_name: Some("Hertz"),
    },
    // Velocity-like dimensions
    DimensionInfo {
        exponents: (0, 1, -1, 0, 0, 0, 0, 0),
        name: "Velocity",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 1, -2, 0, 0, 0, 0, 0),
        name: "Acceleration",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 1, -3, 0, 0, 0, 0, 0),
        name: "Jerk",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    // Force-like dimensions
    DimensionInfo {
        exponents: (1, 1, -1, 0, 0, 0, 0, 0),
        name: "Momentum",
        symbol: None,
        si_symbol: Some("N⋅s"),
        si_long_name: Some("Newton Second"),
    },
    DimensionInfo {
        exponents: (1, 1, -2, 0, 0, 0, 0, 0),
        name: "Force",
        symbol: None,
        si_symbol: Some("N"),
        si_long_name: Some("Newton"),
    },
    DimensionInfo {
        exponents: (1, 2, -2, 0, 0, 0, 0, 0),
        name: "Energy",
        symbol: None,
        si_symbol: Some("J"),
        si_long_name: Some("Joule"),
    },
    DimensionInfo {
        exponents: (1, 2, -3, 0, 0, 0, 0, 0),
        name: "Power",
        symbol: None,
        si_symbol: Some("W"),
        si_long_name: Some("Watt"),
    },
    DimensionInfo {
        exponents: (1, 2, -1, 0, 0, 0, 0, 0),
        name: "Action",
        symbol: None,
        si_symbol: Some("J⋅s"),
        si_long_name: Some("Joule Second"),
    },
    DimensionInfo {
        exponents: (1, -1, -2, 0, 0, 0, 0, 0),
        name: "Pressure",
        symbol: None,
        si_symbol: Some("Pa"),
        si_long_name: Some("Pascal"),
    },
    // Density-like dimensions
    DimensionInfo {
        exponents: (1, -1, 0, 0, 0, 0, 0, 0),
        name: "Linear Mass Density",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (1, -2, 0, 0, 0, 0, 0, 0),
        name: "Surface Mass Density",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (1, -3, 0, 0, 0, 0, 0, 0),
        name: "Volume Mass Density",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    // Viscosity-like dimensions
    DimensionInfo {
        exponents: (1, -1, -1, 0, 0, 0, 0, 0),
        name: "Viscosity",
        symbol: None,
        si_symbol: Some("Pa⋅s"),
        si_long_name: Some("Pascal Second"),
    },
    DimensionInfo {
        exponents: (0, 2, -1, 0, 0, 0, 0, 0),
        name: "Kinematic Viscosity",
        symbol: None,
        si_symbol: Some("St"),
        si_long_name: Some("Stokes"),
    },
    // Surface tension-like dimensions
    DimensionInfo {
        exponents: (1, 0, -2, 0, 0, 0, 0, 0),
        name: "Surface Tension",
        symbol: None,
        si_symbol: Some("N/m"),
        si_long_name: Some("Newton per Meter"),
    },
    // Specific energy-like dimensions
    DimensionInfo {
        exponents: (0, 2, -2, 0, 0, 0, 0, 0),
        name: "Specific Energy",
        symbol: None,
        si_symbol: Some("J/kg"),
        si_long_name: Some("Joule per Kilogram"),
    },
    DimensionInfo {
        exponents: (0, 2, -3, 0, 0, 0, 0, 0),
        name: "Specific Power",
        symbol: None,
        si_symbol: Some("W/kg"),
        si_long_name: Some("Watt per Kilogram"),
    },
    // Flow rate-like dimensions
    DimensionInfo {
        exponents: (1, 0, -1, 0, 0, 0, 0, 0),
        name: "Mass Flow Rate",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 3, -1, 0, 0, 0, 0, 0),
        name: "Volume Flow Rate",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (1, -1, -3, 0, 0, 0, 0, 0),
        name: "Power Density",
        symbol: None,
        si_symbol: Some("W/m"),
        si_long_name: Some("Watt per Meter"),
    },
    DimensionInfo {
        exponents: (1, -2, -2, 0, 0, 0, 0, 0),
        name: "Force Density",
        symbol: None,
        si_symbol: Some("N/m²"),
        si_long_name: Some("Newton per Square Meter"),
    },
    DimensionInfo {
        exponents: (1, 0, -3, 0, 0, 0, 0, 0),
        name: "Heat Flux",
        symbol: None,
        si_symbol: Some("W/m²"),
        si_long_name: Some("Watt per Square Meter"),
    },
    // Electrical dimensions
    DimensionInfo {
        exponents: (0, 0, 1, 1, 0, 0, 0, 0),
        name: "Electric Charge",
        symbol: None,
        si_symbol: Some("C"),
        si_long_name: Some("Coulomb"),
    },
    DimensionInfo {
        exponents: (1, 2, -3, -1, 0, 0, 0, 0),
        name: "Electric Potential",
        symbol: None,
        si_symbol: Some("V"),
        si_long_name: Some("Volt"),
    },
    DimensionInfo {
        exponents: (1, 2, -3, -2, 0, 0, 0, 0),
        name: "Electric Resistance",
        symbol: None,
        si_symbol: Some("Ω"),
        si_long_name: Some("Ohm"),
    },
    DimensionInfo {
        exponents: (-1, -2, 3, 2, 0, 0, 0, 0),
        name: "Electric Conductance",
        symbol: None,
        si_symbol: Some("S"),
        si_long_name: Some("Siemens"),
    },
    DimensionInfo {
        exponents: (-1, -2, 4, 2, 0, 0, 0, 0),
        name: "Capacitance",
        symbol: None,
        si_symbol: Some("F"),
        si_long_name: Some("Farad"),
    },
    DimensionInfo {
        exponents: (1, 2, -2, -2, 0, 0, 0, 0),
        name: "Inductance",
        symbol: None,
        si_symbol: Some("H"),
        si_long_name: Some("Henry"),
    },
    DimensionInfo {
        exponents: (1, 1, -3, -1, 0, 0, 0, 0),
        name: "Electric Field",
        symbol: None,
        si_symbol: Some("V/m"),
        si_long_name: Some("Volt per Meter"),
    },
    DimensionInfo {
        exponents: (1, 0, -2, -1, 0, 0, 0, 0),
        name: "Magnetic Field",
        symbol: None,
        si_symbol: Some("T"),
        si_long_name: Some("Tesla"),
    },
    DimensionInfo {
        exponents: (1, 2, -2, -1, 0, 0, 0, 0),
        name: "Magnetic Flux",
        symbol: None,
        si_symbol: Some("Wb"),
        si_long_name: Some("Weber"),
    },
    DimensionInfo {
        exponents: (0, -1, 1, 1, 0, 0, 0, 0),
        name: "Linear Charge Density",
        symbol: None,
        si_symbol: Some("C/m"),
        si_long_name: Some("Coulomb per Meter"),
    },
    DimensionInfo {
        exponents: (0, -2, 1, 1, 0, 0, 0, 0),
        name: "Surface Charge Density",
        symbol: None,
        si_symbol: Some("C/m²"),
        si_long_name: Some("Coulomb per Square Meter"),
    },
    DimensionInfo {
        exponents: (0, -3, 1, 1, 0, 0, 0, 0),
        name: "Volume Charge Density",
        symbol: None,
        si_symbol: Some("C/m³"),
        si_long_name: Some("Coulomb per Cubic Meter"),
    },
    DimensionInfo {
        exponents: (0, -1, 0, 1, 0, 0, 0, 0),
        name: "Magnetizing Field",
        symbol: None,
        si_symbol: Some("A/m"),
        si_long_name: Some("Ampere per Meter"),
    },
    // Thermodynamic dimensions
    DimensionInfo {
        exponents: (1, 2, -2, 0, -1, 0, 0, 0),
        name: "Entropy",
        symbol: None,
        si_symbol: Some("J/K"),
        si_long_name: Some("Joule per Kelvin"),
    },
    DimensionInfo {
        exponents: (0, 2, -2, 0, -1, 0, 0, 0),
        name: "Specific Heat Capacity",
        symbol: None,
        si_symbol: Some("J/(kg⋅K)"),
        si_long_name: Some("Joule per Kilogram Kelvin"),
    },
    DimensionInfo {
        exponents: (1, 2, -2, 0, -1, -1, 0, 0),
        name: "Molar Heat Capacity",
        symbol: None,
        si_symbol: Some("J/(mol⋅K)"),
        si_long_name: Some("Joule per Mole Kelvin"),
    },
    DimensionInfo {
        exponents: (1, 1, -3, 0, -1, 0, 0, 0),
        name: "Thermal Conductivity",
        symbol: None,
        si_symbol: Some("W/(m⋅K)"),
        si_long_name: Some("Watt per Meter Kelvin"),
    },
    DimensionInfo {
        exponents: (-1, -2, 3, 0, 1, 0, 0, 0),
        name: "Thermal Resistance",
        symbol: None,
        si_symbol: Some("K/W"),
        si_long_name: Some("Kelvin per Watt"),
    },
    DimensionInfo {
        exponents: (0, 0, 0, 0, -1, 0, 0, 0),
        name: "Thermal Expansion",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    // Chemical dimensions
    DimensionInfo {
        exponents: (1, 0, 0, 0, 0, -1, 0, 0),
        name: "Molar Mass",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 3, 0, 0, 0, -1, 0, 0),
        name: "Molar Volume",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, -3, 0, 0, 0, 1, 0, 0),
        name: "Molar Concentration",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (-1, 0, 0, 0, 0, 1, 0, 0),
        name: "Molal Concentration",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, -1, 0, 0, 1, 0, 0),
        name: "Molar Flow Rate",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, -2, -1, 0, 0, 1, 0, 0),
        name: "Molar Flux",
        symbol: None,
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (1, 2, -2, 0, 0, -1, 0, 0),
        name: "Molar Energy",
        symbol: None,
        si_symbol: Some("J/mol"),
        si_long_name: Some("Joule per Mole"),
    },
    // Photometric dimensions
    DimensionInfo {
        exponents: (0, 0, 0, 0, 0, 0, 1, 0),
        name: "Luminous Flux",
        symbol: None,
        si_symbol: Some("lm"),
        si_long_name: Some("Lumen"),
    },
    DimensionInfo {
        exponents: (0, -2, 0, 0, 0, 0, 1, 0),
        name: "Illuminance",
        symbol: None,
        si_symbol: Some("lx"),
        si_long_name: Some("Lux"),
    },
    DimensionInfo {
        exponents: (0, -2, 1, 0, 0, 0, 1, 0),
        name: "Luminous Exposure",
        symbol: None,
        si_symbol: Some("lx⋅s"),
        si_long_name: Some("Lux Second"),
    },
    DimensionInfo {
        exponents: (-1, -2, 3, 0, 0, 0, 1, 0),
        name: "Luminous Efficacy",
        symbol: None,
        si_symbol: Some("lm/W"),
        si_long_name: Some("Lumen per Watt"),
    },
];

/// SI prefix definitions
/// Each prefix maps to its actual scale factor used in the generated unit methods
/// Complete official SI prefix spectrum as defined by BIPM/CGPM
pub const SI_PREFIXES: &[PrefixInfo] = &[
    // Small prefixes (negative powers of 10) - submultiples
    PrefixInfo {
        symbol: "q",
        scale_factor: -30,
        long_name: "quecto",
    }, // 10⁻³⁰ (new 2022)
    PrefixInfo {
        symbol: "r",
        scale_factor: -27,
        long_name: "ronto",
    }, // 10⁻²⁷ (new 2022)
    PrefixInfo {
        symbol: "y",
        scale_factor: -24,
        long_name: "yocto",
    }, // 10⁻²⁴
    PrefixInfo {
        symbol: "z",
        scale_factor: -21,
        long_name: "zepto",
    }, // 10⁻²¹
    PrefixInfo {
        symbol: "a",
        scale_factor: -18,
        long_name: "atto",
    }, // 10⁻¹⁸
    PrefixInfo {
        symbol: "f",
        scale_factor: -15,
        long_name: "femto",
    }, // 10⁻¹⁵
    PrefixInfo {
        symbol: "p",
        scale_factor: -12,
        long_name: "pico",
    }, // 10⁻¹²
    PrefixInfo {
        symbol: "n",
        scale_factor: -9,
        long_name: "nano",
    }, // 10⁻⁹
    PrefixInfo {
        symbol: "u",
        scale_factor: -6,
        long_name: "micro",
    }, // 10⁻⁶
    PrefixInfo {
        symbol: "m",
        scale_factor: -3,
        long_name: "milli",
    }, // 10⁻³
    PrefixInfo {
        symbol: "c",
        scale_factor: -2,
        long_name: "centi",
    }, // 10⁻²
    PrefixInfo {
        symbol: "d",
        scale_factor: -1,
        long_name: "deci",
    }, // 10⁻¹
    // Large prefixes (positive powers of 10) - multiples
    PrefixInfo {
        symbol: "da",
        scale_factor: 1,
        long_name: "deka",
    }, // 10¹
    PrefixInfo {
        symbol: "h",
        scale_factor: 2,
        long_name: "hecto",
    }, // 10²
    PrefixInfo {
        symbol: "k",
        scale_factor: 3,
        long_name: "kilo",
    }, // 10³
    PrefixInfo {
        symbol: "M",
        scale_factor: 6,
        long_name: "mega",
    }, // 10⁶
    PrefixInfo {
        symbol: "G",
        scale_factor: 9,
        long_name: "giga",
    }, // 10⁹
    PrefixInfo {
        symbol: "T",
        scale_factor: 12,
        long_name: "tera",
    }, // 10¹²
    PrefixInfo {
        symbol: "P",
        scale_factor: 15,
        long_name: "peta",
    }, // 10¹⁵
    PrefixInfo {
        symbol: "E",
        scale_factor: 18,
        long_name: "exa",
    }, // 10¹⁸
    PrefixInfo {
        symbol: "Z",
        scale_factor: 21,
        long_name: "zetta",
    }, // 10²¹
    PrefixInfo {
        symbol: "Y",
        scale_factor: 24,
        long_name: "yotta",
    }, // 10²⁴
    PrefixInfo {
        symbol: "R",
        scale_factor: 27,
        long_name: "ronna",
    }, // 10²⁷ (new 2022)
    PrefixInfo {
        symbol: "Q",
        scale_factor: 30,
        long_name: "quetta",
    }, // 10³⁰ (new 2022)
];

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

/// Comprehensive unit literal lookup table
/// This includes all the units defined in default_declarators and imperial_declarators
pub const UNIT_LITERALS: &[UnitLiteralInfo] = &[
    // Base SI units only - prefixing handled systematically by proc macro
    UnitLiteralInfo {
        symbol: "m",
        long_name: "meter",
        type_name: "Meter",
        dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0),
        scale_factors: (0, 0, 0, 0),
        conversion_factor: None,
    },
    UnitLiteralInfo {
        symbol: "g",
        long_name: "gram",
        type_name: "Gram",
        dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0),
        scale_factors: (-3, 0, -3, 0),
        conversion_factor: None,
    },
    UnitLiteralInfo {
        symbol: "s",
        long_name: "second",
        type_name: "Second",
        dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0),
        scale_factors: (0, 0, 0, 0),
        conversion_factor: None,
    },
    UnitLiteralInfo {
        symbol: "A",
        long_name: "ampere",
        type_name: "Ampere",
        dimension_exponents: (0, 0, 0, 1, 0, 0, 0, 0),
        scale_factors: (0, 0, 0, 0),
        conversion_factor: None,
    },
    UnitLiteralInfo {
        symbol: "K",
        long_name: "kelvin",
        type_name: "Kelvin",
        dimension_exponents: (0, 0, 0, 0, 1, 0, 0, 0),
        scale_factors: (0, 0, 0, 0),
        conversion_factor: None,
    },
    UnitLiteralInfo {
        symbol: "mol",
        long_name: "mole",
        type_name: "Mole",
        dimension_exponents: (0, 0, 0, 0, 0, 1, 0, 0),
        scale_factors: (0, 0, 0, 0),
        conversion_factor: None,
    },
    UnitLiteralInfo {
        symbol: "cd",
        long_name: "candela",
        type_name: "Candela",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 1, 0),
        scale_factors: (0, 0, 0, 0),
        conversion_factor: None,
    },
    UnitLiteralInfo {
        symbol: "rad",
        long_name: "radian",
        type_name: "Radian",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        scale_factors: (0, 0, 0, 0),
        conversion_factor: None,
    },
    // Base-60 time units
    UnitLiteralInfo {
        symbol: "min",
        long_name: "minute",
        type_name: "Minute",
        dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0),
        scale_factors: (2, 1, 1, 0),
        conversion_factor: None,
    }, // 1 min = 60 s = 2^2 * 3 * 5
    UnitLiteralInfo {
        symbol: "h",
        long_name: "hour",
        type_name: "Hour",
        dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0),
        scale_factors: (4, 2, 2, 0),
        conversion_factor: None,
    }, // 1 h = 3600 s = 2^4 * 3^2 * 5^2
    UnitLiteralInfo {
        symbol: "hr",
        long_name: "hour",
        type_name: "Hour",
        dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0),
        scale_factors: (4, 2, 2, 0),
        conversion_factor: None,
    }, // 1 hr = 3600 s = 2^4 * 3^2 * 5^2
    UnitLiteralInfo {
        symbol: "d",
        long_name: "day",
        type_name: "Day",
        dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0),
        scale_factors: (7, 3, 2, 0),
        conversion_factor: None,
    }, // 1 d = 86400 s = 2^7 * 3^3 * 5^2
    // Imperial length units
    UnitLiteralInfo {
        symbol: "in",
        long_name: "inch",
        type_name: "Inch",
        dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0),
        scale_factors: (-2, 0, -2, 0),
        conversion_factor: Some(0.0254),
    }, // 1 in = 2.54 cm = 0.0254 m
    UnitLiteralInfo {
        symbol: "yd",
        long_name: "yard",
        type_name: "Yard",
        dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0),
        scale_factors: (0, 0, 0, 0),
        conversion_factor: Some(0.9144),
    }, // 1 yd = 0.9144 m
    UnitLiteralInfo {
        symbol: "ft",
        long_name: "foot",
        type_name: "Foot",
        dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0),
        scale_factors: (0, 0, 0, 0),
        conversion_factor: Some(0.3048),
    }, // 1 ft = 0.3048 m
    UnitLiteralInfo {
        symbol: "mi",
        long_name: "mile",
        type_name: "Mile",
        dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0),
        scale_factors: (3, 0, 3, 0),
        conversion_factor: Some(1.609344),
    }, // 1 mi = 1.609344 km
    // Imperial mass units
    UnitLiteralInfo {
        symbol: "oz",
        long_name: "ounce",
        type_name: "Ounce",
        dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0),
        scale_factors: (-2, 0, -2, 0),
        conversion_factor: Some(2.8349523125),
    }, // 1 oz = 2.8349523125 dag
    UnitLiteralInfo {
        symbol: "lb",
        long_name: "pound",
        type_name: "Pound",
        dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0),
        scale_factors: (0, 0, 0, 0),
        conversion_factor: Some(0.45359237),
    }, // 1 lb = 0.45359237 kg
    // Angle units - all defined by scale factors, no conversion factors needed
    UnitLiteralInfo {
        symbol: "deg",
        long_name: "degree",
        type_name: "Degree",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        scale_factors: (-2, -2, -1, 1),
        conversion_factor: None,
    }, // 1 deg = π/180 rad = 2^-2 * 3^-2 * 5^-1 * π^1
    UnitLiteralInfo {
        symbol: "rev",
        long_name: "revolution",
        type_name: "Revolution",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        scale_factors: (1, 0, 0, 1),
        conversion_factor: None,
    }, // 1 rev = 2π rad = 2^1 * π^1
    UnitLiteralInfo {
        symbol: "rot",
        long_name: "rotation",
        type_name: "Rotation",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        scale_factors: (1, 0, 0, 1),
        conversion_factor: None,
    }, // 1 rot = 2π rad = 2^1 * π^1
    UnitLiteralInfo {
        symbol: "turn",
        long_name: "turn",
        type_name: "Turn",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        scale_factors: (1, 0, 0, 1),
        conversion_factor: None,
    }, // 1 turn = 2π rad = 2^1 * π^1
    UnitLiteralInfo {
        symbol: "arcsec",
        long_name: "arcsecond",
        type_name: "Arcsecond",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        scale_factors: (-6, -4, -3, 1),
        conversion_factor: None,
    }, // 1 arcsec = π/(180*3600) rad = 2^-4 * 3^-2 * 5^-1 * π^1
    UnitLiteralInfo {
        symbol: "arcmin",
        long_name: "arcminute",
        type_name: "Arcminute",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        scale_factors: (-4, -3, -2, 1),
        conversion_factor: None,
    }, // 1 arcmin = π/(180*60) rad = 2^-2 * 3^-1 * 5^-1 * π^1
    UnitLiteralInfo {
        symbol: "gon",
        long_name: "gon",
        type_name: "Gon",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        scale_factors: (-1, -2, -1, 1),
        conversion_factor: None,
    }, // 1 gon = π/200 rad = 2^-1 * 3^-2 * 5^-1 * π^1
    UnitLiteralInfo {
        symbol: "grad",
        long_name: "gradian",
        type_name: "Gradian",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        scale_factors: (-1, -2, -1, 1),
        conversion_factor: None,
    }, // 1 grad = π/200 rad = 2^-1 * 3^-2 * 5^-1 * π^1
];

/// Look up dimension information by name (case-insensitive)
///
/// Returns the dimension info if found, or None if the dimension name is not recognized.
/// The search is case-insensitive and supports various naming conventions:
/// - Canonical names: "Electric Charge"
/// - Underscore variants: "electric_charge"
/// - No space variants: "electriccharge"
/// - UpperCamelCase: "ElectricCharge"
pub fn lookup_dimension_by_name(name: &str) -> Option<&'static DimensionInfo> {
    let name_lower = name.to_lowercase();
    let name_no_spaces = name_lower.replace(' ', "");

    DIMENSION_LOOKUP.iter().find(|info| {
        let info_name_lower = info.name.to_lowercase();

        // Direct match
        if info_name_lower == name_lower {
            return true;
        }

        // Match with spaces removed (for UpperCamelCase support)
        let info_name_no_spaces = info_name_lower.replace(' ', "");
        if info_name_no_spaces == name_no_spaces {
            return true;
        }

        // Handle common naming variations
        match info_name_lower.as_str() {
            "volume mass density" => {
                name_lower == "density"
                    || name_lower == "volume_mass_density"
                    || name_lower == "volumemassdensity"
            }
            "linear mass density" => {
                name_lower == "linear_mass_density" || name_lower == "linearmassdensity"
            }
            "surface mass density" => {
                name_lower == "surface_mass_density" || name_lower == "surfacemassdensity"
            }
            "wave number" => name_lower == "wavenumber" || name_lower == "wave_number",
            "kinematic viscosity" => {
                name_lower == "kinematic_viscosity" || name_lower == "kinematicviscosity"
            }
            "surface tension" => name_lower == "surface_tension" || name_lower == "surfacetension",
            "specific energy" => name_lower == "specific_energy" || name_lower == "specificenergy",
            "specific power" => name_lower == "specific_power" || name_lower == "specificpower",
            "mass flow rate" => name_lower == "mass_flow_rate" || name_lower == "massflowrate",
            "volume flow rate" => {
                name_lower == "volume_flow_rate" || name_lower == "volumeflowrate"
            }
            "power density" => name_lower == "power_density" || name_lower == "powerdensity",
            "force density" => name_lower == "force_density" || name_lower == "forcedensity",
            "heat flux" => name_lower == "heat_flux" || name_lower == "heatflux",
            "electric charge" => {
                name_lower == "electric_charge"
                    || name_lower == "electriccharge"
                    || name_lower == "charge"
            }
            "electric potential" => {
                name_lower == "electric_potential"
                    || name_lower == "electricpotential"
                    || name_lower == "potential"
            }
            "electric resistance" => {
                name_lower == "electric_resistance"
                    || name_lower == "electricresistance"
                    || name_lower == "resistance"
            }
            "electric conductance" => {
                name_lower == "electric_conductance"
                    || name_lower == "electricconductance"
                    || name_lower == "conductance"
            }
            "electric field" => name_lower == "electric_field" || name_lower == "electricfield",
            "magnetic field" => name_lower == "magnetic_field" || name_lower == "magneticfield",
            "magnetic flux" => name_lower == "magnetic_flux" || name_lower == "magneticflux",
            "linear charge density" => {
                name_lower == "linear_charge_density" || name_lower == "linearchargedensity"
            }
            "surface charge density" => {
                name_lower == "surface_charge_density" || name_lower == "surfacechargedensity"
            }
            "volume charge density" => {
                name_lower == "volume_charge_density" || name_lower == "volumechargedensity"
            }
            "magnetizing field" => {
                name_lower == "magnetizing_field" || name_lower == "magnetizingfield"
            }
            "specific heat capacity" => {
                name_lower == "specific_heat_capacity" || name_lower == "specificheatcapacity"
            }
            "molar heat capacity" => {
                name_lower == "molar_heat_capacity" || name_lower == "molarheatcapacity"
            }
            "thermal conductivity" => {
                name_lower == "thermal_conductivity" || name_lower == "thermalconductivity"
            }
            "thermal resistance" => {
                name_lower == "thermal_resistance" || name_lower == "thermalresistance"
            }
            "thermal expansion" => {
                name_lower == "thermal_expansion" || name_lower == "thermalexpansion"
            }
            "molar mass" => name_lower == "molar_mass" || name_lower == "molarmass",
            "molar volume" => name_lower == "molar_volume" || name_lower == "molarvolume",
            "molar concentration" => {
                name_lower == "molar_concentration" || name_lower == "molarconcentration"
            }
            "molal concentration" => {
                name_lower == "molal_concentration" || name_lower == "molalconcentration"
            }
            "molar flow rate" => name_lower == "molar_flow_rate" || name_lower == "molarflowrate",
            "molar flux" => name_lower == "molar_flux" || name_lower == "molarflux",
            "molar energy" => name_lower == "molar_energy" || name_lower == "molarenergy",
            "luminous exposure" => {
                name_lower == "luminous_exposure" || name_lower == "luminousexposure"
            }
            "luminous efficacy" => {
                name_lower == "luminous_efficacy" || name_lower == "luminousefficacy"
            }
            _ => false,
        }
    })
}

/// Look up dimension information by exponents
///
/// Returns the dimension info if found, or None if the exponent combination is not recognized.
pub fn lookup_dimension_by_exponents(
    exponents: DimensionExponents,
) -> Option<&'static DimensionInfo> {
    DIMENSION_LOOKUP
        .iter()
        .find(|info| info.exponents == exponents)
}

/// Look up base unit information by symbol
pub fn lookup_base_unit(unit_symbol: &str) -> Option<&'static BaseUnitInfo> {
    BASE_UNITS.iter().find(|unit| unit.symbol == unit_symbol)
}

/// Look up unit literal information by symbol or name
pub fn lookup_unit_literal(unit_name: &str) -> Option<&'static UnitLiteralInfo> {
    // First try direct lookup
    if let Some(info) = UNIT_LITERALS.iter().find(|info| {
        info.symbol == unit_name ||
        info.long_name == unit_name ||
        info.type_name == unit_name ||
        // Handle plural forms
        info.long_name.to_string() + "s" == unit_name ||
        info.symbol.to_string() + "s" == unit_name
    }) {
        return Some(info);
    }

    // If not found, try to parse as a prefixed unit
    if let Some(info) = parse_prefixed_unit(unit_name) {
        return Some(info);
    }

    // Try to parse as prefixed unit with long names (like "kilometer", "centimeter", etc.)
    parse_prefixed_unit_long_name(unit_name)
}

/// Parse a prefixed unit with long names (like "kilometer", "centimeter", etc.)
fn parse_prefixed_unit_long_name(unit_name: &str) -> Option<&'static UnitLiteralInfo> {
    // Common prefix mappings for long names
    let prefix_mappings = [
        ("kilo", "k"),
        ("centi", "c"),
        ("milli", "m"),
        ("micro", "μ"),
        ("nano", "n"),
        ("pico", "p"),
        ("femto", "f"),
        ("atto", "a"),
        ("zepto", "z"),
        ("yocto", "y"),
        ("deca", "da"),
        ("hecto", "h"),
        ("mega", "M"),
        ("giga", "G"),
        ("tera", "T"),
        ("peta", "P"),
        ("exa", "E"),
        ("zetta", "Z"),
        ("yotta", "Y"),
    ];

    // Common base unit long names
    let base_unit_mappings = [
        ("meter", "m"),
        ("gram", "g"),
        ("second", "s"),
        ("ampere", "A"),
        ("kelvin", "K"),
        ("mole", "mol"),
        ("candela", "cd"),
        ("radian", "rad"),
    ];

    // Try to find a prefix and base unit combination
    for (prefix_long, prefix_short) in prefix_mappings.iter() {
        for (base_long, base_short) in base_unit_mappings.iter() {
            if unit_name.starts_with(prefix_long) && unit_name.ends_with(base_long) {
                let expected_length = prefix_long.len() + base_long.len();
                if unit_name.len() == expected_length {
                    // Find the base unit in UNIT_LITERALS
                    if let Some(base_literal) =
                        UNIT_LITERALS.iter().find(|info| info.symbol == *base_short)
                    {
                        // Find the prefix in SI_PREFIXES
                        if let Some(_prefix_info) =
                            SI_PREFIXES.iter().find(|info| info.symbol == *prefix_short)
                        {
                            // Return the base unit info - the calling code will handle the prefix conversion
                            return Some(base_literal);
                        }
                    }
                }
            }
        }
    }

    None
}

/// Parse a prefixed unit and return the corresponding UnitLiteralInfo
/// This handles units like "km", "cm", "mm", etc. by combining SI prefixes with base units
fn parse_prefixed_unit(unit_name: &str) -> Option<&'static UnitLiteralInfo> {
    // Try to find a base unit that this unit name ends with
    for base_unit in BASE_UNITS {
        if base_unit.symbol == "dimensionless" {
            continue;
        }

        if unit_name.ends_with(base_unit.symbol) {
            let prefix_part = &unit_name[..unit_name.len() - base_unit.symbol.len()];

            // If no prefix, it should have been found in the direct lookup above
            if prefix_part.is_empty() {
                continue;
            }

            // Look up the prefix
            if let Some(_prefix_info) = lookup_si_prefix(prefix_part) {
                // Find the corresponding base unit in UNIT_LITERALS
                if let Some(base_literal) = UNIT_LITERALS
                    .iter()
                    .find(|info| info.symbol == base_unit.symbol)
                {
                    // We need to return the base unit info but with adjusted scale factors
                    // Since we can't create static data at runtime, we'll return the base unit
                    // and let the calling code handle the prefix conversion
                    return Some(base_literal);
                }
            }
        }
    }

    None
}

/// Look up SI prefix information by symbol
pub fn lookup_si_prefix(prefix: &str) -> Option<&'static PrefixInfo> {
    SI_PREFIXES.iter().find(|info| info.symbol == prefix)
}

/// Map scale type names to their base unit symbols
/// This function takes a scale type name (like "Kilogram", "Millimeter") and returns
/// the corresponding base unit symbol (like "g", "m") that can be used in unit expressions.
pub fn scale_type_to_base_unit(scale_type: &str) -> Option<&'static str> {
    // Map the scale type names to their base unit symbols
    match scale_type {
        // Mass scales
        "Kilogram" | "Gram" | "Milligram" | "Microgram" | "Nanogram" | "Picogram" | "Femtogram"
        | "Attogram" | "Zeptogram" | "Yoctogram" | "Megagram" | "Gigagram" | "Teragram"
        | "Petagram" | "Exagram" | "Zettagram" | "Yottagram" => Some("g"),

        // Length scales
        "Meter" | "Millimeter" | "Micrometer" | "Nanometer" | "Picometer" | "Femtometer"
        | "Attometer" | "Zeptometer" | "Yoctometer" | "Kilometer" | "Megameter" | "Gigameter"
        | "Terameter" | "Petameter" | "Exameter" | "Zettameter" | "Yottameter" => Some("m"),

        // Time scales
        "Second" | "Millisecond" | "Microsecond" | "Nanosecond" | "Picosecond" | "Femtosecond"
        | "Attosecond" | "Zeptosecond" | "Yoctosecond" | "Kilosecond" | "Megasecond"
        | "Gigasecond" | "Terasecond" | "Petasecond" | "Exasecond" | "Zettasecond"
        | "Yottasecond" => Some("s"),

        // Current scales
        "Ampere" | "Milliampere" | "Microampere" | "Nanoampere" | "Picoampere" | "Femtoampere"
        | "Attoampere" | "Zeptoampere" | "Yoctoampere" | "Kiloampere" | "Megaampere"
        | "Gigaampere" | "Teraampere" | "Petaampere" | "Exaampere" | "Zettaampere"
        | "Yottaampere" => Some("A"),

        // Temperature scales
        "Kelvin" | "Millikelvin" | "Microkelvin" | "Nanokelvin" | "Picokelvin" | "Femtokelvin"
        | "Attokelvin" | "Zeptokelvin" | "Yoctokelvin" | "Kilokelvin" | "Megakelvin"
        | "Gigakelvin" | "Terakelvin" | "Petakelvin" | "Exakelvin" | "Zettakelvin"
        | "Yottakelvin" => Some("K"),

        // Amount scales
        "Mole" | "Millimole" | "Micromole" | "Nanomole" | "Picomole" | "Femtomole" | "Attomole"
        | "Zeptomole" | "Yoctomole" | "Kilomole" | "Megamole" | "Gigamole" | "Teramole"
        | "Petamole" | "Examole" | "Zettamole" | "Yottamole" => Some("mol"),

        // Luminosity scales
        "Candela" | "Millicandela" | "Microcandela" | "Nanocandela" | "Picocandela"
        | "Femtocandela" | "Attocandela" | "Zeptocandela" | "Yoctocandela" | "Kilocandela"
        | "Megacandela" | "Gigacandela" | "Teracandela" | "Petacandela" | "Exacandela"
        | "Zettacandela" | "Yottacandela" => Some("cd"),

        // Angle scales
        "Radian" | "Milliradian" | "Microradian" | "Nanoradian" | "Picoradian" | "Femtoradian"
        | "Attoradian" | "Zeptoradian" | "Yoctoradian" | "Kiloradian" | "Megaradian"
        | "Gigaradian" | "Teraradian" | "Petaradian" | "Exaradian" | "Zettaradian"
        | "Yottaradian" => Some("rad"),

        _ => None,
    }
}

/// Unit mapping information for compound units
#[derive(Debug, Clone)]
pub struct CompoundUnitInfo {
    pub symbol: &'static str,
    pub dimension_exponents: DimensionExponents,
    pub long_name: &'static str,
}

/// Compound unit definitions for common derived units
pub const COMPOUND_UNITS: &[CompoundUnitInfo] = &[
    // Energy and work
    CompoundUnitInfo {
        symbol: "J",
        dimension_exponents: (1, 2, -2, 0, 0, 0, 0, 0),
        long_name: "Joule",
    },
    // Force
    CompoundUnitInfo {
        symbol: "N",
        dimension_exponents: (1, 1, -2, 0, 0, 0, 0, 0),
        long_name: "Newton",
    },
    // Power
    CompoundUnitInfo {
        symbol: "W",
        dimension_exponents: (1, 2, -3, 0, 0, 0, 0, 0),
        long_name: "Watt",
    },
    // Pressure
    CompoundUnitInfo {
        symbol: "Pa",
        dimension_exponents: (1, -1, -2, 0, 0, 0, 0, 0),
        long_name: "Pascal",
    },
    // Frequency
    CompoundUnitInfo {
        symbol: "Hz",
        dimension_exponents: (0, 0, -1, 0, 0, 0, 0, 0),
        long_name: "Hertz",
    },
    // Electric charge
    CompoundUnitInfo {
        symbol: "C",
        dimension_exponents: (0, 0, 1, 1, 0, 0, 0, 0),
        long_name: "Coulomb",
    },
    // Electric potential
    CompoundUnitInfo {
        symbol: "V",
        dimension_exponents: (1, 2, -3, -1, 0, 0, 0, 0),
        long_name: "Volt",
    },
    // Capacitance
    CompoundUnitInfo {
        symbol: "F",
        dimension_exponents: (-1, -2, 4, 2, 0, 0, 0, 0),
        long_name: "Farad",
    },
    // Electric resistance
    CompoundUnitInfo {
        symbol: "Ω",
        dimension_exponents: (1, 2, -3, -2, 0, 0, 0, 0),
        long_name: "Ohm",
    },
    // Electric conductance
    CompoundUnitInfo {
        symbol: "S",
        dimension_exponents: (-1, -2, 3, 2, 0, 0, 0, 0),
        long_name: "Siemens",
    },
    // Inductance
    CompoundUnitInfo {
        symbol: "H",
        dimension_exponents: (1, 2, -2, -2, 0, 0, 0, 0),
        long_name: "Henry",
    },
    // Magnetic field
    CompoundUnitInfo {
        symbol: "T",
        dimension_exponents: (1, 0, -2, -1, 0, 0, 0, 0),
        long_name: "Tesla",
    },
    // Magnetic flux
    CompoundUnitInfo {
        symbol: "Wb",
        dimension_exponents: (1, 2, -2, -1, 0, 0, 0, 0),
        long_name: "Weber",
    },
    // Luminous flux
    CompoundUnitInfo {
        symbol: "lm",
        dimension_exponents: (0, 0, 0, 0, 0, 0, 1, 0),
        long_name: "Lumen",
    },
    // Illuminance
    CompoundUnitInfo {
        symbol: "lx",
        dimension_exponents: (0, -2, 0, 0, 0, 0, 1, 0),
        long_name: "Lux",
    },
];

/// Check if a unit symbol represents a base unit with SI prefixes
pub fn is_prefixed_base_unit(unit_symbol: &str) -> Option<(&'static str, &'static str)> {
    // Check each base unit to see if the symbol starts with it
    for base_unit in BASE_UNITS {
        if unit_symbol == base_unit.symbol {
            return Some((base_unit.symbol, base_unit.symbol));
        }

        // Check if it's a prefixed version
        for prefix in SI_PREFIXES {
            let prefixed_symbol = format!("{}{}", prefix.symbol, base_unit.symbol);
            if unit_symbol == prefixed_symbol {
                return Some((base_unit.symbol, prefix.symbol));
            }
        }
    }
    None
}

/// Check if a unit symbol represents a compound unit with SI prefixes (like kJ, mW, etc.)
pub fn is_prefixed_compound_unit(unit_symbol: &str) -> Option<(&'static str, &'static str)> {
    // Try to find a compound unit that this unit name ends with
    for compound_unit in COMPOUND_UNITS {
        if unit_symbol.ends_with(compound_unit.symbol) {
            let prefix_part = &unit_symbol[..unit_symbol.len() - compound_unit.symbol.len()];

            // If no prefix, it should have been found in the direct lookup above
            if prefix_part.is_empty() {
                continue;
            }

            // Check if this is a valid prefix and get the prefix symbol
            if let Some(prefix_info) = lookup_si_prefix(prefix_part) {
                return Some((compound_unit.symbol, prefix_info.symbol));
            }
        }
    }

    None
}

/// Convert a long unit name to its short form (e.g., "kilometer" -> "km")
pub fn convert_long_name_to_short(unit_name: &str) -> Option<String> {
    // Try to find a prefix and base unit combination using existing data
    for prefix in SI_PREFIXES {
        for base_unit in BASE_UNITS {
            if unit_name.starts_with(prefix.long_name) && unit_name.ends_with(base_unit.long_name) {
                let expected_length = prefix.long_name.len() + base_unit.long_name.len();
                if unit_name.len() == expected_length {
                    // Construct the short form (e.g., "kilometer" -> "km")
                    return Some(format!("{}{}", prefix.symbol, base_unit.symbol));
                }
            }
        }
    }

    None
}

/// Get the dimension exponents for a unit symbol
pub fn get_unit_dimensions(unit_symbol: &str) -> Option<DimensionExponents> {
    // First check if it's a prefixed base unit
    if let Some((base_symbol, _prefix)) = is_prefixed_base_unit(unit_symbol) {
        if let Some(base_unit) = lookup_base_unit(base_symbol) {
            return Some(base_unit.dimension_exponents);
        }
    }

    // Then check compound units
    if let Some(compound_unit) = COMPOUND_UNITS.iter().find(|u| u.symbol == unit_symbol) {
        return Some(compound_unit.dimension_exponents);
    }

    // Check if it's a prefixed compound unit (like kJ, mW, etc.)
    if let Some((base_symbol, _prefix)) = is_prefixed_compound_unit(unit_symbol) {
        if let Some(compound_unit) = COMPOUND_UNITS.iter().find(|u| u.symbol == base_symbol) {
            return Some(compound_unit.dimension_exponents);
        }
    }

    None
}

/// Get the base unit symbol for a dimension type
pub fn get_base_unit_for_dimension(
    dimension_exponents: DimensionExponents,
) -> Option<&'static str> {
    // Find the base unit that matches these dimension exponents
    BASE_UNITS
        .iter()
        .find(|unit| unit.dimension_exponents == dimension_exponents)
        .map(|unit| unit.symbol)
}

/// Convert dimension exponents to a unit expression string
/// This creates a string like "kg * m^2 / s^2" from dimension exponents
pub fn dimension_exponents_to_unit_expression(
    exponents: DimensionExponents,
    base_units: &[(&str, &str); 8], // (mass, length, time, current, temp, amount, lum, angle)
) -> String {
    let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) =
        exponents;
    let [mass_base, length_base, time_base, current_base, temp_base, amount_base, lum_base, angle_base] =
        base_units;

    let mut terms = Vec::new();

    // Add positive exponents (numerator)
    if mass_exp > 0 {
        terms.push(format!("{}^{}", mass_base.0, mass_exp));
    }
    if length_exp > 0 {
        terms.push(format!("{}^{}", length_base.0, length_exp));
    }
    if time_exp > 0 {
        terms.push(format!("{}^{}", time_base.0, time_exp));
    }
    if current_exp > 0 {
        terms.push(format!("{}^{}", current_base.0, current_exp));
    }
    if temp_exp > 0 {
        terms.push(format!("{}^{}", temp_base.0, temp_exp));
    }
    if amount_exp > 0 {
        terms.push(format!("{}^{}", amount_base.0, amount_exp));
    }
    if lum_exp > 0 {
        terms.push(format!("{}^{}", lum_base.0, lum_exp));
    }
    if angle_exp > 0 {
        terms.push(format!("{}^{}", angle_base.0, angle_exp));
    }

    // Add negative exponents (denominator)
    let mut denom_terms = Vec::new();
    if mass_exp < 0 {
        denom_terms.push(format!("{}^{}", mass_base.0, -mass_exp));
    }
    if length_exp < 0 {
        denom_terms.push(format!("{}^{}", length_base.0, -length_exp));
    }
    if time_exp < 0 {
        denom_terms.push(format!("{}^{}", time_base.0, -time_exp));
    }
    if current_exp < 0 {
        denom_terms.push(format!("{}^{}", current_base.0, -current_exp));
    }
    if temp_exp < 0 {
        denom_terms.push(format!("{}^{}", temp_base.0, -temp_exp));
    }
    if amount_exp < 0 {
        denom_terms.push(format!("{}^{}", amount_base.0, -amount_exp));
    }
    if lum_exp < 0 {
        denom_terms.push(format!("{}^{}", lum_base.0, -lum_exp));
    }
    if angle_exp < 0 {
        denom_terms.push(format!("{}^{}", angle_base.0, -angle_exp));
    }

    // Handle special case of 1/s (frequency)
    if exponents == (0, 0, -1, 0, 0, 0, 0, 0) {
        return format!("1 / {}", time_base.0);
    }

    // Build the expression
    let mut expr = String::new();

    if terms.is_empty() && denom_terms.is_empty() {
        return "1".to_string(); // dimensionless
    }

    if !terms.is_empty() {
        expr.push_str(&terms.join(" * "));
    } else {
        expr.push('1');
    }

    if !denom_terms.is_empty() {
        if denom_terms.len() == 1 {
            expr.push_str(&format!(" / {}", denom_terms[0]));
        } else {
            expr.push_str(&format!(" / ({})", denom_terms.join(" * ")));
        }
    }

    expr
}

/// Parse a scale type name to extract prefix and base unit, then construct the unit symbol
/// This function takes a scale type name (like "Kilogram", "Millimeter") and returns
/// the corresponding unit symbol (like "kg", "mm") that can be used in unit expressions.
///
/// It uses the same parsing logic as the unit! macro to extract prefixes and base units.
pub fn scale_type_to_unit_symbol(scale_type: &str) -> Option<String> {
    // Map scale type names to their corresponding base unit names
    let base_unit_name = match scale_type {
        // Mass scales - all end with "gram"
        name if name.ends_with("gram") => "gram",
        // Length scales - all end with "meter"
        name if name.ends_with("meter") => "meter",
        // Time scales - all end with "second"
        name if name.ends_with("second") => "second",
        // Current scales - all end with "ampere"
        name if name.ends_with("ampere") => "ampere",
        // Temperature scales - all end with "kelvin"
        name if name.ends_with("kelvin") => "kelvin",
        // Amount scales - all end with "mole"
        name if name.ends_with("mole") => "mole",
        // Luminosity scales - all end with "candela"
        name if name.ends_with("candela") => "candela",
        // Angle scales - all end with "radian"
        name if name.ends_with("radian") => "radian",
        _ => return None,
    };

    // Extract the prefix from the scale type name
    let prefix = if scale_type.len() > base_unit_name.len() {
        &scale_type[..scale_type.len() - base_unit_name.len()]
    } else {
        "" // No prefix
    };

    // Map base unit names to their symbols
    let base_unit_symbol = match base_unit_name {
        "gram" => "g",
        "meter" => "m",
        "second" => "s",
        "ampere" => "A",
        "kelvin" => "K",
        "mole" => "mol",
        "candela" => "cd",
        "radian" => "rad",
        _ => return None,
    };

    // Map prefix names to their symbols
    let prefix_symbol = match prefix {
        "" => "",
        "Quecto" => "q",
        "Ronto" => "r",
        "Yocto" => "y",
        "Zepto" => "z",
        "Atto" => "a",
        "Femto" => "f",
        "Pico" => "p",
        "Nano" => "n",
        "Micro" => "u",
        "Milli" => "m",
        "Centi" => "c",
        "Deci" => "d",
        "Deca" => "da",
        "Hecto" => "h",
        "Kilo" => "k",
        "Mega" => "M",
        "Giga" => "G",
        "Tera" => "T",
        "Peta" => "P",
        "Exa" => "E",
        "Zetta" => "Z",
        "Yotta" => "Y",
        "Ronna" => "R",
        "Quetta" => "Q",
        _ => return None,
    };

    // Construct the unit symbol
    Some(format!("{}{}", prefix_symbol, base_unit_symbol))
}
