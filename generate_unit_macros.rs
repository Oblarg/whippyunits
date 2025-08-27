use std::collections::HashMap;
use std::fs;

// ============================================================================
// Import dimensional metadata
// ============================================================================

mod dimensional_metadata;
use dimensional_metadata::{UnitMetadata, TimeUnitMetadata, LENGTH_UNITS, MASS_UNITS, TIME_UNITS};

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
            length_scale: if length_scale == i32::MAX { "9223372036854775807".to_string() } else { length_scale.to_string() },
            mass_exp,
            mass_scale: if mass_scale == i32::MAX { "9223372036854775807".to_string() } else { mass_scale.to_string() },
            time_exp,
            time_p2: if time_p2 == i32::MAX { "9223372036854775807".to_string() } else { time_p2.to_string() },
            time_p3: if time_p3 == i32::MAX { "9223372036854775807".to_string() } else { time_p3.to_string() },
            time_p5: if time_p5 == i32::MAX { "9223372036854775807".to_string() } else { time_p5.to_string() },
            time_scale_order: if time_scale_order == i32::MAX { "9223372036854775807".to_string() } else { time_scale_order.to_string() },
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

// ============================================================================
// Unit Metadata Access Functions
// ============================================================================

fn get_length_units() -> &'static [UnitMetadata] {
    LENGTH_UNITS
}

fn get_mass_units() -> &'static [UnitMetadata] {
    MASS_UNITS
}

fn get_time_units() -> &'static [TimeUnitMetadata] {
    TIME_UNITS
}

// ============================================================================
// Helper Functions
// ============================================================================

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

fn build_units_from_metadata() -> HashMap<&'static str, UnitParams> {
    let mut units: HashMap<&'static str, UnitParams> = HashMap::new();
    
    // Add length units
    for unit in get_length_units() {
        units.insert(
            unit.short_name,
            UnitParams::new(
                1, // length exponent
                unit.scale_value as i32,
                0, // mass exponent
                i32::MAX, // mass scale (unused)
                0, // time exponent
                i32::MAX, // time p2 (unused)
                i32::MAX, // time p3 (unused)
                i32::MAX, // time p5 (unused)
                i32::MAX, // time scale order (unused)
            )
        );
    }
    
    // Add mass units
    for unit in get_mass_units() {
        units.insert(
            unit.short_name,
            UnitParams::new(
                0, // length exponent
                i32::MAX, // length scale (unused)
                1, // mass exponent
                unit.scale_value as i32,
                0, // time exponent
                i32::MAX, // time p2 (unused)
                i32::MAX, // time p3 (unused)
                i32::MAX, // time p5 (unused)
                i32::MAX, // time scale order (unused)
            )
        );
    }
    
    // Add time units
    for unit in get_time_units() {
        units.insert(
            unit.short_name,
            UnitParams::new(
                0, // length exponent
                i32::MAX, // length scale (unused)
                0, // mass exponent
                i32::MAX, // mass scale (unused)
                1, // time exponent
                unit.p2 as i32,
                unit.p3 as i32,
                unit.p5 as i32,
                unit.scale_order as i32,
            )
        );
    }
    
    units
}

// ============================================================================
// Helper Functions for Pattern Generation
// ============================================================================

fn generate_quantity_type_with_exponents(
    length_exp: &str,
    length_scale: i32,
    mass_exp: &str,
    mass_scale: i32,
    time_exp: &str,
    time_p2: i32,
    time_p3: i32,
    time_p5: i32,
    time_scale_order: i32,
) -> String {
    format!(
        "crate::Quantity<\n                {}, {},\n                {}, {},\n                {}, {}, {}, {}, {}\n            >",
        length_exp, length_scale,
        mass_exp, mass_scale,
        time_exp, time_p2, time_p3, time_p5, time_scale_order
    )
}

fn generate_velocity_pattern(length_unit: &UnitMetadata, time_unit: &TimeUnitMetadata, params: &UnitParams) -> String {
    format!(
        "    ({} / {}) => {{ {} }};",
        length_unit.short_name,
        time_unit.short_name,
        params.generate_quantity_type()
    )
}

fn generate_acceleration_pattern(length_unit: &UnitMetadata, time_unit: &TimeUnitMetadata) -> String {
    let quantity_type = generate_quantity_type_with_exponents(
        "$exp", length_unit.scale_value as i32,
        "0", i32::MAX,
        "-$exp", time_unit.p2 as i32, time_unit.p3 as i32, time_unit.p5 as i32, time_unit.scale_order as i32
    );
    
    format!(
        "    ({} / {}^$exp:tt) => {{ {} }};",
        length_unit.short_name,
        time_unit.short_name,
        quantity_type
    )
}

