use proc_macro2::TokenStream;
use quote::quote;
use syn::token::Comma;
use syn::{
    parse::{Parse, ParseStream, Result},
    Ident, Type,
};
use whippyunits_core::api_helpers::{
    scale_type_to_actual_unit_symbol, lookup_unit_literal, get_units_by_exponents, dyn_exponents_to_tuple_format,
};
use whippyunits_core::dimension_exponents::DynDimensionExponents;

/// Convert dimension exponents to a unit expression
/// This converts dimension exponents back to a unit expression string using the provided base units
fn dimension_exponents_to_unit_expression_with_base_units(exponents: DynDimensionExponents, base_units: &[(&str, &str); 8]) -> String {
    let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) = (
        exponents.0[0], exponents.0[1], exponents.0[2], exponents.0[3],
        exponents.0[4], exponents.0[5], exponents.0[6], exponents.0[7]
    );
    
    let mut terms = Vec::new();
    
    // Add each dimension with non-zero exponent
    if mass_exp != 0 {
        terms.push(format!("{}^{}", base_units[0].0, mass_exp));
    }
    if length_exp != 0 {
        terms.push(format!("{}^{}", base_units[1].0, length_exp));
    }
    if time_exp != 0 {
        terms.push(format!("{}^{}", base_units[2].0, time_exp));
    }
    if current_exp != 0 {
        terms.push(format!("{}^{}", base_units[3].0, current_exp));
    }
    if temp_exp != 0 {
        terms.push(format!("{}^{}", base_units[4].0, temp_exp));
    }
    if amount_exp != 0 {
        terms.push(format!("{}^{}", base_units[5].0, amount_exp));
    }
    if lum_exp != 0 {
        terms.push(format!("{}^{}", base_units[6].0, lum_exp));
    }
    if angle_exp != 0 {
        terms.push(format!("{}^{}", base_units[7].0, angle_exp));
    }
    
    if terms.is_empty() {
        "1".to_string() // dimensionless
    } else {
        terms.join(" * ")
    }
}

/// Convert dimension exponents to a unit expression (legacy function for backward compatibility)
/// This converts dimension exponents back to a unit expression string using SI base units
fn dimension_exponents_to_unit_expression(exponents: DynDimensionExponents) -> String {
    // Use SI base units as default
    let base_units = [
        ("kg", "kg"), ("m", "m"), ("s", "s"), ("A", "A"),
        ("K", "K"), ("mol", "mol"), ("cd", "cd"), ("rad", "rad")
    ];
    dimension_exponents_to_unit_expression_with_base_units(exponents, &base_units)
}

