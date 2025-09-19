use whippyunits_default_dimensions::lookup_unit_literal;

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
pub fn format_with_unit(
    value: f64,
    spec: &UnitFormatSpecifier,
) -> Result<String, String> {
    let target_unit_info = lookup_unit_literal(&spec.target_unit)
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
    
    Ok(format!("{} {}", final_value, target_unit_info.symbol))
}
