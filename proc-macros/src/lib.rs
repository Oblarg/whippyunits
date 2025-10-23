#![feature(trait_alias)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod culit_macro;
mod default_declarators_macro;
mod define_base_units_macro;
mod define_generic_dimension;
mod lift_trace;
mod local_unit_literals_macro;
mod local_unit_macro;
mod pow_lookup_macro;
mod radian_erasure_macro;
mod shared_utils;
mod unit_macro;

/// Shared helper function to get the corresponding default declarator type for a unit
/// This is used by both the unit! macro and local_unit! macro to avoid code duplication
fn get_declarator_type_for_unit(unit_name: &str) -> Option<proc_macro2::TokenStream> {
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
            return Some(quote! {
                whippyunits::default_declarators::#type_ident
            });
        }
    }

    // Check if it's a prefixed unit FIRST (before checking unit literals)
    let (prefix, base) = parse_unit_with_prefix_direct(unit_name);
    if let Some(prefix) = prefix {
        // First try to find it as a prefixed base unit
        for dimension in whippyunits_core::Dimension::BASIS {
            if let Some(unit) = dimension
                .units
                .iter()
                .find(|u| u.symbols.contains(&base.as_str()))
            {
                // Use the same naming convention as the default declarators macro
                let unit_singular = unit.name.trim_end_matches('s');
                let combined_name = format!("{}{}", prefix.name(), unit_singular);
                let type_name = whippyunits_core::CapitalizedFmt(&combined_name).to_string();
                let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
                return Some(quote! {
                    whippyunits::default_declarators::#type_ident
                });
            }
        }

        // If not a base unit, try to find it as a prefixed unit literal
        if let Some((_dimension, unit)) = lookup_unit_literal_direct(&base) {
            // Use the same naming convention as the default declarators macro
            let unit_singular = unit.name.trim_end_matches('s');
            let combined_name = format!("{}{}", prefix.name(), unit_singular);
            let type_name = whippyunits_core::CapitalizedFmt(&combined_name).to_string();
            let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
            return Some(quote! {
                whippyunits::default_declarators::#type_ident
            });
        }
    }

    // Check if it's a unit literal that has a corresponding type - only if not a prefixed unit
    if let Some((_dimension, unit)) = lookup_unit_literal_direct(unit_name) {
        // Use the long name to generate the type name, matching the declarator generation logic
        let type_name = whippyunits_core::CapitalizedFmt(unit.name).to_string();
        let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
        return Some(quote! {
            whippyunits::default_declarators::#type_ident
        });
    }

    // For compound units (N, J, Pa, W, V, F, C, etc.) and dimensionless units (1),
    // we don't generate documentation since they don't have corresponding default declarator types
    None
}

// Helper functions that replace api_helpers functions with direct whippyunits-core calls

/// Look up a unit literal (like "min", "h", "g", "m", "s", etc.) in the dimensions data
fn lookup_unit_literal_direct(
    unit_name: &str,
) -> Option<(
    &'static whippyunits_core::Dimension,
    &'static whippyunits_core::Unit,
)> {
    // First try to find by symbol
    if let Some((unit, dimension)) = whippyunits_core::Dimension::find_unit_by_symbol(unit_name) {
        return Some((dimension, unit));
    }

    // Then try to find by name
    if let Some((unit, dimension)) = whippyunits_core::Dimension::find_unit_by_name(unit_name) {
        return Some((dimension, unit));
    }

    None
}

