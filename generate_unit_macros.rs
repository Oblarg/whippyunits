use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct UnitParams {
    length_exp: i32,
    length_scale: String,
    mass_exp: i32,
    mass_scale: String,
    time_exp: i32,
    time_p2: String,
    time_p3: String,
    time_p5: String,
    time_scale_order: String,
}

impl UnitParams {
    fn new(
        length_exp: i32,
        length_scale: i32,
        mass_exp: i32,
        mass_scale: i32,
        time_exp: i32,
        time_p2: i32,
        time_p3: i32,
        time_p5: i32,
        time_scale_order: i32,
    ) -> Self {
        Self {
            length_exp,
            length_scale: if length_scale == i32::MAX { "{isize::MAX}".to_string() } else { length_scale.to_string() },
            mass_exp,
            mass_scale: if mass_scale == i32::MAX { "{isize::MAX}".to_string() } else { mass_scale.to_string() },
            time_exp,
            time_p2: if time_p2 == i32::MAX { "{isize::MAX}".to_string() } else { time_p2.to_string() },
            time_p3: if time_p3 == i32::MAX { "{isize::MAX}".to_string() } else { time_p3.to_string() },
            time_p5: if time_p5 == i32::MAX { "{isize::MAX}".to_string() } else { time_p5.to_string() },
            time_scale_order: if time_scale_order == i32::MAX { "{isize::MAX}".to_string() } else { time_scale_order.to_string() },
        }
    }

    fn generate_quantity_type(&self) -> String {
        format!(
            "crate::Quantity<\n                {}, {},\n                {}, {},\n                {}, {}, {}, {}, {}\n            >",
            self.length_exp, self.length_scale,
            self.mass_exp, self.mass_scale,
            self.time_exp, self.time_p2, self.time_p3, self.time_p5, self.time_scale_order
        )
    }
}

fn combine_units(unit1: &UnitParams, unit2: &UnitParams, operation: &str) -> UnitParams {
    match operation {
        "*" => UnitParams {
            length_exp: unit1.length_exp + unit2.length_exp,
            length_scale: if unit1.length_exp > 0 { unit1.length_scale.clone() } else { unit2.length_scale.clone() },
            mass_exp: unit1.mass_exp + unit2.mass_exp,
            mass_scale: if unit1.mass_exp > 0 { unit1.mass_scale.clone() } else { unit2.mass_scale.clone() },
            time_exp: unit1.time_exp + unit2.time_exp,
            time_p2: if unit1.time_exp > 0 { unit1.time_p2.clone() } else { unit2.time_p2.clone() },
            time_p3: if unit1.time_exp > 0 { unit1.time_p3.clone() } else { unit2.time_p3.clone() },
            time_p5: if unit1.time_exp > 0 { unit1.time_p5.clone() } else { unit2.time_p5.clone() },
            time_scale_order: if unit1.time_exp > 0 { unit1.time_scale_order.clone() } else { unit2.time_scale_order.clone() },
        },
        "/" => UnitParams {
            length_exp: unit1.length_exp - unit2.length_exp,
            length_scale: if unit1.length_exp > 0 { unit1.length_scale.clone() } else { unit2.length_scale.clone() },
            mass_exp: unit1.mass_exp - unit2.mass_exp,
            mass_scale: if unit1.mass_exp > 0 { unit1.mass_scale.clone() } else { unit2.mass_scale.clone() },
            time_exp: unit1.time_exp - unit2.time_exp,
            time_p2: if unit1.time_exp > 0 { unit1.time_p2.clone() } else { unit2.time_p2.clone() },
            time_p3: if unit1.time_exp > 0 { unit1.time_p3.clone() } else { unit2.time_p3.clone() },
            time_p5: if unit1.time_exp > 0 { unit1.time_p5.clone() } else { unit2.time_p5.clone() },
            time_scale_order: if unit1.time_exp > 0 { unit1.time_scale_order.clone() } else { unit2.time_scale_order.clone() },
        },
        _ => unit1.clone(),
    }
}

