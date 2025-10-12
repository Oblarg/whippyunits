use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};
use whippyunits_default_dimensions::{
    lookup_unit_literal, is_prefixed_base_unit,
    scale_type_to_actual_unit_symbol, lookup_si_prefix, SI_PREFIXES,
};

// Import the UnitExpr type from unit_macro
use crate::unit_macro::UnitExpr;

/// Visitor pattern for traversing UnitExpr and generating different types of output
pub trait UnitExprVisitor<T> {
    fn visit_unit(&self, unit: &crate::unit_macro::Unit) -> T;
    fn visit_div(&self, numerator: &UnitExpr, denominator: &UnitExpr) -> T;
    fn visit_mul(&self, left: &UnitExpr, right: &UnitExpr) -> T;
    fn visit_pow(&self, base: &UnitExpr, exponent: i16) -> T;
}

/// Generic visitor implementation that can be parameterized with different strategies
pub struct UnitExprFormatter<F> {
    pub unit_formatter: F,
}

impl<F> UnitExprFormatter<F>
where
    F: Fn(&crate::unit_macro::Unit) -> String,
{
    pub fn new(unit_formatter: F) -> Self {
        Self { unit_formatter }
    }

    pub fn format(&self, expr: &UnitExpr) -> String {
        match expr {
            UnitExpr::Unit(unit) => (self.unit_formatter)(unit),
            UnitExpr::Div(numerator, denominator) => {
                let num_str = self.format(numerator);
                let den_str = self.format(denominator);
                format!("{} / {}", num_str, den_str)
            },
            UnitExpr::Mul(left, right) => {
                let left_str = self.format(left);
                let right_str = self.format(right);
                format!("{} * {}", left_str, right_str)
            },
            UnitExpr::Pow(base, exponent) => {
                let base_str = self.format(base);
                format!("{}^{}", base_str, exponent)
            },
        }
    }
}

/// Processor for handling dimension operations
pub struct DimensionProcessor {
    pub dimensions: (i16, i16, i16, i16, i16, i16, i16, i16),
}

impl DimensionProcessor {
    pub fn new(dimensions: (i16, i16, i16, i16, i16, i16, i16, i16)) -> Self {
        Self { dimensions }
    }

    /// Apply a function to each non-zero dimension
    pub fn apply_to_each<F>(&self, mut f: F) 
    where 
        F: FnMut(&str, i16) 
    {
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) = self.dimensions;
        
        if mass_exp != 0 { f("kg", mass_exp); }
        if length_exp != 0 { f("m", length_exp); }
        if time_exp != 0 { f("s", time_exp); }
        if current_exp != 0 { f("A", current_exp); }
        if temp_exp != 0 { f("K", temp_exp); }
        if amount_exp != 0 { f("mol", amount_exp); }
        if lum_exp != 0 { f("cd", lum_exp); }
        if angle_exp != 0 { f("rad", angle_exp); }
    }

    /// Check if this represents a simple base unit (single dimension = 1, others = 0)
    pub fn is_simple_base_unit(&self) -> bool {
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) = self.dimensions;
        let non_zero_count = [mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp]
            .iter()
            .filter(|&&x| x != 0)
            .count();
        non_zero_count == 1 && [mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp]
            .iter()
            .any(|&x| x == 1)
    }

    /// Get the scale identifier for simple base units
    pub fn get_scale_identifier(&self, mass_scale: &Ident, length_scale: &Ident, time_scale: &Ident, 
                               current_scale: &Ident, temperature_scale: &Ident, amount_scale: &Ident, 
                               luminosity_scale: &Ident, angle_scale: &Ident) -> Option<Ident> {
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) = self.dimensions;
        
        match (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) {
            (1, 0, 0, 0, 0, 0, 0, 0) => Some(mass_scale.clone()),
            (0, 1, 0, 0, 0, 0, 0, 0) => Some(length_scale.clone()),
            (0, 0, 1, 0, 0, 0, 0, 0) => Some(time_scale.clone()),
            (0, 0, 0, 1, 0, 0, 0, 0) => Some(current_scale.clone()),
            (0, 0, 0, 0, 1, 0, 0, 0) => Some(temperature_scale.clone()),
            (0, 0, 0, 0, 0, 1, 0, 0) => Some(amount_scale.clone()),
            (0, 0, 0, 0, 0, 0, 1, 0) => Some(luminosity_scale.clone()),
            (0, 0, 0, 0, 0, 0, 0, 1) => Some(angle_scale.clone()),
            _ => None, // Compound unit
        }
    }
}

