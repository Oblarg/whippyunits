use quote::quote;
use whippyunits_default_dimensions::{BASE_UNITS, SI_PREFIXES, COMPOUND_UNITS};

/// Generate the custom_literal module with all unit macros
/// This uses only the canonical data from default-dimensions
pub fn generate_custom_literal_module() -> proc_macro2::TokenStream {
    // Get all unit symbols from the canonical data
    let unit_symbols = get_all_unit_symbols();
    let type_suffixes = vec!["f64", "f32", "i32", "i64", "u32", "u64"];

    let mut float_macros = Vec::new();
    let mut int_macros = Vec::new();

    // Generate typed macros (existing functionality)
    for unit_symbol in &unit_symbols {
        for type_suffix in &type_suffixes {
            let macro_name = format!("{}_{}", unit_symbol, type_suffix);
            let macro_name_ident = syn::Ident::new(&macro_name, proc_macro2::Span::call_site());

            let unit_ident = syn::Ident::new(unit_symbol, proc_macro2::Span::call_site());

            let type_ident = syn::parse_str::<syn::Type>(type_suffix).unwrap();

            match *type_suffix {
                "f64" | "f32" => {
                    // Float literals: ($value:literal) - simplified contract for culit 0.4
                    float_macros.push(quote! {
                        #[macro_export]
                        macro_rules! #macro_name_ident {
                            ($value:literal) => {{
                                let value: #type_ident = $value;
                                quantity!(value, #unit_ident, #type_ident)
                            }};
                        }
                        pub(crate) use #macro_name_ident;
                    });
                }
                "i32" | "i64" | "u32" | "u64" => {
                    // Integer literals: ($value:literal) - simplified contract for culit 0.4
                    int_macros.push(quote! {
                        #[macro_export]
                        macro_rules! #macro_name_ident {
                            ($value:literal) => {{
                                let value: #type_ident = $value;
                                quantity!(value, #unit_ident, #type_ident)
                            }};
                        }
                        pub(crate) use #macro_name_ident;
                    });
                }
                _ => continue,
            };
        }
    }

    // Generate shortname macros for all units (calls local declarator methods directly)
    // These use the same parsing patterns but call methods like .meters(), .grams(), etc.
    for unit_symbol in &unit_symbols {
        let macro_name_ident = syn::Ident::new(unit_symbol, proc_macro2::Span::call_site());

        // Get the method name from the unit symbol using default-dimensions data
        let method_name = get_method_name_for_unit_symbol(unit_symbol);

        // Check if this is a compound unit that needs special handling
        if method_name.starts_with("__COMPOUND_UNIT__") {
            let unit_ident = syn::Ident::new(unit_symbol, proc_macro2::Span::call_site());
            
            // Create shortname macro for float module using local quantity! macro directly
            float_macros.push(quote! {
                macro_rules! #macro_name_ident {
                    ($value:literal) => {{
                        quantity!($value as f64, #unit_ident, f64)
                    }};
                }
                pub(crate) use #macro_name_ident;
            });

            // Create shortname macro for int module using local quantity! macro directly
            int_macros.push(quote! {
                macro_rules! #macro_name_ident {
                    ($value:literal) => {{
                        quantity!($value as i32, #unit_ident, i32)
                    }};
                }
                pub(crate) use #macro_name_ident;
            });
        } else {
            let method_ident = syn::Ident::new(&method_name, proc_macro2::Span::call_site());

            // Create shortname macro for float module (matches float pattern) - simplified contract for culit 0.4
            float_macros.push(quote! {
                macro_rules! #macro_name_ident {
                    ($value:literal) => {{
                        ($value as f64).#method_ident()
                    }};
                }
                pub(crate) use #macro_name_ident;
            });

            // Create shortname macro for int module (matches int pattern) - simplified contract for culit 0.4
            int_macros.push(quote! {
                macro_rules! #macro_name_ident {
                    ($value:literal) => {{
                        ($value as i32).#method_ident()
                    }};
                }
                pub(crate) use #macro_name_ident;
            });
        }
    }

    quote! {
        #[allow(unused_macros)]
        pub mod custom_literal {
            pub mod float {
                #(#float_macros)*
            }

            pub mod integer {
                #(#int_macros)*
            }
        }
    }
}

