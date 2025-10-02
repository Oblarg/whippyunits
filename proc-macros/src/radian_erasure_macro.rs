use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Ident;

/// Input for the generate_radian_erasure macro
/// Usage: generate_radian_erasure!()
pub struct RadianErasureInput;

/// Input for the generate_radian_to_dimensionless macro
/// Usage: generate_radian_to_dimensionless!()
pub struct RadianToDimensionlessInput;

/// Input for the generate_all_radian_erasures macro
/// Usage: generate_all_radian_erasures!(max_exponent)
pub struct AllRadianErasuresInput {
    pub max_exponent: syn::LitInt,
}

impl Parse for RadianErasureInput {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(RadianErasureInput)
    }
}

impl Parse for RadianToDimensionlessInput {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(RadianToDimensionlessInput)
    }
}

impl Parse for AllRadianErasuresInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let max_exponent = input.parse()?;
        Ok(AllRadianErasuresInput { max_exponent })
    }
}

impl RadianErasureInput {
    pub fn expand(self) -> TokenStream {
        let types = [
            ("f32", "rescale_f32"),
            ("f64", "rescale_f64"),
            ("i16", "rescale_i16"),
            ("i32", "rescale_i32"),
            ("i64", "rescale_i64"),
            ("i128", "rescale_i128"),
        ];

        let exponents = [-3, -2, -1, 1, 2, 3];

        let mut expansions = Vec::new();

        // Generate same-type conversions
        for (type_name, rescale_fn) in types {
            for &exponent in &exponents {
                let type_ident = syn::parse_str::<Ident>(type_name).unwrap();
                let rescale_ident = syn::parse_str::<Ident>(rescale_fn).unwrap();
                
                // Convert exponent to a literal token
                let exponent_lit = syn::LitInt::new(&exponent.to_string(), proc_macro2::Span::call_site());
                
                let expansion = quote! {
                    define_from_for_radians_with_scale!(#exponent_lit, #type_ident, #rescale_ident);
                };
                
                expansions.push(expansion);
            }
        }

        // Generate cross-type conversions for common type pairs
        let cross_type_pairs = [
            (("f32", "rescale_f32"), ("f64", "rescale_f64")),
            (("f64", "rescale_f64"), ("f32", "rescale_f32")),
            (("i32", "rescale_i32"), ("f32", "rescale_f32")),
            (("i32", "rescale_i32"), ("f64", "rescale_f64")),
            (("i64", "rescale_i64"), ("f32", "rescale_f32")),
            (("i64", "rescale_i64"), ("f64", "rescale_f64")),
            (("f32", "rescale_f32"), ("i32", "rescale_i32")),
            (("f64", "rescale_f64"), ("i32", "rescale_i32")),
            (("f32", "rescale_f32"), ("i64", "rescale_i64")),
            (("f64", "rescale_f64"), ("i64", "rescale_i64")),
        ];

        for ((source_type, source_rescale), (target_type, _)) in cross_type_pairs {
            for &exponent in &exponents {
                let source_type_ident = syn::parse_str::<Ident>(source_type).unwrap();
                let target_type_ident = syn::parse_str::<Ident>(target_type).unwrap();
                let source_rescale_ident = syn::parse_str::<Ident>(source_rescale).unwrap();
                
                let exponent_lit = syn::LitInt::new(&exponent.to_string(), proc_macro2::Span::call_site());
                
                let expansion = quote! {
                    define_from_for_radians_with_scale_cross_type!(#exponent_lit, #source_type_ident, #target_type_ident, #source_rescale_ident);
                };
                
                expansions.push(expansion);
            }
        }

        quote! {
            #(#expansions)*
        }
    }
}

impl RadianToDimensionlessInput {
    pub fn expand(self) -> TokenStream {
        let types = ["f32", "f64", "i16", "i32", "i64", "i128"];
        let exponents = [-9, -8, -7, -6, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut expansions = Vec::new();

        for &exponent in &exponents {
            let exponent_lit = syn::LitInt::new(&exponent.to_string(), proc_macro2::Span::call_site());
            
            let type_list = types.iter()
                .map(|&type_name| syn::parse_str::<Ident>(type_name).unwrap())
                .collect::<Vec<_>>();
            
            let expansion = quote! {
                define_from_for_radians!(#exponent_lit, #(#type_list),*);
            };
            
            expansions.push(expansion);
        }

        quote! {
            #(#expansions)*
        }
    }
}

impl AllRadianErasuresInput {
    pub fn expand(self) -> TokenStream {
        let max_exponent = self.max_exponent.base10_parse::<i32>().unwrap();
        
        // Generate radian to scalar conversions (with scale handling)
        let types_with_rescale = [
            ("f32", "rescale_f32"),
            ("f64", "rescale_f64"),
            ("i16", "rescale_i16"),
            ("i32", "rescale_i32"),
            ("i64", "rescale_i64"),
            ("i128", "rescale_i128"),
        ];

        // Generate exponents for radian to scalar (limited range)
        let scalar_exponents: Vec<i32> = (-max_exponent..=max_exponent)
            .filter(|&x| x != 0)
            .collect();

        let mut scalar_expansions = Vec::new();
        
        // Generate same-type conversions
        for (type_name, rescale_fn) in types_with_rescale {
            for &exponent in &scalar_exponents {
                let type_ident = syn::parse_str::<Ident>(type_name).unwrap();
                let rescale_ident = syn::parse_str::<Ident>(rescale_fn).unwrap();
                
                let exponent_lit = syn::LitInt::new(&exponent.to_string(), proc_macro2::Span::call_site());
                
                let expansion = quote! {
                    define_from_for_radians_with_scale!(#exponent_lit, #type_ident, #rescale_ident);
                };
                
                scalar_expansions.push(expansion);
            }
        }

        // Generate cross-type conversions for common type pairs
        let cross_type_pairs = [
            (("f32", "rescale_f32"), ("f64", "rescale_f64")),
            (("f64", "rescale_f64"), ("f32", "rescale_f32")),
            (("i32", "rescale_i32"), ("f32", "rescale_f32")),
            (("i32", "rescale_i32"), ("f64", "rescale_f64")),
            (("i64", "rescale_i64"), ("f32", "rescale_f32")),
            (("i64", "rescale_i64"), ("f64", "rescale_f64")),
            (("f32", "rescale_f32"), ("i32", "rescale_i32")),
            (("f64", "rescale_f64"), ("i32", "rescale_i32")),
            (("f32", "rescale_f32"), ("i64", "rescale_i64")),
            (("f64", "rescale_f64"), ("i64", "rescale_i64")),
        ];

        for ((source_type, source_rescale), (target_type, _)) in cross_type_pairs {
            for &exponent in &scalar_exponents {
                let source_type_ident = syn::parse_str::<Ident>(source_type).unwrap();
                let target_type_ident = syn::parse_str::<Ident>(target_type).unwrap();
                let source_rescale_ident = syn::parse_str::<Ident>(source_rescale).unwrap();
                
                let exponent_lit = syn::LitInt::new(&exponent.to_string(), proc_macro2::Span::call_site());
                
                let expansion = quote! {
                    define_from_for_radians_with_scale_cross_type!(#exponent_lit, #source_type_ident, #target_type_ident, #source_rescale_ident);
                };
                
                scalar_expansions.push(expansion);
            }
        }
        
        // Generate radian to dimensionless quantity conversions
        let types = ["f32", "f64", "i16", "i32", "i64", "i128"];
        
        // Generate exponents for radian to dimensionless (full range)
        let dimensionless_exponents: Vec<i32> = (-max_exponent..=max_exponent)
            .filter(|&x| x != 0)
            .collect();

        let mut dimensionless_expansions = Vec::new();
        for &exponent in &dimensionless_exponents {
            let exponent_lit = syn::LitInt::new(&exponent.to_string(), proc_macro2::Span::call_site());
            
            let type_list = types.iter()
                .map(|&type_name| syn::parse_str::<Ident>(type_name).unwrap())
                .collect::<Vec<_>>();
            
            let expansion = quote! {
                define_from_for_radians!(#exponent_lit, #(#type_list),*);
            };
            
            dimensionless_expansions.push(expansion);
        }
        
        quote! {
            #(#scalar_expansions)*
            #(#dimensionless_expansions)*
        }
    }
}
