use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::{Caret, Comma, Slash, Star};
use syn::{Ident, LitInt, Token};
use whippyunits_default_dimensions::{lookup_dimension_by_name, lookup_dimension_by_symbol, get_all_dimension_names, get_all_dimension_symbols};

// Note: lookup_dimension_by_symbol is now imported from whippyunits_default_dimensions

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
                let supported_names: Vec<&str> = get_all_dimension_names();
                let supported_symbols: Vec<&str> = get_all_dimension_symbols();

                panic!("Unsupported dimension: '{}'. Supported dimension names: {}. Supported dimension symbols: {}", 
                       name_or_symbol,
                       supported_names.join(", "),
                       supported_symbols.join(", "));
            }
            DimensionExpr::Mul(a, b) => {
                let (ma, la, ta, ca, tempa, aa, luma, anga) = a.evaluate();
                let (mb, lb, tb, cb, tempb, ab, lumb, angb) = b.evaluate();
                (
                    ma + mb,
                    la + lb,
                    ta + tb,
                    ca + cb,
                    tempa + tempb,
                    aa + ab,
                    luma + lumb,
                    anga + angb,
                )
            }
            DimensionExpr::Div(a, b) => {
                let (ma, la, ta, ca, tempa, aa, luma, anga) = a.evaluate();
                let (mb, lb, tb, cb, tempb, ab, lumb, angb) = b.evaluate();
                (
                    ma - mb,
                    la - lb,
                    ta - tb,
                    ca - cb,
                    tempa - tempb,
                    aa - ab,
                    luma - lumb,
                    anga - angb,
                )
            }
            DimensionExpr::Pow(base, exp) => {
                let (m, l, t, c, temp, a, lum, ang) = base.evaluate();
                let exp_val: i16 = exp.base10_parse().unwrap();
                (
                    m * exp_val,
                    l * exp_val,
                    t * exp_val,
                    c * exp_val,
                    temp * exp_val,
                    a * exp_val,
                    lum * exp_val,
                    ang * exp_val,
                )
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

        // Generate documentation structs for dimension identifiers used in expressions
        let doc_structs = Self::generate_dimension_documentation(&self.dimension_exprs);

        // Generate the trait definition
        let trait_def = quote! {
            pub trait #trait_name {
                type Unit;
            }
        };

        // Generate implementations for each dimension expression
        let impl_blocks: Vec<TokenStream> = self
            .dimension_exprs
            .iter()
            .map(|expr| {
                let (
                    mass_exp,
                    length_exp,
                    time_exp,
                    current_exp,
                    temp_exp,
                    amount_exp,
                    lum_exp,
                    angle_exp,
                ) = expr.evaluate();
                self.generate_impl(
                    mass_exp,
                    length_exp,
                    time_exp,
                    current_exp,
                    temp_exp,
                    amount_exp,
                    lum_exp,
                    angle_exp,
                )
            })
            .collect();

        quote! {
            {
                #doc_structs
            }
            
            #trait_def

            #(#impl_blocks)*
        }
    }

    fn generate_impl(
        &self,
        mass_exp: i16,
        length_exp: i16,
        time_exp: i16,
        current_exp: i16,
        temp_exp: i16,
        amount_exp: i16,
        lum_exp: i16,
        angle_exp: i16,
    ) -> TokenStream {
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
                whippyunits::quantity_type::Scale<whippyunits::quantity_type::_2<SCALE_P2>, whippyunits::quantity_type::_3<SCALE_P3>, whippyunits::quantity_type::_5<SCALE_P5>, whippyunits::quantity_type::_Pi<SCALE_PI>>,
                whippyunits::quantity_type::Dimension<whippyunits::quantity_type::_M<#mass_exp>, whippyunits::quantity_type::_L<#length_exp>, whippyunits::quantity_type::_T<#time_exp>, whippyunits::quantity_type::_I<#current_exp>, whippyunits::quantity_type::_Θ<#temp_exp>, whippyunits::quantity_type::_N<#amount_exp>, whippyunits::quantity_type::_J<#lum_exp>, whippyunits::quantity_type::_A<#angle_exp>>,
                T
            > {
                type Unit = Self;
            }
        }
    }

    /// Generate documentation structs for dimension identifiers used in expressions
    fn generate_dimension_documentation(dimension_exprs: &Punctuated<DimensionExpr, Comma>) -> TokenStream {
        let mut doc_structs = Vec::new();

        // Generate documentation for each identifier occurrence (no filtering)
        for expr in dimension_exprs {
            Self::collect_and_generate_dimension_docs(expr, &mut doc_structs);
        }

        quote! {
            #(#doc_structs)*
        }
    }

    /// Recursively collect dimension identifiers and generate documentation for each occurrence
    fn collect_and_generate_dimension_docs(expr: &DimensionExpr, doc_structs: &mut Vec<TokenStream>) {
        match expr {
            DimensionExpr::Dimension(ident) => {
                // Generate documentation for this specific occurrence
                if let Some(doc_struct) = Self::generate_single_dimension_doc(ident) {
                    doc_structs.push(doc_struct);
                }
            }
            DimensionExpr::Mul(a, b) => {
                Self::collect_and_generate_dimension_docs(a, doc_structs);
                Self::collect_and_generate_dimension_docs(b, doc_structs);
            }
            DimensionExpr::Div(a, b) => {
                Self::collect_and_generate_dimension_docs(a, doc_structs);
                Self::collect_and_generate_dimension_docs(b, doc_structs);
            }
            DimensionExpr::Pow(base, _) => {
                Self::collect_and_generate_dimension_docs(base, doc_structs);
            }
        }
    }

    /// Generate documentation for a single dimension identifier
    fn generate_single_dimension_doc(identifier: &Ident) -> Option<TokenStream> {
        let dimension_name = identifier.to_string();
        let doc_comment = Self::generate_dimension_doc_comment(&dimension_name);
        
        // Create a new identifier with the same span as the original
        let doc_ident = syn::Ident::new(&dimension_name, identifier.span());
        
        // Get the corresponding dimension trait type
        let trait_type = Self::get_dimension_trait_type(&dimension_name)?;

        // Use quote_spanned to preserve the span information for hover
        // Create a hand-rolled trait alias in a throwaway const block for hover documentation
        Some(quote! {
            const _: () = {
                #doc_comment
                #[allow(dead_code)]
                trait #doc_ident: #trait_type {}
                
                impl<U: #trait_type> #doc_ident for U {}
            };
        })
    }

    /// Generate documentation comment for a dimension
    fn generate_dimension_doc_comment(dimension_name: &str) -> TokenStream {
        let doc_text = Self::get_dimension_documentation_text(dimension_name);
        quote! {
            #[doc = #doc_text]
        }
    }

    /// Get documentation text for a dimension
    #[allow(mixed_script_confusables)]
    fn get_dimension_documentation_text(dimension_name: &str) -> String {
        // Map dimension names/symbols to their documentation
        match dimension_name {
            // Atomic dimensions - full names
            "Mass" => "Atomic dimension: Mass (M) - The fundamental dimension of mass in the SI system".to_string(),
            "Length" => "Atomic dimension: Length (L) - The fundamental dimension of length in the SI system".to_string(),
            "Time" => "Atomic dimension: Time (T) - The fundamental dimension of time in the SI system".to_string(),
            "Current" => "Atomic dimension: Current (I) - The fundamental dimension of electric current in the SI system".to_string(),
            "Temperature" => "Atomic dimension: Temperature (Θ) - The fundamental dimension of thermodynamic temperature in the SI system".to_string(),
            "Amount" => "Atomic dimension: Amount (N) - The fundamental dimension of amount of substance in the SI system".to_string(),
            "Luminosity" => "Atomic dimension: Luminosity (J) - The fundamental dimension of luminous intensity in the SI system".to_string(),
            "Angle" => "Atomic dimension: Angle (A) - The fundamental dimension of plane angle in the SI system".to_string(),
            
            // Atomic dimensions - symbols
            "M" => "Atomic dimension: Mass (M) - The fundamental dimension of mass in the SI system".to_string(),
            "L" => "Atomic dimension: Length (L) - The fundamental dimension of length in the SI system".to_string(),
            "T" => "Atomic dimension: Time (T) - The fundamental dimension of time in the SI system".to_string(),
            "I" => "Atomic dimension: Current (I) - The fundamental dimension of electric current in the SI system".to_string(),
            "Θ" => "Atomic dimension: Temperature (Θ) - The fundamental dimension of thermodynamic temperature in the SI system".to_string(),
            "N" => "Atomic dimension: Amount (N) - The fundamental dimension of amount of substance in the SI system".to_string(),
            "J" => "Atomic dimension: Luminosity (J) - The fundamental dimension of luminous intensity in the SI system".to_string(),
            "A" => "Atomic dimension: Angle (A) - The fundamental dimension of plane angle in the SI system".to_string(),
            
            _ => format!("Dimension: {} - Custom dimension expression", dimension_name),
        }
    }

    /// Get the corresponding dimension trait type for a dimension name/symbol
    fn get_dimension_trait_type(dimension_name: &str) -> Option<TokenStream> {
        // Map dimension names/symbols to their corresponding trait types
        match dimension_name {
            // Atomic dimensions - full names
            "Mass" => Some(quote! { whippyunits::dimension_traits::Mass }),
            "Length" => Some(quote! { whippyunits::dimension_traits::Length }), 
            "Time" => Some(quote! { whippyunits::dimension_traits::Time }),
            "Current" => Some(quote! { whippyunits::dimension_traits::Current }),
            "Temperature" => Some(quote! { whippyunits::dimension_traits::Temperature }),
            "Amount" => Some(quote! { whippyunits::dimension_traits::Amount }),
            "Luminosity" => Some(quote! { whippyunits::dimension_traits::Luminosity }),
            "Angle" => Some(quote! { whippyunits::dimension_traits::Angle }),
            
            // Atomic dimensions - symbols
            "M" => Some(quote! { whippyunits::dimension_traits::Mass }),
            "L" => Some(quote! { whippyunits::dimension_traits::Length }),
            "T" => Some(quote! { whippyunits::dimension_traits::Time }), 
            "I" => Some(quote! { whippyunits::dimension_traits::Current }),
            "Θ" => Some(quote! { whippyunits::dimension_traits::Temperature }),
            "N" => Some(quote! { whippyunits::dimension_traits::Amount }),
            "J" => Some(quote! { whippyunits::dimension_traits::Luminosity }),
            "A" => Some(quote! { whippyunits::dimension_traits::Angle }),
            
            _ => None, // Unknown dimension
        }
    }
}
