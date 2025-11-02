use proc_macro::TokenStream;

/// Generate literal macros module
pub fn define_literals(input: TokenStream) -> TokenStream {
    // Parse the input to see if a module name is provided
    let module_name = if input.is_empty() {
        "custom_literal".to_string()
    } else {
        // Parse as an identifier for the module name
        match syn::parse::<syn::Ident>(input) {
            Ok(ident) => ident.to_string(),
            Err(_) => {
                // If parsing fails, use default
                "custom_literal".to_string()
            }
        }
    };

    let custom_literal_module = crate::utils::culit::generate_custom_literal_module_with_name(&module_name);
    TokenStream::from(custom_literal_module)
}

