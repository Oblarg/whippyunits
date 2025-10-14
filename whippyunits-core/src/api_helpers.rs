use crate::{Dimension, SiPrefix, Unit, dimension_exponents::DynDimensionExponents};
use alloc::{format, string::String, vec::Vec};

/// Check if a unit name is a prefixed base unit (like kg, kW, mm, etc.)
/// Returns Some((base_unit, prefix)) if it is, None otherwise
pub fn is_prefixed_base_unit(unit_name: &str) -> Option<(String, String)> {
    // Try to strip any prefix from the unit name
    if let Some((prefix, base)) = SiPrefix::strip_any_prefix_symbol(unit_name) {
        // Check if the base unit exists
        if Dimension::find_unit_by_symbol(base).is_some() {
            return Some((String::from(base), String::from(prefix.symbol())));
        }
    }
    
    // Also try stripping prefix from name (not just symbol)
    if let Some((prefix, base)) = SiPrefix::strip_any_prefix_name(unit_name) {
        // Check if the base unit exists by name
        if Dimension::find_unit_by_name(base).is_some() {
            return Some((String::from(base), String::from(prefix.symbol())));
        }
    }
    
    None
}

/// Parse a unit name to extract prefix and base unit
/// Returns (prefix_option, base_unit_name)
pub fn parse_unit_with_prefix(unit_name: &str) -> (Option<&'static SiPrefix>, String) {
    // Try to strip any prefix from the unit name
    if let Some((prefix, base)) = SiPrefix::strip_any_prefix_symbol(unit_name) {
        // Check if the base unit exists
        if Dimension::find_unit_by_symbol(base).is_some() {
            return (Some(prefix), String::from(base));
        }
    }
    
    // Also try stripping prefix from name (not just symbol)
    if let Some((prefix, base)) = SiPrefix::strip_any_prefix_name(unit_name) {
        // Check if the base unit exists by name
        if Dimension::find_unit_by_name(base).is_some() {
            return (Some(prefix), String::from(base));
        }
    }
    
    (None, String::from(unit_name))
}

/// Look up a unit literal (like "min", "h", "g", "m", "s", etc.) in the dimensions data
pub fn lookup_unit_literal(unit_name: &str) -> Option<(&'static Dimension, &'static Unit)> {
    // First try to find by symbol
    if let Some((unit, dimension)) = Dimension::find_unit_by_symbol(unit_name) {
        return Some((dimension, unit));
    }
    
    // Then try to find by name
    if let Some((unit, dimension)) = Dimension::find_unit_by_name(unit_name) {
        return Some((dimension, unit));
    }
    
    None
}

/// Look up SI prefix by symbol
pub fn lookup_si_prefix(prefix_symbol: &str) -> Option<&'static SiPrefix> {
    SiPrefix::from_symbol(prefix_symbol)
}

/// Convert dynamic exponents tuple to static tuple
pub fn dyn_exponents_to_tuple(exponents: DynDimensionExponents) -> (i16, i16, i16, i16, i16, i16, i16, i16) {
    (
        exponents.0[0], // mass
        exponents.0[1], // length
        exponents.0[2], // time
        exponents.0[3], // current
        exponents.0[4], // temperature
        exponents.0[5], // amount
        exponents.0[6], // luminous_intensity
        exponents.0[7], // angle
    )
}

/// Look up dimension by name
pub fn lookup_dimension_by_name(name: &str) -> Option<&'static Dimension> {
    Dimension::ALL.iter().find(|dim| dim.name == name)
}

/// Look up dimension by symbol
pub fn lookup_dimension_by_symbol(symbol: &str) -> Option<&'static Dimension> {
    Dimension::ALL.iter().find(|dim| dim.symbol == symbol)
}

/// Get all dimension names
pub fn get_all_dimension_names() -> Vec<&'static str> {
    Dimension::ALL.iter().map(|dim| dim.name).collect()
}

/// Get all dimension symbols
pub fn get_all_dimension_symbols() -> Vec<&'static str> {
    Dimension::ALL.iter().map(|dim| dim.symbol).collect()
}

/// Get atomic dimensions
pub fn get_atomic_dimensions() -> &'static [Dimension] {
    &Dimension::BASIS
}

/// Get all dimensions
pub fn get_all_dimensions() -> &'static [Dimension] {
    Dimension::ALL
}

