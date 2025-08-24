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

/// Test to investigate inlay hint refresh and resolve events
/// This will help us understand how to trigger and capture refresh/resolve messaging
#[tokio::test]
async fn test_inlay_hint_refresh_and_resolve() -> Result<()> {
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
    
    // Send initialization request with inlay hint capabilities including refresh support
    println!("Sending initialization with inlay hint refresh capabilities...");
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
                },
                "workspace": {
                    "inlayHint": {
                        "refreshSupport": true
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
    
    // Send initial inlay hint request to get base hints
    println!("Sending initial inlay hint request...");
    let initial_inlay_hint_request = json!({
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
                    "line": 50,
                    "character": 0
                }
            }
        }
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&initial_inlay_hint_request)?).await?;
    
    // Read initial inlay hint response
    let mut initial_hints = Vec::new();
    let mut messages_read = 0;
    let max_messages = 20;
    
    while messages_read < max_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                messages_read += 1;
                println!("Initial message {}: {}", messages_read, message);
                
                if message.contains("\"id\":2") {
                    println!("*** FOUND INITIAL INLAY HINT RESPONSE! ***");
                    initial_hints.push(message.clone());
                    break;
                }
            }
            Err(e) => {
                println!("Failed to read initial message {}: {}", messages_read + 1, e);
                break;
            }
        }
    }
    
    // Now try to trigger refresh events by making changes to the document
    println!("Attempting to trigger refresh events...");
    
    // Method 1: Send a didChange notification to trigger refresh
    println!("Method 1: Sending didChange notification...");
    let modified_content = test_content.replace("5.0.meters()", "10.0.meters()");
    let did_change_notification = json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didChange",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/")),
                "version": 2
            },
            "contentChanges": [
                {
                    "range": {
                        "start": {"line": 12, "character": 0},
                        "end": {"line": 12, "character": 15}
                    },
                    "text": "    let distance1 = 10.0.meters();"
                }
            ]
        }
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&did_change_notification)?).await?;
    
    // Wait for potential refresh events
    println!("Waiting for refresh events after didChange...");
    tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
    
    // Read messages for refresh events
    let mut refresh_messages = Vec::new();
    messages_read = 0;
    
    while messages_read < max_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                messages_read += 1;
                println!("Refresh message {}: {}", messages_read, message);
                
                // Look for refresh-related messages
                if message.contains("refresh") || message.contains("resolve") {
                    println!("*** FOUND REFRESH/RESOLVE MESSAGE! ***");
                    refresh_messages.push(message.clone());
                    analyze_refresh_message(&message);
                }
                
                // Also look for any notification messages that might be refresh events
                if !message.contains("\"id\":") && message.contains("\"method\":") {
                    println!("*** FOUND NOTIFICATION MESSAGE! ***");
                    refresh_messages.push(message.clone());
                    analyze_refresh_message(&message);
                }
            }
            Err(e) => {
                println!("Failed to read refresh message {}: {}", messages_read + 1, e);
                break;
            }
        }
    }
    
    // Method 2: Try to send a workspace/inlayHint/refresh request
    println!("Method 2: Sending workspace/inlayHint/refresh request...");
    let refresh_request = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "workspace/inlayHint/refresh",
        "params": {}
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&refresh_request)?).await?;
    
    // Read refresh response
    messages_read = 0;
    while messages_read < max_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                messages_read += 1;
                println!("Refresh response message {}: {}", messages_read, message);
                
                if message.contains("\"id\":3") {
                    println!("*** FOUND REFRESH RESPONSE! ***");
                    analyze_refresh_message(&message);
                    break;
                }
            }
            Err(e) => {
                println!("Failed to read refresh response message {}: {}", messages_read + 1, e);
                break;
            }
        }
    }
    
    // Method 3: Try to trigger resolve events by requesting specific inlay hints with data
    println!("Method 3: Attempting to trigger resolve events...");
    
    // First, get inlay hints with data that can be resolved
    let resolve_inlay_hint_request = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "textDocument/inlayHint",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/"))
            },
            "range": {
                "start": {
                    "line": 12,
                    "character": 0
                },
                "end": {
                    "line": 15,
                    "character": 0
                }
            }
        }
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&resolve_inlay_hint_request)?).await?;
    
    // Read inlay hints with data
    let mut hints_with_data = Vec::new();
    messages_read = 0;
    
    while messages_read < max_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                messages_read += 1;
                println!("Resolve hint message {}: {}", messages_read, message);
                
                if message.contains("\"id\":4") {
                    println!("*** FOUND INLAY HINTS WITH DATA! ***");
                    hints_with_data.push(message.clone());
                    
                    // Try to extract data from hints and send resolve requests
                    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&message) {
                        if let Some(result) = json_value.get("result") {
                            if let Some(hints_array) = result.as_array() {
                                for (i, hint) in hints_array.iter().enumerate() {
                                    if let Some(data) = hint.get("data") {
                                        println!("Found hint {} with data: {}", i, data);
                                        
                                        // Send resolve request for this hint
                                        let resolve_request = json!({
                                            "jsonrpc": "2.0",
                                            "id": 5 + i,
                                            "method": "inlayHint/resolve",
                                            "params": {
                                                "position": hint.get("position"),
                                                "label": hint.get("label"),
                                                "kind": hint.get("kind"),
                                                "data": data
                                            }
                                        });
                                        
                                        println!("Sending resolve request for hint {}: {}", i, serde_json::to_string(&resolve_request)?);
                                        send_lsp_message(&mut ra_stdin, &serde_json::to_string(&resolve_request)?).await?;
                                    }
                                }
                            }
                        }
                    }
                    break;
                }
            }
            Err(e) => {
                println!("Failed to read resolve hint message {}: {}", messages_read + 1, e);
                break;
            }
        }
    }
    
    // Read resolve responses
    println!("Reading resolve responses...");
    messages_read = 0;
    while messages_read < max_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                messages_read += 1;
                println!("Resolve response message {}: {}", messages_read, message);
                
                if message.contains("\"id\":") && message.contains("5") {
                    println!("*** FOUND RESOLVE RESPONSE! ***");
                    analyze_refresh_message(&message);
                }
            }
            Err(e) => {
                println!("Failed to read resolve response message {}: {}", messages_read + 1, e);
                break;
            }
        }
    }
    
    // Method 4: Try to trigger refresh by changing workspace settings
    println!("Method 4: Attempting to trigger refresh via workspace settings...");
    let workspace_did_change_configuration = json!({
        "jsonrpc": "2.0",
        "method": "workspace/didChangeConfiguration",
        "params": {
            "settings": {
                "rust-analyzer": {
                    "inlayHints": {
                        "enable": true,
                        "typeHints": true,
                        "parameterHints": true,
                        "chainingHints": true
                    }
                }
            }
        }
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&workspace_did_change_configuration)?).await?;
    
    // Wait for potential refresh events
    println!("Waiting for refresh events after configuration change...");
    tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
    
    // Read any additional messages
    messages_read = 0;
    while messages_read < max_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                messages_read += 1;
                println!("Config change message {}: {}", messages_read, message);
                
                if message.contains("refresh") || message.contains("resolve") || 
                   (!message.contains("\"id\":") && message.contains("\"method\":")) {
                    println!("*** FOUND REFRESH/RESOLVE MESSAGE AFTER CONFIG CHANGE! ***");
                    analyze_refresh_message(&message);
                }
            }
            Err(e) => {
                println!("Failed to read config change message {}: {}", messages_read + 1, e);
                break;
            }
        }
    }
    
    // Save all captured messages for analysis
    println!("Saving captured messages for analysis...");
    let all_messages = {
        let mut messages = Vec::new();
        messages.extend(initial_hints);
        messages.extend(refresh_messages);
        messages.extend(hints_with_data);
        messages
    };
    
    if !all_messages.is_empty() {
        std::fs::write("inlay_hint_refresh_messages.json", serde_json::to_string_pretty(&all_messages)?)?;
        println!("Saved {} messages to inlay_hint_refresh_messages.json", all_messages.len());
    }
    
    // Clean up
    if let Err(e) = rust_analyzer.kill().await {
        eprintln!("Failed to kill rust-analyzer: {}", e);
    }
    
    // Restore original directory
    std::env::set_current_dir(original_dir)?;
    
    Ok(())
}