/// Parse a unit name to extract prefix and base unit
/// Returns (prefix_option, base_unit_name)
fn parse_unit_with_prefix_direct(
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

/// Get all unit symbols that should have literal macros
/// This is the single source of truth for what units should have custom literals
/// Used by both the regular define_literals!() and local unit literals
fn get_all_unit_symbols_for_literals() -> Vec<String> {
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

/// Generate literal macros module - generic function that works for both default and local modes
/// 
/// # Parameters
/// - `module_name`: Name of the module to generate (e.g., "custom_literal" or "local_unit_literals")
/// - `is_local_mode`: If true, uses local quantity! macro (no prefix); if false, uses whippyunits::quantity!
/// - `scale_params`: Optional scale parameters for lift trace (only used if is_local_mode is true)
/// - `for_namespace`: If true, generates just the float/integer submodules without outer wrapper
/// - `namespace_ident`: Optional namespace identifier for disambiguating local macros
fn generate_literal_macros_module(
    module_name: &str,
    is_local_mode: bool,
    scale_params: Option<(syn::Ident, syn::Ident, syn::Ident, syn::Ident, syn::Ident, syn::Ident, syn::Ident, syn::Ident)>,
    for_namespace: bool,
    namespace_ident: Option<syn::Ident>,
) -> proc_macro2::TokenStream {
    // Get all unit symbols using the shared function
    let unit_symbols = get_all_unit_symbols_for_literals();
    
    // Determine the correct quantity! path based on mode
    let quantity_path = if is_local_mode {
        // Local mode: use the local quantity! macro (no prefix, picks up from scope)
        quote! { quantity! }
    } else {
        // Default mode: always use whippyunits::quantity!
        quote! { whippyunits::quantity! }
    };
    
    
    let mut float_macros = Vec::new();
    let mut integer_macros = Vec::new();

    // Generate literal macros for each unit symbol with both float and integer variants
    for unit_symbol in &unit_symbols {
        let unit_ident = syn::Ident::new(unit_symbol, proc_macro2::Span::mixed_site());
        
        // Generate documentation based on mode
        let doc_string = if is_local_mode {
            if let Some((mass_scale, length_scale, time_scale, current_scale, temperature_scale, amount_scale, luminosity_scale, angle_scale)) = &scale_params {
                let local_context = crate::lift_trace::LocalContext {
                    mass_scale: mass_scale.clone(),
                    length_scale: length_scale.clone(),
                    time_scale: time_scale.clone(),
                    current_scale: current_scale.clone(),
                    temperature_scale: temperature_scale.clone(),
                    amount_scale: amount_scale.clone(),
                    luminosity_scale: luminosity_scale.clone(),
                    angle_scale: angle_scale.clone(),
                };
                let transformation_details = local_context.get_transformation_details_for_identifier(unit_symbol);
                // Use the EXACT SAME logic as local_unit_macro
                let lines: Vec<&str> = transformation_details.details.lines().collect();
                let mut formatted_details = String::new();
                for (j, line) in lines.iter().enumerate() {
                    formatted_details.push_str(line);
                    if j < lines.len() - 1 {
                        formatted_details.push_str("<br>");
                    }
                }
                formatted_details
            } else {
                format!("/// Local unit literal for `{}`", unit_symbol)
            }
        } else {
            format!("/// Unit literal for `{}`", unit_symbol)
        };

        // Generate unique inner names for each macro to avoid conflicts
        // For local mode, prefix with the namespace identifier to disambiguate between different local scales
        let inner_prefix = if is_local_mode {
            if let Some(namespace) = &namespace_ident {
                format!("{}_{}", namespace, unit_symbol)
            } else {
                unit_symbol.clone()
            }
        } else {
            unit_symbol.clone()
        };
        
        let inner_f64 = syn::Ident::new(&format!("{}_f64", inner_prefix), proc_macro2::Span::mixed_site());
        let inner_f32 = syn::Ident::new(&format!("{}_f32", inner_prefix), proc_macro2::Span::mixed_site());
        let inner_i32 = syn::Ident::new(&format!("{}_i32", inner_prefix), proc_macro2::Span::mixed_site());
        let inner_i64 = syn::Ident::new(&format!("{}_i64", inner_prefix), proc_macro2::Span::mixed_site());
        let inner_u32 = syn::Ident::new(&format!("{}_u32", inner_prefix), proc_macro2::Span::mixed_site());
        let inner_u64 = syn::Ident::new(&format!("{}_u64", inner_prefix), proc_macro2::Span::mixed_site());

        // Generate float variants
        let unit_f64 = syn::Ident::new(&format!("{}_f64", unit_symbol), proc_macro2::Span::mixed_site());
        let unit_f32 = syn::Ident::new(&format!("{}_f32", unit_symbol), proc_macro2::Span::mixed_site());
        
        float_macros.push(quote! {
            #[doc = #doc_string]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_f64 {
                ($value:literal) => {{
                    #quantity_path($value as f64, #unit_ident, f64)
                }};
            }
            pub use #inner_f64 as #unit_f64;
            
            #[doc = #doc_string]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_f32 {
                ($value:literal) => {{
                    #quantity_path($value as f32, #unit_ident, f32)
                }};
            }
            pub use #inner_f32 as #unit_f32;
        });
        
        // Generate integer variants
        let unit_i32 = syn::Ident::new(&format!("{}_i32", unit_symbol), proc_macro2::Span::mixed_site());
        let unit_i64 = syn::Ident::new(&format!("{}_i64", unit_symbol), proc_macro2::Span::mixed_site());
        let unit_u32 = syn::Ident::new(&format!("{}_u32", unit_symbol), proc_macro2::Span::mixed_site());
        let unit_u64 = syn::Ident::new(&format!("{}_u64", unit_symbol), proc_macro2::Span::mixed_site());
        
        integer_macros.push(quote! {
            #[doc = #doc_string]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_i32 {
                ($value:literal) => {{
                    #quantity_path($value as i32, #unit_ident, i32)
                }};
            }
            pub use #inner_i32 as #unit_i32;
            
            #[doc = #doc_string]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_i64 {
                ($value:literal) => {{
                    #quantity_path($value as i64, #unit_ident, i64)
                }};
            }
            pub use #inner_i64 as #unit_i64;
            
            #[doc = #doc_string]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_u32 {
                ($value:literal) => {{
                    #quantity_path($value as u32, #unit_ident, u32)
                }};
            }
            pub use #inner_u32 as #unit_u32;
            
            #[doc = #doc_string]
            #[macro_export]
            #[doc(hidden)]
            macro_rules! #inner_u64 {
                ($value:literal) => {{
                    #quantity_path($value as u64, #unit_ident, u64)
                }};
            }
            pub use #inner_u64 as #unit_u64;
        });
    }

    // Generate shortname macros for all units (like the culit_macro does)
    for unit_symbol in &unit_symbols {
        let unit_ident = syn::Ident::new(unit_symbol, proc_macro2::Span::mixed_site());

        // Generate documentation based on mode
        let doc_string = if is_local_mode {
            if let Some((mass_scale, length_scale, time_scale, current_scale, temperature_scale, amount_scale, luminosity_scale, angle_scale)) = &scale_params {
                let local_context = crate::lift_trace::LocalContext {
                    mass_scale: mass_scale.clone(),
                    length_scale: length_scale.clone(),
                    time_scale: time_scale.clone(),
                    current_scale: current_scale.clone(),
                    temperature_scale: temperature_scale.clone(),
                    amount_scale: amount_scale.clone(),
                    luminosity_scale: luminosity_scale.clone(),
                    angle_scale: angle_scale.clone(),
                };
                let transformation_details = local_context.get_transformation_details_for_identifier(unit_symbol);
                // Use the EXACT SAME logic as local_unit_macro
                let lines: Vec<&str> = transformation_details.details.lines().collect();
                let mut formatted_details = String::new();
                for (j, line) in lines.iter().enumerate() {
                    formatted_details.push_str(line);
                    if j < lines.len() - 1 {
                        formatted_details.push_str("<br>");
                    }
                }
                formatted_details
            } else {
                format!("/// Local unit literal for `{}`", unit_symbol)
            }
        } else {
            format!("/// Unit literal for `{}`", unit_symbol)
        };

        // Generate unique inner names for shortname macros to avoid conflicts
        // For local mode, prefix with the namespace identifier to disambiguate between different local scales
        let inner_prefix = if is_local_mode {
            if let Some(namespace) = &namespace_ident {
                format!("{}_{}", namespace, unit_symbol)
            } else {
                unit_symbol.clone()
            }
        } else {
            unit_symbol.clone()
        };
        
        let inner_short_float = syn::Ident::new(&format!("{}_float", inner_prefix), proc_macro2::Span::mixed_site());
        let inner_short_int = syn::Ident::new(&format!("{}_int", inner_prefix), proc_macro2::Span::mixed_site());

        // Create shortname macro for float module using #quantity_path macro directly
        float_macros.push(quote! {
            #[doc = #doc_string]
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
            #[doc = #doc_string]
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

/// Computes unit dimensions for a unit expression.
///
/// Usage: `compute_unit_dimensions!(unit_expr)`
/// Returns a tuple of 12 i16 values representing the dimensions
#[proc_macro]
#[doc(hidden)]
pub fn compute_unit_dimensions(input: TokenStream) -> TokenStream {
    let unit_expr: unit_macro::UnitExpr = syn::parse(input).expect("Expected unit expression");

    let result = unit_expr.evaluate();

    // Extract individual values for the quote
    let (d0, d1, d2, d3, d4, d5, d6, d7) = (
        result.dimension_exponents.0[0],
        result.dimension_exponents.0[1],
        result.dimension_exponents.0[2],
        result.dimension_exponents.0[3],
        result.dimension_exponents.0[4],
        result.dimension_exponents.0[5],
        result.dimension_exponents.0[6],
        result.dimension_exponents.0[7],
    );
    let (d8, d9, d10, d11) = (
        result.scale_exponents.0[0],
        result.scale_exponents.0[1],
        result.scale_exponents.0[2],
        result.scale_exponents.0[3],
    );

    quote! {
        (
            whippyunits_core::DynDimensionExponents([#d0, #d1, #d2, #d3, #d4, #d5, #d6, #d7]),
            whippyunits_core::ScaleExponents([#d8, #d9, #d10, #d11])
        )
    }
    .into()
}

/// Defines a trait representing a scale-generic dimension (like Length, Area, Energy).
///
/// Generic dimensions can be used to write arithmetic operations that are generic over a dimensional structure
/// or disjunction of dimensional structures.
///
/// ## Syntax
///
/// ```rust
/// define_generic_dimension!(TraitName, DimensionExpression);
/// ```
///
/// Where:
/// - `TraitName`: The name of the trait to create
/// - `DimensionExpression`: A comma-separated list of "dimension literal expressions".
///     - A "dimension literal expression" is either:
///         - An atomic dimension:
///             - `Length`, `Time`, `Mass`, `Current`, `Temperature`, `Amount`, `Luminosity`, `Angle`
///             - Also accepts single-character symbols: `L`, `T`, `M`, `I`, `Θ`, `N`, `J`, `A`
///         - A multiplication of two or more atomic dimensions:
///             - `M * L` or `M.L` (UCUM style)
///         - A division of two or more atomic dimensions:
///             - `L / T`
///         - An exponentiation of an atomic dimension:
///             - `L^2`, `T^-1` or `L2`, `T^-1` (UCUM implicit exponent notation)
///         - A combination of the above: `M * L^2 / T^2` or `M.L2/T^2` (UCUM style)
///
/// ## Examples
///
/// ```rust
/// use whippyunits::{define_generic_dimension, quantity};
/// use core::ops::Mul;
///
/// // Define a generic Area trait using UCUM syntax
/// define_generic_dimension!(Area, L2);
///
/// // Define a generic Energy trait using UCUM syntax
/// define_generic_dimension!(Energy, M.L2/T^2);
///
/// // Define a velocity that may be *either* linear or angular
/// define_generic_dimension!(Velocity, L/T, A/T);
///
/// // Now you can write generic functions
/// fn calculate_area<D1: Length, D2: Length>(d1: D1, d2: D2) -> impl Area
/// where
///     D1: Mul<D2>,
/// {
///     d1 * d2
/// }
///
/// // This works with any length units
/// let area1: impl Area = calculate_area(1.0.meters(), 2.0.meters());
/// let area2: impl Area = calculate_area(100.0.centimeters(), 200.0.centimeters());
/// let area3: impl Area = calculate_area(1.0.meters(), 200.0.centimeters());
/// ```
#[proc_macro]
pub fn define_generic_dimension(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as define_generic_dimension::DefineGenericDimensionInput);
    input.expand().into()
}

/// Creates a concrete Quantity type from a unit expression.
///
/// This is particularly useful for constraining the result of potentially-type-ambiguous operations,
/// such as multiplication of two quantities with different dimensions.  If you want to construct a
/// quantity with a known value, use the `#quantity_path` macro instead.
///
/// ## Syntax
///
/// ```rust
/// proc_unit!(unit_expr);
/// proc_unit!(unit_expr, storage_type);
/// ```
///
/// Where:
/// - `unit_expr`: A "unit literal expression"
///     - A "unit literal expression" is either:
///         - An atomic unit:
///             - `m`, `kg`, `s`, `A`, `K`, `mol`, `cd`, `rad`
///         - A multiplication of two or more atomic units:
///             - `m * kg`
///         - A division of two or more atomic units:
///             - `m / s`
///         - An exponentiation of an atomic unit:
///             - `m^2`, `s^-1`
///         - A combination of the above:
///             - `m * kg / s^2`
/// - `storage_type`: An optional storage type for the quantity. Defaults to `f64`.
///
/// ## Examples
///
/// ```rust
/// use whippyunits::unit;
///
/// // Constrain a multiplication to compile error if the units are wrong:
/// let area = 5.0m * 5.0m; // ⚠️ Correct, but unchecked; will compile regardless of the units
/// let area = 5.0m * 5.0s; // ❌ BUG: compiles fine, but is not an area
/// let area: unit!(m^2) = 5.0m * 5.0m; // ✅ Correct, will compile only if the units are correct
/// let area: unit!(m^2) = 5.0m * 5.0s; // Compile error, as expected
///
/// // Specify the target dimension of a rescale operation:
/// let area: unit!(mm) = rescale(5.0m); // 5000.0 mm
/// ```
#[proc_macro]
pub fn proc_unit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as unit_macro::UnitMacroInput);
    input.expand().into()
}

#[proc_macro]
#[doc(hidden)]
pub fn local_unit_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as local_unit_macro::LocalQuantityMacroInput);
    input.expand().into()
}

/// Defines custom literal declarators using [culit](https://crates.io/crates/culit).
///
/// By default, this places the literal declarators in the `custom_literal` module.  If
/// a custom module name is provided, the literal declarators will be placed in that module.
/// Culit can be pointed to a specific module by passing the module name to the culit 
/// attribute, e.g. #[culit::culit(unit_literals)].
///
/// ```rust
/// // this must be called at least once in the user's crate, typically
/// // at the crate root
/// whippyunits::define_literals!();
///
/// // optionally, specify a custom module name to avoid conflicts
/// whippyunits::define_literals!(unit_literals);
///
/// // following this, literal declarators are available in any scope tagged with
/// // #[culit::culit]
/// #[culit::culit]
/// fn example() {
///     let distance = 1.0m;
///     let energy = 1.0J_f32;
///     let time = 5ms;
///     let mass = 10mg_i16;
/// }
/// ```
///
/// Literal declarators are effectively macro sugar for the [#quantity_path](crate::#quantity_path) macro.  The following
/// are equivalent:
///
/// ```rust
/// let distance = 1.0m;
/// let distance = custom_literal::float::m(1.0);
/// let distance = #quantity_path(1.0, m);
/// ```
///
/// Backing numeric types are inferred from the type of the literal, but can be overridden by suffixing the literal:
///
/// ```rust
/// let distance = 1.0m; // f64 (default for float literals)
/// let energy = 1.0J_f32; // f32
/// let time = 5ms; // i32 (default for integer literals)
/// let mass = 10mg_i16; // i16
/// ```
///
/// Because literal syntax is somewhat restrictive, we do not support the full set of algebraically-possible
/// unit expressions in literal position; derived units without an established unit symbol (e.g. `m/s`) are
/// not supported.  For arbitrary algebraic expressions, use the [#quantity_path](crate::#quantity_path) macro instead.
///
/// ## Note
///
/// Must be called once in your crate, typically at the module level.
/// The generated literals are only available in scopes tagged with `#[culit::culit]`.
#[proc_macro]
pub fn define_literals(input: TokenStream) -> TokenStream {
    // Parse the input to see if a module name is provided
    let module_name = if input.is_empty() {
        "custom_literal".to_string()
    } else {
        // Parse as an identifier for the module name
        match syn::parse::<syn::Ident>(input) {
            Ok(ident) => ident.to_string(),
            Err(_) => {
                // If parsing fails, use default
                "custom_literal".to_string()
            }
        }
    };
    
    let custom_literal_module = culit_macro::generate_custom_literal_module_with_name(&module_name);
    TokenStream::from(custom_literal_module)
}

/// Generate exponentiation lookup tables with parametric range
/// Usage: pow_lookup!(base: 2, range: -20..=20, type: rational)
#[proc_macro]
#[doc(hidden)]
pub fn pow_lookup(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as pow_lookup_macro::PowLookupInput);
    input.expand().into()
}

/// Generate π exponentiation lookup tables with rational approximation
/// Usage: pow_pi_lookup!(range: -10..=10, type: rational)
#[proc_macro]
#[doc(hidden)]
pub fn pow_pi_lookup(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as pow_lookup_macro::PiPowLookupInput);
    input.expand().into()
}

