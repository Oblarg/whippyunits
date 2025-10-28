use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::Ident;

/// Input for the generate_local_unit_literals macro
/// Usage: generate_local_unit_literals!(mass_scale, length_scale, time_scale, current_scale, temperature_scale, amount_scale, luminosity_scale, angle_scale)
pub struct LocalUnitLiteralsInput {
    pub mass_scale: Ident,
    pub length_scale: Ident,
    pub time_scale: Ident,
    pub current_scale: Ident,
    pub temperature_scale: Ident,
    pub amount_scale: Ident,
    pub luminosity_scale: Ident,
    pub angle_scale: Ident,
}

impl Parse for LocalUnitLiteralsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mass_scale = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let length_scale = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let time_scale = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let current_scale = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let temperature_scale = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let amount_scale = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let luminosity_scale = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let angle_scale = input.parse()?;

        Ok(LocalUnitLiteralsInput {
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

impl LocalUnitLiteralsInput {
    pub fn expand(self) -> TokenStream {
        // Use the single generic function in local mode with lift trace
        // This removes the bespoke lift trace logic since it can pick up from the local quantity macro
        let scale_params = (
            self.mass_scale,
            self.length_scale,
            self.time_scale,
            self.current_scale,
            self.temperature_scale,
            self.amount_scale,
            self.luminosity_scale,
            self.angle_scale,
        );
        crate::generate_literal_macros_module("local_unit_literals", true, Some(scale_params), false, syn::Ident::new("local_unit_literals", proc_macro2::Span::mixed_site())).into()
    }

}
