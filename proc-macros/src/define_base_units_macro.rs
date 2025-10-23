use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Ident;

use crate::shared_utils::generate_scale_name;

/// Input for the define_base_units macro
/// Usage: define_base_units!(Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian, local_scale)
pub struct DefineBaseUnitsInput {
    pub mass_scale: Ident,
    pub length_scale: Ident,
    pub time_scale: Ident,
    pub current_scale: Ident,
    pub temperature_scale: Ident,
    pub amount_scale: Ident,
    pub luminosity_scale: Ident,
    pub angle_scale: Ident,
    pub namespace: Ident,
}

impl Parse for DefineBaseUnitsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mass_scale: Ident = input.parse()?;
        let _comma: syn::token::Comma = input.parse()?;
        let length_scale: Ident = input.parse()?;
        let _comma: syn::token::Comma = input.parse()?;
        let time_scale: Ident = input.parse()?;
        let _comma: syn::token::Comma = input.parse()?;
        let current_scale: Ident = input.parse()?;
        let _comma: syn::token::Comma = input.parse()?;
        let temperature_scale: Ident = input.parse()?;
        let _comma: syn::token::Comma = input.parse()?;
        let amount_scale: Ident = input.parse()?;
        let _comma: syn::token::Comma = input.parse()?;
        let luminosity_scale: Ident = input.parse()?;
        let _comma: syn::token::Comma = input.parse()?;
        let angle_scale: Ident = input.parse()?;
        let _comma: syn::token::Comma = input.parse()?;
        let namespace: Ident = input.parse()?;

        Ok(DefineBaseUnitsInput {
            mass_scale,
            length_scale,
            time_scale,
            current_scale,
            temperature_scale,
            amount_scale,
            luminosity_scale,
            angle_scale,
            namespace,
        })
    }
}

