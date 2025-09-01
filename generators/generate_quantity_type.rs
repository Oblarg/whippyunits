use std::fs;

// ============================================================================
// Import data sources and shared utilities
// ============================================================================

mod unit_data;
mod generator_utils;
use unit_data::{UnitMetadata, LENGTH_UNITS, MASS_UNITS, TIME_UNITS, ALL_DIMENSIONS, DIMENSION_NAMES};
use generator_utils::*;

// ============================================================================
// Quantity Type Generation
// ============================================================================

fn generate_quantity_type() -> String {
    let mut output = String::new();
    
    output.push_str("// Auto-generated Quantity type definition\n");
    output.push_str("// Generated from dimensional_metadata.rs\n");
    output.push_str("// DO NOT EDIT - This file is auto-generated\n\n");
    
    output.push_str("use core::f64;\n");
    output.push_str("use core::ops::{Add, Div, Mul, Sub};\n\n");
    
    // Generate the Quantity struct definition
    output.push_str("#[derive(Clone, Copy)]\n");
    output.push_str("pub struct Quantity<\n");
    
    // Generate const generic parameters for each dimension
    output.push_str(&generate_const_generic_params());
    
    output.push_str("> {\n");
    output.push_str("    pub value: f64,\n");
    output.push_str("}\n\n");
    
    // Generate the impl block
    output.push_str("impl<\n");
    
    // Repeat the const generic parameters for the impl
    output.push_str(&generate_const_generic_params());
    
    output.push_str(">\n");
    output.push_str("    Quantity<\n");
    
    // Repeat the const generic parameters for the type
    output.push_str(&generate_quantity_type_params());
    
    output.push_str("    >\n");
    output.push_str("{\n");
    output.push_str("    pub fn new(value: f64) -> Self {\n");
    output.push_str("        Self { value }\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    // Generate Display and Debug implementations
    output.push_str("// ============================================================================\n");
    output.push_str("// Display and Debug Implementations\n");
    output.push_str("// ============================================================================\n\n");
    
    output.push_str("use core::fmt;\n\n");
    
    output.push_str("impl<\n");
    
    // Repeat const generic parameters for Display impl
    for (dim_index, units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        
        if let Some(first_unit) = units.first() {
            if first_unit.is_time_unit() {
                output.push_str(&format!("    const {}_EXPONENT: isize, const {}_P2: isize, const {}_P3: isize, const {}_P5: isize,\n",
                    dimension_name, dimension_name, dimension_name, dimension_name));
            } else {
                output.push_str(&format!("    const {}_EXPONENT: isize, const {}_SCALE: isize,\n",
                    dimension_name, dimension_name));
            }
        }
    }
    
    output.push_str(">\n");
    output.push_str("    fmt::Display\n");
    output.push_str("    for Quantity<\n");
    
    // Repeat const generic parameters for the type
    for (dim_index, units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        
        if let Some(first_unit) = units.first() {
            if first_unit.is_time_unit() {
                output.push_str(&format!("        {}_EXPONENT, {}_P2, {}_P3, {}_P5,\n",
                    dimension_name, dimension_name, dimension_name, dimension_name));
            } else {
                output.push_str(&format!("        {}_EXPONENT, {}_SCALE,\n",
                    dimension_name, dimension_name));
            }
        }
    }
    
    output.push_str("    >\n");
    output.push_str("{\n");
    output.push_str("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n");
    output.push_str("        write!(f, \"{} {}\", self.value, \"[units]\")\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    output.push_str("impl<\n");
    
    // Repeat const generic parameters for Debug impl
    for (dim_index, units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        
        if let Some(first_unit) = units.first() {
            if first_unit.is_time_unit() {
                output.push_str(&format!("    const {}_EXPONENT: isize, const {}_P2: isize, const {}_P3: isize, const {}_P5: isize,\n",
                    dimension_name, dimension_name, dimension_name, dimension_name));
            } else {
                output.push_str(&format!("    const {}_EXPONENT: isize, const {}_SCALE: isize,\n",
                    dimension_name, dimension_name));
            }
        }
    }
    
    output.push_str(">\n");
    output.push_str("    fmt::Debug\n");
    output.push_str("    for Quantity<\n");
    
    // Repeat const generic parameters for the type
    for (dim_index, units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        
        if let Some(first_unit) = units.first() {
            if first_unit.is_time_unit() {
                output.push_str(&format!("        {}_EXPONENT, {}_P2, {}_P3, {}_P5,\n",
                    dimension_name, dimension_name, dimension_name, dimension_name));
            } else {
                output.push_str(&format!("        {}_EXPONENT, {}_SCALE,\n",
                    dimension_name, dimension_name));
            }
        }
    }
    
    output.push_str("    >\n");
    output.push_str("{\n");
    output.push_str("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n");
    output.push_str("        write!(f, \"Quantity({} {})\", self.value, \"[units]\")\n");
    output.push_str("    }\n");
    output.push_str("}\n");
    
    output
}

// ============================================================================
// Main Generation Function
// ============================================================================

fn main() {
    let generated_code = generate_quantity_type();
    
    // Write to src/generated_quantity_type.rs
    fs::write("src/generated_quantity_type.rs", generated_code).unwrap();
    
    println!("Generated Quantity type in src/generated_quantity_type.rs");
    println!("To use this type, add 'mod generated_quantity_type;' to src/lib.rs");
}
