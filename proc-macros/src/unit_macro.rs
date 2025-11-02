use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::Comma;
use syn::{Ident, Type};
use whippyunits_core::{Dimension, SiPrefix, UnitExpr};

use crate::utils::shared_utils::get_declarator_type_for_unit;

/// Input for the unit macro
pub struct UnitMacroInput {
    pub unit_expr: UnitExpr,
    pub storage_type: Option<Type>,
    pub brand_type: Option<Type>,
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

        // Check if there's another comma followed by a brand type parameter
        let brand_type = if input.peek(Comma) {
            let _comma: Comma = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(UnitMacroInput {
            unit_expr,
            storage_type,
            brand_type,
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

        // Use the specified brand type or default to ()
        let brand_type = self
            .brand_type
            .unwrap_or_else(|| syn::parse_str::<Type>("()").unwrap());

        // Generate documentation structs for unit identifiers in const expression
        let doc_structs = Self::generate_unit_documentation_for_expr(
            &self.unit_expr,
            mass_exp,
            length_exp,
            time_exp,
            current_exp,
            temp_exp,
            amount_exp,
            lum_exp,
            angle_exp,
            p2,
            p3,
            p5,
            pi,
        );

        // Generate the actual quantity type
        let quantity_type = quote! {
            whippyunits::quantity_type::Quantity<
                whippyunits::quantity_type::Scale<whippyunits::quantity_type::_2<#p2>, whippyunits::quantity_type::_3<#p3>, whippyunits::quantity_type::_5<#p5>, whippyunits::quantity_type::_Pi<#pi>>,
                whippyunits::quantity_type::Dimension<whippyunits::quantity_type::_M<#mass_exp>, whippyunits::quantity_type::_L<#length_exp>, whippyunits::quantity_type::_T<#time_exp>, whippyunits::quantity_type::_I<#current_exp>, whippyunits::quantity_type::_Θ<#temp_exp>, whippyunits::quantity_type::_N<#amount_exp>, whippyunits::quantity_type::_J<#lum_exp>, whippyunits::quantity_type::_A<#angle_exp>>,
                #storage_type,
                #brand_type
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
        let doc_structs: Vec<TokenStream> = identifiers
            .into_iter()
            .map(|ident: Ident| {
                let doc_comment = Self::generate_unit_doc_comment(&ident.to_string());
                let unit_name = ident.to_string();

                // Get the proper declarator type for this unit
                if let Some(declarator_type) = get_declarator_type_for_unit(&unit_name) {
                    quote! {
                        const _: () = {
                            #doc_comment
                            #[allow(non_camel_case_types)]
                            type #ident = #declarator_type;
                        };
                    }
                } else {
                    // Fallback for units without declarator types
                    quote! {
                        const _: () = {
                            #doc_comment
                            #[allow(non_camel_case_types)]
                            type #ident = ();
                        };
                    }
                }
            })
            .collect();

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
            use whippyunits_core::{to_unicode_superscript, Dimension, SiPrefix};
            if let Some(prefix_info) = SiPrefix::from_symbol(&prefix_symbol) {
                // PARSE: Get the abstract representation (prefix + base unit)
                let (base_unit_name, base_unit_symbol) = if let Some((base_unit, _)) =
                    Dimension::find_unit_by_symbol(&_base_symbol)
                {
                    (
                        base_unit.name,
                        base_unit.symbols.first().unwrap_or(&base_unit.name),
                    )
                } else if let Some((base_unit, _)) = Dimension::find_unit_by_name(&_base_symbol) {
                    (
                        base_unit.name,
                        base_unit.symbols.first().unwrap_or(&base_unit.name),
                    )
                } else {
                    (_base_symbol.as_str(), &_base_symbol.as_str())
                };

                // TRANSFORM: Convert abstract representation to normalized display format
                let scale_text = if prefix_info.factor_log10() == 0 {
                    "10⁰".to_string()
                } else {
                    format!(
                        "10{}",
                        to_unicode_superscript(prefix_info.factor_log10(), false)
                    )
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
                if dimension
                    .units
                    .first()
                    .map(|first_unit| first_unit.name == unit.name)
                    .unwrap_or(false)
                {
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
                if dimension
                    .units
                    .first()
                    .map(|first_unit| first_unit.name == unit.name)
                    .unwrap_or(false)
                {
                    // Only allow prefixing if the base unit is a metric unit (not imperial)
                    if unit.system == whippyunits_core::System::Metric {
                        return Some((prefix.symbol().to_string(), base.to_string()));
                    }
                }
            }
        }

        None
    }
}
