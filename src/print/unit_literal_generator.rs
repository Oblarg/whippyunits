use crate::print::name_lookup::lookup_dimension_name;
use crate::print::name_lookup::generate_systematic_unit_name;
use crate::print::name_lookup::generate_systematic_unit_name_with_scale_factors;
use crate::print::prettyprint::{generate_prefixed_systematic_unit, generate_prefixed_si_unit};

/// Configuration for unit literal generation
#[derive(Debug, Clone, Copy)]
pub struct UnitLiteralConfig {
    pub verbose: bool,
    pub prefer_si_units: bool,
}

impl Default for UnitLiteralConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            prefer_si_units: true,
        }
    }
}

/// Generate the best unit literal for a given set of dimensions and scales
/// This centralizes the logic shared between pretty_print_quantity and pretty_print_quantity_inlay_hint
pub fn generate_unit_literal(
    exponents: Vec<i16>,
    scale_p2: i16,
    scale_p3: i16,
    scale_p5: i16,
    scale_pi: i16,
    config: UnitLiteralConfig,
) -> String {
    // Generate systematic unit literal (base unit without prefix)
    let base_systematic_literal = generate_systematic_unit_name_with_scale_factors(
        exponents.clone(),
        (scale_p2, scale_p3, scale_p5, scale_pi),
        config.verbose,
    );

    // Check if we found a unit literal match - if so, use it directly without conversion factor
    let systematic_literal = if base_systematic_literal != generate_systematic_unit_name(exponents.clone(), config.verbose) {
        // We found a unit literal match, use it directly
        base_systematic_literal
    } else {
        // No unit literal match, apply SI prefix to the systematic unit literal
        generate_prefixed_systematic_unit(
            exponents.clone(),
            scale_p2, scale_p3, scale_p5, scale_pi,
            &base_systematic_literal,
            config.verbose,
        )
    };

    // If we don't prefer SI units, return the systematic literal
    if !config.prefer_si_units {
        return systematic_literal;
    }

    // Check if we have a recognized dimension with a specific SI unit
    if let Some(info) = lookup_dimension_name(exponents) {
        if let Some(si_shortname) = if config.verbose {
            info.unit_si_shortname
        } else {
            info.unit_si_shortname_symbol
        } {
            // Apply SI prefix to the specific SI unit name
            let prefixed_si_unit = generate_prefixed_si_unit(
                scale_p2, scale_p3, scale_p5, scale_pi,
                si_shortname,
                config.verbose,
            );

            // Return the prefixed SI unit if it's different from the systematic literal
            if prefixed_si_unit != systematic_literal {
                prefixed_si_unit
            } else {
                systematic_literal
            }
        } else {
            // No specific SI unit defined, use the systematic literal
            systematic_literal
        }
    } else {
        // Unknown dimension, use the systematic literal
        systematic_literal
    }
}
