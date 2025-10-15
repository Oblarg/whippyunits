#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]
#![cfg_attr(feature = "nightly", allow(incomplete_features))]

//! Default dimension data for whippyunits
//!
//! This crate provides canonical dimension data that is shared between
//! the main whippyunits library and the proc macro crate without circular dependencies.

#[cfg(not(test))]
extern crate alloc;

mod dimensions;
pub mod dimension_exponents;
pub mod scale_exponents;
pub mod num;
mod prefix;
mod units;

pub use dimensions::*;
pub use prefix::*;
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
