#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]
#![cfg_attr(feature = "nightly", allow(incomplete_features))]

//! Default dimension data for whippyunits
//!
//! This crate provides canonical dimension data that is shared between
//! the main whippyunits library and the proc macro crate without circular dependencies.

#[cfg(not(test))]
extern crate alloc;

#[cfg(not(test))]
use alloc::format;
#[cfg(not(test))]
use alloc::string::String;
#[cfg(not(test))]
use alloc::string::ToString;

pub mod dimension_exponents;
mod dimensions;
pub mod num;
pub mod parser;
mod prefix;
pub mod scale_exponents;
pub mod storage_unit;
mod units;

pub use dimensions::*;
pub use parser::*;
pub use prefix::*;
pub use storage_unit::*;
pub use units::*;

pub struct CapitalizedFmt<'r>(pub &'r str);

impl core::fmt::Display for CapitalizedFmt<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut chars = self.0.chars();

        if let Some(first) = chars.next() {
            write!(f, "{}", first.to_uppercase())?;
        }

        write!(f, "{}", chars.as_str())
    }
}

/// Convert a singular unit name to its plural form.
pub fn make_plural(singular: &str) -> String {
    // Handle exceptions to the "add s" rule
    match singular {
        "inch" => "inches".to_string(),
        "foot" => "feet".to_string(),
        "henry" => "henries".to_string(),
        "stone" => "stone".to_string(),
        "lux" => "lux".to_string(),
        "candela" => "candela".to_string(),
        "fahrenheit" => "fahrenheit".to_string(),
        "rankine" => "rankine".to_string(),
        "psi" => "psi".to_string(),
        "horsepower" => "horsepower".to_string(),
        "torr" => "torr".to_string(),
        "bar" => "bar".to_string(),
        "celsius" => "celsius".to_string(),
        "kelvin" => "kelvin".to_string(),
        _ => {
            // Default: just add 's' (works for 99% of unit names)
            format!("{}s", singular)
        }
    }
}

/// Generate the declarator trait name for a unit based on its system, dimension, and properties.
///
/// This function generates trait names like:
/// - `MetricLength` (storage metric unit)
/// - `MetricLengthNonStorage` (nonstorage metric unit)
/// - `MetricLengthAffine` (affine metric unit)
/// - `MetricLengthNonStorageAffine` (nonstorage affine metric unit)
/// - `ImperialLength` (imperial unit - already nonstorage by nature)
/// - `ImperialLengthAffine` (imperial affine unit)
///
/// Note: Imperial and Astronomical units are already nonstorage by nature, so they don't
/// get a "NonStorage" suffix. Only Metric units get the suffix.
///
/// # Arguments
///
/// * `system` - The unit system (Metric, Imperial, Astronomical)
/// * `dimension_name` - The name of the dimension (e.g., "Length", "Mass")
/// * `conversion_factor` - The conversion factor (1.0 for storage units)
/// * `affine_offset` - The affine offset (0.0 for non-affine units)
///
/// # Returns
///
/// The full trait name as a String
pub fn generate_declarator_trait_name(
    system: System,
    dimension_name: &str,
    conversion_factor: f64,
    affine_offset: f64,
) -> String {
    let sanitized_dimension = dimension_name.replace(" ", "");
    let capitalized_dimension = CapitalizedFmt(&sanitized_dimension).to_string();
    
    if system == System::Metric {
        let base_trait_name = format!("Metric{}", capitalized_dimension);
        
        // Add suffix based on unit type for metric units
        if conversion_factor != 1.0 && affine_offset != 0.0 {
            format!("{}NonStorageAffine", base_trait_name)
        } else if conversion_factor != 1.0 {
            format!("{}NonStorage", base_trait_name)
        } else if affine_offset != 0.0 {
            format!("{}Affine", base_trait_name)
        } else {
            // Pure storage metric unit
            base_trait_name
        }
    } else {
        // For Imperial/Astronomical: base trait name (they're already nonstorage)
        // Only add Affine suffix if needed
        let base_trait_name = format!("{}{}", system.as_str(), capitalized_dimension);
        if affine_offset != 0.0 {
            format!("{}Affine", base_trait_name)
        } else {
            base_trait_name
        }
    }
}

/// Convert any integer to Unicode superscript notation
/// Returns empty string for unity exponent (1) unless show_unity is true
/// Returns "ˀ" for unknown values (i16::MIN)
pub fn to_unicode_superscript(num: i16, show_unity: bool) -> String {
    if num == i16::MIN {
        return "ˀ".to_string();
    }

    if num == 1 && !show_unity {
        return String::new();
    }

    num.to_string()
        .replace('-', "⁻")
        .replace('0', "⁰")
        .replace('1', "¹")
        .replace('2', "²")
        .replace('3', "³")
        .replace('4', "⁴")
        .replace('5', "⁵")
        .replace('6', "⁶")
        .replace('7', "⁷")
        .replace('8', "⁸")
        .replace('9', "⁹")
}
