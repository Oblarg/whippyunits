use crate::print::format_specifiers::{format_with_unit, UnitFormatSpecifier};
use crate::quantity_type::Quantity;
use crate::api::aggregate_scale_factor_float;
use whippyunits_default_dimensions::lookup_unit_literal;

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
    const SCALE_P10: i16,
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
    SCALE_P10,
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
        
        // Calculate conversion factor using existing system
        let conversion_factor = aggregate_scale_factor_float(
            SCALE_P2, SCALE_P3, SCALE_P5, SCALE_P10, SCALE_PI,
            target_unit_info.scale_factors.0, target_unit_info.scale_factors.1, 
            target_unit_info.scale_factors.2, target_unit_info.scale_factors.3, 
            target_unit_info.scale_factors.4
        );
        
        // Convert and format
        let converted_value = self.value.into() * conversion_factor;
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
        
        // Calculate conversion factor using existing system
        let conversion_factor = aggregate_scale_factor_float(
            SCALE_P2, SCALE_P3, SCALE_P5, SCALE_P10, SCALE_PI,
            target_unit_info.scale_factors.0, target_unit_info.scale_factors.1, 
            target_unit_info.scale_factors.2, target_unit_info.scale_factors.3, 
            target_unit_info.scale_factors.4
        );
        
        // Convert and format
        let converted_value = self.value.into() * conversion_factor;
        format_with_unit(converted_value, &spec)
    }
}