/// Pipeline for resolving unit symbols with transformations
pub struct UnitSymbolPipeline<'a> {
    pub unit_name: &'a str,
    pub local_context: &'a LocalContext,
}

impl<'a> UnitSymbolPipeline<'a> {
    pub fn new(unit_name: &'a str, local_context: &'a LocalContext) -> Self {
        Self { unit_name, local_context }
    }

    pub fn resolve_symbol(&self) -> String {
        if let Some((dimension, _)) = lookup_unit_literal(self.unit_name) {
            let dimensions = dimension.exponents;
            
            // Check if this unit gets transformed
            if self.unit_gets_transformed_in_local_context() {
                // Calculate the scale factor difference
                let scale_factor_diff = self.calculate_scale_factor_difference(dimensions);
                
                // Get the prefixed unit name
                self.get_prefixed_unit_name(scale_factor_diff)
            } else {
                // Check if this is a time unit that needs conversion (like h → s)
                if let Some(_time_conversion) = self.get_time_unit_conversion() {
                    // For time units with conversion factors, show the base unit (s)
                    "s".to_string()
                } else if self.is_prefixed_unit() {
                    // Even if the unit doesn't get transformed in local context,
                    // if it's a prefixed unit, we should show the base unit in the trace
                    self.get_base_unit_name()
                } else {
                    self.unit_name.to_string()
                }
            }
        } else {
            self.unit_name.to_string()
        }
    }

    fn unit_gets_transformed_in_local_context(&self) -> bool {
        if let Some((dimension, _)) = lookup_unit_literal(self.unit_name) {
            let dimensions = dimension.exponents;
            let processor = DimensionProcessor::new(dimensions);
            
            // If it's a simple base unit, check if it gets transformed
            if processor.get_scale_identifier(
                &self.local_context.mass_scale,
                &self.local_context.length_scale,
                &self.local_context.time_scale,
                &self.local_context.current_scale,
                &self.local_context.temperature_scale,
                &self.local_context.amount_scale,
                &self.local_context.luminosity_scale,
                &self.local_context.angle_scale,
            ).is_some() {
                // For simple base units, check if there's a scale factor difference
                let scale_factor_diff = self.calculate_scale_factor_difference(dimensions);
                return scale_factor_diff != 0;
            }
            
            // For compound units, check if any of their base units get transformed
            return self.compound_unit_gets_transformed(dimensions);
        }
        
        false
    }

