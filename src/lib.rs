#![feature(custom_inner_attributes)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(trait_alias)]
#![rustfmt::skip]

use std::format;
use std::string::{String, ToString};
use std::vec::Vec;
use std::f64;
use std::marker::ConstParamTy;
use std::ops::{Add, Div, Mul, Sub};
use crate::constants::*;

// ============================================================================
// Core Types and Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum RescaleBehavior {
    SmallerWins,
    LeftHandWins,
    Strict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum CancelledScaleBehavior {
    Retain, // Keep the storage scales even when dimensions are cancelled
    Forget, // Automatically convert to Unused when exponent becomes 0
}

pub trait IsIsize<const S: isize> {}
impl<const S: isize> IsIsize<S> for () {}


#[macro_use]
pub mod unit_macro;
pub mod default_declarators;
pub mod scoped_preferences;
pub mod constants;
pub mod scale_conversion;
pub mod quantity_type;
pub mod print;
pub mod arithmetic;
pub mod dimension_traits;
pub mod scale_resolution;

// Re-export the proc macro
pub use whippyunits_unit_macro::proc_unit;

// Re-export macros that need to be available at crate root
pub use scoped_preferences::*;


