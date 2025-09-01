use std::fs;
use std::collections::HashMap;
use std::string::ToString;
use std::vec::Vec;

// ============================================================================
// Import data sources and shared utilities
// ============================================================================

// Include the data files directly
include!("unit_data.rs");
include!("dimension_data.rs");

use unit_data::{UnitMetadata, LENGTH_UNITS, MASS_UNITS, TIME_UNITS, ALL_DIMENSIONS, DIMENSION_NAMES};
use dimension_data::{DimensionalExponents, DimensionType, DIMENSIONAL_DATA, get_canonical_name};

// ============================================================================
// Dimension Generation
// ============================================================================

fn generate_trait_definition(dimension: &DimensionType) -> String {
    let canonical_name = get_canonical_name(dimension);
    let mut output = String::new();
    
    output.push_str(&format!("pub trait {} {{\n", canonical_name));
    output.push_str("    type Unit;\n");
    output.push_str("    \n");
    output.push_str("    /// Get the unit name for this dimension\n");
    
    // Generate variadic trait signature based on dimension requirements
    if is_atomic_dimension(dimension) {
        // Atomic dimensions use single scale parameter
        output.push_str("    fn unit_name(scale: isize, use_long_names: bool) -> &'static str;\n");
        output.push_str("    \n");
        output.push_str("    /// Get the short unit name\n");
        output.push_str("    fn short_name(scale: isize) -> &'static str {\n");
        output.push_str("        Self::unit_name(scale, false)\n");
        output.push_str("    }\n");
        output.push_str("    \n");
        output.push_str("    /// Get the long unit name\n");
        output.push_str("    fn long_name(scale: isize) -> &'static str {\n");
        output.push_str("        Self::unit_name(scale, true)\n");
        output.push_str("    }\n");
    } else {
        // Composite dimensions use variadic const generic parameters
        let const_params = generate_const_generic_params_for_dimension(dimension);
        output.push_str("    fn unit_name<\n");
        output.push_str(&format!("        {}\n", const_params));
        output.push_str("    >(use_long_names: bool) -> &'static str;\n");
        output.push_str("    \n");
        output.push_str("    /// Get the short unit name\n");
        output.push_str("    fn short_name<\n");
        output.push_str(&format!("        {}\n", const_params));
        output.push_str("    >() -> &'static str {\n");
        output.push_str("        Self::unit_name::<\n");
        output.push_str(&format!("            {}\n", generate_const_generic_arguments_for_dimension(dimension)));
        output.push_str("        >(false)\n");
        output.push_str("    }\n");
        output.push_str("    \n");
        output.push_str("    /// Get the long unit name\n");
        output.push_str("    fn long_name<\n");
        output.push_str(&format!("        {}\n", const_params));
        output.push_str("    >() -> &'static str {\n");
        output.push_str("        Self::unit_name::<\n");
        output.push_str(&format!("            {}\n", generate_const_generic_arguments_for_dimension(dimension)));
        output.push_str("        >(true)\n");
        output.push_str("    }\n");
    }
    
    output.push_str("}\n\n");
    
    output
}

fn generate_trait_implementation(dimension: &DimensionType) -> String {
    let canonical_name = get_canonical_name(dimension);
    let mut output = String::new();
    
    // Generate the impl block with const generic parameters
    output.push_str("#[rustfmt::skip]\n");
    output.push_str("impl<\n");
    
    // Add const generic parameters based on which dimensions are used
    let mut params = Vec::new();
    
    // Check each atomic dimension from unit_data
    for (dim_index, _dimension_name) in DIMENSION_NAMES.iter().enumerate() {
        let exponent = dimension.exponents.get_by_index(dim_index);
        if exponent != 0 {
            let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
            let param_name = if dim_index == 2 {
                "TIME_SCALE_ORDER" // Special case for time dimension
            } else {
                &format!("{}_SCALE", dimension_name)
            };
            params.push(format!("    const {}: isize", param_name));
        }
    }
    
    if !params.is_empty() {
        output.push_str(&params.join(",\n"));
        output.push_str(">\n");
    } else {
        output.push_str(">\n");
    }
    
    output.push_str(canonical_name);
    output.push_str("\n");
    
    // Generate the Quantity type reference
    output.push_str("for ");
    output.push_str(&generate_quantity_type_reference(&dimension.exponents));
    output.push_str(" {\n");
    output.push_str("    type Unit = Self;\n");
    output.push_str("    \n");
    output.push_str(&generate_unit_name_implementation(dimension));
    output.push_str("}\n\n");
    
    output
}

