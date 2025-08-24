use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use proc_macro2::Literal;

use syn::token::Caret;
use syn::{Ident, LitInt, parse_macro_input};

#[proc_macro]
pub fn proc_unit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as UnitExpr);
    let output = input.expand();
    TokenStream::from(output)
}

#[derive(Debug)]
struct UnitExpr {
    terms: Vec<UnitTerm>,
}

#[derive(Debug)]
struct UnitTerm {
    unit: Ident,
    exponent: isize,
}

impl Parse for UnitExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut terms = Vec::new();
        
        // Parse first term
        terms.push(input.parse::<UnitTerm>()?);
        
        // Parse additional terms separated by *
        while input.peek(syn::token::Star) {
            let _star = input.parse::<syn::token::Star>()?;
            terms.push(input.parse::<UnitTerm>()?);
        }
        
        Ok(UnitExpr { terms })
    }
}

impl Parse for UnitTerm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let unit = input.parse::<Ident>()?;
        
        // Check for exponent (^number)
        let exponent = if input.peek(Caret) {
            let _caret = input.parse::<Caret>()?;
            let exp_lit = input.parse::<LitInt>()?;
            exp_lit.base10_parse::<isize>()?
        } else {
            1
        };
        
        Ok(UnitTerm { unit, exponent })
    }
}

impl UnitExpr {
    fn expand(self) -> proc_macro2::TokenStream {
        // Initialize all parameters to unused/default values
        let mut length_exp = 0;
        let mut length_scale = quote! { isize::MAX };
        let mut mass_exp = 0;
        let mut mass_scale = quote! { isize::MAX };
        let mut time_exp = 0;
        let mut time_p2 = quote! { isize::MAX };
        let mut time_p3 = quote! { isize::MAX };
        let mut time_p5 = quote! { isize::MAX };
        let mut time_scale_order = quote! { isize::MAX };

        // Process each term
        for term in self.terms {
            let unit_name = term.unit.to_string();
            let exp = term.exponent;
            
            match unit_name.as_str() {
                // Length units
                "mm" => {
                    length_exp += exp;
                    length_scale = quote! { -1 };
                }
                "m" => {
                    length_exp += exp;
                    length_scale = quote! { 0 };
                }
                "km" => {
                    length_exp += exp;
                    length_scale = quote! { 1 };
                }
                
                // Mass units
                "mg" => {
                    mass_exp += exp;
                    mass_scale = quote! { -1 };
                }
                "g" => {
                    mass_exp += exp;
                    mass_scale = quote! { 0 };
                }
                "kg" => {
                    mass_exp += exp;
                    mass_scale = quote! { 1 };
                }
                
                // Time units
                "ms" => {
                    time_exp += exp;
                    time_p2 = quote! { -3 };
                    time_p3 = quote! { 0 };
                    time_p5 = quote! { -3 };
                    time_scale_order = quote! { -1 };
                }
                "s" => {
                    time_exp += exp;
                    time_p2 = quote! { 0 };
                    time_p3 = quote! { 0 };
                    time_p5 = quote! { 0 };
                    time_scale_order = quote! { 0 };
                }
                "min" => {
                    time_exp += exp;
                    time_p2 = quote! { 2 };
                    time_p3 = quote! { 1 };
                    time_p5 = quote! { 1 };
                    time_scale_order = quote! { 1 };
                }
                
                _ => {
                    // Unknown unit - this will cause a compile error
                    return quote! {
                        compile_error!(concat!("Unknown unit: ", stringify!(#unit_name)))
                    };
                }
            }
        }

        // Generate the Quantity type
        let length_exp_lit = Literal::i64_unsuffixed(length_exp as i64);
        let mass_exp_lit = Literal::i64_unsuffixed(mass_exp as i64);
        let time_exp_lit = Literal::i64_unsuffixed(time_exp as i64);
        
        quote! {
            whippyunits::Quantity<
                { #length_exp_lit }, { #length_scale },
                { #mass_exp_lit }, { #mass_scale },
                { #time_exp_lit }, { #time_p2 }, { #time_p3 }, { #time_p5 }, { #time_scale_order }
            >
        }
    }
}

