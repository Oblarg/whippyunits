use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Ident;

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
        // Get the dimension data from default-dimensions crate
        let si_prefixes = whippyunits_default_dimensions::SI_PREFIXES;
        
        let mut expansions = Vec::new();
        
        // Generate SI base unit declarators
        self.generate_si_base_declarators(&mut expansions, si_prefixes);
        
        // Generate common time declarators
        self.generate_common_time_declarators(&mut expansions);
        
        // Generate common temperature declarators
        self.generate_common_temperature_declarators(&mut expansions);
        
        // Generate common angle declarators
        self.generate_common_angle_declarators(&mut expansions);
        
        // Generate compound unit declarators (J, W, N, Pa, etc.)
        self.generate_compound_unit_declarators(&mut expansions);
        
        quote! {
            #(#expansions)*
        }
    }
    
    fn generate_si_base_declarators(
        &self,
        expansions: &mut Vec<TokenStream>,
        si_prefixes: &[whippyunits_default_dimensions::PrefixInfo],
    ) {
        use whippyunits_default_dimensions::get_atomic_dimensions;
        
        // Get the atomic dimensions (first 8 dimensions are the base dimensions)
        let base_dimensions = get_atomic_dimensions();
        
        for dimension in base_dimensions {
            let (mass_exp, length_exp, time_exp, current_exp, temperature_exp, amount_exp, luminosity_exp, angle_exp) = dimension.exponents;
            
            // Determine the trait name and unit names based on dimension
            let (trait_name, unit_suffix) = match dimension.name {
                "Mass" => ("SIMass", "grams"),
                "Length" => ("SILength", "meters"),
                "Time" => ("SITime", "seconds"),
                "Current" => ("SICurrent", "amperes"),
                "Temperature" => ("SITemperature", "kelvins"),
                "Amount" => ("SIAmount", "moles"),
                "Luminosity" => ("SILuminosity", "candelas"),
                "Angle" => ("SIAngle", "radians"),
                _ => continue,
            };
            
            let trait_ident = syn::parse_str::<Ident>(trait_name).unwrap();
            
            // Generate the scale definitions for each SI prefix
            let mut scale_definitions = Vec::new();
            
            // First, generate the base unit (no prefix)
            let base_scale_name = self.generate_scale_name("", unit_suffix);
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
                let scale_name = self.generate_scale_name(prefix.long_name, unit_suffix);
                let fn_name = format!("{}{}", prefix.long_name, unit_suffix);
                
                // Calculate scale factors
                // For mass (gram base), we need to account for the -3 inherent scale factor
                let (p2, p3, p5, pi) = if dimension.name == "Mass" {
                    // Gram has inherent -3 scale factor, so we add the prefix scale factor
                    let total_scale = -3 + prefix.scale_factor;
                    (total_scale as i16, 0i16, total_scale as i16, 0i16)
                } else {
                    // Other units have 0 inherent scale factor
                    (prefix.scale_factor as i16, 0i16, prefix.scale_factor as i16, 0i16)
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
    
    fn generate_scale_name(&self, prefix_name: &str, unit_suffix: &str) -> String {
        // Systematically generate the correct naming convention
        let unit_singular = unit_suffix.trim_end_matches('s');
        let combined_name = if prefix_name.is_empty() {
            unit_singular.to_string()
        } else {
            format!("{}{}", prefix_name, unit_singular)
        };
        
        // Capitalize only the first letter of the entire name
        whippyunits_default_dimensions::util::capitalize_first(&combined_name)
    }
    
    fn generate_common_time_declarators(&self, expansions: &mut Vec<TokenStream>) {
        use whippyunits_default_dimensions::get_units_by_exponents;
        
        // Find time units (dimension exponents: (0, 0, 1, 0, 0, 0, 0, 0))
        let time_units: Vec<_> = get_units_by_exponents((0, 0, 1, 0, 0, 0, 0, 0))
            .into_iter()
            .filter(|(_dimension, unit)| !unit.symbols.is_empty() && !unit.symbols.contains(&"s")) // Exclude base second
            .collect();
        
        if time_units.is_empty() {
            return;
        }
        
        // Deduplicate by symbol to avoid duplicate definitions
        let mut seen_symbols = std::collections::HashSet::new();
        let mut scale_definitions = Vec::new();
        
        for (_dimension, unit) in time_units {
            // Use the first symbol for type name generation
            let primary_symbol = unit.symbols[0];
            if seen_symbols.insert(primary_symbol) {
                let (p2, p3, p5, pi) = unit.scale_factors.unwrap_or((0, 0, 0, 0));
                // Generate type name from long name
                let type_name = whippyunits_default_dimensions::util::capitalize_first(unit.long_name);
                let scale_name_ident = syn::parse_str::<Ident>(&type_name).unwrap();
                
                // Add 's' to make function names plural
                let fn_name = format!("{}s", unit.long_name);
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
                1,
                0,
                0,
                0,
                0,
                0,
                CommonTime,
                #(#scale_definitions),*
            );
        };
        expansions.push(expansion);
    }
    
    fn generate_common_temperature_declarators(&self, expansions: &mut Vec<TokenStream>) {
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
                (Celsius, celsius, 273.15), // °C to K: C + 273.15
            );
        };
        expansions.push(expansion);
    }
    
    fn generate_common_angle_declarators(&self, expansions: &mut Vec<TokenStream>) {
        use whippyunits_default_dimensions::get_units_by_exponents;
        
        // Find angle units (dimension exponents: (0, 0, 0, 0, 0, 0, 0, 1))
        let angle_units: Vec<_> = get_units_by_exponents((0, 0, 0, 0, 0, 0, 0, 1))
            .into_iter()
            .filter(|(_dimension, unit)| !unit.symbols.is_empty() && !unit.symbols.contains(&"rad")) // Exclude base radian
            .collect();
        
        if angle_units.is_empty() {
            return;
        }
        
        // Deduplicate by symbol to avoid duplicate definitions
        let mut seen_symbols = std::collections::HashSet::new();
        let mut scale_definitions = Vec::new();
        
        for (_dimension, unit) in angle_units {
            // Use the first symbol for type name generation
            let primary_symbol = unit.symbols[0];
            if seen_symbols.insert(primary_symbol) {
                let (p2, p3, p5, pi) = unit.scale_factors.unwrap_or((0, 0, 0, 0));
                // Generate type name from long name
                let type_name = whippyunits_default_dimensions::util::capitalize_first(unit.long_name);
                let scale_name_ident = syn::parse_str::<Ident>(&type_name).unwrap();
                
                // Add 's' to make function names plural
                let fn_name = format!("{}s", unit.long_name);
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
    
    fn generate_compound_unit_declarators(&self, expansions: &mut Vec<TokenStream>) {
        use whippyunits_default_dimensions::get_all_dimensions;
        
        // Generate declarators for all composite units (compound or derived) defined in the dimensions data
        let mut compound_units = Vec::new();
        
        for dimension in get_all_dimensions() {
            use whippyunits_default_dimensions::is_composite_dimension;
            
            // Anything that's not atomic is composite (compound or derived)
            if is_composite_dimension(dimension.exponents) {
                for unit in dimension.units {
                    // Include all units that have symbols (no whitelist needed - the data structure is clean)
                    if !unit.symbols.is_empty() {
                        compound_units.push((dimension, unit));
                    }
                }
            }
        }
        
        // Group compound units by their dimension exponents
        let mut grouped_units: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        for (dimension, unit) in compound_units {
            grouped_units.entry(dimension.exponents).or_default().push((dimension, unit));
        }
        
        // Generate declarators for each group
        for (exponents, units) in grouped_units {
            let (mass_exp, length_exp, time_exp, current_exp, temperature_exp, amount_exp, luminosity_exp, angle_exp) = exponents;
            
            let mut scale_definitions = Vec::new();
            
            for (_dimension, unit) in &units {
                let (p2, p3, p5, pi) = unit.scale_factors.unwrap_or((0, 0, 0, 0));
                
                // Generate type name from long name
                let type_name = whippyunits_default_dimensions::util::capitalize_first(unit.long_name);
                let scale_name_ident = syn::parse_str::<Ident>(&type_name).unwrap();
                
                // Add 's' to make function names plural for composite units
                let fn_name = format!("{}s", unit.long_name);
                let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();
                
                scale_definitions.push(quote! {
                    (#scale_name_ident, #fn_name_ident, #p2, #p3, #p5, #pi)
                });
                
                // Generate prefixed versions of this compound unit
                use whippyunits_default_dimensions::SI_PREFIXES;
                for prefix_info in SI_PREFIXES {
                    // Calculate the new scale factors by adding the prefix scale to p2 and p5
                    let prefixed_p2 = p2 + prefix_info.scale_factor;
                    let prefixed_p3 = p3;
                    let prefixed_p5 = p5 + prefix_info.scale_factor;
                    let prefixed_pi = pi;
                    
                    // Generate prefixed type name
                    let unit_singular = unit.long_name.trim_end_matches('s');
                    let prefixed_type_name = format!("{}{}", prefix_info.long_name, unit_singular);
                    let prefixed_type_name_capitalized = whippyunits_default_dimensions::util::capitalize_first(&prefixed_type_name);
                    let prefixed_scale_name_ident = syn::parse_str::<Ident>(&prefixed_type_name_capitalized).unwrap();
                    
                    // Generate prefixed function name (pluralized)
                    let prefixed_fn_name = format!("{}{}s", prefix_info.long_name, unit.long_name);
                    let prefixed_fn_name_ident = syn::parse_str::<Ident>(&prefixed_fn_name).unwrap();
                    
                    scale_definitions.push(quote! {
                        (#prefixed_scale_name_ident, #prefixed_fn_name_ident, #prefixed_p2, #prefixed_p3, #prefixed_p5, #prefixed_pi)
                    });
                }
            }
            
            if !scale_definitions.is_empty() {
                // Generate a unique trait name for this dimension combination
                // Use a more descriptive name based on the first unit in the group
                let first_unit = &units[0].1;
                let trait_name = format!("{}Unit", whippyunits_default_dimensions::util::capitalize_first(first_unit.long_name));
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
}
