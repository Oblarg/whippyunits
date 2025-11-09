use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, Ident, Token, Type};
use whippyunits_core::{Dimension, get_unit_info};

/// Input for the value! macro
/// Syntax: value!(quantity, unit) or value!(quantity, unit, type) or value!(quantity, unit, type, brand)
pub struct ValueMacroInput {
    quantity: Expr,
    unit: Ident,
    storage_type: Option<Type>,
    brand_type: Option<Type>,
}

impl Parse for ValueMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let quantity: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let unit: Ident = input.parse()?;
        
        let storage_type = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        
        let brand_type = if storage_type.is_some() && input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        
        Ok(ValueMacroInput {
            quantity,
            unit,
            storage_type,
            brand_type,
        })
    }
}

impl ValueMacroInput {
    pub fn expand(self) -> TokenStream {
        let quantity = &self.quantity;
        let unit_ident = &self.unit;
        let unit_name = unit_ident.to_string();
        
        // Check if this is an affine unit
        let is_affine = if let Some(unit_info) = get_unit_info(&unit_name) {
            unit_info.affine_offset != 0.0
        } else {
            false
        };
        
        if is_affine {
            // Handle affine unit: get value in storage unit, then subtract offset
            let unit_info = get_unit_info(&unit_name).unwrap();
            let affine_offset = unit_info.affine_offset;
            
            // Find the storage unit (same scale, no offset, conversion_factor == 1.0)
            let storage_unit_symbol = if let Some((unit, dimension)) = Dimension::find_unit_by_symbol(&unit_name) {
                // Find a storage unit with the same scale
                if let Some(storage_unit) = dimension.units.iter().find(|u| {
                    u.scale == unit.scale
                        && u.conversion_factor == 1.0
                        && u.affine_offset == 0.0
                }) {
                    storage_unit.symbols[0]
                } else {
                    // Fallback: use the first symbol of the first storage unit in the dimension
                    dimension.units.iter()
                        .find(|u| u.conversion_factor == 1.0 && u.affine_offset == 0.0)
                        .map(|u| u.symbols[0])
                        .unwrap_or("K") // Fallback to K for temperature
                }
            } else {
                "K" // Fallback
            };
            
            let storage_unit_ident = Ident::new(storage_unit_symbol, unit_ident.span());
            
            // Determine the storage type and rescale function
            let (storage_type_ty, rescale_fn) = if let Some(ref ty) = self.storage_type {
                let ty_str = quote!(#ty).to_string();
                match ty_str.as_str() {
                    "f32" => (quote!(f32), quote!(rescale_f32)),
                    "i8" => (quote!(i8), quote!(rescale_i8)),
                    "i16" => (quote!(i16), quote!(rescale_i16)),
                    "i32" => (quote!(i32), quote!(rescale_i32)),
                    "i64" => (quote!(i64), quote!(rescale_i64)),
                    "i128" => (quote!(i128), quote!(rescale_i128)),
                    "u8" => (quote!(u8), quote!(rescale_u8)),
                    "u16" => (quote!(u16), quote!(rescale_u16)),
                    "u32" => (quote!(u32), quote!(rescale_u32)),
                    "u64" => (quote!(u64), quote!(rescale_u64)),
                    "u128" => (quote!(u128), quote!(rescale_u128)),
                    _ => (quote!(f64), quote!(rescale)),
                }
            } else {
                (quote!(f64), quote!(rescale))
            };
            
            // Generate: (rescale(quantity) as unit!(storage_unit, T)).unsafe_value - offset
            if let Some(brand_type) = &self.brand_type {
                quote! {
                    {
                        let storage_value = (whippyunits::api::#rescale_fn(#quantity) as whippyunits::unit!(#storage_unit_ident, #storage_type_ty, #brand_type)).unsafe_value;
                        (storage_value as f64 - #affine_offset) as #storage_type_ty
                    }
                }
            } else {
                quote! {
                    {
                        let storage_value = (whippyunits::api::#rescale_fn(#quantity) as whippyunits::unit!(#storage_unit_ident, #storage_type_ty)).unsafe_value;
                        (storage_value as f64 - #affine_offset) as #storage_type_ty
                    }
                }
            }
        } else {
            // Normal unit: use existing logic
            let (storage_type_ty, rescale_fn) = if let Some(ref ty) = self.storage_type {
                let ty_str = quote!(#ty).to_string();
                match ty_str.as_str() {
                    "f32" => (quote!(f32), quote!(rescale_f32)),
                    "i8" => (quote!(i8), quote!(rescale_i8)),
                    "i16" => (quote!(i16), quote!(rescale_i16)),
                    "i32" => (quote!(i32), quote!(rescale_i32)),
                    "i64" => (quote!(i64), quote!(rescale_i64)),
                    "i128" => (quote!(i128), quote!(rescale_i128)),
                    "u8" => (quote!(u8), quote!(rescale_u8)),
                    "u16" => (quote!(u16), quote!(rescale_u16)),
                    "u32" => (quote!(u32), quote!(rescale_u32)),
                    "u64" => (quote!(u64), quote!(rescale_u64)),
                    "u128" => (quote!(u128), quote!(rescale_u128)),
                    _ => (quote!(#ty), quote!(rescale)),
                }
            } else {
                (quote!(f64), quote!(rescale))
            };
            
            if let Some(brand_type) = &self.brand_type {
                quote! {
                    (whippyunits::api::#rescale_fn(#quantity) as whippyunits::unit!(#unit_ident, #storage_type_ty, #brand_type)).unsafe_value
                }
            } else {
                quote! {
                    (whippyunits::api::#rescale_fn(#quantity) as whippyunits::unit!(#unit_ident, #storage_type_ty)).unsafe_value
                }
            }
        }
    }
}

