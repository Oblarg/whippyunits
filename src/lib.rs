#![feature(generic_const_exprs)]
#![feature(trait_alias)]

// ============================================================================
// Core Types and Enums
// ============================================================================

pub trait IsI8<const S: i8> {}
impl<const S: i8> IsI8<S> for () {}

#[macro_use]
pub mod unit_macro;
// #[macro_use]
// pub mod arithmetic_quantity_types;
#[macro_use]
pub mod generated_arithmetic_quantity_types;
pub mod default_declarators;
pub mod scoped_preferences;
pub mod constants;
#[macro_use]
pub mod scale_conversion;
// #[macro_use]
// pub mod quantity_type;
#[macro_use]
pub mod generated_quantity_type;
#[macro_use]
pub mod print;
#[macro_use]
pub mod arithmetic;
pub mod dimension_traits;
#[macro_use]
pub mod scale_resolution;
// #[macro_use]
// pub mod api;
#[macro_use]
pub mod generated_api;

// Re-export macros that need to be available at crate root
pub use scoped_preferences::*;

// Re-export the proc macro for consumers to use
pub use whippyunits_proc_macros::define_generic_dimension;