/// Test to capture live refresh events from a running LSP
/// This test will try to capture refresh events that occur naturally
#[tokio::test]
async fn test_live_refresh_event_capture() -> Result<()> {
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
    
    // Send initialization request
    println!("Sending initialization...");
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
                },
                "workspace": {
                    "inlayHint": {
                        "refreshSupport": true
                    }
                }
            }
        }
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&init_request)?).await?;
    
    // Read initialization response
    let init_response = read_lsp_message(&mut reader).await?;
    println!("Init response received");
    
    // Send initialized notification
    let initialized_notification = json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&initialized_notification)?).await?;
    
    // Send textDocument/didOpen notification
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
    
    // Now listen for any spontaneous refresh events
    println!("Listening for spontaneous refresh events...");
    let mut all_messages = Vec::new();
    let mut messages_read = 0;
    let max_messages = 50;
    let start_time = std::time::Instant::now();
    let timeout_duration = std::time::Duration::from_secs(30);
    
    while messages_read < max_messages && start_time.elapsed() < timeout_duration {
        match tokio::time::timeout(tokio::time::Duration::from_millis(1000), read_lsp_message(&mut reader)).await {
            Ok(Ok(message)) => {
                messages_read += 1;
                println!("Live message {}: {}", messages_read, message);
                all_messages.push(message.clone());
                
                // Check for refresh/resolve related messages
                if message.contains("refresh") || message.contains("resolve") {
                    println!("*** FOUND LIVE REFRESH/RESOLVE MESSAGE! ***");
                    analyze_refresh_message(&message);
                }
                
                // Check for any notification messages
                if !message.contains("\"id\":") && message.contains("\"method\":") {
                    println!("*** FOUND LIVE NOTIFICATION MESSAGE! ***");
                    analyze_refresh_message(&message);
                }
            }
            Ok(Err(e)) => {
                println!("Failed to read live message {}: {}", messages_read + 1, e);
                break;
            }
            Err(_) => {
                // Timeout - this is expected for live listening
                println!("Timeout waiting for message {}", messages_read + 1);
            }
        }
    }
    
    // Save live messages
    if !all_messages.is_empty() {
        std::fs::write("live_refresh_messages.json", serde_json::to_string_pretty(&all_messages)?)?;
        println!("Saved {} live messages to live_refresh_messages.json", all_messages.len());
    }
    
    // Clean up
    if let Err(e) = rust_analyzer.kill().await {
        eprintln!("Failed to kill rust-analyzer: {}", e);
    }
    
    // Restore original directory
    std::env::set_current_dir(original_dir)?;
    
    Ok(())
}

