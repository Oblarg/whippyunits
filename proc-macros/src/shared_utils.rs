/// Shared utilities for proc macros
use syn::Ident;

/// Check if a unit name can be parsed as a valid Rust identifier
/// This filters out units with unicode characters or other invalid identifier characters
pub fn is_valid_identifier(name: &str) -> bool {
    syn::parse_str::<Ident>(name).is_ok()
}

/// Generate scale name using the same logic as default_declarators_macro
/// This ensures consistency between define_base_units and default_declarators macros
pub fn generate_scale_name(prefix_name: &str, unit_suffix: &str) -> String {
    // Systematically generate the correct naming convention
    let unit_singular = unit_suffix.trim_end_matches('s');
    let combined_name = if prefix_name.is_empty() {
        unit_singular.to_string()
    } else {
        format!("{}{}", prefix_name, unit_singular)
    };

    // Use the same capitalization logic as default_declarators_macro
    whippyunits_core::CapitalizedFmt(&combined_name).to_string()
}