    fn compound_unit_gets_transformed(&self, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16)) -> bool {
        let processor = DimensionProcessor::new(dimensions);
        let mut gets_transformed = false;
        
        processor.apply_to_each(|unit_name, _exp| {
            if self.unit_gets_transformed_in_local_context_for_unit(unit_name) {
                gets_transformed = true;
            }
        });
        
        gets_transformed
    }

    fn unit_gets_transformed_in_local_context_for_unit(&self, unit_name: &str) -> bool {
        let pipeline = UnitSymbolPipeline::new(unit_name, self.local_context);
        pipeline.unit_gets_transformed_in_local_context()
    }

    fn calculate_scale_factor_difference(&self, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16)) -> i16 {
        let processor = DimensionProcessor::new(dimensions);
        let mut total_scale_diff = 0;
        
        processor.apply_to_each(|unit_name, exp| {
            let scale_diff = self.get_scale_difference_for_base_unit(unit_name);
            total_scale_diff += scale_diff * exp;
        });
        
        total_scale_diff
    }

    fn get_scale_difference_for_base_unit(&self, default_unit: &str) -> i16 {
        use whippyunits_default_dimensions::{scale_type_to_actual_unit_symbol, is_prefixed_base_unit, lookup_si_prefix};
        
        // Get the local unit symbol based on the unit type
        let local_unit_symbol = match default_unit {
            "kg" => scale_type_to_actual_unit_symbol(&self.local_context.mass_scale.to_string()).unwrap_or_else(|| "kg".to_string()),
            "m" => scale_type_to_actual_unit_symbol(&self.local_context.length_scale.to_string()).unwrap_or_else(|| "m".to_string()),
            "s" => scale_type_to_actual_unit_symbol(&self.local_context.time_scale.to_string()).unwrap_or_else(|| "s".to_string()),
            "A" => scale_type_to_actual_unit_symbol(&self.local_context.current_scale.to_string()).unwrap_or_else(|| "A".to_string()),
            "K" => scale_type_to_actual_unit_symbol(&self.local_context.temperature_scale.to_string()).unwrap_or_else(|| "K".to_string()),
            "mol" => scale_type_to_actual_unit_symbol(&self.local_context.amount_scale.to_string()).unwrap_or_else(|| "mol".to_string()),
            "cd" => scale_type_to_actual_unit_symbol(&self.local_context.luminosity_scale.to_string()).unwrap_or_else(|| "cd".to_string()),
            "rad" => scale_type_to_actual_unit_symbol(&self.local_context.angle_scale.to_string()).unwrap_or_else(|| "rad".to_string()),
            _ => default_unit.to_string(),
        };
        
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

    fn get_prefixed_unit_name(&self, scale_factor_diff: i16) -> String {
        use whippyunits_default_dimensions::SI_PREFIXES;
        
        // Find the prefix that matches the scale factor difference
        if let Some(prefix_info) = SI_PREFIXES.iter().find(|p| p.scale_factor == scale_factor_diff) {
            // Use the Unicode symbol for micro (μ) instead of 'u' for better display
            let prefix_symbol = if prefix_info.symbol == "u" { "μ" } else { prefix_info.symbol };
            
            // If this is a prefixed unit, apply the prefix to the base unit, not the original unit
            let base_unit = if self.is_prefixed_unit() {
                self.get_base_unit_name()
            } else {
                self.unit_name.to_string()
            };
            
            format!("{}{}", prefix_symbol, base_unit)
        } else {
            self.unit_name.to_string()
        }
    }

    fn is_prefixed_unit(&self) -> bool {
        // Check if it's a prefixed base unit
        if is_prefixed_base_unit(self.unit_name).is_some() {
            return true;
        }
        
        // Check if it's a prefixed compound unit
        if let Some((_base_symbol, _prefix)) = is_prefixed_compound_unit(self.unit_name) {
            return true;
        }
        
        false
    }

    fn get_base_unit_name(&self) -> String {
        // Check if it's a prefixed base unit
        if let Some((base_symbol, _prefix)) = is_prefixed_base_unit(self.unit_name) {
            return base_symbol.to_string();
        }
        
        // Check if it's a prefixed compound unit
        if let Some((base_symbol, _prefix)) = is_prefixed_compound_unit(self.unit_name) {
            return base_symbol.to_string();
        }
        
        self.unit_name.to_string()
    }

    fn get_time_unit_conversion(&self) -> Option<String> {
        use whippyunits_default_dimensions::lookup_unit_literal;
        
        if let Some((_dimension, unit)) = lookup_unit_literal(self.unit_name) {
            // Check if this is a time unit with a conversion factor
            if let Some(scale_factors) = unit.scale_factors {
                // Calculate the conversion factor from scale factors
                let (p2, p3, p5, pi) = scale_factors;
                let conversion_factor = 2.0_f64.powi(p2 as i32) * 3.0_f64.powi(p3 as i32) * 5.0_f64.powi(p5 as i32) * std::f64::consts::PI.powi(pi as i32);
                
                if conversion_factor != 1.0 {
                    return Some(format!("{} → s, factor: {}", self.unit_name, conversion_factor as i32));
                }
            }
        }
        
        None
    }
}

/// Generic quote generator for different unit types
pub struct QuoteGenerator<'a> {
    pub storage_type: &'a Type,
    pub lift_trace_doc_shadows: &'a TokenStream,
}