/// Get the method name for a unit symbol using default-dimensions data
/// Maps unit symbols like "m", "kg", "s" to method names like "meters", "kilograms", "seconds"
/// For compound units like "J", "W", etc., returns a special marker to use unit! macro directly
fn get_method_name_for_unit_symbol(unit_symbol: &str) -> String {
    // Handle special cases for angular units that have different method names
    let special_angular_mappings = [
        ("rot", "turns"),
        ("turn", "turns"),
        ("deg", "degrees"),
        ("grad", "gradians"),
        ("gon", "gradians"),
        ("arcmin", "arcminutes"),
        ("arcsec", "arcseconds"),
    ];
    
    if let Some(method_name) = special_angular_mappings.iter().find(|(symbol, _)| *symbol == unit_symbol) {
        return method_name.1.to_string();
    }

    // First, try to find the unit in UNIT_LITERALS
    if let Some(unit_info) = whippyunits_default_dimensions::lookup_unit_literal(unit_symbol) {
        // Check if this is actually a prefixed unit that was returned as base unit
        if let Some((base_symbol, prefix)) = is_prefixed_base_unit(unit_symbol) {
            // This is a prefixed unit, so we need to construct the proper method name
            let base_method = make_plural(unit_info.long_name);
            let prefix_name = get_prefix_name(prefix);
            return format!("{}{}", prefix_name, base_method);
        } else {
            // This is a direct unit, use it as-is
            return make_plural(unit_info.long_name);
        }
    }

    // Check if this is a compound unit (J, W, N, etc.)
    if is_compound_unit(unit_symbol) {
        // For compound units, we'll use a special marker that will be handled differently
        return format!("__COMPOUND_UNIT__{}", unit_symbol);
    }

    // Check if this is a prefixed compound unit (kJ, mW, etc.)
    if let Some((_base_symbol, _prefix)) = is_prefixed_compound_unit(unit_symbol) {
        return format!("__COMPOUND_UNIT__{}", unit_symbol);
    }

    // If not found, try to parse as a prefixed unit
    if let Some((base_symbol, prefix)) = is_prefixed_base_unit(unit_symbol) {
        if let Some(base_unit_info) =
            whippyunits_default_dimensions::lookup_unit_literal(base_symbol)
        {
            let base_method = make_plural(base_unit_info.long_name);
            let prefix_name = get_prefix_name(prefix);
            let result = format!("{}{}", prefix_name, base_method);
            // Add a compile-time error to see what's being generated
            if unit_symbol == "ms" {
                panic!("DEBUG: ms -> base_symbol={}, prefix={}, base_method={}, prefix_name={}, result={}", 
                       base_symbol, prefix, base_method, prefix_name, result);
            }
            return result;
        }
    }

    // Fallback: convert symbol to a reasonable method name
    unit_symbol.to_string()
}

/// Convert singular unit names to plural method names
/// Simply adds 's' to the end of the long name
fn make_plural(singular: &str) -> String {
    format!("{}s", singular)
}

/// Check if a unit symbol is a compound unit (J, W, N, etc.)
fn is_compound_unit(unit_symbol: &str) -> bool {
    whippyunits_default_dimensions::COMPOUND_UNITS
        .iter()
        .any(|compound_unit_info| compound_unit_info.symbol == unit_symbol)
}

/// Check if a unit symbol is a prefixed compound unit (kJ, mW, etc.)
fn is_prefixed_compound_unit(unit_symbol: &str) -> Option<(&str, &str)> {
    // Try to find a compound unit that this unit name ends with
    for compound_unit in whippyunits_default_dimensions::COMPOUND_UNITS {
        if unit_symbol.ends_with(compound_unit.symbol) {
            let prefix_part = &unit_symbol[..unit_symbol.len() - compound_unit.symbol.len()];

            // If no prefix, it should have been found in the direct lookup above
            if prefix_part.is_empty() {
                continue;
            }

            // Check if this is a valid prefix
            if whippyunits_default_dimensions::lookup_si_prefix(prefix_part).is_some() {
                return Some((compound_unit.symbol, prefix_part));
            }
        }
    }

    None
}

