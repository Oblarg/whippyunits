use crate::alloc::{String, ToString, Vec, format};
use whippyunits_core::{Dimension, Unit};

/// Represents a parsed format specifier for unit conversion
#[derive(Debug, Clone, PartialEq)]
pub struct UnitFormatSpecifier {
    pub target_unit: String,
    pub precision: Option<usize>,
    pub width: Option<usize>,
    pub alignment: Option<FormatAlignment>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormatAlignment {
    Left,
    Right,
    Center,
}

/// Parse a format specifier string like "km", "kilometers", "km:2", etc.
pub fn parse_unit_format_specifier(spec: &str) -> Result<UnitFormatSpecifier, String> {
    let parts: Vec<&str> = spec.split(':').collect();

    if parts.is_empty() || parts[0].is_empty() {
        return Err("Empty format specifier".to_string());
    }

    let target_unit = parts[0].to_string();
    let precision = parts.get(1).and_then(|p| p.parse().ok());
    let width = parts.get(2).and_then(|w| w.parse().ok());

    Ok(UnitFormatSpecifier {
        target_unit,
        precision,
        width,
        alignment: None, // Could be extended to support alignment specifiers
    })
}

/// Format a value with the specified unit format using the centralized unit data
pub fn format_with_unit(value: f64, spec: &UnitFormatSpecifier) -> Result<String, String> {
    let target_unit_info = lookup_unit_literal_direct(&spec.target_unit)
        .ok_or_else(|| format!("Unknown unit: {}", spec.target_unit))?;

    // Format the value with precision if specified
    let formatted_value = if let Some(precision) = spec.precision {
        format!("{:.precision$}", value, precision = precision)
    } else {
        format!("{}", value)
    };

    // Apply width formatting if specified
    let final_value = if let Some(width) = spec.width {
        format!("{:>width$}", formatted_value, width = width)
    } else {
        formatted_value
    };

    // For prefixed units (like "km"), use the original unit symbol for display
    // For base units (like "gram"), use the symbol from the unit info
    let display_unit = if spec.target_unit.len() > target_unit_info.1.symbols[0].len() {
        // This might be a long name prefixed unit - try to convert to short form
        if let Some(short_form) = convert_long_name_to_short_direct(&spec.target_unit) {
            short_form
        } else {
            // This is a short prefixed unit (like "km") - use the original unit symbol
            spec.target_unit.clone()
        }
    } else if spec.target_unit == target_unit_info.1.symbols[0] {
        // This is a base unit symbol (like "g") - use the symbol from the unit info
        target_unit_info.1.symbols[0].to_string()
    } else {
        // This is a base unit long name (like "gram") - use the symbol from the unit info
        target_unit_info.1.symbols[0].to_string()
    };

    Ok(format!("{} {}", final_value, display_unit))
}

// Helper functions that replace api_helpers functions with direct whippyunits-core calls

/// Look up a unit literal (like "min", "h", "g", "m", "s", etc.) in the dimensions data
fn lookup_unit_literal_direct(unit_name: &str) -> Option<(&'static Dimension, &'static Unit)> {
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

/// Convert a long unit name to its short symbol form
/// For example, "kilometer" -> "km", "gram" -> "g"
fn convert_long_name_to_short_direct(long_name: &str) -> Option<String> {
    // First try to find by long name
    if let Some((unit, _dimension)) = Dimension::find_unit_by_name(long_name) {
        return Some(String::from(unit.symbols[0]));
    }

    // Try to handle prefixed units by stripping prefix and finding base unit
    if let Some((prefix, base)) = whippyunits_core::SiPrefix::strip_any_prefix_name(long_name) {
        if let Some((unit, _dimension)) = Dimension::find_unit_by_name(base) {
            return Some(format!("{}{}", prefix.symbol(), unit.symbols[0]));
        }
    }

    None
}
