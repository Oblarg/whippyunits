use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Ident;

use crate::shared_utils::{is_valid_identifier, generate_scale_name};

/// Input for the generate_default_declarators macro
/// Usage: generate_default_declarators!()
pub struct DefaultDeclaratorsInput;

impl Parse for DefaultDeclaratorsInput {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(DefaultDeclaratorsInput)
    }
}

impl DefaultDeclaratorsInput {
    pub fn expand(self) -> TokenStream {
        let mut expansions = Vec::new();

        // Generate metric declarators (with base units and prefixed sets)
        self.generate_metric_declarators(&mut expansions);

        // Generate non-metric declarators (Imperial, Astronomical, etc.)
        self.generate_nonmetric_declarators(&mut expansions);

        // Generate the literals module using the new approach
        let literals_module = self.generate_literals_module();
        
        quote! {
            #(#expansions)*
            
            // Automatically generate literals module for culit integration
            #literals_module
        }
    }

    /// Generate metric declarators for all dimensions
    fn generate_metric_declarators(&self, expansions: &mut Vec<TokenStream>) {
        use whippyunits_core::{Dimension, System};

        for dimension in Dimension::ALL {
            // Get all metric units for this dimension
            let metric_units: Vec<_> = dimension.units
                .iter()
                .filter(|unit| unit.system == System::Metric)
                .collect();

            if metric_units.is_empty() {
                continue;
            }

            // Generate a single trait for all metric units in this dimension
            self.generate_dimension_trait(expansions, dimension, &metric_units);
        }
    }

