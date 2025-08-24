use std::process::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader as TokioBufReader};
use tokio::io::AsyncBufReadExt;
use serde_json::json;
use anyhow::Result;
use crate::inlay_hint_processor::InlayHintProcessor;

/// Find the path to rust-analyzer
fn find_rust_analyzer_path() -> Option<String> {
    // Try common locations for rust-analyzer
    let candidates = vec![
        "rust-analyzer",
        "~/.cargo/bin/rust-analyzer",
        "~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rust-analyzer",
        "~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rust-analyzer",
        "~/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rust-analyzer",
        "~/.rustup/toolchains/nightly-aarch64-apple-darwin/bin/rust-analyzer",
    ];
    
    for candidate in candidates {
        let path = if candidate.starts_with("~/") {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            candidate.replace("~", &home)
        } else {
            candidate.to_string()
        };
        
        if Command::new(&path)
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_ok()
        {
            return Some(path);
        }
    }
    
    None
}

/// Test to intercept and analyze inlay hint messages
/// This will help us understand the structure and framing of inlay hints
/// before attempting to hook them to our existing prettyprint pipeline
#[tokio::test]
async fn test_inlay_hint_intercept() -> Result<()> {
    // Skip if rust-analyzer not available
    let rust_analyzer_path = find_rust_analyzer_path();
    if rust_analyzer_path.is_none() {
        eprintln!("rust-analyzer not available, skipping test");
        return Ok(());
    }
    let rust_analyzer_path = rust_analyzer_path.unwrap();

    // Use the actual whippyunits project directory
    let project_dir = std::env::current_dir()?.parent().unwrap().to_path_buf();
    let basic_test_path = project_dir.join("examples").join("basic_test.rs");
    
    // Read the existing basic_test.rs file
    let test_content = std::fs::read_to_string(&basic_test_path)?;
    println!("Using basic_test.rs: {}", basic_test_path.display());
    
    // Change to the project directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&project_dir)?;
    
    // Spawn rust-analyzer
    println!("Spawning rust-analyzer from: {}", rust_analyzer_path);
    let mut rust_analyzer = tokio::process::Command::new(&rust_analyzer_path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    
    // Get handles to rust-analyzer's stdin/stdout
    let mut ra_stdin = rust_analyzer.stdin.take().expect("Failed to get rust-analyzer stdin");
    let ra_stdout = rust_analyzer.stdout.take().expect("Failed to get rust-analyzer stdout");
    let mut reader = TokioBufReader::new(ra_stdout);
    
    // Send initialization request with inlay hint capabilities
    println!("Sending initialization with inlay hint capabilities...");
    let init_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": 123,
            "rootUri": format!("file://{}", project_dir.to_string_lossy()),
            "capabilities": {
                "textDocument": {
                    "inlayHint": {
                        "dynamicRegistration": true,
                        "resolveSupport": {
                            "properties": ["tooltip", "textEdits", "label"]
                        }
                    }
                }
            }
        }
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&init_request)?).await?;
    
    // Read initialization response
    let init_response = read_lsp_message(&mut reader).await?;
    println!("Init response received (length: {})", init_response.len());
    
    // Send initialized notification
    println!("Sending initialized notification...");
    let initialized_notification = json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&initialized_notification)?).await?;
    
    // Send textDocument/didOpen notification for basic_test.rs
    println!("Sending didOpen for basic_test.rs...");
    let did_open_notification = json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/")),
                "languageId": "rust",
                "version": 1,
                "text": test_content
            }
        }
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&did_open_notification)?).await?;
    
    // Wait for rust-analyzer to process
    println!("Waiting for rust-analyzer to process...");
    tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    
    // Send inlay hint request for the entire document
    println!("Sending inlay hint request...");
    let inlay_hint_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/inlayHint",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/"))
            },
            "range": {
                "start": {
                    "line": 0,
                    "character": 0
                },
                "end": {
                    "line": 20, // Use a smaller range that should exist
                    "character": 0
                }
            }
        }
    });
    let inlay_hint_request_str = serde_json::to_string(&inlay_hint_request)?;
    println!("Inlay hint request: {}", inlay_hint_request_str);
    send_lsp_message(&mut ra_stdin, &inlay_hint_request_str).await?;
    
    // Read messages until we get a response with id: 2 or timeout
    println!("Reading messages for inlay hints...");
    let mut messages_read = 0;
    let max_messages = 30;
    let mut found_inlay_hint_response = false;
    let mut inlay_hint_messages = Vec::new();
    
    while messages_read < max_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                messages_read += 1;
                println!("Message {}: {}", messages_read, message);
                
                // Check if this is our inlay hint response
                if message.contains("\"id\":2") {
                    println!("*** FOUND INLAY HINT RESPONSE! ***");
                    found_inlay_hint_response = true;
                    inlay_hint_messages.push(message.clone());
                    
                    // Analyze the message structure
                    analyze_inlay_hint_message(&message);
                }
                
                // Also check for any response with "result" field that might be inlay hints
                if message.contains("\"id\":") && message.contains("\"result\":") {
                    println!("*** FOUND SOME RESPONSE WITH RESULT! ***");
                    if !inlay_hint_messages.contains(&message) {
                        inlay_hint_messages.push(message.clone());
                    }
                }
            }
            Err(e) => {
                println!("Failed to read message {}: {}", messages_read + 1, e);
                break;
            }
        }
    }
    
    if !found_inlay_hint_response {
        println!("No inlay hint response found after {} messages", messages_read);
        println!("This might indicate that rust-analyzer doesn't support inlay hints or they're disabled");
    }
    
    // Try a different approach - send a more specific inlay hint request
    println!("Trying alternative inlay hint request...");
    let alt_inlay_hint_request = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "textDocument/inlayHint",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/"))
            },
            "range": {
                "start": {
                    "line": 10,
                    "character": 0
                },
                "end": {
                    "line": 20,
                    "character": 0
                }
            }
        }
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&alt_inlay_hint_request)?).await?;
    
    // Read a few more messages
    for _ in 0..10 {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                println!("Additional message: {}", message);
                if message.contains("\"id\":3") {
                    println!("*** FOUND ALTERNATIVE INLAY HINT RESPONSE! ***");
                    analyze_inlay_hint_message(&message);
                }
            }
            Err(_) => break,
        }
    }
    
    // Clean up
    if let Err(e) = rust_analyzer.kill().await {
        eprintln!("Failed to kill rust-analyzer: {}", e);
    }
    
    // Restore original directory
    std::env::set_current_dir(original_dir)?;
    
    Ok(())
}

