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
}

/// Unit literal information for formatting and conversion
#[derive(Debug, Clone)]
pub struct UnitLiteralInfo {
    pub symbol: &'static str,
    pub long_name: &'static str,
    pub type_name: &'static str, // The actual type name (e.g., "Kilometer", "Meter")
    pub dimension_exponents: DimensionExponents,
    pub scale_factors: (i16, i16, i16, i16, i16), // p2, p3, p5, p10, pi
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
pub const SI_PREFIXES: &[PrefixInfo] = &[
    // Small prefixes (negative powers of 10)
    PrefixInfo { symbol: "p", scale_factor: -12, long_name: "pico" },
    PrefixInfo { symbol: "n", scale_factor: -9, long_name: "nano" },
    PrefixInfo { symbol: "u", scale_factor: -6, long_name: "micro" },
    PrefixInfo { symbol: "m", scale_factor: -3, long_name: "milli" },
    
    // Large prefixes (positive powers of 10)
    PrefixInfo { symbol: "k", scale_factor: 3, long_name: "kilo" },
    PrefixInfo { symbol: "M", scale_factor: 6, long_name: "mega" },
    PrefixInfo { symbol: "G", scale_factor: 9, long_name: "giga" },
    PrefixInfo { symbol: "T", scale_factor: 12, long_name: "tera" },
    PrefixInfo { symbol: "P", scale_factor: 15, long_name: "peta" },
    PrefixInfo { symbol: "E", scale_factor: 18, long_name: "exa" },
    PrefixInfo { symbol: "Z", scale_factor: 21, long_name: "zetta" },
    PrefixInfo { symbol: "Y", scale_factor: 24, long_name: "yotta" },
];

/// Base unit definitions
/// Each base unit maps to its dimension exponents and inherent scale factor
pub const BASE_UNITS: &[BaseUnitInfo] = &[
    // Mass - all mass units are relative to kilogram (scale factor 0)
    BaseUnitInfo { symbol: "g", dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0), inherent_scale_factor: -3, long_name: "gram" },
    
    // Length - all length units are relative to meter (scale factor 0)
    BaseUnitInfo { symbol: "m", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), inherent_scale_factor: 0, long_name: "meter" },
    
    // Time - all time units are relative to second (scale factor 0)
    BaseUnitInfo { symbol: "s", dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0), inherent_scale_factor: 0, long_name: "second" },
    BaseUnitInfo { symbol: "min", dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0), inherent_scale_factor: 0, long_name: "minute" },
    BaseUnitInfo { symbol: "h", dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0), inherent_scale_factor: 0, long_name: "hour" },
    BaseUnitInfo { symbol: "d", dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0), inherent_scale_factor: 0, long_name: "day" },
    BaseUnitInfo { symbol: "yr", dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0), inherent_scale_factor: 0, long_name: "year" },
    
    // Current - all current units are relative to ampere (scale factor 0)
    BaseUnitInfo { symbol: "A", dimension_exponents: (0, 0, 0, 1, 0, 0, 0, 0), inherent_scale_factor: 0, long_name: "ampere" },
    
    // Temperature - all temperature units are relative to kelvin (scale factor 0)
    BaseUnitInfo { symbol: "K", dimension_exponents: (0, 0, 0, 0, 1, 0, 0, 0), inherent_scale_factor: 0, long_name: "kelvin" },
    
    // Amount - all amount units are relative to mole (scale factor 0)
    BaseUnitInfo { symbol: "mol", dimension_exponents: (0, 0, 0, 0, 0, 1, 0, 0), inherent_scale_factor: 0, long_name: "mole" },
    
    // Luminosity - all luminosity units are relative to candela (scale factor 0)
    BaseUnitInfo { symbol: "cd", dimension_exponents: (0, 0, 0, 0, 0, 0, 1, 0), inherent_scale_factor: 0, long_name: "candela" },
    
    // Angle - all angle units are relative to radian (scale factor 0)
    BaseUnitInfo { symbol: "rad", dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1), inherent_scale_factor: 0, long_name: "radian" },
    BaseUnitInfo { symbol: "deg", dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 1), inherent_scale_factor: 0, long_name: "degree" },
    
    // Imperial length units
    BaseUnitInfo { symbol: "in", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), inherent_scale_factor: -2, long_name: "inch" }, // 1 in = 2.54 cm = 0.0254 m = 10^-2 * 2.54 m
    BaseUnitInfo { symbol: "ft", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), inherent_scale_factor: 0, long_name: "foot" }, // 1 ft = 0.3048 m
    BaseUnitInfo { symbol: "yd", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), inherent_scale_factor: 0, long_name: "yard" }, // 1 yd = 0.9144 m
    BaseUnitInfo { symbol: "mi", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), inherent_scale_factor: 3, long_name: "mile" }, // 1 mi = 1.609344 km = 10^3 * 1.609344 m
    
    // Imperial mass units
    BaseUnitInfo { symbol: "oz", dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0), inherent_scale_factor: -3, long_name: "ounce" }, // 1 oz = 28.349523125 g
    BaseUnitInfo { symbol: "lb", dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0), inherent_scale_factor: 0, long_name: "pound" }, // 1 lb = 0.45359237 kg
    BaseUnitInfo { symbol: "st", dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0), inherent_scale_factor: 0, long_name: "stone" }, // 1 st = 6.35029318 kg
    BaseUnitInfo { symbol: "ton", dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0), inherent_scale_factor: 3, long_name: "ton" }, // 1 ton = 1.0160469088 Mg
    
    // Special cases
    BaseUnitInfo { symbol: "dimensionless", dimension_exponents: (0, 0, 0, 0, 0, 0, 0, 0), inherent_scale_factor: 0, long_name: "dimensionless" },
];

