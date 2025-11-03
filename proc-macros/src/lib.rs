#![feature(trait_alias)]
#![allow(mixed_script_confusables)]

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod compute_unit_dimensions_macro;
mod define_generic_dimension_macro;
mod define_literals_macro;
mod define_local_quantity_macro;
mod define_unit_declarators_macro;
mod generate_all_radian_erasures_macro;
mod generate_default_declarators_macro;
mod generate_literals_module_macro;
mod generate_local_unit_literals_macro;
mod local_unit_type_macro;
mod pow_lookup_macro;
mod unit_macro;

mod utils {
    pub mod culit;
    pub mod dimension_suggestions;
    pub mod lift_trace;
    pub mod literal_macros;
    pub mod scale_suggestions;
    pub mod shared_utils;
    pub mod unit_suggestions;
}

#[proc_macro]
#[doc(hidden)]
pub fn compute_unit_dimensions(input: TokenStream) -> TokenStream {
    compute_unit_dimensions_macro::compute_unit_dimensions(input)
}

#[proc_macro]
pub fn define_generic_dimension(input: TokenStream) -> TokenStream {
    let input =
        parse_macro_input!(input as define_generic_dimension_macro::DefineGenericDimensionInput);
    input.expand().into()
}

#[proc_macro]
pub fn proc_unit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as unit_macro::UnitMacroInput);
    input.expand().into()
}

#[proc_macro]
#[doc(hidden)]
pub fn local_unit_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as local_unit_type_macro::LocalQuantityMacroInput);
    input.expand().into()
}

#[proc_macro]
pub fn define_literals(input: TokenStream) -> TokenStream {
    define_literals_macro::define_literals(input)
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
    let input =
        parse_macro_input!(input as generate_all_radian_erasures_macro::AllRadianErasuresInput);
    input.expand().into()
}

/// Generate default declarators using the source of truth from whippyunits-core
/// Usage: generate_default_declarators!()
#[proc_macro]
#[doc(hidden)]
pub fn generate_default_declarators(input: TokenStream) -> TokenStream {
    let input =
        parse_macro_input!(input as generate_default_declarators_macro::DefaultDeclaratorsInput);
    input.expand().into()
}

/// Generate literals module for culit integration
/// Usage: generate_literals_module!()
#[proc_macro]
#[doc(hidden)]
pub fn generate_literals_module(input: TokenStream) -> TokenStream {
    generate_literals_module_macro::generate_literals_module(input)
}

/// Generate local unit literals namespace with lift trace documentation
#[proc_macro]
#[doc(hidden)]
pub fn generate_local_unit_literals(input: TokenStream) -> TokenStream {
    let input =
        parse_macro_input!(input as generate_local_unit_literals_macro::LocalUnitLiteralsInput);
    input.expand().into()
}

/// Define a local quantity trait and implementations for a given scale and set of units.
///
/// This is an internal macro used by define_unit_declarators! to generate the trait definitions.
/// Based on the original scoped_preferences.rs implementation.
#[proc_macro]
#[doc(hidden)]
pub fn define_local_quantity(input: TokenStream) -> TokenStream {
    define_local_quantity_macro::define_local_quantity(input)
}

/// Define a set of declarators that auto-convert to a given set of base units.
///
/// See [`define_unit_declarators`] for full documentation.
#[proc_macro]
pub fn define_unit_declarators(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as define_unit_declarators_macro::DefineBaseUnitsInput);
    input.expand().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        // Test that the macro can parse valid input
        let input = "LengthOrMass, Length, Mass";
        let parsed =
            syn::parse_str::<define_generic_dimension_macro::DefineGenericDimensionInput>(input);
        assert!(parsed.is_ok());

        let parsed = parsed.unwrap();
        assert_eq!(parsed.trait_name.to_string(), "LengthOrMass");
        assert_eq!(parsed.dimension_exprs.len(), 2);
    }

    #[test]
    fn test_expand_macro() {
        // Test that the macro expands without panicking
        let input = syn::parse_str::<define_generic_dimension_macro::DefineGenericDimensionInput>(
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
