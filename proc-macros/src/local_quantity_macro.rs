use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type, parse::{Parse, ParseStream, Result}};
use syn::token::Comma;
use whippyunits_default_dimensions::{
    scale_type_to_unit_symbol, 
    get_unit_dimensions, 
    dimension_exponents_to_unit_expression
};

/// Input for the local quantity macro
/// This takes a unit identifier, local scale parameters, and optional storage type
pub struct LocalQuantityMacroInput {
    pub unit_ident: Ident,
    pub mass_scale: Ident,
    pub length_scale: Ident,
    pub time_scale: Ident,
    pub current_scale: Ident,
    pub temperature_scale: Ident,
    pub amount_scale: Ident,
    pub luminosity_scale: Ident,
    pub angle_scale: Ident,
    pub storage_type: Option<Type>,
}

impl Parse for LocalQuantityMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the unit identifier first
        let unit_ident: Ident = input.parse()?;
        
        // Expect a comma
        let _comma: Comma = input.parse()?;
        
        // Parse the local scale parameters
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
        
        // Check if there's a comma followed by a storage type parameter
        let storage_type = if input.peek(Comma) {
            let _comma: Comma = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };
        
        Ok(LocalQuantityMacroInput {
            unit_ident,
            mass_scale,
            length_scale,
            time_scale,
            current_scale,
            temperature_scale,
            amount_scale,
            luminosity_scale,
            angle_scale,
            storage_type,
        })
    }
}

impl LocalQuantityMacroInput {
    pub fn expand(self) -> TokenStream {
        let unit_name = self.unit_ident.to_string();
        
        // Use the specified storage type or default to f64
        let storage_type = self.storage_type.clone().unwrap_or_else(|| {
            syn::parse_str::<Type>("f64").unwrap()
        });
        
        // Get the actual unit symbols for each scale type before moving the values
        let mass_base = scale_type_to_unit_symbol(&self.mass_scale.to_string()).unwrap_or_else(|| "g".to_string());
        let length_base = scale_type_to_unit_symbol(&self.length_scale.to_string()).unwrap_or_else(|| "m".to_string());
        let time_base = scale_type_to_unit_symbol(&self.time_scale.to_string()).unwrap_or_else(|| "s".to_string());
        let current_base = scale_type_to_unit_symbol(&self.current_scale.to_string()).unwrap_or_else(|| "A".to_string());
        let temperature_base = scale_type_to_unit_symbol(&self.temperature_scale.to_string()).unwrap_or_else(|| "K".to_string());
        let amount_base = scale_type_to_unit_symbol(&self.amount_scale.to_string()).unwrap_or_else(|| "mol".to_string());
        let luminosity_base = scale_type_to_unit_symbol(&self.luminosity_scale.to_string()).unwrap_or_else(|| "cd".to_string());
        let angle_base = scale_type_to_unit_symbol(&self.angle_scale.to_string()).unwrap_or_else(|| "rad".to_string());
        
        // Use data-driven approach to map unit identifiers to their dimensions
        if let Some(dimensions) = get_unit_dimensions(&unit_name) {
            // Check if it's a simple base unit (single dimension = 1, others = 0)
            if let Some(scale_ident) = self.get_scale_for_dimensions(dimensions) {
                quote! { whippyunits::default_declarators::#scale_ident<#storage_type> }
            } else {
                // It's a compound unit - generate the unit expression
                let base_units = [
                    (mass_base.as_str(), mass_base.as_str()),
                    (length_base.as_str(), length_base.as_str()), 
                    (time_base.as_str(), time_base.as_str()),
                    (current_base.as_str(), current_base.as_str()),
                    (temperature_base.as_str(), temperature_base.as_str()),
                    (amount_base.as_str(), amount_base.as_str()),
                    (luminosity_base.as_str(), luminosity_base.as_str()),
                    (angle_base.as_str(), angle_base.as_str())
                ];
                
                let unit_expr = dimension_exponents_to_unit_expression(dimensions, &base_units);
                let unit_expr_ident = syn::parse_str::<Ident>(&unit_expr).unwrap_or_else(|_| {
                    // If parsing fails, fall back to the original unit
                    self.unit_ident.clone()
                });
                
                quote! { whippyunits::unit!(#unit_expr_ident, #storage_type) }
            }
        } else {
            // For unknown units, fall back to the original unit type
            let unit_ident = self.unit_ident;
            quote! { whippyunits::unit!(#unit_ident, #storage_type) }
        }
    }
    
    /// Get the appropriate scale identifier for given dimension exponents
    /// Returns Some(scale_ident) if it's a simple base unit, None for compound units
    fn get_scale_for_dimensions(&self, dimensions: (i16, i16, i16, i16, i16, i16, i16, i16)) -> Option<Ident> {
        match dimensions {
            (1, 0, 0, 0, 0, 0, 0, 0) => Some(self.mass_scale.clone()),
            (0, 1, 0, 0, 0, 0, 0, 0) => Some(self.length_scale.clone()),
            (0, 0, 1, 0, 0, 0, 0, 0) => Some(self.time_scale.clone()),
            (0, 0, 0, 1, 0, 0, 0, 0) => Some(self.current_scale.clone()),
            (0, 0, 0, 0, 1, 0, 0, 0) => Some(self.temperature_scale.clone()),
            (0, 0, 0, 0, 0, 1, 0, 0) => Some(self.amount_scale.clone()),
            (0, 0, 0, 0, 0, 0, 1, 0) => Some(self.luminosity_scale.clone()),
            (0, 0, 0, 0, 0, 0, 0, 1) => Some(self.angle_scale.clone()),
            _ => None, // Compound unit
        }
    }
}