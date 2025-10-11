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
    fn generate_lift_trace_doc_shadows_for_expr(&self, _lift_trace: &str) -> TokenStream {
        let unit_identifiers = self.unit_expr.collect_unit_identifiers();
        let mut doc_shadows = Vec::new();

        // Generate a comprehensive trace for the entire expression
        let comprehensive_trace = self.generate_comprehensive_lift_trace();

        for identifier in unit_identifiers {
            let doc_shadow = self.generate_lift_trace_doc_shadow_for_identifier(&identifier, &comprehensive_trace);
            doc_shadows.push(doc_shadow);
        }

        quote! {
            #(#doc_shadows)*
        }
    }

    /// Generate the transformed unit expression string showing proper unit symbols
    fn generate_transformed_unit_expression_string(&self) -> String {
        match &self.unit_expr {
            UnitExpr::Unit(unit) => {
                let unit_name = unit.name.to_string();
                self.get_transformed_unit_symbol(&unit_name)
            },
            UnitExpr::Div(numerator, denominator) => {
                let num_str = match numerator.as_ref() {
                    UnitExpr::Unit(unit) => self.get_transformed_unit_symbol(&unit.name.to_string()),
                    _ => self.generate_transformed_unit_expression_string_for_expr(numerator),
                };
                let den_str = match denominator.as_ref() {
                    UnitExpr::Unit(unit) => self.get_transformed_unit_symbol(&unit.name.to_string()),
                    _ => self.generate_transformed_unit_expression_string_for_expr(denominator),
                };
                format!("{} / {}", num_str, den_str)
            },
            UnitExpr::Mul(left, right) => {
                let left_str = match left.as_ref() {
                    UnitExpr::Unit(unit) => self.get_transformed_unit_symbol(&unit.name.to_string()),
                    _ => self.generate_transformed_unit_expression_string_for_expr(left),
                };
                let right_str = match right.as_ref() {
                    UnitExpr::Unit(unit) => self.get_transformed_unit_symbol(&unit.name.to_string()),
                    _ => self.generate_transformed_unit_expression_string_for_expr(right),
                };
                format!("{} * {}", left_str, right_str)
            },
            UnitExpr::Pow(base, exponent) => {
                let base_str = self.generate_transformed_unit_expression_string_for_expr(base);
                format!("{}^{}", base_str, exponent)
            },
        }
    }

    /// Generate the final unit expression string (showing the most appropriate unit name)
    fn generate_final_unit_expression_string(&self) -> String {
        // For now, return the same as transformed, but this could be enhanced to show
        // the most appropriate unit name (e.g., μJ/s → μW)
        let transformed = self.generate_transformed_unit_expression_string();
        
        // Check if this is a power expression that has a common name
        if transformed == "μJ / s" {
            "μW".to_string()
        } else if transformed == "J / s" {
            "W".to_string()
        } else if transformed == "kJ / s" {
            "kW".to_string()
        } else if transformed == "mJ / s" {
            "mW".to_string()
        } else {
            transformed
        }
    }

    /// Generate transformed unit expression string for a sub-expression
    fn generate_transformed_unit_expression_string_for_expr(&self, expr: &UnitExpr) -> String {
        match expr {
            UnitExpr::Unit(unit) => self.get_transformed_unit_symbol(&unit.name.to_string()),
            UnitExpr::Div(numerator, denominator) => {
                let num_str = self.generate_transformed_unit_expression_string_for_expr(numerator);
                let den_str = self.generate_transformed_unit_expression_string_for_expr(denominator);
                format!("{} / {}", num_str, den_str)
            },
            UnitExpr::Mul(left, right) => {
                let left_str = self.generate_transformed_unit_expression_string_for_expr(left);
                let right_str = self.generate_transformed_unit_expression_string_for_expr(right);
                format!("{} * {}", left_str, right_str)
            },
            UnitExpr::Pow(base, exponent) => {
                let base_str = self.generate_transformed_unit_expression_string_for_expr(base);
                format!("{}^{}", base_str, exponent)
            },
        }
    }

    /// Get the transformed unit symbol for a given unit name
    fn get_transformed_unit_symbol(&self, unit_name: &str) -> String {
        if let Some((dimension, _)) = lookup_unit_literal(unit_name) {
            let dimensions = dimension.exponents;
            
            // Check if this unit gets transformed
            if self.unit_gets_transformed_in_local_context(unit_name) {
                // Calculate the scale factor difference
                let scale_factor_diff = self.calculate_scale_factor_difference(dimensions);
                
                // Get the prefixed unit name
                self.get_prefixed_unit_name(unit_name, scale_factor_diff)
            } else {
                unit_name.to_string()
            }
        } else {
            unit_name.to_string()
        }
    }

    /// Generate a comprehensive lift trace for the entire expression
    fn generate_comprehensive_lift_trace(&self) -> String {
        let original_expr = self.generate_input_unit_expression_decomposed_string();
        let output_expr = self.generate_output_unit_expression_string();
        
        let mut trace = String::new();
        
        // Add summary line showing the overall transformation in unit symbols
        let input_expr = self.generate_input_unit_expression_string();
        let transformed_expr = self.generate_transformed_unit_expression_string();
        let final_expr = self.generate_final_unit_expression_string();
        
        // Show the transformation chain: input → transformed → final
        if transformed_expr != final_expr {
            trace.push_str(&format!("{} → {} → {}<br><br>", input_expr, transformed_expr, final_expr));
        } else {
            trace.push_str(&format!("{} → {}<br><br>", input_expr, transformed_expr));
        }
        
        // Add transformation details for each unit in the expression
        trace.push_str("Transformations:<br>");
        
        let unit_identifiers = self.unit_expr.collect_unit_identifiers();
        for (i, identifier) in unit_identifiers.iter().enumerate() {
            let unit_name = identifier.to_string();
            
            // Get transformation details as individual lines
            let transformation_details = self.get_transformation_details_for_identifier(&unit_name);
            let lines: Vec<&str> = transformation_details.details.lines().collect();
            
            // Add each line component-wise to the trace with HTML line breaks
            for (j, line) in lines.iter().enumerate() {
                trace.push_str(line);
                if j < lines.len() - 1 {
                    trace.push_str("<br>");
                }
            }
            
            if i < unit_identifiers.len() - 1 {
                trace.push_str("\n\n");
            }
        }
        
        trace
    }

    /// Generate a doc shadow for a single unit identifier with the enhanced lift trace
    fn generate_lift_trace_doc_shadow_for_identifier(&self, identifier: &Ident, lift_trace: &str) -> TokenStream {
        // Create a new identifier with the same span as the original, using upper camel case
        let doc_ident_name = format!("Local{}", identifier.to_string().to_uppercase());
        let doc_ident = syn::Ident::new(&doc_ident_name, identifier.span());
        
        // Determine the target type for this unit identifier in the local context
        let target_type = self.get_local_target_type_for_unit(&identifier.to_string());
        
        // Use the comprehensive trace directly
        let enhanced_trace = lift_trace.to_string();
        
        quote! {
            const _: () = {
                #[doc = #enhanced_trace]
                #[allow(dead_code, non_camel_case_types)]
                type #doc_ident = #target_type;
            };
        }
    }

    /// Generate the input unit expression in symbolic form for the lift trace
    pub fn generate_input_unit_expression_string(&self) -> String {
        // Generate the symbolic unit expression (like "J / s")
        match &self.unit_expr {
            UnitExpr::Unit(unit) => unit.name.to_string(),
            UnitExpr::Div(numerator, denominator) => {
                let num_str = match numerator.as_ref() {
                    UnitExpr::Unit(unit) => unit.name.to_string(),
                    _ => self.generate_unit_expression_string_for_expr(numerator),
                };
                let den_str = match denominator.as_ref() {
                    UnitExpr::Unit(unit) => unit.name.to_string(),
                    _ => self.generate_unit_expression_string_for_expr(denominator),
                };
                format!("{} / {}", num_str, den_str)
            },
            UnitExpr::Mul(left, right) => {
                let left_str = match left.as_ref() {
                    UnitExpr::Unit(unit) => unit.name.to_string(),
                    _ => self.generate_unit_expression_string_for_expr(left),
                };
                let right_str = match right.as_ref() {
                    UnitExpr::Unit(unit) => unit.name.to_string(),
                    _ => self.generate_unit_expression_string_for_expr(right),
                };
                format!("{} * {}", left_str, right_str)
            },
            UnitExpr::Pow(base, exponent) => {
                let base_str = self.generate_unit_expression_string_for_expr(base);
                format!("{}^{}", base_str, exponent)
            },
        }
    }

    /// Helper method to generate unit expression string for nested expressions
    fn generate_unit_expression_string_for_expr(&self, expr: &UnitExpr) -> String {
        match expr {
            UnitExpr::Unit(unit) => unit.name.to_string(),
            UnitExpr::Div(numerator, denominator) => {
                let num_str = self.generate_unit_expression_string_for_expr(numerator);
                let den_str = self.generate_unit_expression_string_for_expr(denominator);
                format!("{} / {}", num_str, den_str)
            },
            UnitExpr::Mul(left, right) => {
                let left_str = self.generate_unit_expression_string_for_expr(left);
                let right_str = self.generate_unit_expression_string_for_expr(right);
                format!("{} * {}", left_str, right_str)
            },
            UnitExpr::Pow(base, exponent) => {
                let base_str = self.generate_unit_expression_string_for_expr(base);
                format!("{}^{}", base_str, exponent)
            },
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

    /// Get the local target type for a single unit identifier (reuses existing lift logic)
    fn get_local_target_type_for_unit(&self, unit_name: &str) -> TokenStream {
        // First, try to use the shared helper for units that don't get transformed in local context
        if let Some(shared_type) = crate::get_declarator_type_for_unit(unit_name) {
            // Check if this unit gets transformed in the local context
            if !self.unit_gets_transformed_in_local_context(unit_name) {
                // Unit doesn't get transformed, use the shared declarator type
                return shared_type;
            }
            // Unit gets transformed, fall through to local logic
        }
        
        // For units that get transformed or don't have declarator types, use local logic
        self.get_local_transformed_type_for_unit(unit_name)
    }

    /// Check if a unit gets transformed in the local context
    fn unit_gets_transformed_in_local_context(&self, unit_name: &str) -> bool {
        // Check if this is a simple base unit that maps to a local scale
        if let Some((dimension, _)) = lookup_unit_literal(unit_name) {
            let dimensions = dimension.exponents;
            
            // If it's a simple base unit, check if it gets transformed
            if let Some(_scale_ident) = self.get_scale_for_dimensions(dimensions) {
                // For simple base units, check if there's a scale factor difference
                let scale_factor_diff = self.calculate_scale_factor_difference(dimensions);
                return scale_factor_diff != 0;
            }
            
            // For compound units, check if any of their base units get transformed
            return self.compound_unit_gets_transformed(dimensions);
        }
        
        false
    }

    /// Check if a compound unit gets transformed in the local context
    fn compound_unit_gets_transformed(&self, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16)) -> bool {
        // Check each dimension to see if it would use a different local scale
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) = dimensions;
        
        // Check if any base dimension gets transformed
        if mass_exp != 0 && self.unit_gets_transformed_in_local_context("kg") { return true; }
        if length_exp != 0 && self.unit_gets_transformed_in_local_context("m") { return true; }
        if time_exp != 0 && self.unit_gets_transformed_in_local_context("s") { return true; }
        if current_exp != 0 && self.unit_gets_transformed_in_local_context("A") { return true; }
        if temp_exp != 0 && self.unit_gets_transformed_in_local_context("K") { return true; }
        if amount_exp != 0 && self.unit_gets_transformed_in_local_context("mol") { return true; }
        if lum_exp != 0 && self.unit_gets_transformed_in_local_context("cd") { return true; }
        if angle_exp != 0 && self.unit_gets_transformed_in_local_context("rad") { return true; }
        
        false
    }

    /// Get the local transformed type for a unit (fallback when shared helper isn't sufficient)
    fn get_local_transformed_type_for_unit(&self, unit_name: &str) -> TokenStream {
        // For units that get transformed, we need to generate the local type
        // This should match the logic in handle_single_unit but return just the type
        
        if let Some((dimension, _)) = lookup_unit_literal(unit_name) {
            let dimensions = dimension.exponents;
            
            // Check if it's a simple base unit
            if let Some(scale_ident) = self.get_scale_for_dimensions(dimensions) {
                // For simple base units, the target type is the local scale declarator
                quote! {
                    whippyunits::default_declarators::#scale_ident<f64>
                }
            } else {
                // For compound units, calculate the scale factor difference and find the appropriate prefixed type
                let scale_factor_diff = self.calculate_scale_factor_difference(dimensions);
                
                if scale_factor_diff != 0 {
                    // Try to find a prefixed version that matches the scale factor
                    if let Some(prefixed_type) = self.find_prefixed_type_by_scale_factor(unit_name, scale_factor_diff) {
                        prefixed_type
                    } else {
                        // Fall back to original if no prefixed type found
                        match crate::get_declarator_type_for_unit(unit_name) {
                            Some(declarator_type) => declarator_type,
                            None => quote! { () },
                        }
                    }
                } else {
                    // No scale factor difference, use the original type
                    match crate::get_declarator_type_for_unit(unit_name) {
                        Some(declarator_type) => declarator_type,
                        None => quote! { () },
                    }
                }
            }
        } else {
            // Unknown unit, fall back to using it as-is
            match crate::get_declarator_type_for_unit(unit_name) {
                Some(declarator_type) => declarator_type,
                None => quote! { () },
            }
        }
    }

    /// Calculate the scale factor difference between local and default units
    fn calculate_scale_factor_difference(&self, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16)) -> i16 {
        use whippyunits_default_dimensions::{BASE_UNITS, scale_type_to_actual_unit_symbol};
        
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) = dimensions;
        
        let mut total_scale_diff = 0;
        
        // Check each dimension for scale differences using the centralized utilities
        if mass_exp != 0 {
            total_scale_diff += self.get_scale_difference_for_base_unit("kg", &self.mass_scale) * mass_exp;
        }
        if length_exp != 0 {
            total_scale_diff += self.get_scale_difference_for_base_unit("m", &self.length_scale) * length_exp;
        }
        if time_exp != 0 {
            total_scale_diff += self.get_scale_difference_for_base_unit("s", &self.time_scale) * time_exp;
        }
        if current_exp != 0 {
            total_scale_diff += self.get_scale_difference_for_base_unit("A", &self.current_scale) * current_exp;
        }
        if temp_exp != 0 {
            total_scale_diff += self.get_scale_difference_for_base_unit("K", &self.temperature_scale) * temp_exp;
        }
        if amount_exp != 0 {
            total_scale_diff += self.get_scale_difference_for_base_unit("mol", &self.amount_scale) * amount_exp;
        }
        if lum_exp != 0 {
            total_scale_diff += self.get_scale_difference_for_base_unit("cd", &self.luminosity_scale) * lum_exp;
        }
        if angle_exp != 0 {
            total_scale_diff += self.get_scale_difference_for_base_unit("rad", &self.angle_scale) * angle_exp;
        }
        
        total_scale_diff
    }

    /// Get the scale difference for a specific base unit using centralized utilities
    fn get_scale_difference_for_base_unit(&self, default_unit: &str, local_scale: &Ident) -> i16 {
        use whippyunits_default_dimensions::{BASE_UNITS, scale_type_to_actual_unit_symbol, is_prefixed_base_unit, lookup_si_prefix};
        
        // Get the default base unit info
        let _default_base_unit = BASE_UNITS.iter().find(|u| u.symbol == default_unit);
        
        // Get the local unit symbol
        let local_unit_symbol = scale_type_to_actual_unit_symbol(&local_scale.to_string()).unwrap_or_else(|| default_unit.to_string());
        
        // If the local unit is the same as default, no scale difference
        if local_unit_symbol == default_unit {
            return 0;
        }
        
        // Check if the local unit is a prefixed version of the default unit
        if let Some((prefix_symbol, base_symbol)) = is_prefixed_base_unit(&local_unit_symbol) {
            if base_symbol == default_unit {
                // Get the prefix scale factor
                if let Some(prefix_info) = lookup_si_prefix(prefix_symbol) {
                    return prefix_info.scale_factor;
                }
            }
        }
        
        // If we can't determine the scale difference, return 0
        0
    }

    /// Find a prefixed type by scale factor using centralized utilities
    fn find_prefixed_type_by_scale_factor(&self, unit_name: &str, scale_factor_diff: i16) -> Option<TokenStream> {
        use whippyunits_default_dimensions::{SI_PREFIXES, lookup_si_prefix};
        
        // Find the prefix that matches the scale factor difference
        for prefix_info in SI_PREFIXES {
            if prefix_info.scale_factor == scale_factor_diff {
                // Try to find a prefixed version of this unit
                let prefixed_unit_name = format!("{}{}", prefix_info.symbol, unit_name);
                if let Some(declarator_type) = crate::get_declarator_type_for_unit(&prefixed_unit_name) {
                    return Some(declarator_type);
                }
            }
        }
        
        None
    }

    /// Generate enhanced lift trace for a specific identifier with bolded formatting
    fn generate_enhanced_lift_trace_for_identifier(&self, identifier: &Ident, _lift_trace: &str) -> String {
        let unit_name = identifier.to_string();
        
        // Get the original and output expressions
        let original_expr = self.generate_input_unit_expression_decomposed_string();
        let output_expr = self.generate_output_unit_expression_string();
        
        // Generate the full aggregate transformation with the specific identifier bolded
        let mut trace = String::new();
        
        // Show the full expression transformation with bolded identifier
        let bolded_original = self.bold_identifier_in_expression(&original_expr, &unit_name);
        let bolded_output = self.bold_identifier_in_expression(&output_expr, &unit_name);
        
        trace.push_str(&format!("**{}** = {}\n", bolded_original, original_expr));
        trace.push_str("         ↓\n");
        trace.push_str(&format!("         = **{}**\n", bolded_output));
        
        // Add transformation details for this specific identifier
        trace.push_str("\nTransformations:\n");
        trace.push_str(&self.get_transformation_details_for_identifier(&unit_name).details);
        
        trace
    }

    /// Bold a specific identifier within an expression
    fn bold_identifier_in_expression(&self, expression: &str, identifier: &str) -> String {
        // Simple replacement - in a more sophisticated implementation, we could parse the expression
        // and only bold the identifier when it appears as a standalone unit
        expression.replace(identifier, &format!("**{}**", identifier))
    }

    /// Get transformation details for a specific identifier
    fn get_transformation_details_for_identifier(&self, unit_name: &str) -> TransformationDetails {
        if let Some((dimension, _)) = lookup_unit_literal(unit_name) {
            let dimensions = dimension.exponents;
            
            // Check if this unit gets transformed
            if self.unit_gets_transformed_in_local_context(unit_name) {
                // Calculate the scale factor difference
                let scale_factor_diff = self.calculate_scale_factor_difference(dimensions);
                
                // Get the target type
                let target_type = if let Some(scale_ident) = self.get_scale_for_dimensions(dimensions) {
                    // Simple base unit
                    format!("{}", scale_ident)
                } else {
                    // Compound unit - find the prefixed type
                    if let Some(_prefixed_type) = self.find_prefixed_type_by_scale_factor(unit_name, scale_factor_diff) {
                        // Extract the type name from the token stream
                        format!("Prefixed{}", unit_name) // This is a placeholder - we'd need to parse the token stream
                    } else {
                        unit_name.to_string()
                    }
                };
                
                // Generate the transformation details
                let details = self.generate_transformation_explanation(unit_name, &target_type, dimensions, scale_factor_diff);
                
                TransformationDetails {
                    target_type,
                    details,
                }
            } else {
                // No transformation
                TransformationDetails {
                    target_type: unit_name.to_string(),
                    details: format!("**{}** (no change)", unit_name),
                }
            }
        } else {
            // Unknown unit
            TransformationDetails {
                target_type: unit_name.to_string(),
                details: format!("  **{}**: Unknown unit", unit_name),
            }
        }
    }

    /// Generate detailed transformation explanation
    fn generate_transformation_explanation(&self, unit_name: &str, _target_type: &str, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16), scale_factor_diff: i16) -> String {
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) = dimensions;
        
        // Generate the dimensional analysis
        let mut dim_parts = Vec::new();
        if mass_exp != 0 { dim_parts.push(format!("kg^{}", mass_exp)); }
        if length_exp != 0 { dim_parts.push(format!("m^{}", length_exp)); }
        if time_exp != 0 { dim_parts.push(format!("s^{}", time_exp)); }
        if current_exp != 0 { dim_parts.push(format!("A^{}", current_exp)); }
        if temp_exp != 0 { dim_parts.push(format!("K^{}", temp_exp)); }
        if amount_exp != 0 { dim_parts.push(format!("mol^{}", amount_exp)); }
        if lum_exp != 0 { dim_parts.push(format!("cd^{}", lum_exp)); }
        if angle_exp != 0 { dim_parts.push(format!("rad^{}", angle_exp)); }
        
        let original_dims = dim_parts.join(" * ");
        
        // Generate the transformed dimensions
        let mut transformed_parts = Vec::new();
        if mass_exp != 0 { 
            let mass_unit = if self.unit_gets_transformed_in_local_context("kg") { "kg" } else { "kg" };
            transformed_parts.push(format!("{}^{}", mass_unit, mass_exp)); 
        }
        if length_exp != 0 { 
            let length_unit = if self.unit_gets_transformed_in_local_context("m") { "mm" } else { "m" };
            transformed_parts.push(format!("{}^{}", length_unit, length_exp)); 
        }
        if time_exp != 0 { 
            let time_unit = if self.unit_gets_transformed_in_local_context("s") { "s" } else { "s" };
            transformed_parts.push(format!("{}^{}", time_unit, time_exp)); 
        }
        if current_exp != 0 { 
            let current_unit = if self.unit_gets_transformed_in_local_context("A") { "A" } else { "A" };
            transformed_parts.push(format!("{}^{}", current_unit, current_exp)); 
        }
        if temp_exp != 0 { 
            let temp_unit = if self.unit_gets_transformed_in_local_context("K") { "K" } else { "K" };
            transformed_parts.push(format!("{}^{}", temp_unit, temp_exp)); 
        }
        if amount_exp != 0 { 
            let amount_unit = if self.unit_gets_transformed_in_local_context("mol") { "mol" } else { "mol" };
            transformed_parts.push(format!("{}^{}", amount_unit, amount_exp)); 
        }
        if lum_exp != 0 { 
            let lum_unit = if self.unit_gets_transformed_in_local_context("cd") { "cd" } else { "cd" };
            transformed_parts.push(format!("{}^{}", lum_unit, lum_exp)); 
        }
        if angle_exp != 0 { 
            let angle_unit = if self.unit_gets_transformed_in_local_context("rad") { "rad" } else { "rad" };
            transformed_parts.push(format!("{}^{}", angle_unit, angle_exp)); 
        }
        
        let transformed_dims = transformed_parts.join(" * ");
        
        // Get the proper prefixed unit name
        let prefixed_unit_name = self.get_prefixed_unit_name(unit_name, scale_factor_diff);
        
        // Generate transformation explanation as individual lines
        let mut lines = Vec::new();
        lines.push(format!("**{}** = {}", unit_name, original_dims));
        
        // Add transformation steps
        if scale_factor_diff != 0 {
            // Find which dimensions are being transformed
            if length_exp != 0 && self.unit_gets_transformed_in_local_context("m") {
                lines.push(format!("       ↓ (length: m → mm, factor: 10^-3)"));
                if length_exp != 1 {
                    lines.push(format!("       ↓ (exponent: {}, total factor: 10^{})", length_exp, scale_factor_diff));
                }
            }
            if mass_exp != 0 && self.unit_gets_transformed_in_local_context("kg") {
                lines.push(format!("       ↓ (mass: kg → g, factor: 10^3)"));
                if mass_exp != 1 {
                    lines.push(format!("       ↓ (exponent: {}, total factor: 10^{})", mass_exp, scale_factor_diff));
                }
            }
            // Add other dimension transformations as needed
        }
        
        lines.push(format!("       = {}", transformed_dims));
        lines.push(format!("       = **{}**", prefixed_unit_name));
        
        // Join with newlines for the details string
        lines.join("\n")
    }

    /// Get the proper prefixed unit name from scale factor difference
    fn get_prefixed_unit_name(&self, unit_name: &str, scale_factor_diff: i16) -> String {
        use whippyunits_default_dimensions::SI_PREFIXES;
        
        // Find the prefix that matches the scale factor difference
        if let Some(prefix_info) = SI_PREFIXES.iter().find(|p| p.scale_factor == scale_factor_diff) {
            // Use the Unicode symbol for micro (μ) instead of 'u' for better display
            let prefix_symbol = if prefix_info.symbol == "u" { "μ" } else { prefix_info.symbol };
            format!("{}{}", prefix_symbol, unit_name)
        } else {
            unit_name.to_string()
        }
    }

    /// Get the long name for a unit (e.g., "J" -> "Joule", "W" -> "Watt")
    fn get_unit_long_name(&self, unit_name: &str) -> String {
        use whippyunits_default_dimensions::lookup_unit_literal;
        
        if let Some((_dimension, unit)) = lookup_unit_literal(unit_name) {
            unit.long_name.to_string()
        } else {
            unit_name.to_string()
        }
    }
}

/// Helper struct for transformation details
struct TransformationDetails {
    target_type: String,
    details: String,
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
