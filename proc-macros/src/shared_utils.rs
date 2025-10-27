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
pub fn generate_scale_name(prefix_name: &str, unit_name: &str) -> String {
    // Systematically generate the correct naming convention
    // Use the unit name as-is (it's already singular) for type names
    let combined_name = if prefix_name.is_empty() {
        unit_name.to_string()
    } else {
        format!("{}{}", prefix_name, unit_name)
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
            // Use the same logic as generate_scale_name to ensure consistency
            let type_name = generate_scale_name(prefix.name(), base_unit.name);
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

/// Get the corresponding default declarator type for a unit based on dimension and scale exponents
/// This uses the same logic as default declarators to ensure consistency
pub fn get_declarator_type_for_exponents(
    dimension_exponents: whippyunits_core::dimension_exponents::DynDimensionExponents,
    scale_exponents: whippyunits_core::scale_exponents::ScaleExponents,
) -> Option<TokenStream> {
    use whippyunits_core::{Dimension, System, SiPrefix};
    use whippyunits_core::dimension_exponents::DynDimensionExponents;

    // Skip dimensionless units - they don't have corresponding default declarator types
    if dimension_exponents == DynDimensionExponents::ZERO {
        return None;
    }

    // Find the dimension that matches these exponents
    let matching_dimension = Dimension::ALL
        .iter()
        .find(|dim| dim.exponents == dimension_exponents)?;

    // Look for a unit in this dimension that matches the scale exponents
    // We need to find a unit that has the same scale exponents
    for unit in matching_dimension.units {
        if unit.scale == scale_exponents {
            // Found a matching unit - generate type name using same logic as default declarators
            let type_name = if unit.system == System::Metric {
                // For metric units, use the same logic as default_declarators_macro
                // Check if this is a base unit (first unit in the dimension)
                let is_base_unit = matching_dimension.units[0].name == unit.name;
                
                if is_base_unit {
                    // For base units, use the singular name for type generation
                    generate_scale_name("", unit.name)
                } else {
                    // For derived units, use the capitalized name
                    whippyunits_core::CapitalizedFmt(unit.name).to_string()
                }
            } else {
                // For non-metric units, use the capitalized name
                whippyunits_core::CapitalizedFmt(unit.name).to_string()
            };

            let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
            return Some(quote::quote! {
                whippyunits::default_declarators::#type_ident
            });
        }
    }

    // If no exact match found, check if this could be a prefixed unit
    // Look for a base unit in the same dimension and see if we can match with a prefix
    for unit in matching_dimension.units {
        if unit.system == System::Metric {
            // Check if this could be a prefixed version of the base unit
            // The base unit is the first unit in the dimension
            let base_unit = &matching_dimension.units[0];
            
            // Check if the scale difference corresponds to a known prefix
            let scale_diff = scale_exponents.0[0] - base_unit.scale.0[0]; // Check p2 difference
            if let Some(prefix) = SiPrefix::ALL.iter().find(|p| p.factor_log10() == scale_diff) {
                // Generate prefixed type name using same logic as default_declarators_macro
                let type_name = generate_scale_name(prefix.name(), base_unit.name);
                
                let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
                return Some(quote::quote! {
                    whippyunits::default_declarators::#type_ident
                });
            }
        }
    }

    None
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