fn generate_force_pattern(mass_unit: &UnitMetadata, length_unit: &UnitMetadata, time_unit: &TimeUnitMetadata) -> String {
    let quantity_type = generate_quantity_type_with_exponents(
        "$exp", length_unit.scale_value as i32,
        "$exp", mass_unit.scale_value as i32,
        "-$exp", time_unit.p2 as i32, time_unit.p3 as i32, time_unit.p5 as i32, time_unit.scale_order as i32
    );
    
    format!(
        "    ({} * {} / {}^$exp:tt) => {{ {} }};",
        mass_unit.short_name,
        length_unit.short_name,
        time_unit.short_name,
        quantity_type
    )
}

fn generate_energy_power_pattern(mass_unit: &UnitMetadata, length_unit: &UnitMetadata, time_unit: &TimeUnitMetadata) -> String {
    let quantity_type = generate_quantity_type_with_exponents(
        "$exp1", length_unit.scale_value as i32,
        "$exp1", mass_unit.scale_value as i32,
        "-$exp2", time_unit.p2 as i32, time_unit.p3 as i32, time_unit.p5 as i32, time_unit.scale_order as i32
    );
    
    format!(
        "    ({} * {}^$exp1:tt / {}^$exp2:tt) => {{ {} }};",
        mass_unit.short_name,
        length_unit.short_name,
        time_unit.short_name,
        quantity_type
    )
}

fn generate_complex_compound_pattern(
    unit1_name: &str,
    unit2_name: &str,
    unit3_name: &str,
    final_params: &UnitParams
) -> String {
    format!(
        "    ({} * {} / {}) => {{ {} }};",
        unit1_name,
        unit2_name,
        unit3_name,
        final_params.generate_quantity_type()
    )
}

fn generate_single_unit_pattern(unit_name: &str, params: &UnitParams) -> String {
    format!(
        "    ({}) => {{ {} }};",
        unit_name,
        params.generate_quantity_type()
    )
}

fn generate_unit_with_exponent_pattern(unit_name: &str, scale_value: i32) -> String {
    let quantity_type = generate_quantity_type_with_exponents(
        "$exp", scale_value,
        "0", i32::MAX,
        "0", i32::MAX, i32::MAX, i32::MAX, i32::MAX
    );
    
    format!(
        "    ({}^$exp:tt) => {{ {} }};",
        unit_name,
        quantity_type
    )
}

fn generate_time_unit_with_exponent_pattern(unit: &TimeUnitMetadata) -> String {
    let quantity_type = generate_quantity_type_with_exponents(
        "0", i32::MAX,
        "0", i32::MAX,
        "$exp", unit.p2 as i32, unit.p3 as i32, unit.p5 as i32, unit.scale_order as i32
    );
    
    format!(
        "    ({}^$exp:tt) => {{ {} }};",
        unit.short_name,
        quantity_type
    )
}

fn generate_compound_unit_pattern(unit1_name: &str, unit2_name: &str, params: &UnitParams) -> String {
    format!(
        "    ({} * {}) => {{ {} }};",
        unit1_name,
        unit2_name,
        params.generate_quantity_type()
    )
}

// ============================================================================
// Pattern Generation
// ============================================================================

