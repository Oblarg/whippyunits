use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Ident;

use crate::shared_utils::{generate_scale_name, is_valid_identifier};

/// Unit type classification
#[derive(Debug, Clone, Copy, PartialEq)]
enum UnitType {
    Storage,
    NonStorage,
    StorageAffine,
    NonStorageAffine,
}

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

    /// Extract dimension exponents from a dimension
    fn extract_dimension_exponents(
        dimension: &whippyunits_core::Dimension,
    ) -> (i16, i16, i16, i16, i16, i16, i16, i16) {
        (
            dimension.exponents.0[0], // mass
            dimension.exponents.0[1], // length
            dimension.exponents.0[2], // time
            dimension.exponents.0[3], // current
            dimension.exponents.0[4], // temperature
            dimension.exponents.0[5], // amount
            dimension.exponents.0[6], // luminous_intensity
            dimension.exponents.0[7], // angle
        )
    }

    /// Extract scale factors from a unit
    fn extract_scale_factors(unit: &whippyunits_core::Unit) -> (i16, i16, i16, i16) {
        (
            unit.scale.0[0],
            unit.scale.0[1],
            unit.scale.0[2],
            unit.scale.0[3],
        )
    }

    /// Generate trait name for metric units
    fn generate_metric_trait_name(dimension_name: &str) -> String {
        let sanitized_name = dimension_name.replace(" ", "");
        format!(
            "Metric{}",
            whippyunits_core::CapitalizedFmt(&sanitized_name).to_string()
        )
    }

    /// Generate trait name for system units (Imperial, Astronomical, etc.)
    fn generate_system_trait_name(
        system: &whippyunits_core::System,
        dimension_name: &str,
    ) -> String {
        let system_name = system.as_str();
        format!(
            "{}{}",
            system_name,
            whippyunits_core::CapitalizedFmt(dimension_name).to_string()
        )
    }

    /// Generate trait name for affine units
    fn generate_affine_trait_name(base_trait_name: &str) -> String {
        format!("{}Affine", base_trait_name)
    }

    /// Generate trait name for non-storage units
    fn generate_nonstorage_trait_name(base_trait_name: &str) -> String {
        format!("{}NonStorage", base_trait_name)
    }

    /// Generate trait name for non-storage affine units
    fn generate_nonstorage_affine_trait_name(base_trait_name: &str) -> String {
        format!("{}NonStorageAffine", base_trait_name)
    }

    /// Classify a unit by its conversion factor and affine offset
    fn classify_unit(unit: &whippyunits_core::Unit) -> UnitType {
        match (unit.conversion_factor == 1.0, unit.affine_offset != 0.0) {
            (true, false) => UnitType::Storage,
            (false, false) => UnitType::NonStorage,
            (true, true) => UnitType::StorageAffine,
            (false, true) => UnitType::NonStorageAffine,
        }
    }

    /// Process units of a specific type and generate the appropriate trait
    fn process_units_by_type(
        &self,
        expansions: &mut Vec<TokenStream>,
        units: &[&whippyunits_core::Unit],
        unit_type: UnitType,
        dimension: &whippyunits_core::Dimension,
        base_trait_name: &str,
    ) {
        let (
            mass_exp,
            length_exp,
            time_exp,
            current_exp,
            temperature_exp,
            amount_exp,
            luminosity_exp,
            angle_exp,
        ) = Self::extract_dimension_exponents(dimension);

        // Filter units by type
        let filtered_units: Vec<_> = units
            .iter()
            .filter(|unit| Self::classify_unit(unit) == unit_type && is_valid_identifier(unit.name))
            .collect();

        if filtered_units.is_empty() {
            return;
        }

        match unit_type {
            UnitType::Storage => {
                let mut scale_definitions = Vec::new();
                for unit in &filtered_units {
                    let (p2, p3, p5, pi) = Self::extract_scale_factors(unit);
                    let fn_name = whippyunits_core::make_plural(unit.name);
                    let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();
                    let scale_name = generate_scale_name("", unit.name);
                    let scale_name_ident = syn::parse_str::<Ident>(&scale_name).unwrap();

                    scale_definitions.push(quote! {
                        (#scale_name_ident, #fn_name_ident, #p2, #p3, #p5, #pi)
                    });
                }

                let trait_ident = syn::parse_str::<Ident>(base_trait_name).unwrap();
                let expansion = self.generate_storage_quantity_expansion(
                    mass_exp,
                    length_exp,
                    time_exp,
                    current_exp,
                    temperature_exp,
                    amount_exp,
                    luminosity_exp,
                    angle_exp,
                    &trait_ident,
                    &scale_definitions,
                );
                expansions.push(expansion);
            }
            UnitType::NonStorage => {
                let mut scale_definitions = Vec::new();
                for unit in &filtered_units {
                    let (p2, p3, p5, pi) = Self::extract_scale_factors(unit);
                    let fn_name = whippyunits_core::make_plural(unit.name);
                    let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();
                    let conversion_factor = unit.conversion_factor;

                    scale_definitions.push(quote! {
                        (#fn_name_ident, #conversion_factor, #p2, #p3, #p5, #pi)
                    });
                }

                let trait_name = Self::generate_nonstorage_trait_name(base_trait_name);
                let trait_ident = syn::parse_str::<Ident>(&trait_name).unwrap();
                let expansion = self.generate_nonstorage_quantity_expansion(
                    mass_exp,
                    length_exp,
                    time_exp,
                    current_exp,
                    temperature_exp,
                    amount_exp,
                    luminosity_exp,
                    angle_exp,
                    &trait_ident,
                    &scale_definitions,
                );
                expansions.push(expansion);
            }
            UnitType::StorageAffine => {
                let mut scale_definitions = Vec::new();
                for unit in &filtered_units {
                    let fn_name = whippyunits_core::make_plural(unit.name);
                    let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();
                    let scale_name = generate_scale_name("", unit.name);
                    let scale_name_ident = syn::parse_str::<Ident>(&scale_name).unwrap();
                    let affine_offset = unit.affine_offset;

                    scale_definitions.push(quote! {
                        (#scale_name_ident, #fn_name_ident, #affine_offset)
                    });
                }

                let trait_name = Self::generate_affine_trait_name(base_trait_name);
                let trait_ident = syn::parse_str::<Ident>(&trait_name).unwrap();
                let storage_scale_name = self.get_storage_scale_name_for_dimension(&dimension.name);
                let storage_scale_ident = syn::parse_str::<Ident>(&storage_scale_name).unwrap();

                let expansion = self.generate_storage_affine_quantity_expansion(
                    mass_exp,
                    length_exp,
                    time_exp,
                    current_exp,
                    temperature_exp,
                    amount_exp,
                    luminosity_exp,
                    angle_exp,
                    &trait_ident,
                    &storage_scale_ident,
                    &scale_definitions,
                );
                expansions.push(expansion);
            }
            UnitType::NonStorageAffine => {
                let mut scale_definitions = Vec::new();
                for unit in &filtered_units {
                    let (p2, p3, p5, pi) = Self::extract_scale_factors(unit);
                    let fn_name = whippyunits_core::make_plural(unit.name);
                    let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();
                    let conversion_factor = unit.conversion_factor;
                    let affine_offset = unit.affine_offset;

                    scale_definitions.push(quote! {
                        (#fn_name_ident, #conversion_factor, #affine_offset, #p2, #p3, #p5, #pi)
                    });
                }

                let trait_name = Self::generate_nonstorage_affine_trait_name(base_trait_name);
                let trait_ident = syn::parse_str::<Ident>(&trait_name).unwrap();
                let expansion = self.generate_nonstorage_affine_quantity_expansion(
                    mass_exp,
                    length_exp,
                    time_exp,
                    current_exp,
                    temperature_exp,
                    amount_exp,
                    luminosity_exp,
                    angle_exp,
                    &trait_ident,
                    &scale_definitions,
                );
                expansions.push(expansion);
            }
        }
    }

    /// Generate metric declarators for all dimensions
    fn generate_metric_declarators(&self, expansions: &mut Vec<TokenStream>) {
        use whippyunits_core::{Dimension, System};

        for dimension in Dimension::ALL {
            // Get all metric units for this dimension
            let metric_units: Vec<_> = dimension
                .units
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
        ) = Self::extract_dimension_exponents(dimension);

        // Generate trait name from dimension name
        let trait_name = Self::generate_metric_trait_name(&dimension.name);
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
                // For type names, use the unit name as-is (it's already singular)
                let base_scale_name = generate_scale_name("", unit.name);
                // For function names, use the plural form
                let unit_suffix = whippyunits_core::make_plural(unit.name);
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
                    // For type names, use the unit name as-is (it's already singular)
                    let scale_name = generate_scale_name(prefix.name(), unit.name);
                    // For function names, use the plural form
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
                // Non-base unit: classify based on conversion factor and affine offset
                if unit.affine_offset != 0.0 {
                    // This is an affine unit - handle separately
                    continue; // We'll handle affine units in a separate trait
                } else if unit.conversion_factor == 1.0 {
                    // This is a storage unit (conversion_factor == 1.0)
                    let (p2, p3, p5, pi) = Self::extract_scale_factors(unit);

                    let type_name = generate_scale_name("", unit.name);
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

                        // For type names, use the unit name as-is (it's already singular)
                        let prefixed_type_name = generate_scale_name(prefix_info.name(), unit.name);
                        let prefixed_scale_name_ident =
                            syn::parse_str::<Ident>(&prefixed_type_name).unwrap();

                        // For function names, use the plural form
                        let unit_suffix = whippyunits_core::make_plural(unit.name);
                        let prefixed_fn_name = format!("{}{}", prefix_info.name(), unit_suffix);
                        let prefixed_fn_name_ident =
                            syn::parse_str::<Ident>(&prefixed_fn_name).unwrap();

                        scale_definitions.push(quote! {
                            (#prefixed_scale_name_ident, #prefixed_fn_name_ident, #prefixed_p2, #prefixed_p3, #prefixed_p5, #prefixed_pi)
                        });
                    }
                } else {
                    // This is a non-storage unit (conversion_factor != 1.0) - handle separately
                    continue; // We'll handle non-storage units in a separate trait
                }
            }
        }

        // Generate the main trait for storage units (conversion_factor == 1.0)
        if !scale_definitions.is_empty() {
            let expansion = self.generate_storage_quantity_expansion(
                mass_exp,
                length_exp,
                time_exp,
                current_exp,
                temperature_exp,
                amount_exp,
                luminosity_exp,
                angle_exp,
                &trait_ident,
                &scale_definitions,
            );
            expansions.push(expansion);
        }

        // Process other unit types using the unified system
        self.process_units_by_type(
            expansions,
            metric_units,
            UnitType::NonStorage,
            dimension,
            &trait_name,
        );
        self.process_units_by_type(
            expansions,
            metric_units,
            UnitType::StorageAffine,
            dimension,
            &trait_name,
        );
        self.process_units_by_type(
            expansions,
            metric_units,
            UnitType::NonStorageAffine,
            dimension,
            &trait_name,
        );
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
    fn generate_system_declarators(
        &self,
        expansions: &mut Vec<TokenStream>,
        system: whippyunits_core::System,
    ) {
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
            ) = Self::extract_dimension_exponents(dimension);

            // Convert units to the format expected by process_units_by_type
            let units_refs: Vec<&whippyunits_core::Unit> =
                units.iter().map(|(_dim, unit)| *unit).collect();
            let base_trait_name = Self::generate_system_trait_name(&system, dimension_name);

            // Check if we need the special non-storage trait with docs
            let has_nonstorage_units = units.iter().any(|(_dimension, unit)| {
                unit.conversion_factor != 1.0 && unit.affine_offset == 0.0
            });

            if has_nonstorage_units {
                // Special case: use the detailed non-storage trait with docs
                let mut unit_definitions = Vec::new();
                for (_dimension, unit) in &units {
                    if !is_valid_identifier(unit.name)
                        || unit.conversion_factor == 1.0
                        || unit.affine_offset != 0.0
                    {
                        continue;
                    }

                    let fn_name = whippyunits_core::make_plural(unit.name);
                    let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();
                    let conversion_factor = unit.conversion_factor;
                    let (p2, p3, p5, pi) = Self::extract_scale_factors(unit);
                    let storage_unit_name = self.get_storage_unit_name(
                        p2,
                        p3,
                        p5,
                        pi,
                        mass_exp,
                        length_exp,
                        time_exp,
                        current_exp,
                        temperature_exp,
                        amount_exp,
                        luminosity_exp,
                        angle_exp,
                    );

                    unit_definitions.push((
                        fn_name_ident,
                        conversion_factor,
                        p2,
                        p3,
                        p5,
                        pi,
                        storage_unit_name,
                    ));
                }

                let trait_ident = syn::parse_str::<Ident>(&base_trait_name).unwrap();
                let expansion = self.generate_nonstorage_trait_with_docs(
                    mass_exp,
                    length_exp,
                    time_exp,
                    current_exp,
                    temperature_exp,
                    amount_exp,
                    luminosity_exp,
                    angle_exp,
                    &trait_ident,
                    &unit_definitions,
                );
                expansions.push(expansion);
            } else {
                // Use the unified system for storage units
                self.process_units_by_type(
                    expansions,
                    &units_refs,
                    UnitType::Storage,
                    dimension,
                    &base_trait_name,
                );
            }

            // Process other unit types using the unified system
            self.process_units_by_type(
                expansions,
                &units_refs,
                UnitType::StorageAffine,
                dimension,
                &base_trait_name,
            );
            self.process_units_by_type(
                expansions,
                &units_refs,
                UnitType::NonStorageAffine,
                dimension,
                &base_trait_name,
            );
        }
    }

    /// Generate the literals module for culit integration
    fn generate_literals_module(&self) -> TokenStream {
        // Use the new proc macro to generate literals module
        quote! {
            whippyunits_proc_macros::generate_literals_module!();
        }
    }

    /// Get the storage unit name from scale exponents and dimension exponents
    /// This uses the exact same logic as the prettyprint to ensure consistency
    fn get_storage_unit_name(
        &self,
        p2: i16,
        p3: i16,
        p5: i16,
        pi: i16,
        mass_exp: i16,
        length_exp: i16,
        time_exp: i16,
        current_exp: i16,
        temperature_exp: i16,
        amount_exp: i16,
        luminosity_exp: i16,
        angle_exp: i16,
    ) -> String {
        use whippyunits_core::{
            dimension_exponents::DynDimensionExponents,
            scale_exponents::ScaleExponents,
            storage_unit::{generate_unit_literal, UnitLiteralConfig},
        };

        // Create scale exponents from the parameters
        let scale_factors = ScaleExponents([p2, p3, p5, pi]);

        // Create dimension exponents from the parameters
        let dimension_exponents = DynDimensionExponents([
            mass_exp,
            length_exp,
            time_exp,
            current_exp,
            temperature_exp,
            amount_exp,
            luminosity_exp,
            angle_exp,
        ]);

        // Use the exact same logic as prettyprint by calling the same functions from core
        // This ensures the proc macro generates the same storage unit names as the inlay hints
        let unit_literal = generate_unit_literal(
            dimension_exponents,
            scale_factors,
            UnitLiteralConfig {
                verbose: true, // Use long names for storage units
                prefer_si_units: true,
            },
        );

        // If we got a unit literal, use it; otherwise fall back to systematic generation
        if !unit_literal.is_empty() {
            unit_literal
        } else {
            // Fallback to systematic generation
            use whippyunits_core::storage_unit::generate_systematic_unit_name;
            let exponents_vec = dimension_exponents.0.to_vec();
            generate_systematic_unit_name(exponents_vec, true)
        }
    }

    /// Get the storage scale name for a given dimension using the canonical core function
    fn get_storage_scale_name_for_dimension(&self, dimension_name: &str) -> String {
        use whippyunits_core::{
            scale_exponents::ScaleExponents, storage_unit::get_storage_unit_name_by_dimension_name,
        };

        // Use identity scale factors to get the base storage unit name
        let scale_factors = ScaleExponents::IDENTITY;
        let unit_name =
            get_storage_unit_name_by_dimension_name(scale_factors, dimension_name, true);
        whippyunits_core::CapitalizedFmt(&unit_name).to_string()
    }

    /// Generate a storage quantity trait expansion
    fn generate_storage_quantity_expansion(
        &self,
        mass_exp: i16,
        length_exp: i16,
        time_exp: i16,
        current_exp: i16,
        temperature_exp: i16,
        amount_exp: i16,
        luminosity_exp: i16,
        angle_exp: i16,
        trait_ident: &Ident,
        scale_definitions: &[TokenStream],
    ) -> TokenStream {
        quote! {
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
        }
    }

    /// Generate a non-storage quantity trait expansion
    fn generate_nonstorage_quantity_expansion(
        &self,
        mass_exp: i16,
        length_exp: i16,
        time_exp: i16,
        current_exp: i16,
        temperature_exp: i16,
        amount_exp: i16,
        luminosity_exp: i16,
        angle_exp: i16,
        trait_ident: &Ident,
        scale_definitions: &[TokenStream],
    ) -> TokenStream {
        quote! {
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
                #(#scale_definitions),*
            );
        }
    }

    /// Generate a storage affine quantity trait expansion
    fn generate_storage_affine_quantity_expansion(
        &self,
        mass_exp: i16,
        length_exp: i16,
        time_exp: i16,
        current_exp: i16,
        temperature_exp: i16,
        amount_exp: i16,
        luminosity_exp: i16,
        angle_exp: i16,
        trait_ident: &Ident,
        storage_scale_ident: &Ident,
        scale_definitions: &[TokenStream],
    ) -> TokenStream {
        quote! {
            define_affine_quantity!(
                #mass_exp,
                #length_exp,
                #time_exp,
                #current_exp,
                #temperature_exp,
                #amount_exp,
                #luminosity_exp,
                #angle_exp,
                #trait_ident,
                #storage_scale_ident,
                #(#scale_definitions),*
            );
        }
    }

    /// Generate a non-storage affine quantity trait expansion
    fn generate_nonstorage_affine_quantity_expansion(
        &self,
        mass_exp: i16,
        length_exp: i16,
        time_exp: i16,
        current_exp: i16,
        temperature_exp: i16,
        amount_exp: i16,
        luminosity_exp: i16,
        angle_exp: i16,
        trait_ident: &Ident,
        scale_definitions: &[TokenStream],
    ) -> TokenStream {
        quote! {
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
                #(#scale_definitions),*
            );
        }
    }

    /// Generate the entire non-storage trait with documentation
    fn generate_nonstorage_trait_with_docs(
        &self,
        mass_exp: i16,
        length_exp: i16,
        time_exp: i16,
        current_exp: i16,
        temperature_exp: i16,
        amount_exp: i16,
        luminosity_exp: i16,
        angle_exp: i16,
        trait_ident: &Ident,
        unit_definitions: &[(Ident, f64, i16, i16, i16, i16, String)],
    ) -> TokenStream {
        use quote::quote;
        // Note: These types are re-exported from the main crate, not whippyunits_core
        // We'll use the fully qualified paths in the generated code

        // Generate trait methods with documentation
        let mut trait_methods = Vec::new();
        let mut impl_f64_methods = Vec::new();
        let mut impl_i32_methods = Vec::new();
        let mut impl_i64_methods = Vec::new();

        for (fn_name_ident, conversion_factor, p2, p3, p5, pi, storage_unit_name) in
            unit_definitions
        {
            let doc_string = format!(
                "Storage unit: **{}**<br>Conversion factor: **{}**",
                storage_unit_name, conversion_factor
            );

            // Generate trait method with documentation
            trait_methods.push(quote! {
                #[doc = #doc_string]
                fn #fn_name_ident(self) -> crate::quantity_type::Quantity<
                    crate::Scale<crate::_2<#p2>, crate::_3<#p3>, crate::_5<#p5>, crate::_Pi<#pi>>,
                    crate::Dimension<crate::_M<#mass_exp>, crate::_L<#length_exp>, crate::_T<#time_exp>, crate::_I<#current_exp>, crate::_Θ<#temperature_exp>, crate::_N<#amount_exp>, crate::_J<#luminosity_exp>, crate::_A<#angle_exp>>,
                    T,
                >;
            });

            // Generate f64 implementation
            impl_f64_methods.push(quote! {
                fn #fn_name_ident(self) -> crate::quantity_type::Quantity<
                    crate::Scale<crate::_2<#p2>, crate::_3<#p3>, crate::_5<#p5>, crate::_Pi<#pi>>,
                    crate::Dimension<crate::_M<#mass_exp>, crate::_L<#length_exp>, crate::_T<#time_exp>, crate::_I<#current_exp>, crate::_Θ<#temperature_exp>, crate::_N<#amount_exp>, crate::_J<#luminosity_exp>, crate::_A<#angle_exp>>,
                    f64,
                > {
                    crate::quantity_type::Quantity::<crate::Scale<crate::_2<#p2>, crate::_3<#p3>, crate::_5<#p5>, crate::_Pi<#pi>>, crate::Dimension<crate::_M<#mass_exp>, crate::_L<#length_exp>, crate::_T<#time_exp>, crate::_I<#current_exp>, crate::_Θ<#temperature_exp>, crate::_N<#amount_exp>, crate::_J<#luminosity_exp>, crate::_A<#angle_exp>>, f64>::new(self * #conversion_factor)
                }
            });

            // Generate i32 implementation
            impl_i32_methods.push(quote! {
                fn #fn_name_ident(self) -> crate::quantity_type::Quantity<
                    crate::Scale<crate::_2<#p2>, crate::_3<#p3>, crate::_5<#p5>, crate::_Pi<#pi>>,
                    crate::Dimension<crate::_M<#mass_exp>, crate::_L<#length_exp>, crate::_T<#time_exp>, crate::_I<#current_exp>, crate::_Θ<#temperature_exp>, crate::_N<#amount_exp>, crate::_J<#luminosity_exp>, crate::_A<#angle_exp>>,
                    i32,
                > {
                    crate::quantity_type::Quantity::<crate::Scale<crate::_2<#p2>, crate::_3<#p3>, crate::_5<#p5>, crate::_Pi<#pi>>, crate::Dimension<crate::_M<#mass_exp>, crate::_L<#length_exp>, crate::_T<#time_exp>, crate::_I<#current_exp>, crate::_Θ<#temperature_exp>, crate::_N<#amount_exp>, crate::_J<#luminosity_exp>, crate::_A<#angle_exp>>, i32>::new((self as f64 * #conversion_factor) as i32)
                }
            });

            // Generate i64 implementation
            impl_i64_methods.push(quote! {
                fn #fn_name_ident(self) -> crate::quantity_type::Quantity<
                    crate::Scale<crate::_2<#p2>, crate::_3<#p3>, crate::_5<#p5>, crate::_Pi<#pi>>,
                    crate::Dimension<crate::_M<#mass_exp>, crate::_L<#length_exp>, crate::_T<#time_exp>, crate::_I<#current_exp>, crate::_Θ<#temperature_exp>, crate::_N<#amount_exp>, crate::_J<#luminosity_exp>, crate::_A<#angle_exp>>,
                    i64,
                > {
                    crate::quantity_type::Quantity::<crate::Scale<crate::_2<#p2>, crate::_3<#p3>, crate::_5<#p5>, crate::_Pi<#pi>>, crate::Dimension<crate::_M<#mass_exp>, crate::_L<#length_exp>, crate::_T<#time_exp>, crate::_I<#current_exp>, crate::_Θ<#temperature_exp>, crate::_N<#amount_exp>, crate::_J<#luminosity_exp>, crate::_A<#angle_exp>>, i64>::new((self as f64 * #conversion_factor) as i64)
                }
            });
        }

        quote! {
            pub trait #trait_ident<T = f64> {
                #(#trait_methods)*
            }

            impl #trait_ident<f64> for f64 {
                #(#impl_f64_methods)*
            }

            impl #trait_ident<i32> for i32 {
                #(#impl_i32_methods)*
            }

            impl #trait_ident<i64> for i64 {
                #(#impl_i64_methods)*
            }
        }
    }
}
