use crate::print::utils::get_si_prefix;
use std::collections::HashMap;

/// Represents the scale factors for a unit dimension
#[derive(Debug, Clone)]
pub struct UnitScale {
    /// Scale factors keyed by prime number (e.g., 2, 3, 5, 10)
    pub prime_scales: HashMap<i8, i8>,
    /// Scale factors for mathematical constants (e.g., π)
    pub constant_scales: HashMap<&'static str, i8>,
}

impl UnitScale {
    pub fn new() -> Self {
        Self {
            prime_scales: HashMap::new(),
            constant_scales: HashMap::new(),
        }
    }
    
    pub fn with_prime_scale(mut self, prime: i8, scale: i8) -> Self {
        self.prime_scales.insert(prime, scale);
        self
    }
    
    pub fn with_constant_scale(mut self, constant: &'static str, scale: i8) -> Self {
        self.constant_scales.insert(constant, scale);
        self
    }
}

/// Configuration for a unit dimension
#[derive(Debug, Clone)]
pub struct UnitConfig {
    pub exponent: i8,
    pub scale: UnitScale,
    pub base_name: &'static str,
    pub base_symbol: &'static str,
    pub base_scale_offset: i8,
    pub unknown_text: Option<&'static str>,
}

impl UnitConfig {
    pub fn simple(
        exponent: i8,
        prime_scale: i8,
        base_name: &'static str,
        base_symbol: &'static str,
        base_scale_offset: i8,
    ) -> Self {
        Self {
            exponent,
            scale: UnitScale::new().with_prime_scale(10i8, prime_scale),
            base_name,
            base_symbol,
            base_scale_offset,
            unknown_text: None,
        }
    }
    
