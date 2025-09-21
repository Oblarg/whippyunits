use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, parse::{Parse, ParseStream, Result}};
use syn::token::Comma;
use whippyunits_default_dimensions::scale_type_to_unit_symbol;

/// Input for the local quantity macro
/// This takes a unit identifier and local scale parameters
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
        })
    }
}

impl LocalQuantityMacroInput {
    pub fn expand(self) -> TokenStream {
        let unit_name = self.unit_ident.to_string();
        
        // Get the actual unit symbols for each scale type before moving the values
        let mass_base = scale_type_to_unit_symbol(&self.mass_scale.to_string()).unwrap_or_else(|| "g".to_string());
        let length_base = scale_type_to_unit_symbol(&self.length_scale.to_string()).unwrap_or_else(|| "m".to_string());
        let time_base = scale_type_to_unit_symbol(&self.time_scale.to_string()).unwrap_or_else(|| "s".to_string());
        let current_base = scale_type_to_unit_symbol(&self.current_scale.to_string()).unwrap_or_else(|| "A".to_string());
        let temperature_base = scale_type_to_unit_symbol(&self.temperature_scale.to_string()).unwrap_or_else(|| "K".to_string());
        let amount_base = scale_type_to_unit_symbol(&self.amount_scale.to_string()).unwrap_or_else(|| "mol".to_string());
        let luminosity_base = scale_type_to_unit_symbol(&self.luminosity_scale.to_string()).unwrap_or_else(|| "cd".to_string());
        let angle_base = scale_type_to_unit_symbol(&self.angle_scale.to_string()).unwrap_or_else(|| "rad".to_string());
        
        // Extract the scale identifiers for use in quote
        let mass_scale = self.mass_scale;
        let length_scale = self.length_scale;
        let time_scale = self.time_scale;
        let current_scale = self.current_scale;
        let temperature_scale = self.temperature_scale;
        let amount_scale = self.amount_scale;
        let luminosity_scale = self.luminosity_scale;
        let angle_scale = self.angle_scale;
        
        // Create identifiers for the base units
        let mass_base_ident = syn::parse_str::<Ident>(&mass_base).unwrap();
        let length_base_ident = syn::parse_str::<Ident>(&length_base).unwrap();
        let time_base_ident = syn::parse_str::<Ident>(&time_base).unwrap();
        let current_base_ident = syn::parse_str::<Ident>(&current_base).unwrap();
        let _temperature_base_ident = syn::parse_str::<Ident>(&temperature_base).unwrap();
        let _amount_base_ident = syn::parse_str::<Ident>(&amount_base).unwrap();
        let luminosity_base_ident = syn::parse_str::<Ident>(&luminosity_base).unwrap();
        let _angle_base_ident = syn::parse_str::<Ident>(&angle_base).unwrap();
        
        // Map unit identifiers to their dimensions and return the appropriate local scale type
        match unit_name.as_str() {
            // Mass units - map to local mass scale
            "kg" | "g" | "mg" | "ug" | "ng" | "pg" | "fg" | "ag" | "zg" | "yg" |
            "Mg" | "Gg" | "Tg" | "Pg" | "Eg" | "Zg" | "Yg" => {
                quote! { whippyunits::default_declarators::#mass_scale<f64> }
            },
            // Length units - map to local length scale
            "m" | "mm" | "um" | "nm" | "pm" | "fm" | "am" | "zm" | "ym" |
            "km" | "Mm" | "Gm" | "Tm" | "Pm" | "Em" | "Zm" | "Ym" => {
                quote! { whippyunits::default_declarators::#length_scale<f64> }
            },
            // Time units - map to local time scale
            "s" | "ms" | "us" | "ns" | "ps" | "fs" | "as" | "zs" | "ys" |
            "ks" | "Ms" | "Gs" | "Ts" | "Ps" | "Es" | "Zs" | "Ys" => {
                quote! { whippyunits::default_declarators::#time_scale<f64> }
            },
            // Current units - map to local current scale
            "A" | "mA" | "uA" | "nA" | "pA" | "fA" | "aA" | "zA" | "yA" |
            "kA" | "MA" | "GA" | "TA" | "PA" | "EA" | "ZA" | "YA" => {
                quote! { whippyunits::default_declarators::#current_scale<f64> }
            },
            // Temperature units - map to local temperature scale
            "K" | "mK" | "uK" | "nK" | "pK" | "fK" | "aK" | "zK" | "yK" |
            "kK" | "MK" | "GK" | "TK" | "PK" | "EK" | "ZK" | "YK" => {
                quote! { whippyunits::default_declarators::#temperature_scale<f64> }
            },
            // Amount units - map to local amount scale
            "mol" | "mmol" | "umol" | "nmol" | "pmol" | "fmol" | "amol" | "zmol" | "ymol" |
            "kmol" | "Mmol" | "Gmol" | "Tmol" | "Pmol" | "Emol" | "Zmol" | "Ymol" => {
                quote! { whippyunits::default_declarators::#amount_scale<f64> }
            },
            // Luminosity units - map to local luminosity scale
            "cd" | "mcd" | "ucd" | "ncd" | "pcd" | "fcd" | "acd" | "zcd" | "ycd" |
            "kcd" | "Mcd" | "Gcd" | "Tcd" | "Pcd" | "Ecd" | "Zcd" | "Ycd" => {
                quote! { whippyunits::default_declarators::#luminosity_scale<f64> }
            },
            // Angle units - map to local angle scale
            "rad" | "mrad" | "urad" | "nrad" | "prad" | "frad" | "arad" | "zrad" | "yrad" |
            "krad" | "Mrad" | "Grad" | "Trad" | "Prad" | "Erad" | "Zrad" | "Yrad" => {
                quote! { whippyunits::default_declarators::#angle_scale<f64> }
            },
            // Compound units - decompose and reconstruct using local scales
            "J" => {
                // Joule = kg * m^2 / s^2
                quote! { whippyunits::unit!(#mass_base_ident * #length_base_ident^2 / #time_base_ident^2) }
            },
            "N" => {
                // Newton = kg * m / s^2
                quote! { whippyunits::unit!(#mass_base_ident * #length_base_ident / #time_base_ident^2) }
            },
            "W" => {
                // Watt = kg * m^2 / s^3
                quote! { whippyunits::unit!(#mass_base_ident * #length_base_ident^2 / #time_base_ident^3) }
            },
            "Pa" => {
                // Pascal = kg / (m * s^2)
                quote! { whippyunits::unit!(#mass_base_ident / (#length_base_ident * #time_base_ident^2)) }
            },
            "Hz" => {
                // Hertz = 1 / s
                quote! { whippyunits::unit!(1 / #time_base_ident) }
            },
            "C" => {
                // Coulomb = A * s
                quote! { whippyunits::unit!(#current_base_ident * #time_base_ident) }
            },
            "V" => {
                // Volt = kg * m^2 / (A * s^3)
                quote! { whippyunits::unit!(#mass_base_ident * #length_base_ident^2 / (#current_base_ident * #time_base_ident^3)) }
            },
            "F" => {
                // Farad = A^2 * s^4 / (kg * m^2)
                quote! { whippyunits::unit!(#current_base_ident^2 * #time_base_ident^4 / (#mass_base_ident * #length_base_ident^2)) }
            },
            "Ω" => {
                // Ohm = kg * m^2 / (A^2 * s^3)
                quote! { whippyunits::unit!(#mass_base_ident * #length_base_ident^2 / (#current_base_ident^2 * #time_base_ident^3)) }
            },
            "S" => {
                // Siemens = A^2 * s^3 / (kg * m^2)
                quote! { whippyunits::unit!(#current_base_ident^2 * #time_base_ident^3 / (#mass_base_ident * #length_base_ident^2)) }
            },
            "H" => {
                // Henry = kg * m^2 / (A^2 * s^2)
                quote! { whippyunits::unit!(#mass_base_ident * #length_base_ident^2 / (#current_base_ident^2 * #time_base_ident^2)) }
            },
            "T" => {
                // Tesla = kg / (A * s^2)
                quote! { whippyunits::unit!(#mass_base_ident / (#current_base_ident * #time_base_ident^2)) }
            },
            "Wb" => {
                // Weber = kg * m^2 / (A * s^2)
                quote! { whippyunits::unit!(#mass_base_ident * #length_base_ident^2 / (#current_base_ident * #time_base_ident^2)) }
            },
            "lm" => {
                // Lumen = cd * sr (steradian is dimensionless)
                quote! { whippyunits::unit!(#luminosity_base_ident) }
            },
            "lx" => {
                // Lux = cd / m^2
                quote! { whippyunits::unit!(#luminosity_base_ident / #length_base_ident^2) }
            },
            // For other units, fall back to the original unit type
            _ => {
                let unit_ident = self.unit_ident;
                quote! { whippyunits::unit!(#unit_ident) }
            }
        }
    }
}