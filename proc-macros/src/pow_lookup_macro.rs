use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Token};

/// Input for the pow_lookup macro
/// Syntax: pow_lookup!(pow2, 2, 20, rational)
/// or: pow_lookup!(pow3_float, 3.0, 15, float)
pub struct PowLookupInput {
    pub name: Ident,
    pub base: Expr,
    pub max_exp: i32,
    pub output_type: OutputType,
}

#[derive(Debug, Clone)]
pub enum OutputType {
    Rational,
    Float,
}

impl Parse for PowLookupInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse: <name>, <base>, <max_exp>, <output_type>
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let base: Expr = input.parse()?;
        input.parse::<Token![,]>()?;

        let max_exp: syn::LitInt = input.parse()?;
        input.parse::<Token![,]>()?;

        let type_ident: Ident = input.parse()?;
        let output_type = match type_ident.to_string().as_str() {
            "rational" => OutputType::Rational,
            "float" => OutputType::Float,
            _ => {
                return Err(syn::Error::new_spanned(
                    &type_ident,
                    "Expected 'rational' or 'float'",
                ))
            }
        };

        Ok(PowLookupInput {
            name,
            base,
            max_exp: max_exp.base10_parse()?,
            output_type,
        })
    }
}

impl PowLookupInput {
    pub fn expand(self) -> proc_macro2::TokenStream {
        let base = &self.base;
        let max_exp = self.max_exp;

        match self.output_type {
            OutputType::Rational => self.expand_rational(base, -max_exp, max_exp),
            OutputType::Float => self.expand_float(base, -max_exp, max_exp),
        }
    }

    fn expand_rational(
        &self,
        base: &Expr,
        range_start: i32,
        range_end: i32,
    ) -> proc_macro2::TokenStream {
        let mut match_arms = Vec::new();

        for exp in range_start..=range_end {
            let (num, den) = if exp >= 0 {
                // For positive exponents: (base^exp, 1)
                (base_to_power(base, exp), quote! { 1 })
            } else {
                // For negative exponents: (1, base^|exp|)
                (quote! { 1 }, base_to_power(base, -exp))
            };

            match_arms.push(quote! {
                #exp => (#num, #den),
            });
        }

        // Add default case
        match_arms.push(quote! {
            _ => (1, 1), // fallback for out-of-range values
        });

        let func_name = &self.name;

        quote! {
            pub const fn #func_name(exp: i32) -> (i128, i128) {
                match exp {
                    #(#match_arms)*
                }
            }
        }
    }

    fn expand_float(
        &self,
        base: &Expr,
        range_start: i32,
        range_end: i32,
    ) -> proc_macro2::TokenStream {
        let mut match_arms = Vec::new();

        for exp in range_start..=range_end {
            let value = if exp >= 0 {
                // For positive exponents: base^exp
                base_to_power_float(base, exp)
            } else {
                // For negative exponents: 1.0 / base^|exp|
                let power = base_to_power_float(base, -exp);
                quote! { 1.0 / (#power) }
            };

            match_arms.push(quote! {
                #exp => #value,
            });
        }

        // Add default case
        match_arms.push(quote! {
            _ => 1.0, // fallback for out-of-range values
        });

        let func_name = &self.name;

        quote! {
            pub const fn #func_name(exp: i32) -> f64 {
                match exp {
                    #(#match_arms)*
                }
            }
        }
    }
}

