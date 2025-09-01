// Shared utilities for code generation
// This ensures all generators produce consistent Quantity type structures
// All functions are completely data-driven from unit_data and dimension_data

// Removed unused import
use crate::unit_data::{ALL_DIMENSIONS, DIMENSION_NAMES};
use std::string::{String, ToString};
use std::vec::Vec;

// Note: This module provides utilities for generators to use
// The actual generators (generate_dimensions.rs, etc.) will import
// unit_data.rs and dimension_data.rs directly and use these utilities

// Define the UnitMetadata type that matches unit_data.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitMetadata {
    pub scale_order: isize,
    pub short_name: &'static str,
    pub long_name: &'static str,
    pub exponential_scales: &'static [(isize, isize)],
}

impl UnitMetadata {
    pub const fn is_time_unit(&self) -> bool {
        self.exponential_scales.len() > 1
    }
}

/// Generate const generic parameters for the Quantity type
/// This ensures all generators use the same parameter structure
pub fn generate_const_generic_params(all_dimensions: &[&[UnitMetadata]], dimension_names: &[&str]) -> String {
    let mut params = String::new();
    
    for (dim_index, units) in all_dimensions.iter().enumerate() {
        let dimension_name = dimension_names[dim_index].to_uppercase();
        
        if let Some(first_unit) = units.first() {
            if first_unit.is_time_unit() {
                // Complex dimension - needs P2, P3, P5, SCALE_ORDER
                params.push_str(&format!("    const {}_EXPONENT: isize, const {}_P2: isize, const {}_P3: isize, const {}_P5: isize, const {}_SCALE_ORDER: isize,\n",
                    dimension_name, dimension_name, dimension_name, dimension_name, dimension_name));
            } else {
                // Simple dimension - just scale
                params.push_str(&format!("    const {}_EXPONENT: isize, const {}_SCALE: isize,\n",
                    dimension_name, dimension_name));
            }
        }
    }
    
    params
}

/// Generate Quantity type parameters (without const keyword)
pub fn generate_quantity_type_params() -> String {
    let mut params = String::new();
    
    for (dim_index, units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        
        if let Some(first_unit) = units.first() {
            if first_unit.is_time_unit() {
                params.push_str(&format!("        {}_EXPONENT, {}_P2, {}_P3, {}_P5, {}_SCALE_ORDER,\n",
                    dimension_name, dimension_name, dimension_name, dimension_name, dimension_name));
            } else {
                params.push_str(&format!("        {}_EXPONENT, {}_SCALE,\n",
                    dimension_name, dimension_name));
            }
        }
    }
    
    params
}

/// Generate const generic parameters with suffix (for binary operations)
pub fn generate_const_generic_params_with_suffix(suffix: &str) -> String {
    let mut params = String::new();
    
    for (dim_index, units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        
        if let Some(first_unit) = units.first() {
            if first_unit.is_time_unit() {
                params.push_str(&format!("    const {}_EXPONENT{}: isize, const {}_P2{}: isize, const {}_P3{}: isize, const {}_P5{}: isize, const {}_SCALE_ORDER{}: isize,\n",
                    dimension_name, suffix, dimension_name, suffix, dimension_name, suffix, dimension_name, suffix, dimension_name, suffix));
            } else {
                params.push_str(&format!("    const {}_EXPONENT{}: isize, const {}_SCALE{}: isize,\n",
                    dimension_name, suffix, dimension_name, suffix));
            }
        }
    }
    
    params
}

/// Generate Quantity type parameters with suffix
pub fn generate_quantity_type_params_with_suffix(suffix: &str) -> String {
    let mut params = String::new();
    
    for (dim_index, units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        
        if let Some(first_unit) = units.first() {
            if first_unit.is_time_unit() {
                params.push_str(&format!("        {}_EXPONENT{}, {}_P2{}, {}_P3{}, {}_P5{}, {}_SCALE_ORDER{},\n",
                    dimension_name, suffix, dimension_name, suffix, dimension_name, suffix, dimension_name, suffix, dimension_name, suffix));
            } else {
                params.push_str(&format!("        {}_EXPONENT{}, {}_SCALE{},\n",
                    dimension_name, suffix, dimension_name, suffix));
            }
        }
    }
    
    params
}

/// Generate where clauses for IsIsize constraints
pub fn generate_where_clauses_for_exponent_arithmetic(log_op: &str) -> String {
    let mut clauses = String::new();
    
    for (dim_index, _units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        clauses.push_str(&format!("        (): IsIsize<{{ {}_EXPONENT1 {} {}_EXPONENT2 }}>,\n", 
            dimension_name, log_op, dimension_name));
    }
    
    clauses
}

/// Generate scale where clauses for complex dimensions (like time)
pub fn generate_scale_where_clauses(scale_order: &str) -> String {
    let mut clauses = String::new();
    
    // Find complex dimensions (those with multiple exponential scales)
    for (dim_index, units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        
        if let Some(first_unit) = units.first() {
            if first_unit.is_time_unit() {
                // Generate scale functions based on the dimension name
                clauses.push_str(&format!("        (): IsIsize<{{ {}_scale_2({}) }}>,\n", dimension_name.to_lowercase(), scale_order));
                clauses.push_str(&format!("        (): IsIsize<{{ {}_scale_3({}) }}>,\n", dimension_name.to_lowercase(), scale_order));
                clauses.push_str(&format!("        (): IsIsize<{{ {}_scale_5({}) }}>,\n", dimension_name.to_lowercase(), scale_order));
                break;
            }
        }
    }
    
    clauses
}