fn generate_macro_patterns() -> Vec<String> {
    let mut patterns = Vec::new();
    
    // Unit definitions
    let mut units: HashMap<&str, UnitParams> = HashMap::new();
    units.insert("mm", UnitParams::new(1, -1, 0, i32::MAX, 0, i32::MAX, i32::MAX, i32::MAX, i32::MAX));
    units.insert("m", UnitParams::new(1, 0, 0, i32::MAX, 0, i32::MAX, i32::MAX, i32::MAX, i32::MAX));
    units.insert("km", UnitParams::new(1, 1, 0, i32::MAX, 0, i32::MAX, i32::MAX, i32::MAX, i32::MAX));
    
    units.insert("mg", UnitParams::new(0, i32::MAX, 1, -1, 0, i32::MAX, i32::MAX, i32::MAX, i32::MAX));
    units.insert("g", UnitParams::new(0, i32::MAX, 1, 0, 0, i32::MAX, i32::MAX, i32::MAX, i32::MAX));
    units.insert("kg", UnitParams::new(0, i32::MAX, 1, 1, 0, i32::MAX, i32::MAX, i32::MAX, i32::MAX));
    
    units.insert("ms", UnitParams::new(0, i32::MAX, 0, i32::MAX, 1, -3, 0, -3, -1));
    units.insert("s", UnitParams::new(0, i32::MAX, 0, i32::MAX, 1, 0, 0, 0, 0));
    units.insert("min", UnitParams::new(0, i32::MAX, 0, i32::MAX, 1, 2, 1, 1, 1));

    // ============================================================================
    // SINGLE UNITS (Base units)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // SINGLE UNITS (Base units)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    // Length units
    patterns.push("    // Length units".to_string());
    for unit in &["mm", "m", "km"] {
        let params = &units[unit];
        patterns.push(format!("    ({}) => {{ {} }};", unit, params.generate_quantity_type()));
    }
    
    // Mass units  
    patterns.push("    // Mass units".to_string());
    for unit in &["mg", "g", "kg"] {
        let params = &units[unit];
        patterns.push(format!("    ({}) => {{ {} }};", unit, params.generate_quantity_type()));
    }
    
    // Time units
    patterns.push("    // Time units".to_string());
    for unit in &["ms", "s", "min"] {
        let params = &units[unit];
        patterns.push(format!("    ({}) => {{ {} }};", unit, params.generate_quantity_type()));
    }
    
    patterns.push("".to_string());
    
    // ============================================================================
    // UNITS WITH EXPONENTS (Squared, cubed, etc.)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // UNITS WITH EXPONENTS (Squared, cubed, etc.)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    // Length squared
    patterns.push("    // Length squared".to_string());
    for unit in &["mm", "m", "km"] {
        let mut params = units[unit].clone();
        params.length_exp *= 2;
        patterns.push(format!("    ({}^2) => {{ {} }};", unit, params.generate_quantity_type()));
    }
    
    // Length cubed
    patterns.push("    // Length cubed".to_string());
    for unit in &["mm", "m", "km"] {
        let mut params = units[unit].clone();
        params.length_exp *= 3;
        patterns.push(format!("    ({}^3) => {{ {} }};", unit, params.generate_quantity_type()));
    }
    
    // Mass squared
    patterns.push("    // Mass squared".to_string());
    for unit in &["mg", "g", "kg"] {
        let mut params = units[unit].clone();
        params.mass_exp *= 2;
        patterns.push(format!("    ({}^2) => {{ {} }};", unit, params.generate_quantity_type()));
    }
    
    // Time squared
    patterns.push("    // Time squared".to_string());
    for unit in &["ms", "s", "min"] {
        let mut params = units[unit].clone();
        params.time_exp *= 2;
        patterns.push(format!("    ({}^2) => {{ {} }};", unit, params.generate_quantity_type()));
    }
    
    patterns.push("".to_string());
    
    // ============================================================================
    // COMPOUND UNITS (Two units multiplied)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // COMPOUND UNITS (Two units multiplied)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    // Length × Length
    patterns.push("    // Length × Length".to_string());
    for unit1 in &["mm", "m", "km"] {
        for unit2 in &["mm", "m", "km"] {
            if unit1 <= unit2 {  // Avoid duplicates
                let params = combine_units(&units[unit1], &units[unit2], "*");
                patterns.push(format!("    ({} * {}) => {{ {} }};", unit1, unit2, params.generate_quantity_type()));
            }
        }
    }
    
    // Length × Mass
    patterns.push("    // Length × Mass".to_string());
    for length_unit in &["mm", "m", "km"] {
        for mass_unit in &["mg", "g", "kg"] {
            let params = combine_units(&units[length_unit], &units[mass_unit], "*");
            patterns.push(format!("    ({} * {}) => {{ {} }};", length_unit, mass_unit, params.generate_quantity_type()));
        }
    }
    
    // Length × Time
    patterns.push("    // Length × Time".to_string());
    for length_unit in &["mm", "m", "km"] {
        for time_unit in &["ms", "s", "min"] {
            let params = combine_units(&units[length_unit], &units[time_unit], "*");
            patterns.push(format!("    ({} * {}) => {{ {} }};", length_unit, time_unit, params.generate_quantity_type()));
        }
    }
    
    // Mass × Time
    patterns.push("    // Mass × Time".to_string());
    for mass_unit in &["mg", "g", "kg"] {
        for time_unit in &["ms", "s", "min"] {
            let params = combine_units(&units[mass_unit], &units[time_unit], "*");
            patterns.push(format!("    ({} * {}) => {{ {} }};", mass_unit, time_unit, params.generate_quantity_type()));
        }
    }
    
    patterns.push("".to_string());
    
    // ============================================================================
    // VELOCITY UNITS (Length / Time)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // VELOCITY UNITS (Length / Time)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    for length_unit in &["mm", "m", "km"] {
        for time_unit in &["ms", "s", "min"] {
            let params = combine_units(&units[length_unit], &units[time_unit], "/");
            patterns.push(format!("    ({} * {}^-1) => {{ {} }};", length_unit, time_unit, params.generate_quantity_type()));
            patterns.push(format!("    ({}/{}) => {{ {} }};", length_unit, time_unit, params.generate_quantity_type()));
        }
    }
    
    patterns.push("".to_string());
    
    // ============================================================================
    // ACCELERATION UNITS (Length / Time²)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // ACCELERATION UNITS (Length / Time²)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    for length_unit in &["mm", "m", "km"] {
        for time_unit in &["ms", "s", "min"] {
            // Create time^-2 unit
            let mut time_params = units[time_unit].clone();
            time_params.time_exp = -2;
            let params = combine_units(&units[length_unit], &time_params, "*");
            patterns.push(format!("    ({} * {}^-2) => {{ {} }};", length_unit, time_unit, params.generate_quantity_type()));
            patterns.push(format!("    ({}/{}^2) => {{ {} }};", length_unit, time_unit, params.generate_quantity_type()));
        }
    }
    
    patterns.push("".to_string());
    
    // ============================================================================
    // FORCE UNITS (Mass × Length / Time²)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // FORCE UNITS (Mass × Length / Time²)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    for mass_unit in &["mg", "g", "kg"] {
        for length_unit in &["mm", "m", "km"] {
            for time_unit in &["ms", "s", "min"] {
                // Create time^-2 unit
                let mut time_params = units[time_unit].clone();
                time_params.time_exp = -2;
                // Combine: mass * length * time^-2
                let temp_params = combine_units(&units[mass_unit], &units[length_unit], "*");
                let final_params = combine_units(&temp_params, &time_params, "*");
                patterns.push(format!("    ({} * {} * {}^-2) => {{ {} }};", mass_unit, length_unit, time_unit, final_params.generate_quantity_type()));
            }
        }
    }
    
    patterns.push("".to_string());
    
    // ============================================================================
    // ENERGY UNITS (Mass × Length² / Time²)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // ENERGY UNITS (Mass × Length² / Time²)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    for mass_unit in &["mg", "g", "kg"] {
        for length_unit in &["mm", "m", "km"] {
            for time_unit in &["ms", "s", "min"] {
                // Create length^2 unit
                let mut length_params = units[length_unit].clone();
                length_params.length_exp = 2;
                // Create time^-2 unit
                let mut time_params = units[time_unit].clone();
                time_params.time_exp = -2;
                // Combine: mass * length^2 * time^-2
                let temp_params = combine_units(&units[mass_unit], &length_params, "*");
                let final_params = combine_units(&temp_params, &time_params, "*");
                patterns.push(format!("    ({} * {}^2 * {}^-2) => {{ {} }};", mass_unit, length_unit, time_unit, final_params.generate_quantity_type()));
            }
        }
    }
    
    patterns.push("".to_string());
    
    // ============================================================================
    // POWER UNITS (Mass × Length² / Time³)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // POWER UNITS (Mass × Length² / Time³)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    for mass_unit in &["mg", "g", "kg"] {
        for length_unit in &["mm", "m", "km"] {
            for time_unit in &["ms", "s", "min"] {
                // Create length^2 unit
                let mut length_params = units[length_unit].clone();
                length_params.length_exp = 2;
                // Create time^-3 unit
                let mut time_params = units[time_unit].clone();
                time_params.time_exp = -3;
                // Combine: mass * length^2 * time^-3
                let temp_params = combine_units(&units[mass_unit], &length_params, "*");
                let final_params = combine_units(&temp_params, &time_params, "*");
                patterns.push(format!("    ({} * {}^2 * {}^-3) => {{ {} }};", mass_unit, length_unit, time_unit, final_params.generate_quantity_type()));
            }
        }
    }
    
    patterns.push("".to_string());
    
    patterns
}

