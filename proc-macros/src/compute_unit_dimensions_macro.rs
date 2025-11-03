use proc_macro::TokenStream;
use quote::quote;
use syn::parse;

/// Computes unit dimensions for a unit expression.
///
/// Usage: `compute_unit_dimensions!(unit_expr)`
/// Returns a tuple of 12 i16 values representing the dimensions
pub fn compute_unit_dimensions(input: TokenStream) -> TokenStream {
    let unit_expr: whippyunits_core::UnitExpr = parse(input).expect("Expected unit expression");

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
            whippyunits_core::dimension_exponents::DynDimensionExponents([#d0, #d1, #d2, #d3, #d4, #d5, #d6, #d7]),
            whippyunits_core::scale_exponents::ScaleExponents([#d8, #d9, #d10, #d11])
        )
    }
    .into()
}