/// Convert a scale type name to the actual unit symbol
/// This is used for mapping scale types like "Kilogram" to "kg"
pub fn scale_type_to_actual_unit_symbol(scale_type: &str) -> Option<String> {
    use crate::Unit;
    
    // Try to find a unit that matches the scale type name
    for unit in Unit::BASES.iter() {
        if unit.name == scale_type {
            return Some(String::from(unit.symbols[0]));
        }
    }
    
    // Try to find in all dimensions
    for dimension in Dimension::ALL {
        for unit in dimension.units {
            if unit.name == scale_type {
                return Some(String::from(unit.symbols[0]));
            }
        }
    }
    
    // Try to parse as a prefixed scale type name (like "Kilogram" -> "kilo" + "gram")
    if let Some((prefix, base)) = SiPrefix::strip_any_prefix_name(scale_type) {
        // Find the base unit
        for unit in Unit::BASES.iter() {
            if unit.name == base {
                return Some(format!("{}{}", prefix.symbol(), unit.symbols[0]));
            }
        }
        
        // Try to find in all dimensions
        for dimension in Dimension::ALL {
            for unit in dimension.units {
                if unit.name == base {
                    return Some(format!("{}{}", prefix.symbol(), unit.symbols[0]));
                }
            }
        }
    }
    
    None
}

/// Get units by their dimension exponents
pub fn get_units_by_exponents(exponents: (i16, i16, i16, i16, i16, i16, i16, i16)) -> Vec<(&'static Dimension, &'static Unit)> {
    let mut result = Vec::new();
    
    for dimension in Dimension::ALL {
        if dimension.exponents.0 == [exponents.0, exponents.1, exponents.2, exponents.3, exponents.4, exponents.5, exponents.6, exponents.7] {
            for unit in dimension.units {
                result.push((dimension, unit));
            }
        }
    }
    
    result
}

/// Convert DynDimensionExponents to tuple format
pub fn dyn_exponents_to_tuple_format(exponents: DynDimensionExponents) -> (i16, i16, i16, i16, i16, i16, i16, i16) {
    (
        exponents.0[0], // mass
        exponents.0[1], // length
        exponents.0[2], // time
        exponents.0[3], // current
        exponents.0[4], // temperature
        exponents.0[5], // amount
        exponents.0[6], // luminous_intensity
        exponents.0[7], // angle
    )
}

/// Convert a long unit name to its short symbol form
/// For example, "kilometer" -> "km", "gram" -> "g"
pub fn convert_long_name_to_short(long_name: &str) -> Option<String> {
    // First try to find by long name
    if let Some((unit, _dimension)) = Dimension::find_unit_by_name(long_name) {
        return Some(String::from(unit.symbols[0]));
    }
    
    // Try to handle prefixed units by stripping prefix and finding base unit
    if let Some((prefix, base)) = SiPrefix::strip_any_prefix_name(long_name) {
        if let Some((unit, _dimension)) = Dimension::find_unit_by_name(base) {
            return Some(format!("{}{}", prefix.symbol(), unit.symbols[0]));
        }
    }
    
    None
}

/// Get the symbols for all atomic dimensions (SI base quantities)
/// Returns symbols in order: Mass, Length, Time, Current, Temperature, Amount, Luminosity, Angle
pub fn get_atomic_dimension_symbols() -> Vec<&'static str> {
    Dimension::BASIS.iter().map(|dim| dim.symbol).collect()
}

/// Look up a dimension by its exponents
pub fn lookup_dimension_by_exponents(exponents: (i16, i16, i16, i16, i16, i16, i16, i16)) -> Option<&'static Dimension> {
    let dyn_exponents = DynDimensionExponents([exponents.0, exponents.1, exponents.2, exponents.3, exponents.4, exponents.5, exponents.6, exponents.7]);
    Dimension::find_dimension_by_exponents(dyn_exponents)
}

/// Get unit dimensions for a unit literal (alias for get_quantity_dimensions)
pub fn get_unit_dimensions(unit_literal: &str) -> Option<(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16)> {
    // This is a simplified version that just returns the base dimensions
    // The full implementation would need to handle prefixes and conversion factors
    if let Some((_dimension, unit)) = lookup_unit_literal(unit_literal) {
        let (m, l, t, c, temp, a, lum, ang) = (
            unit.exponents.0[0], // mass
            unit.exponents.0[1], // length
            unit.exponents.0[2], // time
            unit.exponents.0[3], // current
            unit.exponents.0[4], // temperature
            unit.exponents.0[5], // amount
            unit.exponents.0[6], // luminosity
            unit.exponents.0[7], // angle
        );
        let (p2, p3, p5, pi) = (unit.scale.0[0], unit.scale.0[1], unit.scale.0[2], unit.scale.0[3]);
        Some((m, l, t, c, temp, a, lum, ang, p2, p3, p5, pi))
    } else {
        None
    }
}

/// Get the scale factor for a prefix
pub fn get_prefix_scale_factor(prefix: &SiPrefix) -> f64 {
    prefix.factor_log10() as f64
}
