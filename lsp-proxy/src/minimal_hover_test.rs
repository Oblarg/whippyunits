use std::process::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader as TokioBufReader};
use tokio::io::AsyncBufReadExt;
use serde_json::json;
use anyhow::Result;

/// Minimal test to intercept a single hover response
#[tokio::test]
async fn test_minimal_hover_intercept() -> Result<()> {
    // Skip if rust-analyzer not available
    if !Command::new("rust-analyzer").arg("--version").output().is_ok() {
        eprintln!("rust-analyzer not available, skipping test");
        return Ok(());
    }

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
    println!("Spawning rust-analyzer...");
    let mut rust_analyzer = tokio::process::Command::new("rust-analyzer")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    
    // Get handles to rust-analyzer's stdin/stdout
    let mut ra_stdin = rust_analyzer.stdin.take().expect("Failed to get rust-analyzer stdin");
    let ra_stdout = rust_analyzer.stdout.take().expect("Failed to get rust-analyzer stdout");
    let mut reader = TokioBufReader::new(ra_stdout);
    
    // Send initialization request
    println!("Sending initialization...");
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
    
    // Send hover request for distance1 variable name (line 13, character 10-18)
    println!("Sending hover request for distance1 variable name...");
    let hover_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/hover",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/"))
            },
            "position": {
                "line": 13,
                "character": 10
            }
        }
    });
    let hover_request_str = serde_json::to_string(&hover_request)?;
    println!("Hover request: {}", hover_request_str);
    send_lsp_message(&mut ra_stdin, &hover_request_str).await?;
    
    // Read messages until we get a response with id: 2 or timeout
    println!("Reading messages...");
    let mut messages_read = 0;
    let max_messages = 20;
    let mut found_hover_response = false;
    
    while messages_read < max_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                messages_read += 1;
                println!("Message {}: {}", messages_read, message);
                
                // Check if this is our hover response
                if message.contains("\"id\":2") {
                    println!("*** FOUND HOVER RESPONSE! ***");
                    found_hover_response = true;
                    break;
                }
                
                // Also check for any response with "result" field
                if message.contains("\"id\":") && message.contains("\"result\":") {
                    println!("*** FOUND SOME RESPONSE WITH RESULT! ***");
                }
            }
            Err(e) => {
                println!("Failed to read message {}: {}", messages_read + 1, e);
                break;
            }
        }
    }
    
    if !found_hover_response {
        println!("No hover response found after {} messages", messages_read);
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
