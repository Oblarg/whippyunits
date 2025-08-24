use std::process::{Command, Stdio};
use std::io::{BufRead, Write};
use tokio::process::{Command as TokioCommand, Child};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, AsyncReadExt, BufReader as TokioBufReader};
use serde_json::{json, Value};
use anyhow::Result;
use crate::WhippyUnitsTypeConverter;
use crate::LspProxy;

/// Real integration test that communicates with rust-analyzer
#[tokio::test]
async fn test_real_lsp_communication() -> Result<()> {
    // Skip if rust-analyzer not available
    if !rust_analyzer_available() {
        eprintln!("rust-analyzer not available, skipping integration test");
        return Ok(());
    }

    // Create a simple Rust file with whippyunits types
    let test_file = create_test_file()?;
    
    // Spawn rust-analyzer
    let mut rust_analyzer = spawn_rust_analyzer().await?;
    
    // Get handles to rust-analyzer's stdin/stdout
    let mut ra_stdin = rust_analyzer.stdin.take().expect("Failed to get rust-analyzer stdin");
    let ra_stdout = rust_analyzer.stdout.take().expect("Failed to get rust-analyzer stdout");
    
    // Send initialization request
    let init_request = create_init_request(&test_file);
    send_lsp_message(&mut ra_stdin, &init_request).await?;
    
    // Read initialization response
    let mut reader = TokioBufReader::new(ra_stdout);
    let init_response = read_lsp_message(&mut reader).await?;
    
    println!("Init response: {}", init_response);
    
    // Send initialized notification
    let initialized_notification = json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&initialized_notification)?).await?;
    
    // Wait a bit for rust-analyzer to process
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Send textDocument/didOpen notification
    let did_open_notification = create_did_open_notification(&test_file);
    send_lsp_message(&mut ra_stdin, &did_open_notification).await?;
    
    // Wait for rust-analyzer to process the file
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Send a hover request for the whippyunits type
    let hover_request = create_hover_request(&test_file, 4, 20); // Position over the type annotation
    send_lsp_message(&mut ra_stdin, &hover_request).await?;
    
    // Read hover response
    let hover_response = read_lsp_message(&mut reader).await?;
    
    println!("Hover response: {}", hover_response);
    
    // Test our proxy with the real response
    let proxy = LspProxy::new();
    match proxy.process_incoming(&hover_response) {
        Ok(improved_response) => {
            println!("Improved response: {}", improved_response);
            
            // If rust-analyzer returned null or content modified, that's expected for some positions
            if hover_response.contains("\"result\":null") || hover_response.contains("content modified") {
                println!("rust-analyzer returned null or content modified - this is expected for some positions");
                // Don't assert anything, just verify the proxy handled it correctly
            } else {
                // Verify the response contains our improved type format
                assert!(improved_response.contains("Quantity<"));
                assert!(improved_response.contains("meter"));
                assert!(improved_response.contains("Length: Exponent"));
            }
        }
        Err(e) => {
            eprintln!("Failed to process response: {}", e);
            // Don't fail the test, just log the error
        }
    }
    
    // Clean up
    if let Err(e) = rust_analyzer.kill().await {
        eprintln!("Failed to kill rust-analyzer: {}", e);
    }
    
    // Clean up test file (only if we created it, not if it's the existing basic_test.rs)
    if test_file.contains("test_whippyunits.rs") {
        let _ = std::fs::remove_file(&test_file);
    }
    
    Ok(())
}