impl DefineBaseUnitsInput {
    pub fn expand(self) -> TokenStream {
        // Get the SI prefixes from whippyunits-core crate
        let si_prefixes = whippyunits_core::SiPrefix::ALL;

        // Extract the scale parameters
        let mass_scale = self.mass_scale;
        let length_scale = self.length_scale;
        let time_scale = self.time_scale;
        let current_scale = self.current_scale;
        let temperature_scale = self.temperature_scale;
        let amount_scale = self.amount_scale;
        let luminosity_scale = self.luminosity_scale;
        let angle_scale = self.angle_scale;
        let namespace = self.namespace;

        // Generate all trait definitions using the same iteration strategy as default_declarators
        let mut trait_definitions = Vec::new();
        Self::generate_local_quantity_traits(
            &mut trait_definitions,
            si_prefixes,
            &mass_scale,
            &length_scale,
            &time_scale,
            &current_scale,
            &temperature_scale,
            &amount_scale,
            &luminosity_scale,
            &angle_scale,
        );

        // Generate literals module
        let literals_module = Self::generate_literals_module_static(
            &mass_scale,
            &length_scale,
            &time_scale,
            &current_scale,
            &temperature_scale,
            &amount_scale,
            &luminosity_scale,
            &angle_scale,
            &namespace,
        );

        quote! {
            pub mod #namespace {
                use whippyunits::rescale_f64;
                use whippyunits::rescale_i32;
                use whippyunits::rescale_i64;
                use whippyunits::local_unit_type;
                
                // Generate the trait definitions and implementations for each dimension
                #(#trait_definitions)*

                pub mod literals {
                    #literals_module
                }
            }

            /// Define a local quantity with the specified value and storage type, scaled to the local base units.
            ///
            /// This is a *local shadow* of the [quantity!](crate::quantity!) macro - if you are surprised by this,
            /// look for an invocation of [define_base_units!](crate::define_base_units!) in the scope.  This macro will always
            /// store values in the local base units.  Therefore, the  *declaration type* of a `quantity!` invocation is
            /// not necessarily the same as the *storage type* of the quantity.  When in doubt, use a concrete type assertion
            /// with [unit!](crate::unit!), whose behavior does not depend on the base units.
            ///
            /// ## Syntax
            ///
            /// ```rust
            /// use whippyunits::quantity;
            ///
            /// // Basic quantities
            /// let distance = quantity!(5.0, m);
            /// let mass = quantity!(2.5, kg);
            /// let time = quantity!(10.0, s);
            ///
            /// // Compound units
            /// let velocity = quantity!(10.0, m / s);
            /// let acceleration = quantity!(9.81, m / s^2);
            /// let force = quantity!(100.0, kg * m / s^2);
            /// let energy = quantity!(50.0, kg * m^2 / s^2);
            ///
            /// // With explicit storage type
            /// let distance_f32 = quantity!(5.0, m, f32);
            /// let mass_i32 = quantity!(2, kg, i32);
            ///
            /// // Complex expressions
            /// let power = quantity!(1000.0, kg * m^2 / s^3);
            /// let pressure = quantity!(101325.0, kg / m / s^2);
            /// ```
            macro_rules! quantity {
                ($value:expr, $unit:expr) => {
                    {
                        let declared_quantity = <whippyunits::unit!($unit, f64)>::new($value);
                        let rescaled = whippyunits::rescale_f64(declared_quantity);
                        rescaled as whippyunits::local_unit_type!($unit, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale)
                    }
                };
                ($value:expr, $unit:expr, f64) => {
                    {
                        let declared_quantity = <whippyunits::unit!($unit, f64)>::new($value);
                        let rescaled = whippyunits::rescale_f64(declared_quantity);
                        rescaled as whippyunits::local_unit_type!($unit, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale)
                    }
                };
                ($value:expr, $unit:expr, i32) => {
                    {
                        let declared_quantity = <whippyunits::unit!($unit, i32)>::new($value);
                        let rescaled = whippyunits::rescale_i32(declared_quantity);
                        rescaled as whippyunits::local_unit_type!($unit, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, i32)
                    }
                };
                ($value:expr, $unit:expr, i64) => {
                    {
                        let declared_quantity = <whippyunits::unit!($unit, i64)>::new($value);
                        let rescaled = whippyunits::rescale_i64(declared_quantity);
                        rescaled as whippyunits::local_unit_type!($unit, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, i64)
                    }
                };
                ($value:expr, $unit:expr, $storage_type:ty) => {
                    {
                        let declared_quantity = <whippyunits::unit!($unit, $storage_type)>::new($value);
                        let rescaled = whippyunits::rescale(declared_quantity);
                        rescaled as whippyunits::local_unit_type!($unit, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, $storage_type)
                    }
                };
            }


        }
    }

    /// Generate local quantity traits using the same iteration strategy as default_declarators
    fn generate_local_quantity_traits(
        expansions: &mut Vec<TokenStream>,
        si_prefixes: &[whippyunits_core::SiPrefix],
        mass_scale: &Ident,
        length_scale: &Ident,
        time_scale: &Ident,
        current_scale: &Ident,
        temperature_scale: &Ident,
        amount_scale: &Ident,
        luminosity_scale: &Ident,
        angle_scale: &Ident,
    ) {
        use whippyunits_core::Dimension;

        // Get the atomic dimensions (first 8 dimensions are the base dimensions)
        let base_dimensions = Dimension::BASIS;

        for dimension in base_dimensions {
            let (
                _mass_exp,
                _length_exp,
                _time_exp,
                _current_exp,
                _temperature_exp,
                _amount_exp,
                _luminosity_exp,
                _angle_exp,
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

            // Determine the trait name and unit names based on dimension
            let (trait_name, unit_suffix, scale_ident) = match dimension.name {
                "Mass" => ("LocalMass", "grams", mass_scale),
                "Length" => ("LocalLength", "meters", length_scale),
                "Time" => ("LocalTime", "seconds", time_scale),
                "Current" => ("LocalCurrent", "amperes", current_scale),
                "Temperature" => ("LocalTemperature", "kelvins", temperature_scale),
                "Amount" => ("LocalAmount", "moles", amount_scale),
                "Luminosity" => ("LocalLuminosity", "candelas", luminosity_scale),
                "Angle" => ("LocalAngle", "radians", angle_scale),
                _ => continue,
            };

            let trait_ident = syn::parse_str::<Ident>(trait_name).unwrap();

            // Generate the scale definitions for each SI prefix
            let mut scale_definitions = Vec::new();

            // First, generate the base unit (no prefix)
            let base_scale_name = generate_scale_name("", unit_suffix);
            let base_fn_name = unit_suffix.to_string();

            let base_scale_name_ident = syn::parse_str::<Ident>(&base_scale_name).unwrap();
            let base_fn_name_ident = syn::parse_str::<Ident>(&base_fn_name).unwrap();

            scale_definitions.push(quote! {
                (#base_scale_name_ident, #base_fn_name_ident)
            });

            // Then generate all the prefixed units
            for prefix in si_prefixes {
                // Generate the correct naming convention using the source of truth
                let scale_name = generate_scale_name(prefix.name(), unit_suffix);
                let fn_name = format!("{}{}", prefix.name(), unit_suffix);

                let scale_name_ident = syn::parse_str::<Ident>(&scale_name).unwrap();
                let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

                scale_definitions.push(quote! {
                    (#scale_name_ident, #fn_name_ident)
                });
            }

            // Note: Additional units (like Celsius, Fahrenheit, etc.) are not included in the base dimensions
            // They would need to be handled separately, just like in default_declarators_macro.rs

            // Generate the trait definition and implementations
            let mut trait_methods = Vec::new();
            let mut impl_f64_methods = Vec::new();
            let mut impl_i32_methods = Vec::new();
            let mut impl_i64_methods = Vec::new();

            for scale_def in &scale_definitions {
                // Parse the scale definition to extract method names and scale names
                let scale_str = scale_def.to_string();
                if let Some((scale_name_str, fn_name_str)) = Self::parse_scale_tuple(&scale_str) {
                    let scale_name_ident = syn::parse_str::<Ident>(&scale_name_str).unwrap();
                    let fn_name_ident = syn::parse_str::<Ident>(&fn_name_str).unwrap();
                    
                    // Generate trait method - return the local scale type (the actual default declarator type)
                    trait_methods.push(quote! {
                        fn #fn_name_ident(self) -> whippyunits::default_declarators::#scale_ident<T>;
                    });
                    
                    // Generate f64 implementation - convert from default declarator to local scale
                    impl_f64_methods.push(quote! {
                        fn #fn_name_ident(self) -> whippyunits::default_declarators::#scale_ident<f64> {
                            whippyunits::rescale_f64(whippyunits::default_declarators::#scale_name_ident::new(self))
                        }
                    });
                    
                    // Generate i32 implementation
                    impl_i32_methods.push(quote! {
                        fn #fn_name_ident(self) -> whippyunits::default_declarators::#scale_ident<i32> {
                            whippyunits::rescale_i32(whippyunits::default_declarators::#scale_name_ident::new(self))
                        }
                    });
                    
                    // Generate i64 implementation
                    impl_i64_methods.push(quote! {
                        fn #fn_name_ident(self) -> whippyunits::default_declarators::#scale_ident<i64> {
                            whippyunits::rescale_i64(whippyunits::default_declarators::#scale_name_ident::new(self))
                        }
                    });
                }
            }

            let expansion = quote! {
                // Generate the trait definition (generic over storage type)
                pub trait #trait_ident<T = f64> {
                    #(#trait_methods)*
                }

                // Generate extension trait implementations for f64 (default)
                impl #trait_ident<f64> for f64 {
                    #(#impl_f64_methods)*
                }

                // Generate extension trait implementations for i32
                impl #trait_ident<i32> for i32 {
                    #(#impl_i32_methods)*
                }

                // Generate extension trait implementations for i64
                impl #trait_ident<i64> for i64 {
                    #(#impl_i64_methods)*
                }
            };

            expansions.push(expansion);
        }
    }

    /// Parse a scale tuple string like "(Grams, grams)" into (scale_name, fn_name)
    fn parse_scale_tuple(scale_str: &str) -> Option<(String, String)> {
        // Remove parentheses and split by comma
        let trimmed = scale_str.trim_start_matches('(').trim_end_matches(')');
        let parts: Vec<&str> = trimmed.split(',').collect();
        
        if parts.len() == 2 {
            let scale_name = parts[0].trim().to_string();
            let fn_name = parts[1].trim().to_string();
            Some((scale_name, fn_name))
        } else {
            None
        }
    }

    /// Generate literals module with proper scale parameters
    fn generate_literals_module_static(
        mass_scale: &Ident,
        length_scale: &Ident,
        time_scale: &Ident,
        current_scale: &Ident,
        temperature_scale: &Ident,
        amount_scale: &Ident,
        luminosity_scale: &Ident,
        angle_scale: &Ident,
        namespace: &Ident,
    ) -> TokenStream {
        // Use the actual scale parameters to generate the literals module
        let scale_params = (
            mass_scale.clone(),
            length_scale.clone(),
            time_scale.clone(),
            current_scale.clone(),
            temperature_scale.clone(),
            amount_scale.clone(),
            luminosity_scale.clone(),
            angle_scale.clone(),
        );
        super::generate_literal_macros_module("literals", true, Some(scale_params), true, Some(namespace.clone()))
    }
}