    /// Generate a single trait for all units in a dimension
    fn generate_dimension_trait(
        &self,
        expansions: &mut Vec<TokenStream>,
        dimension: &whippyunits_core::Dimension,
        metric_units: &[&whippyunits_core::Unit],
    ) {
        use whippyunits_core::SiPrefix;

        let (
            mass_exp,
            length_exp,
            time_exp,
            current_exp,
            temperature_exp,
            amount_exp,
            luminosity_exp,
            angle_exp,
        ) = (
            dimension.exponents.0[0], // mass
            dimension.exponents.0[1], // length
            dimension.exponents.0[2], // time
            dimension.exponents.0[3], // current
            dimension.exponents.0[4], // temperature
            dimension.exponents.0[5], // amount
            dimension.exponents.0[6], // luminous_intensity
            dimension.exponents.0[7], // angle
        );

        // Generate trait name from dimension name, sanitizing spaces
        let sanitized_name = dimension.name.replace(" ", "");
        let trait_name = format!("Metric{}", whippyunits_core::CapitalizedFmt(&sanitized_name).to_string());
        let trait_ident = syn::parse_str::<Ident>(&trait_name).unwrap();

        let mut scale_definitions = Vec::new();

        // Process each metric unit
        for (i, unit) in metric_units.iter().enumerate() {
            // Skip units with invalid identifier names
            if !is_valid_identifier(unit.name) {
                continue;
            }

            let is_base_unit = i == 0; // First unit is the base unit

            if is_base_unit {
                // Generate base unit with all SI prefixes
                let unit_suffix = whippyunits_core::make_plural(unit.name);
                
                // Generate the base unit (no prefix)
                let base_scale_name = generate_scale_name("", &unit_suffix);
                let base_fn_name = unit_suffix.to_string();

                // Calculate scale factors for base unit
                let (base_p2, base_p3, base_p5, base_pi) = if dimension.name == "Mass" {
                    // Gram has inherent -3 scale factor
                    (-3i16, 0i16, -3i16, 0i16)
                } else {
                    // Other units have 0 inherent scale factor
                    (0i16, 0i16, 0i16, 0i16)
                };

                let base_scale_name_ident = syn::parse_str::<Ident>(&base_scale_name).unwrap();
                let base_fn_name_ident = syn::parse_str::<Ident>(&base_fn_name).unwrap();

                scale_definitions.push(quote! {
                    (#base_scale_name_ident, #base_fn_name_ident, #base_p2, #base_p3, #base_p5, #base_pi)
                });

                // Generate all the prefixed units
                for prefix in SiPrefix::ALL {
                    let scale_name = generate_scale_name(prefix.name(), &unit_suffix);
                    let fn_name = format!("{}{}", prefix.name(), unit_suffix);

                    // Calculate scale factors
                    let (p2, p3, p5, pi) = if dimension.name == "Mass" {
                        // Gram has inherent -3 scale factor, so we add the prefix scale factor
                        let total_scale = -3 + prefix.factor_log10();
                        (total_scale as i16, 0i16, total_scale as i16, 0i16)
                    } else {
                        // Other units have 0 inherent scale factor
                        (
                            prefix.factor_log10() as i16,
                            0i16,
                            prefix.factor_log10() as i16,
                            0i16,
                        )
                    };

                    let scale_name_ident = syn::parse_str::<Ident>(&scale_name).unwrap();
                    let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

                    scale_definitions.push(quote! {
                        (#scale_name_ident, #fn_name_ident, #p2, #p3, #p5, #pi)
                    });
                }
            } else {
                // Non-base unit: classify as affine or non-affine
                if unit.affine_offset != 0.0 {
                    // This is an affine unit - handle separately
                    continue; // We'll handle affine units in a separate trait
                } else {
                    // This is a non-affine unit
                    let (p2, p3, p5, pi) = (
                        unit.scale.0[0],
                        unit.scale.0[1],
                        unit.scale.0[2],
                        unit.scale.0[3],
                    );

                    let type_name = whippyunits_core::CapitalizedFmt(unit.name).to_string();
                    let scale_name_ident = syn::parse_str::<Ident>(&type_name).unwrap();

                    let fn_name = whippyunits_core::make_plural(unit.name);
                    let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

                    scale_definitions.push(quote! {
                        (#scale_name_ident, #fn_name_ident, #p2, #p3, #p5, #pi)
                    });

                    // Generate prefixed versions of this compound unit
                    for prefix_info in SiPrefix::ALL {
                        let prefixed_p2 = p2 + prefix_info.factor_log10();
                        let prefixed_p3 = p3;
                        let prefixed_p5 = p5 + prefix_info.factor_log10();
                        let prefixed_pi = pi;

                        let unit_singular = unit.name.trim_end_matches('s');
                        let prefixed_type_name = format!("{}{}", prefix_info.name(), unit_singular);
                        let prefixed_type_name_capitalized =
                            whippyunits_core::CapitalizedFmt(&prefixed_type_name).to_string();
                        let prefixed_scale_name_ident =
                            syn::parse_str::<Ident>(&prefixed_type_name_capitalized).unwrap();

                        let prefixed_fn_name = whippyunits_core::make_plural(&format!(
                            "{}{}",
                            prefix_info.name(),
                            unit_singular
                        ));
                        let prefixed_fn_name_ident =
                            syn::parse_str::<Ident>(&prefixed_fn_name).unwrap();

                        scale_definitions.push(quote! {
                            (#prefixed_scale_name_ident, #prefixed_fn_name_ident, #prefixed_p2, #prefixed_p3, #prefixed_p5, #prefixed_pi)
                        });
                    }
                }
            }
        }

        // Generate the main trait for non-affine units
        if !scale_definitions.is_empty() {
            let expansion = quote! {
                define_quantity!(
                    #mass_exp,
                    #length_exp,
                    #time_exp,
                    #current_exp,
                    #temperature_exp,
                    #amount_exp,
                    #luminosity_exp,
                    #angle_exp,
                    #trait_ident,
                    #(#scale_definitions),*
                );
            };

            expansions.push(expansion);
        }

        // Handle affine units separately with "Affine" suffix
        let mut affine_units = Vec::new();
        for unit in metric_units {
            if unit.affine_offset != 0.0 {
                affine_units.push(unit);
            }
        }

        if !affine_units.is_empty() {
            let mut affine_scale_definitions = Vec::new();

            for unit in &affine_units {
                let fn_name = whippyunits_core::make_plural(unit.name);
                let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

                let conversion_factor = unit.conversion_factor;
                let affine_offset = unit.affine_offset;

                let (p2, p3, p5, pi) = (
                    unit.scale.0[0],
                    unit.scale.0[1],
                    unit.scale.0[2],
                    unit.scale.0[3],
                );

                affine_scale_definitions.push(quote! {
                    (#fn_name_ident, #conversion_factor, #affine_offset, #p2, #p3, #p5, #pi)
                });
            }

            // Generate trait name for affine units with "Affine" suffix
            let affine_trait_name = format!("{}Affine", trait_name);
            let affine_trait_ident = syn::parse_str::<Ident>(&affine_trait_name).unwrap();

            let expansion = quote! {
                define_nonstorage_affine_quantity!(
                    #mass_exp,
                    #length_exp,
                    #time_exp,
                    #current_exp,
                    #temperature_exp,
                    #amount_exp,
                    #luminosity_exp,
                    #angle_exp,
                    #affine_trait_ident,
                    #(#affine_scale_definitions),*
                );
            };

            expansions.push(expansion);
        }
    }


    /// Generate non-metric declarators for all systems
    fn generate_nonmetric_declarators(&self, expansions: &mut Vec<TokenStream>) {
        use whippyunits_core::System;

        // Define all non-metric systems to iterate over
        const NON_METRIC_SYSTEMS: &[System] = &[System::Imperial, System::Astronomical];

        // Loop over all non-metric systems
        for system in NON_METRIC_SYSTEMS {
            self.generate_system_declarators(expansions, *system);
        }
    }

    /// Generate declarators for a specific system (Imperial, Astronomical, etc.)
    fn generate_system_declarators(&self, expansions: &mut Vec<TokenStream>, system: whippyunits_core::System) {
        use whippyunits_core::Dimension;

        // Collect all units from the specified system
        let mut system_units = Vec::new();

        for dimension in Dimension::ALL {
            for unit in dimension.units {
                if unit.system == system {
                    system_units.push((dimension, unit));
                }
            }
        }

        if system_units.is_empty() {
            return;
        }

        // Group units by their dimension name (not exponents)
        let mut grouped_units: std::collections::HashMap<_, Vec<_>> =
            std::collections::HashMap::new();
        for (dimension, unit) in system_units {
            grouped_units
                .entry(dimension.name)
                .or_default()
                .push((dimension, unit));
        }

        // Generate declarators for each dimension group
        for (dimension_name, units) in grouped_units {
            // Get dimension exponents from the first unit (all units in a dimension have same exponents)
            let dimension = &units[0].0;
            let (
                mass_exp,
                length_exp,
                time_exp,
                current_exp,
                temperature_exp,
                amount_exp,
                luminosity_exp,
                angle_exp,
            ) = (
                dimension.exponents.0[0], // mass
                dimension.exponents.0[1], // length
                dimension.exponents.0[2], // time
                dimension.exponents.0[3], // current
                dimension.exponents.0[4], // temperature
                dimension.exponents.0[5], // amount
                dimension.exponents.0[6], // luminous_intensity
                dimension.exponents.0[7], // angle
            );

            // Check if any units have affine offset
            let has_affine_units = units
                .iter()
                .any(|(_dimension, unit)| unit.affine_offset != 0.0);

            if has_affine_units {
                // If any units have affine offset, use the affine macro for all units
                let mut affine_scale_definitions = Vec::new();

                for (_dimension, unit) in &units {
                    // Skip units with invalid identifier names (e.g., unicode characters)
                    if !is_valid_identifier(unit.name) {
                        continue;
                    }

                    // Generate function name (pluralized)
                    let fn_name = whippyunits_core::make_plural(unit.name);
                    let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

                    // Extract conversion factor and affine offset
                    let conversion_factor = unit.conversion_factor;
                    let affine_offset = unit.affine_offset;

                    // Extract scale parameters directly from the unit
                    let (p2, p3, p5, pi) = (
                        unit.scale.0[0],
                        unit.scale.0[1],
                        unit.scale.0[2],
                        unit.scale.0[3],
                    );

                    affine_scale_definitions.push(quote! {
                        (#fn_name_ident, #conversion_factor, #affine_offset, #p2, #p3, #p5, #pi)
                    });
                }

                // Generate trait name using the system's canonical name from units.rs
                let system_name = system.as_str();
                let trait_name = format!(
                    "{}{}",
                    system_name,
                    whippyunits_core::CapitalizedFmt(dimension_name).to_string()
                );
                let trait_ident = syn::parse_str::<Ident>(&trait_name).unwrap();

                let expansion = quote! {
                    define_nonstorage_affine_quantity!(
                        #mass_exp,
                        #length_exp,
                        #time_exp,
                        #current_exp,
                        #temperature_exp,
                        #amount_exp,
                        #luminosity_exp,
                        #angle_exp,
                        #trait_ident,
                        #(#affine_scale_definitions),*
                    );
                };
                expansions.push(expansion);
            } else {
                // If no units have affine offset, use the regular nonstorage macro
                let mut regular_scale_definitions = Vec::new();

                for (_dimension, unit) in &units {
                    // Skip units with invalid identifier names (e.g., unicode characters)
                    if !is_valid_identifier(unit.name) {
                        continue;
                    }

                    // Generate function name (pluralized)
                    let fn_name = whippyunits_core::make_plural(unit.name);
                    let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

                    // Extract conversion factor
                    let conversion_factor = unit.conversion_factor;

                    // Extract scale parameters directly from the unit
                    let (p2, p3, p5, pi) = (
                        unit.scale.0[0],
                        unit.scale.0[1],
                        unit.scale.0[2],
                        unit.scale.0[3],
                    );

                    regular_scale_definitions.push(quote! {
                        (#fn_name_ident, #conversion_factor, #p2, #p3, #p5, #pi)
                    });
                }

                // Generate trait name using the system's canonical name from units.rs
                let system_name = system.as_str();
                let trait_name = format!(
                    "{}{}",
                    system_name,
                    whippyunits_core::CapitalizedFmt(dimension_name).to_string()
                );
                let trait_ident = syn::parse_str::<Ident>(&trait_name).unwrap();

                let expansion = quote! {
                    define_nonstorage_quantity!(
                        #mass_exp,
                        #length_exp,
                        #time_exp,
                        #current_exp,
                        #temperature_exp,
                        #amount_exp,
                        #luminosity_exp,
                        #angle_exp,
                        #trait_ident,
                        #(#regular_scale_definitions),*
                    );
                };
                expansions.push(expansion);
            }
        }
    }

    /// Generate the literals module for culit integration
    fn generate_literals_module(&self) -> TokenStream {
        // Use the new proc macro to generate literals module
        quote! {
            whippyunits_proc_macros::generate_literals_module!();
        }
    }
}