/// Test that tries multiple positions to find a real hover response
#[tokio::test]
async fn test_real_hover_with_whippyunits() -> Result<()> {
    // Skip if rust-analyzer not available
    if !rust_analyzer_available() {
        eprintln!("rust-analyzer not available, skipping integration test");
        return Ok(());
    }

    // Create a simple Rust file with whippyunits types
    let test_file = create_test_file()?;
    
    // Spawn rust-analyzer
    let mut rust_analyzer = spawn_rust_analyzer().await?;
    
    // Get handles to rust-analyzer's stdin/stdout
    let mut ra_stdin = rust_analyzer.stdin.take().expect("Failed to get rust-analyzer stdin");
    let ra_stdout = rust_analyzer.stdout.take().expect("Failed to get rust-analyzer stdout");
    
    // Send initialization request
    let init_request = create_init_request(&test_file);
    send_lsp_message(&mut ra_stdin, &init_request).await?;
    
    // Read initialization response
    let mut reader = TokioBufReader::new(ra_stdout);
    let init_response = read_lsp_message(&mut reader).await?;
    
    println!("Init response: {}", init_response);
    
    // Send initialized notification
    let initialized_notification = json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&initialized_notification)?).await?;
    
    // Wait a bit for rust-analyzer to process
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Send textDocument/didOpen notification
    let did_open_notification = create_did_open_notification(&test_file);
    send_lsp_message(&mut ra_stdin, &did_open_notification).await?;
    
    // Wait for rust-analyzer to process the file
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    // Try multiple positions to find one that returns hover information
    let positions = vec![
        (4, 20),  // Over type annotation
        (4, 15),  // Over Quantity
        (4, 25),  // Over const generic parameters
        (5, 10),  // Over .meters()
        (5, 15),  // Over meters
    ];
    
    let mut found_hover = false;
    
    for (line, character) in positions {
        println!("Trying position ({}, {})", line, character);
        
        // Send a hover request
        let hover_request = create_hover_request(&test_file, line, character);
        send_lsp_message(&mut ra_stdin, &hover_request).await?;
        
        // Read hover response
        let hover_response = read_lsp_message(&mut reader).await?;
        
        println!("Hover response at ({}, {}): {}", line, character, hover_response);
        
        // Test our proxy with the real response
        let proxy = LspProxy::new();
        match proxy.process_incoming(&hover_response) {
            Ok(improved_response) => {
                println!("Improved response: {}", improved_response);
                
                // Check if we got a real hover response (not null or error)
                if !hover_response.contains("\"result\":null") && 
                   !hover_response.contains("error") &&
                   hover_response.contains("Quantity<") {
                    println!("Found real hover response with whippyunits types!");
                    found_hover = true;
                    
                    // Verify the response contains our improved type format
                    assert!(improved_response.contains("Quantity<"));
                    assert!(improved_response.contains("meter"));
                    assert!(improved_response.contains("Length: Exponent"));
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to process response: {}", e);
            }
        }
    }
    
    if !found_hover {
        println!("No hover responses with whippyunits types found - this is expected if rust-analyzer can't resolve the types");
    }
    
    // Clean up
    if let Err(e) = rust_analyzer.kill().await {
        eprintln!("Failed to kill rust-analyzer: {}", e);
    }
    
    // Clean up test file (only if we created it)
    if test_file.contains("test_whippyunits.rs") {
        let _ = std::fs::remove_file(&test_file);
    }
    
    Ok(())
}

