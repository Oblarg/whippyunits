// ============================================================================
// Shared Generator Utilities
// ============================================================================
// This module contains utilities shared across all generators

// ============================================================================
// Import data sources from parent module
// ============================================================================

use crate::unit_data::{UnitMetadata, LENGTH_UNITS, MASS_UNITS, TIME_UNITS, ALL_DIMENSIONS, DIMENSION_NAMES};

// ============================================================================
// Shared Utility Functions
// ============================================================================

/// Generate const generic parameters for the Quantity type
/// This ensures all generators use the same parameter structure
pub fn generate_const_generic_params() -> String {
    let mut params = String::new();
    
    for (dim_index, units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        
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

/// Generate const generic parameters with a suffix (for arithmetic operations)
pub fn generate_const_generic_params_with_suffix(suffix: &str) -> String {
    let mut params = String::new();
    
    for (dim_index, units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        
        if let Some(first_unit) = units.first() {
            if first_unit.is_time_unit() {
                // Complex dimension - needs P2, P3, P5, SCALE_ORDER
                params.push_str(&format!("    const {}_EXPONENT{}: isize, const {}_P2{}: isize, const {}_P3{}: isize, const {}_P5{}: isize, const {}_SCALE_ORDER{}: isize,\n",
                    dimension_name, suffix, dimension_name, suffix, dimension_name, suffix, dimension_name, suffix, dimension_name, suffix));
            } else {
                // Simple dimension - just scale
                params.push_str(&format!("    const {}_EXPONENT{}: isize, const {}_SCALE{}: isize,\n",
                    dimension_name, suffix, dimension_name, suffix));
            }
        }
    }
    
    params
}

/// Generate Quantity type parameters with a suffix (for arithmetic operations)
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

/// Generate where clauses for exponent arithmetic operations
pub fn generate_where_clauses_for_exponent_arithmetic(log_op: &str) -> String {
    let mut clauses = String::new();
    
    for (dim_index, _units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        clauses.push_str(&format!("        (): IsIsize<{{ {}_EXPONENT1 {} {}_EXPONENT2 }}>,\n", 
            dimension_name, log_op, dimension_name));
    }
    
    clauses
}

/// Generate scale where clauses for time units
pub fn generate_scale_where_clauses(scale_order: &str) -> String {
    let mut clauses = String::new();
    
    for (dim_index, units) in ALL_DIMENSIONS.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        
        if let Some(first_unit) = units.first() {
            if first_unit.is_time_unit() {
                clauses.push_str(&format!("        (): IsIsize<{{ {}_scale_2({}) }}>,\n", dimension_name.to_lowercase(), scale_order));
                clauses.push_str(&format!("        (): IsIsize<{{ {}_scale_3({}) }}>,\n", dimension_name.to_lowercase(), scale_order));
                clauses.push_str(&format!("        (): IsIsize<{{ {}_scale_5({}) }}>,\n", dimension_name.to_lowercase(), scale_order));
            }
        }
    }
    
    clauses
}



/// Get unused constant name for a dimension
pub fn get_unused_constant_name(dimension_index: usize) -> String {
    if dimension_index < DIMENSION_NAMES.len() {
        format!("{}_UNUSED", DIMENSION_NAMES[dimension_index].to_uppercase())
    } else {
        "UNUSED".to_string()
    }
}

/// Check if a dimension is complex (time-like)
pub fn is_complex_dimension(dimension_index: usize) -> bool {
    if dimension_index < ALL_DIMENSIONS.len() {
        if let Some(first_unit) = ALL_DIMENSIONS[dimension_index].first() {
            first_unit.is_time_unit()
        } else {
            false
        }
    } else {
        false
    }
}

/// Match a dimension from a unit vector (array of atomic exponents)
/// Returns (dimension_name, unit_name) if a match is found, None otherwise
/// This function is called at runtime during code generation
/// Works directly with the source of truth in unit_data.rs
pub fn match_dimension_from_atomic_exponents(
    unit_vector: &[isize],
    use_long_names: bool,
) -> Option<(&'static str, &'static str)> {
    // Check if the unit vector matches any atomic dimension from unit_data
    // The unit vector should have the same length as the number of atomic dimensions
    if unit_vector.len() != ALL_DIMENSIONS.len() {
        return None;
    }
    
    // Check if this unit vector represents a pure atomic dimension
    // (i.e., only one non-zero exponent)
    let mut non_zero_count = 0;
    let mut non_zero_index = 0;
    
    for (dim_index, &exponent) in unit_vector.iter().enumerate() {
        if exponent != 0 {
            non_zero_count += 1;
            non_zero_index = dim_index;
        }
    }
    
    // If exactly one non-zero exponent, it's an atomic dimension
    if non_zero_count == 1 {
        let dimension_name = DIMENSION_NAMES[non_zero_index];
        let unit_name = ""; // Will be generated based on the dimension
        
        return Some((dimension_name, unit_name));
    }
    
    // If multiple non-zero exponents, it's a composite dimension
    // Check against DIMENSIONAL_DATA for composite matches
    for dimension in DIMENSIONAL_DATA {
        // Check if the unit vector matches this composite dimension's atomic exponents
        let mut matches = true;
        for (dim_index, &exponent) in unit_vector.iter().enumerate() {
            let expected_exponent = dimension.exponents.get_by_index(dim_index);
            if exponent != expected_exponent {
                matches = false;
                break;
            }
        }
        
        if matches {
            // Found a match! Get the dimension name
            let dimension_name = dimension.names[0];
            let unit_name = ""; // Will be generated based on the dimension
            
            return Some((dimension_name, unit_name));
        }
    }
    
    None // No match found
}


