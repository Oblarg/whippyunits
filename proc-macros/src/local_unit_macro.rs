use proc_macro2::TokenStream;
use quote::quote;
use syn::token::Comma;
use syn::{
    parse::{Parse, ParseStream, Result},
    Ident, Type,
};
use whippyunits_default_dimensions::{
    dimension_exponents_to_unit_expression, scale_type_to_actual_unit_symbol,
    lookup_unit_literal, is_prefixed_base_unit,
};

// Import the UnitExpr type from unit_macro
use crate::unit_macro::UnitExpr;

/// Input for the local quantity macro
/// This takes a unit expression, local scale parameters, and optional storage type
pub struct LocalQuantityMacroInput {
    pub unit_expr: UnitExpr,
    pub mass_scale: Ident,
    pub length_scale: Ident,
    pub time_scale: Ident,
    pub current_scale: Ident,
    pub temperature_scale: Ident,
    pub amount_scale: Ident,
    pub luminosity_scale: Ident,
    pub angle_scale: Ident,
    pub storage_type: Option<Type>,
}

impl Parse for LocalQuantityMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the unit expression first
        let unit_expr: UnitExpr = input.parse()?;

        // Expect a comma
        let _comma: Comma = input.parse()?;

        // Parse the local scale parameters
        let mass_scale: Ident = input.parse()?;
        let _comma: Comma = input.parse()?;
        let length_scale: Ident = input.parse()?;
        let _comma: Comma = input.parse()?;
        let time_scale: Ident = input.parse()?;
        let _comma: Comma = input.parse()?;
        let current_scale: Ident = input.parse()?;
        let _comma: Comma = input.parse()?;
        let temperature_scale: Ident = input.parse()?;
        let _comma: Comma = input.parse()?;
        let amount_scale: Ident = input.parse()?;
        let _comma: Comma = input.parse()?;
        let luminosity_scale: Ident = input.parse()?;
        let _comma: Comma = input.parse()?;
        let angle_scale: Ident = input.parse()?;

        // Check if there's a comma followed by a storage type parameter
        let storage_type = if input.peek(Comma) {
            let _comma: Comma = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(LocalQuantityMacroInput {
            unit_expr,
            mass_scale,
            length_scale,
            time_scale,
            current_scale,
            temperature_scale,
            amount_scale,
            luminosity_scale,
            angle_scale,
            storage_type,
        })
    }
}

