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
        // Generate the lift trace docstring
        let input_expr_decomposed = self.generate_input_unit_expression_decomposed_string();
        let output_expr = self.generate_output_unit_expression_string();
        let lift_trace = format!("{} -> {}", input_expr_decomposed, output_expr);
        
        // Generate lift trace doc shadows for each unit identifier in the expression
        let lift_trace_doc_shadows = self.generate_lift_trace_doc_shadows_for_expr(&lift_trace);
        
        // Use the specified storage type or default to f64
        let storage_type = self
            .storage_type
            .clone()
            .unwrap_or_else(|| syn::parse_str::<Type>("f64").unwrap());

        // Check if this is a single unit (not an algebraic expression)
        if let UnitExpr::Unit(unit) = &self.unit_expr {
            let unit_name = unit.name.to_string();
            self.handle_single_unit(&unit_name, &storage_type, &lift_trace_doc_shadows)
        } else {
            // It's an algebraic expression (like J/s, m*s, etc.)
            self.handle_algebraic_expression(&storage_type, &lift_trace_doc_shadows)
        }
    }

    /// Generate lift trace doc shadows for each unit identifier in the expression
    fn generate_lift_trace_doc_shadows_for_expr(&self, lift_trace: &str) -> TokenStream {
        let unit_identifiers = self.unit_expr.collect_unit_identifiers();
        let mut doc_shadows = Vec::new();

        for identifier in unit_identifiers {
            let doc_shadow = self.generate_lift_trace_doc_shadow_for_identifier(&identifier, lift_trace);
            doc_shadows.push(doc_shadow);
        }

        quote! {
            #(#doc_shadows)*
        }
    }

    /// Generate a doc shadow for a single unit identifier with the lift trace
    fn generate_lift_trace_doc_shadow_for_identifier(&self, identifier: &Ident, lift_trace: &str) -> TokenStream {
        // Create a new identifier with the same span as the original, using upper camel case
        let doc_ident_name = format!("LiftTrace{}", identifier.to_string().to_uppercase());
        let doc_ident = syn::Ident::new(&doc_ident_name, identifier.span());
        
        quote! {
            const _: () = {
                #[doc = #lift_trace]
                #[allow(dead_code, non_camel_case_types)]
                type #doc_ident = ();
            };
        }
    }

    /// Generate the input unit expression in decomposed form for the lift trace
    pub fn generate_input_unit_expression_decomposed_string(&self) -> String {
        // Use SI base units for the "before" state
        let si_base_units = [
            ("kg", "kg"),
            ("m", "m"), 
            ("s", "s"),
            ("A", "A"),
            ("K", "K"),
            ("mol", "mol"),
            ("cd", "cd"),
            ("rad", "rad"),
        ];
        
        self.generate_unit_expression_with_base_units(&si_base_units)
    }

    /// Get the local base units array, converting scale types to actual unit symbols
    fn get_local_base_units(&self) -> [(String, String); 8] {
        let mass_base = scale_type_to_actual_unit_symbol(&self.mass_scale.to_string()).unwrap_or_else(|| "g".to_string());
        let length_base = scale_type_to_actual_unit_symbol(&self.length_scale.to_string()).unwrap_or_else(|| "m".to_string());
        let time_base = scale_type_to_actual_unit_symbol(&self.time_scale.to_string()).unwrap_or_else(|| "s".to_string());
        let current_base = scale_type_to_actual_unit_symbol(&self.current_scale.to_string()).unwrap_or_else(|| "A".to_string());
        let temperature_base = scale_type_to_actual_unit_symbol(&self.temperature_scale.to_string()).unwrap_or_else(|| "K".to_string());
        let amount_base = scale_type_to_actual_unit_symbol(&self.amount_scale.to_string()).unwrap_or_else(|| "mol".to_string());
        let luminosity_base = scale_type_to_actual_unit_symbol(&self.luminosity_scale.to_string()).unwrap_or_else(|| "cd".to_string());
        let angle_base = scale_type_to_actual_unit_symbol(&self.angle_scale.to_string()).unwrap_or_else(|| "rad".to_string());
        
        [
            (mass_base.clone(), mass_base),
            (length_base.clone(), length_base),
            (time_base.clone(), time_base),
            (current_base.clone(), current_base),
            (temperature_base.clone(), temperature_base),
            (amount_base.clone(), amount_base),
            (luminosity_base.clone(), luminosity_base),
            (angle_base.clone(), angle_base),
        ]
    }

    /// Shared helper to generate unit expression string with given base units
    fn generate_unit_expression_with_base_units(&self, base_units: &[(&str, &str); 8]) -> String {
        // Evaluate the unit expression to get dimension exponents
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp, _p2, _p3, _p5, _pi) = self.unit_expr.evaluate();
        
        // Use the provided base units to generate the expression
        dimension_exponents_to_unit_expression((mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp), base_units)
    }

    /// Generate the output unit expression string for the lift trace
    pub fn generate_output_unit_expression_string(&self) -> String {
        // Check if this is a single unit (not an algebraic expression)
        if let UnitExpr::Unit(unit) = &self.unit_expr {
            let unit_name = unit.name.to_string();
            self.generate_single_unit_expression_string(&unit_name)
        } else {
            // It's an algebraic expression (like J/s, m*s, etc.)
            self.generate_algebraic_expression_string()
        }
    }

    /// Generate expression string for a single unit
    fn generate_single_unit_expression_string(&self, unit_name: &str) -> String {
        // Check if it's a prefixed compound unit (like kPa, mW, kJ)
        if let Some((base_symbol, _prefix)) = is_prefixed_compound_unit(unit_name) {
            // Get dimensions for the base unit (without prefix)
            if let Some((dimension, _)) = lookup_unit_literal(base_symbol) {
                let dimensions = dimension.exponents;
                self.generate_unit_expression_from_dimensions(dimensions)
            } else {
                // Fallback to original unit
                unit_name.to_string()
            }
        } else {
            // For non-prefixed single units, use the original logic
            if let Some((dimension, _)) = lookup_unit_literal(unit_name) {
                let dimensions = dimension.exponents;
                // Check if it's a simple base unit
                if let Some(scale_ident) = self.get_scale_for_dimensions(dimensions) {
                    scale_ident.to_string()
                } else {
                    // It's a compound unit - generate the unit expression
                    self.generate_unit_expression_from_dimensions(dimensions)
                }
            } else {
                // Unknown unit, fall back to original
                unit_name.to_string()
            }
        }
    }

    /// Generate expression string for an algebraic expression
    fn generate_algebraic_expression_string(&self) -> String {
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp, _p2, _p3, _p5, _pi) = self.evaluate_dimensions();

        // Check if it's a simple base unit (single dimension = 1, others = 0)
        if let Some(scale_ident) = self.get_scale_for_dimensions((mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp)) {
            scale_ident.to_string()
        } else {
            // It's a compound unit - generate the unit expression using local base units
            self.generate_unit_expression_from_dimensions((mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp))
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

    /// Convert local base units array to string references for use in dimension_exponents_to_unit_expression
    fn get_local_base_units_refs(&self) -> [(String, String); 8] {
        self.get_local_base_units()
    }

    /// Generate a quote! block for a simple base unit with the given scale identifier
    fn generate_simple_base_unit_quote(&self, scale_ident: &Ident, storage_type: &Type, lift_trace_doc_shadows: &TokenStream) -> TokenStream {
        quote! {
            <whippyunits::Helper<{
                #lift_trace_doc_shadows
                0
            }, whippyunits::default_declarators::#scale_ident<#storage_type>> as whippyunits::GetSecondGeneric>::Type
        }
    }

    /// Generate a quote! block for a compound unit with the given unit expression
    fn generate_compound_unit_quote(&self, unit_expr_parsed: &syn::Expr, storage_type: &Type, lift_trace_doc_shadows: &TokenStream) -> TokenStream {
        quote! {
            <whippyunits::Helper<{
                #lift_trace_doc_shadows
                0
            }, whippyunits::unit!(#unit_expr_parsed, #storage_type)> as whippyunits::GetSecondGeneric>::Type
        }
    }

    /// Evaluate unit expression dimensions and return the exponents
    fn evaluate_dimensions(&self) -> (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) {
        self.unit_expr.evaluate()
    }

    /// Generate unit expression string from dimensions using local base units
    fn generate_unit_expression_from_dimensions(&self, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16)) -> String {
        let base_units = self.get_local_base_units_refs();
        let base_units_refs: [(&str, &str); 8] = [
            (base_units[0].0.as_str(), base_units[0].1.as_str()),
            (base_units[1].0.as_str(), base_units[1].1.as_str()),
            (base_units[2].0.as_str(), base_units[2].1.as_str()),
            (base_units[3].0.as_str(), base_units[3].1.as_str()),
            (base_units[4].0.as_str(), base_units[4].1.as_str()),
            (base_units[5].0.as_str(), base_units[5].1.as_str()),
            (base_units[6].0.as_str(), base_units[6].1.as_str()),
            (base_units[7].0.as_str(), base_units[7].1.as_str()),
        ];
        dimension_exponents_to_unit_expression(dimensions, &base_units_refs)
    }

    /// Handle prefixed compound unit logic for a single unit
    fn handle_prefixed_compound_unit(&self, unit_name: &str, storage_type: &Type, lift_trace_doc_shadows: &TokenStream) -> Option<TokenStream> {
        if let Some((base_symbol, _prefix)) = is_prefixed_compound_unit(unit_name) {
            if let Some((dimension, _)) = lookup_unit_literal(base_symbol) {
                let dimensions = dimension.exponents;
                let unit_expr = self.generate_unit_expression_from_dimensions(dimensions);
                let unit_expr_parsed = syn::parse_str::<syn::Expr>(&unit_expr).unwrap_or_else(|_| {
                    syn::parse_str::<syn::Expr>(base_symbol).unwrap()
                });
                return Some(self.generate_compound_unit_quote(&unit_expr_parsed, storage_type, lift_trace_doc_shadows));
            }
        }
        None
    }

    /// Handle single unit logic (both prefixed and non-prefixed)
    fn handle_single_unit(&self, unit_name: &str, storage_type: &Type, lift_trace_doc_shadows: &TokenStream) -> TokenStream {
        // First try to handle as prefixed compound unit
        if let Some(quote_result) = self.handle_prefixed_compound_unit(unit_name, storage_type, lift_trace_doc_shadows) {
            return quote_result;
        }

        // Handle as regular unit
        if let Some((dimension, _)) = lookup_unit_literal(unit_name) {
            let dimensions = dimension.exponents;
            
            // Check if it's a simple base unit
            if let Some(scale_ident) = self.get_scale_for_dimensions(dimensions) {
                self.generate_simple_base_unit_quote(&scale_ident, storage_type, lift_trace_doc_shadows)
            } else {
                // It's a compound unit - generate the unit expression
                let unit_expr = self.generate_unit_expression_from_dimensions(dimensions);
                let unit_expr_parsed = syn::parse_str::<syn::Expr>(&unit_expr).unwrap_or_else(|_| {
                    syn::parse_str::<syn::Expr>(unit_name).unwrap()
                });
                self.generate_compound_unit_quote(&unit_expr_parsed, storage_type, lift_trace_doc_shadows)
            }
        } else {
            // Unknown unit, fall back to original
            self.generate_compound_unit_quote(&syn::parse_str::<syn::Expr>(unit_name).unwrap(), storage_type, lift_trace_doc_shadows)
        }
    }

    /// Handle algebraic expression logic
    fn handle_algebraic_expression(&self, storage_type: &Type, lift_trace_doc_shadows: &TokenStream) -> TokenStream {
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp, _p2, _p3, _p5, _pi) = self.evaluate_dimensions();

        // Check if it's a simple base unit (single dimension = 1, others = 0)
        if let Some(scale_ident) = self.get_scale_for_dimensions((mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp)) {
            self.generate_simple_base_unit_quote(&scale_ident, storage_type, lift_trace_doc_shadows)
        } else {
            // It's a compound unit - generate the unit expression using local base units
            let unit_expr = self.generate_unit_expression_from_dimensions((mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp));
            let unit_expr_parsed = syn::parse_str::<syn::Expr>(&unit_expr).unwrap_or_else(|_| {
                // If parsing fails, fall back to a generic unit expression
                syn::parse_str::<syn::Expr>("m").unwrap()
            });
            self.generate_compound_unit_quote(&unit_expr_parsed, storage_type, lift_trace_doc_shadows)
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
