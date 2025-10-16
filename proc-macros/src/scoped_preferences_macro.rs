use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Ident;

/// Input for the generate_scoped_preferences macro
/// Usage: generate_scoped_preferences!()
pub struct ScopedPreferencesInput;

impl Parse for ScopedPreferencesInput {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(ScopedPreferencesInput)
    }
}

impl ScopedPreferencesInput {
    pub fn expand(self) -> TokenStream {
        // Get the SI prefixes from whippyunits-core crate
        let si_prefixes = whippyunits_core::SiPrefix::ALL;

        // Generate the define_base_units macro with all the type names from source of truth
        self.generate_define_base_units_macro(si_prefixes)
    }

    fn generate_define_base_units_macro(
        &self,
        si_prefixes: &[whippyunits_core::SiPrefix],
    ) -> TokenStream {
        // Generate mass units
        let mass_units = self.generate_mass_units(si_prefixes);

        // Generate length units
        let length_units = self.generate_length_units(si_prefixes);

        // Generate time units
        let time_units = self.generate_time_units(si_prefixes);

        // Generate current units
        let current_units = self.generate_current_units(si_prefixes);

        // Generate amount units
        let amount_units = self.generate_amount_units(si_prefixes);

        // Generate luminosity units
        let luminosity_units = self.generate_luminosity_units(si_prefixes);

        // Generate angle units
        let angle_units = self.generate_angle_units(si_prefixes);

        quote! {
            #[macro_export]
            macro_rules! define_base_units {
                (
                    $mass_scale:ident,
                    $length_scale:ident,
                    $time_scale:ident,
                    $current_scale:ident,
                    $temperature_scale:ident,
                    $amount_scale:ident,
                    $luminosity_scale:ident,
                    $angle_scale:ident
                ) => {
                    // Note: Users must call define_literals!() separately to get custom literals.
                    // This macro only defines the scoped preferences and local quantity types.
                    //
                    // Example usage:
                    //   define_base_units!(Kilogram, Kilometer, Second, Ampere, Kelvin, Mole, Candela, Radian);
                    //   whippyunits::define_literals!(); // Required for custom literals like 5.0m, 2.5kg

                    $crate::define_local_quantity!(
                        $mass_scale,
                        LocalMass,
                        #(#mass_units),*
                    );

                    $crate::define_local_quantity!(
                        $length_scale,
                        LocalLength,
                        #(#length_units),*
                    );

                    $crate::define_local_quantity!(
                        $time_scale,
                        LocalTime,
                        #(#time_units),*
                    );

                    $crate::define_local_quantity!(
                        $current_scale,
                        LocalCurrent,
                        #(#current_units),*
                    );

                    $crate::define_local_quantity!(
                        $amount_scale,
                        LocalAmount,
                        #(#amount_units),*
                    );

                    $crate::define_local_quantity!(
                        $luminosity_scale,
                        LocalLuminosity,
                        #(#luminosity_units),*
                    );

                    $crate::define_local_quantity!(
                        $angle_scale,
                        LocalAngle,
                        #(#angle_units),*
                    );

                    /// Define a quantity with the specified value and storage type, scaled to the local base units.
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
                    /// quantity!(value, unit_expression)
                    /// quantity!(value, unit_expression, storage_type)
                    /// ```
                    ///
                    /// where:
                    /// - `value`: The value of the quantity
                    /// - `unit_expression`: A "unit literal expression"
                    ///     - A "unit literal expression" is either:
                    ///         - An atomic unit:
                    ///             - `m`, `kg`, `s`, `A`, `K`, `mol`, `cd`, `rad`
                    ///         - A multiplication of two or more atomic units:
                    ///             - `m * kg`
                    ///         - A division of two or more atomic units:
                    ///             - `m / s`
                    ///         - An exponentiation of an atomic unit:
                    ///             - `m^2`, `s^-1`
                    ///         - A combination of the above:
                    ///             - `m * kg / s^2`
                    /// - `storage_type`: An optional storage type for the quantity. Defaults to `f64`.
                    ///
                    /// ## Examples
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
                    #[macro_export]
                    macro_rules! quantity {
                        ($value:expr, $unit:expr) => {
                            {
                                // Create a quantity using the declared units (source)
                                let declared_quantity = <whippyunits::unit!($unit, f64)>::new($value);

                                // Rescale it to the local scale units (target) - the target type is generated by local_unit_type!
                                let rescaled: $crate::local_unit_type!($unit, $mass_scale, $length_scale, $time_scale, $current_scale, $temperature_scale, $amount_scale, $luminosity_scale, $angle_scale) = whippyunits::rescale_f64(declared_quantity);
                                rescaled as $crate::local_unit_type!($unit, $mass_scale, $length_scale, $time_scale, $current_scale, $temperature_scale, $amount_scale, $luminosity_scale, $angle_scale)

                            }
                        };
                        ($value:expr, $unit:expr, f64) => {
                            {
                                // Create a quantity using the declared units (source)
                                let declared_quantity = <whippyunits::unit!($unit, f64)>::new($value);

                                // Rescale it to the local scale units (target) - the target type is generated by local_unit_type!
                                let rescaled: $crate::local_unit_type!($unit, $mass_scale, $length_scale, $time_scale, $current_scale, $temperature_scale, $amount_scale, $luminosity_scale, $angle_scale) = whippyunits::rescale_f64(declared_quantity);
                                rescaled as $crate::local_unit_type!($unit, $mass_scale, $length_scale, $time_scale, $current_scale, $temperature_scale, $amount_scale, $luminosity_scale, $angle_scale)
                            }
                        };
                        ($value:expr, $unit:expr, i32) => {
                            {
                                // Create a quantity using the declared units (source)
                                let declared_quantity = <whippyunits::unit!($unit, i32)>::new($value);

                                // Rescale it to the local scale units (target) - the target type is generated by local_unit_type!
                                let rescaled: $crate::local_unit_type!($unit, $mass_scale, $length_scale, $time_scale, $current_scale, $temperature_scale, $amount_scale, $luminosity_scale, $angle_scale, i32) = whippyunits::rescale_i32(declared_quantity);
                                rescaled as $crate::local_unit_type!($unit, $mass_scale, $length_scale, $time_scale, $current_scale, $temperature_scale, $amount_scale, $luminosity_scale, $angle_scale, i32)
                            }
                        };
                        ($value:expr, $unit:expr, i64) => {
                            {
                                // Create a quantity using the declared units (source)
                                let declared_quantity = <whippyunits::unit!($unit, i64)>::new($value);

                                // Rescale it to the local scale units (target) - the target type is generated by local_unit_type!
                                let rescaled: $crate::local_unit_type!($unit, $mass_scale, $length_scale, $time_scale, $current_scale, $temperature_scale, $amount_scale, $luminosity_scale, $angle_scale, i64) = whippyunits::rescale_i64(declared_quantity);
                                rescaled as $crate::local_unit_type!($unit, $mass_scale, $length_scale, $time_scale, $current_scale, $temperature_scale, $amount_scale, $luminosity_scale, $angle_scale, i64)
                            }
                        };
                        ($value:expr, $unit:expr, $storage_type:ty) => {
                            {
                                // Create a quantity using the declared units (source)
                                let declared_quantity = <whippyunits::unit!($unit, $storage_type)>::new($value);

                                // For other storage types, fall back to f64 rescaling
                                let rescaled: $crate::local_unit_type!($unit, $mass_scale, $length_scale, $time_scale, $current_scale, $temperature_scale, $amount_scale, $luminosity_scale, $angle_scale, f64) = whippyunits::rescale_f64(declared_quantity);
                                rescaled as $crate::local_unit_type!($unit, $mass_scale, $length_scale, $time_scale, $current_scale, $temperature_scale, $amount_scale, $luminosity_scale, $angle_scale, f64)
                            }
                        };
                    }
                };
            }
        }
    }

