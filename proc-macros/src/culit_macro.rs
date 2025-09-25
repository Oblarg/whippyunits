use quote::quote;
use whippyunits_default_dimensions::{BASE_UNITS, SI_PREFIXES};

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
                    // Float literals: ($before_decimal:literal $after_decimal:literal $exponent:literal)
                    float_macros.push(quote! {
                        #[macro_export]
                        macro_rules! #macro_name_ident {
                            ($before_decimal:literal $after_decimal:literal $exponent:literal) => {{
                                let value: #type_ident = format!("{}.{}{}", $before_decimal, $after_decimal, $exponent).parse().unwrap();
                                quantity!(value, #unit_ident, #type_ident)
                            }};
                        }
                        pub(crate) use #macro_name_ident;
                    });
                }
                "i32" | "i64" | "u32" | "u64" => {
                    // Integer literals: ($value:literal $base:literal)
                    int_macros.push(quote! {
                        #[macro_export]
                        macro_rules! #macro_name_ident {
                            ($value:literal $base:literal) => {{
                                let value: #type_ident = #type_ident::from_str_radix($value, $base).unwrap();
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
    
    // Generate shortname macros for all units (delegates to unit! macro with new)
    // These use the same parsing patterns but delegate to unit! instead of quantity!
    for unit_symbol in &unit_symbols {
        let macro_name_ident = syn::Ident::new(unit_symbol, proc_macro2::Span::call_site());
        let unit_ident = syn::Ident::new(unit_symbol, proc_macro2::Span::call_site());
        
        // Create shortname macro for float module (matches float pattern)
        float_macros.push(quote! {
            macro_rules! #macro_name_ident {
                ($before_decimal:literal $after_decimal:literal $exponent:literal) => {{
                    let value: f64 = format!("{}.{}{}", $before_decimal, $after_decimal, $exponent).parse().unwrap();
                    <whippyunits::unit!(#unit_ident)>::new(value)
                }};
            }
            pub(crate) use #macro_name_ident;
        });
        
        // Create shortname macro for int module (matches int pattern)
        int_macros.push(quote! {
            macro_rules! #macro_name_ident {
                ($value:literal $base:literal) => {{
                    let value: i32 = i32::from_str_radix($value, $base).unwrap();
                    <whippyunits::unit!(#unit_ident)>::new(value)
                }};
            }
            pub(crate) use #macro_name_ident;
        });
        
    }
    
    quote! {
        pub mod custom_literal {
            pub mod float {
                #(#float_macros)*
            }
            
            pub mod int {
                #(#int_macros)*
            }
        }
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
    
    // Add prefixed units from the canonical data
    for prefix in SI_PREFIXES.iter() {
        for unit in BASE_UNITS.iter() {
            if unit.symbol != "dimensionless" {
                symbols.push(format!("{}{}", prefix.symbol, unit.symbol));
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
        "use", "where", "while", "async", "await", "dyn"
    ];
    
    symbols.retain(|symbol| !rust_keywords.contains(&symbol.as_str()));
    
    symbols
}
