use std::fs;

// ============================================================================
// Configuration
// ============================================================================

/// Maximum dimensionality for pow functions (currently prototyping with 3)
const MAX_DIMENSIONALITY: i32 = 3;

// ============================================================================
// Unit Metadata Structures (copied from dimensional_metadata.rs)
// ============================================================================

#[derive(Debug, Clone)]
struct UnitMetadata {
    scale_value: i32,
    short_name: String,
    long_name: String,
    dimension: String,
    is_base_unit: bool,
    exponential_scale: i32,
}

#[derive(Debug, Clone)]
struct TimeUnitMetadata {
    scale_order: i32,
    p2: i32,
    p3: i32,
    p5: i32,
    short_name: String,
    long_name: String,
    dimension: String,
    is_base_unit: bool,
    exponential_scales: [(i32, i32); 3], // [(prime_factor, exponent), ...]
}

// ============================================================================
// Unit Definitions (copied from dimensional_metadata.rs)
// ============================================================================

fn get_length_units() -> Vec<UnitMetadata> {
    vec![
        UnitMetadata {
            scale_value: -1,
            short_name: "mm".to_string(),
            long_name: "millimeter".to_string(),
            dimension: "Length".to_string(),
            is_base_unit: false,
            exponential_scale: 1000,
        },
        UnitMetadata {
            scale_value: 0,
            short_name: "m".to_string(),
            long_name: "meter".to_string(),
            dimension: "Length".to_string(),
            is_base_unit: true,
            exponential_scale: 1000,
        },
        UnitMetadata {
            scale_value: 1,
            short_name: "km".to_string(),
            long_name: "kilometer".to_string(),
            dimension: "Length".to_string(),
            is_base_unit: false,
            exponential_scale: 1000,
        },
    ]
}

fn get_mass_units() -> Vec<UnitMetadata> {
    vec![
        UnitMetadata {
            scale_value: -1,
            short_name: "mg".to_string(),
            long_name: "milligram".to_string(),
            dimension: "Mass".to_string(),
            is_base_unit: false,
            exponential_scale: 1000,
        },
        UnitMetadata {
            scale_value: 0,
            short_name: "g".to_string(),
            long_name: "gram".to_string(),
            dimension: "Mass".to_string(),
            is_base_unit: false,
            exponential_scale: 1000,
        },
        UnitMetadata {
            scale_value: 1,
            short_name: "kg".to_string(),
            long_name: "kilogram".to_string(),
            dimension: "Mass".to_string(),
            is_base_unit: true,
            exponential_scale: 1000,
        },
    ]
}

fn get_time_units() -> Vec<TimeUnitMetadata> {
    vec![
        TimeUnitMetadata {
            scale_order: -1,
            p2: -3,
            p3: 0,
            p5: -3,
            short_name: "ms".to_string(),
            long_name: "millisecond".to_string(),
            dimension: "Time".to_string(),
            is_base_unit: false,
            exponential_scales: [(2, -3), (3, 0), (5, -3)],
        },
        TimeUnitMetadata {
            scale_order: 0,
            p2: 0,
            p3: 0,
            p5: 0,
            short_name: "s".to_string(),
            long_name: "second".to_string(),
            dimension: "Time".to_string(),
            is_base_unit: true,
            exponential_scales: [(2, 0), (3, 0), (5, 0)],
        },
        TimeUnitMetadata {
            scale_order: 1,
            p2: 2,
            p3: 1,
            p5: 1,
            short_name: "min".to_string(),
            long_name: "minute".to_string(),
            dimension: "Time".to_string(),
            is_base_unit: false,
            exponential_scales: [(2, 2), (3, 1), (5, 1)],
        },
    ]
}

// ============================================================================
// Constant Generation
// ============================================================================