fn generate_macro_file() -> String {
    let header = r#"// Auto-generated unit macro with human-readable triangular structure
// Generated by generate_unit_macros.rs
// 
// This macro provides LSP-friendly declarative patterns for common unit expressions.
// For complex expressions not covered here, use proc_unit!() instead.

#[macro_export]
macro_rules! unit {
"#;
    
    let patterns = generate_macro_patterns();
    
    let footer = r#"    // ============================================================================
    // CATCH-ALL FOR UNKNOWN UNITS
    // ============================================================================
    ($unknown:tt) => {
        compile_error!(concat!("Unknown unit: ", stringify!($unknown), ". Use proc_unit!() for complex expressions."))
    };
}
"#;
    
    header.to_string() + &patterns.join("\n") + "\n" + footer
}

fn main() {
    let output = generate_macro_file();
    
    // Count patterns before writing
    let pattern_count = output.lines().filter(|line| line.contains("=>")).count();
    
    // Write to file
    fs::write("src/generated_unit_macro.rs", &output).expect("Failed to write file");
    
    println!("✅ Generated src/generated_unit_macro.rs");
    println!("📊 Generated {} patterns", pattern_count);
    println!();
    println!("🔍 Structure overview:");
    println!("   • Single units (base units)");
    println!("   • Units with exponents (squared, cubed)");
    println!("   • Compound units (two units multiplied)");
    println!("   • Velocity units (length / time)");
    println!("   • Acceleration units (length / time²)");
    println!("   • Force units (mass × length / time²)");
    println!("   • Energy units (mass × length² / time²)");
    println!("   • Power units (mass × length² / time³)");
    println!();
    println!("🚀 Ready to use! The macro is now LSP-friendly and human-readable.");
}