    pub fn composite(
        exponent: i8,
        prime_scales: Vec<(i8, i8)>, // (prime, scale) pairs
        constant_scales: Vec<(&'static str, i8)>, // (constant, scale) pairs
        base_name: &'static str,
        base_symbol: &'static str,
        base_scale_offset: i8,
        unknown_text: &'static str,
    ) -> Self {
        let mut scale = UnitScale::new();
        for (prime, scale_val) in prime_scales {
            scale.prime_scales.insert(prime, scale_val);
        }
        for (constant, scale_val) in constant_scales {
            scale.constant_scales.insert(constant, scale_val);
        }
        
        Self {
            exponent,
            scale,
            base_name,
            base_symbol,
            base_scale_offset,
            unknown_text: Some(unknown_text),
        }
    }
}

pub fn generate_systematic_unit_name(
    mass_exponent: i8, mass_scale_p10: i8,
    length_exponent: i8, length_scale_p10: i8,
    time_exponent: i8, time_scale_p2: i8, time_scale_p3: i8, time_scale_p5: i8,
    electric_current_exponent: i8, electric_current_scale_p10: i8,
    temperature_exponent: i8, temperature_scale_p10: i8,
    amount_of_substance_exponent: i8, amount_of_substance_scale_p10: i8,
    luminous_intensity_exponent: i8, luminous_intensity_scale_p10: i8,
    angle_exponent: i8, angle_scale_p2: i8, angle_scale_p3: i8, angle_scale_p5: i8, angle_scale_pi: i8,
    long_name: bool,
) -> String {
    // Helper function to get unicode exponent
    fn get_unicode_exponent(exp: i8) -> String {
        crate::print::utils::to_unicode_superscript(exp, false)
    }
    
    // Helper function to render a unit with scale information
    fn render_unit_with_scale(
        unit_string: &mut String,
        scale: &UnitScale,
        base_unit: &str,
        base_scale_offset: i8,
        exponent_str: &str,
        long_name: bool,
    ) {
        // Check if any scale is unknown (i8::MIN)
        let has_unknown_scale = scale.prime_scales.values().any(|&s| s == i8::MIN) ||
                               scale.constant_scales.values().any(|&s| s == i8::MIN);
        
        if has_unknown_scale {
            unit_string.push_str("?");
            return;
        }
        
        // For simple units with only a 10-scale, use SI prefix logic
        if scale.prime_scales.len() == 1 && scale.prime_scales.contains_key(&10) && scale.constant_scales.is_empty() {
            let scale_p10 = scale.prime_scales[&10];
            
            // For mass units, adjust scale by +3 since base unit is "gram" but scales are relative to "kilogram"
            let adjusted_scale = if base_unit == "gram" || base_unit == "g" { 
                scale_p10 + base_scale_offset 
            } else { 
                scale_p10 + base_scale_offset 
            };
            
            let unit_part = if let Some(prefix) = get_si_prefix(adjusted_scale, long_name) {
                format!("{}{}", prefix, base_unit)
            } else if adjusted_scale != 0 {
                format!("(10{} {})", get_unicode_exponent(adjusted_scale), base_unit)
            } else {
                base_unit.to_string()
            };
            
            unit_string.push_str(&format!("{}{}", unit_part, exponent_str));
        } else {
            // For composite units, we need more complex logic
            // For now, use the unknown text if available, otherwise show the base unit
            unit_string.push_str(&format!("{}{}", base_unit, exponent_str));
        }
    }

    // Check if all exponents are unknown
    if mass_exponent == i8::MIN && length_exponent == i8::MIN && time_exponent == i8::MIN {
        return "?".to_string();
    }
    
    // Create unit configurations using the new generic approach
    let unit_configs = vec![
        UnitConfig::simple(mass_exponent, mass_scale_p10, "gram", "g", 3),
        UnitConfig::simple(length_exponent, length_scale_p10, "meter", "m", 0),
        UnitConfig::composite(
            time_exponent,
            vec![(2, time_scale_p2), (3, time_scale_p3), (5, time_scale_p5)],
            vec![],
            "second", "s", 0,
            if long_name { "unknown time unit" } else { "t?" }
        ),
        UnitConfig::simple(electric_current_exponent, electric_current_scale_p10, "ampere", "A", 0),
        UnitConfig::simple(temperature_exponent, temperature_scale_p10, "kelvin", "K", 0),
        UnitConfig::simple(amount_of_substance_exponent, amount_of_substance_scale_p10, "mole", "mol", 0),
        UnitConfig::simple(luminous_intensity_exponent, luminous_intensity_scale_p10, "candela", "cd", 0),
        UnitConfig::composite(
            angle_exponent,
            vec![(2, angle_scale_p2), (3, angle_scale_p3), (5, angle_scale_p5)],
            vec![("pi", angle_scale_pi)],
            "radian", "rad", 0,
            if long_name { "unknown angle unit" } else { "a?" }
        ),
    ];
    
    let unit_parts: Vec<String> = unit_configs
        .iter()
        .filter(|config| config.exponent != 0)
        .map(|config| {
            let base_unit = if long_name { config.base_name } else { config.base_symbol };
            let mut part = String::new();
            
            // For composite units, check if we can represent them systematically
            if config.unknown_text.is_some() {
                // This is a composite unit - check if it matches a known pattern
                let scale_2 = config.scale.prime_scales.get(&2).copied().unwrap_or(0);
                let scale_3 = config.scale.prime_scales.get(&3).copied().unwrap_or(0);
                let scale_5 = config.scale.prime_scales.get(&5).copied().unwrap_or(0);
                let scale_pi = config.scale.constant_scales.get("pi").copied().unwrap_or(0);
                
                // Check for known composite time units
                let known_time_unit = if base_unit == "second" || base_unit == "s" {
                    match (scale_2, scale_3, scale_5) {
                        (0, 0, 0) => Some("second"), // Pure second
                        (2, 1, 1) => Some("minute"), // 1 minute = 60 seconds = 2^2 * 3^1 * 5^1
                        (4, 2, 2) => Some("hour"),   // 1 hour = 3600 seconds = 2^4 * 3^2 * 5^2
                        (7, 3, 2) => Some("day"),    // 1 day = 86400 seconds = 2^7 * 3^3 * 5^2
                        _ => None,
                    }
                } else {
                    None
                };
                
                // Check for known composite angle units
                let known_angle_unit = if base_unit == "radian" || base_unit == "rad" {
                    match (scale_2, scale_3, scale_5, scale_pi) {
                        (0, 0, 0, 0) => Some("radian"),     // Pure radian
                        (1, 0, 0, 1) => Some("rotation"),   // 1 rotation = 2π radians
                        (-2, -2, -1, 1) => Some("degree"),  // 1 degree = π/180 radians
                        (-3, 0, -2, 1) => Some("gradian"),  // 1 gradian = π/200 radians
                        (-4, -3, -2, 1) => Some("arcminute"), // 1 arcminute = π/10800 radians
                        (-5, -4, -2, 1) => Some("arcsecond"), // 1 arcsecond = π/648000 radians
                        _ => None,
                    }
                } else {
                    None
                };
                
                // Check for pure power of 10 scaling (for time and angle units)
                let is_pure_power_of_10 = if base_unit == "second" || base_unit == "s" {
                    // Time unit: check if 2 and 5 scales match and 3 scale is 0
                    scale_2 == scale_5 && scale_3 == 0
                } else if base_unit == "radian" || base_unit == "rad" {
                    // Angle unit: check if 2 and 5 scales match and 3 and pi scales are 0
                    scale_2 == scale_5 && scale_3 == 0 && scale_pi == 0
                } else {
                    false
                };
                
                if let Some(unit_name) = known_time_unit {
                    // Use the known composite time unit name
                    let unit_symbol = if long_name { unit_name } else { 
                        match unit_name {
                            "second" => "s",
                            "minute" => "min", 
                            "hour" => "h",
                            "day" => "d",
                            _ => unit_name,
                        }
                    };
                    part.push_str(&format!("{}{}", unit_symbol, get_unicode_exponent(config.exponent)));
                } else if let Some(unit_name) = known_angle_unit {
                    // Use the known composite angle unit name
                    let unit_symbol = if long_name { unit_name } else { 
                        match unit_name {
                            "radian" => "rad",
                            "rotation" => "rev", 
                            "degree" => "°",
                            "gradian" => "gon",
                            "arcminute" => "'",
                            "arcsecond" => "\"",
                            _ => unit_name,
                        }
                    };
                    part.push_str(&format!("{}{}", unit_symbol, get_unicode_exponent(config.exponent)));
                } else if is_pure_power_of_10 {
                    // Treat as simple unit with power-of-10 scaling
                    render_unit_with_scale(&mut part, &config.scale, base_unit, config.base_scale_offset, &get_unicode_exponent(config.exponent), long_name);
                } else {
                    // Use unknown text for complex composite units
                    part.push_str(config.unknown_text.unwrap());
                }
            } else {
                // Simple unit
                render_unit_with_scale(&mut part, &config.scale, base_unit, config.base_scale_offset, &get_unicode_exponent(config.exponent), long_name);
            }
            
            part
        })
        .collect();
    
    unit_parts.join("·")
}

pub struct DimensionNames {
    pub dimension_name: &'static str,
    // only for dimensions with semi-systematic simplified names (e.g. Energy, Joule/J)
    pub unit_si_shortname_symbol: Option<&'static str>,
    pub unit_si_shortname: Option<&'static str>,
}

pub fn lookup_dimension_name(
    mass_exponent: i8,
    length_exponent: i8,
    time_exponent: i8,
    electric_current_exponent: i8,
    temperature_exponent: i8,
    amount_of_substance_exponent: i8,
    luminous_intensity_exponent: i8,
    angle_exponent: i8,
) -> Option<DimensionNames> {
    let dimension_lookup = [
        ((1, 0, 0), ("Mass", None, None)),
        ((0, 1, 0), ("Length", None, None)),
        ((0, 0, 1), ("Time", None, None)),
        ((0, 0, -1), ("Frequency", Some("Hz"), Some("Hertz"))),
        ((0, 2, 0), ("Area", None, None)),
        ((0, -1, 0), ("Inverse Length", None, None)),
        ((1, 2, -2), ("Energy", Some("J"), Some("Joule"))),
    ];
    
    dimension_lookup
        .iter()
        .find(|((m, l, t), _)| {
            *m == mass_exponent && 
            *l == length_exponent && 
            *t == time_exponent &&
            electric_current_exponent == 0 &&
            temperature_exponent == 0 &&
            amount_of_substance_exponent == 0 &&
            luminous_intensity_exponent == 0 &&
            angle_exponent == 0
        })
        .map(|(_, (name, symbol, long_name))| DimensionNames {
            dimension_name: name,
            unit_si_shortname_symbol: symbol.map(|s| s),
            unit_si_shortname: long_name.map(|s| s),
        })
}