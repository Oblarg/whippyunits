use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Caret, Comma, Slash, Star};
use syn::{Ident, LitInt, Type};
use whippyunits_default_dimensions::{
    BASE_UNITS, COMPOUND_UNITS, DIMENSION_LOOKUP, SI_PREFIXES, UNIT_LITERALS,
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

/// Parse a unit name to extract prefix and base unit
fn parse_unit_name(unit_name: &str) -> (Option<&str>, &str) {
    // First check if the entire unit name is a valid base unit (prioritize exact matches)
    if is_valid_base_unit(unit_name) {
        return (None, unit_name);
    }

    // Check if the entire unit name is a valid compound unit (like J, W, N, etc.)
    if is_valid_compound_unit(unit_name) {
        return (None, unit_name);
    }

    // Check if the entire unit name is a valid unit literal (like min, h, hr, d, etc.)
    if is_valid_unit_literal(unit_name) {
        return (None, unit_name);
    }

    // Try to find the longest matching prefix for base units
    for prefix_info in SI_PREFIXES.iter().rev() {
        if unit_name.starts_with(prefix_info.symbol) {
            let base = &unit_name[prefix_info.symbol.len()..];
            if !base.is_empty() && is_valid_base_unit(base) {
                return (Some(prefix_info.symbol), base);
            }
        }
    }

    // Try to find the longest matching prefix for compound units (like kJ, kW, etc.)
    for prefix_info in SI_PREFIXES.iter().rev() {
        if unit_name.starts_with(prefix_info.symbol) {
            let base = &unit_name[prefix_info.symbol.len()..];
            if !base.is_empty() && is_valid_compound_unit(base) {
                return (Some(prefix_info.symbol), base);
            }
        }
    }

    // No prefix found, treat as base unit
    (None, unit_name)
}

/// Check if a string is a valid base unit
fn is_valid_base_unit(unit: &str) -> bool {
    BASE_UNITS
        .iter()
        .any(|base_unit_info| base_unit_info.symbol == unit)
}

/// Check if a string is a valid compound unit (like J, W, N, etc.)
fn is_valid_compound_unit(unit: &str) -> bool {
    COMPOUND_UNITS
        .iter()
        .any(|compound_unit_info| compound_unit_info.symbol == unit)
}

/// Check if a string is a valid unit literal (like min, h, hr, d, etc.)
fn is_valid_unit_literal(unit: &str) -> bool {
    UNIT_LITERALS
        .iter()
        .any(|unit_literal_info| unit_literal_info.symbol == unit)
}

/// Get the power of 10 for a prefix
fn get_prefix_power(prefix: &str) -> i16 {
    SI_PREFIXES
        .iter()
        .find(|prefix_info| prefix_info.symbol == prefix)
        .map(|prefix_info| prefix_info.scale_factor)
        .unwrap_or(0)
}

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
            base_unit_info.inherent_scale_factor,
        );
    }

    // Try compound units from default-dimensions
    if let Some(compound_unit_info) = COMPOUND_UNITS.iter().find(|info| info.symbol == base_unit) {
        let (m, l, t, c, temp, a, lum, ang) = compound_unit_info.dimension_exponents;
        return (m, l, t, c, temp, a, lum, ang, 0); // p10_offset is 0 for SI derived units
    }

    // If not found, try to find it in the shared dimension data by SI symbol
    if let Some(dim_info) = DIMENSION_LOOKUP.iter().find(|info| {
        info.si_symbol
            .map(|symbol| symbol == base_unit)
            .unwrap_or(false)
    }) {
        // Convert the dimension exponents to the format expected by the unit macro
        // The shared data has (mass, length, time, current, temperature, amount, luminosity, angle)
        // The unit macro expects (mass, length, time, current, temperature, amount, luminosity, angle, p10_offset)
        let (m, l, t, c, temp, a, lum, ang) = dim_info.exponents;
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
    // First check if this is a unit literal (like min, h, hr, d)
    if let Some(unit_literal_info) = UNIT_LITERALS.iter().find(|info| info.symbol == unit_name) {
        let (mass, length, time, current, temp, amount, lum, angle) =
            unit_literal_info.dimension_exponents;
        let (p2, p3, p5, pi) = unit_literal_info.scale_factors;
        return (
            mass, length, time, current, temp, amount, lum, angle, p2, p3, p5, pi,
        );
    }

    let (prefix, base_unit) = parse_unit_name(unit_name);

    // Get base unit dimensions and inherent scale
    let (mass, length, time, current, temp, amount, lum, angle, inherent_p10) =
        get_base_unit_dimensions(base_unit);

    // Get prefix power of 10
    let prefix_p10 = prefix.map(get_prefix_power).unwrap_or(0);

    // Calculate final scale: add prefix power to inherent scale
    // The inherent scale accounts for the base unit's offset (e.g., gram has -3 offset)
    let final_scale = inherent_p10 + prefix_p10;

    // Get special time scale factors
    let (p2, p3, p5) = get_time_scale_factors(base_unit);

    // All units now use p2 and p5 positions to represent powers of 10 as 2^n × 5^n
    // This ensures a strict bijection between type identities and mathematical meaning
    let (final_p2, final_p5) = if time > 0 || angle > 0 {
        // Time or angle units: combine existing factors with power of 10
        (p2 + final_scale, p5 + final_scale)
    } else {
        // Other units: put power of 10 in both p2 and p5 positions
        (final_scale, final_scale)
    };

    (
        mass, length, time, current, temp, amount, lum, angle, final_p2, p3, final_p5, 0,
    )
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

        // Use the specified storage type or default to f64
        let storage_type = self
            .storage_type
            .unwrap_or_else(|| syn::parse_str::<Type>("f64").unwrap());

        quote! {
            whippyunits::quantity_type::Quantity<
                #mass_exp, #length_exp, #time_exp, #current_exp, #temp_exp, #amount_exp, #lum_exp, #angle_exp,
                #p2, #p3, #p5, #pi,
                #storage_type
            >
        }
    }
}
