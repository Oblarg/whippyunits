#![feature(generic_const_exprs)]
#![feature(trait_alias)]

// ============================================================================
// Core Types and Enums
// ============================================================================

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
pub mod int_conversion;

// Re-export macros that need to be available at crate root
pub use scoped_preferences::*;

// Re-export the proc macro for consumers to use
pub use whippyunits_proc_macros::define_generic_dimension;


