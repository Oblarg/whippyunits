/// Shared utilities for proc macros
use syn::Ident;
use proc_macro2::TokenStream;

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

/// Shared helper function to get the corresponding default declarator type for a unit
/// This is used by both the unit! macro and local_unit! macro to avoid code duplication
pub fn get_declarator_type_for_unit(unit_name: &str) -> Option<TokenStream> {
    // Skip dimensionless units - they don't have corresponding default declarator types
    if unit_name == "dimensionless" {
        return None;
    }

    // Check if it's a base unit (these have corresponding types)
    let atomic_dimensions = whippyunits_core::Dimension::BASIS;
    for dimension in atomic_dimensions {
        if let Some(unit) = dimension
            .units
            .iter()
            .find(|u| u.symbols.contains(&unit_name))
        {
            let type_name = whippyunits_core::CapitalizedFmt(unit.name).to_string();
            let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
            return Some(quote::quote! {
                whippyunits::default_declarators::#type_ident
            });
        }
    }

    // Check if it's a prefixed unit FIRST (before checking unit literals)
    let (prefix_opt, base) = parse_unit_with_prefix_core(unit_name);
    if let Some(prefix) = prefix_opt {
        // Find the base unit
        if let Some((base_unit, _)) = whippyunits_core::Dimension::find_unit_by_symbol(&base) {
            // Generate the prefixed type name by combining prefix name with base unit name
            let unit_singular = base_unit.name.trim_end_matches('s');
            let combined_name = format!("{}{}", prefix.name(), unit_singular);
            let type_name = whippyunits_core::CapitalizedFmt(&combined_name).to_string();
            let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
            return Some(quote::quote! {
                whippyunits::default_declarators::#type_ident
            });
        }
    }

    // Check if it's a unit literal (like min, h, hr, d, etc.)
    if let Some((_dimension, unit)) = lookup_unit_literal_direct(unit_name) {
        let type_name = whippyunits_core::CapitalizedFmt(unit.name).to_string();
        let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
        return Some(quote::quote! {
            whippyunits::default_declarators::#type_ident
        });
    }

    None
}

/// Parse unit name to extract prefix and base unit for UCUM
fn parse_unit_with_prefix_direct(unit_name: &str) -> (Option<&str>, String) {
    // Use the centralized parsing logic from whippyunits-core
    let (prefix_opt, base_unit) = parse_unit_with_prefix_core(unit_name);
    let prefix_str = prefix_opt.map(|p| p.symbol());
    (prefix_str, base_unit)
}

/// Parse a unit name to extract prefix and base unit
/// Returns (prefix_option, base_unit_name)
fn parse_unit_with_prefix_core(unit_name: &str) -> (Option<&'static whippyunits_core::SiPrefix>, String) {
    // Try to strip any prefix from the unit name
    if let Some((prefix, base)) = whippyunits_core::SiPrefix::strip_any_prefix_symbol(unit_name) {
        // Check if the base unit exists
        if whippyunits_core::Dimension::find_unit_by_symbol(base).is_some() {
            return (Some(prefix), String::from(base));
        }
    }

    // Also try stripping prefix from name (not just symbol)
    if let Some((prefix, base)) = whippyunits_core::SiPrefix::strip_any_prefix_name(unit_name) {
        // Check if the base unit exists by name
        if whippyunits_core::Dimension::find_unit_by_name(base).is_some() {
            return (Some(prefix), String::from(base));
        }
    }

    (None, String::from(unit_name))
}

/// Look up unit literal information directly
fn lookup_unit_literal_direct(unit_name: &str) -> Option<(&whippyunits_core::Dimension, &whippyunits_core::Unit)> {
    // Check all dimensions for this unit
    for dimension in whippyunits_core::Dimension::ALL {
        if let Some(unit) = dimension.units.iter().find(|u| {
            u.symbols.contains(&unit_name) || u.name == unit_name
        }) {
            return Some((dimension, unit));
        }
    }
    None
}



