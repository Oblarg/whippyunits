use quote::quote;

/// Generate the custom_literal module with all unit macros using a custom module name
/// This uses only the canonical data from whippyunits-core
pub fn generate_custom_literal_module_with_name(module_name: &str) -> proc_macro2::TokenStream {
    // Get all unit symbols from the canonical data (filtered to exclude Rust keywords)
    let unit_symbols = crate::get_all_unit_symbols_for_literals();
    let type_suffixes = vec!["f64", "f32", "i32", "i64", "u32", "u64"];

    let mut float_macros = Vec::new();
    let mut int_macros = Vec::new();

    // Generate typed macros (existing functionality)
    for unit_symbol in &unit_symbols {
        for type_suffix in &type_suffixes {
            let macro_name = format!("{}_{}", unit_symbol, type_suffix);
            let unit_ident = syn::Ident::new(unit_symbol, proc_macro2::Span::mixed_site());
            let macro_name_ident = syn::Ident::new(&macro_name, unit_ident.span());

            let type_ident = syn::parse_str::<syn::Type>(type_suffix).unwrap();

            match *type_suffix {
                "f64" | "f32" => {
                    float_macros.push(quote! {
                        #[macro_export]
                        macro_rules! #macro_name_ident {
                            ($value:literal) => {{
                                whippyunits::quantity!($value, #unit_ident, #type_ident)
                            }};
                        }
                        pub(crate) use #macro_name_ident;
                    });
                }
                "i32" | "i64" | "u32" | "u64" => {
                    int_macros.push(quote! {
                        #[macro_export]
                        macro_rules! #macro_name_ident {
                            ($value:literal) => {{
                                whippyunits::quantity!($value, #unit_ident, #type_ident)
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
        let unit_ident = syn::Ident::new(unit_symbol, proc_macro2::Span::mixed_site());
        let macro_name_ident = syn::Ident::new(unit_symbol, unit_ident.span());

        // Create shortname macro for float module using quantity! macro directly
        float_macros.push(quote! {
            macro_rules! #macro_name_ident {
                ($value:literal) => {{
                    whippyunits::quantity!($value as f64, #unit_ident, f64)
                }};
            }
            pub(crate) use #macro_name_ident;
        });

        // Create shortname macro for int module using quantity! macro directly
        int_macros.push(quote! {
            macro_rules! #macro_name_ident {
                ($value:literal) => {{
                    whippyunits::quantity!($value as i32, #unit_ident, i32)
                }};
            }
            pub(crate) use #macro_name_ident;
        });
    }

    // Use the single generic function in default mode (no lift trace)
    crate::generate_literal_macros_module(module_name, false, None, false, syn::Ident::new(module_name, proc_macro2::Span::mixed_site())).into()
}


