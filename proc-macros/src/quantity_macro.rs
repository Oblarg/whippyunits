use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Comma;
use syn::{Expr, Type};
use whippyunits_core::{Dimension, UnitExpr, get_unit_info, calculate_unit_conversion_factors};

/// Input for the quantity macro
pub struct QuantityMacroInput {
    pub value: Expr,
    pub unit_expr: UnitExpr,
    pub storage_type: Option<Type>,
    pub brand_type: Option<Type>,
}

impl Parse for QuantityMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let value: Expr = input.parse()?;
        let _comma: Comma = input.parse()?;
        let unit_expr: UnitExpr = input.parse()?;

        // Check if there's a comma followed by a storage type parameter
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

        Ok(QuantityMacroInput {
            value,
            unit_expr,
            storage_type,
            brand_type,
        })
    }
}

impl QuantityMacroInput {
    pub fn expand(self) -> TokenStream {
        // Check if this is a simple atomic unit that's nonstorage/affine
        // For these, we can dispatch directly to the declarator
        if let UnitExpr::Unit(unit) = &self.unit_expr {
            if let Some(unit_info) = get_unit_info(&unit.name.to_string()) {
                // Check if it's nonstorage or affine
                let is_nonstorage = unit_info.conversion_factor != 1.0;
                let is_affine = unit_info.affine_offset != 0.0;
                
                if is_nonstorage || is_affine {
                    // Dispatch to appropriate declarator (handles conversion internally)
                    return self.expand_with_declarator(unit_info);
                }
            }
        }
        
        // For storage units or compound units (including those with nonstorage),
        // use shared conversion factor calculation
        self.expand_with_conversion_factors()
    }
    
    fn expand_with_declarator(&self, unit_info: &whippyunits_core::Unit) -> TokenStream {
        // Find the dimension for this unit
        let dimension = Dimension::ALL.iter().find(|dim| {
            dim.units.iter().any(|u| u.name == unit_info.name)
        });
        
        let Some(dimension) = dimension else {
            // Fall back to conversion factors approach if dimension not found
            return self.expand_with_conversion_factors();
        };
        
        // Generate trait name using shared logic from whippyunits-core
        let full_trait_name = whippyunits_core::generate_declarator_trait_name(
            unit_info.system,
            &dimension.name,
            unit_info.conversion_factor,
            unit_info.affine_offset,
        );
        
        // If this is a pure storage metric unit, we shouldn't reach here for nonstorage dispatch
        if unit_info.system == whippyunits_core::System::Metric
            && unit_info.conversion_factor == 1.0
            && unit_info.affine_offset == 0.0
        {
            return self.expand_with_conversion_factors();
        }
        
        // Get the method name (plural form of unit name)
        let method_name = whippyunits_core::make_plural(unit_info.name);
        
        let trait_ident = syn::Ident::new(&full_trait_name, proc_macro2::Span::call_site());
        let method_ident = syn::Ident::new(&method_name, proc_macro2::Span::call_site());
        
        let value_expr = &self.value;
        
        // Generate the declarator call
        // The trait methods are called directly on the value type
        // The trait is generic, but the impls are for specific types (f64, i32)
        quote! {
            {
                use whippyunits::default_declarators::#trait_ident;
                (#value_expr).#method_ident()
            }
        }
    }
    
    fn expand_with_conversion_factors(&self) -> TokenStream {
        // Use shared logic: evaluate unit expression and calculate conversion factors
        // (same approach as deserialize/fmt methods)
        let result = self.unit_expr.evaluate_with_mode(whippyunits_core::EvaluationMode::Tolerant);
        let (conversion_factor, affine_offset) = calculate_unit_conversion_factors(&self.unit_expr);
        
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
        
        let storage_type_ty = self
            .storage_type
            .as_ref()
            .map(|t| quote! { #t })
            .unwrap_or_else(|| quote! { f64 });
        
        let brand_type_ty = self
            .brand_type
            .as_ref()
            .map(|t| quote! { #t })
            .unwrap_or_else(|| quote! { () });
        
        let value_expr = &self.value;
        let has_nonstorage = conversion_factor != 1.0 || affine_offset != 0.0;
        
        if has_nonstorage {
            // Apply conversion factor and affine offset (same logic as deserialize)
            let cf = conversion_factor;
            let af = affine_offset;
            
            quote! {
                {
                    use whippyunits::quantity::{Quantity, Scale, Dimension, _2, _3, _5, _Pi, _M, _L, _T, _I, _Θ, _N, _J, _A};
                    let raw_value: #storage_type_ty = #value_expr;
                    let converted_value = (raw_value as f64) * #cf + #af;
                    Quantity::<Scale<_2<#p2>, _3<#p3>, _5<#p5>, _Pi<#pi>>, Dimension<_M<#mass_exp>, _L<#length_exp>, _T<#time_exp>, _I<#current_exp>, _Θ<#temp_exp>, _N<#amount_exp>, _J<#lum_exp>, _A<#angle_exp>>, #storage_type_ty, #brand_type_ty>::new(converted_value as #storage_type_ty)
                }
            }
        } else {
            // Pure storage unit - no conversion needed
            quote! {
                {
                    use whippyunits::quantity::{Quantity, Scale, Dimension, _2, _3, _5, _Pi, _M, _L, _T, _I, _Θ, _N, _J, _A};
                    Quantity::<Scale<_2<#p2>, _3<#p3>, _5<#p5>, _Pi<#pi>>, Dimension<_M<#mass_exp>, _L<#length_exp>, _T<#time_exp>, _I<#current_exp>, _Θ<#temp_exp>, _N<#amount_exp>, _J<#lum_exp>, _A<#angle_exp>>, #storage_type_ty, #brand_type_ty>::new(#value_expr)
                }
            }
        }
    }
}