fn generate_quantity_type_reference(exponents: &DimensionalExponents) -> String {
    let mut output = String::new();
    output.push_str("Quantity<\n");
    
    // Generate parameters for each atomic dimension
    for (dim_index, _dimension_name) in DIMENSION_NAMES.iter().enumerate() {
        let exponent = exponents.get_by_index(dim_index);
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        
        if exponent == 0 {
            // Zero exponent - use unused constant
            let unused_constant = format!("{}_UNUSED", dimension_name);
            if dim_index == 2 {
                // Time dimension has multiple unused constants
                output.push_str(&format!("    0, {}, {}, {}, {}>", unused_constant, unused_constant, unused_constant, unused_constant));
            } else {
                output.push_str(&format!("    0, {}, \n", unused_constant));
            }
        } else {
            // Non-zero exponent - use scale parameter
            if dim_index == 2 {
                // Time dimension - complex with P2, P3, P5
                output.push_str(&format!("    {}, {{ time_scale_2(TIME_SCALE_ORDER) }}, {{ time_scale_3(TIME_SCALE_ORDER) }}, {{ time_scale_5(TIME_SCALE_ORDER) }}, TIME_SCALE_ORDER>", exponent));
            } else {
                let scale_param = format!("{}_SCALE", dimension_name);
                output.push_str(&format!("    {}, {}, \n", exponent, scale_param));
            }
        }
    }
    
    output
}

fn generate_unit_name_implementation(dimension: &DimensionType) -> String {
    let mut output = String::new();
    
    if is_atomic_dimension(dimension) {
        // Atomic dimensions use single scale parameter
        let units = get_units_for_atomic_dimension(dimension);
        
        output.push_str("    fn unit_name(scale: isize, use_long_names: bool) -> &'static str {\n");
        output.push_str("        match scale {\n");
        
        // Add cases for each unit in this atomic dimension
        for unit in units {
            let short_name = unit.short_name;
            let long_name = unit.long_name;
            let scale = unit.scale_order;
            
            output.push_str(&format!("            {} => {{\n", scale));
            output.push_str("                if use_long_names {\n");
            output.push_str(&format!("                    \"{}\"\n", long_name));
            output.push_str("                } else {\n");
            output.push_str(&format!("                    \"{}\"\n", short_name));
            output.push_str("                }\n");
            output.push_str("            }\n");
        }
        
        // Handle unused scale
        let unused_constant = get_unused_constant_name_for_dimension(dimension);
        output.push_str(&format!("            {} => \"\", // unused\n", unused_constant));
        output.push_str("            _ => \"unknown\",\n");
        output.push_str("        }\n");
        output.push_str("    }\n");
    } else {
        // Composite dimensions use variadic const generic parameters
        let const_params = generate_const_generic_params_for_dimension(dimension);
        output.push_str(&format!("    fn unit_name<{}>(use_long_names: bool) -> &'static str {{\n", const_params));
        output.push_str("        // TODO: Implement composite unit name resolution based on scale combinations\n");
        output.push_str("        // For now, return empty string - no unit names defined for composite dimensions yet\n");
        output.push_str("        \"\"\n");
        output.push_str("    }\n");
    }
    
    output
}

