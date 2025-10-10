//! Centralized unit parsing logic for whippyunits
//!
//! This module provides unified parsing functions that eliminate code duplication
//! across different parts of the whippyunits codebase.

use crate::{BASE_UNITS, SI_PREFIXES};
use crate::util::lookup_unit_literal;

/// Parse a unit name to extract prefix and base unit
/// 
/// Returns (prefix, base_unit) where prefix is None if no prefix is found.
/// This is the canonical parsing function used throughout the codebase.
/// 
/// # Examples
/// ```
/// use whippyunits_default_dimensions::parsing::parse_unit_with_prefix;
/// 
/// assert_eq!(parse_unit_with_prefix("km"), (Some("k"), "m"));
/// assert_eq!(parse_unit_with_prefix("m"), (None, "m"));
/// assert_eq!(parse_unit_with_prefix("min"), (None, "min")); // Exact match, not "m" + "in"
/// ```
pub fn parse_unit_with_prefix(unit_name: &str) -> (Option<&'static str>, &str) {
    // Use the existing is_prefixed_base_unit function which already handles this logic correctly
    if let Some((base_symbol, prefix_symbol)) = crate::util::is_prefixed_base_unit(unit_name) {
        return (Some(prefix_symbol), base_symbol);
    }

    // If not a prefixed unit, return as-is
    (None, unit_name)
}

/// Check if a string is a valid base unit
/// 
/// This checks against the BASE_UNITS array for atomic SI base units.
pub fn is_valid_base_unit(unit: &str) -> bool {
    BASE_UNITS.iter().any(|info| info.symbol == unit)
}

/// Check if a string is a valid unit literal (including compound units)
/// 
/// This checks against the DIMENSIONS data for all known units, including
/// compound units like J, W, N, etc.
pub fn is_valid_unit_literal(unit: &str) -> bool {
    lookup_unit_literal(unit).is_some()
}

/// Check if a string is a valid compound unit (has any non-unity dimension exponents)
/// 
/// Compound units are derived units like J (joule), W (watt), N (newton), Hz (hertz), etc.
/// Any unit with non-unity exponents (including negative exponents) is considered compound.
pub fn is_valid_compound_unit(unit: &str) -> bool {
    if let Some((dimension, _)) = lookup_unit_literal(unit) {
        let (m, l, t, c, temp, a, lum, ang) = dimension.exponents;
        // Check if any exponent is not 0 or 1 (i.e., has non-unity exponents)
        return [m, l, t, c, temp, a, lum, ang].iter().any(|&x| x != 0 && x != 1);
    }
    false
}

/// Get prefix information by symbol
/// 
/// Returns the PrefixInfo for the given prefix symbol, or None if not found.
pub fn get_prefix_info(prefix: &str) -> Option<&'static crate::PrefixInfo> {
    SI_PREFIXES.iter().find(|info| info.symbol == prefix)
}

/// Get prefix scale factor (power of 10)
/// 
/// Returns the scale factor for the given prefix symbol, or 0 if not found.
pub fn get_prefix_scale_factor(prefix: &str) -> i16 {
    get_prefix_info(prefix).map(|info| info.scale_factor).unwrap_or(0)
}

/// Parse a unit name and return detailed information
/// 
/// This is a more comprehensive parsing function that returns structured information
/// about the parsed unit, including whether it's a base unit, compound unit, or prefixed unit.
#[derive(Debug, Clone, PartialEq)]
pub struct UnitParseResult {
    pub prefix: Option<&'static str>,
    pub base_unit: String,
    pub is_base_unit: bool,
    pub is_compound_unit: bool,
    pub is_prefixed: bool,
}

