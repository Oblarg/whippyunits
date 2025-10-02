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
        let dimension_lookup = whippyunits_default_dimensions::DIMENSION_LOOKUP;
        let si_prefixes = whippyunits_default_dimensions::SI_PREFIXES;
        
        let mut expansions = Vec::new();
        
        // Generate SI base unit declarators
        self.generate_si_base_declarators(&mut expansions, dimension_lookup, si_prefixes);
        
        // Generate common time declarators
        self.generate_common_time_declarators(&mut expansions);
        
        // Generate common temperature declarators
        self.generate_common_temperature_declarators(&mut expansions);
        
        // Generate common angle declarators
        self.generate_common_angle_declarators(&mut expansions);
        
        quote! {
            #(#expansions)*
        }
    }
    
    fn generate_si_base_declarators(
        &self,
        expansions: &mut Vec<TokenStream>,
        dimension_lookup: &[whippyunits_default_dimensions::DimensionInfo],
        si_prefixes: &[whippyunits_default_dimensions::PrefixInfo],
    ) {
        // Find the base dimensions (exponents with exactly one 1 and rest 0)
        let base_dimensions = dimension_lookup.iter().filter(|info| {
            let (mass, length, time, current, temp, amount, lum, angle) = info.exponents;
            let non_zero_count = [mass, length, time, current, temp, amount, lum, angle]
                .iter()
                .filter(|&&x| x != 0)
                .count();
            non_zero_count == 1 && [mass, length, time, current, temp, amount, lum, angle]
                .iter()
                .any(|&x| x == 1)
        });
        
        for dimension in base_dimensions {
            let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) = dimension.exponents;
            
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
                    #temp_exp,
                    #amount_exp,
                    #lum_exp,
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
        capitalize_first(&combined_name)
    }
    
    fn generate_common_time_declarators(&self, expansions: &mut Vec<TokenStream>) {
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
                (Minute, minutes, 2, 1, 1, 0),
                (Hour, hours, 4, 2, 2, 0),
                (Day, days, 7, 3, 2, 0),
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
                (Turn, turns, 1, 0, 0, 1),
                (Degrees, degrees, -2, -2, -1, 1),
                (Gradians, gradians, -3, 0, -2, 1),
                (Arcminutes, arcminutes, -4, -3, -2, 1),
                (Arcseconds, arcseconds, -6, -4, -3, 1),
            );
        };
        expansions.push(expansion);
    }
}

/// Capitalize the first character of a string
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}