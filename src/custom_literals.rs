//! Custom literals for whippyunits using the culit crate
//!
//! This module provides custom literals like `100m_f64`, `5.5kg_f64`, etc.
//! The literals are generated using canonical data from default-dimensions.

extern crate culit;
pub use culit::culit;

// The custom_literal module is defined at the root level in lib.rs

/// Get all unit symbols that should have custom literals
/// This uses the canonical data from default-dimensions
pub fn get_unit_symbols_for_literals() -> Vec<String> {
    let mut symbols = Vec::new();

    // Use the canonical data from default-dimensions
    // First, add all base units
    for base_unit in whippyunits_default_dimensions::BASE_UNITS {
        if base_unit.symbol != "dimensionless" {
            symbols.push(base_unit.symbol.to_string());
        }
    }

    // Generate common prefixed units by combining SI prefixes with base units
    // Only generate the most commonly used combinations to avoid bloat
    let common_base_units = ["g", "m", "s", "A", "K", "mol", "cd", "rad"];
    let common_prefixes = [
        "k", "m", "u", "n", "p", "f", "a", "z", "y", "c", "d", "da", "h", "M", "G", "T", "P", "E",
        "Z", "Y",
    ];

    for base_unit in &common_base_units {
        for prefix in &common_prefixes {
            let prefixed_unit = format!("{}{}", prefix, base_unit);
            symbols.push(prefixed_unit);
        }
    }

    symbols.sort();
    symbols.dedup();
    symbols
}

/// Get all type suffixes that should be supported
pub fn get_type_suffixes() -> Vec<&'static str> {
    vec!["f64", "f32", "i32", "i64", "u32", "u64"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unit_symbols() {
        let symbols = get_unit_symbols_for_literals();
        assert!(symbols.contains(&"m".to_string()));
        assert!(symbols.contains(&"kg".to_string()));
        assert!(symbols.contains(&"s".to_string()));
        assert!(symbols.contains(&"mm".to_string()));
        assert!(symbols.contains(&"km".to_string()));
    }

    #[test]
    fn test_get_type_suffixes() {
        let suffixes = get_type_suffixes();
        assert!(suffixes.contains(&"f64"));
        assert!(suffixes.contains(&"f32"));
        assert!(suffixes.contains(&"i32"));
        assert!(suffixes.contains(&"i64"));
        assert!(suffixes.contains(&"u32"));
        assert!(suffixes.contains(&"u64"));
    }
}