impl UnitParseResult {
    /// Parse a unit name and return detailed information
    pub fn parse(unit_name: &str) -> Self {
        let (prefix, base_unit) = parse_unit_with_prefix(unit_name);
        let is_base_unit = is_valid_base_unit(&base_unit);
        let is_compound_unit = is_valid_compound_unit(&base_unit);
        let is_prefixed = prefix.is_some();

        Self {
            prefix,
            base_unit: base_unit.to_string(),
            is_base_unit,
            is_compound_unit,
            is_prefixed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unit_with_prefix() {
        // Base units
        assert_eq!(parse_unit_with_prefix("m"), (None, "m"));
        assert_eq!(parse_unit_with_prefix("g"), (None, "g"));
        assert_eq!(parse_unit_with_prefix("s"), (None, "s"));

        // Prefixed units
        assert_eq!(parse_unit_with_prefix("km"), (Some("k"), "m"));
        assert_eq!(parse_unit_with_prefix("cm"), (Some("c"), "m"));
        assert_eq!(parse_unit_with_prefix("mm"), (Some("m"), "m"));
        assert_eq!(parse_unit_with_prefix("kg"), (Some("k"), "g"));

        // Compound units (should not be parsed as prefixed)
        assert_eq!(parse_unit_with_prefix("J"), (None, "J"));
        assert_eq!(parse_unit_with_prefix("W"), (None, "W"));
        assert_eq!(parse_unit_with_prefix("N"), (None, "N"));

        // Prefixed compound units
        assert_eq!(parse_unit_with_prefix("kJ"), (Some("k"), "J"));
        assert_eq!(parse_unit_with_prefix("MW"), (Some("M"), "W"));

        // Special cases - exact matches should not be parsed as prefixed
        assert_eq!(parse_unit_with_prefix("min"), (None, "min"));
        assert_eq!(parse_unit_with_prefix("h"), (None, "h"));
        assert_eq!(parse_unit_with_prefix("hr"), (None, "hr"));
    }

    #[test]
    fn test_is_valid_base_unit() {
        assert!(is_valid_base_unit("m"));
        assert!(is_valid_base_unit("g"));
        assert!(is_valid_base_unit("s"));
        assert!(is_valid_base_unit("A"));
        assert!(is_valid_base_unit("K"));
        assert!(is_valid_base_unit("mol"));
        assert!(is_valid_base_unit("cd"));
        assert!(is_valid_base_unit("rad"));

        assert!(!is_valid_base_unit("J"));
        assert!(!is_valid_base_unit("W"));
        assert!(!is_valid_base_unit("N"));
        assert!(!is_valid_base_unit("min"));
        assert!(!is_valid_base_unit("h"));
    }

    #[test]
    fn test_is_valid_unit_literal() {
        assert!(is_valid_unit_literal("m"));
        assert!(is_valid_unit_literal("g"));
        assert!(is_valid_unit_literal("J"));
        assert!(is_valid_unit_literal("W"));
        assert!(is_valid_unit_literal("N"));
        assert!(is_valid_unit_literal("min"));
        assert!(is_valid_unit_literal("h"));
        assert!(is_valid_unit_literal("hr"));

        assert!(!is_valid_unit_literal("xyz"));
        assert!(!is_valid_unit_literal("unknown"));
    }

    #[test]
    fn test_is_valid_compound_unit() {
        assert!(is_valid_compound_unit("J"));
        assert!(is_valid_compound_unit("W"));
        assert!(is_valid_compound_unit("N"));
        assert!(is_valid_compound_unit("Pa"));
        assert!(is_valid_compound_unit("Hz"));

        assert!(!is_valid_compound_unit("m"));
        assert!(!is_valid_compound_unit("g"));
        assert!(!is_valid_compound_unit("s"));
        assert!(!is_valid_compound_unit("min"));
        assert!(!is_valid_compound_unit("h"));
    }

    #[test]
    fn test_get_prefix_info() {
        let kilo = get_prefix_info("k");
        assert!(kilo.is_some());
        assert_eq!(kilo.unwrap().symbol, "k");
        assert_eq!(kilo.unwrap().scale_factor, 3);

        let milli = get_prefix_info("m");
        assert!(milli.is_some());
        assert_eq!(milli.unwrap().symbol, "m");
        assert_eq!(milli.unwrap().scale_factor, -3);

        assert!(get_prefix_info("xyz").is_none());
    }

    #[test]
    fn test_get_prefix_scale_factor() {
        assert_eq!(get_prefix_scale_factor("k"), 3);
        assert_eq!(get_prefix_scale_factor("m"), -3);
        assert_eq!(get_prefix_scale_factor("c"), -2);
        assert_eq!(get_prefix_scale_factor("M"), 6);
        assert_eq!(get_prefix_scale_factor("xyz"), 0);
    }

    #[test]
    fn test_unit_parse_result() {
        let result = UnitParseResult::parse("km");
        assert_eq!(result.prefix, Some("k"));
        assert_eq!(result.base_unit, "m");
        assert!(result.is_base_unit);
        assert!(!result.is_compound_unit);
        assert!(result.is_prefixed);

        let result = UnitParseResult::parse("J");
        assert_eq!(result.prefix, None);
        assert_eq!(result.base_unit, "J");
        assert!(!result.is_base_unit);
        assert!(result.is_compound_unit);
        assert!(!result.is_prefixed);

        let result = UnitParseResult::parse("kJ");
        assert_eq!(result.prefix, Some("k"));
        assert_eq!(result.base_unit, "J");
        assert!(!result.is_base_unit);
        assert!(result.is_compound_unit);
        assert!(result.is_prefixed);
    }
}
