use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitInt};
use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Star, Slash, Caret};
use whippyunits_default_dimensions::DIMENSION_LOOKUP;

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
            Ok(UnitExpr::Unit(Unit { name: syn::Ident::new("dimensionless", proc_macro2::Span::call_site()), exponent: 1 }))
        } else {
            let ident: Ident = input.parse()?;
            Ok(UnitExpr::Unit(Unit { name: ident, exponent: 1 }))
        }
    }
    
    /// Evaluate the unit expression to get dimension exponents and scale factors
    fn evaluate(&self) -> (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) {
        match self {
            UnitExpr::Unit(unit) => {
                let (mass, length, time, current, temp, amount, lum, angle, p2, p3, p5, p10, pi) = 
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
                    p10 * unit.exponent,
                    pi * unit.exponent,
                )
            },
            UnitExpr::Mul(a, b) => {
                let (ma, la, ta, ca, tempa, aa, luma, anga, p2a, p3a, p5a, p10a, pia) = a.evaluate();
                let (mb, lb, tb, cb, tempb, ab, lumb, angb, p2b, p3b, p5b, p10b, pib) = b.evaluate();
                (
                    ma + mb, la + lb, ta + tb, ca + cb, tempa + tempb, aa + ab, luma + lumb, anga + angb,
                    p2a + p2b, p3a + p3b, p5a + p5b, p10a + p10b, pia + pib
                )
            },
            UnitExpr::Div(a, b) => {
                let (ma, la, ta, ca, tempa, aa, luma, anga, p2a, p3a, p5a, p10a, pia) = a.evaluate();
                let (mb, lb, tb, cb, tempb, ab, lumb, angb, p2b, p3b, p5b, p10b, pib) = b.evaluate();
                (
                    ma - mb, la - lb, ta - tb, ca - cb, tempa - tempb, aa - ab, luma - lumb, anga - angb,
                    p2a - p2b, p3a - p3b, p5a - p5b, p10a - p10b, pia - pib
                )
            },
            UnitExpr::Pow(base, exp) => {
                let (m, l, t, c, temp, a, lum, ang, p2, p3, p5, p10, pi) = base.evaluate();
                let exp_val: i16 = exp.base10_parse().unwrap();
                (
                    m * exp_val, l * exp_val, t * exp_val, c * exp_val, temp * exp_val, a * exp_val, 
                    lum * exp_val, ang * exp_val, p2 * exp_val, p3 * exp_val, p5 * exp_val, p10 * exp_val, pi * exp_val
                )
            }
        }
    }
}

/// SI prefix definitions
/// Each prefix maps to its actual scale factor used in the generated unit methods
/// These values match exactly what the actual unit methods return
const SI_PREFIXES: &[(&str, i16)] = &[
    // Small prefixes (negative powers of 10)
    ("p", -12),  // pico
    ("n", -9),  // nano
    ("u", -6),   // micro 
    ("m", -3),   // milli
    
    // Large prefixes (positive powers of 10)
    ("k", 3),    // kilo
    ("M", 6),    // mega
    ("G", 9),    // giga
    ("T", 12),   // tera
    ("P", 15),   // peta
    ("E", 18),   // exa
    ("Z", 21),   // zetta
    ("Y", 24),   // yotta
];

/// Base unit definitions
/// Each base unit maps to its dimension exponents (mass, length, time, current, temperature, amount, luminosity, angle)
/// and its inherent scale factor (p10_offset)
const BASE_UNITS: &[(&str, (i16, i16, i16, i16, i16, i16, i16, i16, i16))] = &[
    // Mass - all mass units are relative to kilogram (scale factor 0)
    ("g", (1, 0, 0, 0, 0, 0, 0, 0, -3)),     // gram: scale -3 (1g = 10^-3 kg)
    
    // Length - all length units are relative to meter (scale factor 0)
    ("m", (0, 1, 0, 0, 0, 0, 0, 0, 0)),      // meter: no inherent scale
    
    // Time - all time units are relative to second (scale factor 0)
    ("s", (0, 0, 1, 0, 0, 0, 0, 0, 0)),      // second: no inherent scale
    ("min", (0, 0, 1, 0, 0, 0, 0, 0, 0)),    // minute: 60s = 2^2 * 3 * 5, so p2=2, p3=1, p5=1
    ("h", (0, 0, 1, 0, 0, 0, 0, 0, 0)),      // hour: 3600s = 2^4 * 3^2 * 5^2, so p2=4, p3=2, p5=2
    ("d", (0, 0, 1, 0, 0, 0, 0, 0, 0)),      // day: 86400s = 2^7 * 3^3 * 5^2, so p2=7, p3=3, p5=2
    ("yr", (0, 0, 1, 0, 0, 0, 0, 0, 0)),     // year (approximate: 365.25 days)
    
    // Current - all current units are relative to ampere (scale factor 0)
    ("A", (0, 0, 0, 1, 0, 0, 0, 0, 0)),      // ampere: no inherent scale
    
    // Temperature - all temperature units are relative to kelvin (scale factor 0)
    ("K", (0, 0, 0, 0, 1, 0, 0, 0, 0)),      // kelvin: no inherent scale
    
    // Amount - all amount units are relative to mole (scale factor 0)
    ("mol", (0, 0, 0, 0, 0, 1, 0, 0, 0)),    // mole: no inherent scale
    
    // Luminosity - all luminosity units are relative to candela (scale factor 0)
    ("cd", (0, 0, 0, 0, 0, 0, 1, 0, 0)),     // candela: no inherent scale
    
    // Angle - all angle units are relative to radian (scale factor 0)
    ("rad", (0, 0, 0, 0, 0, 0, 0, 1, 0)),    // radian: no inherent scale
    ("deg", (0, 0, 0, 0, 0, 0, 0, 1, 0)),    // degrees (dimensionless but treated as angle)
    
    // Special cases
    ("dimensionless", (0, 0, 0, 0, 0, 0, 0, 0, 0)),
];