fn generate_macro_patterns() -> Vec<String> {
    let mut patterns = Vec::new();
    
    // Unit definitions from centralized metadata
    let units = build_units_from_metadata();

    // ============================================================================
    // SINGLE UNITS (Base units)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // SINGLE UNITS (Base units)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    // Length units
    patterns.push("    // Length units".to_string());
    for unit in get_length_units() {
        let params = &units[&unit.short_name];
        patterns.push(generate_single_unit_pattern(&unit.short_name, params));
    }
    
    // Mass units  
    patterns.push("    // Mass units".to_string());
    for unit in get_mass_units() {
        let params = &units[&unit.short_name];
        patterns.push(generate_single_unit_pattern(&unit.short_name, params));
    }
    
    // Time units
    patterns.push("    // Time units".to_string());
    for unit in get_time_units() {
        let params = &units[&unit.short_name];
        patterns.push(generate_single_unit_pattern(&unit.short_name, params));
    }
    
    patterns.push("".to_string());
    
    // ============================================================================
    // UNITS WITH EXPONENTS (Squared, cubed, etc.)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // UNITS WITH EXPONENTS (Squared, cubed, etc.)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    // Length units with dynamic exponents
    patterns.push("    // Length units with dynamic exponents".to_string());
    for unit in get_length_units() {
        patterns.push(generate_unit_with_exponent_pattern(unit.short_name, unit.scale_value as i32));
    }
    
    // Mass units with dynamic exponents
    patterns.push("    // Mass units with dynamic exponents".to_string());
    for unit in get_mass_units() {
        patterns.push(generate_unit_with_exponent_pattern(unit.short_name, unit.scale_value as i32));
    }
    
    // Time units with dynamic exponents
    patterns.push("    // Time units with dynamic exponents".to_string());
    for unit in get_time_units() {
        patterns.push(generate_time_unit_with_exponent_pattern(&unit));
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
    for unit1 in get_length_units() {
        for unit2 in get_length_units() {
            if unit1.short_name <= unit2.short_name {  // Avoid duplicates
                let params = combine_units(&units[unit1.short_name], &units[unit2.short_name], "*");
                patterns.push(generate_compound_unit_pattern(unit1.short_name, unit2.short_name, &params));
            }
        }
    }
    
    // Mass × Time
    patterns.push("    // Mass × Time".to_string());
    for mass_unit in get_mass_units() {
        for time_unit in get_time_units() {
            let params = combine_units(&units[mass_unit.short_name], &units[time_unit.short_name], "*");
            patterns.push(generate_compound_unit_pattern(mass_unit.short_name, time_unit.short_name, &params));
        }
    }
    
    patterns.push("".to_string());
    
    // ============================================================================
    // VELOCITY UNITS (Length / Time)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // VELOCITY UNITS (Length / Time)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    for length_unit in get_length_units() {
        for time_unit in get_time_units() {
            let params = combine_units(&units[length_unit.short_name], &units[time_unit.short_name], "/");
            patterns.push(generate_velocity_pattern(length_unit, time_unit, &params));
        }
    }
    
    patterns.push("".to_string());
    
    // ============================================================================
    // ACCELERATION UNITS (Length / Time²)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // ACCELERATION UNITS (Length / Time²)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    for length_unit in get_length_units() {
        for time_unit in get_time_units() {
            patterns.push(generate_acceleration_pattern(&length_unit, &time_unit));
        }
    }
    
    patterns.push("".to_string());
    
    // ============================================================================
    // FORCE UNITS (Mass × Length / Time²)
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // FORCE UNITS (Mass × Length / Time²)".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    for mass_unit in get_mass_units() {
        for length_unit in get_length_units() {
            for time_unit in get_time_units() {
                patterns.push(generate_force_pattern(&mass_unit, &length_unit, &time_unit));
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
    
    for mass_unit in get_mass_units() {
        for length_unit in get_length_units() {
            for time_unit in get_time_units() {
                patterns.push(generate_energy_power_pattern(&mass_unit, &length_unit, &time_unit));
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
    
    for mass_unit in get_mass_units() {
        for length_unit in get_length_units() {
            for time_unit in get_time_units() {
                patterns.push(generate_energy_power_pattern(&mass_unit, &length_unit, &time_unit));
            }
        }
    }
    
    patterns.push("".to_string());
    
    // ============================================================================
    // COMPLEX COMPOUND UNITS
    // ============================================================================
    patterns.push("    // ============================================================================".to_string());
    patterns.push("    // COMPLEX COMPOUND UNITS".to_string());
    patterns.push("    // ============================================================================".to_string());
    
    // (Length * Length) / Time (area per time)
    for length_unit1 in get_length_units() {
        for length_unit2 in get_length_units() {
            for time_unit in get_time_units() {
                let temp_params = combine_units(&units[length_unit1.short_name], &units[length_unit2.short_name], "*");
                let final_params = combine_units(&temp_params, &units[time_unit.short_name], "/");
                patterns.push(generate_complex_compound_pattern(length_unit1.short_name, length_unit2.short_name, time_unit.short_name, &final_params));
            }
        }
    }
    
    // (Mass * Mass) / Time (mass squared per time)
    for mass_unit1 in get_mass_units() {
        for mass_unit2 in get_mass_units() {
            for time_unit in get_time_units() {
                let temp_params = combine_units(&units[mass_unit1.short_name], &units[mass_unit2.short_name], "*");
                let final_params = combine_units(&temp_params, &units[time_unit.short_name], "/");
                patterns.push(generate_complex_compound_pattern(mass_unit1.short_name, mass_unit2.short_name, time_unit.short_name, &final_params));
            }
        }
    }
    
    // (Time * Time) / Time (time)
    for time_unit1 in get_time_units() {
        for time_unit2 in get_time_units() {
            for time_unit3 in get_time_units() {
                let temp_params = combine_units(&units[time_unit1.short_name], &units[time_unit2.short_name], "*");
                let final_params = combine_units(&temp_params, &units[time_unit3.short_name], "/");
                patterns.push(generate_complex_compound_pattern(time_unit1.short_name, time_unit2.short_name, time_unit3.short_name, &final_params));
            }
        }
    }
    
    patterns.push("".to_string());
    
    patterns
}

fn generate_macro_file() -> String {
    let header = r#"// Auto-generated unit macro with human-readable triangular structure
// Generated by generate_unit_macros_clean.rs
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
    println!("📝 All patterns are generated from centralized metadata.");
}
