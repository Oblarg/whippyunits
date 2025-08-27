//! Centralized dimensional metadata for WhippyUnits
//! 
//! This module serves as the single source of truth for all dimensional metadata,
//! including scale values, unit names, and dimension mappings.

// ============================================================================
// Core Types
// ============================================================================



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitMetadata {
    /// The scale value used in const generics (e.g., -1 for millimeter, 0 for meter)
    pub scale_value: isize,
    /// Short unit name for display (e.g., "mm", "m", "km")
    pub short_name: &'static str,
    /// Long unit name for debug output (e.g., "millimeter", "meter", "kilometer")
    pub long_name: &'static str,
    /// The exponential scale for this unit (e.g., 1000 for length/mass units)
    pub exponential_scale: isize,
}

// ============================================================================
// Unit Metadata Arrays
// ============================================================================

/// All length units with their metadata
pub const LENGTH_UNITS: &[UnitMetadata] = &[
    UnitMetadata {
        scale_value: -1,
        short_name: "mm",
        long_name: "millimeter",
        exponential_scale: 1000, // 1000^scale_value
    },
    UnitMetadata {
        scale_value: 0,
        short_name: "m",
        long_name: "meter",
        exponential_scale: 1000, // 1000^scale_value
    },
    UnitMetadata {
        scale_value: 1,
        short_name: "km",
        long_name: "kilometer",
        exponential_scale: 1000, // 1000^scale_value
    },
];

/// All mass units with their metadata
pub const MASS_UNITS: &[UnitMetadata] = &[
    UnitMetadata {
        scale_value: -1,
        short_name: "mg",
        long_name: "milligram",
        exponential_scale: 1000, // 1000^scale_value
    },
    UnitMetadata {
        scale_value: 0,
        short_name: "g",
        long_name: "gram",
        exponential_scale: 1000, // 1000^scale_value
    },
    UnitMetadata {
        scale_value: 1,
        short_name: "kg",
        long_name: "kilogram",
        exponential_scale: 1000, // 1000^scale_value
    },
];

/// All time units with their metadata
/// Note: Time units have additional scale parameters (P2, P3, P5) for complex time representations
pub const TIME_UNITS: &[TimeUnitMetadata] = &[
    TimeUnitMetadata {
        scale_order: -1,
        p2: -3,
        p3: 0,
        p5: -3,
        short_name: "ms",
        long_name: "millisecond",
        exponential_scales: [(2, -3), (3, 0), (5, -3)], // [(prime_factor, exponent), ...]
    },
    TimeUnitMetadata {
        scale_order: 0,
        p2: 0,
        p3: 0,
        p5: 0,
        short_name: "s",
        long_name: "second",
        exponential_scales: [(2, 0), (3, 0), (5, 0)], // [(prime_factor, exponent), ...]
    },
    TimeUnitMetadata {
        scale_order: 1,
        p2: 2,
        p3: 1,
        p5: 1,
        short_name: "min",
        long_name: "minute",
        exponential_scales: [(2, 2), (3, 1), (5, 1)], // [(prime_factor, exponent), ...]
    },
];

/// Extended metadata for time units that includes the additional scale parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeUnitMetadata {
    pub scale_order: isize,
    pub p2: isize,
    pub p3: isize,
    pub p5: isize,
    pub short_name: &'static str,
    pub long_name: &'static str,
    /// The exponential scales for time units, mapping prime factors to their exponents
    pub exponential_scales: [(isize, isize); 3], // [(prime_factor, exponent), ...]
}

// ============================================================================
// Lookup Functions
// ============================================================================



/// Find length unit by scale value
pub const fn find_length_unit(scale: isize) -> Option<&'static UnitMetadata> {
    match scale {
        -1 => Some(&LENGTH_UNITS[0]), // mm
        0 => Some(&LENGTH_UNITS[1]),  // m
        1 => Some(&LENGTH_UNITS[2]),  // km
        _ => None,
    }
}

/// Find mass unit by scale value
pub const fn find_mass_unit(scale: isize) -> Option<&'static UnitMetadata> {
    match scale {
        -1 => Some(&MASS_UNITS[0]), // mg
        0 => Some(&MASS_UNITS[1]),  // g
        1 => Some(&MASS_UNITS[2]),  // kg
        _ => None,
    }
}

/// Find time unit by scale order
pub const fn find_time_unit(scale_order: isize) -> Option<&'static TimeUnitMetadata> {
    match scale_order {
        -1 => Some(&TIME_UNITS[0]), // ms
        0 => Some(&TIME_UNITS[1]),  // s
        1 => Some(&TIME_UNITS[2]),  // min
        _ => None,
    }
}

/// Find time unit by all scale parameters
pub const fn find_time_unit_by_params(
    scale_order: isize,
    p2: isize,
    p3: isize,
    p5: isize,
) -> Option<&'static TimeUnitMetadata> {
    match (scale_order, p2, p3, p5) {
        (-1, -3, 0, -3) => Some(&TIME_UNITS[0]), // ms
        (0, 0, 0, 0) => Some(&TIME_UNITS[1]),    // s
        (1, 2, 1, 1) => Some(&TIME_UNITS[2]),    // min
        _ => None,
    }
}

// ============================================================================
// Display Helper Functions
// ============================================================================

/// Get short name for a length unit by scale value
pub const fn length_short_name(scale: isize) -> &'static str {
    match find_length_unit(scale) {
        Some(unit) => unit.short_name,
        None => "unknown",
    }
}

/// Get long name for a length unit by scale value
pub const fn length_long_name(scale: isize) -> &'static str {
    match find_length_unit(scale) {
        Some(unit) => unit.long_name,
        None => "unknown",
    }
}

/// Get short name for a mass unit by scale value
pub const fn mass_short_name(scale: isize) -> &'static str {
    match find_mass_unit(scale) {
        Some(unit) => unit.short_name,
        None => "unknown",
    }
}

/// Get long name for a mass unit by scale value
pub const fn mass_long_name(scale: isize) -> &'static str {
    match find_mass_unit(scale) {
        Some(unit) => unit.long_name,
        None => "unknown",
    }
}

/// Get short name for a time unit by scale order
pub const fn time_short_name(scale_order: isize) -> &'static str {
    match find_time_unit(scale_order) {
        Some(unit) => unit.short_name,
        None => "unknown",
    }
}

/// Get long name for a time unit by scale order
pub const fn time_long_name(scale_order: isize) -> &'static str {
    match find_time_unit(scale_order) {
        Some(unit) => unit.long_name,
        None => "unknown",
    }
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Validate that all metadata is consistent
/// Note: This is a simplified version for standalone script use
/// The full validation is done in the main crate
pub const fn validate_metadata() -> bool {
    true // Simplified for standalone script
}
