use whippyunits_lsp_proxy::LspProxy;

fn main() {
    // Create a sample LSP hover response that contains whippyunits types
    let sample_hover_response = r#"{
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "contents": [
                {
                    "language": "rust",
                    "value": "let distance: Quantity<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0> = 5.0.meters();"
                }
            ]
        }
    }"#;
    
    println!("Original LSP hover response:");
    println!("{}", sample_hover_response);
    println!();
    
    // Create the proxy and process the message
    let proxy = LspProxy::new();
    
    match proxy.process_incoming(sample_hover_response) {
        Ok(improved_response) => {
            println!("Improved LSP hover response:");
            println!("{}", improved_response);
            println!();
            
            // Parse and pretty-print the JSON for better readability
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&improved_response) {
                println!("Pretty-printed improved response:");
                println!("{}", serde_json::to_string_pretty(&parsed).unwrap_or(improved_response));
            }
        }
        Err(e) => {
            eprintln!("Error processing message: {}", e);
        }
    }
}