/// Analyze the structure of an inlay hint message
fn analyze_inlay_hint_message(message: &str) {
    println!("\n=== INLAY HINT MESSAGE ANALYSIS ===");
    println!("Raw message: {}", message);
    
    // Try to parse as JSON
    match serde_json::from_str::<serde_json::Value>(message) {
        Ok(json_value) => {
            println!("Successfully parsed as JSON");
            
            // Check if it's a response
            if let Some(id) = json_value.get("id") {
                println!("Message ID: {}", id);
            }
            
            // Check if it has a result field
            if let Some(result) = json_value.get("result") {
                println!("Has result field: {}", result);
                
                // Check if result is an array (typical for inlay hints)
                if let Some(array) = result.as_array() {
                    println!("Result is an array with {} items", array.len());
                    
                    for (i, item) in array.iter().enumerate() {
                        println!("Item {}: {}", i, item);
                        
                        // Analyze individual inlay hint structure
                        if let Some(position) = item.get("position") {
                            println!("  Position: {}", position);
                        }
                        
                        if let Some(kind) = item.get("kind") {
                            println!("  Kind: {}", kind);
                        }
                        
                        if let Some(label) = item.get("label") {
                            println!("  Label: {}", label);
                        }
                        
                        if let Some(tooltip) = item.get("tooltip") {
                            println!("  Tooltip: {}", tooltip);
                        }
                        
                        if let Some(text_edits) = item.get("textEdits") {
                            println!("  TextEdits: {}", text_edits);
                        }
                    }
                } else {
                    println!("Result is not an array: {}", result);
                }
            } else {
                println!("No result field found");
            }
            
            // Check for error field
            if let Some(error) = json_value.get("error") {
                println!("Has error field: {}", error);
            }
        }
        Err(e) => {
            println!("Failed to parse as JSON: {}", e);
        }
    }
    
    println!("=== END ANALYSIS ===\n");
}

