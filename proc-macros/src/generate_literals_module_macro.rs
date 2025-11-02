use proc_macro::TokenStream;

/// Generate literals module for culit integration
pub fn generate_literals_module(_input: TokenStream) -> TokenStream {
    // This function delegates to the utility function in utils::literal_macros
    let literals_module = crate::utils::literal_macros::generate_literal_macros_module(
        "literals",
        false,
        None,
        false,
        syn::Ident::new("default_declarators", proc_macro2::Span::mixed_site()),
    );
    literals_module.into()
}