fn generate_length_constants() -> String {
    let units = get_length_units();
    let mut output = String::new();
    
    output.push_str("// ============================================================================\n");
    output.push_str("// Length Scale Constants (Auto-generated)\n");
    output.push_str("// ============================================================================\n\n");
    
    for unit in &units {
        let const_name = match unit.short_name.as_str() {
            "mm" => "MILLIMETER_SCALE",
            "m" => "METER_SCALE", 
            "km" => "KILOMETER_SCALE",
            _ => &format!("{}_SCALE", unit.short_name.to_uppercase()),
        };
        output.push_str(&format!("pub const {}: isize = {};\n", const_name, unit.scale_value));
    }
    
    output.push_str("pub const LENGTH_UNUSED: isize = isize::MAX;\n\n");
    
    output
}

fn generate_mass_constants() -> String {
    let units = get_mass_units();
    let mut output = String::new();
    
    output.push_str("// ============================================================================\n");
    output.push_str("// Mass Scale Constants (Auto-generated)\n");
    output.push_str("// ============================================================================\n\n");
    
    for unit in &units {
        let const_name = match unit.short_name.as_str() {
            "mg" => "MILLIGRAM_SCALE",
            "g" => "GRAM_SCALE",
            "kg" => "KILOGRAM_SCALE",
            _ => &format!("{}_SCALE", unit.short_name.to_uppercase()),
        };
        output.push_str(&format!("pub const {}: isize = {};\n", const_name, unit.scale_value));
    }
    
    output.push_str("pub const MASS_UNUSED: isize = isize::MAX;\n\n");
    
    output
}

fn generate_time_constants() -> String {
    let units = get_time_units();
    let mut output = String::new();
    
    output.push_str("// ============================================================================\n");
    output.push_str("// Time Scale Constants (Auto-generated)\n");
    output.push_str("// ============================================================================\n\n");
    
    for unit in &units {
        let base_name = match unit.short_name.as_str() {
            "ms" => "MILLISECOND",
            "s" => "SECOND",
            "min" => "MINUTE",
            _ => &unit.short_name.to_uppercase(),
        };
        output.push_str(&format!("pub const {}_SCALE_ORDER: isize = {};\n", base_name, unit.scale_order));
        output.push_str(&format!("pub const {}_SCALE_P2: isize = {};\n", base_name, unit.p2));
        output.push_str(&format!("pub const {}_SCALE_P3: isize = {};\n", base_name, unit.p3));
        output.push_str(&format!("pub const {}_SCALE_P5: isize = {};\n", base_name, unit.p5));
        output.push_str("\n");
    }
    
    output.push_str("pub const TIME_UNUSED: isize = isize::MAX;\n\n");
    
    output
}