/// Test to check if rust-analyzer supports inlay hints
#[tokio::test]
async fn test_inlay_hint_support() -> Result<()> {
    // Skip if rust-analyzer not available
    let rust_analyzer_path = find_rust_analyzer_path();
    if rust_analyzer_path.is_none() {
        eprintln!("rust-analyzer not available, skipping test");
        return Ok(());
    }
    let rust_analyzer_path = rust_analyzer_path.unwrap();

    // Use the actual whippyunits project directory
    let project_dir = std::env::current_dir()?.parent().unwrap().to_path_buf();
    
    // Change to the project directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&project_dir)?;
    
    // Spawn rust-analyzer
    println!("Spawning rust-analyzer from: {}", rust_analyzer_path);
    let mut rust_analyzer = tokio::process::Command::new(&rust_analyzer_path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    
    // Get handles to rust-analyzer's stdin/stdout
    let mut ra_stdin = rust_analyzer.stdin.take().expect("Failed to get rust-analyzer stdin");
    let ra_stdout = rust_analyzer.stdout.take().expect("Failed to get rust-analyzer stdout");
    let mut reader = TokioBufReader::new(ra_stdout);
    
    // Send initialization request
    let init_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": 123,
            "rootUri": format!("file://{}", project_dir.to_string_lossy()),
            "capabilities": {}
        }
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&init_request)?).await?;
    
    // Read initialization response
    let init_response = read_lsp_message(&mut reader).await?;
    println!("Init response: {}", init_response);
    
    // Parse the response to check server capabilities
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&init_response) {
        if let Some(result) = json_value.get("result") {
            if let Some(capabilities) = result.get("capabilities") {
                println!("Server capabilities: {}", capabilities);
                
                // Check for inlay hint support
                if let Some(text_document) = capabilities.get("textDocument") {
                    if let Some(inlay_hint) = text_document.get("inlayHint") {
                        println!("Inlay hint support found: {}", inlay_hint);
                    } else {
                        println!("No inlay hint support found in textDocument capabilities");
                    }
                } else {
                    println!("No textDocument capabilities found");
                }
            }
        }
    }
    
    // Clean up
    if let Err(e) = rust_analyzer.kill().await {
        eprintln!("Failed to kill rust-analyzer: {}", e);
    }
    
    // Restore original directory
    std::env::set_current_dir(original_dir)?;
    
    Ok(())
}

