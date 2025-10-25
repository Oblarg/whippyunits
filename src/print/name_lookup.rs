use whippyunits_core::{
     scale_exponents::ScaleExponents,
    storage_unit::{
        generate_systematic_composite_unit_name,
        generate_systematic_unit_name as core_generate_systematic_unit_name,
        generate_systematic_unit_name_with_scale_factors as core_generate_systematic_unit_name_with_scale_factors,
        lookup_dimension_name as core_lookup_dimension_name,
        DimensionNames as CoreDimensionNames,
    },
};

/// Generate systematic unit name with scale factors
/// This version can look up unit literals by their scale factors
pub fn generate_systematic_unit_name_with_scale_factors(
    exponents: Vec<i16>,
    scale_factors: ScaleExponents,
    long_name: bool,
) -> String {
    // Delegate to the core implementation
    core_generate_systematic_unit_name_with_scale_factors(exponents, scale_factors, long_name)
}

pub fn generate_systematic_unit_name(exponents: Vec<i16>, long_name: bool) -> String {
    // Delegate to the core implementation
    core_generate_systematic_unit_name(exponents, long_name)
}

pub fn generate_systematic_unit_name_with_format(
    exponents: Vec<i16>,
    long_name: bool,
    format: crate::print::prettyprint::UnitFormat,
) -> String {
    // Convert Vec<i16> to DynDimensionExponents for the core function
    if exponents.len() != 8 {
        return "?".to_string();
    }
    
    let dimension_exponents = whippyunits_core::dimension_exponents::DynDimensionExponents([
        exponents[0], exponents[1], exponents[2], exponents[3],
        exponents[4], exponents[5], exponents[6], exponents[7],
    ]);
    
    // Use the centralized logic from whippyunits-core
    let base_result = generate_systematic_composite_unit_name(dimension_exponents, long_name);
    
    // Apply format-specific transformations
    match format {
        crate::print::prettyprint::UnitFormat::Unicode => base_result,
        crate::print::prettyprint::UnitFormat::Ucum => {
            // Convert Unicode format to UCUM format
            convert_unicode_to_ucum_format(&base_result)
        }
    }
}

/// Convert Unicode format unit string to UCUM format
fn convert_unicode_to_ucum_format(unicode_unit: &str) -> String {
    // This is a simplified conversion - in practice, you might need more sophisticated logic
    // For now, just return the unicode format as-is since the core logic already handles
    // the basic formatting correctly
    unicode_unit.to_string()
}

pub type DimensionNames = CoreDimensionNames;

pub fn lookup_dimension_name(exponents: Vec<i16>) -> Option<DimensionNames> {
    // Delegate to the core implementation
    core_lookup_dimension_name(exponents)
}
