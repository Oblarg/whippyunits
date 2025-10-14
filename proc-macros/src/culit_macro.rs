use quote::quote;
use whippyunits_core::{Dimension, SiPrefix};

/// Generate the custom_literal module with all unit macros
/// This uses only the canonical data from whippyunits-core
pub fn generate_custom_literal_module() -> proc_macro2::TokenStream {
    // Get all unit symbols from the canonical data (filtered to exclude Rust keywords)
    let unit_symbols = get_all_unit_symbols_local();
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
                        #[doc(hidden)]
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
                        #[doc(hidden)]
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

        // Get the method name from the unit symbol using whippyunits-core data
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

/// Get the method name for a unit symbol using whippyunits-core data
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

    // First, try to find the unit in the unified dimensions data
    if let Some((unit, _dimension)) = Dimension::find_unit_by_symbol(unit_symbol) {
        // Check if this is actually a compound unit that should use quantity! macro
        if is_compound_unit(unit_symbol) {
            return format!("__COMPOUND_UNIT__{}", unit_symbol);
        }
        
        // Check if this is actually a prefixed unit that was returned as base unit
        if let Some((prefix, _base_symbol)) = SiPrefix::strip_any_prefix_symbol(unit_symbol) {
            // This is a prefixed unit, so we need to construct the proper method name
            let base_method = make_plural(unit.name);
            let prefix_name = get_prefix_name(prefix.symbol());
            return format!("{}{}", prefix_name, base_method);
        } else {
            // This is a direct unit, use it as-is
            return make_plural(unit.name);
        }
    }

    // Check if this is a compound unit (J, W, N, etc.)
    if is_compound_unit(unit_symbol) {
        // For compound units, we'll use a special marker that will be handled differently
        return format!("__COMPOUND_UNIT__{}", unit_symbol);
    }

    // Check if this is a prefixed compound unit (kJ, mW, etc.)
    if let Some((_prefix, _base_symbol)) = SiPrefix::strip_any_prefix_symbol(unit_symbol) {
        return format!("__COMPOUND_UNIT__{}", unit_symbol);
    }

    // If not found, try to parse as a prefixed unit
    if let Some((prefix, base_symbol)) = SiPrefix::strip_any_prefix_symbol(unit_symbol) {
        if let Some((unit, _dimension)) = Dimension::find_unit_by_symbol(&base_symbol) {
            let base_method = make_plural(unit.name);
            let prefix_name = get_prefix_name(prefix.symbol());
            let result = format!("{}{}", prefix_name, base_method);
            // Add a compile-time error to see what's being generated
            if unit_symbol == "ms" {
                panic!("DEBUG: ms -> base_symbol={}, prefix={}, base_method={}, prefix_name={}, result={}", 
                       base_symbol, prefix.symbol(), base_method, prefix_name, result);
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

/// Check if a unit symbol is a compound unit (J, W, N, etc.) or derived unit (Hz, lm, etc.)
fn is_compound_unit(unit_symbol: &str) -> bool {
    if let Some((_unit, dimension)) = Dimension::find_unit_by_symbol(unit_symbol) {
        // Check if this is not one of the 8 base dimensions
        return !Dimension::BASIS.contains(dimension);
    }
    false
}


/// Get the prefix name for a prefix symbol (e.g., "k" -> "kilo", "m" -> "milli")
fn get_prefix_name(prefix_symbol: &str) -> String {
    if let Some(prefix_info) = SiPrefix::from_symbol(prefix_symbol) {
        // Convert the long name to a method-friendly name
        match prefix_info.name() {
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
            _ => prefix_info.name().to_string(),
        }
    } else {
        prefix_symbol.to_string()
    }
}

/// Get all unit symbols from the canonical data in whippyunits-core
/// This is the single source of truth for what units should have custom literals
fn get_all_unit_symbols_local() -> Vec<String> {
    use whippyunits_core::{Dimension, Unit, SiPrefix};
    let mut symbols = Vec::new();

    // Add base units from the canonical data
    for unit in Unit::BASES.iter() {
        if unit.name != "dimensionless" {
            for symbol in unit.symbols {
                symbols.push(symbol.to_string());
            }
        }
    }

    // Add all units from the unified dimensions data (including compound units and unit literals)
    for dimension in Dimension::ALL {
        for unit in dimension.units {
            for symbol in unit.symbols {
                symbols.push(symbol.to_string());
            }
        }
    }

    // Add prefixed units from the canonical data (base units)
    for prefix in SiPrefix::ALL {
        for unit in Unit::BASES.iter() {
            if unit.name != "dimensionless" {
                for symbol in unit.symbols {
                    symbols.push(format!("{}{}", prefix.symbol(), symbol));
                }
            }
        }
    }

    // Add prefixed compound units and derived units (kJ, mW, kN, mHz, etc.)
    for prefix in SiPrefix::ALL {
        for dimension in Dimension::ALL {
            // Anything that's not atomic is composite (compound or derived)
            if !Dimension::BASIS.contains(dimension) {
                for unit in dimension.units {
                    // Skip prefixed versions for units with conversion factors (imperial units)
                    // as they are stored internally in SI units and don't need prefixed types
                    if !unit.has_conversion() {
                        for symbol in unit.symbols {
                            symbols.push(format!("{}{}", prefix.symbol(), symbol));
                        }
                    }
                }
            }
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

