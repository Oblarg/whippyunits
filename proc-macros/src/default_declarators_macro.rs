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
        // Get the dimension data from whippyunits-core crate
        let si_prefixes = whippyunits_core::SiPrefix::ALL;

        let mut expansions = Vec::new();

        // Generate SI base unit declarators
        self.generate_si_base_declarators(&mut expansions, si_prefixes);

        // Generate non-base unit declarators (J, W, N, Pa, etc.)
        self.generate_non_base_unit_declarators(&mut expansions);

        // Generate non-metric (imperial) unit declarators
        self.generate_nonmetric_declarators(&mut expansions);

        // Generate the literals module using the new approach
        let literals_module = self.generate_literals_module();
        
        quote! {
            #(#expansions)*
            
            // Automatically generate literals module for culit integration
            #literals_module
        }
    }

    fn generate_si_base_declarators(
        &self,
        expansions: &mut Vec<TokenStream>,
        si_prefixes: &[whippyunits_core::SiPrefix],
    ) {
        use whippyunits_core::Dimension;

        // Get all dimensions and process their base units (first unit in each dimension)
        for dimension in Dimension::ALL {
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

            // Get the first unit (base unit) from this dimension
            let base_unit = match dimension.units.first() {
                Some(unit) => unit,
                None => continue,
            };

            // Only process metric base units
            if base_unit.system != whippyunits_core::System::Metric {
                continue;
            }

            // Get the base unit name from the dimension programmatically
            let base_unit_name = base_unit.name;
            let unit_suffix = whippyunits_core::make_plural(base_unit_name);
            
            // Generate trait name from dimension name, converting spaces to underscores
            let sanitized_name = dimension.name.replace(" ", "");
            let trait_name = format!("SI{}", whippyunits_core::CapitalizedFmt(&sanitized_name).to_string());

            let trait_ident = syn::parse_str::<Ident>(&trait_name).unwrap();

            // Generate the scale definitions for each SI prefix
            let mut scale_definitions = Vec::new();

            // First, generate the base unit (no prefix)
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

            // Then generate all the prefixed units
            for prefix in si_prefixes {
                // Generate the correct naming convention using the source of truth
                let scale_name = generate_scale_name(prefix.name(), &unit_suffix);
                let fn_name = format!("{}{}", prefix.name(), unit_suffix);

                // Calculate scale factors
                // For mass (gram base), we need to account for the -3 inherent scale factor
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
    }



    fn generate_common_temperature_declarators(&self, expansions: &mut Vec<TokenStream>) {
        use whippyunits_core::{Dimension, System};

        // Find temperature units (dimension exponents: (0, 0, 0, 0, 1, 0, 0, 0))
        let temperature_dimension = Dimension::TEMPERATURE;

        // Get the base unit for this dimension (Kelvin)
        let base_unit = temperature_dimension
            .units
            .iter()
            .find(|unit| unit.conversion_factor == 1.0 && unit.affine_offset == 0.0)
            .unwrap();

        let temperature_units: Vec<_> = temperature_dimension
            .units
            .iter()
            .filter(|unit| {
                !unit.symbols.is_empty()
                    && !std::ptr::eq(*unit, base_unit)
                    && unit.system == System::Metric
            }) // Exclude base unit and imperial units
            .collect();

        if temperature_units.is_empty() {
            return;
        }

        // Separate units with affine offset only (like Celsius) from units with both conversion factor and affine offset (like Fahrenheit)
        let mut affine_only_units = Vec::new();
        let mut nonstorage_affine_units = Vec::new();

        for unit in temperature_units {
            // Skip units with invalid identifier names (e.g., unicode characters)
            if !is_valid_identifier(unit.name) {
                continue;
            }

            if unit.conversion_factor == 1.0 {
                // Pure affine unit (like Celsius)
                let fn_name = whippyunits_core::make_plural(unit.name);
                let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();
                let type_name = whippyunits_core::CapitalizedFmt(unit.name).to_string();
                let type_name_ident = syn::parse_str::<Ident>(&type_name).unwrap();
                let affine_offset = unit.affine_offset;

                affine_only_units.push(quote! {
                    (#type_name_ident, #fn_name_ident, #affine_offset)
                });
            } else {
                // Unit with both conversion factor and affine offset (like Fahrenheit)
                let fn_name = whippyunits_core::make_plural(unit.name);
                let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();
                let (p2, p3, p5, pi) = (
                    unit.scale.0[0],
                    unit.scale.0[1],
                    unit.scale.0[2],
                    unit.scale.0[3],
                );
                let conversion_factor = unit.conversion_factor;
                let affine_offset = unit.affine_offset;

                nonstorage_affine_units.push(quote! {
                    (#fn_name_ident, #conversion_factor, #affine_offset, #p2, #p3, #p5, #pi)
                });
            }
        }

        // Generate affine-only units (like Celsius)
        if !affine_only_units.is_empty() {
            let expansion = quote! {
                define_affine_quantity!(
                    0,
                    0,
                    0,
                    0,
                    1,
                    0,
                    0,
                    0, // temperature dimension
                    CommonTemperature,
                    Kelvin,
                    #(#affine_only_units),*
                );
            };
            expansions.push(expansion);
        }

        // Generate nonstorage affine units (like Fahrenheit)
        if !nonstorage_affine_units.is_empty() {
            let expansion = quote! {
                define_nonstorage_affine_quantity!(
                    0,
                    0,
                    0,
                    0,
                    1,
                    0,
                    0,
                    0, // temperature dimension
                    NonStorageTemperature,
                    #(#nonstorage_affine_units),*
                );
            };
            expansions.push(expansion);
        }
    }

    fn generate_common_angle_declarators(&self, expansions: &mut Vec<TokenStream>) {
        use whippyunits_core::Dimension;

        // Find angle units (dimension exponents: (0, 0, 0, 0, 0, 0, 0, 1))
        let angle_dimension = Dimension::ANGLE;
        let angle_units: Vec<_> = angle_dimension
            .units
            .iter()
            .filter(|unit| !unit.symbols.is_empty() && !unit.symbols.contains(&"rad")) // Exclude base radian
            .collect();

        if angle_units.is_empty() {
            return;
        }

        // Deduplicate by symbol to avoid duplicate definitions
        let mut seen_symbols = std::collections::HashSet::new();
        let mut scale_definitions = Vec::new();

        for unit in angle_units {
            // Skip units with invalid identifier names (e.g., unicode characters)
            if !is_valid_identifier(unit.name) {
                continue;
            }

            // Use the first symbol for type name generation
            let primary_symbol = unit.symbols[0];
            if seen_symbols.insert(primary_symbol) {
                let (p2, p3, p5, pi) = (
                    unit.scale.0[0],
                    unit.scale.0[1],
                    unit.scale.0[2],
                    unit.scale.0[3],
                );
                // Generate type name from long name
                let type_name = whippyunits_core::CapitalizedFmt(unit.name).to_string();
                let scale_name_ident = syn::parse_str::<Ident>(&type_name).unwrap();

                // Add 's' to make function names plural
                let fn_name = whippyunits_core::make_plural(unit.name);
                let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

                scale_definitions.push(quote! {
                    (#scale_name_ident, #fn_name_ident, #p2, #p3, #p5, #pi)
                });
            }
        }

        let expansion = quote! {
            define_quantity!(
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                CommonAngle,
                #(#scale_definitions),*
            );
        };
        expansions.push(expansion);
    }

    fn generate_non_base_unit_declarators(&self, expansions: &mut Vec<TokenStream>) {
        use whippyunits_core::{Dimension, System};

        // Collect all metric units from all dimensions
        let mut metric_units = Vec::new();

        for dimension in Dimension::ALL {
            for (i, unit) in dimension.units.iter().enumerate() {
                if !unit.symbols.is_empty() && unit.system == System::Metric {
                    // For all dimensions: include metric units that are NOT the base unit
                    // (the base unit already gets prefix treatment in generate_si_base_declarators)
                    if i > 0 {
                        metric_units.push((dimension, unit));
                    }
                }
            }
        }

        if metric_units.is_empty() {
            return;
        }

        // Group metric units by their dimension name (not exponents)
        let mut grouped_units: std::collections::HashMap<_, Vec<_>> =
            std::collections::HashMap::new();
        for (dimension, unit) in metric_units {
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

            let mut scale_definitions = Vec::new();

            for (_dimension, unit) in &units {
                // Skip units with invalid identifier names (e.g., unicode characters)
                if !is_valid_identifier(unit.name) {
                    continue;
                }

                let (p2, p3, p5, pi) = (
                    unit.scale.0[0],
                    unit.scale.0[1],
                    unit.scale.0[2],
                    unit.scale.0[3],
                );

                // Generate type name from long name
                let type_name = whippyunits_core::CapitalizedFmt(unit.name).to_string();
                let scale_name_ident = syn::parse_str::<Ident>(&type_name).unwrap();

                // Generate function name (pluralized)
                let fn_name = whippyunits_core::make_plural(unit.name);
                let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

                scale_definitions.push(quote! {
                    (#scale_name_ident, #fn_name_ident, #p2, #p3, #p5, #pi)
                });

                // Generate prefixed versions of this compound unit
                use whippyunits_core::SiPrefix;
                for prefix_info in SiPrefix::ALL {
                    // Calculate the new scale factors by adding the prefix scale to p2 and p5
                    let prefixed_p2 = p2 + prefix_info.factor_log10();
                    let prefixed_p3 = p3;
                    let prefixed_p5 = p5 + prefix_info.factor_log10();
                    let prefixed_pi = pi;

                    // Generate prefixed type name
                    let unit_singular = unit.name.trim_end_matches('s');
                    let prefixed_type_name = format!("{}{}", prefix_info.name(), unit_singular);
                    let prefixed_type_name_capitalized =
                        whippyunits_core::CapitalizedFmt(&prefixed_type_name).to_string();
                    let prefixed_scale_name_ident =
                        syn::parse_str::<Ident>(&prefixed_type_name_capitalized).unwrap();

                    // Generate prefixed function name (pluralized)
                    let unit_singular = unit.name.trim_end_matches('s');
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

            if !scale_definitions.is_empty() {
                // Generate a trait name based on the dimension (like ImperialVolume)
                // Remove spaces and capitalize first letter to make valid Rust identifier
                let mut trait_name = dimension_name.to_string();
                if let Some(first) = trait_name.chars().next() {
                    trait_name = format!("{}{}", first.to_uppercase(), &trait_name[1..]);
                }
                trait_name = trait_name.replace(" ", "");
                let trait_name = format!("Metric{}", trait_name);
                let trait_ident = syn::parse_str::<Ident>(&trait_name).unwrap();

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
        }
    }

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