/// Check if a unit symbol is a prefixed base unit (e.g., "km", "cm", "mm")
fn is_prefixed_base_unit(unit_symbol: &str) -> Option<(&str, &str)> {
    // Try to find a base unit that this unit name ends with
    for base_unit in whippyunits_default_dimensions::BASE_UNITS {
        if base_unit.symbol == "dimensionless" {
            continue;
        }

        if unit_symbol.ends_with(base_unit.symbol) {
            let prefix_part = &unit_symbol[..unit_symbol.len() - base_unit.symbol.len()];

            // If no prefix, it should have been found in the direct lookup above
            if prefix_part.is_empty() {
                continue;
            }

            // Check if this is a valid prefix
            if whippyunits_default_dimensions::lookup_si_prefix(prefix_part).is_some() {
                return Some((base_unit.symbol, prefix_part));
            }

        }
    }

    None
}

/// Get the prefix name for a prefix symbol (e.g., "k" -> "kilo", "m" -> "milli")
fn get_prefix_name(prefix_symbol: &str) -> String {
    if let Some(prefix_info) = whippyunits_default_dimensions::lookup_si_prefix(prefix_symbol) {
        // Convert the long name to a method-friendly name
        match prefix_info.long_name {
            "kilo" => "kilo".to_string(),
            "milli" => "milli".to_string(),
            "micro" => "micro".to_string(),
            "nano" => "nano".to_string(),
            "pico" => "pico".to_string(),
            "femto" => "femto".to_string(),
            "atto" => "atto".to_string(),
            "zepto" => "zepto".to_string(),
            "yocto" => "yocto".to_string(),
            "ronto" => "ronto".to_string(),
            "quecto" => "quecto".to_string(),
            "deca" => "deca".to_string(),
            "hecto" => "hecto".to_string(),
            "mega" => "mega".to_string(),
            "giga" => "giga".to_string(),
            "tera" => "tera".to_string(),
            "peta" => "peta".to_string(),
            "exa" => "exa".to_string(),
            "zetta" => "zetta".to_string(),
            "yotta" => "yotta".to_string(),
            "ronna" => "ronna".to_string(),
            "quetta" => "quetta".to_string(),
            "deci" => "deci".to_string(),
            "centi" => "centi".to_string(),
            _ => prefix_info.long_name.to_string(),
        }
    } else {
        prefix_symbol.to_string()
    }
}

/// Get all unit symbols from the canonical data in default-dimensions
/// This is the single source of truth for what units should have custom literals
fn get_all_unit_symbols() -> Vec<String> {
    let mut symbols = Vec::new();

    // Add base units from the canonical data
    for unit in BASE_UNITS.iter() {
        if unit.symbol != "dimensionless" {
            symbols.push(unit.symbol.to_string());
        }
    }

    // Add compound units from the canonical data
    for compound_unit in COMPOUND_UNITS.iter() {
        symbols.push(compound_unit.symbol.to_string());
    }

    // Add additional units from UNIT_LITERALS (including angular units like deg, rot, turn, etc.)
    for unit_literal in whippyunits_default_dimensions::UNIT_LITERALS.iter() {
        symbols.push(unit_literal.symbol.to_string());
    }

    // Add prefixed units from the canonical data (base units)
    for prefix in SI_PREFIXES.iter() {
        for unit in BASE_UNITS.iter() {
            if unit.symbol != "dimensionless" {
                symbols.push(format!("{}{}", prefix.symbol, unit.symbol));
            }
        }
    }

    // Add prefixed compound units (kJ, mW, etc.)
    for prefix in SI_PREFIXES.iter() {
        for compound_unit in COMPOUND_UNITS.iter() {
            symbols.push(format!("{}{}", prefix.symbol, compound_unit.symbol));
        }
    }

    symbols.sort();
    symbols.dedup();

    // Filter out Rust keywords
    let rust_keywords = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn",
    ];

    symbols.retain(|symbol| !rust_keywords.contains(&symbol.as_str()));

    symbols
}

