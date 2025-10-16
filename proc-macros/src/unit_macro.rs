use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Caret, Comma, Slash, Star};
use syn::{Ident, LitInt, Type};
use whippyunits_core::{
    dimension_exponents::{DimensionExponents, DynDimensionExponents},
    scale_exponents::ScaleExponents,
    Dimension, SiPrefix, Unit,
};

/// Represents a unit with optional exponent
#[derive(Debug, Clone)]
pub struct UnitExprUnit {
    pub name: Ident,
    pub exponent: i16,
}

/// Result of evaluating a unit expression
#[derive(Debug, Clone, Copy)]
pub struct UnitEvaluationResult {
    pub dimension_exponents: DynDimensionExponents,
    pub scale_exponents: ScaleExponents,
}

/// Represents a unit expression that can be parsed
#[derive(Clone)]
pub enum UnitExpr {
    Unit(UnitExprUnit),
    Mul(Box<UnitExpr>, Box<UnitExpr>),
    Div(Box<UnitExpr>, Box<UnitExpr>),
    Pow(Box<UnitExpr>, LitInt),
}

impl Parse for UnitExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut left = Self::parse_factor(input)?;

        while input.peek(Slash) {
            let _slash: Slash = input.parse()?;
            let right = Self::parse_factor(input)?;
            left = UnitExpr::Div(Box::new(left), Box::new(right));
        }

        Ok(left)
    }
}

