#[cfg(not(test))]
extern crate alloc;

use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Caret, Slash, Star, Dot};
use syn::{Ident, LitInt};

#[cfg(not(test))]
use alloc::boxed::Box;
#[cfg(not(test))]
use alloc::string::ToString;
#[cfg(not(test))]
use alloc::vec::Vec;

use crate::{
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
                        if unit.system == crate::System::Metric {
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
                        if unit.system == crate::System::Metric {
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

