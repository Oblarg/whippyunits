use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

/// Get all unit symbols that should have literal macros
/// This is the single source of truth for what units should have custom literals
/// Used by both the regular define_literals!() and local unit literals
pub fn get_all_unit_symbols_for_literals() -> Vec<String> {
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
                            let prefixed_symbol = format!("{}{}", prefix.symbol(), symbol);
                            symbols.push(prefixed_symbol);
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

/// Generate literal macros module - generic function that works for both default and local modes
///
/// # Parameters
/// - `module_name`: Name of the module to generate (e.g., "custom_literal" or "local_unit_literals")
/// - `is_local_mode`: If true, uses local quantity! macro (no prefix); if false, uses whippyunits::quantity!
/// - `scale_params`: Optional scale parameters for lift trace (only used if is_local_mode is true)
/// - `for_namespace`: If true, generates just the float/integer submodules without outer wrapper
/// - `namespace_ident`: Optional namespace identifier for disambiguating local macros
pub fn generate_literal_macros_module(
    module_name: &str,
    is_local_mode: bool,
    scale_params: Option<(
        Ident,
        Ident,
        Ident,
        Ident,
        Ident,
        Ident,
        Ident,
        Ident,
    )>,
    for_namespace: bool,
    namespace_ident: Ident,
) -> TokenStream2 {
    // Get all unit symbols using the shared function
    let unit_symbols = get_all_unit_symbols_for_literals();

    // Determine the correct quantity! path based on mode
    let quantity_path = if is_local_mode {
        // Local mode: use the prefixed macro name from parent namespace module
        let prefixed_macro_name = syn::Ident::new(
            &format!("{}_quantity", namespace_ident.to_string()),
            namespace_ident.span(),
        );
        quote! { #prefixed_macro_name! }
    } else {
        // Default mode: always use whippyunits::quantity!
        quote! { whippyunits::quantity! }
    };

    let mut float_macros = Vec::new();
    let mut integer_macros = Vec::new();

    // Generate all literal macros for each unit symbol in a single iteration
    for unit_symbol in &unit_symbols {
        let unit_ident = syn::Ident::new(unit_symbol, proc_macro2::Span::mixed_site());

        // Helper function to generate docstring with optional storage type parameter
        let generate_doc_string = |storage_type: Option<&str>| {
            if is_local_mode {
                let mut formatted_details = String::new();
                let equivalent_text = if let Some(storage_type) = storage_type {
                    format!(
                        "equivalent to: `{}::quantity!(value, {}, {})`<br><hr><br>",
                        namespace_ident.to_string(),
                        unit_symbol,
                        storage_type
                    )
                } else {
                    format!(
                        "equivalent to: `{}::quantity!(value, {})`<br><hr><br>",
                        namespace_ident.to_string(),
                        unit_symbol
                    )
                };
                formatted_details.push_str(&equivalent_text);
                if let Some((
                    mass_scale,
                    length_scale,
                    time_scale,
                    current_scale,
                    temperature_scale,
                    amount_scale,
                    luminosity_scale,
                    angle_scale,
                )) = &scale_params
                {
                    let local_context = crate::utils::lift_trace::LocalContext {
                        mass_scale: mass_scale.clone(),
                        length_scale: length_scale.clone(),
                        time_scale: time_scale.clone(),
                        current_scale: current_scale.clone(),
                        temperature_scale: temperature_scale.clone(),
                        amount_scale: amount_scale.clone(),
                        luminosity_scale: luminosity_scale.clone(),
                        angle_scale: angle_scale.clone(),
                    };
                    let transformation_details =
                        local_context.get_transformation_details_for_identifier(unit_symbol);
                    let lines: Vec<&str> = transformation_details.details.lines().collect();
                    for (j, line) in lines.iter().enumerate() {
                        formatted_details.push_str(line);
                        if j < lines.len() - 1 {
                            formatted_details.push_str("<br>");
                        }
                    }
                }
                formatted_details
            } else {
                if let Some(storage_type) = storage_type {
                    format!(
                        "equivalent to: `default_declarators::quantity!(value, {}, {})`<br>",
                        unit_symbol, storage_type
                    )
                } else {
                    format!(
                        "equivalent to: `default_declarators::quantity!(value, {})`<br>",
                        unit_symbol
                    )
                }
            }
        };

        // Generate base documentation for shortname macros (without storage type)
        let base_doc_string = generate_doc_string(None);

        // Generate documentation for suffixed literals (with storage type parameter)
        let doc_string_f64 = generate_doc_string(Some("f64"));
        let doc_string_f32 = generate_doc_string(Some("f32"));
        let doc_string_i32 = generate_doc_string(Some("i32"));
        let doc_string_i64 = generate_doc_string(Some("i64"));
        let doc_string_u32 = generate_doc_string(Some("u32"));
        let doc_string_u64 = generate_doc_string(Some("u64"));

        // Generate unique inner names for each macro to avoid conflicts
        // For local mode, prefix with the namespace identifier to disambiguate between different local scales
        let inner_prefix = if is_local_mode {
            format!("{}_{}", namespace_ident, unit_symbol)
        } else {
            unit_symbol.clone()
        };

        // Generate all inner macro identifiers
        let inner_f64 = syn::Ident::new(
            &format!("{}_f64", inner_prefix),
            proc_macro2::Span::mixed_site(),
        );
        let inner_f32 = syn::Ident::new(
            &format!("{}_f32", inner_prefix),
            proc_macro2::Span::mixed_site(),
        );
        let inner_i32 = syn::Ident::new(
            &format!("{}_i32", inner_prefix),
            proc_macro2::Span::mixed_site(),
        );
        let inner_i64 = syn::Ident::new(
            &format!("{}_i64", inner_prefix),
            proc_macro2::Span::mixed_site(),
        );
        let inner_u32 = syn::Ident::new(
            &format!("{}_u32", inner_prefix),
            proc_macro2::Span::mixed_site(),
        );
        let inner_u64 = syn::Ident::new(
            &format!("{}_u64", inner_prefix),
            proc_macro2::Span::mixed_site(),
        );
        let inner_short_float = syn::Ident::new(
            &format!("{}_float", inner_prefix),
            proc_macro2::Span::mixed_site(),
        );
        let inner_short_int = syn::Ident::new(
            &format!("{}_int", inner_prefix),
            proc_macro2::Span::mixed_site(),
        );

        // Generate all outer macro identifiers
        let unit_f64 = syn::Ident::new(
            &format!("{}_f64", unit_symbol),
            proc_macro2::Span::mixed_site(),
        );
        let unit_f32 = syn::Ident::new(
            &format!("{}_f32", unit_symbol),
            proc_macro2::Span::mixed_site(),
        );
        let unit_i32 = syn::Ident::new(
            &format!("{}_i32", unit_symbol),
            proc_macro2::Span::mixed_site(),
        );
        let unit_i64 = syn::Ident::new(
            &format!("{}_i64", unit_symbol),
            proc_macro2::Span::mixed_site(),
        );
        let unit_u32 = syn::Ident::new(
            &format!("{}_u32", unit_symbol),
            proc_macro2::Span::mixed_site(),
        );
        let unit_u64 = syn::Ident::new(
            &format!("{}_u64", unit_symbol),
            proc_macro2::Span::mixed_site(),
        );

        // Generate typed float macros
        float_macros.push(quote! {
            #[doc = #doc_string_f64]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_f64 {
                ($value:literal) => {{
                    #quantity_path($value as f64, #unit_ident, f64)
                }};
            }
            pub use #inner_f64 as #unit_f64;

            #[doc = #doc_string_f32]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_f32 {
                ($value:literal) => {{
                    #quantity_path($value as f32, #unit_ident, f32)
                }};
            }
            pub use #inner_f32 as #unit_f32;
        });

        // Generate typed integer macros
        integer_macros.push(quote! {
            #[doc = #doc_string_i32]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_i32 {
                ($value:literal) => {{
                    #quantity_path($value as i32, #unit_ident, i32)
                }};
            }
            pub use #inner_i32 as #unit_i32;

            #[doc = #doc_string_i64]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_i64 {
                ($value:literal) => {{
                    #quantity_path($value as i64, #unit_ident, i64)
                }};
            }
            pub use #inner_i64 as #unit_i64;

            #[doc = #doc_string_u32]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_u32 {
                ($value:literal) => {{
                    #quantity_path($value as u32, #unit_ident, u32)
                }};
            }
            pub use #inner_u32 as #unit_u32;

            #[doc = #doc_string_u64]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_u64 {
                ($value:literal) => {{
                    #quantity_path($value as u64, #unit_ident, u64)
                }};
            }
            pub use #inner_u64 as #unit_u64;
        });

        // Create shortname macro for float module using #quantity_path macro directly
        float_macros.push(quote! {
            #[doc = #base_doc_string]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_short_float {
                ($value:literal) => {{
                    #quantity_path($value as f64, #unit_ident, f64)
                }};
            }
            pub use #inner_short_float as #unit_ident;
        });

        // Create shortname macro for int module using #quantity_path macro directly
        integer_macros.push(quote! {
            #[doc = #doc_string_i32]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_short_int {
                ($value:literal) => {{
                    #quantity_path($value as i32, #unit_ident, i32)
                }};
            }
            pub use #inner_short_int as #unit_ident;
        });
    }

    if for_namespace {
        // For namespace use, generate just the float and integer submodules without outer wrapper
        quote! {
            #[allow(unused_macros)]
            pub mod float {
                #(#float_macros)*
            }

            #[allow(unused_macros)]
            pub mod integer {
                #(#integer_macros)*
            }
        }
    } else {
        // For regular use, generate the full module structure
        let module_ident = syn::Ident::new(module_name, proc_macro2::Span::mixed_site());

        quote! {
            #[allow(unused_macros)]
            /// Custom literal declarator sugar for the [quantity!](crate::quantity!) macro, for use with
            /// the [culit](https://crates.io/crates/culit) crate.
            ///
            /// ```rust
            /// #[culit::culit(whippyunits::default_declarators::literals)]
            /// fn main() {
            ///     let distance = 1.0m;
            /// }
            /// ```
            ///
            /// Literal declarators are effectively macro sugar for the [quantity!](crate::quantity!) macro.  The following
            /// are equivalent:
            ///
            /// ```rust
            /// # #[culit::culit(whippyunits::default_declarators::literals)]
            /// # fn main() {
            /// let distance = 1.0m;
            /// let distance = whippyunits::default_declarators::literals::float::m!(1.0);
            /// let distance = whippyunits::quantity!(1.0, m);
            /// # }
            /// ```
            ///
            /// Backing numeric types are inferred from the type of the literal, but can be overridden by suffixing the literal:
            ///
            /// ```rust
            /// # #[culit::culit(whippyunits::default_declarators::literals)]
            /// # fn main() {
            /// let distance = 1.0m; // f64 (default for float literals)
            /// let energy = 1.0J_f32; // f32
            /// let time = 5ms; // i32 (default for integer literals)
            /// # }
            /// ```
            ///
            /// Because literal syntax is somewhat restrictive, we do not support the full set of algebraically-possible
            /// unit expressions in literal position; derived units without an established unit symbol (e.g. `m/s`) are
            /// not supported.  For arbitrary algebraic expressions, use the [quantity!](crate::quantity!) macro instead.
            pub mod #module_ident {
                #[allow(unused_macros)]
                pub mod float {
                    #(#float_macros)*
                }

                #[allow(unused_macros)]
                pub mod integer {
                    #(#integer_macros)*
                }
            }
        }
    }
}

