use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Token, LitInt};
use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Star, Slash, Caret};

// Parse dimension expressions like "Length / Time" or "Mass * Length^2 / Time^2"
pub enum DimensionExpr {
    Dimension(Ident),
    Mul(Box<DimensionExpr>, Box<DimensionExpr>),
    Div(Box<DimensionExpr>, Box<DimensionExpr>),
    Pow(Box<DimensionExpr>, LitInt),
}

impl Parse for DimensionExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut left = Self::parse_factor(input)?;
        
        while input.peek(Slash) {
            let _slash: Slash = input.parse()?;
            let right = Self::parse_factor(input)?;
            left = DimensionExpr::Div(Box::new(left), Box::new(right));
        }
        
        Ok(left)
    }
}

impl DimensionExpr {
    fn parse_factor(input: ParseStream) -> Result<Self> {
        let mut left = Self::parse_power(input)?;
        
        while input.peek(Star) {
            let _star: Star = input.parse()?;
            let right = Self::parse_power(input)?;
            left = DimensionExpr::Mul(Box::new(left), Box::new(right));
        }
        
        Ok(left)
    }
    
    fn parse_power(input: ParseStream) -> Result<Self> {
        let base = Self::parse_atom(input)?;
        
        if input.peek(Caret) {
            let _caret: Caret = input.parse()?;
            let exponent: LitInt = input.parse()?;
            Ok(DimensionExpr::Pow(Box::new(base), exponent))
        } else {
            Ok(base)
        }
    }
    
    fn parse_atom(input: ParseStream) -> Result<Self> {
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            content.parse()
        } else {
            let ident: Ident = input.parse()?;
            Ok(DimensionExpr::Dimension(ident))
        }
    }
    
    // Evaluate the expression to get dimension exponents
    fn evaluate(&self) -> (i8, i8, i8) {
        match self {
            DimensionExpr::Dimension(ident) => {
                let name = ident.to_string().to_lowercase();
                match name.as_str() {
                    "mass" => (1, 0, 0),
                    "length" => (0, 1, 0),
                    "time" => (0, 0, 1),
                    _ => panic!("Unsupported dimension: {}", name)
                }
            },
            DimensionExpr::Mul(a, b) => {
                let (ma, la, ta) = a.evaluate();
                let (mb, lb, tb) = b.evaluate();
                (ma + mb, la + lb, ta + tb)
            },
            DimensionExpr::Div(a, b) => {
                let (ma, la, ta) = a.evaluate();
                let (mb, lb, tb) = b.evaluate();
                (ma - mb, la - lb, ta - tb)
            },
            DimensionExpr::Pow(base, exp) => {
                let (m, l, t) = base.evaluate();
                let exp_val: i8 = exp.base10_parse().unwrap();
                (m * exp_val, l * exp_val, t * exp_val)
            }
        }
    }
}

pub struct DefineGenericDimensionInput {
    pub trait_name: Ident,
    pub _comma: Token![,],
    pub dimension_expr: DimensionExpr,
}

impl Parse for DefineGenericDimensionInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(DefineGenericDimensionInput {
            trait_name: input.parse()?,
            _comma: input.parse()?,
            dimension_expr: input.parse()?,
        })
    }
}

impl DefineGenericDimensionInput {
    pub fn expand(self) -> TokenStream {
        let trait_name = &self.trait_name;
        let (mass_exp, length_exp, time_exp) = self.dimension_expr.evaluate();
        
        // Generate the trait definition
        let trait_def = quote! {
            pub trait #trait_name {
                type Unit;
            }
        };
        
        // Generate the implementation with the calculated exponents
        let impl_block = self.generate_impl(mass_exp, length_exp, time_exp);
        
        quote! {
            #trait_def
            
            #impl_block
        }
    }
    
    fn generate_impl(&self, mass_exp: i8, length_exp: i8, time_exp: i8) -> TokenStream {
        let trait_name = &self.trait_name;
        
        // Determine which scale parameters we need
        let mut const_decls = Vec::new();
        let mut generic_params = Vec::new();
        
        // Mass dimension
        if mass_exp != 0 {
            let const_ident = Ident::new("MASS_SCALE_P10", proc_macro2::Span::call_site());
            const_decls.push(quote! { const #const_ident: i8 });
            generic_params.push(quote! { #mass_exp }); // MASS_EXPONENT
            generic_params.push(quote! { #const_ident }); // MASS_SCALE_P10
        } else {
            generic_params.push(quote! { #mass_exp }); // MASS_EXPONENT
            generic_params.push(quote! { 0_i8 }); // MASS_SCALE_P10
        }
        
        // Length dimension
        if length_exp != 0 {
            let const_ident = Ident::new("LENGTH_SCALE_P10", proc_macro2::Span::call_site());
            const_decls.push(quote! { const #const_ident: i8 });
            generic_params.push(quote! { #length_exp }); // LENGTH_EXPONENT
            generic_params.push(quote! { #const_ident }); // LENGTH_SCALE_P10
        } else {
            generic_params.push(quote! { #length_exp }); // LENGTH_EXPONENT
            generic_params.push(quote! { 0_i8 }); // LENGTH_SCALE_P10
        }
        
        // Time dimension
        if time_exp != 0 {
            let const_p2 = Ident::new("TIME_SCALE_P2", proc_macro2::Span::call_site());
            let const_p3 = Ident::new("TIME_SCALE_P3", proc_macro2::Span::call_site());
            let const_p5 = Ident::new("TIME_SCALE_P5", proc_macro2::Span::call_site());
            const_decls.push(quote! { 
                const #const_p2: i8,
                const #const_p3: i8,
                const #const_p5: i8
            });
            generic_params.push(quote! { #time_exp }); // TIME_EXPONENT
            generic_params.push(quote! { #const_p2 }); // TIME_SCALE_P2
            generic_params.push(quote! { #const_p3 }); // TIME_SCALE_P3
            generic_params.push(quote! { #const_p5 }); // TIME_SCALE_P5
        } else {
            generic_params.push(quote! { #time_exp }); // TIME_EXPONENT
            generic_params.push(quote! { 0_i8 }); // TIME_SCALE_P2
            generic_params.push(quote! { 0_i8 }); // TIME_SCALE_P3
            generic_params.push(quote! { 0_i8 }); // TIME_SCALE_P5
        }
        
        quote! {
            impl <
                #(#const_decls),*
            > #trait_name for whippyunits::quantity_type::Quantity<
                #(#generic_params),*
            > {
                type Unit = Self;
            }
        }
    }
}
