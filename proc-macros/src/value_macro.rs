use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Comma;
use syn::{Expr, Ident, Type};
use whippyunits_core::{get_unit_info, Dimension, EvaluationMode, UnitExpr};

use crate::utils::shared_utils::generate_unit_documentation_for_expr;

/// Input for the value! macro
/// Syntax: value!(quantity, unit_expr) or value!(quantity, unit_expr, type) or value!(quantity, unit_expr, type, brand)
pub struct ValueMacroInput {
    quantity: Expr,
    unit_expr: UnitExpr,
    storage_type: Option<Type>,
    brand_type: Option<Type>,
}

impl Parse for ValueMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let quantity: Expr = input.parse()?;
        let _comma: Comma = input.parse()?;
        let unit_expr: UnitExpr = input.parse()?;

        let storage_type = if input.peek(Comma) {
            let _comma: Comma = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        let brand_type = if storage_type.is_some() && input.peek(Comma) {
            let _comma: Comma = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(ValueMacroInput {
            quantity,
            unit_expr,
            storage_type,
            brand_type,
        })
    }
}

impl ValueMacroInput {
    pub fn expand(self) -> TokenStream {
        // Generate documentation structs for unit identifiers
        // For value! macro, use storage type for affine/nonstorage units (like quantity!)
        let doc_structs = generate_unit_documentation_for_expr(&self.unit_expr, true);

        let quantity = &self.quantity;

        // Check if this is a simple atomic unit that's affine
        let is_affine = if let UnitExpr::Unit(unit) = &self.unit_expr {
            if let Some(unit_info) = get_unit_info(&unit.name.to_string()) {
                unit_info.affine_offset != 0.0
            } else {
                false
            }
        } else {
            false // Compound units can't be affine
        };

        // Evaluate unit expression with tolerant mode (allows nonstorage units)
        let result = self.unit_expr.evaluate_with_mode(EvaluationMode::Tolerant);

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

        // Determine the storage type and rescale function
        let (storage_type_ty, rescale_fn) = if let Some(ref ty) = self.storage_type {
            let ty_str = quote!(#ty).to_string();
            match ty_str.as_str() {
                "f32" => (quote!(f32), quote!(rescale_f32)),
                "i8" => (quote!(i8), quote!(rescale_i8)),
                "i16" => (quote!(i16), quote!(rescale_i16)),
                "i32" => (quote!(i32), quote!(rescale_i32)),
                "i64" => (quote!(i64), quote!(rescale_i64)),
                "i128" => (quote!(i128), quote!(rescale_i128)),
                "u8" => (quote!(u8), quote!(rescale_u8)),
                "u16" => (quote!(u16), quote!(rescale_u16)),
                "u32" => (quote!(u32), quote!(rescale_u32)),
                "u64" => (quote!(u64), quote!(rescale_u64)),
                "u128" => (quote!(u128), quote!(rescale_u128)),
                _ => (quote!(#ty), quote!(rescale)),
            }
        } else {
            (quote!(f64), quote!(rescale))
        };

        let brand_type_ty = self
            .brand_type
            .as_ref()
            .map(|t| quote! { #t })
            .unwrap_or_else(|| quote! { () });

        // Construct the target unit type directly (like unit! macro does)
        let target_unit_type = quote! {
            whippyunits::quantity::Quantity<
                whippyunits::quantity::Scale<whippyunits::quantity::_2<#p2>, whippyunits::quantity::_3<#p3>, whippyunits::quantity::_5<#p5>, whippyunits::quantity::_Pi<#pi>>,
                whippyunits::quantity::Dimension<whippyunits::quantity::_M<#mass_exp>, whippyunits::quantity::_L<#length_exp>, whippyunits::quantity::_T<#time_exp>, whippyunits::quantity::_I<#current_exp>, whippyunits::quantity::_Θ<#temp_exp>, whippyunits::quantity::_N<#amount_exp>, whippyunits::quantity::_J<#lum_exp>, whippyunits::quantity::_A<#angle_exp>>,
                #storage_type_ty,
                #brand_type_ty
            >
        };

        if is_affine {
            // Handle affine unit: get value in storage unit, then subtract offset
            let unit = match &self.unit_expr {
                UnitExpr::Unit(u) => u,
                _ => unreachable!(), // We already checked it's Unit
            };
            let unit_name = unit.name.to_string();
            let unit_info = get_unit_info(&unit_name).unwrap();
            let affine_offset = unit_info.affine_offset;

            // Find the storage unit (same scale, no offset, conversion_factor == 1.0)
            let storage_unit_symbol = if let Some((unit, dimension)) =
                Dimension::find_unit_by_symbol(&unit_name)
            {
                // Find a storage unit with the same scale
                if let Some(storage_unit) = dimension.units.iter().find(|u| {
                    u.scale == unit.scale && u.conversion_factor == 1.0 && u.affine_offset == 0.0
                }) {
                    storage_unit.symbols[0]
                } else {
                    // Fallback: use the first symbol of the first storage unit in the dimension
                    dimension
                        .units
                        .iter()
                        .find(|u| u.conversion_factor == 1.0 && u.affine_offset == 0.0)
                        .map(|u| u.symbols[0])
                        .unwrap_or("K") // Fallback to K for temperature
                }
            } else {
                "K" // Fallback
            };

            let storage_unit_ident = Ident::new(storage_unit_symbol, unit.name.span());

            // Generate: (rescale(quantity) as storage_unit_type).unsafe_value - offset
            quote! {
                {
                    const _: () = {
                        #doc_structs
                    };
                    let storage_value = (whippyunits::api::#rescale_fn(#quantity) as whippyunits::unit!(#storage_unit_ident, #storage_type_ty, #brand_type_ty)).unsafe_value;
                    (storage_value as f64 - #affine_offset) as #storage_type_ty
                }
            }
        } else {
            // Normal unit (simple or compound): cast to target type and get unsafe_value
            // The rescale function handles nonstorage unit conversions
            quote! {
                {
                    const _: () = {
                        #doc_structs
                    };
                    (whippyunits::api::#rescale_fn(#quantity) as #target_unit_type).unsafe_value
                }
            }
        }
    }
}