/// Convert a base expression to a power expression for rational numbers
fn base_to_power(base: &Expr, exp: i32) -> proc_macro2::TokenStream {
    match exp {
        0 => quote! { 1 },
        1 => quote! { #base },
        _ => {
            let mut result = quote! { #base };
            for _ in 1..exp {
                result = quote! { #result * #base };
            }
            result
        }
    }
}

/// Convert a base expression to a power expression for float numbers
fn base_to_power_float(base: &Expr, exp: i32) -> proc_macro2::TokenStream {
    match exp {
        0 => quote! { 1.0 },
        1 => quote! { #base },
        _ => {
            let mut result = quote! { #base };
            for _ in 1..exp {
                result = quote! { #result * #base };
            }
            result
        }
    }
}

/// Special macro for π exponentiation with rational approximation
/// Syntax: pow_pi_lookup!(pow_pi, 8, rational)
pub struct PiPowLookupInput {
    pub name: Ident,
    pub max_exp: i32,
    pub output_type: OutputType,
}

impl Parse for PiPowLookupInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse: <name>, <max_exp>, <output_type>
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let max_exp: syn::LitInt = input.parse()?;
        input.parse::<Token![,]>()?;

        let type_ident: Ident = input.parse()?;
        let output_type = match type_ident.to_string().as_str() {
            "rational" => OutputType::Rational,
            "float" => OutputType::Float,
            _ => {
                return Err(syn::Error::new_spanned(
                    &type_ident,
                    "Expected 'rational' or 'float'",
                ))
            }
        };

        Ok(PiPowLookupInput {
            name,
            max_exp: max_exp.base10_parse()?,
            output_type,
        })
    }
}

impl PiPowLookupInput {
    pub fn expand(self) -> proc_macro2::TokenStream {
        let max_exp = self.max_exp;

        match self.output_type {
            OutputType::Rational => self.expand_pi_rational(-max_exp, max_exp),
            OutputType::Float => self.expand_pi_float(-max_exp, max_exp),
        }
    }

    fn expand_pi_rational(&self, range_start: i32, range_end: i32) -> proc_macro2::TokenStream {
        let mut match_arms = Vec::new();

        for exp in range_start..=range_end {
            let (num, den) = if exp >= 0 {
                // For positive exponents: (355^exp, 113^exp)
                let num_val = 355_i128.pow(exp as u32);
                let den_val = 113_i128.pow(exp as u32);
                (quote! { #num_val }, quote! { #den_val })
            } else {
                // For negative exponents: (113^|exp|, 355^|exp|)
                let num_val = 113_i128.pow((-exp) as u32);
                let den_val = 355_i128.pow((-exp) as u32);
                (quote! { #num_val }, quote! { #den_val })
            };

            match_arms.push(quote! {
                #exp => (#num, #den),
            });
        }

        // Add default case
        match_arms.push(quote! {
            _ => (1, 1), // fallback for out-of-range values
        });

        let func_name = &self.name;

        quote! {
            pub const fn #func_name(exp: i32) -> (i128, i128) {
                match exp {
                    #(#match_arms)*
                }
            }
        }
    }

    fn expand_pi_float(&self, range_start: i32, range_end: i32) -> proc_macro2::TokenStream {
        let mut match_arms = Vec::new();

        for exp in range_start..=range_end {
            let value = if exp >= 0 {
                // For positive exponents: π^exp
                if exp == 0 {
                    quote! { 1.0 }
                } else if exp == 1 {
                    quote! { core::f64::consts::PI }
                } else {
                    let mut result = quote! { core::f64::consts::PI };
                    for _ in 1..exp {
                        result = quote! { #result * core::f64::consts::PI };
                    }
                    result
                }
            } else {
                // For negative exponents: 1.0 / π^|exp|
                let abs_exp = -exp;
                if abs_exp == 1 {
                    quote! { 1.0 / core::f64::consts::PI }
                } else {
                    let mut result = quote! { core::f64::consts::PI };
                    for _ in 1..abs_exp {
                        result = quote! { #result * core::f64::consts::PI };
                    }
                    quote! { 1.0 / (#result) }
                }
            };

            match_arms.push(quote! {
                #exp => #value,
            });
        }

        // Add default case
        match_arms.push(quote! {
            _ => 1.0, // fallback for out-of-range values
        });

        let func_name = &self.name;

        quote! {
            pub const fn #func_name(exp: i32) -> f64 {
                match exp {
                    #(#match_arms)*
                }
            }
        }
    }
}