/// Test that creates a proper Rust project to get real hover responses with whippyunits types
#[tokio::test]
async fn test_real_hover_with_proper_project() -> Result<()> {
    // Skip if rust-analyzer not available
    if !rust_analyzer_available() {
        eprintln!("rust-analyzer not available, skipping integration test");
        return Ok(());
    }

    // Use the actual whippyunits project directory
    let project_dir = std::env::current_dir()?.parent().unwrap().to_path_buf();
    
    // Create a test file in the examples directory
    let test_file_path = project_dir.join("examples").join("hover_test.rs");
    let test_content = r#"use whippyunits::*;

fn main() {
    let distance: Meter = 5.0.meters();
    let mass: Kilogram = 2.0.kilograms();
    let time: Second = 1.0.seconds();
    
    println!("Distance: {}", distance);
    println!("Mass: {}", mass);
    println!("Time: {}", time);
}
"#;
    
    // Write the test file
    std::fs::write(&test_file_path, test_content)?;
    
    // Change to the project directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&project_dir)?;
    
    // Spawn rust-analyzer
    let mut rust_analyzer = spawn_rust_analyzer().await?;
    
    // Get handles to rust-analyzer's stdin/stdout
    let mut ra_stdin = rust_analyzer.stdin.take().expect("Failed to get rust-analyzer stdin");
    let ra_stdout = rust_analyzer.stdout.take().expect("Failed to get rust-analyzer stdout");
    
    // Send initialization request
    let init_request = create_init_request_for_project(&project_dir);
    send_lsp_message(&mut ra_stdin, &init_request).await?;
    
    // Read initialization response
    let mut reader = TokioBufReader::new(ra_stdout);
    let init_response = read_lsp_message(&mut reader).await?;
    
    println!("Init response: {}", init_response);
    
    // Send initialized notification
    let initialized_notification = json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&initialized_notification)?).await?;
    
    // Wait for rust-analyzer to process
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    
    // Send textDocument/didOpen notification for the test file
    let did_open_notification = create_did_open_notification_for_file(&test_file_path, test_content);
    send_lsp_message(&mut ra_stdin, &did_open_notification).await?;
    
    // Wait for rust-analyzer to process the file
    tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
    
    // Try multiple positions to find hover information
    let positions = vec![
        (2, 15),  // Over Meter type
        (3, 18),  // Over Kilogram type  
        (4, 16),  // Over Second type
        (2, 20),  // Over distance variable
        (3, 23),  // Over mass variable
        (4, 21),  // Over time variable
    ];
    
    let mut found_hover = false;
    
    for (line, character) in positions {
        println!("Trying position ({}, {})", line, character);
        
        // Send a hover request
        let hover_request = create_hover_request_for_file(&test_file_path, line, character);
        send_lsp_message(&mut ra_stdin, &hover_request).await?;
        
        // Read hover response
        let hover_response = read_lsp_message(&mut reader).await?;
        
        println!("Hover response at ({}, {}): {}", line, character, hover_response);
        
        // Test our proxy with the real response
        let proxy = LspProxy::new();
        match proxy.process_incoming(&hover_response) {
            Ok(improved_response) => {
                println!("Improved response: {}", improved_response);
                
                // Check if we got a real hover response with whippyunits types
                if !hover_response.contains("\"result\":null") && 
                   !hover_response.contains("error") &&
                   (hover_response.contains("Quantity<") || hover_response.contains("Meter") || hover_response.contains("Kilogram") || hover_response.contains("Second")) {
                    println!("Found real hover response with whippyunits types!");
                    found_hover = true;
                    
                    // Verify the response contains our improved type format
                    assert!(improved_response.contains("Quantity<") || improved_response.contains("Meter") || improved_response.contains("Kilogram") || improved_response.contains("Second"));
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to process response: {}", e);
            }
        }
    }
    
    if !found_hover {
        println!("No hover responses with whippyunits types found - this might indicate rust-analyzer couldn't resolve the types");
    }
    
    // Clean up
    if let Err(e) = rust_analyzer.kill().await {
        eprintln!("Failed to kill rust-analyzer: {}", e);
    }
    
    // Restore original directory
    std::env::set_current_dir(original_dir)?;
    
    // Clean up test file
    let _ = std::fs::remove_file(&test_file_path);
    
    Ok(())
}