fn is_atomic_dimension(dimension: &DimensionType) -> bool {
    // Atomic dimensions are those with only one non-zero exponent
    let mut non_zero_count = 0;
    
    for dim_index in 0..DIMENSION_NAMES.len() {
        let exponent = dimension.exponents.get_by_index(dim_index);
        if exponent != 0 {
            non_zero_count += 1;
        }
    }
    
    non_zero_count == 1
}

fn get_units_for_atomic_dimension(dimension: &DimensionType) -> Vec<&'static UnitMetadata> {
    // Only atomic dimensions have units defined in unit_data.rs
    // Check which atomic dimension this is by finding the non-zero exponent
    for (dim_index, _dimension_name) in DIMENSION_NAMES.iter().enumerate() {
        let exponent = dimension.exponents.get_by_index(dim_index);
        if exponent != 0 {
            // Found the atomic dimension - return its units
            return match dim_index {
                0 => LENGTH_UNITS.iter().collect(), // Length-like dimension
                1 => MASS_UNITS.iter().collect(),   // Mass-like dimension
                2 => TIME_UNITS.iter().collect(),   // Time-like dimension
                _ => vec![], // Unknown dimension
            };
        }
    }
    
    // No non-zero exponents found - composite dimension
    vec![]
}

fn generate_const_generic_params_for_dimension(dimension: &DimensionType) -> String {
    let mut params = Vec::new();
    
    // Check each atomic dimension from unit_data
    for (dim_index, _dimension_name) in DIMENSION_NAMES.iter().enumerate() {
        let exponent = dimension.exponents.get_by_index(dim_index);
        if exponent != 0 {
            let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
            let param_name = if dim_index == 2 {
                "TIME_SCALE_ORDER" // Special case for time dimension
            } else {
                &format!("{}_SCALE", dimension_name)
            };
            params.push(format!("const {}: isize", param_name));
        }
    }
    
    params.join(",\n        ")
}

fn generate_const_generic_arguments_for_dimension(dimension: &DimensionType) -> String {
    let mut args = Vec::new();
    
    // Check each atomic dimension from unit_data
    for (dim_index, _dimension_name) in DIMENSION_NAMES.iter().enumerate() {
        let exponent = dimension.exponents.get_by_index(dim_index);
        if exponent != 0 {
            let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
            let arg_name = if dim_index == 2 {
                "TIME_SCALE_ORDER" // Special case for time dimension
            } else {
                &format!("{}_SCALE", dimension_name)
            };
            args.push(arg_name.to_string());
        }
    }
    
    args.join(",\n            ")
}



fn get_unused_constant_name_for_dimension(dimension: &DimensionType) -> String {
    // Determine which dimension this is based on its exponents
    // Check which atomic dimension this is by finding the non-zero exponent
    for (dim_index, _dimension_name) in DIMENSION_NAMES.iter().enumerate() {
        let exponent = dimension.exponents.get_by_index(dim_index);
        if exponent != 0 {
            // Found the atomic dimension - return its unused constant name
            // The pattern is {DIMENSION_NAME}_UNUSED
            let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
            return format!("{}_UNUSED", dimension_name);
        }
    }
    
    // No non-zero exponents found - complex dimension
    "isize::MAX".to_string()
}

fn generate_trait_aliases(dimension: &DimensionType) -> String {
    let mut output = String::new();
    let canonical_name = get_canonical_name(dimension);
    
    // Generate aliases for all names except the canonical one
    for alias in dimension.names.iter().skip(1) {
        output.push_str(&format!("pub trait {} = {};\n", alias, canonical_name));
    }
    
    if dimension.names.len() > 1 {
        output.push_str("\n");
    }
    
    output
}

fn generate_section_header(title: &str) -> String {
    let mut output = String::new();
    output.push_str("// ===========================================================================\n");
    output.push_str(&format!("// {}\n", title));
    output.push_str("// ===========================================================================\n\n");
    output
}

