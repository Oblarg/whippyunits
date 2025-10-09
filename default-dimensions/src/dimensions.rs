//! Unified dimension data for whippyunits
//!
//! This module consolidates all dimension information that was previously scattered
//! across DIMENSION_LOOKUP, UNIT_LITERALS, and COMPOUND_UNITS into a single,
//! hierarchical taxonomy.

/// Dimension exponents tuple: (mass, length, time, current, temperature, amount, luminosity, angle)
pub type DimensionExponents = (i16, i16, i16, i16, i16, i16, i16, i16);

/// A unit within a dimension
#[derive(Debug, Clone)]
pub struct Unit {
    pub symbols: &'static [&'static str], // ["rad"], ["deg"], ["g"], ["h", "hr"] (unit symbols)
    pub long_name: &'static str,          // "radian", "degree", "gram", "hour"
    pub scale_factors: Option<(i16, i16, i16, i16)>, // Optional - absence means all 0 (SI base)
    pub conversion_factor: Option<f64>, // Optional - presence means declarator-only support
}

/// A dimension with its associated units
#[derive(Debug, Clone)]
pub struct Dimension {
    pub name: &'static str,           // "Angle", "Mass", "Length"
    pub symbol: &'static str,         // "A", "M", "L" (dimension symbol)
    pub exponents: DimensionExponents, // (0,0,0,0,0,0,0,1) for angle
    pub units: &'static [Unit],       // All units for this dimension
}