// Import the UnitExpr type from unit_macro
use crate::unit_macro::UnitExpr;
use crate::lift_trace::{UnitExprFormatter, DimensionProcessor, QuoteGenerator, LocalContext, is_prefixed_compound_unit};

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
        
        // Use the specified storage type or default to f64
        let storage_type = self
            .storage_type
            .clone()
            .unwrap_or_else(|| syn::parse_str::<Type>("f64").unwrap());

        // Generate lift trace doc shadows for each unit identifier in the expression
        let lift_trace_doc_shadows = self.generate_lift_trace_doc_shadows_for_expr(&lift_trace, &storage_type);

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
    fn generate_lift_trace_doc_shadows_for_expr(&self, _lift_trace: &str, storage_type: &Type) -> TokenStream {
        let unit_identifiers = self.unit_expr.collect_unit_identifiers();
        let mut doc_shadows = Vec::new();

        // Generate a comprehensive trace for the entire expression
        let comprehensive_trace = self.generate_comprehensive_lift_trace();

        for identifier in unit_identifiers {
            let doc_shadow = self.generate_lift_trace_doc_shadow_for_identifier(&identifier, &comprehensive_trace, storage_type);
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

    /// Generate the final unit expression string (showing the most appropriate unit name)
    fn generate_final_unit_expression_string(&self) -> String {
        let transformed = self.generate_transformed_unit_expression_string();
        
        // Evaluate the dimensions of the transformed expression to find the most appropriate unit name
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp, _p2, _p3, _p5, _pi) = self.evaluate_dimensions();
        
        // Try to find a common unit name for these dimensions
        if let Some(common_unit_name) = self.find_common_unit_name_for_dimensions((mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp)) {
            common_unit_name
        } else {
            // If no common unit name found, return the transformed expression
            transformed
        }
    }

    /// Get the local context for unit transformations
    fn get_local_context(&self) -> LocalContext {
        LocalContext {
            mass_scale: self.mass_scale.clone(),
            length_scale: self.length_scale.clone(),
            time_scale: self.time_scale.clone(),
            current_scale: self.current_scale.clone(),
            temperature_scale: self.temperature_scale.clone(),
            amount_scale: self.amount_scale.clone(),
            luminosity_scale: self.luminosity_scale.clone(),
            angle_scale: self.angle_scale.clone(),
        }
    }

    /// Get the transformed unit symbol for a given unit name
    fn get_transformed_unit_symbol(&self, unit_name: &str) -> String {
        if let Some((dimension, _)) = lookup_unit_literal(unit_name) {
            let dimensions = dimension.exponents;
            
            // Check if this unit gets transformed
            if self.unit_gets_transformed_in_local_context(unit_name) {
                // Calculate the scale factor difference
                let scale_factor_diff = self.calculate_scale_factor_difference(dyn_exponents_to_tuple_format(dimensions));
                
                // Get the prefixed unit name
                self.get_prefixed_unit_name(unit_name, scale_factor_diff)
            } else {
                // Check if this is a time unit that needs conversion (like h → s)
                if let Some(_time_conversion) = self.get_time_unit_conversion(unit_name) {
                    // For time units with conversion factors, show the base unit (s)
                    "s".to_string()
                } else if self.is_prefixed_unit(unit_name) {
                    // Even if the unit doesn't get transformed in local context,
                    // if it's a prefixed unit, we should show the base unit in the trace
                    self.get_base_unit_name(unit_name)
                } else {
                    unit_name.to_string()
                }
            }
        } else {
            unit_name.to_string()
        }
    }

    /// Generate a comprehensive lift trace for the entire expression
    fn generate_comprehensive_lift_trace(&self) -> String {
        let _original_expr = self.generate_input_unit_expression_decomposed_string();
        let _output_expr = self.generate_output_unit_expression_string();
        
        let mut trace = String::new();
        
        // Add summary line showing the overall transformation in unit symbols
        let input_expr = self.generate_input_unit_expression_string();
        let transformed_expr = self.generate_transformed_unit_expression_string();
        let final_expr = self.generate_final_unit_expression_string();
        
        // Show the transformation chain: input → transformed = final
        if transformed_expr != final_expr {
            trace.push_str(&format!("{} → {} = {}<br><br>", input_expr, transformed_expr, final_expr));
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
    fn generate_lift_trace_doc_shadow_for_identifier(&self, identifier: &Ident, lift_trace: &str, storage_type: &Type) -> TokenStream {
        // Create a new identifier with the same span as the original, using upper camel case
        let doc_ident_name = format!("Local{}", identifier.to_string().to_uppercase());
        let doc_ident = syn::Ident::new(&doc_ident_name, identifier.span());
        
        // Determine the target type for this unit identifier in the local context
        let target_type = self.get_local_target_type_for_unit(&identifier.to_string(), storage_type);
        
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
        let formatter = UnitExprFormatter::new(|unit| unit.name.to_string());
        formatter.format(&self.unit_expr)
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
        // Debug: print the scale type names
        eprintln!("Scale type names: mass={}, length={}, time={}", 
                 self.mass_scale, self.length_scale, self.time_scale);
        
        let mass_base = scale_type_to_actual_unit_symbol(&self.mass_scale.to_string()).unwrap_or_else(|| "g".to_string());
        let length_base = scale_type_to_actual_unit_symbol(&self.length_scale.to_string()).unwrap_or_else(|| "m".to_string());
        let time_base = scale_type_to_actual_unit_symbol(&self.time_scale.to_string()).unwrap_or_else(|| "s".to_string());
        let current_base = scale_type_to_actual_unit_symbol(&self.current_scale.to_string()).unwrap_or_else(|| "A".to_string());
        let temperature_base = scale_type_to_actual_unit_symbol(&self.temperature_scale.to_string()).unwrap_or_else(|| "K".to_string());
        let amount_base = scale_type_to_actual_unit_symbol(&self.amount_scale.to_string()).unwrap_or_else(|| "mol".to_string());
        let luminosity_base = scale_type_to_actual_unit_symbol(&self.luminosity_scale.to_string()).unwrap_or_else(|| "cd".to_string());
        let angle_base = scale_type_to_actual_unit_symbol(&self.angle_scale.to_string()).unwrap_or_else(|| "rad".to_string());
        
        // Debug: print the converted unit symbols
        eprintln!("Converted unit symbols: mass={}, length={}, time={}", 
                 mass_base, length_base, time_base);
        
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
        dimension_exponents_to_unit_expression_with_base_units(DynDimensionExponents([mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp]), base_units)
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
            if let Some((dimension, _)) = lookup_unit_literal(&base_symbol) {
                let dimensions = dimension.exponents;
                self.generate_unit_expression_from_dimensions(dyn_exponents_to_tuple_format(dimensions))
            } else {
                // Fallback to original unit
                unit_name.to_string()
            }
        } else {
            // For non-prefixed single units, use the original logic
            if let Some((dimension, _)) = lookup_unit_literal(unit_name) {
                let dimensions = dimension.exponents;
                // Check if it's a simple base unit
                if let Some(scale_ident) = self.get_scale_for_dimensions(dyn_exponents_to_tuple_format(dimensions)) {
                    scale_ident.to_string()
                } else {
                    // It's a compound unit - generate the unit expression
                    self.generate_unit_expression_from_dimensions(dyn_exponents_to_tuple_format(dimensions))
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
        let processor = DimensionProcessor::new(dimensions);
        processor.get_scale_identifier(
            &self.mass_scale,
            &self.length_scale,
            &self.time_scale,
            &self.current_scale,
            &self.temperature_scale,
            &self.amount_scale,
            &self.luminosity_scale,
            &self.angle_scale,
        )
    }

    /// Convert local base units array to string references for use in dimension_exponents_to_unit_expression
    fn get_local_base_units_refs(&self) -> [(String, String); 8] {
        self.get_local_base_units()
    }

    /// Generate a quote! block for a simple base unit with the given scale identifier
    fn generate_simple_base_unit_quote(&self, scale_ident: &Ident, storage_type: &Type, lift_trace_doc_shadows: &TokenStream) -> TokenStream {
        let generator = QuoteGenerator::new(storage_type, lift_trace_doc_shadows);
        generator.generate_for_simple_base_unit(scale_ident)
    }

    /// Generate a quote! block for a compound unit with the given unit expression
    fn generate_compound_unit_quote(&self, unit_expr_parsed: &syn::Expr, storage_type: &Type, lift_trace_doc_shadows: &TokenStream) -> TokenStream {
        let generator = QuoteGenerator::new(storage_type, lift_trace_doc_shadows);
        generator.generate_for_compound_unit(unit_expr_parsed)
    }

    /// Evaluate unit expression dimensions and return the exponents
    fn evaluate_dimensions(&self) -> (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) {
        self.unit_expr.evaluate()
    }

    /// Generate unit expression string from dimensions using local base units
    fn generate_unit_expression_from_dimensions(&self, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16)) -> String {
        // Decompose into component units and map each to its local scale counterpart
        let local_base_units = self.get_local_base_units_refs();
        // Debug: print the local base units
        eprintln!("Local base units: {:?}", local_base_units);
        let base_units_refs: [(&str, &str); 8] = [
            (local_base_units[0].0.as_str(), local_base_units[0].1.as_str()),
            (local_base_units[1].0.as_str(), local_base_units[1].1.as_str()),
            (local_base_units[2].0.as_str(), local_base_units[2].1.as_str()),
            (local_base_units[3].0.as_str(), local_base_units[3].1.as_str()),
            (local_base_units[4].0.as_str(), local_base_units[4].1.as_str()),
            (local_base_units[5].0.as_str(), local_base_units[5].1.as_str()),
            (local_base_units[6].0.as_str(), local_base_units[6].1.as_str()),
            (local_base_units[7].0.as_str(), local_base_units[7].1.as_str()),
        ];
        let result = dimension_exponents_to_unit_expression_with_base_units(DynDimensionExponents([dimensions.0, dimensions.1, dimensions.2, dimensions.3, dimensions.4, dimensions.5, dimensions.6, dimensions.7]), &base_units_refs);
        // Debug: print the generated unit expression
        eprintln!("Generated unit expression: {}", result);
        result
    }

    /// Handle prefixed compound unit logic for a single unit
    fn handle_prefixed_compound_unit(&self, unit_name: &str, storage_type: &Type, lift_trace_doc_shadows: &TokenStream) -> Option<TokenStream> {
        if let Some((base_symbol, _prefix)) = is_prefixed_compound_unit(unit_name) {
            if let Some((dimension, _)) = lookup_unit_literal(&base_symbol) {
                let dimensions = dimension.exponents;
                let unit_expr = self.generate_unit_expression_from_dimensions(dyn_exponents_to_tuple_format(dimensions));
                let unit_expr_parsed = syn::parse_str::<syn::Expr>(&unit_expr).unwrap_or_else(|_| {
                    syn::parse_str::<syn::Expr>(&base_symbol).unwrap()
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
            if let Some(scale_ident) = self.get_scale_for_dimensions(dyn_exponents_to_tuple_format(dimensions)) {
                self.generate_simple_base_unit_quote(&scale_ident, storage_type, lift_trace_doc_shadows)
            } else {
                // It's a compound unit - generate the unit expression
                let unit_expr = self.generate_unit_expression_from_dimensions(dyn_exponents_to_tuple_format(dimensions));
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
    fn get_local_target_type_for_unit(&self, unit_name: &str, storage_type: &Type) -> TokenStream {
        // Check if this is a time unit that needs conversion (like h → s)
        if let Some(_time_conversion) = self.get_time_unit_conversion(unit_name) {
            // For time units with conversion factors, return the target time unit declarator
            return quote! {
                whippyunits::default_declarators::Second
            };
        }
        
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
        self.get_local_transformed_type_for_unit(unit_name, storage_type)
    }

    /// Check if a unit gets transformed in the local context
    fn unit_gets_transformed_in_local_context(&self, unit_name: &str) -> bool {
        let local_context = self.get_local_context();
        local_context.unit_gets_transformed_in_local_context(unit_name)
    }

    /// Check if a compound unit gets transformed in the local context
    fn compound_unit_gets_transformed(&self, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16)) -> bool {
        let local_context = self.get_local_context();
        local_context.compound_unit_gets_transformed(dimensions)
    }

    /// Get the local transformed type for a unit (fallback when shared helper isn't sufficient)
    fn get_local_transformed_type_for_unit(&self, unit_name: &str, _storage_type: &Type) -> TokenStream {
        // For units that get transformed, we need to generate the local type
        // This should match the logic in handle_single_unit but return just the type
        
        if let Some((dimension, _)) = lookup_unit_literal(unit_name) {
            let dimensions = dimension.exponents;
            
            // Check if it's a simple base unit
            if let Some(scale_ident) = self.get_scale_for_dimensions(dyn_exponents_to_tuple_format(dimensions)) {
                // For simple base units, the target type is the local scale declarator
                quote! {
                    whippyunits::default_declarators::#scale_ident
                }
            } else {
                // For compound units, calculate the scale factor difference and find the appropriate prefixed type
                let scale_factor_diff = self.calculate_scale_factor_difference(dyn_exponents_to_tuple_format(dimensions));
                
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
        let local_context = self.get_local_context();
        local_context.calculate_scale_factor_difference(dimensions)
    }

    /// Get the scale difference for a specific base unit using centralized utilities
    fn get_scale_difference_for_base_unit(&self, default_unit: &str, _local_scale: &Ident) -> i16 {
        let local_context = self.get_local_context();
        local_context.get_scale_difference_for_base_unit(default_unit)
    }

    /// Find a prefixed type by scale factor using centralized utilities
    fn find_prefixed_type_by_scale_factor(&self, unit_name: &str, scale_factor_diff: i16) -> Option<TokenStream> {
        let local_context = self.get_local_context();
        local_context.find_prefixed_type_by_scale_factor(unit_name, scale_factor_diff)
    }

    /// Generate enhanced lift trace for a specific identifier with bolded formatting
    fn generate_enhanced_lift_trace_for_identifier(&self, identifier: &Ident, lift_trace: &str) -> String {
        let local_context = self.get_local_context();
        local_context.generate_enhanced_lift_trace_for_identifier(identifier, lift_trace)
    }

    /// Bold a specific identifier within an expression
    fn bold_identifier_in_expression(&self, expression: &str, identifier: &str) -> String {
        let local_context = self.get_local_context();
        local_context.bold_identifier_in_expression(expression, identifier)
    }

    /// Get transformation details for a specific identifier
    fn get_transformation_details_for_identifier(&self, unit_name: &str) -> crate::lift_trace::TransformationDetails {
        let local_context = self.get_local_context();
        local_context.get_transformation_details_for_identifier(unit_name)
    }

    /// Generate detailed transformation explanation
    fn generate_transformation_explanation(&self, unit_name: &str, target_type: &str, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16), scale_factor_diff: i16) -> String {
        let local_context = self.get_local_context();
        local_context.generate_transformation_explanation(unit_name, target_type, dimensions, scale_factor_diff)
    }

    /// Get the proper prefixed unit name from scale factor difference
    fn get_prefixed_unit_name(&self, unit_name: &str, scale_factor_diff: i16) -> String {
        let local_context = self.get_local_context();
        local_context.get_prefixed_unit_name(unit_name, scale_factor_diff)
    }

    /// Get the long name for a unit (e.g., "J" -> "Joule", "W" -> "Watt")
    fn get_unit_long_name(&self, unit_name: &str) -> String {
        let local_context = self.get_local_context();
        local_context.get_unit_long_name(unit_name)
    }

    /// Check if a unit is a prefixed unit (like kW, mW, etc.)
    fn is_prefixed_unit(&self, unit_name: &str) -> bool {
        let local_context = self.get_local_context();
        local_context.is_prefixed_unit(unit_name)
    }

    /// Get the base unit name from a prefixed unit (e.g., "kW" -> "W")
    fn get_base_unit_name(&self, unit_name: &str) -> String {
        let local_context = self.get_local_context();
        local_context.get_base_unit_name(unit_name)
    }

    /// Get time unit conversion information (e.g., "h" -> "h → s, factor: 3600")
    fn get_time_unit_conversion(&self, unit_name: &str) -> Option<String> {
        let local_context = self.get_local_context();
        local_context.get_time_unit_conversion(unit_name)
    }

    /// Find a common unit name for given dimensions by looking up known units
    fn find_common_unit_name_for_dimensions(&self, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16)) -> Option<String> {
        use whippyunits_core::{Dimension, SiPrefix};
        
        // Use the existing infrastructure to find units with these dimensions
        let matching_units = get_units_by_exponents(dimensions);
        
        if let Some((_dimension, unit)) = matching_units.first() {
            // Get the primary symbol for this unit
            let base_unit_symbol = unit.symbols.first()?;
            
            // Calculate the scale factor difference to determine the appropriate prefix
            let scale_factor_diff = self.calculate_scale_factor_difference(dimensions);
            
            if scale_factor_diff != 0 {
                // Find the appropriate prefix
                if let Some(prefix_info) = SiPrefix::ALL.iter().find(|p| p.factor_log10() == scale_factor_diff) {
                    let prefix_symbol = if prefix_info.symbol() == "u" { "μ" } else { prefix_info.symbol() };
                    return Some(format!("{}{}", prefix_symbol, base_unit_symbol));
                }
            } else {
                return Some(base_unit_symbol.to_string());
            }
        }
        
        None
    }
}