/// Test to specifically look for inlay hints in basic_test.rs around whippyunits types
#[tokio::test]
async fn test_inlay_hints_with_whippyunits_types() -> Result<()> {
    // Skip if rust-analyzer not available
    let rust_analyzer_path = find_rust_analyzer_path();
    if rust_analyzer_path.is_none() {
        eprintln!("rust-analyzer not available, skipping test");
        return Ok(());
    }
    let rust_analyzer_path = rust_analyzer_path.unwrap();

    // Create the inlay hint processor
    let processor = InlayHintProcessor::new();

    // Use the actual whippyunits project directory
    let project_dir = std::env::current_dir()?.parent().unwrap().to_path_buf();
    let basic_test_path = project_dir.join("examples").join("basic_test.rs");
    
    // Read the existing basic_test.rs file
    let test_content = std::fs::read_to_string(&basic_test_path)?;
    println!("Using basic_test.rs: {}", basic_test_path.display());
    
    // Change to the project directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&project_dir)?;
    
    // Spawn rust-analyzer
    println!("Spawning rust-analyzer from: {}", rust_analyzer_path);
    let mut rust_analyzer = tokio::process::Command::new(&rust_analyzer_path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    
    // Get handles to rust-analyzer's stdin/stdout
    let mut ra_stdin = rust_analyzer.stdin.take().expect("Failed to get rust-analyzer stdin");
    let ra_stdout = rust_analyzer.stdout.take().expect("Failed to get rust-analyzer stdout");
    let mut reader = TokioBufReader::new(ra_stdout);
    
    // Send initialization request with inlay hint capabilities
    println!("Sending initialization with inlay hint capabilities...");
    let init_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": 123,
            "rootUri": format!("file://{}", project_dir.to_string_lossy()),
            "capabilities": {
                "textDocument": {
                    "inlayHint": {
                        "dynamicRegistration": true,
                        "resolveSupport": {
                            "properties": ["tooltip", "textEdits", "label"]
                        }
                    }
                }
            }
        }
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&init_request)?).await?;
    
    // Read initialization response
    let init_response = read_lsp_message(&mut reader).await?;
    println!("Init response received (length: {})", init_response.len());
    
    // Send initialized notification
    println!("Sending initialized notification...");
    let initialized_notification = json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&initialized_notification)?).await?;
    
    // Send textDocument/didOpen notification for basic_test.rs
    println!("Sending didOpen for basic_test.rs...");
    let did_open_notification = json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/")),
                "languageId": "rust",
                "version": 1,
                "text": test_content
            }
        }
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&did_open_notification)?).await?;
    
    // Wait for rust-analyzer to process
    println!("Waiting for rust-analyzer to process...");
    tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    
    // Try multiple ranges where whippyunits types are likely to be used
    let ranges = vec![
        // Range around the main function where whippyunits types are used
        (10, 25),
        // Range around the distance1 variable
        (12, 18),
        // Range around the distance2 variable
        (13, 18),
        // Range around the distance3 variable
        (14, 18),
    ];
    
    for (start_line, end_line) in ranges {
        println!("Trying inlay hint request for lines {} to {}", start_line, end_line);
        
        let inlay_hint_request = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "textDocument/inlayHint",
            "params": {
                "textDocument": {
                    "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/"))
                },
                "range": {
                    "start": {
                        "line": start_line,
                        "character": 0
                    },
                    "end": {
                        "line": end_line,
                        "character": 0
                    }
                }
            }
        });
        
        send_lsp_message(&mut ra_stdin, &serde_json::to_string(&inlay_hint_request)?).await?;
        
        // Read messages until we get a response with id: 2 or timeout
        let mut messages_read = 0;
        let max_messages = 10;
        let mut found_inlay_hint_response = false;
        
        while messages_read < max_messages {
            match read_lsp_message(&mut reader).await {
                Ok(message) => {
                    messages_read += 1;
                    
                    // Check if this is our inlay hint response
                    if message.contains("\"id\":2") {
                        println!("*** FOUND INLAY HINT RESPONSE for lines {} to {}! ***", start_line, end_line);
                        found_inlay_hint_response = true;
                        
                        // Check if it contains whippyunits types
                        if message.contains("Quantity") {
                            println!("*** FOUND WHIPPYUNITS TYPES IN INLAY HINTS! ***");
                            
                            // Print the original message
                            println!("\n=== ORIGINAL INLAY HINT MESSAGE ===");
                            println!("{}", message);
                            println!("=== END ORIGINAL MESSAGE ===\n");
                            
                            // Process the message
                            match processor.process_inlay_hint_response(&message) {
                                Ok(processed_message) => {
                                    println!("=== PROCESSED INLAY HINT MESSAGE ===");
                                    println!("{}", processed_message);
                                    println!("=== END PROCESSED MESSAGE ===\n");
                                    
                                    // Also show a pretty-printed comparison
                                    println!("=== JSON COMPARISON ===");
                                    if let Ok(original_json) = serde_json::from_str::<serde_json::Value>(&message) {
                                        if let Ok(processed_json) = serde_json::from_str::<serde_json::Value>(&processed_message) {
                                            println!("Original JSON (pretty):");
                                            println!("{}", serde_json::to_string_pretty(&original_json)?);
                                            println!("\nProcessed JSON (pretty):");
                                            println!("{}", serde_json::to_string_pretty(&processed_json)?);
                                        }
                                    }
                                    println!("=== END JSON COMPARISON ===\n");
                                }
                                Err(e) => {
                                    println!("Failed to process inlay hint message: {}", e);
                                }
                            }
                        } else {
                            // Still analyze the message structure for non-whippyunits types
                            analyze_inlay_hint_message(&message);
                        }
                        break;
                    }
                }
                
                Err(e) => {
                    println!("Failed to read message {}: {}", messages_read + 1, e);
                    break;
                }
            }
        }
        
        if !found_inlay_hint_response {
            println!("No inlay hint response found for lines {} to {}", start_line, end_line);
        }
        
        // Wait a bit before trying the next range
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
    
    // Clean up
    if let Err(e) = rust_analyzer.kill().await {
        eprintln!("Failed to kill rust-analyzer: {}", e);
    }
    
    // Restore original directory
    std::env::set_current_dir(original_dir)?;
    
    Ok(())
}

