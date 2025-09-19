#![feature(generic_const_exprs)]
#![feature(trait_alias)]

// ============================================================================
// Core Types and Enums
// ============================================================================

pub trait IsI16<const S: i16> {}
impl<const S: i16> IsI16<S> for () {}

// #[macro_use]
// pub mod arithmetic_quantity_types;
pub mod arithmetic_quantity_types;
pub mod default_declarators;
pub mod scoped_preferences;
pub mod scale_conversion;
// #[macro_use]
// pub mod quantity_type;
pub mod quantity_type;
pub mod print;
pub mod arithmetic;
pub mod dimension_traits;
// #[macro_use]
// pub mod api;
pub mod api;

// Re-export macros that need to be available at crate root
pub use scoped_preferences::*;

// Re-export the proc macros for consumers to use
pub use whippyunits_proc_macros::define_generic_dimension;
pub use whippyunits_proc_macros::proc_unit as unit;


