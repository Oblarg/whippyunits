use proc_macro2::TokenStream;
use quote::quote;
/// Shared utilities for proc macros
use syn::Ident;
use whippyunits_core::{Dimension, SiPrefix, UnitExpr};

/// Check if a unit name can be parsed as a valid Rust identifier
/// This filters out units with unicode characters or other invalid identifier characters
pub fn is_valid_identifier(name: &str) -> bool {
    syn::parse_str::<Ident>(name).is_ok()
}

/// Generate scale name using the same logic as generate_default_declarators_macro
/// This ensures consistency between define_base_units and default_declarators macros
pub fn generate_scale_name(prefix_name: &str, unit_name: &str) -> String {
    // Systematically generate the correct naming convention
    // Use the unit name as-is (it's already singular) for type names
    let combined_name = if prefix_name.is_empty() {
        unit_name.to_string()
    } else {
        format!("{}{}", prefix_name, unit_name)
    };

    // Use the same capitalization logic as generate_default_declarators_macro
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
    use whippyunits_core::dimension_exponents::DynDimensionExponents;
    use whippyunits_core::{Dimension, SiPrefix, System};

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
                // For metric units, use the same logic as generate_default_declarators_macro
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
                // For non-metric units, skip - they don't have types in default_declarators
                continue;
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
            if let Some(prefix) = SiPrefix::ALL
                .iter()
                .find(|p| p.factor_log10() == scale_diff)
            {
                // Generate prefixed type name using same logic as generate_default_declarators_macro
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
fn parse_unit_with_prefix_core(
    unit_name: &str,
) -> (Option<&'static whippyunits_core::SiPrefix>, String) {
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
fn lookup_unit_literal_direct(
    unit_name: &str,
) -> Option<(&whippyunits_core::Dimension, &whippyunits_core::Unit)> {
    // Check all dimensions for this unit
    for dimension in whippyunits_core::Dimension::ALL {
        if let Some(unit) = dimension
            .units
            .iter()
            .find(|u| u.symbols.contains(&unit_name) || u.name == unit_name)
        {
            return Some((dimension, unit));
        }
    }
    None
}

/// Get the storage unit type for a given unit (for affine or nonstorage units)
/// Returns the declarator type for the storage unit if the unit is affine or nonstorage,
/// otherwise returns None (meaning use the unit's own type)
pub fn get_storage_unit_type_for_unit(unit_name: &str) -> Option<TokenStream> {
    use whippyunits_core::Dimension;

    // Find the unit and its dimension
    let (unit, dimension) =
        if let Some((unit, dimension)) = Dimension::find_unit_by_symbol(unit_name) {
            (unit, dimension)
        } else if let Some((unit, dimension)) = Dimension::find_unit_by_name(unit_name) {
            (unit, dimension)
        } else {
            return None;
        };

    // Check if this is an affine or nonstorage unit
    let is_nonstorage = unit.conversion_factor != 1.0;
    let is_affine = unit.affine_offset != 0.0;

    if !is_nonstorage && !is_affine {
        // This is a storage unit, so return None to use its own type
        return None;
    }

    // Find the storage unit (same scale, conversion_factor == 1.0, affine_offset == 0.0)
    let storage_unit = dimension
        .units
        .iter()
        .find(|u| u.scale == unit.scale && u.conversion_factor == 1.0 && u.affine_offset == 0.0)
        .or_else(|| {
            // Fallback: find any storage unit in the dimension
            dimension
                .units
                .iter()
                .find(|u| u.conversion_factor == 1.0 && u.affine_offset == 0.0)
        })?;

    // Get the declarator type for the storage unit
    get_declarator_type_for_unit(storage_unit.symbols[0])
}

/// Recursively collect identifiers from a unit expression
pub fn collect_identifiers_from_expr(expr: &UnitExpr, identifiers: &mut Vec<Ident>) {
    match expr {
        UnitExpr::Unit(unit) => {
            identifiers.push(unit.name.clone());
        }
        UnitExpr::Mul(left, right) => {
            collect_identifiers_from_expr(left, identifiers);
            collect_identifiers_from_expr(right, identifiers);
        }
        UnitExpr::Div(left, right) => {
            collect_identifiers_from_expr(left, identifiers);
            collect_identifiers_from_expr(right, identifiers);
        }
        UnitExpr::Pow(base, _) => {
            collect_identifiers_from_expr(base, identifiers);
        }
    }
}

/// Generate documentation comment for a unit
pub fn generate_unit_doc_comment(unit_name: &str) -> TokenStream {
    let doc_text = get_unit_documentation_text(unit_name);
    quote! {
        #[doc = #doc_text]
    }
}

/// Get documentation text for a unit
pub fn get_unit_documentation_text(unit_name: &str) -> String {
    // Try to get information from the whippyunits-core data
    if let Some(unit_info) = get_unit_doc_info(unit_name) {
        unit_info
    } else {
        format!("{} ({})", unit_name.to_uppercase(), unit_name)
    }
}

/// Get unit documentation information from whippyunits-core data
pub fn get_unit_doc_info(unit_name: &str) -> Option<String> {
    // First check for exact unit match (prioritize exact matches over prefix matches)
    if let Some((unit, _dimension)) = Dimension::find_unit_by_symbol(unit_name) {
        // Use the first symbol from unit.symbols as the abbreviation
        let symbol = unit.symbols.first().unwrap_or(&unit_name);
        return Some(format!("{} ({})", unit.name, symbol));
    }

    if let Some((unit, _dimension)) = Dimension::find_unit_by_name(unit_name) {
        // Use the first symbol from unit.symbols as the abbreviation
        let symbol = unit.symbols.first().unwrap_or(&unit_name);
        return Some(format!("{} ({})", unit.name, symbol));
    }

    // Only if no exact match found, check if it's a prefixed unit
    if let Some((prefix_symbol, _base_symbol)) = parse_prefixed_unit(unit_name) {
        use whippyunits_core::to_unicode_superscript;
        if let Some(prefix_info) = SiPrefix::from_symbol(&prefix_symbol) {
            // PARSE: Get the abstract representation (prefix + base unit)
            let (base_unit_name, base_unit_symbol) =
                if let Some((base_unit, _)) = Dimension::find_unit_by_symbol(&_base_symbol) {
                    (
                        base_unit.name,
                        base_unit.symbols.first().unwrap_or(&base_unit.name),
                    )
                } else if let Some((base_unit, _)) = Dimension::find_unit_by_name(&_base_symbol) {
                    (
                        base_unit.name,
                        base_unit.symbols.first().unwrap_or(&base_unit.name),
                    )
                } else {
                    (_base_symbol.as_str(), &_base_symbol.as_str())
                };

            // TRANSFORM: Convert abstract representation to normalized display format
            let scale_text = if prefix_info.factor_log10() == 0 {
                "10⁰".to_string()
            } else {
                format!(
                    "10{}",
                    to_unicode_superscript(prefix_info.factor_log10(), false)
                )
            };

            let prefixed_unit_name = format!("{}{}", prefix_info.name(), base_unit_name);
            let prefixed_symbol = format!("{}{}", prefix_info.symbol(), base_unit_symbol);

            return Some(format!(
                "{} ({}) - Prefix: {} ({}), Base: {}",
                prefixed_unit_name,
                prefixed_symbol,
                prefix_info.name(),
                scale_text,
                base_unit_name
            ));
        }
    }

    None
}

/// Parse a unit name to extract prefix and base unit
///
/// This function now uses the centralized parsing logic from whippyunits-core.
/// Only allows prefixing of base units (first unit in each dimension by declaration order).
pub fn parse_prefixed_unit(unit_name: &str) -> Option<(String, String)> {
    // Try to strip any prefix from the unit name
    if let Some((prefix, base)) = SiPrefix::strip_any_prefix_symbol(unit_name) {
        // Check if the base unit exists and is a base unit (first unit in its dimension)
        if let Some((unit, dimension)) = Dimension::find_unit_by_symbol(base) {
            // Check if this is the first unit in its dimension (base unit)
            if dimension
                .units
                .first()
                .map(|first_unit| first_unit.name == unit.name)
                .unwrap_or(false)
            {
                // Only allow prefixing if the base unit is a metric unit (not imperial)
                if unit.system == whippyunits_core::System::Metric {
                    return Some((prefix.symbol().to_string(), base.to_string()));
                }
            }
        }
    }

    // Also try stripping prefix from name (not just symbol)
    if let Some((prefix, base)) = SiPrefix::strip_any_prefix_name(unit_name) {
        // Check if the base unit exists by name and is a base unit
        if let Some((unit, dimension)) = Dimension::find_unit_by_name(base) {
            // Check if this is the first unit in its dimension (base unit)
            if dimension
                .units
                .first()
                .map(|first_unit| first_unit.name == unit.name)
                .unwrap_or(false)
            {
                // Only allow prefixing if the base unit is a metric unit (not imperial)
                if unit.system == whippyunits_core::System::Metric {
                    return Some((prefix.symbol().to_string(), base.to_string()));
                }
            }
        }
    }

    None
}

/// Generate documentation structs for unit identifiers in the expression
/// For affine or nonstorage units, shows doc shadows corresponding to the storage type
pub fn generate_unit_documentation_for_expr(
    unit_expr: &UnitExpr,
    use_storage_type_for_affine: bool,
) -> TokenStream {
    // Extract identifiers from the unit expression
    let mut identifiers = Vec::new();
    collect_identifiers_from_expr(unit_expr, &mut identifiers);

    // Generate documentation for each identifier
    let doc_structs: Vec<TokenStream> = identifiers
        .into_iter()
        .map(|ident: Ident| {
            let unit_name = ident.to_string();

            // For affine or nonstorage units, use the storage unit type if requested
            let declarator_type = if use_storage_type_for_affine {
                if let Some(storage_type) = get_storage_unit_type_for_unit(&unit_name) {
                    storage_type
                } else if let Some(declarator_type) = get_declarator_type_for_unit(&unit_name) {
                    declarator_type
                } else {
                    // Fallback for units without declarator types
                    quote! { () }
                }
            } else {
                // Get the proper declarator type for this unit
                if let Some(declarator_type) = get_declarator_type_for_unit(&unit_name) {
                    declarator_type
                } else {
                    // Fallback for units without declarator types
                    quote! { () }
                }
            };

            let doc_comment = generate_unit_doc_comment(&unit_name);

            quote! {
                const _: () = {
                    #doc_comment
                    #[allow(non_camel_case_types)]
                    type #ident = #declarator_type;
                };
            }
        })
        .collect();

    quote! {
        #(#doc_structs)*
    }
}