/// Parse a unit name to extract prefix and base unit
fn parse_unit_name(unit_name: &str) -> (Option<&str>, &str) {
    // Try to find the longest matching prefix
    for (prefix, _) in SI_PREFIXES.iter().rev() {
        if unit_name.starts_with(prefix) {
            let base = &unit_name[prefix.len()..];
            if !base.is_empty() && is_valid_base_unit(base) {
                return (Some(prefix), base);
            }
        }
    }
    
    // No prefix found, treat as base unit
    (None, unit_name)
}

/// Check if a string is a valid base unit
fn is_valid_base_unit(unit: &str) -> bool {
    BASE_UNITS.iter().any(|(base_unit, _)| *base_unit == unit)
}

/// Get the power of 10 for a prefix
fn get_prefix_power(prefix: &str) -> i16 {
    SI_PREFIXES.iter()
        .find(|(p, _)| *p == prefix)
        .map(|(_, power)| *power)
        .unwrap_or(0)
}

/// Get dimension exponents and inherent scale for a base unit
fn get_base_unit_dimensions(base_unit: &str) -> (i16, i16, i16, i16, i16, i16, i16, i16, i16) {
    // First try the hardcoded base units
    if let Some((_, dims)) = BASE_UNITS.iter().find(|(unit, _)| *unit == base_unit) {
        return *dims;
    }
    
    // If not found, try to find it in the shared dimension data by SI symbol
    if let Some(dim_info) = DIMENSION_LOOKUP.iter().find(|info| {
        info.si_symbol.map(|symbol| symbol == base_unit).unwrap_or(false)
    }) {
        // Convert the dimension exponents to the format expected by the unit macro
        // The shared data has (mass, length, time, current, temperature, amount, luminosity, angle)
        // The unit macro expects (mass, length, time, current, temperature, amount, luminosity, angle, p10_offset)
        let (m, l, t, c, temp, a, lum, ang) = dim_info.exponents;
        return (m, l, t, c, temp, a, lum, ang, 0); // p10_offset is 0 for SI derived units
    }
    
    panic!("Unknown base unit: {}", base_unit)
}

/// Get special scale factors for time units that aren't powers of 10
fn get_time_scale_factors(base_unit: &str) -> (i16, i16, i16) {
    match base_unit {
        "s" => (0, 0, 0),           // seconds: no special factors
        "min" => (2, 1, 1),         // 60s = 2^2 * 3 * 5
        "h" => (4, 2, 2),           // 3600s = 2^4 * 3^2 * 5^2
        "d" => (7, 3, 2),           // 86400s = 2^7 * 3^3 * 5^2
        "yr" => (7, 3, 2),          // year (approximate, same as day for simplicity)
        _ => (0, 0, 0),
    }
}

/// Get dimension exponents and scale factors for a unit
/// Returns (mass, length, time, current, temperature, amount, luminosity, angle, p2, p3, p5, p10, pi)
fn get_unit_dimensions(unit_name: &str) -> (i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) {
    let (prefix, base_unit) = parse_unit_name(unit_name);
    
    // Get base unit dimensions and inherent scale
    let (mass, length, time, current, temp, amount, lum, angle, inherent_p10) = get_base_unit_dimensions(base_unit);
    
    // Get prefix power of 10
    let prefix_p10 = prefix.map(get_prefix_power).unwrap_or(0);
    
    // Calculate final scale: add prefix power to inherent scale
    // The inherent scale accounts for the base unit's offset (e.g., gram has -3 offset)
    let final_scale = inherent_p10 + prefix_p10;
    
    // Get special time scale factors
    let (p2, p3, p5) = get_time_scale_factors(base_unit);
    
    // Determine where to put the scale factor based on the unit type
    // Time and Angle units use p2 and p5 positions (multiply to give power of 10), others use p10 position
    let (final_p2, final_p5, final_p10) = if time > 0 || angle > 0 {
        // Time or angle units: scale factor goes in both p2 and p5 positions
        (final_scale, final_scale, 0)
    } else {
        // Other units: scale factor goes in p10 position
        (p2, p5, final_scale)
    };
    
    (mass, length, time, current, temp, amount, lum, angle, final_p2, p3, final_p5, final_p10, 0)
}

/// Input for the unit macro
pub struct UnitMacroInput {
    pub unit_expr: UnitExpr,
}

impl Parse for UnitMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(UnitMacroInput {
            unit_expr: input.parse()?,
        })
    }
}

impl UnitMacroInput {
    pub fn expand(self) -> TokenStream {
        let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp, 
             p2, p3, p5, p10, pi) = self.unit_expr.evaluate();
        
        quote! {
            whippyunits::quantity_type::Quantity<
                #mass_exp, #length_exp, #time_exp, #current_exp, #temp_exp, #amount_exp, #lum_exp, #angle_exp,
                #p2, #p3, #p5, #p10, #pi,
                f64
            >
        }
    }
}
