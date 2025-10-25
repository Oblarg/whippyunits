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
mod prefix;
pub mod scale_exponents;
pub mod storage_unit;
mod units;

pub use dimensions::*;
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