impl UnitExpr {
    fn parse_factor(input: ParseStream) -> Result<Self> {
        let mut left = Self::parse_power(input)?;

        while input.peek(Star) {
            let _star: Star = input.parse()?;
            let right = Self::parse_power(input)?;
            left = UnitExpr::Mul(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    /// Collect all unit identifiers used in this expression
    pub fn collect_unit_identifiers(&self) -> Vec<Ident> {
        let mut identifiers = Vec::new();
        self.collect_identifiers_recursive(&mut identifiers);
        identifiers
    }

    fn collect_identifiers_recursive(&self, identifiers: &mut Vec<Ident>) {
        match self {
            UnitExpr::Unit(unit) => {
                identifiers.push(unit.name.clone());
            }
            UnitExpr::Mul(a, b) => {
                a.collect_identifiers_recursive(identifiers);
                b.collect_identifiers_recursive(identifiers);
            }
            UnitExpr::Div(a, b) => {
                a.collect_identifiers_recursive(identifiers);
                b.collect_identifiers_recursive(identifiers);
            }
            UnitExpr::Pow(base, _) => {
                base.collect_identifiers_recursive(identifiers);
            }
        }
    }

    fn parse_power(input: ParseStream) -> Result<Self> {
        let base = Self::parse_atom(input)?;

        if input.peek(Caret) {
            let _caret: Caret = input.parse()?;
            let exponent: LitInt = input.parse()?;
            Ok(UnitExpr::Pow(Box::new(base), exponent))
        } else {
            Ok(base)
        }
    }

    fn parse_atom(input: ParseStream) -> Result<Self> {
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            content.parse()
        } else if input.peek(syn::LitInt) {
            // Handle numeric literals like "1" in "1 / m"
            let _lit: syn::LitInt = input.parse()?;
            // For now, we'll treat numeric literals as dimensionless
            // In a more sophisticated implementation, we could handle them properly
            Ok(UnitExpr::Unit(UnitExprUnit {
                name: syn::Ident::new("dimensionless", proc_macro2::Span::call_site()),
                exponent: 1,
            }))
        } else {
            let ident: Ident = input.parse()?;
            Ok(UnitExpr::Unit(UnitExprUnit {
                name: ident,
                exponent: 1,
            }))
        }
    }

    /// Convert the unit expression to a string representation
    pub fn to_string(&self) -> String {
        match self {
            UnitExpr::Unit(unit) => {
                if unit.exponent == 1 {
                    unit.name.to_string()
                } else {
                    format!("{}^{}", unit.name, unit.exponent)
                }
            }
            UnitExpr::Mul(a, b) => {
                format!("{} * {}", a.to_string(), b.to_string())
            }
            UnitExpr::Div(a, b) => {
                format!("{} / {}", a.to_string(), b.to_string())
            }
            UnitExpr::Pow(base, exp) => {
                format!("{}^{}", base.to_string(), exp)
            }
        }
    }

    /// Evaluate the unit expression to get dimension exponents and scale factors
    pub fn evaluate(&self) -> UnitEvaluationResult {
        match self {
            UnitExpr::Unit(unit) => {
                if let Some(unit_info) = get_unit_info(&unit.name.to_string()) {
                    // Get the dimension exponents and scale exponents from the unit
                    let mut dimension_exponents = unit_info.exponents.value();
                    let mut scale_exponents = unit_info.scale;

                    // Check if this is a prefixed unit and adjust scale factors accordingly
                    // BUT ONLY if the unit name is NOT a valid unit symbol by itself
                    let is_valid_unit_symbol =
                        Dimension::find_unit_by_symbol(&unit.name.to_string()).is_some();
                    if !is_valid_unit_symbol {
                        if let Some((prefix, _base)) =
                            SiPrefix::strip_any_prefix_symbol(&unit.name.to_string())
                        {
                            let prefix_factor = prefix.factor_log10();
                            // Apply the prefix factor to the scale factors (powers of 2 and 5 for log10)
                            scale_exponents =
                                scale_exponents.mul(ScaleExponents::_10(prefix_factor));
                        }
                    }

                    // Apply the unit exponent to both dimension and scale exponents
                    if unit.exponent != 1 {
                        dimension_exponents = dimension_exponents * unit.exponent;
                        scale_exponents = scale_exponents.scalar_exp(unit.exponent);
                    }

                    UnitEvaluationResult {
                        dimension_exponents,
                        scale_exponents,
                    }
                } else {
                    // Handle dimensionless or unknown units
                    UnitEvaluationResult {
                        dimension_exponents: DynDimensionExponents::ZERO,
                        scale_exponents: ScaleExponents::IDENTITY,
                    }
                }
            }
            UnitExpr::Mul(a, b) => {
                let result_a = a.evaluate();
                let result_b = b.evaluate();
                UnitEvaluationResult {
                    dimension_exponents: result_a.dimension_exponents
                        + result_b.dimension_exponents,
                    scale_exponents: result_a.scale_exponents.mul(result_b.scale_exponents),
                }
            }
            UnitExpr::Div(a, b) => {
                let result_a = a.evaluate();
                let result_b = b.evaluate();
                UnitEvaluationResult {
                    dimension_exponents: result_a.dimension_exponents
                        + (-result_b.dimension_exponents),
                    scale_exponents: result_a.scale_exponents.mul(result_b.scale_exponents.neg()),
                }
            }
            UnitExpr::Pow(base, exp) => {
                let result = base.evaluate();
                let exp_val: i16 = exp.base10_parse().unwrap();
                UnitEvaluationResult {
                    dimension_exponents: result.dimension_exponents * exp_val,
                    scale_exponents: result.scale_exponents.scalar_exp(exp_val),
                }
            }
        }
    }
}

// Removed parse_unit_name - now using centralized parsing from whippyunits-core

// Removed duplicate parsing functions - now using centralized parsing from whippyunits-core

/// Get unit information for a unit name, handling prefixes and conversions
/// Returns the complete Unit struct with dimensions and scale factors
pub fn get_unit_info(unit_name: &str) -> Option<&'static Unit> {
    // Handle dimensionless units (like "1" in "1 / km")
    if unit_name == "dimensionless" {
        return None; // Dimensionless units don't have a corresponding Unit
    }

    // First check if this is a unit literal (like min, h, hr, d, g, m, s, etc.)
    // This must come before prefix checking to avoid false positives
    if let Some((unit, _dimension)) = Dimension::find_unit_by_symbol(unit_name) {
        return Some(unit);
    }

    if let Some((unit, _dimension)) = Dimension::find_unit_by_name(unit_name) {
        return Some(unit);
    }

    // Then check if this is a prefixed unit (like kg, kW, mm, etc.)
    if let Some((_prefix, base)) = SiPrefix::strip_any_prefix_symbol(unit_name) {
        // Check if the base unit exists
        if let Some((unit, _dimension)) = Dimension::find_unit_by_symbol(base) {
            // For prefixed units, we need to return a unit with adjusted scale factors
            // But since we can't modify the static unit, we need to handle this differently
            // The scale factor adjustment should be handled in the evaluation logic
            return Some(unit);
        }
    }

    // If not found, return None
    None
}

/// Input for the unit macro
pub struct UnitMacroInput {
    pub unit_expr: UnitExpr,
    pub storage_type: Option<Type>,
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

        Ok(UnitMacroInput {
            unit_expr,
            storage_type,
        })
    }
}

