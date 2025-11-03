use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{token::Comma, Ident};

use crate::utils::lift_trace::scale_type_to_actual_unit_symbol;
use crate::utils::scale_suggestions::find_similar_scales;
use crate::utils::shared_utils::generate_scale_name;

/// Input for the define_unit_declarators macro
/// Usage: define_unit_declarators!(local_scale, Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian)
/// Or with brand: define_unit_declarators!(local_scale, MyBrand, Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian)
/// Or brand-only: define_unit_declarators!(local_scale, MyBrand)
pub struct DefineBaseUnitsInput {
    pub namespace: Ident,
    pub brand: Option<Ident>,
    pub base_units: Option<(Ident, Ident, Ident, Ident, Ident, Ident, Ident, Ident)>, // (mass, length, time, current, temp, amount, lum, angle)
}

impl Parse for DefineBaseUnitsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse namespace first
        let namespace: Ident = input.parse()?;

        // Count total number of comma-separated identifiers remaining
        let fork = input.fork();
        let mut total_params = 0;
        // Skip the comma after namespace
        if fork.parse::<Comma>().is_err() {
            // No comma, so no params
            return Ok(DefineBaseUnitsInput {
                namespace,
                brand: None,
                base_units: None,
            });
        }
        // Count identifiers
        let peek = fork;
        while !peek.is_empty() {
            if peek.parse::<Ident>().is_ok() {
                total_params += 1;
                // Check if there's a comma after this ident
                if peek.is_empty() {
                    break; // No more tokens
                }
                if peek.parse::<Comma>().is_err() {
                    break; // No comma, we're done
                }
            } else {
                break; // Not an identifier, stop counting
            }
        }

        // Map parameter count to behavior (excluding namespace):
        // 1 param => branded default shadow: define_unit_declarators!(namespace, Brand)
        // 8 params => unbranded rescaling shadow: define_unit_declarators!(namespace, Kilogram, Millimeter, ...)
        // 9 params => branded rescaling shadow: define_unit_declarators!(namespace, Brand, Kilogram, Millimeter, ...)

        if total_params == 0 {
            // Just namespace - no brand, no base units (probably an error, but handle gracefully)
            return Ok(DefineBaseUnitsInput {
                namespace,
                brand: None,
                base_units: None,
            });
        } else if total_params == 1 {
            // namespace, brand => branded default shadow
            let _comma: Comma = input.parse()?;
            let brand_ident: Ident = input.parse()?;
            // Allow optional trailing comma
            let _ = input.parse::<Comma>().ok();
            return Ok(DefineBaseUnitsInput {
                namespace,
                brand: Some(brand_ident),
                base_units: None,
            });
        } else if total_params == 8 {
            // namespace, 8 base units => unbranded rescaling shadow
            let _comma: Comma = input.parse()?;
            let mass_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let length_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let time_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let current_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let temperature_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let amount_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let luminosity_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let angle_scale: Ident = input.parse()?;
            return Ok(DefineBaseUnitsInput {
                namespace,
                brand: None,
                base_units: Some((
                    mass_scale,
                    length_scale,
                    time_scale,
                    current_scale,
                    temperature_scale,
                    amount_scale,
                    luminosity_scale,
                    angle_scale,
                )),
            });
        } else if total_params == 9 {
            // namespace, brand, 8 base units => branded rescaling shadow
            let _comma: Comma = input.parse()?;
            let brand_ident: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let mass_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let length_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let time_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let current_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let temperature_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let amount_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let luminosity_scale: Ident = input.parse()?;
            let _comma: Comma = input.parse()?;
            let angle_scale: Ident = input.parse()?;
            return Ok(DefineBaseUnitsInput {
                namespace,
                brand: Some(brand_ident),
                base_units: Some((
                    mass_scale,
                    length_scale,
                    time_scale,
                    current_scale,
                    temperature_scale,
                    amount_scale,
                    luminosity_scale,
                    angle_scale,
                )),
            });
        } else {
            return Err(syn::Error::new(
                input.span(),
                format!(
                    "Expected 1, 8, or 9 parameters after namespace, found {}",
                    total_params
                ),
            ));
        }
    }
}