#[tokio::test]
async fn test_hover_at_inlay_hint_locations() -> Result<()> {
    println!("=== Testing Hover at Inlay Hint Locations ===");
    
    // Start rust-analyzer
    let mut rust_analyzer = tokio::process::Command::new("/Users/emichaelbarnettgmail.com/.rustup/toolchains/nightly-aarch64-apple-darwin/bin/rust-analyzer")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    
    let mut stdin = rust_analyzer.stdin.take().unwrap();
    let stdout = rust_analyzer.stdout.take().unwrap();
    let mut reader = TokioBufReader::new(stdout);
    
    // Get the project directory
    let project_dir = std::env::current_dir()?.parent().unwrap().to_path_buf();
    let basic_test_path = project_dir.join("examples").join("basic_test.rs");
    let test_content = std::fs::read_to_string(&basic_test_path)?;
    
    // Change to the project directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&project_dir)?;
    
    // Initialize LSP connection
    println!("Initializing LSP connection...");
    let init_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": std::process::id(),
            "rootUri": format!("file://{}", project_dir.to_string_lossy().replace("\\", "/")),
            "capabilities": {
                "textDocument": {
                    "hover": {},
                    "inlayHint": {
                        "resolveSupport": {
                            "properties": ["tooltip", "textEdits", "label"]
                        }
                    }
                }
            }
        }
    });
    
    send_lsp_message(&mut stdin, &serde_json::to_string(&init_request)?).await?;
    
    // Read init response
    let init_response = read_lsp_message(&mut reader).await?;
    println!("Init response: {}", init_response);
    
    // Send initialized notification
    let initialized_notification = json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_lsp_message(&mut stdin, &serde_json::to_string(&initialized_notification)?).await?;
    
    // Open the document
    println!("Opening document...");
    let open_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/")),
                "languageId": "rust",
                "version": 1,
                "text": test_content
            }
        }
    });
    send_lsp_message(&mut stdin, &serde_json::to_string(&open_request)?).await?;
    
    // Wait for rust-analyzer to process the file
    tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    
    // Send hover request at the inlay hint location (line 12, character 17 - after the colon)
    println!("Sending hover request at inlay hint location (line 12, character 17)...");
    let hover_request = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "textDocument/hover",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/"))
            },
            "position": {
                "line": 12,
                "character": 17
            }
        }
    });
    send_lsp_message(&mut stdin, &serde_json::to_string(&hover_request)?).await?;
    
    // Read hover response
    let mut hover_messages_read = 0;
    let max_hover_messages = 10;
    
    while hover_messages_read < max_hover_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                hover_messages_read += 1;
                println!("Hover message {}: {}", hover_messages_read, message);
                
                // Check if this contains Quantity types
                if message.contains("Quantity") {
                    println!("*** FOUND QUANTITY TYPE IN HOVER RESPONSE! ***");
                    
                    // Save to file for analysis
                    std::fs::write("hover_response.json", &message)?;
                    println!("Saved hover response to hover_response.json");
                }
                
                // Parse to see if it's the hover response
                if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&message) {
                    if let Some(id) = json_value.get("id") {
                        if id == 3 {
                            println!("This is the hover response for request ID 3");
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to read hover message {}: {}", hover_messages_read + 1, e);
                break;
            }
        }
    }
    
    // Send hover request at the second inlay hint location (line 13, character 17)
    println!("Sending hover request at second inlay hint location (line 13, character 17)...");
    let hover_request2 = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "textDocument/hover",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/"))
            },
            "position": {
                "line": 13,
                "character": 17
            }
        }
    });
    send_lsp_message(&mut stdin, &serde_json::to_string(&hover_request2)?).await?;
    
    // Read second hover response
    let mut hover_messages_read2 = 0;
    let max_hover_messages2 = 10;
    
    while hover_messages_read2 < max_hover_messages2 {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                hover_messages_read2 += 1;
                println!("Hover message 2 {}: {}", hover_messages_read2, message);
                
                // Check if this contains Quantity types
                if message.contains("Quantity") {
                    println!("*** FOUND QUANTITY TYPE IN SECOND HOVER RESPONSE! ***");
                    
                    // Save to file for analysis
                    std::fs::write("hover_response2.json", &message)?;
                    println!("Saved second hover response to hover_response2.json");
                }
                
                // Parse to see if it's the hover response
                if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&message) {
                    if let Some(id) = json_value.get("id") {
                        if id == 4 {
                            println!("This is the second hover response for request ID 4");
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to read second hover message {}: {}", hover_messages_read2 + 1, e);
                break;
            }
        }
    }
    
    // Clean up
    if let Err(e) = rust_analyzer.kill().await {
        eprintln!("Failed to kill rust-analyzer: {}", e);
    }
    
    // Restore original directory
    std::env::set_current_dir(original_dir)?;
    
    Ok(())
}

