use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Token, LitInt};
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Star, Slash, Caret, Comma};
use whippyunits_default_dimensions::{lookup_dimension_by_name, DIMENSION_LOOKUP};

/// Look up dimension information by symbol
/// 
/// Returns the dimension info if found, or None if the symbol is not recognized.
fn lookup_dimension_by_symbol(symbol: &str) -> Option<&'static whippyunits_default_dimensions::DimensionInfo> {
    DIMENSION_LOOKUP.iter().find(|info| {
        if let Some(dim_symbol) = info.symbol {
            dim_symbol == symbol
        } else {
            false
        }
    })
}

// Parse dimension expressions like "Length / Time", "L / T", or "Mass * Length^2 / Time^2", "M * L^2 / T^2"
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
    fn evaluate(&self) -> (i16, i16, i16, i16, i16, i16, i16, i16) {
        match self {
            DimensionExpr::Dimension(ident) => {
                let name_or_symbol = ident.to_string();
                
                // First try to look up by name
                if let Some(dim_info) = lookup_dimension_by_name(&name_or_symbol) {
                    return dim_info.exponents;
                }
                
                // If not found by name, try to look up by symbol
                if let Some(dim_info) = lookup_dimension_by_symbol(&name_or_symbol) {
                    return dim_info.exponents;
                }
                
                // If neither works, generate a helpful error message
                let supported_names: Vec<&str> = DIMENSION_LOOKUP
                    .iter()
                    .map(|info| info.name)
                    .collect();
                let supported_symbols: Vec<&str> = DIMENSION_LOOKUP
                    .iter()
                    .filter_map(|info| info.symbol)
                    .collect();
                
                panic!("Unsupported dimension: '{}'. Supported dimension names: {}. Supported dimension symbols: {}", 
                       name_or_symbol, 
                       supported_names.join(", "),
                       supported_symbols.join(", "));
            },
            DimensionExpr::Mul(a, b) => {
                let (ma, la, ta, ca, tempa, aa, luma, anga) = a.evaluate();
                let (mb, lb, tb, cb, tempb, ab, lumb, angb) = b.evaluate();
                (ma + mb, la + lb, ta + tb, ca + cb, tempa + tempb, aa + ab, luma + lumb, anga + angb)
            },
            DimensionExpr::Div(a, b) => {
                let (ma, la, ta, ca, tempa, aa, luma, anga) = a.evaluate();
                let (mb, lb, tb, cb, tempb, ab, lumb, angb) = b.evaluate();
                (ma - mb, la - lb, ta - tb, ca - cb, tempa - tempb, aa - ab, luma - lumb, anga - angb)
            },
            DimensionExpr::Pow(base, exp) => {
                let (m, l, t, c, temp, a, lum, ang) = base.evaluate();
                let exp_val: i16 = exp.base10_parse().unwrap();
                (m * exp_val, l * exp_val, t * exp_val, c * exp_val, temp * exp_val, a * exp_val, lum * exp_val, ang * exp_val)
            }
        }
    }
}

pub struct DefineGenericDimensionInput {
    pub trait_name: Ident,
    pub _comma: Token![,],
    pub dimension_exprs: Punctuated<DimensionExpr, Comma>,
}

impl Parse for DefineGenericDimensionInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(DefineGenericDimensionInput {
            trait_name: input.parse()?,
            _comma: input.parse()?,
            dimension_exprs: input.parse_terminated(DimensionExpr::parse, Token![,])?,
        })
    }
}

impl DefineGenericDimensionInput {
    pub fn expand(self) -> TokenStream {
        let trait_name = &self.trait_name;
        
        // Generate the trait definition
        let trait_def = quote! {
            pub trait #trait_name {
                type Unit;
            }
        };
        
        // Generate implementations for each dimension expression
        let impl_blocks: Vec<TokenStream> = self.dimension_exprs
            .iter()
            .map(|expr| {
                let (mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp) = expr.evaluate();
                self.generate_impl(mass_exp, length_exp, time_exp, current_exp, temp_exp, amount_exp, lum_exp, angle_exp)
            })
            .collect();
        
        quote! {
            #trait_def
            
            #(#impl_blocks)*
        }
    }
    
    fn generate_impl(&self, mass_exp: i16, length_exp: i16, time_exp: i16, current_exp: i16, temp_exp: i16, amount_exp: i16, lum_exp: i16, angle_exp: i16) -> TokenStream {
        let trait_name = &self.trait_name;
        
        // For simplicity, we'll use 0 for all scale parameters
        // In a more sophisticated implementation, we could determine scale parameters based on the dimensions
        quote! {
            impl <
                const SCALE_P2: i16,
                const SCALE_P3: i16,
                const SCALE_P5: i16,
                const SCALE_PI: i16,
                T
            > #trait_name for whippyunits::quantity_type::Quantity<
                #mass_exp, #length_exp, #time_exp, #current_exp, #temp_exp, #amount_exp, #lum_exp, #angle_exp,
                SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI,
                T
            > {
                type Unit = Self;
            }
        }
    }
}