/// Analyze refresh/resolve messages to understand their structure
fn analyze_refresh_message(message: &str) {
    println!("\n=== REFRESH/RESOLVE MESSAGE ANALYSIS ===");
    println!("Raw message: {}", message);
    
    // Try to parse as JSON
    match serde_json::from_str::<serde_json::Value>(message) {
        Ok(json_value) => {
            println!("Successfully parsed as JSON");
            
            // Check if it's a notification
            if let Some(method) = json_value.get("method") {
                println!("Method: {}", method);
                
                // Check for refresh-related methods
                if let Some(method_str) = method.as_str() {
                    match method_str {
                        "workspace/inlayHint/refresh" => {
                            println!("*** WORKSPACE INLAY HINT REFRESH NOTIFICATION ***");
                        }
                        "textDocument/inlayHint/refresh" => {
                            println!("*** TEXT DOCUMENT INLAY HINT REFRESH NOTIFICATION ***");
                        }
                        "inlayHint/resolve" => {
                            println!("*** INLAY HINT RESOLVE REQUEST ***");
                        }
                        _ => {
                            println!("Other method: {}", method_str);
                        }
                    }
                }
            }
            
            // Check if it's a response
            if let Some(id) = json_value.get("id") {
                println!("Message ID: {}", id);
            }
            
            // Check if it has a result field
            if let Some(result) = json_value.get("result") {
                println!("Has result field: {}", result);
            }
            
            // Check for error field
            if let Some(error) = json_value.get("error") {
                println!("Has error field: {}", error);
            }
            
            // Check for params field
            if let Some(params) = json_value.get("params") {
                println!("Has params field: {}", params);
            }
        }
        Err(e) => {
            println!("Failed to parse as JSON: {}", e);
        }
    }
    
    println!("=== END REFRESH/RESOLVE ANALYSIS ===\n");
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