impl<'a> QuoteGenerator<'a> {
    pub fn new(storage_type: &'a Type, lift_trace_doc_shadows: &'a TokenStream) -> Self {
        Self { storage_type, lift_trace_doc_shadows }
    }

    pub fn generate_for_simple_base_unit(&self, scale_ident: &Ident) -> TokenStream {
        let lift_trace_doc_shadows = self.lift_trace_doc_shadows;
        let storage_type = self.storage_type;
        quote! {
            <whippyunits::Helper<{
                #lift_trace_doc_shadows
                0
            }, whippyunits::default_declarators::#scale_ident<#storage_type>> as whippyunits::GetSecondGeneric>::Type
        }
    }

    pub fn generate_for_compound_unit(&self, unit_expr_parsed: &syn::Expr) -> TokenStream {
        let lift_trace_doc_shadows = self.lift_trace_doc_shadows;
        let storage_type = self.storage_type;
        quote! {
            <whippyunits::Helper<{
                #lift_trace_doc_shadows
                0
            }, whippyunits::unit!(#unit_expr_parsed, #storage_type)> as whippyunits::GetSecondGeneric>::Type
        }
    }
}

/// Context information needed for local unit transformations
pub struct LocalContext {
    pub mass_scale: Ident,
    pub length_scale: Ident,
    pub time_scale: Ident,
    pub current_scale: Ident,
    pub temperature_scale: Ident,
    pub amount_scale: Ident,
    pub luminosity_scale: Ident,
    pub angle_scale: Ident,
}

/// Check if a unit symbol is a prefixed compound unit (kPa, mW, etc.)
pub fn is_prefixed_compound_unit(unit_symbol: &str) -> Option<(&str, &str)> {
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

/// Unit transformation logic for local contexts
impl LocalContext {
    /// Check if a unit gets transformed in the local context
    pub fn unit_gets_transformed_in_local_context(&self, unit_name: &str) -> bool {
        // Check if this is a simple base unit that maps to a local scale
        if let Some((dimension, _)) = lookup_unit_literal(unit_name) {
            let dimensions = dimension.exponents;
            let processor = DimensionProcessor::new(dimensions);
            
            // If it's a simple base unit, check if it gets transformed
            if processor.get_scale_identifier(
                &self.mass_scale,
                &self.length_scale,
                &self.time_scale,
                &self.current_scale,
                &self.temperature_scale,
                &self.amount_scale,
                &self.luminosity_scale,
                &self.angle_scale,
            ).is_some() {
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
    pub fn compound_unit_gets_transformed(&self, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16)) -> bool {
        let processor = DimensionProcessor::new(dimensions);
        let mut gets_transformed = false;
        
        processor.apply_to_each(|unit_name, _exp| {
            if self.unit_gets_transformed_in_local_context(unit_name) {
                gets_transformed = true;
            }
        });
        
        gets_transformed
    }

    /// Calculate the scale factor difference between local and default units
    pub fn calculate_scale_factor_difference(&self, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16)) -> i16 {
        let processor = DimensionProcessor::new(dimensions);
        let mut total_scale_diff = 0;
        
        processor.apply_to_each(|unit_name, exp| {
            let scale_diff = self.get_scale_difference_for_base_unit(unit_name);
            total_scale_diff += scale_diff * exp;
        });
        
        total_scale_diff
    }

    /// Get the scale difference for a specific base unit using centralized utilities
    pub fn get_scale_difference_for_base_unit(&self, default_unit: &str) -> i16 {
        // Get the local unit symbol based on the unit type
        let local_unit_symbol = match default_unit {
            "kg" => scale_type_to_actual_unit_symbol(&self.mass_scale.to_string()).unwrap_or_else(|| "kg".to_string()),
            "m" => scale_type_to_actual_unit_symbol(&self.length_scale.to_string()).unwrap_or_else(|| "m".to_string()),
            "s" => scale_type_to_actual_unit_symbol(&self.time_scale.to_string()).unwrap_or_else(|| "s".to_string()),
            "A" => scale_type_to_actual_unit_symbol(&self.current_scale.to_string()).unwrap_or_else(|| "A".to_string()),
            "K" => scale_type_to_actual_unit_symbol(&self.temperature_scale.to_string()).unwrap_or_else(|| "K".to_string()),
            "mol" => scale_type_to_actual_unit_symbol(&self.amount_scale.to_string()).unwrap_or_else(|| "mol".to_string()),
            "cd" => scale_type_to_actual_unit_symbol(&self.luminosity_scale.to_string()).unwrap_or_else(|| "cd".to_string()),
            "rad" => scale_type_to_actual_unit_symbol(&self.angle_scale.to_string()).unwrap_or_else(|| "rad".to_string()),
            _ => default_unit.to_string(),
        };
        
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
    pub fn find_prefixed_type_by_scale_factor(&self, unit_name: &str, scale_factor_diff: i16) -> Option<TokenStream> {
        // Find the prefix that matches the scale factor difference
        for prefix_info in SI_PREFIXES {
            if prefix_info.scale_factor == scale_factor_diff {
                // Get the base unit name (e.g., "kW" -> "W")
                let base_unit_name = self.get_base_unit_name(unit_name);
                
                // Try to find a prefixed version of the base unit
                let prefixed_unit_name = format!("{}{}", prefix_info.symbol, base_unit_name);
                if let Some(declarator_type) = crate::get_declarator_type_for_unit(&prefixed_unit_name) {
                    return Some(declarator_type);
                }
            }
        }
        
        None
    }
}

/// Unit symbol and name utilities
impl LocalContext {
    /// Get the proper prefixed unit name from scale factor difference
    pub fn get_prefixed_unit_name(&self, unit_name: &str, scale_factor_diff: i16) -> String {
        // Find the prefix that matches the scale factor difference
        if let Some(prefix_info) = SI_PREFIXES.iter().find(|p| p.scale_factor == scale_factor_diff) {
            // Use the Unicode symbol for micro (μ) instead of 'u' for better display
            let prefix_symbol = if prefix_info.symbol == "u" { "μ" } else { prefix_info.symbol };
            
            // If this is a prefixed unit, apply the prefix to the base unit, not the original unit
            let base_unit = if self.is_prefixed_unit(unit_name) {
                self.get_base_unit_name(unit_name)
            } else {
                unit_name.to_string()
            };
            
            format!("{}{}", prefix_symbol, base_unit)
        } else {
            unit_name.to_string()
        }
    }

    /// Get the long name for a unit (e.g., "J" -> "Joule", "W" -> "Watt")
    pub fn get_unit_long_name(&self, unit_name: &str) -> String {
        if let Some((_dimension, unit)) = lookup_unit_literal(unit_name) {
            unit.long_name.to_string()
        } else {
            unit_name.to_string()
        }
    }

    /// Check if a unit is a prefixed unit (like kW, mW, etc.)
    pub fn is_prefixed_unit(&self, unit_name: &str) -> bool {
        // Check if it's a prefixed base unit
        if is_prefixed_base_unit(unit_name).is_some() {
            return true;
        }
        
        // Check if it's a prefixed compound unit
        if let Some((_base_symbol, _prefix)) = is_prefixed_compound_unit(unit_name) {
            return true;
        }
        
        false
    }

    /// Get the base unit name from a prefixed unit (e.g., "kW" -> "W")
    pub fn get_base_unit_name(&self, unit_name: &str) -> String {
        // Check if it's a prefixed base unit
        if let Some((base_symbol, _prefix)) = is_prefixed_base_unit(unit_name) {
            return base_symbol.to_string();
        }
        
        // Check if it's a prefixed compound unit
        if let Some((base_symbol, _prefix)) = is_prefixed_compound_unit(unit_name) {
            return base_symbol.to_string();
        }
        
        unit_name.to_string()
    }

    /// Get time unit conversion information (e.g., "h" -> "h → s, factor: 3600")
    pub fn get_time_unit_conversion(&self, unit_name: &str) -> Option<String> {
        if let Some((_dimension, unit)) = lookup_unit_literal(unit_name) {
            // Check if this is a time unit with a conversion factor
            if let Some(scale_factors) = unit.scale_factors {
                // Calculate the conversion factor from scale factors
                let (p2, p3, p5, pi) = scale_factors;
                let conversion_factor = 2.0_f64.powi(p2 as i32) * 3.0_f64.powi(p3 as i32) * 5.0_f64.powi(p5 as i32) * std::f64::consts::PI.powi(pi as i32);
                
                if conversion_factor != 1.0 {
                    return Some(format!("{} → s, factor: {}", unit_name, conversion_factor as i32));
                }
            }
        }
        
        None
    }
}

/// Helper struct for transformation details
pub struct TransformationDetails {
    pub target_type: String,
    pub details: String,
}

/// Transformation details generation
impl LocalContext {
    /// Generate enhanced lift trace for a specific identifier with bolded formatting
    pub fn generate_enhanced_lift_trace_for_identifier(&self, identifier: &Ident, _lift_trace: &str) -> String {
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
    pub fn bold_identifier_in_expression(&self, expression: &str, identifier: &str) -> String {
        // Simple replacement - in a more sophisticated implementation, we could parse the expression
        // and only bold the identifier when it appears as a standalone unit
        expression.replace(identifier, &format!("**{}**", identifier))
    }

    /// Get transformation details for a specific identifier
    pub fn get_transformation_details_for_identifier(&self, unit_name: &str) -> TransformationDetails {
        if let Some((dimension, _)) = lookup_unit_literal(unit_name) {
            let dimensions = dimension.exponents;
            
            // Check if this unit gets transformed
            if self.unit_gets_transformed_in_local_context(unit_name) {
                // Calculate the scale factor difference
                let scale_factor_diff = self.calculate_scale_factor_difference(dimensions);
                
                // Get the target type
                let target_type = if let Some(scale_ident) = DimensionProcessor::new(dimensions).get_scale_identifier(
                    &self.mass_scale,
                    &self.length_scale,
                    &self.time_scale,
                    &self.current_scale,
                    &self.temperature_scale,
                    &self.amount_scale,
                    &self.luminosity_scale,
                    &self.angle_scale,
                ) {
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
                // No transformation in local context, but check for time unit conversions
                let mut details = format!("**{}**", unit_name);
                
                // Check if this is a time unit that needs conversion
                if let Some(time_conversion) = self.get_time_unit_conversion(unit_name) {
                    details.push_str(&format!(" = {}", time_conversion));
                } else {
                    details.push_str(" (no change)");
                }
                
                TransformationDetails {
                    target_type: unit_name.to_string(),
                    details,
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
    pub fn generate_transformation_explanation(&self, unit_name: &str, _target_type: &str, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16), scale_factor_diff: i16) -> String {
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) = dimensions;
        
        // Check if this is a prefixed unit that needs to show prefix dropping
        let is_prefixed_unit = self.is_prefixed_unit(unit_name);
        let base_unit_name = if is_prefixed_unit {
            self.get_base_unit_name(unit_name)
        } else {
            unit_name.to_string()
        };
        
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
        
        // Show prefix dropping if this is a prefixed unit
        if is_prefixed_unit {
            lines.push(format!("**{}** = {} (drop prefix: {} → {})", base_unit_name, original_dims, unit_name, base_unit_name));
        } else {
            lines.push(format!("**{}** = {}", unit_name, original_dims));
        }
        
        // Add transformation steps for scale changes
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
        
        // Add time unit conversions (like hour → seconds)
        if time_exp != 0 {
            if let Some(time_conversion) = self.get_time_unit_conversion(unit_name) {
                lines.push(format!("       ↓ ({})", time_conversion));
            }
        }
        
        lines.push(format!("       = {}", transformed_dims));
        lines.push(format!("       = **{}**", prefixed_unit_name));
        
        // Join with newlines for the details string
        lines.join("\n")
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

    /// Generate the output unit expression string for the lift trace
    pub fn generate_output_unit_expression_string(&self) -> String {
        // This would need to be implemented based on the unit expression
        // For now, return a placeholder
        "output_expr".to_string()
    }

    /// Shared helper to generate unit expression string with given base units
    pub fn generate_unit_expression_with_base_units(&self, _base_units: &[(&str, &str); 8]) -> String {
        // This would need the unit expression to evaluate
        // For now, return a placeholder
        "unit_expr".to_string()
    }
}
