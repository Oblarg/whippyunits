use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Caret, Comma, Slash, Star};
use syn::{Ident, LitInt, Type};
use whippyunits_default_dimensions::{
    BASE_UNITS, SI_PREFIXES, lookup_unit_literal, is_valid_unit_literal,
    is_prefixed_base_unit, lookup_si_prefix, get_unit_dimensions as get_unit_dimensions_from_crate,
    // Use centralized parsing functions
    parse_unit_with_prefix,
};

/// Represents a unit with optional exponent
#[derive(Debug, Clone)]
pub struct Unit {
    pub name: Ident,
    pub exponent: i16,
}

/// Represents a unit expression that can be parsed
#[derive(Clone)]
pub enum UnitExpr {
    Unit(Unit),
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
            Ok(UnitExpr::Unit(Unit {
                name: syn::Ident::new("dimensionless", proc_macro2::Span::call_site()),
                exponent: 1,
            }))
        } else {
            let ident: Ident = input.parse()?;
            Ok(UnitExpr::Unit(Unit {
                name: ident,
                exponent: 1,
            }))
        }
    }

    /// Evaluate the unit expression to get dimension exponents and scale factors
    pub fn evaluate(&self) -> (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) {
        match self {
        UnitExpr::Unit(unit) => {
            let (mass, length, time, current, temp, amount, lum, angle, p2, p3, p5, pi) =
                get_unit_dimensions(&unit.name.to_string());
            (
                mass * unit.exponent,
                length * unit.exponent,
                time * unit.exponent,
                current * unit.exponent,
                temp * unit.exponent,
                amount * unit.exponent,
                lum * unit.exponent,
                angle * unit.exponent,
                p2 * unit.exponent,
                p3 * unit.exponent,
                p5 * unit.exponent,
                pi * unit.exponent,
            )
        }
            UnitExpr::Mul(a, b) => {
                let (ma, la, ta, ca, tempa, aa, luma, anga, p2a, p3a, p5a, pia) = a.evaluate();
                let (mb, lb, tb, cb, tempb, ab, lumb, angb, p2b, p3b, p5b, pib) = b.evaluate();
                (
                    ma + mb,
                    la + lb,
                    ta + tb,
                    ca + cb,
                    tempa + tempb,
                    aa + ab,
                    luma + lumb,
                    anga + angb,
                    p2a + p2b,
                    p3a + p3b,
                    p5a + p5b,
                    pia + pib,
                )
            }
            UnitExpr::Div(a, b) => {
                let (ma, la, ta, ca, tempa, aa, luma, anga, p2a, p3a, p5a, pia) = a.evaluate();
                let (mb, lb, tb, cb, tempb, ab, lumb, angb, p2b, p3b, p5b, pib) = b.evaluate();
                (
                    ma - mb,
                    la - lb,
                    ta - tb,
                    ca - cb,
                    tempa - tempb,
                    aa - ab,
                    luma - lumb,
                    anga - angb,
                    p2a - p2b,
                    p3a - p3b,
                    p5a - p5b,
                    pia - pib,
                )
            }
            UnitExpr::Pow(base, exp) => {
                let (m, l, t, c, temp, a, lum, ang, p2, p3, p5, pi) = base.evaluate();
                let exp_val: i16 = exp.base10_parse().unwrap();
                (
                    m * exp_val,
                    l * exp_val,
                    t * exp_val,
                    c * exp_val,
                    temp * exp_val,
                    a * exp_val,
                    lum * exp_val,
                    ang * exp_val,
                    p2 * exp_val,
                    p3 * exp_val,
                    p5 * exp_val,
                    pi * exp_val,
                )
            }
        }
    }
}

// Removed parse_unit_name - now using centralized parsing from default-dimensions

// Removed duplicate parsing functions - now using centralized parsing from default-dimensions

/// Get dimension exponents and inherent scale for a base unit or compound unit
fn get_base_unit_dimensions(base_unit: &str) -> (i16, i16, i16, i16, i16, i16, i16, i16, i16) {
    // First try the base units from default-dimensions
    if let Some(base_unit_info) = BASE_UNITS.iter().find(|info| info.symbol == base_unit) {
        let (m, l, t, c, temp, a, lum, ang) = base_unit_info.dimension_exponents;
        return (
            m,
            l,
            t,
            c,
            temp,
            a,
            lum,
            ang,
            base_unit_info.prefix_scale_offset,
        );
    }

    // Try compound units from default-dimensions (now handled by lookup_unit_literal below)

    // If not found, try to find it in the shared dimension data by symbol
    if let Some((dimension, _)) = lookup_unit_literal(base_unit) {
        // Convert the dimension exponents to the format expected by the unit macro
        // The shared data has (mass, length, time, current, temperature, amount, luminosity, angle)
        // The unit macro expects (mass, length, time, current, temperature, amount, luminosity, angle, p10_offset)
        let (m, l, t, c, temp, a, lum, ang) = dimension.exponents;
        return (m, l, t, c, temp, a, lum, ang, 0); // p10_offset is 0 for SI derived units
    }

    panic!("Unknown base unit or compound unit: {}", base_unit)
}

