#![feature(trait_alias)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod culit_macro;
mod define_generic_dimension;
mod local_unit_macro;
mod pow_lookup_macro;
mod unit_macro;
mod radian_erasure_macro;
mod default_declarators_macro;
mod scoped_preferences_macro;

/// Shared helper function to get the corresponding default declarator type for a unit
/// This is used by both the unit! macro and local_unit! macro to avoid code duplication
fn get_declarator_type_for_unit(unit_name: &str) -> Option<proc_macro2::TokenStream> {
    use whippyunits_default_dimensions::{BASE_UNITS, lookup_unit_literal, parse_unit_with_prefix, SI_PREFIXES};

    // Skip dimensionless units - they don't have corresponding default declarator types
    if unit_name == "dimensionless" {
        return None;
    }
    
    // Check if it's a base unit (these have corresponding types)
    if let Some(base_unit) = BASE_UNITS.iter().find(|u| u.symbol == unit_name) {
        let type_name = whippyunits_default_dimensions::util::capitalize_first(&base_unit.long_name);
        let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
        return Some(quote! {
            whippyunits::default_declarators::#type_ident
        });
    }
    
    // Check if it's a prefixed unit FIRST (before checking unit literals)
    let (prefix, base) = parse_unit_with_prefix(unit_name);
    if let Some(prefix_symbol) = prefix {
        // First try to find it as a prefixed base unit
        if let Some(base_unit) = BASE_UNITS.iter().find(|u| u.symbol == base) {
            // Get the prefix long name for proper type naming
            if let Some(prefix_info) = SI_PREFIXES.iter().find(|p| p.symbol == prefix_symbol) {
                // Use the same naming convention as the default declarators macro
                let unit_singular = base_unit.long_name.trim_end_matches('s');
                let combined_name = format!("{}{}", prefix_info.long_name, unit_singular);
                let type_name = whippyunits_default_dimensions::util::capitalize_first(&combined_name);
                let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
                return Some(quote! {
                    whippyunits::default_declarators::#type_ident
                });
            }
        }
        
        // If not a base unit, try to find it as a prefixed unit literal
        if let Some((_dimension, unit)) = lookup_unit_literal(&base) {
            if let Some(prefix_info) = SI_PREFIXES.iter().find(|p| p.symbol == prefix_symbol) {
                // Use the same naming convention as the default declarators macro
                let unit_singular = unit.long_name.trim_end_matches('s');
                let combined_name = format!("{}{}", prefix_info.long_name, unit_singular);
                let type_name = whippyunits_default_dimensions::util::capitalize_first(&combined_name);
                let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
                return Some(quote! {
                    whippyunits::default_declarators::#type_ident
                });
            }
        }
    }
    
    // Check if it's a unit literal that has a corresponding type - only if not a prefixed unit
    if let Some((_dimension, unit)) = lookup_unit_literal(unit_name) {
        // Use the long name to generate the type name, matching the declarator generation logic
        let type_name = whippyunits_default_dimensions::util::capitalize_first(unit.long_name);
        let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
        return Some(quote! {
            whippyunits::default_declarators::#type_ident
        });
    }
    
    // For compound units (N, J, Pa, W, V, F, C, etc.) and dimensionless units (1), 
    // we don't generate documentation since they don't have corresponding default declarator types
    None
}

/// Computes unit dimensions for a unit expression.
///
/// Usage: `compute_unit_dimensions!(unit_expr)`
/// Returns a tuple of 12 i16 values representing the dimensions
#[proc_macro]
#[doc(hidden)]
pub fn compute_unit_dimensions(input: TokenStream) -> TokenStream {
    let unit_expr: unit_macro::UnitExpr = syn::parse(input).expect("Expected unit expression");

    let dimensions = unit_expr.evaluate();

    let (d0, d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11) = dimensions;
    quote! {
        (#d0, #d1, #d2, #d3, #d4, #d5, #d6, #d7, #d8, #d9, #d10, #d11)
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
///             - `M * L`
///         - A division of two or more atomic dimensions:
///             - `L / T`
///         - An exponentiation of an atomic dimension:
///             - `L^2`, `T^-1`
///         - A combination of the above: `M * L^2 / T^2`
///
/// ## Examples
///
/// ```rust
/// use whippyunits::{define_generic_dimension, quantity};
/// use core::ops::Mul;
///
/// // Define a generic Area trait
/// define_generic_dimension!(Area, Length^2);
///
/// // Define a generic Energy trait
/// define_generic_dimension!(Energy, M * L^2 / T^2);
/// 
/// // Define a velocity that may be *either* linear or angular
/// define_generic_dimension!(Velocity, L / T, A / T);
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
/// quantity with a known value, use the `quantity!` macro instead.
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
/// Culit is designed to look for literal implementations in the scope of the crate in which
/// the literal is used; accordingly, this macro must be called in the user's crate to generate
/// the necessary `custom_literal` module and corresponding macro implementations:
/// 
/// ```rust
/// // this must be called at least once in the user's crate, typically 
/// // at the crate root
/// whippyunits::define_literals!();
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
/// Literal declarators are effectively macro sugar for the [quantity!](crate::quantity!) macro.  The following
/// are equivalent:
/// 
/// ```rust
/// let distance = 1.0m;
/// let distance = custom_literal::float::m(1.0);
/// let distance = quantity!(1.0, m);
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
/// Literal declarators will use *whatever version of the quantity! macro is in scope*, so if you have changed
/// the local base units via [define_base_units!](crate::define_base_units!), the literal syntax will
/// automatically use the appropriately-scaled declarators.
/// 
/// Because literal syntax is somewhat restrictive, we do not support the full set of algebraically-possible
/// unit expressions in literal position; derived units without an established unit symbol (e.g. `m/s`) are 
/// not supported.  For arbitrary algebraic expressions, use the [quantity!](crate::quantity!) macro instead.
/// 
/// ## Note
///
/// Must be called once in your crate, typically at the module level.
/// The generated literals are only available in scopes tagged with `#[culit::culit]`.
#[proc_macro]
pub fn define_literals(_input: TokenStream) -> TokenStream {
    let custom_literal_module = culit_macro::generate_custom_literal_module();
    TokenStream::from(custom_literal_module)
}

#[proc_macro]
#[doc(hidden)]
pub fn generate_scoped_preferences(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as scoped_preferences_macro::ScopedPreferencesInput);
    input.expand().into()
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

/// Generate default declarators using the source of truth from default-dimensions
/// Usage: generate_default_declarators!()
#[proc_macro]
#[doc(hidden)]
pub fn generate_default_declarators(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as default_declarators_macro::DefaultDeclaratorsInput);
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
