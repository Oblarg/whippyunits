use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Comma;
use syn::Type;
use whippyunits_core::UnitExpr;

use crate::utils::shared_utils::generate_unit_documentation_for_expr;

/// Input for the unit macro
pub struct UnitMacroInput {
    pub unit_expr: UnitExpr,
    pub storage_type: Option<Type>,
    pub brand_type: Option<Type>,
}

impl Parse for UnitMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let unit_expr = input.parse()?;

        // Check if there's a comma followed by a type parameter
        let storage_type = if input.peek(Comma) {
            let _comma: Comma = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        // Check if there's another comma followed by a brand type parameter
        let brand_type = if input.peek(Comma) {
            let _comma: Comma = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(UnitMacroInput {
            unit_expr,
            storage_type,
            brand_type,
        })
    }
}

impl UnitMacroInput {
    pub fn expand(self) -> TokenStream {
        // Validate that no nonstorage units are used (strict mode requirement)
        if let Some(error_msg) = self.unit_expr.validate_strict() {
            return quote! {
                compile_error!(#error_msg);
            };
        }

        let result = self.unit_expr.evaluate();
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) = (
            result.dimension_exponents.0[0],
            result.dimension_exponents.0[1],
            result.dimension_exponents.0[2],
            result.dimension_exponents.0[3],
            result.dimension_exponents.0[4],
            result.dimension_exponents.0[5],
            result.dimension_exponents.0[6],
            result.dimension_exponents.0[7],
        );
        let (p2, p3, p5, pi) = (
            result.scale_exponents.0[0],
            result.scale_exponents.0[1],
            result.scale_exponents.0[2],
            result.scale_exponents.0[3],
        );

        // Use the specified storage type or default to f64
        let storage_type = self
            .storage_type
            .unwrap_or_else(|| syn::parse_str::<Type>("f64").unwrap());

        // Use the specified brand type or default to ()
        let brand_type = self
            .brand_type
            .unwrap_or_else(|| syn::parse_str::<Type>("()").unwrap());

        // Generate documentation structs for unit identifiers in const expression
        // For unit! macro, we don't use storage type for affine units (they're not allowed in strict mode)
        let doc_structs = generate_unit_documentation_for_expr(&self.unit_expr, false);

        // Generate the actual quantity type
        let quantity_type = quote! {
            whippyunits::quantity::Quantity<
                whippyunits::quantity::Scale<whippyunits::quantity::_2<#p2>, whippyunits::quantity::_3<#p3>, whippyunits::quantity::_5<#p5>, whippyunits::quantity::_Pi<#pi>>,
                whippyunits::quantity::Dimension<whippyunits::quantity::_M<#mass_exp>, whippyunits::quantity::_L<#length_exp>, whippyunits::quantity::_T<#time_exp>, whippyunits::quantity::_I<#current_exp>, whippyunits::quantity::_Θ<#temp_exp>, whippyunits::quantity::_N<#amount_exp>, whippyunits::quantity::_J<#lum_exp>, whippyunits::quantity::_A<#angle_exp>>,
                #storage_type,
                #brand_type
            >
        };

        quote! {
            <whippyunits::Helper<{
                #doc_structs
                0
            }, #quantity_type> as whippyunits::GetSecondGeneric>::Type
        }
    }
}