/// Generate all radian erasure implementations (both to scalar and to dimensionless quantities)
/// Usage: generate_all_radian_erasures!()
#[proc_macro]
#[doc(hidden)]
pub fn generate_all_radian_erasures(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as radian_erasure_macro::AllRadianErasuresInput);
    input.expand().into()
}

/// Generate default declarators using the source of truth from whippyunits-core
/// Usage: generate_default_declarators!()
#[proc_macro]
#[doc(hidden)]
pub fn generate_default_declarators(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as default_declarators_macro::DefaultDeclaratorsInput);
    input.expand().into()
}

/// Generate literals module for culit integration
/// Usage: generate_literals_module!()
#[proc_macro]
#[doc(hidden)]
pub fn generate_literals_module(_input: TokenStream) -> TokenStream {
    let literals_module = generate_literal_macros_module("literals", false, None, false, None);
    literals_module.into()
}

/// Generate local unit literals namespace with lift trace documentation
#[proc_macro]
#[doc(hidden)]
pub fn generate_local_unit_literals(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as local_unit_literals_macro::LocalUnitLiteralsInput);
    input.expand().into()
}

/// Define a local quantity trait and implementations for a given scale and set of units.
///
/// This is an internal macro used by define_base_units! to generate the trait definitions.
/// Based on the original scoped_preferences.rs implementation.
#[proc_macro]
#[doc(hidden)]
pub fn define_local_quantity(_input: TokenStream) -> TokenStream {
    // This macro should be called from within the define_base_units macro
    // It generates the trait and implementations based on the original pattern
    quote! {
        // This will be expanded by the define_base_units macro
        // The actual implementation is in the define_base_units_macro.rs file
    }.into()
}

