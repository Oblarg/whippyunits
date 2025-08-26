//! Centralized dimensional metadata for WhippyUnits
//! 
//! This module serves as the single source of truth for all dimensional metadata,
//! including scale values, unit names, and dimension mappings.

use core::marker::ConstParamTy;

// ============================================================================
// Core Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum Dimension {
    Length,
    Mass,
    Time,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitMetadata {
    /// The scale value used in const generics (e.g., -1 for millimeter, 0 for meter)
    pub scale_value: isize,
    /// Short unit name for display (e.g., "mm", "m", "km")
    pub short_name: &'static str,
    /// Long unit name for debug output (e.g., "millimeter", "meter", "kilometer")
    pub long_name: &'static str,
    /// Which dimension this unit belongs to
    pub dimension: Dimension,
    /// Whether this unit is the base unit for its dimension (used for conversions)
    pub is_base_unit: bool,
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
        dimension: Dimension::Length,
        is_base_unit: false,
        exponential_scale: 1000, // 1000^scale_value
    },
    UnitMetadata {
        scale_value: 0,
        short_name: "m",
        long_name: "meter",
        dimension: Dimension::Length,
        is_base_unit: true,
        exponential_scale: 1000, // 1000^scale_value
    },
    UnitMetadata {
        scale_value: 1,
        short_name: "km",
        long_name: "kilometer",
        dimension: Dimension::Length,
        is_base_unit: false,
        exponential_scale: 1000, // 1000^scale_value
    },
];

/// All mass units with their metadata
pub const MASS_UNITS: &[UnitMetadata] = &[
    UnitMetadata {
        scale_value: -1,
        short_name: "mg",
        long_name: "milligram",
        dimension: Dimension::Mass,
        is_base_unit: false,
        exponential_scale: 1000, // 1000^scale_value
    },
    UnitMetadata {
        scale_value: 0,
        short_name: "g",
        long_name: "gram",
        dimension: Dimension::Mass,
        is_base_unit: false,
        exponential_scale: 1000, // 1000^scale_value
    },
    UnitMetadata {
        scale_value: 1,
        short_name: "kg",
        long_name: "kilogram",
        dimension: Dimension::Mass,
        is_base_unit: true,
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
        dimension: Dimension::Time,
        is_base_unit: false,
        exponential_scales: [(2, -3), (3, 0), (5, -3)], // [(prime_factor, exponent), ...]
    },
    TimeUnitMetadata {
        scale_order: 0,
        p2: 0,
        p3: 0,
        p5: 0,
        short_name: "s",
        long_name: "second",
        dimension: Dimension::Time,
        is_base_unit: true,
        exponential_scales: [(2, 0), (3, 0), (5, 0)], // [(prime_factor, exponent), ...]
    },
    TimeUnitMetadata {
        scale_order: 1,
        p2: 2,
        p3: 1,
        p5: 1,
        short_name: "min",
        long_name: "minute",
        dimension: Dimension::Time,
        is_base_unit: false,
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
    pub dimension: Dimension,
    pub is_base_unit: bool,
    /// The exponential scales for time units, mapping prime factors to their exponents
    pub exponential_scales: [(isize, isize); 3], // [(prime_factor, exponent), ...]
}

// ============================================================================
// Lookup Functions
// ============================================================================

/// Find unit metadata by scale value and dimension
pub const fn find_unit_by_scale(scale: isize, dimension: Dimension) -> Option<&'static UnitMetadata> {
    match dimension {
        Dimension::Length => find_length_unit(scale),
        Dimension::Mass => find_mass_unit(scale),
        Dimension::Time => None, // Time units need special handling
    }
}

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
// Conversion Helper Functions
// ============================================================================

/// Get conversion factor for length unit
pub const fn length_conversion_factor(from_scale: isize, to_scale: isize, value: f64) -> f64 {
    let from_factor = match find_length_unit(from_scale) {
        Some(unit) => unit.conversion_factor,
        None => 1.0,
    };
    let to_factor = match find_length_unit(to_scale) {
        Some(unit) => unit.conversion_factor,
        None => 1.0,
    };
    value * from_factor / to_factor
}

/// Get conversion factor for mass unit
pub const fn mass_conversion_factor(from_scale: isize, to_scale: isize, value: f64) -> f64 {
    let from_factor = match find_mass_unit(from_scale) {
        Some(unit) => unit.conversion_factor,
        None => 1.0,
    };
    let to_factor = match find_mass_unit(to_scale) {
        Some(unit) => unit.conversion_factor,
        None => 1.0,
    };
    value * from_factor / to_factor
}

/// Get conversion factor for time unit
pub const fn time_conversion_factor(
    from_p2: isize,
    from_p3: isize,
    from_p5: isize,
    to_p2: isize,
    to_p3: isize,
    to_p5: isize,
    value: f64,
) -> f64 {
    // This would need to be implemented based on your time conversion logic
    // For now, returning the value unchanged
    value
}

// ============================================================================
// Constants Generated from Metadata
// ============================================================================

// These constants can be generated from the metadata arrays above
// They maintain backward compatibility with existing code

pub const MILLIMETER_SCALE: isize = -1;
pub const METER_SCALE: isize = 0;
pub const KILOMETER_SCALE: isize = 1;
pub const LENGTH_UNUSED: isize = isize::MAX;

pub const MILLIGRAM_SCALE: isize = -1;
pub const GRAM_SCALE: isize = 0;
pub const KILOGRAM_SCALE: isize = 1;
pub const MASS_UNUSED: isize = isize::MAX;

pub const MILLISECOND_SCALE_ORDER: isize = -1;
pub const MILLISECOND_SCALE_P2: isize = -3;
pub const MILLISECOND_SCALE_P3: isize = 0;
pub const MILLISECOND_SCALE_P5: isize = -3;
pub const SECOND_SCALE_ORDER: isize = 0;
pub const SECOND_SCALE_P2: isize = 0;
pub const SECOND_SCALE_P3: isize = 0;
pub const SECOND_SCALE_P5: isize = 0;
pub const MINUTE_SCALE_ORDER: isize = 1;
pub const MINUTE_SCALE_P2: isize = 2;
pub const MINUTE_SCALE_P3: isize = 1;
pub const MINUTE_SCALE_P5: isize = 1;
pub const TIME_UNUSED: isize = isize::MAX;

// ============================================================================
// Validation Functions
// ============================================================================

/// Validate that all metadata is consistent
pub const fn validate_metadata() -> bool {
    // Check that all scale values are unique within each dimension
    let mut length_scales = [false; 3]; // -1, 0, 1
    let mut mass_scales = [false; 3];   // -1, 0, 1
    let mut time_scales = [false; 3];   // -1, 0, 1
    
    // Validate length units
    for unit in LENGTH_UNITS {
        let idx = (unit.scale_value + 1) as usize;
        if idx < 3 {
            length_scales[idx] = true;
        }
    }
    
    // Validate mass units
    for unit in MASS_UNITS {
        let idx = (unit.scale_value + 1) as usize;
        if idx < 3 {
            mass_scales[idx] = true;
        }
    }
    
    // Validate time units
    for unit in TIME_UNITS {
        let idx = (unit.scale_order + 1) as usize;
        if idx < 3 {
            time_scales[idx] = true;
        }
    }
    
    // All scale values should be present
    length_scales.iter().all(|&x| x) && 
    mass_scales.iter().all(|&x| x) && 
    time_scales.iter().all(|&x| x)
}

// Compile-time validation
const _: () = assert!(validate_metadata(), "Dimensional metadata validation failed");