    fn generate_mass_units(&self, si_prefixes: &[whippyunits_core::SiPrefix]) -> Vec<TokenStream> {
        let mut units = Vec::new();

        // Add base gram
        units.push(quote! { (Gram, grams) });

        // Add all SI prefixed grams that actually exist in default declarators
        for prefix in si_prefixes {
            let type_name = self.generate_scale_name(prefix.name(), "gram");
            let fn_name = format!("{}grams", prefix.name());
            let type_ident = syn::parse_str::<Ident>(&type_name).unwrap();
            let fn_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

            units.push(quote! { (#type_ident, #fn_ident) });
        }

        units
    }

    fn generate_length_units(
        &self,
        si_prefixes: &[whippyunits_core::SiPrefix],
    ) -> Vec<TokenStream> {
        let mut units = Vec::new();

        // Add base meter
        units.push(quote! { (Meter, meters) });

        // Add all SI prefixed meters
        for prefix in si_prefixes {
            let type_name = self.generate_scale_name(prefix.name(), "meter");
            let fn_name = format!("{}meters", prefix.name());
            let type_ident = syn::parse_str::<Ident>(&type_name).unwrap();
            let fn_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

            units.push(quote! { (#type_ident, #fn_ident) });
        }

        units
    }

    fn generate_time_units(&self, si_prefixes: &[whippyunits_core::SiPrefix]) -> Vec<TokenStream> {
        let mut units = Vec::new();

        // Add base second
        units.push(quote! { (Second, seconds) });

        // Add all SI prefixed seconds
        for prefix in si_prefixes {
            let type_name = self.generate_scale_name(prefix.name(), "second");
            let fn_name = format!("{}seconds", prefix.name());
            let type_ident = syn::parse_str::<Ident>(&type_name).unwrap();
            let fn_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

            units.push(quote! { (#type_ident, #fn_ident) });
        }

        // Add common time units
        units.push(quote! { (Minute, minutes) });
        units.push(quote! { (Hour, hours) });
        units.push(quote! { (Day, days) });

        units
    }

    fn generate_current_units(
        &self,
        si_prefixes: &[whippyunits_core::SiPrefix],
    ) -> Vec<TokenStream> {
        let mut units = Vec::new();

        // Add base ampere
        units.push(quote! { (Ampere, amperes) });

        // Add all SI prefixed amperes
        for prefix in si_prefixes {
            let type_name = self.generate_scale_name(prefix.name(), "ampere");
            let fn_name = format!("{}amperes", prefix.name());
            let type_ident = syn::parse_str::<Ident>(&type_name).unwrap();
            let fn_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

            units.push(quote! { (#type_ident, #fn_ident) });
        }

        units
    }

    fn generate_amount_units(
        &self,
        si_prefixes: &[whippyunits_core::SiPrefix],
    ) -> Vec<TokenStream> {
        let mut units = Vec::new();

        // Add base mole
        units.push(quote! { (Mole, moles) });

        // Add all SI prefixed moles
        for prefix in si_prefixes {
            let type_name = self.generate_scale_name(prefix.name(), "mole");
            let fn_name = format!("{}moles", prefix.name());
            let type_ident = syn::parse_str::<Ident>(&type_name).unwrap();
            let fn_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

            units.push(quote! { (#type_ident, #fn_ident) });
        }

        units
    }

    fn generate_luminosity_units(
        &self,
        si_prefixes: &[whippyunits_core::SiPrefix],
    ) -> Vec<TokenStream> {
        let mut units = Vec::new();

        // Add base candela
        units.push(quote! { (Candela, candelas) });

        // Add all SI prefixed candelas
        for prefix in si_prefixes {
            let type_name = self.generate_scale_name(prefix.name(), "candela");
            let fn_name = format!("{}candelas", prefix.name());
            let type_ident = syn::parse_str::<Ident>(&type_name).unwrap();
            let fn_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

            units.push(quote! { (#type_ident, #fn_ident) });
        }

        units
    }

    fn generate_angle_units(&self, si_prefixes: &[whippyunits_core::SiPrefix]) -> Vec<TokenStream> {
        let mut units = Vec::new();

        // Add base radian
        units.push(quote! { (Radian, radians) });

        // Add all SI prefixed radians
        for prefix in si_prefixes {
            let type_name = self.generate_scale_name(prefix.name(), "radian");
            let fn_name = format!("{}radians", prefix.name());
            let type_ident = syn::parse_str::<Ident>(&type_name).unwrap();
            let fn_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

            units.push(quote! { (#type_ident, #fn_ident) });
        }

        // Add common angle units
        units.push(quote! { (Turn, turns) });
        units.push(quote! { (Degree, degrees) });
        units.push(quote! { (Gradian, gradians) });
        units.push(quote! { (Arcminute, arcminutes) });
        units.push(quote! { (Arcsecond, arcseconds) });

        units
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
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