/// Define a set of declarators that auto-convert to a given set of base units.
///
/// ## Syntax
///
/// ```rust
/// define_base_units!(
///     $mass_scale:ident,
///     $length_scale:ident,
///     $time_scale:ident,
///     $current_scale:ident,
///     $temperature_scale:ident,
///     $amount_scale:ident,
///     $luminosity_scale:ident,
///     $angle_scale:ident,
///     $namespace:ident
/// );
/// ```
/// 
/// where: 
/// - $mass_scale: The scale for mass units (full unit name, e.g. "Kilogram")
/// - $length_scale: The scale for length units (full unit name, e.g. "Kilometer")
/// - $time_scale: The scale for time units (full unit name, e.g. "Second")
/// - $current_scale: The scale for current units (full unit name, e.g. "Ampere")
/// - $temperature_scale: The scale for temperature units (full unit name, e.g. "Kelvin")
/// - $amount_scale: The scale for amount units (full unit name, e.g. "Mole")
/// - $luminosity_scale: The scale for luminosity units (full unit name, e.g. "Candela")
/// - $angle_scale: The scale for angle units (full unit name, e.g. "Radian")
/// - $namespace: The name for the declarator module
///
/// ## Usage
///
/// ```rust
/// define_base_units!(Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian, local_scale);
/// 
/// // autoconverting literals are available in the inner "literals" module
/// #[culit::culit(local_scale::literals)]
/// fn example() {
///     // trait declarators and the quantity! macro are available in the module
///     use local_scale::*;
///     let distance = 1.0.meters(); // automatically stores as 1000.0 millimeters
///     let distance = quantity!(1.0, m); // so does this
///     let distance = 1.0m; // and so does this!
/// 
///     // compound/derived units are "lifted" to the provided scale preferences
///     let energy = 1.0J; // kg * mm^2 / s^2 yields microJoules, so this stores as 1000.0 * 1000.0 microJoules
/// }
/// ```
/// 
/// Hovering on unit identifiers or literals will provide documentation on the auto-conversion, showing both the
/// declared unit and the unit to which it is converted, along with a detailed trace of the conversion chain.
#[proc_macro]
pub fn define_base_units(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as define_base_units_macro::DefineBaseUnitsInput);
    input.expand().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        // Test that the macro can parse valid input
        let input = "LengthOrMass, Length, Mass";
        let parsed = syn::parse_str::<define_generic_dimension::DefineGenericDimensionInput>(input);
        assert!(parsed.is_ok());

        let parsed = parsed.unwrap();
        assert_eq!(parsed.trait_name.to_string(), "LengthOrMass");
        assert_eq!(parsed.dimension_exprs.len(), 2);
    }

    #[test]
    fn test_expand_macro() {
        // Test that the macro expands without panicking
        let input = syn::parse_str::<define_generic_dimension::DefineGenericDimensionInput>(
            "LengthOrMass, Length, Mass",
        )
        .unwrap();

        let expanded = input.expand();
        // The expanded code should contain the trait name
        let expanded_str = expanded.to_string();
        assert!(expanded_str.contains("LengthOrMass"));
        assert!(expanded_str.contains("trait"));
    }
}