fn generate_dimensions_rs() -> String {
    let mut output = String::new();
    
    // Header
    output.push_str("use crate::{\n");
    output.push_str("    time_scale_2, time_scale_3, time_scale_5, IsIsize, Quantity,\n");
    
    // Generate unused constant imports dynamically
    for (dim_index, _dimension_name) in DIMENSION_NAMES.iter().enumerate() {
        let dimension_name = DIMENSION_NAMES[dim_index].to_uppercase();
        let unused_constant = format!("{}_UNUSED", dimension_name);
        if dim_index == DIMENSION_NAMES.len() - 1 {
            output.push_str(&format!("    {},\n", unused_constant));
        } else {
            output.push_str(&format!("    {}, ", unused_constant));
        }
    }
    output.push_str("};\n\n");
    
    // Group dimensions by category - use a simpler approach to avoid duplicates
    let mut categories = Vec::new();
    
    // First, categorize all dimensions by their structure
    let mut atomic_dimensions = Vec::new();
    let mut composite_dimensions = Vec::new();
    
    for dimension in DIMENSIONAL_DATA {
        let mut non_zero_count = 0;
        let mut non_zero_indices = Vec::new();
        
        for (check_dim_index, _) in DIMENSION_NAMES.iter().enumerate() {
            let exponent = dimension.exponents.get_by_index(check_dim_index);
            if exponent != 0 {
                non_zero_count += 1;
                non_zero_indices.push(check_dim_index);
            }
        }
        
        if non_zero_count == 1 {
            // Atomic dimension - categorize by its single non-zero dimension
            atomic_dimensions.push((non_zero_indices[0], dimension));
        } else if non_zero_count > 1 {
            // Composite dimension
            composite_dimensions.push(dimension);
        }
    }
    
    // Group atomic dimensions by their primary dimension
    let mut atomic_groups: std::collections::HashMap<usize, Vec<&DimensionType>> = std::collections::HashMap::new();
    for (dim_index, dimension) in atomic_dimensions {
        atomic_groups.entry(dim_index).or_insert_with(Vec::new).push(dimension);
    }
    
    // Create categories for atomic dimensions
    for (dim_index, dimensions) in atomic_groups {
        let dimension_name = DIMENSION_NAMES[dim_index];
        let category_name = format!("{}-like units", dimension_name.to_uppercase());
        categories.push((category_name.as_str(), dimensions));
    }
    
    // Add composite dimensions category
    if !composite_dimensions.is_empty() {
        categories.push(("Composite units", composite_dimensions));
    }
    
    // Generate each category
    for (category_name, dimensions) in categories {
        output.push_str(&generate_section_header(category_name));
        
        // Generate all trait definitions first
        for dimension in &dimensions {
            output.push_str(&generate_trait_definition(dimension));
        }
        
        // Generate all trait implementations
        for dimension in &dimensions {
            output.push_str(&generate_trait_implementation(dimension));
        }
        
        // Generate all trait aliases
        for dimension in &dimensions {
            output.push_str(&generate_trait_aliases(dimension));
        }
    }
    
    // Additional aliases section
    output.push_str(&generate_section_header("Additional aliases for existing dimensions"));
    
    // Find dimensions that have aliases and generate cross-references
    for dimension in DIMENSIONAL_DATA {
        if dimension.names.len() > 1 {
            let canonical = dimension.names.first().unwrap();
            for alias in dimension.names.iter().skip(1) {
                output.push_str(&format!("pub trait {} = {};\n", alias, canonical));
            }
        }
    }
    
    output
}

// ============================================================================
// Main Generation Function
// ============================================================================

fn main() {
    let generated_code = generate_dimensions_rs();
    
    // Write to src/dimensions.rs
    fs::write("src/dimensions.rs", generated_code).unwrap();
    
    println!("Generated dimensions.rs in src/dimensions.rs");
    println!("The file is ready to use!");
}