/// Get special scale factors for time units that aren't powers of 10
fn get_time_scale_factors(base_unit: &str) -> (i16, i16, i16) {
    match base_unit {
        "s" => (0, 0, 0),   // seconds: no special factors
        "min" => (2, 1, 1), // 60s = 2^2 * 3 * 5
        "h" => (4, 2, 2),   // 3600s = 2^4 * 3^2 * 5^2
        "hr" => (4, 2, 2),  // 3600s = 2^4 * 3^2 * 5^2 (alternative symbol)
        "d" => (7, 3, 2),   // 86400s = 2^7 * 3^3 * 5^2
        "yr" => (7, 3, 2),  // year (approximate, same as day for simplicity)
        _ => (0, 0, 0),
    }
}

/// Get dimension exponents and scale factors for a unit
/// Returns (mass, length, time, current, temperature, amount, luminosity, angle, p2, p3, p5, pi)
pub fn get_unit_dimensions(
    unit_name: &str,
) -> (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) {
    // Handle dimensionless units (like "1" in "1 / km")
    if unit_name == "dimensionless" {
        return (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    }

    // First check if this is a unit literal (like min, h, hr, d)
    if let Some((dimension, unit)) = lookup_unit_literal(unit_name) {
        let (mass, length, time, current, temp, amount, lum, angle) = dimension.exponents;
        let (p2, p3, p5, pi) = unit.scale_factors.unwrap_or((0, 0, 0, 0));
        
        // Check if this is a prefixed unit and calculate the correct scale factors
        if let Some((base_symbol, prefix)) = is_prefixed_base_unit(unit_name) {
            let prefix_power = lookup_si_prefix(prefix).map(|p| p.scale_factor).unwrap_or(0);
            // Calculate final scale factors: add prefix power to base scale factors
            let final_scale_factors = (
                p2 + prefix_power,
                p3,
                p5 + prefix_power,
                pi,
            );
            
            return (
                mass, length, time, current, temp, amount, lum, angle, 
                final_scale_factors.0, final_scale_factors.1, final_scale_factors.2, final_scale_factors.3,
            );
        }
        
        return (
            mass, length, time, current, temp, amount, lum, angle, p2, p3, p5, pi,
        );
    }

    // If not found, return all zeros
    eprintln!("Unit '{}' not found, returning zero dimensions", unit_name);
    (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
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
        let (
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
        ) = self.unit_expr.evaluate();
        
        // Debug output to see what values we're getting

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
        // Try to get information from the default-dimensions data
        if let Some(unit_info) = Self::get_unit_info(unit_name) {
            format!("Unit: {} - {}", unit_name, unit_info)
        } else {
            format!("Unit: {}", unit_name)
        }
    }

    /// Get unit information from default-dimensions data
    fn get_unit_info(unit_name: &str) -> Option<String> {
        use whippyunits_default_dimensions::{BASE_UNITS, lookup_unit_literal};

        // Check base units
        if let Some(base_unit) = BASE_UNITS.iter().find(|u| u.symbol == unit_name) {
            return Some(format!("Base unit: {}", base_unit.long_name));
        }

        // Check if it's a prefixed unit FIRST (before checking compound units)
        if let Some((prefix_symbol, base_symbol)) = Self::parse_prefixed_unit(unit_name) {
            // First check if it's a prefixed base unit
            if let Some(base_unit) = BASE_UNITS.iter().find(|u| u.symbol == base_symbol) {
                // Get prefix information
                use whippyunits_default_dimensions::SI_PREFIXES;
                if let Some(prefix_info) = SI_PREFIXES.iter().find(|p| p.symbol == prefix_symbol) {
                    // Calculate the effective scale factor, accounting for base unit offset
                    let effective_scale = prefix_info.scale_factor + base_unit.prefix_scale_offset;
                    let scale_text = if effective_scale == 0 {
                        "10^0".to_string()
                    } else {
                        format!("10^{}", effective_scale)
                    };
                    return Some(format!("Prefix: {} ({}), Base: {}", 
                        prefix_info.long_name, scale_text, base_unit.long_name));
                }
            }
            
            // If not a base unit, check if it's a prefixed compound unit
            if let Some((dimension, unit)) = lookup_unit_literal(&base_symbol) {
                use whippyunits_default_dimensions::SI_PREFIXES;
                if let Some(prefix_info) = SI_PREFIXES.iter().find(|p| p.symbol == prefix_symbol) {
                    // For compound units, the scale factor is just the prefix scale factor
                    let scale_text = if prefix_info.scale_factor == 0 {
                        "10^0".to_string()
                    } else {
                        format!("10^{}", prefix_info.scale_factor)
                    };
                    return Some(format!("Prefix: {} ({}), Base: {} ({})", 
                        prefix_info.long_name, scale_text, unit.long_name, dimension.name));
                }
            }
        }

        // Check unit literals (including compound units) - only if not a prefixed unit
        if let Some((dimension, unit)) = lookup_unit_literal(unit_name) {
            return Some(format!("Unit: {} ({})", unit.long_name, dimension.name));
        }

        None
    }

    /// Parse a unit name to extract prefix and base unit
    /// 
    /// This function now uses the centralized parsing logic from default-dimensions.
    fn parse_prefixed_unit(unit_name: &str) -> Option<(String, String)> {
        let (prefix, base) = parse_unit_with_prefix(unit_name);
        if let Some(prefix) = prefix {
            Some((prefix.to_string(), base.to_string()))
        } else {
            None
        }
    }

    /// Get the corresponding default declarator type for a unit
    fn get_declarator_type_for_unit(unit_name: &str) -> Option<TokenStream> {
        use whippyunits_default_dimensions::{BASE_UNITS, lookup_unit_literal};

        // Only generate documentation for units that actually have corresponding types in default_declarators
        // This includes base units with SI prefixes and some unit literals, but NOT compound units like N, J, Pa, etc.
        
        // Skip dimensionless units - they don't have corresponding default declarator types
        if unit_name == "dimensionless" {
            return None;
        }
        
        // Check if it's a base unit (these have corresponding types)
        if let Some(base_unit) = BASE_UNITS.iter().find(|u| u.symbol == unit_name) {
            let type_name = whippyunits_default_dimensions::util::capitalize_first(&base_unit.long_name);
            let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
            return Some(quote! {
                whippyunits::default_declarators::#type_ident
            });
        }
        
        // Check if it's a prefixed unit FIRST (before checking unit literals)
        if let Some((prefix_symbol, base)) = Self::parse_prefixed_unit(unit_name) {
            // First try to find it as a prefixed base unit
            if let Some(base_unit) = BASE_UNITS.iter().find(|u| u.symbol == base) {
                // Get the prefix long name for proper type naming
                use whippyunits_default_dimensions::SI_PREFIXES;
                if let Some(prefix_info) = SI_PREFIXES.iter().find(|p| p.symbol == &prefix_symbol) {
                    // Use the same naming convention as the default declarators macro
                    let unit_singular = base_unit.long_name.trim_end_matches('s');
                    let combined_name = format!("{}{}", prefix_info.long_name, unit_singular);
                    let type_name = whippyunits_default_dimensions::util::capitalize_first(&combined_name);
                    let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
                    return Some(quote! {
                        whippyunits::default_declarators::#type_ident
                    });
                }
            }
            
            // If not a base unit, try to find it as a prefixed unit literal
            if let Some((_dimension, unit)) = lookup_unit_literal(&base) {
                use whippyunits_default_dimensions::SI_PREFIXES;
                if let Some(prefix_info) = SI_PREFIXES.iter().find(|p| p.symbol == &prefix_symbol) {
                    // Use the same naming convention as the default declarators macro
                    let unit_singular = unit.long_name.trim_end_matches('s');
                    let combined_name = format!("{}{}", prefix_info.long_name, unit_singular);
                    let type_name = whippyunits_default_dimensions::util::capitalize_first(&combined_name);
                    let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
                    return Some(quote! {
                        whippyunits::default_declarators::#type_ident
                    });
                }
            }
        }
        
        // Check if it's a unit literal that has a corresponding type - only if not a prefixed unit
        if let Some((_dimension, unit)) = lookup_unit_literal(unit_name) {
            // Use the long name to generate the type name, matching the declarator generation logic
            let type_name = whippyunits_default_dimensions::util::capitalize_first(unit.long_name);
            let type_ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
            return Some(quote! {
                whippyunits::default_declarators::#type_ident
            });
        }
        
        // For compound units (N, J, Pa, W, V, F, C, etc.) and dimensionless units (1), 
        // we don't generate documentation since they don't have corresponding default declarator types
        None
    }

}
