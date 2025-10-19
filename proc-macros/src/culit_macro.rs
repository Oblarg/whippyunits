use quote::quote;

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
            let unit_ident = syn::parse_str::<syn::Ident>(unit_symbol).unwrap();
            let macro_name_ident = syn::Ident::new(&macro_name, unit_ident.span());

            let type_ident = syn::parse_str::<syn::Type>(type_suffix).unwrap();

            match *type_suffix {
                "f64" | "f32" => {
                    // Float literals: ($value:literal) - simplified contract for culit 0.4
                    float_macros.push(quote! {
                        #[macro_export]
                        macro_rules! #macro_name_ident {
                            ($value:literal) => {{
                                quantity!($value, #unit_ident, #type_ident)
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
                                quantity!($value, #unit_ident, #type_ident)
                            }};
                        }
                        pub(crate) use #macro_name_ident;
                    });
                }
                _ => continue,
            };
        }
    }

    // Generate shortname macros for all units (always use quantity! directly)
    // This eliminates the redundant method-based approach and unifies all paths through quantity!
    for unit_symbol in &unit_symbols {
        let unit_ident = syn::parse_str::<syn::Ident>(unit_symbol).unwrap();
        let macro_name_ident = syn::Ident::new(unit_symbol, unit_ident.span());

        // Create shortname macro for float module using quantity! macro directly
        float_macros.push(quote! {
            macro_rules! #macro_name_ident {
                ($value:literal) => {{
                    quantity!($value as f64, #unit_ident, f64)
                }};
            }
            pub(crate) use #macro_name_ident;
        });

        // Create shortname macro for int module using quantity! macro directly
        int_macros.push(quote! {
            macro_rules! #macro_name_ident {
                ($value:literal) => {{
                    quantity!($value as i32, #unit_ident, i32)
                }};
            }
            pub(crate) use #macro_name_ident;
        });
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


/// Get all unit symbols from the canonical data in whippyunits-core
/// This is the single source of truth for what units should have custom literals
fn get_all_unit_symbols_local() -> Vec<String> {
    use whippyunits_core::{Dimension, SiPrefix, Unit};
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