impl UnitMacroInput {
    pub fn expand(self) -> TokenStream {
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

        // Generate documentation structs for unit identifiers in const expression
        let doc_structs = Self::generate_unit_documentation_for_expr(&self.unit_expr);

        // Generate the actual quantity type
        let quantity_type = quote! {
            whippyunits::quantity_type::Quantity<
                whippyunits::quantity_type::Scale<whippyunits::quantity_type::_2<#p2>, whippyunits::quantity_type::_3<#p3>, whippyunits::quantity_type::_5<#p5>, whippyunits::quantity_type::_Pi<#pi>>,
                whippyunits::quantity_type::Dimension<whippyunits::quantity_type::_M<#mass_exp>, whippyunits::quantity_type::_L<#length_exp>, whippyunits::quantity_type::_T<#time_exp>, whippyunits::quantity_type::_I<#current_exp>, whippyunits::quantity_type::_Θ<#temp_exp>, whippyunits::quantity_type::_N<#amount_exp>, whippyunits::quantity_type::_J<#lum_exp>, whippyunits::quantity_type::_A<#angle_exp>>,
                #storage_type
            >
        };

        quote! {
            <whippyunits::Helper<{
                #doc_structs
                0
            }, #quantity_type> as whippyunits::GetSecondGeneric>::Type
        }
    }

    /// Generate documentation structs for each unit identifier in the expression
    fn generate_unit_documentation_for_expr(unit_expr: &UnitExpr) -> TokenStream {
        let unit_identifiers = unit_expr.collect_unit_identifiers();
        let mut doc_structs = Vec::new();

        for identifier in unit_identifiers {
            if let Some(doc_struct) = Self::generate_single_unit_doc(&identifier) {
                doc_structs.push(doc_struct);
            }
        }

        quote! {
            #(#doc_structs)*
        }
    }

    /// Generate documentation for a single unit identifier
    fn generate_single_unit_doc(identifier: &Ident) -> Option<TokenStream> {
        let unit_name = identifier.to_string();
        let doc_comment = Self::generate_unit_doc_comment(&unit_name);

        // Create a new identifier with the same span as the original
        let doc_ident = syn::Ident::new(&unit_name, identifier.span());

        // Get the corresponding default declarator type
        let declarator_type = Self::get_declarator_type_for_unit(&unit_name)?;

        Some(quote! {
            const _: () = {
                #doc_comment
                #[allow(non_camel_case_types)]
                type #doc_ident = #declarator_type;
            };
        })
    }

    /// Generate documentation comment for a unit
    fn generate_unit_doc_comment(unit_name: &str) -> TokenStream {
        let doc_text = Self::get_unit_documentation_text(unit_name);
        quote! {
            #[doc = #doc_text]
        }
    }

    /// Get documentation text for a unit
    fn get_unit_documentation_text(unit_name: &str) -> String {
        // Try to get information from the whippyunits-core data
        if let Some(unit_info) = Self::get_unit_doc_info(unit_name) {
            format!("Unit: {} - {}", unit_name, unit_info)
        } else {
            format!("Unit: {}", unit_name)
        }
    }

    /// Get unit documentation information from whippyunits-core data
    fn get_unit_doc_info(unit_name: &str) -> Option<String> {
        // Use the new get_unit_info function to get the Unit struct
        if let Some(unit) = get_unit_info(unit_name) {
            // Check if it's a prefixed unit
            if let Some((prefix_symbol, _base_symbol)) = Self::parse_prefixed_unit(unit_name) {
                use whippyunits_core::SiPrefix;
                if let Some(prefix_info) = SiPrefix::from_symbol(&prefix_symbol) {
                    let scale_text = if prefix_info.factor_log10() == 0 {
                        "10^0".to_string()
                    } else {
                        format!("10^{}", prefix_info.factor_log10())
                    };
                    return Some(format!(
                        "Prefix: {} ({}), Base: {}",
                        prefix_info.name(),
                        scale_text,
                        unit.name
                    ));
                }
            }

            // Regular unit
            return Some(format!("Unit: {}", unit.name));
        }

        None
    }

    /// Parse a unit name to extract prefix and base unit
    ///
    /// This function now uses the centralized parsing logic from whippyunits-core.
    fn parse_prefixed_unit(unit_name: &str) -> Option<(String, String)> {
        // Try to strip any prefix from the unit name
        if let Some((prefix, base)) = SiPrefix::strip_any_prefix_symbol(unit_name) {
            // Check if the base unit exists
            if Dimension::find_unit_by_symbol(base).is_some() {
                return Some((prefix.symbol().to_string(), base.to_string()));
            }
        }

        // Also try stripping prefix from name (not just symbol)
        if let Some((prefix, base)) = SiPrefix::strip_any_prefix_name(unit_name) {
            // Check if the base unit exists by name
            if Dimension::find_unit_by_name(base).is_some() {
                return Some((prefix.symbol().to_string(), base.to_string()));
            }
        }

        None
    }

    /// Get the corresponding default declarator type for a unit
    fn get_declarator_type_for_unit(unit_name: &str) -> Option<TokenStream> {
        // Use the shared helper function to avoid code duplication
        crate::get_declarator_type_for_unit(unit_name)
    }
}
