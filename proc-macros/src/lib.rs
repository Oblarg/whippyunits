use proc_macro::TokenStream;
use syn::parse_macro_input;

mod define_generic_dimension;
mod unit_macro;
mod local_quantity_macro;

#[proc_macro]
pub fn define_generic_dimension(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as define_generic_dimension::DefineGenericDimensionInput);
    input.expand().into()
}

#[proc_macro]
pub fn proc_unit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as unit_macro::UnitMacroInput);
    input.expand().into()
}

#[proc_macro]
pub fn local_unit_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as local_quantity_macro::LocalQuantityMacroInput);
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
            "LengthOrMass, Length, Mass"
        ).unwrap();
        
        let expanded = input.expand();
        // The expanded code should contain the trait name
        let expanded_str = expanded.to_string();
        assert!(expanded_str.contains("LengthOrMass"));
        assert!(expanded_str.contains("trait"));
    }
}