/// Comprehensive unit literal lookup table
/// This includes all the units defined in default_declarators and imperial_declarators
pub const UNIT_LITERALS: &[UnitLiteralInfo] = &[
    // Length units (from SILength)
    UnitLiteralInfo { symbol: "m", long_name: "meter", type_name: "Meter", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, 0, 0) },
    UnitLiteralInfo { symbol: "km", long_name: "kilometer", type_name: "Kilometer", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, 3, 0) },
    UnitLiteralInfo { symbol: "cm", long_name: "centimeter", type_name: "Centimeter", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, -2, 0) },
    UnitLiteralInfo { symbol: "mm", long_name: "millimeter", type_name: "Millimeter", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, -3, 0) },
    
    // Mass units (from SIMass)
    UnitLiteralInfo { symbol: "kg", long_name: "kilogram", type_name: "Kilogram", dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, 0, 0) },
    UnitLiteralInfo { symbol: "g", long_name: "gram", type_name: "Gram", dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, -3, 0) },
    
    // Time units (from SITime and CommonTime)
    UnitLiteralInfo { symbol: "s", long_name: "second", type_name: "Second", dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, 0, 0) },
    UnitLiteralInfo { symbol: "min", long_name: "minute", type_name: "Minute", dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0), scale_factors: (2, 1, 1, 0, 0) },
    UnitLiteralInfo { symbol: "h", long_name: "hour", type_name: "Hour", dimension_exponents: (0, 0, 1, 0, 0, 0, 0, 0), scale_factors: (4, 2, 2, 0, 0) },
    
    // Imperial length units (from ImperialLength)
    UnitLiteralInfo { symbol: "in", long_name: "inch", type_name: "Inch", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, -2, 0) },
    UnitLiteralInfo { symbol: "ft", long_name: "foot", type_name: "Foot", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, 0, 0) },
    UnitLiteralInfo { symbol: "yd", long_name: "yard", type_name: "Yard", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, 0, 0) },
    UnitLiteralInfo { symbol: "mi", long_name: "mile", type_name: "Mile", dimension_exponents: (0, 1, 0, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, 3, 0) },
    
    // Imperial mass units (from ImperialMass)
    UnitLiteralInfo { symbol: "oz", long_name: "ounce", type_name: "Ounce", dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, -3, 0) },
    UnitLiteralInfo { symbol: "lb", long_name: "pound", type_name: "Pound", dimension_exponents: (1, 0, 0, 0, 0, 0, 0, 0), scale_factors: (0, 0, 0, 0, 0) },
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

/// Look up unit literal information by symbol or name
pub fn lookup_unit_literal(unit_name: &str) -> Option<&'static UnitLiteralInfo> {
    UNIT_LITERALS.iter().find(|info| {
        info.symbol == unit_name || 
        info.long_name == unit_name ||
        info.type_name == unit_name ||
        // Handle plural forms
        info.long_name.to_string() + "s" == unit_name ||
        info.symbol.to_string() + "s" == unit_name
    })
}

/// Look up SI prefix information by symbol
pub fn lookup_si_prefix(prefix: &str) -> Option<&'static PrefixInfo> {
    SI_PREFIXES.iter().find(|info| info.symbol == prefix)
}

/// Look up base unit information by symbol
pub fn lookup_base_unit(unit: &str) -> Option<&'static BaseUnitInfo> {
    BASE_UNITS.iter().find(|info| info.symbol == unit)
}
