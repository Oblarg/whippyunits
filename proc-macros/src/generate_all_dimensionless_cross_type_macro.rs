use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

const ALL_TYPES: [&str; 14] = [
    "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
    "usize",
];

/// Input for the generate_all_dimensionless_cross_type macro.
/// Usage: generate_all_dimensionless_cross_type!()
pub struct AllDimensionlessCrossTypeInput;

impl Parse for AllDimensionlessCrossTypeInput {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(AllDimensionlessCrossTypeInput)
    }
}

impl AllDimensionlessCrossTypeInput {
    pub fn expand(self) -> TokenStream {
        let mut expansions = Vec::new();

        for source in &ALL_TYPES {
            for target in &ALL_TYPES {
                if source == target {
                    continue;
                }

                let source_ident = syn::parse_str::<syn::Ident>(source).unwrap();
                let target_ident = syn::parse_str::<syn::Ident>(target).unwrap();

                expansions.push(quote! {
                    define_from_dimensionless_cross_type!(#source_ident, #target_ident);
                });
            }
        }

        quote! { #(#expansions)* }
    }
}
