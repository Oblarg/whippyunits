use std::fs;
use std::collections::HashSet;

// ============================================================================
// Import data sources and shared utilities
// ============================================================================

mod dimension_data;
use dimension_data::{get_unit_scales, ScaleFactor};

// ============================================================================
// Helper functions for parameter generation
// ============================================================================

fn format_prime_name(prime: i8) -> String {
    format!("P{}", prime)
}

fn format_constant_name(constant: &str) -> String {
    constant.to_uppercase()
}

fn collect_all_parameters(unit_scales: &[dimension_data::UnitScale]) -> Vec<String> {
    let mut params = Vec::new();
    
    for scale in unit_scales {
        let scale_name_upper = scale.name.to_uppercase();
        
        // Add exponent parameter
        params.push(format!("const {}_EXPONENT: i8", scale_name_upper));
        
        // Add prime scale parameters
        for prime in &scale.primes {
            let prime_name = format_prime_name(*prime);
            params.push(format!("const {}_SCALE_{}: i8", scale_name_upper, prime_name));
        }
        
        // Add constant scale parameters from composite levels
        if let Some(composite_levels) = &scale.composite_prime_levels {
            let mut constants_added = HashSet::new();
            for factors in composite_levels.values() {
                for factor in factors {
                    if let ScaleFactor::Constant(constant_def, _) = factor {
                        if constants_added.insert(constant_def.name) {
                            let constant_name_upper = format_constant_name(constant_def.name);
                            params.push(format!("const {}_SCALE_{}: i8", scale_name_upper, constant_name_upper));
                        }
                    }
                }
            }
        }
    }
    
    // Add type parameter
    params.push("T = f64".to_string());
    params
}

fn collect_impl_type_parameters(unit_scales: &[dimension_data::UnitScale]) -> Vec<String> {
    let mut params = Vec::new();
    
    for scale in unit_scales {
        let scale_name_upper = scale.name.to_uppercase();
        
        // Add exponent parameter
        params.push(format!("const {}_EXPONENT: i8", scale_name_upper));
        
        // Add prime scale parameters
        for prime in &scale.primes {
            let prime_name = format_prime_name(*prime);
            params.push(format!("const {}_SCALE_{}: i8", scale_name_upper, prime_name));
        }
        
        // Add constant scale parameters from composite levels
        if let Some(composite_levels) = &scale.composite_prime_levels {
            let mut constants_added = HashSet::new();
            for factors in composite_levels.values() {
                for factor in factors {
                    if let ScaleFactor::Constant(constant_def, _) = factor {
                        if constants_added.insert(constant_def.name) {
                            let constant_name_upper = format_constant_name(constant_def.name);
                            params.push(format!("const {}_SCALE_{}: i8", scale_name_upper, constant_name_upper));
                        }
                    }
                }
            }
        }
    }
    
    // Add type parameter
    params.push("T".to_string());
    params
}

fn collect_impl_struct_parameters(unit_scales: &[dimension_data::UnitScale]) -> Vec<String> {
    let mut params = Vec::new();
    
    for scale in unit_scales {
        let scale_name_upper = scale.name.to_uppercase();
        
        // Add exponent parameter
        params.push(format!("{}_EXPONENT", scale_name_upper));
        
        // Add prime scale parameters
        for prime in &scale.primes {
            let prime_name = format_prime_name(*prime);
            params.push(format!("{}_SCALE_{}", scale_name_upper, prime_name));
        }
        
        // Add constant scale parameters from composite levels
        if let Some(composite_levels) = &scale.composite_prime_levels {
            let mut constants_added = HashSet::new();
            for factors in composite_levels.values() {
                for factor in factors {
                    if let ScaleFactor::Constant(constant_def, _) = factor {
                        if constants_added.insert(constant_def.name) {
                            let constant_name_upper = format_constant_name(constant_def.name);
                            params.push(format!("{}_SCALE_{}", scale_name_upper, constant_name_upper));
                        }
                    }
                }
            }
        }
    }
    
    // Add type parameter
    params.push("T".to_string());
    params
}

fn collect_macro_parameters(unit_scales: &[dimension_data::UnitScale]) -> Vec<String> {
    let mut params = Vec::new();
    
    for scale in unit_scales {
        let scale_name_upper = scale.name.to_uppercase();
        
        // Add exponent parameter
        params.push(format!("{}_EXPONENT", scale_name_upper));
        
        // Add prime scale parameters
        for prime in &scale.primes {
            let prime_name = format_prime_name(*prime);
            params.push(format!("{}_SCALE_{}", scale_name_upper, prime_name));
        }
        
        // Add constant scale parameters from composite levels
        if let Some(composite_levels) = &scale.composite_prime_levels {
            let mut constants_added = HashSet::new();
            for factors in composite_levels.values() {
                for factor in factors {
                    if let ScaleFactor::Constant(constant_def, _) = factor {
                        if constants_added.insert(constant_def.name) {
                            let constant_name_upper = format_constant_name(constant_def.name);
                            params.push(format!("{}_SCALE_{}", scale_name_upper, constant_name_upper));
                        }
                    }
                }
            }
        }
    }
    
    // Add type parameter
    params.push("T".to_string());
    params
}