fn generate_pow_functions() -> String {
    let mut output = String::new();
    
    output.push_str("// ============================================================================\n");
    output.push_str("// Power Functions (Auto-generated)\n");
    output.push_str("// ============================================================================\n\n");
    
    // Generate pow1000 for length/mass (they all use 1000)
    output.push_str("const fn pow1000(exp: isize) -> f64 {\n");
    output.push_str("    match exp {\n");
    for i in -MAX_DIMENSIONALITY..=MAX_DIMENSIONALITY {
        let value = if i >= 0 { 1000.0_f64.powi(i) } else { 1.0 / 1000.0_f64.powi(-i) };
        // Check if the value is an integer (whole number)
        if value.fract() == 0.0 {
            output.push_str(&format!("        {} => {}.0_f64,\n", i, value as i64));
        } else {
            output.push_str(&format!("        {} => {}_f64,\n", i, value));
        }
    }
    output.push_str("        _ => 1.0_f64, // we'll only test small values during prototyping\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    // Generate pow2, pow3, pow5 for time units
    output.push_str("pub const fn pow2(exp: isize) -> f64 {\n");
    output.push_str("    match exp {\n");
    for i in -MAX_DIMENSIONALITY..=MAX_DIMENSIONALITY {
        let value = if i >= 0 { 2.0_f64.powi(i) } else { 1.0 / 2.0_f64.powi(-i) };
        // Check if the value is an integer (whole number)
        if value.fract() == 0.0 {
            output.push_str(&format!("        {} => {}.0_f64,\n", i, value as i64));
        } else {
            output.push_str(&format!("        {} => {}_f64,\n", i, value));
        }
    }
    output.push_str("        _ => 1.0_f64, // we'll only test small values during prototyping\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    output.push_str("pub const fn pow3(exp: isize) -> f64 {\n");
    output.push_str("    match exp {\n");
    for i in -MAX_DIMENSIONALITY..=MAX_DIMENSIONALITY {
        let value = if i >= 0 { 3.0_f64.powi(i) } else { 1.0 / 3.0_f64.powi(-i) };
        // Check if the value is an integer (whole number)
        if value.fract() == 0.0 {
            output.push_str(&format!("        {} => {}.0_f64,\n", i, value as i64));
        } else {
            output.push_str(&format!("        {} => {}_f64,\n", i, value));
        }
    }
    output.push_str("        _ => 1.0_f64, // we'll only test small values during prototyping\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    output.push_str("pub const fn pow5(exp: isize) -> f64 {\n");
    output.push_str("    match exp {\n");
    for i in -MAX_DIMENSIONALITY..=MAX_DIMENSIONALITY {
        let value = if i >= 0 { 5.0_f64.powi(i) } else { 1.0 / 5.0_f64.powi(-i) };
        // Check if the value is an integer (whole number)
        if value.fract() == 0.0 {
            output.push_str(&format!("        {} => {}.0_f64,\n", i, value as i64));
        } else {
            output.push_str(&format!("        {} => {}_f64,\n", i, value));
        }
    }
    output.push_str("        _ => 1.0_f64, // we'll only test small values during prototyping\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    output
}

fn generate_conversion_functions() -> String {
    let mut output = String::new();
    
    output.push_str("// ============================================================================\n");
    output.push_str("// Generic Conversion Functions (Auto-generated)\n");
    output.push_str("// ============================================================================\n\n");
    
    let single_scale_dimensions = collect_single_scale_dimensions();
    let multi_scale_dimensions = collect_multi_scale_dimensions();
    
    output.push_str(&generate_dimension_specific_functions(&single_scale_dimensions, &multi_scale_dimensions));
    
    output
}

fn collect_single_scale_dimensions() -> Vec<(&'static str, i32, &'static str)> {
    let mut dimensions = Vec::new();
    
    let length_units = get_length_units();
    let mass_units = get_mass_units();
    
    if let Some(unit) = length_units.first() {
        dimensions.push(("Length", unit.exponential_scale, "LENGTH_UNUSED"));
    }
    if let Some(unit) = mass_units.first() {
        dimensions.push(("Mass", unit.exponential_scale, "MASS_UNUSED"));
    }
    
    dimensions
}

fn collect_multi_scale_dimensions() -> Vec<(&'static str, Vec<(i32, i32)>, &'static str)> {
    let mut dimensions = Vec::new();
    
    let time_units = get_time_units();
    if let Some(unit) = time_units.first() {
        dimensions.push(("Time", unit.exponential_scales.to_vec(), "TIME_UNUSED"));
    }
    
    dimensions
}

fn generate_unused_scale_patterns(num_factors: usize) -> Vec<String> {
    let mut patterns = Vec::new();
    for i in 0..num_factors * 2 {
        let mut pattern = vec!["_".to_string(); num_factors * 2];
        pattern[i] = "UNUSED".to_string();
        patterns.push(format!("({})", pattern.join(", ")));
    }
    patterns
}



fn generate_dimension_specific_functions(
    single_scale: &[(&str, i32, &str)],
    multi_scale: &[(&str, Vec<(i32, i32)>, &str)]
) -> String {
    let mut output = String::new();
    
    output.push_str("// ============================================================================\n");
    output.push_str("// Dimension-Specific Conversion Functions (Auto-generated)\n");
    output.push_str("// ============================================================================\n\n");
    
    // Generate specific functions for single-scale dimensions
    for (dim_name, exp_scale, unused_const) in single_scale {
        let func_name = format!("{}_conversion_factor", dim_name.to_lowercase());
        output.push_str(&format!("/// Convert between {} units\n", dim_name));
        output.push_str(&format!("pub const fn {}(from: isize, to: isize, exponent: isize) -> f64 {{\n", func_name));
        output.push_str("    let diff: isize = (from - to) * exponent;\n");
        output.push_str(&format!("    const UNUSED: isize = {};\n", unused_const));
        output.push_str("    match (from, to) {\n");
        output.push_str("        (UNUSED, _) | (_, UNUSED) => 1.0_f64,\n");
        output.push_str(&format!("        _ => pow{}(diff),\n", exp_scale));
        output.push_str("    }\n");
        output.push_str("}\n\n");
    }
    
    // Generate specific functions for multi-scale dimensions
    for (dim_name, exp_scales, unused_const) in multi_scale {
        let func_name = format!("{}_conversion_factor", dim_name.to_lowercase());
        output.push_str(&format!("/// Convert between {} units\n", dim_name));
        output.push_str(&format!("pub const fn {}(\n", func_name));
        
        // Generate parameter list based on the number of factors
        let mut from_params = Vec::new();
        let mut to_params = Vec::new();
        for (prime_factor, _) in exp_scales.iter() {
            from_params.push(format!("from_p{}: isize", prime_factor));
            to_params.push(format!("to_p{}: isize", prime_factor));
        }
        
        output.push_str(&format!("    {},\n", from_params.join(",\n    ")));
        output.push_str(&format!("    {},\n", to_params.join(",\n    ")));
        output.push_str("    exponent: isize,\n");
        output.push_str(") -> f64 {\n");
        
        // Generate diff variables
        for (prime_factor, _) in exp_scales.iter() {
            output.push_str(&format!("    let diff_p{}: isize = (from_p{} - to_p{}) * exponent;\n", prime_factor, prime_factor, prime_factor));
        }
        
        output.push_str(&format!("    const UNUSED: isize = {};\n", unused_const));
        
        // Generate match pattern for unused scales
        let num_factors = exp_scales.len();
        let match_patterns = generate_unused_scale_patterns(num_factors);
        
        // Generate match variables using parameter names directly
        let match_vars: Vec<String> = exp_scales.iter().flat_map(|(prime_factor, _)| {
            vec![format!("from_p{}", prime_factor), format!("to_p{}", prime_factor)]
        }).collect();
        
        output.push_str(&format!("    match ({}) {{\n", match_vars.join(", ")));
        
        for pattern in &match_patterns {
            output.push_str(&format!("        {} => 1.0_f64,\n", pattern));
        }
        
        // Generate the pow function calls
        let pow_calls: Vec<String> = exp_scales.iter().map(|(prime_factor, _)| {
            format!("pow{}(diff_p{})", prime_factor, prime_factor)
        }).collect();
        
        output.push_str(&format!("        _ => {},\n", pow_calls.join(" * ")));
        output.push_str("    }\n");
        output.push_str("}\n\n");
    }
    
    output
}

fn generate_time_scale_helper_functions() -> String {
    let mut output = String::new();
    
    output.push_str("// ============================================================================\n");
    output.push_str("// Time Scale Helper Functions (Auto-generated)\n");
    output.push_str("// ============================================================================\n\n");
    
    output.push_str("pub const fn time_scale_2(scale_order: isize) -> isize {\n");
    output.push_str("    match scale_order {\n");
    let time_units = get_time_units();
    for unit in &time_units {
        output.push_str(&format!("        {} => {},\n", unit.scale_order, unit.p2));
    }
    output.push_str("        _ => isize::MAX,\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    output.push_str("pub const fn time_scale_3(scale_order: isize) -> isize {\n");
    output.push_str("    match scale_order {\n");
    for unit in &time_units {
        output.push_str(&format!("        {} => {},\n", unit.scale_order, unit.p3));
    }
    output.push_str("        _ => isize::MAX,\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    output.push_str("pub const fn time_scale_5(scale_order: isize) -> isize {\n");
    output.push_str("    match scale_order {\n");
    for unit in &time_units {
        output.push_str(&format!("        {} => {},\n", unit.scale_order, unit.p5));
    }
    output.push_str("        _ => isize::MAX,\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    output
}

fn generate_display_functions() -> String {
    let mut output = String::new();
    
    output.push_str("// ============================================================================\n");
    output.push_str("// Display Helper Functions (Auto-generated)\n");
    output.push_str("// ============================================================================\n\n");
    
    // Length display functions
    output.push_str("pub const fn length_short_name(scale: isize) -> &'static str {\n");
    output.push_str("    match scale {\n");
    let length_units = get_length_units();
    for unit in &length_units {
        output.push_str(&format!("        {} => \"{}\",\n", unit.scale_value, unit.short_name));
    }
    output.push_str("        _ => \"unknown\",\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    output.push_str("pub const fn length_long_name(scale: isize) -> &'static str {\n");
    output.push_str("    match scale {\n");
    for unit in &length_units {
        output.push_str(&format!("        {} => \"{}\",\n", unit.scale_value, unit.long_name));
    }
    output.push_str("        _ => \"unknown\",\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    // Mass display functions
    output.push_str("pub const fn mass_short_name(scale: isize) -> &'static str {\n");
    output.push_str("    match scale {\n");
    let mass_units = get_mass_units();
    for unit in &mass_units {
        output.push_str(&format!("        {} => \"{}\",\n", unit.scale_value, unit.short_name));
    }
    output.push_str("        _ => \"unknown\",\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    output.push_str("pub const fn mass_long_name(scale: isize) -> &'static str {\n");
    output.push_str("    match scale {\n");
    for unit in &mass_units {
        output.push_str(&format!("        {} => \"{}\",\n", unit.scale_value, unit.long_name));
    }
    output.push_str("        _ => \"unknown\",\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    // Time display functions
    output.push_str("pub const fn time_short_name(scale_order: isize) -> &'static str {\n");
    output.push_str("    match scale_order {\n");
    let time_units = get_time_units();
    for unit in &time_units {
        output.push_str(&format!("        {} => \"{}\",\n", unit.scale_order, unit.short_name));
    }
    output.push_str("        _ => \"unknown\",\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    output.push_str("pub const fn time_long_name(scale_order: isize) -> &'static str {\n");
    output.push_str("    match scale_order {\n");
    for unit in &time_units {
        output.push_str(&format!("        {} => \"{}\",\n", unit.scale_order, unit.long_name));
    }
    output.push_str("        _ => \"unknown\",\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    output
}

// ============================================================================
// Main Generation Function
// ============================================================================

fn generate_all_constants() -> String {
    let mut output = String::new();
    
    output.push_str("// Auto-generated scale constants and helper functions\n");
    output.push_str("// Generated from dimensional_metadata.rs\n");
    output.push_str("// DO NOT EDIT - This file is auto-generated\n\n");
    
    output.push_str(&generate_length_constants());
    output.push_str(&generate_mass_constants());
    output.push_str(&generate_time_constants());
    output.push_str(&generate_time_scale_helper_functions());
    output.push_str(&generate_pow_functions());
    output.push_str(&generate_conversion_functions());
    output.push_str(&generate_display_functions());
    
    output
}

fn main() {
    let generated_code = generate_all_constants();
    
    // Write to src/generated_constants.rs
    fs::write("src/generated_constants.rs", generated_code).unwrap();
    
    println!("Generated scale constants in src/generated_constants.rs");
    println!("To use these constants, add 'mod generated_constants;' to src/lib.rs");
}