// These functions are not used by the unit macro generator
// Commented out to avoid compilation errors

/*
/// Get the canonical name (first name) from a dimension type
pub fn get_canonical_name(dimension: &DimensionType) -> &'static str {
    dimension.names.first().unwrap_or(&"Unknown")
}

/// Find a dimension by any of its names
pub fn find_dimension_by_name(name: &str) -> Option<DimensionType> {
    get_dimensional_data().into_iter().find(|d| d.names.contains(&name))
}

/// Generate the Quantity type reference for a given dimensional exponents
/// Maps dimension_data exponents to unit_data structure
pub fn generate_quantity_type_for_exponents(exponents: &DimensionalExponents, use_unused: bool) -> String {
    let mut quantity_type = String::new();
    quantity_type.push_str("Quantity<\n");
    
    // Map dimensional exponents to the unit_data structure
    // We need to map the dimension_data structure to unit_data structure
    // This requires knowing the mapping between dimension_data fields and unit_data indices
    
    // For now, we'll assume the mapping is:
    // dimension_data.length -> unit_data[0] (first dimension)
    // dimension_data.mass -> unit_data[1] (second dimension)  
    // dimension_data.time -> unit_data[2] (third dimension)
    
    // Map dimension names to unit_data indices
    let dimension_mapping: Vec<_> = DIMENSION_NAMES.iter().enumerate().collect();
    
    for (unit_index, dimension_name) in dimension_mapping {
        let exponent = *exponents.atomic_exponents.get(dimension_name).unwrap_or(&0);
        
        if unit_index < ALL_DIMENSIONS.len() {
            let dimension_name = DIMENSION_NAMES[unit_index].to_uppercase();
            let units = ALL_DIMENSIONS[unit_index];
            
            if let Some(first_unit) = units.first() {
                if first_unit.is_time_unit() {
                    // Complex dimension with P2, P3, P5
                    if exponent == 0 && use_unused {
                        quantity_type.push_str(&format!("    0, {}_UNUSED, {}_UNUSED, {}_UNUSED, {}_UNUSED,\n", 
                            dimension_name, dimension_name, dimension_name, dimension_name));
                    } else {
                        quantity_type.push_str(&format!("    {}, {{ {}_scale_2({}_SCALE_ORDER) }}, {{ {}_scale_3({}_SCALE_ORDER) }}, {{ {}_scale_5({}_SCALE_ORDER) }}, {}_SCALE_ORDER,\n", 
                            exponent, dimension_name.to_lowercase(), dimension_name, dimension_name.to_lowercase(), dimension_name, dimension_name.to_lowercase(), dimension_name, dimension_name));
                    }
                } else {
                    // Simple dimension with just scale
                    if exponent == 0 && use_unused {
                        quantity_type.push_str(&format!("    0, {}_UNUSED,\n", dimension_name));
                    } else {
                        quantity_type.push_str(&format!("    {}, {}_SCALE,\n", exponent, dimension_name));
                    }
                }
            }
        }
    }
    
    quantity_type.push_str(">\n");
    
    quantity_type
}

/// Generate where clauses for scale constraints based on dimensional exponents
pub fn generate_scale_where_clauses_for_exponents(exponents: &DimensionalExponents) -> String {
    let mut clauses = String::new();
    
    // Map dimension names to unit_data indices
    let dimension_mapping: Vec<_> = DIMENSION_NAMES.iter().enumerate().collect();
    
    for (unit_index, dimension_name) in dimension_mapping {
        let exponent = *exponents.atomic_exponents.get(dimension_name).unwrap_or(&0);
        
        if exponent != 0 && unit_index < ALL_DIMENSIONS.len() {
            let dimension_name = DIMENSION_NAMES[unit_index].to_uppercase();
            let units = ALL_DIMENSIONS[unit_index];
            
            if let Some(first_unit) = units.first() {
                if first_unit.is_time_unit() {
                    // Complex dimension needs scale where clauses
                    clauses.push_str(&format!("where (): IsIsize<{{ {}_scale_2({}_SCALE_ORDER) }}>,\n", 
                        dimension_name.to_lowercase(), dimension_name));
                    clauses.push_str(&format!("      (): IsIsize<{{ {}_scale_3({}_SCALE_ORDER) }}>,\n", 
                        dimension_name.to_lowercase(), dimension_name));
                    clauses.push_str(&format!("      (): IsIsize<{{ {}_scale_5({}_SCALE_ORDER) }}>\n", 
                        dimension_name.to_lowercase(), dimension_name));
                }
            }
        }
    }
    
    clauses
}
*/

/// Get the unused constant name for a dimension
pub fn get_unused_constant_name(dimension_index: usize) -> String {
    if dimension_index < DIMENSION_NAMES.len() {
        format!("{}_UNUSED", DIMENSION_NAMES[dimension_index].to_uppercase())
    } else {
        "UNUSED".to_string()
    }
}

/// Check if a dimension is complex (has multiple scale parameters)
pub fn is_complex_dimension(dimension_index: usize) -> bool {
    if dimension_index < ALL_DIMENSIONS.len() {
        if let Some(first_unit) = ALL_DIMENSIONS[dimension_index].first() {
            return first_unit.is_time_unit();
        }
    }
    false
}