fn collect_macro_parameters_with_type(unit_scales: &[dimension_data::UnitScale]) -> Vec<String> {
    let mut params = Vec::new();
    
    for scale in unit_scales {
        let scale_name_upper = scale.name.to_uppercase();
        
        // Add exponent parameter
        params.push(format!("{}_EXPONENT", scale_name_upper));
        
        // Add prime scale parameters
        for prime in &scale.primes {
            let prime_name = format_prime_name(*prime);
            params.push(format!("{}_SCALE_{}", scale_name_upper, prime_name));
        }
        
        // Add constant scale parameters from composite levels
        if let Some(composite_levels) = &scale.composite_prime_levels {
            let mut constants_added = HashSet::new();
            for factors in composite_levels.values() {
                for factor in factors {
                    if let ScaleFactor::Constant(constant_def, _) = factor {
                        if constants_added.insert(constant_def.name) {
                            let constant_name_upper = format_constant_name(constant_def.name);
                            params.push(format!("{}_SCALE_{}", scale_name_upper, constant_name_upper));
                        }
                    }
                }
            }
        }
    }
    
    // Add type parameter
    params.push("$T".to_string());
    params
}

// ============================================================================
// Main generation function
// ============================================================================

fn generate_quantity_type_file() -> String {
    let unit_scales = get_unit_scales();
    
    // Generate all parameter lists
    let struct_params = collect_all_parameters(&unit_scales);
    let impl_type_params = collect_impl_type_parameters(&unit_scales);
    let impl_struct_params = collect_impl_struct_parameters(&unit_scales);
    let macro_params = collect_macro_parameters(&unit_scales);
    let macro_params_with_type = collect_macro_parameters_with_type(&unit_scales);
    
    // Generate macro patterns
    let macro_patterns = format!(
        "    () => {{\n        Quantity<\n            {}\n        >\n    }};\n    ($T:ty) => {{\n        Quantity<\n            {}\n        >\n    }};",
        macro_params.join(",\n            "),
        macro_params_with_type.join(",\n            ")
    );
    
    format!(r#"//! Generated Quantity Type with Full Base Unit Dimensions
//! 
//! This file is auto-generated from dimension_data.rs and includes support
//! for all base unit dimensions defined in the system.
//! 
//! Base dimensions supported:
//! {}

#[derive(Clone, Copy, PartialEq)]
pub struct Quantity<
    {}
> {{
    pub value: T,
}}

impl<
    {}
>
    Quantity<
        {}
    >
{{
    pub fn new(value: T) -> Self {{
        Self {{ value }}
    }}
}}

#[macro_export]
macro_rules! quantity_type {{
{}
}}
"#,
        unit_scales.iter()
            .map(|s| format!("//! - {} (primes: {:?})", s.name, s.primes))
            .collect::<Vec<_>>()
            .join("\n"),
        struct_params.join(",\n    "),
        impl_type_params.join(",\n    "),
        impl_struct_params.join(",\n        "),
        macro_patterns
    )
}

fn main() {
    let output = generate_quantity_type_file();
    
    // Write to file
    fs::write("../src/generated_quantity_type.rs", &output).expect("Failed to write file");
    
    let unit_scales = get_unit_scales();
    let total_params = unit_scales.iter()
        .map(|s| {
            let mut count = 1; // exponent
            count += s.primes.len(); // primes
            
            // Add constants from composite levels
            if let Some(composite_levels) = &s.composite_prime_levels {
                let mut constants_added = HashSet::new();
                for factors in composite_levels.values() {
                    for factor in factors {
                        if let ScaleFactor::Constant(constant_name, _) = factor {
                            if constants_added.insert(constant_name) {
                                count += 1;
                            }
                        }
                    }
                }
            }
            count
        })
        .sum::<usize>();
    
    println!("✅ Generated src/generated_quantity_type.rs");
    println!("📊 Generated Quantity struct with {} total parameters", total_params);
    println!("🔧 Base dimensions supported:");
    for scale in &unit_scales {
        println!("   • {} (primes: {:?})", scale.name, scale.primes);
    }
    println!();
    println!("🚀 Ready to use! The expanded Quantity type now supports all base unit dimensions.");
    println!("📝 All parameters are generated from centralized dimension data.");
}