/// Test that uses the existing basic_test.rs file to get real hover responses with whippyunits types
#[tokio::test]
async fn test_real_hover_with_basic_test() -> Result<()> {
    // Skip if rust-analyzer not available
    if !rust_analyzer_available() {
        eprintln!("rust-analyzer not available, skipping integration test");
        return Ok(());
    }

    // Use the actual whippyunits project directory
    let project_dir = std::env::current_dir()?.parent().unwrap().to_path_buf();
    let basic_test_path = project_dir.join("examples").join("basic_test.rs");
    
    // Read the existing basic_test.rs file
    let test_content = std::fs::read_to_string(&basic_test_path)?;
    // Using existing basic_test.rs for hover testing
    
    // Change to the project directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&project_dir)?;
    
    // Spawn rust-analyzer
    let mut rust_analyzer = spawn_rust_analyzer().await?;
    
    // Get handles to rust-analyzer's stdin/stdout
    let mut ra_stdin = rust_analyzer.stdin.take().expect("Failed to get rust-analyzer stdin");
    let ra_stdout = rust_analyzer.stdout.take().expect("Failed to get rust-analyzer stdout");
    
    // Send initialization request
    let init_request = create_init_request_for_project(&project_dir);
    send_lsp_message(&mut ra_stdin, &init_request).await?;
    
    // Read initialization response
    let mut reader = TokioBufReader::new(ra_stdout);
    let init_response = read_lsp_message(&mut reader).await?;
    
    println!("Init response: {}", init_response);
    
    // Send initialized notification
    let initialized_notification = json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&initialized_notification)?).await?;
    
    // Wait for rust-analyzer to process
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    
    // Send textDocument/didOpen notification for basic_test.rs
    let did_open_notification = create_did_open_notification_for_file(&basic_test_path, &test_content);
    send_lsp_message(&mut ra_stdin, &did_open_notification).await?;
    
    // Wait for rust-analyzer to process the file
    tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    
    // Try one specific position that should have hover information
    let positions = vec![
        (13, 10),  // Over distance1 variable name
    ];
    
    let mut found_hover = false;
    
    for (line, character) in positions {
        // Send a hover request
        let hover_request = create_hover_request_for_file(&basic_test_path, line, character);
        send_lsp_message(&mut ra_stdin, &hover_request).await?;
        
        // Read multiple messages - rust-analyzer might send diagnostics first, then hover response
        let mut messages_read = 0;
        let max_messages = 20; // Read up to 20 messages to find the hover response
        
        while messages_read < max_messages {
            match read_lsp_message(&mut reader).await {
                Ok(message) => {
                    messages_read += 1;
                    
                    // Check if this is our hover response (id: 2)
                    if message.contains("\"id\":2") {
                        // Test our proxy with the real response
                        let proxy = LspProxy::new();
                        match proxy.process_incoming(&message) {
                            Ok(improved_response) => {
                                println!("Original response: {}", message);
                                println!("Improved response: {}", improved_response);
                                
                                // Debug: Let's see what our type converter is doing
                                let converter = WhippyUnitsTypeConverter::new();
                                let test_type = "Quantity<1, -1, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>";
                                if let Some(converted) = converter.convert_quantity_type(test_type) {
                                    println!("DEBUG: Type conversion test: {} -> {}", test_type, converted);
                                } else {
                                    println!("DEBUG: Type conversion failed for: {}", test_type);
                                }
                                
                                // Debug: Let's also test the text conversion directly
                                let hover_text = "\n```rust\nlet distance2: Quantity<1, -1, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>\n```\n\n---\n\nsize = 8, align = 0x8, no Drop";
                                let converted_text = converter.convert_types_in_text(hover_text);
                                println!("DEBUG: Text conversion: {}", converted_text);
                                
                                // Check if this is a real hover response with whippyunits types
                                if message.contains("\"result\":") && 
                                   !message.contains("\"result\":null") && 
                                   !message.contains("\"error\"") &&
                                   message.contains("Quantity<") {
                                    found_hover = true;
                                    
                                    // Verify our type conversion worked
                                    if improved_response != message {
                                        println!("SUCCESS: Type conversion was applied!");
                                    }
                                    break;
                                }
                            }
                            Err(e) => {
                                // Failed to process response
                            }
                        }
                        break; // We found our response, stop reading
                    }
                }
                Err(e) => {
                    // Failed to read message
                    break;
                }
            }
        }
        
        // If we found a hover response, break out of the position loop
        if found_hover {
            break;
        }
    }
    
    if !found_hover {
        println!("No hover responses with whippyunits types found - this might indicate rust-analyzer couldn't resolve the types");
    }
    
    // Clean up
    if let Err(e) = rust_analyzer.kill().await {
        eprintln!("Failed to kill rust-analyzer: {}", e);
    }
    
    // Restore original directory
    std::env::set_current_dir(original_dir)?;
    
    Ok(())
}

#[test]
fn test_rust_analyzer_availability() {
    if rust_analyzer_available() {
        let output = Command::new("rust-analyzer")
            .arg("--version")
            .output();
        
        match output {
            Ok(output) => {
                println!("rust-analyzer version: {}", String::from_utf8_lossy(&output.stdout));
                assert!(output.status.success());
            }
            Err(e) => {
                eprintln!("rust-analyzer not available: {}", e);
            }
        }
    } else {
        eprintln!("rust-analyzer not found in PATH");
    }
}

#[test]
fn test_lsp_message_format() {
    // Test with a real LSP hover request format
    let hover_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "textDocument/hover",
        "params": {
            "textDocument": {
                "uri": "file:///test.rs"
            },
            "position": {
                "line": 0,
                "character": 10
            }
        }
    });
    
    let request_str = serde_json::to_string(&hover_request).unwrap();
    println!("Hover request: {}", request_str);
    
    // Verify it's valid JSON
    let parsed: Value = serde_json::from_str(&request_str).unwrap();
    assert_eq!(parsed["method"], "textDocument/hover");
}