async fn send_lsp_message(stdin: &mut tokio::process::ChildStdin, message: &str) -> Result<()> {
    // LSP protocol requires Content-Length header
    let header = format!("Content-Length: {}\r\n\r\n", message.len());
    stdin.write_all(header.as_bytes()).await?;
    stdin.write_all(message.as_bytes()).await?;
    stdin.flush().await?;
    Ok(())
}

async fn read_lsp_message(reader: &mut TokioBufReader<tokio::process::ChildStdout>) -> Result<String> {
    let mut line = String::new();
    let mut content_length = None;
    
    // Read headers
    loop {
        line.clear();
        reader.read_line(&mut line).await?;
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            break; // End of headers
        }
        
        if trimmed.starts_with("Content-Length: ") {
            // Parse content length
            content_length = Some(trimmed
                .split(": ")
                .nth(1)
                .unwrap()
                .parse::<usize>()?);
        }
    }
    
    // Read the message content
    if let Some(length) = content_length {
        // Read exact number of bytes
        let mut message = vec![0u8; length];
        reader.read_exact(&mut message).await?;
        Ok(String::from_utf8(message)?)
    } else {
        // No Content-Length header, read until newline
        let mut message = String::new();
        reader.read_line(&mut message).await?;
        
        if message.trim().is_empty() {
            Err(anyhow::anyhow!("Empty message received"))
        } else {
            Ok(message.trim().to_string())
        }
    }
}
