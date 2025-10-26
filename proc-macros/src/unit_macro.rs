use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Caret, Comma, Slash, Star, Dot};
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

        // Handle both * and . as multiplication operators (UCUM format uses .)
        while input.peek(Star) || input.peek(Dot) {
            if input.peek(Star) {
                let _star: Star = input.parse()?;
            } else if input.peek(Dot) {
                let _dot: Dot = input.parse()?;
            }
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
            // Handle numeric literals like "1" in "1 / m" or "10" in "10^4 m"
            let lit: syn::LitInt = input.parse()?;
            let base_value: i32 = lit.base10_parse()?;
            
            // Check if this is followed by a caret (^) for power notation
            if input.peek(Caret) {
                let _caret: Caret = input.parse()?;
                let exponent_lit: LitInt = input.parse()?;
                let exponent: i32 = exponent_lit.base10_parse()?;
                
                // Handle power-of-10 expressions like "10^4"
                if base_value == 10 {
                    // This is a power-of-10 scale factor
                    // We need to create a special unit that represents this scale
                    // For now, we'll create a unit with the appropriate scale factor
                    Ok(UnitExpr::Unit(UnitExprUnit {
                        name: syn::Ident::new("power_of_10", proc_macro2::Span::call_site()),
                        exponent: exponent as i16,
                    }))
                } else {
                    // For other bases, treat as regular power expression
                    Ok(UnitExpr::Pow(
                        Box::new(UnitExpr::Unit(UnitExprUnit {
                            name: syn::Ident::new("dimensionless", proc_macro2::Span::call_site()),
                            exponent: 1,
                        })),
                        exponent_lit,
                    ))
                }
            } else {
                // Regular numeric literal - check if it's a power of 10
                if base_value == 10 {
                    // Treat "10" as "10^1" for power-of-10 scale factors
                    Ok(UnitExpr::Unit(UnitExprUnit {
                        name: syn::Ident::new("power_of_10", proc_macro2::Span::call_site()),
                        exponent: 1,
                    }))
                } else {
                    // Other numeric literals - treat as dimensionless
                    Ok(UnitExpr::Unit(UnitExprUnit {
                        name: syn::Ident::new("dimensionless", proc_macro2::Span::call_site()),
                        exponent: 1,
                    }))
                }
            }
        } else {
            let ident: Ident = input.parse()?;
            
            // Check for implicit exponent notation (UCUM format like "s2" instead of "s^2")
            let ident_str = ident.to_string();
            if let Some(pos) = ident_str.chars().position(|c| c.is_ascii_digit()) {
                let base_name = &ident_str[..pos];
                let exp_str = &ident_str[pos..];
                if let Ok(exp) = exp_str.parse::<i16>() {
                    // This is implicit exponent notation
                    let base_ident = syn::Ident::new(base_name, ident.span());
                    Ok(UnitExpr::Unit(UnitExprUnit {
                        name: base_ident,
                        exponent: exp,
                    }))
                } else {
                    // Not a valid exponent, treat as regular unit
                    Ok(UnitExpr::Unit(UnitExprUnit {
                        name: ident,
                        exponent: 1,
                    }))
                }
            } else {
                // Regular unit identifier
                Ok(UnitExpr::Unit(UnitExprUnit {
                    name: ident,
                    exponent: 1,
                }))
            }
        }
    }


    /// Evaluate the unit expression to get dimension exponents and scale factors
    pub fn evaluate(&self) -> UnitEvaluationResult {
        match self {
            UnitExpr::Unit(unit) => {
                // Handle special power-of-10 scale factors
                if unit.name.to_string() == "power_of_10" {
                    return UnitEvaluationResult {
                        dimension_exponents: DynDimensionExponents::ZERO,
                        scale_exponents: ScaleExponents::_10(unit.exponent),
                    };
                }
                
                if let Some(unit_info) = get_unit_info(&unit.name.to_string()) {
                    // Get the dimension exponents and scale exponents from the unit
                    let mut dimension_exponents = unit_info.exponents.value();
                    let mut scale_exponents = unit_info.scale;

                    // Check if this is a prefixed unit and adjust scale factors accordingly
                    // BUT ONLY if the unit name is NOT a valid unit symbol by itself
                    let is_valid_unit_symbol =
                        Dimension::find_unit_by_symbol(&unit.name.to_string()).is_some();
                    if !is_valid_unit_symbol {
                        // Try all prefixes until we find one with a valid base unit
                        for prefix in SiPrefix::ALL {
                            // Try prefix symbol first (e.g., "kW" -> "W")
                            if let Some(base) = prefix.strip_prefix_symbol(&unit.name.to_string()) {
                                if !base.is_empty() {
                                    // Check if the base unit exists
                                    if Dimension::find_unit_by_symbol(base).is_some() {
                                        let prefix_factor = prefix.factor_log10();
                                        // Apply the prefix factor to the scale factors (powers of 2 and 5 for log10)
                                        scale_exponents =
                                            scale_exponents.mul(ScaleExponents::_10(prefix_factor));
                                        break;
                                    }
                                }
                            }
                            // Try prefix name (e.g., "kilowatt" -> "watt")
                            if let Some(base) = prefix.strip_prefix_name(&unit.name.to_string()) {
                                if !base.is_empty() {
                                    // Check if the base unit exists by name
                                    if Dimension::find_unit_by_name(base).is_some() {
                                        let prefix_factor = prefix.factor_log10();
                                        // Apply the prefix factor to the scale factors (powers of 2 and 5 for log10)
                                        scale_exponents =
                                            scale_exponents.mul(ScaleExponents::_10(prefix_factor));
                                        break;
                                    }
                                }
                            }
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
    // Only allow prefixing of base units (first unit in each dimension) and only for metric units
    for prefix in SiPrefix::ALL {
        if let Some(base) = prefix.strip_prefix_symbol(unit_name) {
            if !base.is_empty() {
                // Check if the base unit exists and is a base unit (first unit in its dimension)
                if let Some((unit, dimension)) = Dimension::find_unit_by_symbol(base) {
                    // Check if this is the first unit in its dimension (base unit)
                    if dimension.units.first().map(|first_unit| first_unit.name == unit.name).unwrap_or(false) {
                        // Only allow prefixing if the base unit is a metric unit (not imperial)
                        if unit.system == whippyunits_core::System::Metric {
                            return Some(unit);
                        }
                    }
                }
            }
        }
        if let Some(base) = prefix.strip_prefix_name(unit_name) {
            if !base.is_empty() {
                // Check if the base unit exists by name and is a base unit
                if let Some((unit, dimension)) = Dimension::find_unit_by_name(base) {
                    // Check if this is the first unit in its dimension (base unit)
                    if dimension.units.first().map(|first_unit| first_unit.name == unit.name).unwrap_or(false) {
                        // Only allow prefixing if the base unit is a metric unit (not imperial)
                        if unit.system == whippyunits_core::System::Metric {
                            return Some(unit);
                        }
                    }
                }
            }
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
        let doc_structs = Self::generate_unit_documentation_for_expr(
            &self.unit_expr,
            mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp,
            p2, p3, p5, pi
        );

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

    /// Generate documentation structs for the unit expression using dimension and scale exponents
    /// This uses the same authoritative typename logic as default declarators
    fn generate_unit_documentation_for_expr(
        unit_expr: &UnitExpr,
        _mass_exp: i16,
        _length_exp: i16,
        _time_exp: i16,
        _current_exp: i16,
        _temperature_exp: i16,
        _amount_exp: i16,
        _luminosity_exp: i16,
        _angle_exp: i16,
        _p2: i16,
        _p3: i16,
        _p5: i16,
        _pi: i16,
    ) -> TokenStream {
        // Extract identifiers from the unit expression
        let mut identifiers = Vec::new();
        Self::collect_identifiers_from_expr(unit_expr, &mut identifiers);
        
        // Generate documentation for each identifier
        let doc_structs: Vec<TokenStream> = identifiers.into_iter().map(|ident| {
            let doc_comment = Self::generate_unit_doc_comment(&ident.to_string());
            quote! {
                const _: () = {
                    #doc_comment
                    #[allow(non_camel_case_types)]
                    type #ident = ();
                };
            }
        }).collect();

        quote! {
            #(#doc_structs)*
        }
    }

    /// Recursively collect identifiers from a unit expression
    fn collect_identifiers_from_expr(expr: &UnitExpr, identifiers: &mut Vec<Ident>) {
        match expr {
            UnitExpr::Unit(unit) => {
                identifiers.push(unit.name.clone());
            }
            UnitExpr::Mul(left, right) => {
                Self::collect_identifiers_from_expr(left, identifiers);
                Self::collect_identifiers_from_expr(right, identifiers);
            }
            UnitExpr::Div(left, right) => {
                Self::collect_identifiers_from_expr(left, identifiers);
                Self::collect_identifiers_from_expr(right, identifiers);
            }
            UnitExpr::Pow(base, _) => {
                Self::collect_identifiers_from_expr(base, identifiers);
            }
        }
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
            unit_info
        } else {
            format!("{} ({})", unit_name.to_uppercase(), unit_name)
        }
    }

    /// Get unit documentation information from whippyunits-core data
    fn get_unit_doc_info(unit_name: &str) -> Option<String> {
        // First check for exact unit match (prioritize exact matches over prefix matches)
        if let Some((unit, _dimension)) = Dimension::find_unit_by_symbol(unit_name) {
            // Use the first symbol from unit.symbols as the abbreviation
            let symbol = unit.symbols.first().unwrap_or(&unit_name);
            return Some(format!("{} ({})", unit.name, symbol));
        }
        
        if let Some((unit, _dimension)) = Dimension::find_unit_by_name(unit_name) {
            // Use the first symbol from unit.symbols as the abbreviation
            let symbol = unit.symbols.first().unwrap_or(&unit_name);
            return Some(format!("{} ({})", unit.name, symbol));
        }

        // Only if no exact match found, check if it's a prefixed unit
        if let Some((prefix_symbol, _base_symbol)) = Self::parse_prefixed_unit(unit_name) {
            use whippyunits_core::{SiPrefix, to_unicode_superscript, Dimension};
            if let Some(prefix_info) = SiPrefix::from_symbol(&prefix_symbol) {
                // PARSE: Get the abstract representation (prefix + base unit)
                let (base_unit_name, base_unit_symbol) = if let Some((base_unit, _)) = Dimension::find_unit_by_symbol(&_base_symbol) {
                    (base_unit.name, base_unit.symbols.first().unwrap_or(&base_unit.name))
                } else if let Some((base_unit, _)) = Dimension::find_unit_by_name(&_base_symbol) {
                    (base_unit.name, base_unit.symbols.first().unwrap_or(&base_unit.name))
                } else {
                    (_base_symbol.as_str(), &_base_symbol.as_str())
                };
                
                // TRANSFORM: Convert abstract representation to normalized display format
                let scale_text = if prefix_info.factor_log10() == 0 {
                    "10⁰".to_string()
                } else {
                    format!("10{}", to_unicode_superscript(prefix_info.factor_log10(), false))
                };
                
                let prefixed_unit_name = format!("{}{}", prefix_info.name(), base_unit_name);
                let prefixed_symbol = format!("{}{}", prefix_info.symbol(), base_unit_symbol);
                
                return Some(format!(
                    "{} ({}) - Prefix: {} ({}), Base: {}",
                    prefixed_unit_name,
                    prefixed_symbol,
                    prefix_info.name(),
                    scale_text,
                    base_unit_name
                ));
            }
        }

        None
    }

    /// Parse a unit name to extract prefix and base unit
    ///
    /// This function now uses the centralized parsing logic from whippyunits-core.
    /// Only allows prefixing of base units (first unit in each dimension by declaration order).
    fn parse_prefixed_unit(unit_name: &str) -> Option<(String, String)> {
        // Try to strip any prefix from the unit name
        if let Some((prefix, base)) = SiPrefix::strip_any_prefix_symbol(unit_name) {
            // Check if the base unit exists and is a base unit (first unit in its dimension)
            if let Some((unit, dimension)) = Dimension::find_unit_by_symbol(base) {
                // Check if this is the first unit in its dimension (base unit)
                if dimension.units.first().map(|first_unit| first_unit.name == unit.name).unwrap_or(false) {
                    // Only allow prefixing if the base unit is a metric unit (not imperial)
                    if unit.system == whippyunits_core::System::Metric {
                        return Some((prefix.symbol().to_string(), base.to_string()));
                    }
                }
            }
        }

        // Also try stripping prefix from name (not just symbol)
        if let Some((prefix, base)) = SiPrefix::strip_any_prefix_name(unit_name) {
            // Check if the base unit exists by name and is a base unit
            if let Some((unit, dimension)) = Dimension::find_unit_by_name(base) {
                // Check if this is the first unit in its dimension (base unit)
                if dimension.units.first().map(|first_unit| first_unit.name == unit.name).unwrap_or(false) {
                    // Only allow prefixing if the base unit is a metric unit (not imperial)
                    if unit.system == whippyunits_core::System::Metric {
                        return Some((prefix.symbol().to_string(), base.to_string()));
                    }
                }
            }
        }

        None
    }


    /// Get the storage unit name from scale exponents and dimension exponents
    /// This uses the exact same logic as the prettyprint to ensure consistency
    fn get_storage_unit_name(
        p2: i16, p3: i16, p5: i16, pi: i16,
        mass_exp: i16, length_exp: i16, time_exp: i16, current_exp: i16,
        temperature_exp: i16, amount_exp: i16, luminosity_exp: i16, angle_exp: i16
    ) -> String {
        use whippyunits_core::{
            scale_exponents::ScaleExponents, 
            dimension_exponents::DynDimensionExponents,
            storage_unit::{UnitLiteralConfig, generate_unit_literal}
        };
        
        // Create scale exponents from the parameters
        let scale_factors = ScaleExponents([p2, p3, p5, pi]);
        
        // Create dimension exponents from the parameters
        let dimension_exponents = DynDimensionExponents([mass_exp, length_exp, time_exp, current_exp, temperature_exp, amount_exp, luminosity_exp, angle_exp]);
        
        // Use the exact same logic as prettyprint by calling the same functions from core
        // This ensures the proc macro generates the same storage unit names as the inlay hints
        let unit_literal = generate_unit_literal(
            dimension_exponents,
            scale_factors,
            UnitLiteralConfig {
                verbose: true, // Use long names for storage units
                prefer_si_units: true,
            },
        );
        
        // If we got a unit literal, use it; otherwise fall back to systematic generation
        if !unit_literal.is_empty() {
            unit_literal
        } else {
            // Fallback to systematic generation
            use whippyunits_core::storage_unit::generate_systematic_unit_name;
            let exponents_vec = dimension_exponents.0.to_vec();
            generate_systematic_unit_name(exponents_vec, true)
        }
    }
}