impl LocalQuantityMacroInput {
    pub fn expand(self) -> TokenStream {
        // Use the specified storage type or default to f64
        let storage_type = self
            .storage_type
            .clone()
            .unwrap_or_else(|| syn::parse_str::<Type>("f64").unwrap());

        // Get the actual unit symbols for each scale type
        let mass_base = scale_type_to_actual_unit_symbol(&self.mass_scale.to_string())
            .unwrap_or_else(|| "g".to_string());
        let length_base = scale_type_to_actual_unit_symbol(&self.length_scale.to_string())
            .unwrap_or_else(|| "m".to_string());
        let time_base = scale_type_to_actual_unit_symbol(&self.time_scale.to_string())
            .unwrap_or_else(|| "s".to_string());
        let current_base = scale_type_to_actual_unit_symbol(&self.current_scale.to_string())
            .unwrap_or_else(|| "A".to_string());
        let temperature_base = scale_type_to_actual_unit_symbol(&self.temperature_scale.to_string())
            .unwrap_or_else(|| "K".to_string());
        let amount_base = scale_type_to_actual_unit_symbol(&self.amount_scale.to_string())
            .unwrap_or_else(|| "mol".to_string());
        let luminosity_base = scale_type_to_actual_unit_symbol(&self.luminosity_scale.to_string())
            .unwrap_or_else(|| "cd".to_string());
        let angle_base = scale_type_to_actual_unit_symbol(&self.angle_scale.to_string())
            .unwrap_or_else(|| "rad".to_string());

        // Check if this is a single unit (not an algebraic expression)
        if let UnitExpr::Unit(unit) = &self.unit_expr {
            let unit_name = unit.name.to_string();
            
            // Check if it's a prefixed compound unit (like kPa, mW, kJ)
            if let Some((base_symbol, _prefix)) = is_prefixed_compound_unit(&unit_name) {
                // For prefixed compound units, we need to handle them specially
                // by converting to base unit first, then applying local scale conversion
                let base_units = [
                    (mass_base.as_str(), mass_base.as_str()),
                    (length_base.as_str(), length_base.as_str()),
                    (time_base.as_str(), time_base.as_str()),
                    (current_base.as_str(), current_base.as_str()),
                    (temperature_base.as_str(), temperature_base.as_str()),
                    (amount_base.as_str(), amount_base.as_str()),
                    (luminosity_base.as_str(), luminosity_base.as_str()),
                    (angle_base.as_str(), angle_base.as_str()),
                ];

                // Get dimensions for the base unit (without prefix)
                if let Some((dimension, _)) = lookup_unit_literal(base_symbol) {
                    let dimensions = dimension.exponents;
                    let unit_expr = dimension_exponents_to_unit_expression(dimensions, &base_units);
                    let unit_expr_parsed = syn::parse_str::<syn::Expr>(&unit_expr).unwrap_or_else(|_| {
                        syn::parse_str::<syn::Expr>(base_symbol).unwrap()
                    });

                    quote! { whippyunits::unit!(#unit_expr_parsed, #storage_type) }
                } else {
                    // Fallback to original unit
                    let unit_ident = &unit.name;
                    quote! { whippyunits::unit!(#unit_ident, #storage_type) }
                }
            } else {
                // For non-prefixed single units, use the original logic
                if let Some((dimension, _)) = lookup_unit_literal(&unit_name) {
                    let dimensions = dimension.exponents;
                    // Check if it's a simple base unit
                    if let Some(scale_ident) = self.get_scale_for_dimensions(dimensions) {
                        quote! { whippyunits::default_declarators::#scale_ident<#storage_type> }
                    } else {
                        // It's a compound unit - generate the unit expression
                        let base_units = [
                            (mass_base.as_str(), mass_base.as_str()),
                            (length_base.as_str(), length_base.as_str()),
                            (time_base.as_str(), time_base.as_str()),
                            (current_base.as_str(), current_base.as_str()),
                            (temperature_base.as_str(), temperature_base.as_str()),
                            (amount_base.as_str(), amount_base.as_str()),
                            (luminosity_base.as_str(), luminosity_base.as_str()),
                            (angle_base.as_str(), angle_base.as_str()),
                        ];

                        let unit_expr = dimension_exponents_to_unit_expression(dimensions, &base_units);
                        let unit_expr_parsed = syn::parse_str::<syn::Expr>(&unit_expr).unwrap_or_else(|_| {
                            syn::parse_str::<syn::Expr>(&unit_name).unwrap()
                        });

                        quote! { whippyunits::unit!(#unit_expr_parsed, #storage_type) }
                    }
                } else {
                    // Unknown unit, fall back to original
                    let unit_ident = &unit.name;
                    quote! { whippyunits::unit!(#unit_ident, #storage_type) }
                }
            }
        } else {
            // It's an algebraic expression (like J/s, m*s, etc.)
            // Evaluate the unit expression to get dimension exponents
            let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp, _p2, _p3, _p5, _pi) = self.unit_expr.evaluate();

            // Check if it's a simple base unit (single dimension = 1, others = 0)
            if let Some(scale_ident) = self.get_scale_for_dimensions((mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp)) {
                quote! { whippyunits::default_declarators::#scale_ident<#storage_type> }
            } else {
                // It's a compound unit - generate the unit expression using local base units
                let base_units = [
                    (mass_base.as_str(), mass_base.as_str()),
                    (length_base.as_str(), length_base.as_str()),
                    (time_base.as_str(), time_base.as_str()),
                    (current_base.as_str(), current_base.as_str()),
                    (temperature_base.as_str(), temperature_base.as_str()),
                    (amount_base.as_str(), amount_base.as_str()),
                    (luminosity_base.as_str(), luminosity_base.as_str()),
                    (angle_base.as_str(), angle_base.as_str()),
                ];

                let unit_expr = dimension_exponents_to_unit_expression((mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp), &base_units);
                let unit_expr_parsed = syn::parse_str::<syn::Expr>(&unit_expr).unwrap_or_else(|_| {
                    // If parsing fails, fall back to a generic unit expression
                    syn::parse_str::<syn::Expr>("m").unwrap()
                });

                quote! { whippyunits::unit!(#unit_expr_parsed, #storage_type) }
            }
        }
    }

    /// Get the appropriate scale identifier for given dimension exponents
    /// Returns Some(scale_ident) if it's a simple base unit, None for compound units
    fn get_scale_for_dimensions(
        &self,
        dimensions: (i16, i16, i16, i16, i16, i16, i16, i16),
    ) -> Option<Ident> {
        match dimensions {
            (1, 0, 0, 0, 0, 0, 0, 0) => Some(self.mass_scale.clone()),
            (0, 1, 0, 0, 0, 0, 0, 0) => Some(self.length_scale.clone()),
            (0, 0, 1, 0, 0, 0, 0, 0) => Some(self.time_scale.clone()),
            (0, 0, 0, 1, 0, 0, 0, 0) => Some(self.current_scale.clone()),
            (0, 0, 0, 0, 1, 0, 0, 0) => Some(self.temperature_scale.clone()),
            (0, 0, 0, 0, 0, 1, 0, 0) => Some(self.amount_scale.clone()),
            (0, 0, 0, 0, 0, 0, 1, 0) => Some(self.luminosity_scale.clone()),
            (0, 0, 0, 0, 0, 0, 0, 1) => Some(self.angle_scale.clone()),
            _ => None, // Compound unit
        }
    }
}

/// Check if a unit symbol is a prefixed compound unit (kPa, mW, etc.)
fn is_prefixed_compound_unit(unit_symbol: &str) -> Option<(&str, &str)> {
    // Use the new is_prefixed_base_unit function from the util module
    if let Some((base_symbol, prefix)) = is_prefixed_base_unit(unit_symbol) {
        // Check if the base unit is a compound unit (has multiple non-zero dimension exponents)
        if let Some((dimension, _)) = lookup_unit_literal(base_symbol) {
            let (m, l, t, c, temp, a, lum, ang) = dimension.exponents;
            let non_zero_count = [m, l, t, c, temp, a, lum, ang].iter().filter(|&&x| x != 0).count();
            if non_zero_count > 1 {
                return Some((base_symbol, prefix));
            }
        }
    }
    None
}
