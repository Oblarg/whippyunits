use crate::alloc::String;
use whippyunits_core::{
    dimension_exponents::DynDimensionExponents, scale_exponents::ScaleExponents,
    storage_unit::generate_unit_literal as core_generate_unit_literal,
};

// Re-export for external use
pub use whippyunits_core::storage_unit::UnitLiteralConfig;

/// Generate the best unit literal for a given set of dimensions and scales
/// This delegates to the core implementation to ensure consistency
pub fn generate_unit_literal(
    exponents: DynDimensionExponents,
    scale_factors: ScaleExponents,
    config: UnitLiteralConfig,
) -> String {
    // Use the core implementation to ensure consistency between proc macro and inlay hints
    core_generate_unit_literal(exponents, scale_factors, config)
}
