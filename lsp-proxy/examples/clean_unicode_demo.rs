use whippyunits_lsp_proxy::{WhippyUnitsTypeConverter, DisplayConfig};

fn main() {
    let converter = WhippyUnitsTypeConverter::new();
    
    // Test inputs: typical whippyunits Quantity types
    let test_inputs = vec![
        "Quantity<1, 0, 0, 0, 0, 0, 0, 0, 0>",  // Simple meter
        "Quantity<2, 0, 0, 0, 0, 0, 0, 0, 0>",  // Square meter
        "Quantity<1, 0, 1, 0, 0, 0, 0, 0, 0>",  // Meter-gram
        "Quantity<1, 0, 0, 0, 1, 0, 0, 0, 0>",  // Meter-second
        "Quantity<2, 0, 1, 0, -1, 0, 0, 0, 0>", // Complex: m²·g·s⁻¹
    ];
    
    println!("🚀 WHIPPYUNITS LSP PROXY - CLEAN UNICODE MODE (State 1)");
    println!("======================================================\n");
    
    // Configure for clean Unicode mode (State 1)
    let config = DisplayConfig { verbose: false, unicode: true };
    println!("Configuration: verbose=false, unicode=true\n");
    
    for (i, test_input) in test_inputs.iter().enumerate() {
        println!("Test {}: {}", i + 1, test_input);
        let result = converter.convert_types_in_text_with_config(test_input, &config);
        println!("Output:  {}", result);
        println!();
    }
    
    println!("This would be the clean mode - simple units with Unicode superscripts!");
    println!("Your current configuration is verbose mode for maximum detail.");
}
