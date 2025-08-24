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
    
    println!("🚀 WHIPPYUNITS LSP PROXY - VERBOSE UNICODE MODE (State 3)");
    println!("========================================================\n");
    
    // Configure for verbose Unicode mode (State 3)
    let config = DisplayConfig { verbose: true, unicode: true, include_raw: false };
    println!("Configuration: verbose=true, unicode=true\n");
    
    for (i, test_input) in test_inputs.iter().enumerate() {
        println!("Test {}: {}", i + 1, test_input);
        let result = converter.convert_types_in_text_with_config(test_input, &config);
        println!("Output:  {}", result);
        println!();
    }
    
    println!("This is what you'll see in VS Code hover tooltips!");
    println!("The verbose mode shows full const generic details with Unicode support.");
}