/// Unified taxonomy of all dimensions and their units
pub const DIMENSIONS: &[Dimension] = &[
    // Mass dimension
    Dimension {
        name: "Mass",
        symbol: "M",
        exponents: (1, 0, 0, 0, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["g"],
                long_name: "gram",
                scale_factors: Some((-3, 0, -3, 0)), // 10^-3 (base scale offset)
                conversion_factor: None,
            },
            Unit {
                symbols: &["oz"],
                long_name: "ounce",
                scale_factors: Some((-2, 0, -2, 0)), // 2^-2 * 5^-2 = 0.01 (stored in decigrams)
                conversion_factor: Some(2.8349523125),
            },
            Unit {
                symbols: &["lb"],
                long_name: "pound",
                scale_factors: None, // 2^0 * 3^0 * 5^0 = 0 (stored in kilograms)
                conversion_factor: Some(0.45359237),
            },
        ],
    },
    
    // Length dimension
    Dimension {
        name: "Length",
        symbol: "L",
        exponents: (0, 1, 0, 0, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["m"],
                long_name: "meter",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
            Unit {
                symbols: &["in"],
                long_name: "inch",
                scale_factors: Some((-2, 0, -2, 0)), // 2^-2 * 5^-2 = 0.01 (stored in centimeters)
                conversion_factor: Some(0.0254),
            },
            Unit {
                symbols: &["ft"],
                long_name: "foot",
                scale_factors: None, // 2^0 * 3^0 * 5^0 = 0 (stored in meters)
                conversion_factor: Some(0.3048),
            },
            Unit {
                symbols: &["yd"],
                long_name: "yard",
                scale_factors: None, // 2^0 * 3^0 * 5^0 = 0 (stored in meters)
                conversion_factor: Some(0.9144),
            },
            Unit {
                symbols: &["mi"],
                long_name: "mile",
                scale_factors: Some((3, 0, 3, 0)), // 2^3 * 3^0 * 5^3 = 1000 (stored in kilometers)
                conversion_factor: Some(1.609344),
            },
        ],
    },
    
    // Time dimension
    Dimension {
        name: "Time",
        symbol: "T",
        exponents: (0, 0, 1, 0, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["s"],
                long_name: "second",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
            Unit {
                symbols: &["min"],
                long_name: "minute",
                scale_factors: Some((2, 1, 1, 0)), // 60 = 2^2 * 3^1 * 5^1
                conversion_factor: None,
            },
            Unit {
                symbols: &["h", "hr"],
                long_name: "hour",
                scale_factors: Some((4, 2, 2, 0)), // 3600 = 2^4 * 3^2 * 5^2
                conversion_factor: None,
            },
            Unit {
                symbols: &["d"],
                long_name: "day",
                scale_factors: Some((7, 1, 2, 0)), // 86400 = 2^7 * 3^1 * 5^2
                conversion_factor: None,
            },
        ],
    },
    
    // Current dimension
    Dimension {
        name: "Current",
        symbol: "I",
        exponents: (0, 0, 0, 1, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["A"],
                long_name: "ampere",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Temperature dimension
    Dimension {
        name: "Temperature",
        symbol: "θ",
        exponents: (0, 0, 0, 0, 1, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["K"],
                long_name: "kelvin",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Amount dimension
    Dimension {
        name: "Amount",
        symbol: "N",
        exponents: (0, 0, 0, 0, 0, 1, 0, 0),
        units: &[
            Unit {
                symbols: &["mol"],
                long_name: "mole",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Luminosity dimension
    Dimension {
        name: "Luminosity",
        symbol: "Cd",
        exponents: (0, 0, 0, 0, 0, 0, 1, 0),
        units: &[
            Unit {
                symbols: &["cd"],
                long_name: "candela",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Angle dimension
    Dimension {
        name: "Angle",
        symbol: "A",
        exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        units: &[
            Unit {
                symbols: &["rad"],
                long_name: "radian",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
            Unit {
                symbols: &["deg"],
                long_name: "degree",
                scale_factors: Some((-2, -2, -1, 1)), // π/180 = 2^-2 * 3^-2 * 5^-1 * π^1
                conversion_factor: None,
            },
            Unit {
                symbols: &["grad", "gon"],
                long_name: "gradian",
                scale_factors: Some((-3, -1, -1, 1)), // π/200 = 2^-3 * 3^-1 * 5^-1 * π^1
                conversion_factor: None,
            },
            Unit {
                symbols: &["rot", "turn"],
                long_name: "turn",
                scale_factors: Some((1, 0, 0, 1)), // 2π = 2^1 * 3^0 * 5^0 * π^1
                conversion_factor: None,
            },
            Unit {
                symbols: &["arcmin"],
                long_name: "arcminute",
                scale_factors: Some((-4, -2, -2, 1)), // π/10800 = 2^-4 * 3^-2 * 5^-2 * π^1
                conversion_factor: None,
            },
            Unit {
                symbols: &["arcsec"],
                long_name: "arcsecond",
                scale_factors: Some((-6, -2, -2, 1)), // π/648000 = 2^-6 * 3^-2 * 5^-2 * π^1
                conversion_factor: None,
            },
        ],
    },
    
    // Area dimension
    Dimension {
        name: "Area",
        symbol: "L²",
        exponents: (0, 2, 0, 0, 0, 0, 0, 0),
        units: &[
            // No atomic units for area - it's a derived dimension
        ],
    },
    
    // Volume dimension
    Dimension {
        name: "Volume",
        symbol: "L³",
        exponents: (0, 3, 0, 0, 0, 0, 0, 0),
        units: &[
            // No atomic units for volume - it's a derived dimension
        ],
    },
    
    // Frequency dimension
    Dimension {
        name: "Frequency",
        symbol: "T⁻¹",
        exponents: (0, 0, -1, 0, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["Hz"],
                long_name: "hertz",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Force dimension
    Dimension {
        name: "Force",
        symbol: "MLT⁻²",
        exponents: (1, 1, -2, 0, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["N"],
                long_name: "newton",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Energy dimension
    Dimension {
        name: "Energy",
        symbol: "ML²T⁻²",
        exponents: (1, 2, -2, 0, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["J"],
                long_name: "joule",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Power dimension
    Dimension {
        name: "Power",
        symbol: "ML²T⁻³",
        exponents: (1, 2, -3, 0, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["W"],
                long_name: "watt",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Pressure dimension
    Dimension {
        name: "Pressure",
        symbol: "ML⁻¹T⁻²",
        exponents: (1, -1, -2, 0, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["Pa"],
                long_name: "pascal",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Electric charge dimension
    Dimension {
        name: "Electric Charge",
        symbol: "TI",
        exponents: (0, 0, 1, 1, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["C"],
                long_name: "coulomb",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Electric potential dimension
    Dimension {
        name: "Electric Potential",
        symbol: "ML²T⁻³I⁻¹",
        exponents: (1, 2, -3, -1, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["V"],
                long_name: "volt",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Capacitance dimension
    Dimension {
        name: "Capacitance",
        symbol: "M⁻¹L⁻²T⁴I²",
        exponents: (-1, -2, 4, 2, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["F"],
                long_name: "farad",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Electric resistance dimension
    Dimension {
        name: "Electric Resistance",
        symbol: "ML²T⁻³I⁻²",
        exponents: (1, 2, -3, -2, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["Ω"],
                long_name: "ohm",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Electric conductance dimension
    Dimension {
        name: "Electric Conductance",
        symbol: "M⁻¹L⁻²T³I²",
        exponents: (-1, -2, 3, 2, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["S"],
                long_name: "siemens",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Inductance dimension
    Dimension {
        name: "Inductance",
        symbol: "ML²T⁻²I⁻²",
        exponents: (1, 2, -2, -2, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["H"],
                long_name: "henry",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Magnetic field dimension
    Dimension {
        name: "Magnetic Field",
        symbol: "MT⁻²I⁻¹",
        exponents: (1, 0, -2, -1, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["T"],
                long_name: "tesla",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Magnetic flux dimension
    Dimension {
        name: "Magnetic Flux",
        symbol: "ML²T⁻²I⁻¹",
        exponents: (1, 2, -2, -1, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["Wb"],
                long_name: "weber",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Illuminance dimension
    Dimension {
        name: "Illuminance",
        symbol: "L⁻²Cd",
        exponents: (0, -2, 0, 0, 0, 0, 1, 0),
        units: &[
            Unit {
                symbols: &["lx"],
                long_name: "lux",
                scale_factors: None, // SI base unit
                conversion_factor: None,
            },
        ],
    },
    
    // Volume Mass Density dimension
    Dimension {
        name: "Volume Mass Density",
        symbol: "ML⁻³",
        exponents: (1, -3, 0, 0, 0, 0, 0, 0),
        units: &[
            // No atomic units for volume mass density - it's a derived dimension
        ],
    },
    
    // Linear Mass Density dimension
    Dimension {
        name: "Linear Mass Density",
        symbol: "ML⁻¹",
        exponents: (1, -1, 0, 0, 0, 0, 0, 0),
        units: &[
            // No atomic units for linear mass density - it's a derived dimension
        ],
    },
    
    // Dynamic Viscosity dimension
    Dimension {
        name: "Dynamic Viscosity",
        symbol: "ML⁻¹T⁻¹",
        exponents: (1, -1, -1, 0, 0, 0, 0, 0),
        units: &[
            // No atomic units for dynamic viscosity - it's a derived dimension
        ],
    },
    
    // Kinematic Viscosity dimension
    Dimension {
        name: "Kinematic Viscosity",
        symbol: "L²T⁻¹",
        exponents: (0, 2, -1, 0, 0, 0, 0, 0),
        units: &[
            Unit {
                symbols: &["St"],
                long_name: "stokes",
                scale_factors: Some((-4, 0, -4, 0)), // 10^-4 (stored in square centimeters per second)
                conversion_factor: None,
            },
        ],
    },
];

/// Look up a dimension by its symbol or name
pub fn lookup_dimension(dimension_name: &str) -> Option<&'static Dimension> {
    DIMENSIONS.iter().find(|dim| {
        dim.symbol == dimension_name || dim.name == dimension_name
    })
}

/// Look up a unit by its symbol across all dimensions
pub fn lookup_unit(unit_symbol: &str) -> Option<(&'static Dimension, &'static Unit)> {
    for dimension in DIMENSIONS {
        if let Some(unit) = dimension.units.iter().find(|unit| unit.symbols.contains(&unit_symbol)) {
            return Some((dimension, unit));
        }
    }
    None
}

/// Look up a dimension by its exponents
pub fn lookup_dimension_by_exponents(exponents: DimensionExponents) -> Option<&'static Dimension> {
    DIMENSIONS.iter().find(|dim| dim.exponents == exponents)
}

/// Get dimension exponents for a unit symbol
pub fn get_dimension_exponents(unit_symbol: &str) -> Option<DimensionExponents> {
    lookup_unit(unit_symbol).map(|(dimension, _)| dimension.exponents)
}

/// Check if a unit symbol represents a compound unit (has non-zero exponents in multiple dimensions)
pub fn is_compound_unit(unit_symbol: &str) -> bool {
    if let Some((dimension, _)) = lookup_unit(unit_symbol) {
        // Count non-zero exponents
        let (m, l, t, c, temp, a, lum, ang) = dimension.exponents;
        let non_zero_count = [m, l, t, c, temp, a, lum, ang].iter().filter(|&&x| x != 0).count();
        non_zero_count > 1
    } else {
        false
    }
}

/// Get all units that have metadata (scale_factors or conversion_factor)
pub fn get_units_with_metadata() -> Vec<(&'static Dimension, &'static Unit)> {
    let mut result = Vec::new();
    for dimension in DIMENSIONS {
        for unit in dimension.units {
            if unit.scale_factors.is_some() || unit.conversion_factor.is_some() {
                result.push((dimension, unit));
            }
        }
    }
    result
}

/// Get all pure SI base units (no metadata)
pub fn get_si_base_units() -> Vec<(&'static Dimension, &'static Unit)> {
    let mut result = Vec::new();
    for dimension in DIMENSIONS {
        for unit in dimension.units {
            if unit.scale_factors.is_none() && unit.conversion_factor.is_none() {
                result.push((dimension, unit));
            }
        }
    }
    result
}