impl DefineBaseUnitsInput {
    pub fn expand(self) -> TokenStream {
        // Handle brand-only case (no base units specified)
        if self.base_units.is_none() {
            return self.expand_brand_only();
        }

        // Get the SI prefixes from whippyunits-core crate
        let si_prefixes = whippyunits_core::SiPrefix::ALL;

        // Extract the scale parameters from the tuple
        let base_units_tuple = self.base_units.as_ref().unwrap();
        let mass_scale = base_units_tuple.0.clone();
        let length_scale = base_units_tuple.1.clone();
        let time_scale = base_units_tuple.2.clone();
        let current_scale = base_units_tuple.3.clone();
        let temperature_scale = base_units_tuple.4.clone();
        let amount_scale = base_units_tuple.5.clone();
        let luminosity_scale = base_units_tuple.6.clone();
        let angle_scale = base_units_tuple.7.clone();
        let namespace = self.namespace;

        // Get the brand identifier - we'll use it directly in the generated code
        // The type will be created from the identifier in the outer scope where Brand is defined
        let brand_ident = self.brand.clone();

        // Create a token stream for the brand type - use the namespace-qualified identifier if present, otherwise ()
        // Since the macro is exported, it needs to reference the brand type with the full module path
        let brand_type = if let Some(ref ident) = brand_ident {
            quote! { #namespace::#ident }
        } else {
            quote! { () }
        };

        // Create the brand struct definition token stream if brand is present
        let brand_struct_def = if let Some(ref ident) = brand_ident {
            quote! {
                pub struct #ident;
            }
        } else {
            quote! {}
        };

        // Generate documentation structs for scale identifiers
        let doc_structs = Self::generate_scale_documentation(
            &mass_scale,
            &length_scale,
            &time_scale,
            &current_scale,
            &temperature_scale,
            &amount_scale,
            &luminosity_scale,
            &angle_scale,
        );

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
            &brand_ident,
        );

        // Generate base units documentation string
        let base_units_docstring = Self::generate_base_units_docstring(
            &mass_scale,
            &length_scale,
            &time_scale,
            &current_scale,
            &temperature_scale,
            &amount_scale,
            &luminosity_scale,
            &angle_scale,
        );

        // Generate brand note for documentation (only if brand is present)
        let brand_note_doc_if_present: Vec<String> = if let Some(ref brand) = brand_ident {
            vec![format!(
                "\n\n\
                All quantities in this module are branded with the **{}** type parameter.",
                brand
            )]
        } else {
            vec![]
        };
        let brand_note_doc_for_macro: Vec<String> = brand_note_doc_if_present.clone();

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

        // Create the prefixed macro name identifier
        let prefixed_macro_name = syn::Ident::new(
            &format!("{}_quantity", namespace.to_string()),
            namespace.span(),
        );

        quote! {
            // Generate documentation structs for scale identifiers (validation happens here)
            const _: () = {
                #doc_structs
            };

            #[doc = #base_units_docstring]
            #(#[doc = #brand_note_doc_if_present])*
            ///
            /// Declarator module for local base units.
            /// This shadows the entire [default_declarators](crate::default_declarators) module,
            /// but automatically converts all declared units to the local base units before storing the value.
            ///
            /// ## Example
            ///
            /// ```rust
            /// define_base_units!(Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian, local_scale);
            /// #[culit::culit(local_scale::literals)]
            /// fn main() {
            ///     use local_scale::*;
            ///     let distance = 1.0.meters(); // automatically converted to 1000.0 millimeters
            ///     let distance = quantity!(1.0, m); // automatically converted to 1000.0 millimeters
            ///     let distance = 1.0m; // automatically converted to 1000.0 millimeters
            /// }
            /// ```
            ///
            /// Literal declarators are also available in the inner `literals` module, for use with
            /// the [culit](https://crates.io/crates/culit) crate.
            pub mod #namespace {
                use whippyunits::rescale_f64;
                use whippyunits::rescale_i32;
                use whippyunits::rescale_i64;
                use whippyunits::local_unit;

                // Define the brand type locally in this module
                #brand_struct_def

                // Generate the trait definitions and implementations for each dimension
                #(#trait_definitions)*

                #[doc = #base_units_docstring]
                ///
                /// Custom literal declarator sugar for the local base units, for use with
                /// the [culit](https://crates.io/crates/culit) crate.  This sugars the local version of the
                /// quantity! macro, which automatically converts all declared units to the local base units
                /// before storing the value.
                ///
                /// ## Example
                ///
                /// ```rust
                /// define_base_units!(Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian, local_scale);
                /// #[culit::culit(local_scale::literals)]
                /// fn main() {
                ///     let distance = 1.0m;
                ///     assert_eq!(distance.unsafe_value, 1000.0); // automatically converted to millimeters
                ///     let energy = 1.0J; // automatically converted to microJoules
                ///     assert_eq!(energy.unsafe_value, 1000.0 * 1000.0);
                /// }
                /// ```
                ///
                /// Hovering on the literal declarators will provide documentation of the auto-conversion, showing both the
                /// declared unit and the unit to which it is converted, along with a detailed trace of the conversion chain.
                pub mod literals {
                    #literals_module
                }

                /// Define a local quantity with the specified value and storage type, scaled to the local base units.
                #(#[doc = #brand_note_doc_for_macro])*
                ///
                /// This is a *local shadow* of the [quantity!](crate::quantity!) macro - if you are surprised by this,
                /// look for an invocation of [define_unit_declarators!](crate::define_unit_declarators!) in the scope.  This macro will always
                /// store values in the local base units.  Therefore, the  *declaration type* of a `quantity!` invocation is
                /// not necessarily the same as the *storage type* of the quantity.  When in doubt, use a concrete type assertion
                /// with [unit!](crate::unit!), whose behavior does not depend on the base units.
                ///
                /// ## Syntax
                ///
                /// ```rust
                /// use branded_scale::quantity;
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
                macro_rules! #prefixed_macro_name {
                    ($value:expr, $unit:expr) => {
                        {
                            let declared_quantity = <whippyunits::unit!($unit, f64, #brand_type)>::new($value);
                            whippyunits::rescale_f64(declared_quantity) as whippyunits::local_unit!($unit, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, f64, #brand_type)
                        }
                    };
                    ($value:expr, $unit:expr, f64) => {
                        {
                            let declared_quantity = <whippyunits::unit!($unit, f64, #brand_type)>::new($value);
                            whippyunits::rescale_f64(declared_quantity) as whippyunits::local_unit!($unit, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, f64, #brand_type)
                        }
                    };
                    ($value:expr, $unit:expr, i32) => {
                        {
                            let declared_quantity = <whippyunits::unit!($unit, i32, #brand_type)>::new($value);
                            whippyunits::rescale_i32(declared_quantity) as whippyunits::local_unit!($unit, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, i32, #brand_type)
                        }
                    };
                    ($value:expr, $unit:expr, i64) => {
                        {
                            let declared_quantity = <whippyunits::unit!($unit, i64, #brand_type)>::new($value);
                            whippyunits::rescale_i64(declared_quantity) as whippyunits::local_unit!($unit, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, i64, #brand_type)
                        }
                    };
                    ($value:expr, $unit:expr, f32) => {
                        {
                            let declared_quantity = <whippyunits::unit!($unit, f32, #brand_type)>::new($value);
                            whippyunits::rescale_f32(declared_quantity) as whippyunits::local_unit!($unit, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, f32, #brand_type)
                        }
                    };
                }

                // Re-export the prefixed macro as quantity! for convenience
                pub use #prefixed_macro_name as quantity;
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
        brand_ident: &Option<Ident>,
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
            let trait_name = format!(
                "Local{}",
                whippyunits_core::CapitalizedFmt(&sanitized_name).to_string()
            );

            // Determine scale identifier based on dimension (used later for documentation)
            let _scale_ident = match dimension.name {
                "Mass" => mass_scale,
                "Length" => length_scale,
                "Time" => time_scale,
                "Current" => current_scale,
                "Temperature" => temperature_scale,
                "Amount" => amount_scale,
                "Luminosity" => luminosity_scale,
                "Angle" => angle_scale,
                _ => continue,
            };

            let trait_ident = syn::parse_str::<Ident>(&trait_name).unwrap();

            // Generate the scale definitions for each SI prefix
            let mut scale_definitions = Vec::new();

            // First, generate the base unit (no prefix)
            let base_scale_name = generate_scale_name("", base_unit_name);
            let base_fn_name = unit_suffix.to_string();

            let base_scale_name_ident = syn::parse_str::<Ident>(&base_scale_name).unwrap();
            let base_fn_name_ident = syn::parse_str::<Ident>(&base_fn_name).unwrap();

            scale_definitions.push(quote! {
                (#base_scale_name_ident, #base_fn_name_ident)
            });

            // Then generate all the prefixed units
            for prefix in si_prefixes {
                // Generate the correct naming convention using the source of truth
                let scale_name = generate_scale_name(prefix.name(), base_unit_name);
                let fn_name = format!("{}{}", prefix.name(), unit_suffix);

                let scale_name_ident = syn::parse_str::<Ident>(&scale_name).unwrap();
                let fn_name_ident = syn::parse_str::<Ident>(&fn_name).unwrap();

                scale_definitions.push(quote! {
                    (#scale_name_ident, #fn_name_ident)
                });
            }

            // Note: Additional units (like Celsius, Fahrenheit, etc.) are not included in the base dimensions
            // They would need to be handled separately, just like in generate_default_declarators_macro.rs

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

                    // Convert scale identifier (e.g., "Millimeter", "Kilogram") to unit symbol (e.g., "mm", "kg")
                    let unit_symbol = scale_type_to_actual_unit_symbol(&scale_name_str)
                        .unwrap_or_else(|| {
                            base_unit
                                .symbols
                                .first()
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "m".to_string())
                        });
                    let unit_symbol_ident = syn::parse_str::<Ident>(&unit_symbol).unwrap();

                    // Get the brand type token stream
                    let brand_type_tokens = if let Some(ref ident) = brand_ident {
                        quote! { #ident }
                    } else {
                        quote! { () }
                    };

                    // Generate trait method - construct the type directly using Helper pattern (same as local_unit!)
                    trait_methods.push(quote! {
                        fn #fn_name_ident(self) -> <whippyunits::Helper<{
                            0
                        }, whippyunits::local_unit!(#unit_symbol_ident, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, T, #brand_type_tokens)> as whippyunits::GetSecondGeneric>::Type;
                    });

                    // Generate f64 implementation
                    impl_f64_methods.push(quote! {
                        fn #fn_name_ident(self) -> <whippyunits::Helper<{
                            0
                        }, whippyunits::local_unit!(#unit_symbol_ident, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, f64, #brand_type_tokens)> as whippyunits::GetSecondGeneric>::Type {
                            let q = whippyunits::default_declarators::#scale_name_ident::new(self);
                            // Use type annotation to let rescale infer the target scale parameters (local_unit uses base unit scales)
                            // Note: rescale returns a quantity with brand (), so we need to convert the brand
                            type TargetQuantityUnbranded = whippyunits::local_unit!(#unit_symbol_ident, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, f64);
                            let rescaled: TargetQuantityUnbranded = whippyunits::rescale(q);
                            // Convert brand from () to #brand_type_tokens by reconstructing with new brand
                            <whippyunits::local_unit!(#unit_symbol_ident, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, f64, #brand_type_tokens)>::new(rescaled.unsafe_value)
                        }
                    });

                    // Generate i32 implementation
                    impl_i32_methods.push(quote! {
                        fn #fn_name_ident(self) -> <whippyunits::Helper<{
                            0
                        }, whippyunits::local_unit!(#unit_symbol_ident, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, i32, #brand_type_tokens)> as whippyunits::GetSecondGeneric>::Type {
                            let q = whippyunits::default_declarators::#scale_name_ident::new(self);
                            // Use type annotation to let rescale_i32 infer the target scale parameters (local_unit uses base unit scales)
                            // Note: rescale_i32 returns a quantity with brand (), so we need to convert the brand
                            type TargetQuantityUnbranded = whippyunits::local_unit!(#unit_symbol_ident, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, i32);
                            let rescaled: TargetQuantityUnbranded = whippyunits::rescale_i32(q);
                            // Convert brand from () to #brand_type_tokens by reconstructing with new brand
                            <whippyunits::local_unit!(#unit_symbol_ident, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, i32, #brand_type_tokens)>::new(rescaled.unsafe_value)
                        }
                    });

                    // Generate i64 implementation
                    impl_i64_methods.push(quote! {
                        fn #fn_name_ident(self) -> <whippyunits::Helper<{
                            0
                        }, whippyunits::local_unit!(#unit_symbol_ident, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, i64, #brand_type_tokens)> as whippyunits::GetSecondGeneric>::Type {
                            let q = whippyunits::default_declarators::#scale_name_ident::new(self);
                            // Use type annotation to let rescale_i64 infer the target scale parameters (local_unit uses base unit scales)
                            // Note: rescale_i64 returns a quantity with brand (), so we need to convert the brand
                            type TargetQuantityUnbranded = whippyunits::local_unit!(#unit_symbol_ident, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, i64);
                            let rescaled: TargetQuantityUnbranded = whippyunits::rescale_i64(q);
                            // Convert brand from () to #brand_type_tokens by reconstructing with new brand
                            <whippyunits::local_unit!(#unit_symbol_ident, #mass_scale, #length_scale, #time_scale, #current_scale, #temperature_scale, #amount_scale, #luminosity_scale, #angle_scale, i64, #brand_type_tokens)>::new(rescaled.unsafe_value)
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
        crate::utils::literal_macros::generate_literal_macros_module(
            "literals",
            true,
            Some(scale_params),
            true,
            namespace.clone(),
        )
    }

    /// Generate documentation structs for scale identifiers
    fn generate_scale_documentation(
        mass_scale: &Ident,
        length_scale: &Ident,
        time_scale: &Ident,
        current_scale: &Ident,
        temperature_scale: &Ident,
        amount_scale: &Ident,
        luminosity_scale: &Ident,
        angle_scale: &Ident,
    ) -> TokenStream {
        let mut doc_structs = Vec::new();

        // Generate documentation for each scale identifier
        let scales = vec![
            ("Mass", mass_scale),
            ("Length", length_scale),
            ("Time", time_scale),
            ("Current", current_scale),
            ("Temperature", temperature_scale),
            ("Amount", amount_scale),
            ("Luminosity", luminosity_scale),
            ("Angle", angle_scale),
        ];

        for (dimension_name, scale_ident) in scales {
            if let Some(doc_struct) = Self::generate_single_scale_doc(scale_ident, dimension_name) {
                doc_structs.push(doc_struct);
            }
        }

        quote! {
            #(#doc_structs)*
        }
    }

    /// Generate documentation string showing the defined base units
    fn generate_base_units_docstring(
        mass_scale: &Ident,
        length_scale: &Ident,
        time_scale: &Ident,
        current_scale: &Ident,
        temperature_scale: &Ident,
        amount_scale: &Ident,
        luminosity_scale: &Ident,
        angle_scale: &Ident,
    ) -> String {
        format!(
            "Base units: **{}, {}, {}, {}, {}, {}, {}, {}** <br>",
            mass_scale,
            length_scale,
            time_scale,
            current_scale,
            temperature_scale,
            amount_scale,
            luminosity_scale,
            angle_scale
        )
    }

    /// Generate documentation for a single scale identifier
    fn generate_single_scale_doc(identifier: &Ident, dimension_name: &str) -> Option<TokenStream> {
        let scale_name = identifier.to_string();

        // Check if the scale is valid first
        if !Self::is_valid_scale(&scale_name) {
            let error_message = Self::generate_scale_error_message(&scale_name);
            return Some(quote! {
                const _: () = {
                    compile_error!(#error_message);
                };
            });
        }

        let doc_comment = Self::generate_scale_doc_comment(&scale_name, dimension_name);

        // Create a new identifier with the same span as the original
        let doc_ident = syn::Ident::new(&scale_name, identifier.span());

        // Get the corresponding default declarator type
        let declarator_type = Self::get_declarator_type_for_scale(&scale_name)?;

        Some(quote! {
            const _: () = {
                #doc_comment
                #[allow(non_camel_case_types)]
                type #doc_ident = #declarator_type;
            };
        })
    }

    /// Generate documentation comment for a scale
    fn generate_scale_doc_comment(scale_name: &str, dimension_name: &str) -> TokenStream {
        let doc_text = Self::get_scale_documentation_text(scale_name, dimension_name);
        quote! {
            #[doc = #doc_text]
        }
    }

    /// Get documentation text for a scale
    fn get_scale_documentation_text(scale_name: &str, dimension_name: &str) -> String {
        format!(
            "Scale identifier: {} - Base unit for {} dimension. This will be used as the storage unit for all {} quantities in the local scale.",
            scale_name, dimension_name, dimension_name
        )
    }

    /// Check if a scale name is valid
    fn is_valid_scale(scale_name: &str) -> bool {
        // Check if it's a valid default declarator type by looking up the actual type
        Self::get_declarator_type_for_scale(scale_name).is_some()
    }

    /// Generate error message with suggestions for an unknown scale
    fn generate_scale_error_message(scale_name: &str) -> String {
        let suggestions = find_similar_scales(scale_name, 0.7);
        if suggestions.is_empty() {
            format!(
                "Unknown scale identifier '{}'. Please use a valid default declarator type name (e.g., Kilogram, Meter, Second, etc.).",
                scale_name
            )
        } else {
            let suggestion_list = suggestions
                .iter()
                .map(|(suggestion, _)| format!("'{}'", suggestion))
                .collect::<Vec<_>>()
                .join(", ");

            format!(
                "Unknown scale identifier '{}'. Did you mean: {}?",
                scale_name, suggestion_list
            )
        }
    }

    /// Get the corresponding default declarator type for a scale
    fn get_declarator_type_for_scale(scale_name: &str) -> Option<TokenStream> {
        // For scale identifiers, we need to check if they correspond to actual default declarator types
        // Scale identifiers are typically the capitalized names of base units

        // Check if it's a base unit name (like "Second", "Kilogram", "Meter", etc.)
        let atomic_dimensions = whippyunits_core::Dimension::BASIS;
        for dimension in atomic_dimensions {
            for unit in dimension.units {
                // Check if the scale name matches the unit name (capitalized)
                let unit_name_capitalized = whippyunits_core::CapitalizedFmt(unit.name).to_string();
                if unit_name_capitalized == scale_name {
                    let type_ident = syn::Ident::new(&scale_name, proc_macro2::Span::call_site());
                    return Some(quote! {
                        whippyunits::default_declarators::#type_ident
                    });
                }
            }
        }

        // Check if it's a prefixed unit name (like "Kilogram", "Millimeter", etc.)
        for prefix in whippyunits_core::SiPrefix::ALL {
            for dimension in atomic_dimensions {
                if let Some(base_unit) = dimension.units.first() {
                    // Use the same logic as generate_scale_name to ensure consistency
                    let type_name = crate::utils::shared_utils::generate_scale_name(
                        prefix.name(),
                        base_unit.name,
                    );
                    if type_name == scale_name {
                        let type_ident =
                            syn::Ident::new(&scale_name, proc_macro2::Span::call_site());
                        return Some(quote! {
                            whippyunits::default_declarators::#type_ident
                        });
                    }
                }
            }
        }

        None
    }

    /// Expand when no base units are specified - generate exact shadow of default_declarators with brand
    fn expand_brand_only(self) -> TokenStream {
        let namespace = self.namespace;
        let brand_ident = self.brand.clone();

        // Create brand type tokens
        let brand_type = if let Some(ref ident) = brand_ident {
            quote! { #namespace::#ident }
        } else {
            quote! { () }
        };

        let brand_struct_def = if let Some(ref ident) = brand_ident {
            quote! {
                pub struct #ident;
            }
        } else {
            quote! {}
        };

        // Generate trait wrappers that delegate to default_declarators
        let mut expansions = Vec::new();
        Self::generate_branded_trait_wrappers(&mut expansions, &brand_ident);

        // Generate literals module
        let literals_module = Self::generate_literals_module_static(
            &syn::Ident::new("Meter", proc_macro2::Span::call_site()),
            &syn::Ident::new("Meter", proc_macro2::Span::call_site()),
            &syn::Ident::new("Second", proc_macro2::Span::call_site()),
            &syn::Ident::new("Ampere", proc_macro2::Span::call_site()),
            &syn::Ident::new("Kelvin", proc_macro2::Span::call_site()),
            &syn::Ident::new("Mole", proc_macro2::Span::call_site()),
            &syn::Ident::new("Candela", proc_macro2::Span::call_site()),
            &syn::Ident::new("Radian", proc_macro2::Span::call_site()),
            &namespace,
        );

        // Create prefixed macro name
        let prefixed_macro_name = syn::Ident::new(
            &format!("{}_quantity", namespace.to_string()),
            namespace.span(),
        );

        // Generate brand-specific documentation
        let brand_doc = if let Some(ref brand) = brand_ident {
            format!(
                "Branded declarator module with brand type **{}**.\n\n\
                This module provides a thin wrapper around [default_declarators](crate::default_declarators) \
                that adds the `{}` brand type parameter to all quantities. Unlike rescaling declarators, \
                this module does not perform any unit conversion - quantities are stored exactly as declared.",
                brand, brand
            )
        } else {
            "Declarator module delegating to [default_declarators](crate::default_declarators)."
                .to_string()
        };

        let quantity_doc = if let Some(ref brand) = brand_ident {
            format!(
                "Define a branded quantity with the specified value and storage type.\n\n\
                This is a *thin wrapper* around [quantity!](crate::quantity!) that adds the `{}` brand type. \
                Unlike rescaling declarators, this macro does not perform any unit conversion - the quantity \
                is stored exactly as declared, with the brand type applied.\n\n\
                ## Syntax\n\n\
                ```rust\n\
                use {}::quantity;\n\n\
                // Basic quantities with brand\n\
                let distance = quantity!(5.0, m);\n\
                let mass = quantity!(2.5, kg);\n\n\
                // With explicit storage type\n\
                let distance_f32 = quantity!(5.0, m, f32);\n\
                let mass_i32 = quantity!(2, kg, i32);\n\
                ```",
                brand, namespace
            )
        } else {
            "Define a quantity with the specified value and storage type.\n\n\
            This macro delegates to [quantity!](crate::quantity!) without modification.\n\n\
            ## Syntax\n\n\
            ```rust\n\
            use {}::quantity;\n\n\
            // Basic quantities\n\
            let distance = quantity!(5.0, m);\n\
            let mass = quantity!(2.5, kg);\n\n\
            // With explicit storage type\n\
            let distance_f32 = quantity!(5.0, m, f32);\n\
            let mass_i32 = quantity!(2, kg, i32);\n\
            ```"
            .replace("{}", &namespace.to_string())
        };

        quote! {
            #[doc = #brand_doc]
            pub mod #namespace {
                #brand_struct_def

                #(#expansions)*

                #[doc = "Custom literal declarator sugar for use with the [culit](https://crates.io/crates/culit) crate."]
                pub mod literals {
                    #literals_module
                }

                #[doc = #quantity_doc]
                #[macro_export]
                macro_rules! #prefixed_macro_name {
                    ($value:expr, $unit:expr) => {
                        <whippyunits::unit!($unit, f64, #brand_type)>::new($value)
                    };
                    ($value:expr, $unit:expr, $storage_type:ty) => {
                        <whippyunits::unit!($unit, $storage_type, #brand_type)>::new($value)
                    };
                }

                pub use #prefixed_macro_name as quantity;
            }
        }
    }

    /// Generate thin trait wrappers that delegate to default_declarators and convert brand
    fn generate_branded_trait_wrappers(
        expansions: &mut Vec<TokenStream>,
        brand_ident: &Option<Ident>,
    ) {
        use crate::utils::shared_utils::{generate_scale_name, is_valid_identifier};
        use whippyunits_core::{Dimension, System};

        let brand_type_tokens = if let Some(ref ident) = brand_ident {
            quote! { #ident }
        } else {
            quote! { () }
        };

        // Iterate over all dimensions like default_declarators does
        for dimension in Dimension::ALL {
            let metric_units: Vec<_> = dimension
                .units
                .iter()
                .filter(|unit| unit.system == System::Metric)
                .collect();

            if metric_units.is_empty() {
                continue;
            }

            // Generate trait name matching default_declarators
            let trait_name = format!("Local{}", dimension.name);
            let trait_ident = match syn::parse_str::<Ident>(&trait_name) {
                Ok(ident) => ident,
                Err(_) => continue, // Skip if we can't parse the trait name
            };

            let mut trait_methods = Vec::new();
            let mut impl_f64_methods = Vec::new();
            let mut impl_i32_methods = Vec::new();
            let mut impl_i64_methods = Vec::new();

            // For each storage unit (conversion_factor == 1.0, no affine)
            for unit in &metric_units {
                if !is_valid_identifier(unit.name)
                    || unit.conversion_factor != 1.0
                    || unit.affine_offset != 0.0
                {
                    continue;
                }

                let scale_name = generate_scale_name("", unit.name);
                let scale_name_ident = match syn::parse_str::<Ident>(&scale_name) {
                    Ok(ident) => ident,
                    Err(_) => continue, // Skip if we can't parse the scale name
                };
                let fn_name = whippyunits_core::make_plural(unit.name);
                let fn_name_ident = match syn::parse_str::<Ident>(&fn_name) {
                    Ok(ident) => ident,
                    Err(_) => continue, // Skip if we can't parse the function name
                };

                // Get unit symbol for the return type
                let unit_symbol = unit
                    .symbols
                    .first()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "m".to_string());
                let unit_symbol_ident = match syn::parse_str::<Ident>(&unit_symbol) {
                    Ok(ident) => ident,
                    Err(_) => continue, // Skip if we can't parse the unit symbol
                };

                // Generate trait method signature
                trait_methods.push(quote! {
                    fn #fn_name_ident(self) -> <whippyunits::Helper<{
                        0
                    }, whippyunits::unit!(#unit_symbol_ident, T, #brand_type_tokens)> as whippyunits::GetSecondGeneric>::Type;
                });

                // Generate impls that delegate to default_declarators and convert brand
                impl_f64_methods.push(quote! {
                    fn #fn_name_ident(self) -> <whippyunits::Helper<{
                        0
                    }, whippyunits::unit!(#unit_symbol_ident, f64, #brand_type_tokens)> as whippyunits::GetSecondGeneric>::Type {
                        let q = whippyunits::default_declarators::#scale_name_ident::new(self);
                        <whippyunits::unit!(#unit_symbol_ident, f64, #brand_type_tokens)>::new(q.unsafe_value)
                    }
                });

                impl_i32_methods.push(quote! {
                    fn #fn_name_ident(self) -> <whippyunits::Helper<{
                        0
                    }, whippyunits::unit!(#unit_symbol_ident, i32, #brand_type_tokens)> as whippyunits::GetSecondGeneric>::Type {
                        let q = whippyunits::default_declarators::#scale_name_ident::new(self);
                        <whippyunits::unit!(#unit_symbol_ident, i32, #brand_type_tokens)>::new(q.unsafe_value)
                    }
                });

                impl_i64_methods.push(quote! {
                    fn #fn_name_ident(self) -> <whippyunits::Helper<{
                        0
                    }, whippyunits::unit!(#unit_symbol_ident, i64, #brand_type_tokens)> as whippyunits::GetSecondGeneric>::Type {
                        let q = whippyunits::default_declarators::#scale_name_ident::new(self);
                        <whippyunits::unit!(#unit_symbol_ident, i64, #brand_type_tokens)>::new(q.unsafe_value)
                    }
                });
            }

            // Generate trait with all methods
            if !trait_methods.is_empty() {
                expansions.push(quote! {
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
                });
            }
        }
    }
}