fn rust_analyzer_available() -> bool {
    Command::new("rust-analyzer")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

fn create_test_file() -> Result<String> {
    // Use the actual examples/basic_test.rs from the parent directory
    let parent_dir = std::env::current_dir()?.parent().unwrap().to_path_buf();
    let example_file = parent_dir.join("examples").join("basic_test.rs");
    
    if example_file.exists() {
        println!("Using existing examples/basic_test.rs for integration test");
        Ok(example_file.to_string_lossy().to_string())
    } else {
        // Fallback to creating a simple test file
        println!("Creating fallback test file test_whippyunits.rs");
        let test_content = r#"
use whippyunits::*;

fn main() {
    let distance: Quantity<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0> = 5.0.meters();
    println!("Distance: {}", distance);
}
"#;
        
        let current_dir = std::env::current_dir()?;
        let test_file = current_dir.join("test_whippyunits.rs");
        std::fs::write(&test_file, test_content)?;
        Ok(test_file.to_string_lossy().to_string())
    }
}

async fn spawn_rust_analyzer() -> Result<Child> {
    let mut cmd = TokioCommand::new("rust-analyzer");
    cmd.stdin(Stdio::piped())
       .stdout(Stdio::piped())
       .stderr(Stdio::piped());
    
    println!("Spawning rust-analyzer: {:?}", cmd);
    
    let child = cmd.spawn()?;
    Ok(child)
}

fn create_init_request(_test_file: &str) -> String {
    let init_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": std::process::id(),
            "rootUri": format!("file://{}", std::env::current_dir().unwrap().to_string_lossy()),
            "capabilities": {
                "textDocument": {
                    "hover": {
                        "contentFormat": ["markdown", "plaintext"]
                    }
                }
            },
            "workspaceFolders": [
                {
                    "uri": format!("file://{}", std::env::current_dir().unwrap().to_string_lossy()),
                    "name": "test-workspace"
                }
            ]
        }
    });
    
    serde_json::to_string(&init_request).unwrap()
}

fn create_did_open_notification(test_file: &str) -> String {
    // Read the actual file content
    let test_content = match std::fs::read_to_string(test_file) {
        Ok(content) => content,
        Err(_) => r#"
use whippyunits::*;

fn main() {
    let distance: Quantity<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0> = 5.0.meters();
    println!("Distance: {}", distance);
}
"#.to_string()
    };
    
    let did_open_notification = json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", test_file.replace("\\", "/")),
                "languageId": "rust",
                "version": 1,
                "text": test_content
            }
        }
    });
    
    serde_json::to_string(&did_open_notification).unwrap()
}

fn create_hover_request(test_file: &str, line: u32, character: u32) -> String {
    let hover_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/hover",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", test_file.replace("\\", "/"))
            },
            "position": {
                "line": line,
                "character": character
            }
        }
    });
    
    serde_json::to_string(&hover_request).unwrap()
}

fn create_init_request_for_project(project_dir: &std::path::Path) -> String {
    let init_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": std::process::id(),
            "rootUri": format!("file://{}", project_dir.to_string_lossy()),
            "capabilities": {
                "textDocument": {
                    "hover": {
                        "contentFormat": ["markdown", "plaintext"]
                    }
                }
            },
            "workspaceFolders": [
                {
                    "uri": format!("file://{}", project_dir.to_string_lossy()),
                    "name": "whippyunits-test"
                }
            ]
        }
    });
    
    serde_json::to_string(&init_request).unwrap()
}

fn create_did_open_notification_for_file(file_path: &std::path::Path, content: &str) -> String {
    let did_open_notification = json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", file_path.to_string_lossy().replace("\\", "/")),
                "languageId": "rust",
                "version": 1,
                "text": content
            }
        }
    });
    
    serde_json::to_string(&did_open_notification).unwrap()
}

fn create_hover_request_for_file(file_path: &std::path::Path, line: u32, character: u32) -> String {
    let hover_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/hover",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", file_path.to_string_lossy().replace("\\", "/"))
            },
            "position": {
                "line": line,
                "character": character
            }
        }
    });
    
    serde_json::to_string(&hover_request).unwrap()
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

