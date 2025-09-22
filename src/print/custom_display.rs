use crate::print::format_specifiers::{format_with_unit, UnitFormatSpecifier};
use crate::quantity_type::Quantity;
use crate::api::aggregate_scale_factor_float;
use whippyunits_default_dimensions::lookup_unit_literal;

/// Calculate the conversion factor from the source unit to the target unit
fn calculate_conversion_factor<const SCALE_P2: i16, const SCALE_P3: i16, const SCALE_P5: i16, const SCALE_PI: i16>(
    unit: &str, 
    target_unit_info: &whippyunits_default_dimensions::UnitLiteralInfo
) -> f64 {
    // First try to parse as a prefixed unit (short names like "km", "cm", etc.)
    if let Some(prefix_info) = whippyunits_default_dimensions::lookup_si_prefix(
        &unit[..unit.len() - target_unit_info.symbol.len()]
    ) {
        // This is a prefixed unit - create the target scale factors from the base unit + prefix
        let prefix_scale = prefix_info.scale_factor;
        // Convert the target unit's scale factors to use only prime factors
        // The old system had scale_factors.3 as SCALE_P10, now we need to put that in p2 and p5
        // The prefix_scale is the power of 10, so we add it to both p2 and p5
        let (target_p2, target_p3, target_p5, target_pi) = (
            target_unit_info.scale_factors.0 + prefix_scale, // p2 gets prefix
            target_unit_info.scale_factors.1,                // p3 unchanged
            target_unit_info.scale_factors.2 + prefix_scale, // p5 gets prefix
            target_unit_info.scale_factors.3                 // pi unchanged
        );
        
        // Calculate conversion factor from source to target
        let result = aggregate_scale_factor_float(
            SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI,
            target_p2, target_p3, target_p5, target_pi
        );
        result
    } else {
        // Try to parse as a long name prefixed unit using existing data from default-dimensions
        for prefix in whippyunits_default_dimensions::SI_PREFIXES {
            for base_unit in whippyunits_default_dimensions::BASE_UNITS {
                // Check both singular and plural forms
                let base_singular = base_unit.long_name;
                let base_plural = base_unit.long_name.to_string() + "s";
                
                if unit.starts_with(prefix.long_name) && (unit.ends_with(base_singular) || unit.ends_with(&base_plural)) {
                    let expected_length_singular = prefix.long_name.len() + base_singular.len();
                    let expected_length_plural = prefix.long_name.len() + base_plural.len();
                    
                    if unit.len() == expected_length_singular || unit.len() == expected_length_plural {
                        // Found a long name prefixed unit - get the prefix scale factor
                        let prefix_scale = prefix.scale_factor;
                        let (target_p2, target_p3, target_p5, target_pi) = (
                            target_unit_info.scale_factors.0 + prefix_scale, // p2 gets prefix
                            target_unit_info.scale_factors.1,                // p3 unchanged
                            target_unit_info.scale_factors.2 + prefix_scale, // p5 gets prefix
                            target_unit_info.scale_factors.3                 // pi unchanged
                        );
                        
                        // Calculate conversion factor from source to target
                        let result = aggregate_scale_factor_float(
                            SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI,
                            target_p2, target_p3, target_p5, target_pi
                        );
                        return result;
                    }
                }
            }
        }
        
        // If not a prefixed unit, check if it has a conversion factor
        if let Some(unit_conversion_factor) = target_unit_info.conversion_factor {
            // This unit has a conversion factor (imperial units, time units, etc.)
            1.0 / unit_conversion_factor
        } else {
            // Use the scale factors from the target unit info
            let (p2, p3, p5, pi) = (
                target_unit_info.scale_factors.0, // p2
                target_unit_info.scale_factors.1, // p3
                target_unit_info.scale_factors.2, // p5
                target_unit_info.scale_factors.3  // pi
            );
            aggregate_scale_factor_float(
                SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI,
                p2, p3, p5, pi
            )
        }
    }
}

/// Extension trait to add custom formatting methods to Quantity types
pub trait QuantityFormatExt {
    fn format_as(&self, unit: &str) -> Result<String, String>;
    fn format_as_with_precision(&self, unit: &str, precision: usize) -> Result<String, String>;
}

impl<
    const MASS_EXPONENT: i16,
    const LENGTH_EXPONENT: i16,
    const TIME_EXPONENT: i16,
    const CURRENT_EXPONENT: i16,
    const TEMPERATURE_EXPONENT: i16,
    const AMOUNT_EXPONENT: i16,
    const LUMINOSITY_EXPONENT: i16,
    const ANGLE_EXPONENT: i16,
    const SCALE_P2: i16,
    const SCALE_P3: i16,
    const SCALE_P5: i16,
    const SCALE_PI: i16,
    T
> QuantityFormatExt for Quantity<
    MASS_EXPONENT,
    LENGTH_EXPONENT,
    TIME_EXPONENT,
    CURRENT_EXPONENT,
    TEMPERATURE_EXPONENT,
    AMOUNT_EXPONENT,
    LUMINOSITY_EXPONENT,
    ANGLE_EXPONENT,
    SCALE_P2,
    SCALE_P3,
    SCALE_P5,
    SCALE_PI,
    T
>
where
    T: Copy + Into<f64>,
{

    fn format_as(&self, unit: &str) -> Result<String, String> {
        let spec = UnitFormatSpecifier {
            target_unit: unit.to_string(),
            precision: None,
            width: None,
            alignment: None,
        };
        
        // Get target unit info from centralized data
        let target_unit_info = lookup_unit_literal(unit)
            .ok_or_else(|| format!("Unknown unit: {}", unit))?;
        
        // Check dimension compatibility
        let source_dims = (
            MASS_EXPONENT,
            LENGTH_EXPONENT,
            TIME_EXPONENT,
            CURRENT_EXPONENT,
            TEMPERATURE_EXPONENT,
            AMOUNT_EXPONENT,
            LUMINOSITY_EXPONENT,
            ANGLE_EXPONENT,
        );
        
        if source_dims != target_unit_info.dimension_exponents {
            return Err(format!("Dimension mismatch: cannot convert to {}", unit));
        }
        
        // Calculate conversion factor using the helper function
        let conversion_factor = calculate_conversion_factor::<SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI>(unit, target_unit_info);
        
        // Convert and format
        let original_value: f64 = self.value.into();
        let converted_value = original_value * conversion_factor;
        format_with_unit(converted_value, &spec)
    }
    
    fn format_as_with_precision(&self, unit: &str, precision: usize) -> Result<String, String> {
        let spec = UnitFormatSpecifier {
            target_unit: unit.to_string(),
            precision: Some(precision),
            width: None,
            alignment: None,
        };
        
        // Get target unit info from centralized data
        let target_unit_info = lookup_unit_literal(unit)
            .ok_or_else(|| format!("Unknown unit: {}", unit))?;
        
        // Check dimension compatibility
        let source_dims = (
            MASS_EXPONENT,
            LENGTH_EXPONENT,
            TIME_EXPONENT,
            CURRENT_EXPONENT,
            TEMPERATURE_EXPONENT,
            AMOUNT_EXPONENT,
            LUMINOSITY_EXPONENT,
            ANGLE_EXPONENT,
        );
        
        if source_dims != target_unit_info.dimension_exponents {
            return Err(format!("Dimension mismatch: cannot convert to {}", unit));
        }
        
        // Calculate conversion factor using the helper function
        let conversion_factor = calculate_conversion_factor::<SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI>(unit, target_unit_info);
        
        // Convert and format
        let converted_value = self.value.into() * conversion_factor;
        format_with_unit(converted_value, &spec)
    }
}
