//! Default dimension data for whippyunits
//!
//! This crate provides canonical dimension data that can be shared between
//! the main whippyunits library and the proc macro crate without circular dependencies.

pub mod dimensions;
pub mod base_units;
pub mod util;
pub mod parsing;

// Re-export types from dimensions module
pub use dimensions::{DimensionExponents, Unit, Dimension};
pub use base_units::{BaseUnitInfo, BASE_UNITS, lookup_base_unit};
pub use util::{
    lookup_dimension_by_name, lookup_dimension_by_exponents, lookup_dimension_by_symbol,
    get_all_dimension_names, get_all_dimension_symbols, get_atomic_dimensions, 
    get_atomic_dimension_symbols, get_all_dimensions, lookup_unit_literal, is_prefixed_base_unit,
    get_unit_dimensions, is_valid_unit_literal, get_all_unit_symbols, get_units_by_exponents,
    dimension_exponents_to_unit_expression, scale_type_to_unit_symbol, scale_type_to_actual_unit_symbol, convert_long_name_to_short
};
pub use parsing::{
    parse_unit_with_prefix, is_valid_base_unit, is_valid_unit_literal as is_valid_unit_literal_parsing,
    is_valid_compound_unit, get_prefix_info, get_prefix_scale_factor, UnitParseResult
};

// Legacy type aliases for backward compatibility
pub type DimensionInfo = Dimension;
pub type UnitLiteralInfo = Unit;

/// SI prefix information
#[derive(Debug, Clone)]
pub struct PrefixInfo {
    pub symbol: &'static str,
    pub scale_factor: i16,
    pub long_name: &'static str,
}

/// Legacy DIMENSION_LOOKUP - now replaced by functions in util module
/// This constant is deprecated and will be removed in a future version.
/// Use the lookup functions in the util module instead.
#[deprecated(note = "Use lookup functions in util module instead")]
pub const DIMENSION_LOOKUP: &[DimensionInfo] = &[];

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
        long_name: "deca",
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

/// Legacy UNIT_LITERALS - now replaced by functions in util module
/// This constant is deprecated and will be removed in a future version.
/// Use the lookup functions in the util module instead.
#[deprecated(note = "Use lookup functions in util module instead")]
pub const UNIT_LITERALS: &[UnitLiteralInfo] = &[];

// Legacy lookup functions removed - use functions in util module instead

// Legacy unit literal lookup removed - use functions in util module instead

// Legacy parsing functions removed - use functions in util module instead

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

/// Check if a dimension is atomic (exactly one non-zero exponent and that exponent is 1)
/// 
/// Atomic dimensions are the fundamental base dimensions like mass, length, time, etc.
/// Composite dimensions include both compound units (multiple non-zero exponents) and 
/// derived units (single non-zero exponent that is not 1, like frequency T^-1).
pub fn is_atomic_dimension(exponents: DimensionExponents) -> bool {
    let (m, l, t, c, temp, a, lum, ang) = exponents;
    let non_zero_count = [m, l, t, c, temp, a, lum, ang].iter().filter(|&&x| x != 0).count();
    // Atomic dimensions have exactly one non-zero exponent and that exponent is 1
    non_zero_count == 1 && (m == 1 || l == 1 || t == 1 || c == 1 || temp == 1 || a == 1 || lum == 1 || ang == 1)
}

/// Check if a dimension is composite (not atomic)
/// 
/// Composite dimensions include both compound units (multiple non-zero exponents) and 
/// derived units (single non-zero exponent that is not 1, like frequency T^-1).
pub fn is_composite_dimension(exponents: DimensionExponents) -> bool {
    !is_atomic_dimension(exponents)
}

// Legacy unit conversion functions removed - use functions in util module instead

// Legacy dimension expression function removed - use functions in util module instead

// Legacy scale type parsing function removed - use functions in util module instead
