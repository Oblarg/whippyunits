use std::process::{Command, Stdio};
use std::io::Write;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader as TokioBufReader};
use tokio::io::AsyncBufReadExt;
use tokio::process::Command as TokioCommand;
use serde_json::json;
use anyhow::Result;

/// Send an LSP message to rust-analyzer
async fn send_lsp_message(stdin: &mut tokio::process::ChildStdin, message: &str) -> Result<()> {
    let content_length = message.len();
    let formatted_message = format!("Content-Length: {}\r\n\r\n{}", content_length, message);
    stdin.write_all(formatted_message.as_bytes()).await?;
    stdin.flush().await?;
    Ok(())
}

/// Read an LSP message from rust-analyzer
async fn read_lsp_message(reader: &mut TokioBufReader<tokio::process::ChildStdout>) -> Result<String> {
    let mut line = String::new();
    reader.read_line(&mut line).await?;
    
    // Parse Content-Length header
    let content_length: usize = line
        .trim()
        .strip_prefix("Content-Length: ")
        .ok_or_else(|| anyhow::anyhow!("Invalid Content-Length header: {}", line))?
        .parse()?;
    
    // Read the empty line
    let mut empty_line = String::new();
    reader.read_line(&mut empty_line).await?;
    
    // Read the JSON payload
    let mut json_payload = vec![0u8; content_length];
    reader.read_exact(&mut json_payload).await?;
    
    let message = String::from_utf8(json_payload)?;
    Ok(message)
}

/// Semantic token structure
#[derive(Debug)]
struct SemanticToken {
    line: u32,
    char: u32,
    length: u32,
    token_type: u32,
    token_modifiers: u32,
}

/// Decode delta-encoded semantic tokens
fn decode_semantic_tokens(data: &[u32]) -> Vec<SemanticToken> {
    let mut tokens = Vec::new();
    let mut line = 0u32;
    let mut char = 0u32;
    
    for chunk in data.chunks(5) {
        if chunk.len() == 5 {
            let line_delta = chunk[0];
            let char_delta = chunk[1];
            let length = chunk[2];
            let token_type = chunk[3];
            let token_modifiers = chunk[4];
            
            // Apply line delta
            line += line_delta;
            
            // Apply character delta - if we moved to a new line, char_delta is relative to start of line
            if line_delta > 0 {
                char = char_delta; // Reset to start of new line
            } else {
                char += char_delta; // Add to current position on same line
            }
            
            tokens.push(SemanticToken {
                line,
                char,
                length,
                token_type,
                token_modifiers,
            });
        }
    }
    
    tokens
}

/// Extract text for a token from the source code using UTF-8 byte positions
fn extract_token_text(token: &SemanticToken, source_lines: &[&str]) -> String {
    let line_idx = token.line as usize;
    if line_idx < source_lines.len() {
        let line = source_lines[line_idx];
        let byte_pos = token.char as usize; // This is actually a byte position, not char position
        let length = token.length as usize;
        let end_byte_pos = byte_pos + length;
        
        if end_byte_pos <= line.len() {
            // Extract the bytes and convert to string
            match std::str::from_utf8(&line.as_bytes()[byte_pos..end_byte_pos]) {
                Ok(s) => s.to_string(),
                Err(_) => String::new(),
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Analyze semantic tokens message and decode the data
fn analyze_semantic_tokens_message(message: &str, source_code: &str) {
    println!("\n=== ANALYZING SEMANTIC TOKENS MESSAGE ===");
    
    // Parse the JSON message
    match serde_json::from_str::<serde_json::Value>(message) {
        Ok(json_value) => {
            if let Some(result) = json_value.get("result") {
                if let Some(data) = result.get("data") {
                    if let Some(data_array) = data.as_array() {
                        // Convert to u32 array
                        let data_u32: Vec<u32> = data_array
                            .iter()
                            .filter_map(|v| v.as_u64().map(|n| n as u32))
                            .collect();
                        
                        println!("Raw data array (first 20 elements): {:?}", &data_u32[..data_u32.len().min(20)]);
                        println!("Total data elements: {}", data_u32.len());
                        
                        // Decode the tokens
                        let tokens = decode_semantic_tokens(&data_u32);
                        println!("Decoded {} tokens", tokens.len());
                        
                        // Split source code into lines
                        let source_lines: Vec<&str> = source_code.lines().collect();
                        
                        // Debug: Show source code lines
                        println!("\nSource code lines:");
                        for (i, line) in source_lines.iter().enumerate() {
                            println!("  Line {}: '{}'", i, line);
                        }
                        
                        // Debug: Show first 20 tokens with their source line
                        println!("\nFirst 20 tokens with source line context:");
                        for (i, token) in tokens.iter().take(20).enumerate() {
                            let text = extract_token_text(token, &source_lines);
                            let line_idx = token.line as usize;
                            let source_line = if line_idx < source_lines.len() {
                                source_lines[line_idx]
                            } else {
                                "OUT_OF_BOUNDS"
                            };
                            let line_len = source_line.len();
                            let char_pos = token.char as usize;
                            let end_pos = char_pos + token.length as usize;
                            
                            println!("  {}: line={}, char={}, length={}, type={}, modifiers={}, text='{}', source_line='{}' (line_len={}, char_pos={}, end_pos={}, valid={})", 
                                i, token.line, token.char, token.length, token.token_type, token.token_modifiers, text, source_line, line_len, char_pos, end_pos, end_pos <= line_len);
                        }
                        
                        // Debug: Show the raw data values for the first few tokens
                        println!("\nRaw data values for first 20 tokens:");
                        for i in 0..20 {
                            let start_idx = i * 5;
                            if start_idx + 4 < data_u32.len() {
                                println!("  Token {}: [line_delta={}, char_delta={}, length={}, type={}, modifiers={}]", 
                                    i, data_u32[start_idx], data_u32[start_idx + 1], data_u32[start_idx + 2], data_u32[start_idx + 3], data_u32[start_idx + 4]);
                            }
                        }
                        
                        // Print first 50 tokens with their text
                        println!("\nFirst 50 tokens:");
                        for (i, token) in tokens.iter().take(50).enumerate() {
                            let text = extract_token_text(token, &source_lines);
                            println!("  {}: line={}, char={}, length={}, type={}, modifiers={}, text='{}'", 
                                i, token.line, token.char, token.length, token.token_type, token.token_modifiers, text);
                        }
                        
                        // Look for potential whippyunits types
                        println!("\nLooking for whippyunits patterns...");
                        for (i, token) in tokens.iter().enumerate() {
                            let text = extract_token_text(token, &source_lines);
                            if text.contains("Quantity") || text.contains("whippyunits") || text.contains("meters") || 
                               text.contains("::") || text.contains("set_unit_preferences") || text.contains("distance") {
                                println!("  Potential whippyunits token {}: '{}' at line={}, char={}", 
                                    i, text, token.line, token.char);
                            }
                        }
                        
                        // Show all tokens with non-empty text (limit to first 100)
                        println!("\nAll tokens with non-empty text (first 100):");
                        let mut count = 0;
                        for (i, token) in tokens.iter().enumerate() {
                            if count >= 100 { break; }
                            let text = extract_token_text(token, &source_lines);
                            if !text.is_empty() {
                                println!("  {}: line={}, char={}, length={}, type={}, modifiers={}, text='{}'", 
                                    i, token.line, token.char, token.length, token.token_type, token.token_modifiers, text);
                                count += 1;
                            }
                        }
                    } else {
                        println!("Data is not an array");
                    }
                } else {
                    println!("No 'data' field in result");
                }
            } else {
                println!("No 'result' field in message");
            }
        }
        Err(e) => {
            println!("Failed to parse JSON: {}", e);
        }
    }
}

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

/// Test to verify the correct format for semantic tokens requests
#[tokio::test]
async fn test_semantic_tokens_format() -> Result<()> {
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
    
    // Send LSP message helper function
    async fn send_lsp_message(writer: &mut tokio::process::ChildStdin, content: &str) -> Result<()> {
        let message = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);
        writer.write_all(message.as_bytes()).await?;
        writer.flush().await?;
        Ok(())
    }
    
    // Read LSP message helper function
    async fn read_lsp_message(reader: &mut TokioBufReader<tokio::process::ChildStdout>) -> Result<String> {
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        
        if line.starts_with("Content-Length:") {
            let content_length: usize = line
                .trim_start_matches("Content-Length: ")
                .trim_end_matches("\r\n")
                .parse()?;
            
            // Read the empty line
            line.clear();
            reader.read_line(&mut line).await?;
            
            // Read the JSON content
            let mut buffer = vec![0u8; content_length];
            reader.read_exact(&mut buffer).await?;
            
            return Ok(String::from_utf8(buffer)?);
        }
        
        Ok(line)
    }
    
    // Try different initialization approaches
    println!("=== TESTING DIFFERENT INITIALIZATION APPROACHES ===");
    
    // Approach 1: Use exact same initialization as working inlay hint test
    println!("Approach 1: Same initialization as inlay hint test");
    let init_request_1 = json!({
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
    
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&init_request_1)?).await?;
    
    // Read initialization response
    let init_response = read_lsp_message(&mut reader).await?;
    println!("Init response (length: {}): {}", init_response.len(), init_response);
    
    // Send initialized notification
    let initialized_notification = json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&initialized_notification)?).await?;
    
    // Wait a moment
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
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
    
    // Wait for rust-analyzer to process
    tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
    
    // Try different semantic tokens request formats
    println!("\n=== TESTING DIFFERENT SEMANTIC TOKENS REQUEST FORMATS ===");
    
    // Format 1: Full document request
    println!("Format 1: Full document request");
    let semantic_request_1 = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/semanticTokens/full",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/"))
            }
        }
    });
    
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&semantic_request_1)?).await?;
    
    // Read multiple messages to find the response
    println!("Reading messages for semantic tokens response...");
    let mut messages_read = 0;
    let max_messages = 20;
    let mut found_semantic_response = false;
    
    while messages_read < max_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                messages_read += 1;
                println!("Message {}: {}", messages_read, message);
                
                // Check if this is our semantic tokens response
                if message.contains("\"id\":2") {
                    println!("*** FOUND SEMANTIC TOKENS RESPONSE! ***");
                    found_semantic_response = true;
                    
                    // Analyze the semantic tokens message
                    analyze_semantic_tokens_message(&message, &test_content);
                    break;
                }
                
                // Also check for any response with "result" field
                if message.contains("\"result\":") {
                    println!("*** FOUND RESPONSE WITH RESULT! ***");
                }
            }
            Err(e) => {
                println!("Failed to read message {}: {}", messages_read + 1, e);
                break;
            }
        }
    }
    
    if !found_semantic_response {
        println!("No semantic tokens response found in {} messages", messages_read);
    }
    
    // Wait a moment
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    
    // Format 2: Range request
    println!("Format 2: Range request");
    let semantic_request_2 = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "textDocument/semanticTokens/range",
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
                    "line": 20,
                    "character": 0
                }
            }
        }
    });
    
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&semantic_request_2)?).await?;
    
    // Read multiple messages for range request
    println!("Reading messages for range semantic tokens response...");
    messages_read = 0;
    found_semantic_response = false;
    
    while messages_read < max_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                messages_read += 1;
                println!("Range message {}: {}", messages_read, message);
                
                // Check if this is our semantic tokens response
                if message.contains("\"id\":3") {
                    println!("*** FOUND RANGE SEMANTIC TOKENS RESPONSE! ***");
                    found_semantic_response = true;
                    
                    // Analyze the range semantic tokens message
                    analyze_semantic_tokens_message(&message, &test_content);
                    break;
                }
            }
            Err(e) => {
                println!("Failed to read range message {}: {}", messages_read + 1, e);
                break;
            }
        }
    }
    
    if !found_semantic_response {
        println!("No range semantic tokens response found in {} messages", messages_read);
    }
    
    // Clean up
    std::env::set_current_dir(original_dir)?;
    
    Ok(())
}

#[tokio::test]
async fn test_cached_type_messages() -> Result<()> {
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
    
    // Send initialization request (using the working format from our semantic tokens test)
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
                }
            }
        }
    });
    
    // Send LSP message helper function
    async fn send_lsp_message(writer: &mut tokio::process::ChildStdin, content: &str) -> Result<()> {
        let message = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);
        writer.write_all(message.as_bytes()).await?;
        writer.flush().await?;
        Ok(())
    }
    
    // Read LSP message helper function
    async fn read_lsp_message(reader: &mut TokioBufReader<tokio::process::ChildStdout>) -> Result<String> {
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        
        if line.starts_with("Content-Length:") {
            let content_length: usize = line
                .trim_start_matches("Content-Length: ")
                .trim_end_matches("\r\n")
                .parse()?;
            
            // Read the empty line
            line.clear();
            reader.read_line(&mut line).await?;
            
            // Read the JSON content
            let mut buffer = vec![0u8; content_length];
            reader.read_exact(&mut buffer).await?;
            
            return Ok(String::from_utf8(buffer)?);
        }
        
        Ok(line)
    }
    
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
    
    // Wait a moment after initialized
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
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
    tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
    
    // Test 1: Document Symbols
    println!("\n=== TESTING DOCUMENT SYMBOLS ===");
    let document_symbols_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/documentSymbol",
        "params": {
            "textDocument": {
                "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/"))
            }
        }
    });
    
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&document_symbols_request)?).await?;
    
    // Read response for document symbols
    let mut found_document_symbols = false;
    for _ in 0..10 {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                if message.contains("\"id\":2") {
                    println!("*** FOUND DOCUMENT SYMBOLS RESPONSE! ***");
                    found_document_symbols = true;
                    analyze_document_symbols_message(&message, &test_content);
                    break;
                }
            }
            Err(e) => {
                println!("Failed to read document symbols response: {}", e);
                break;
            }
        }
    }
    
    if !found_document_symbols {
        println!("No document symbols response found");
    }
    
    // Test 2: Type Definition (for a specific position)
    println!("\n=== TESTING TYPE DEFINITION ===");
    let type_definition_request = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "textDocument/typeDefinition",
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
    
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&type_definition_request)?).await?;
    
    // Read response for type definition
    let mut found_type_definition = false;
    for _ in 0..10 {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                if message.contains("\"id\":3") {
                    println!("*** FOUND TYPE DEFINITION RESPONSE! ***");
                    found_type_definition = true;
                    analyze_type_definition_message(&message, &test_content);
                    break;
                }
            }
            Err(e) => {
                println!("Failed to read type definition response: {}", e);
                break;
            }
        }
    }
    
    if !found_type_definition {
        println!("No type definition response found");
    }
    
    // Test 3: Completion (for a position that might trigger type info)
    println!("\n=== TESTING COMPLETION ===");
    let completion_request = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "textDocument/completion",
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
    
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&completion_request)?).await?;
    
    // Read response for completion
    let mut found_completion = false;
    for _ in 0..10 {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                if message.contains("\"id\":4") {
                    println!("*** FOUND COMPLETION RESPONSE! ***");
                    found_completion = true;
                    
                    // Save the complete response to a file for inspection
                    std::fs::write("completion_response.json", &message)?;
                    println!("Complete completion response saved to completion_response.json");
                    
                    analyze_completion_message(&message, &test_content);
                    break;
                }
            }
            Err(e) => {
                println!("Failed to read completion response: {}", e);
                break;
            }
        }
    }
    
    if !found_completion {
        println!("No completion response found");
    }
    
    // Clean up
    std::env::set_current_dir(original_dir)?;
    
    Ok(())
}

#[tokio::test]
async fn test_completion_response_only() -> Result<()> {
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
                }
            }
        }
    });
    
    // Send LSP message helper function
    async fn send_lsp_message(writer: &mut tokio::process::ChildStdin, content: &str) -> Result<()> {
        let message = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);
        writer.write_all(message.as_bytes()).await?;
        writer.flush().await?;
        Ok(())
    }
    
    // Read LSP message helper function
    async fn read_lsp_message(reader: &mut TokioBufReader<tokio::process::ChildStdout>) -> Result<String> {
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        
        if line.starts_with("Content-Length:") {
            let content_length: usize = line
                .trim_start_matches("Content-Length: ")
                .trim_end_matches("\r\n")
                .parse()?;
            
            // Read the empty line
            line.clear();
            reader.read_line(&mut line).await?;
            
            // Read the JSON content
            let mut buffer = vec![0u8; content_length];
            reader.read_exact(&mut buffer).await?;
            
            return Ok(String::from_utf8(buffer)?);
        }
        
        Ok(line)
    }
    
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
    
    // Wait a moment after initialized
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
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
    tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
    
    // Test Completion only
    println!("\n=== TESTING COMPLETION ONLY ===");
    let completion_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/completion",
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
    
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&completion_request)?).await?;
    
    // Read response for completion
    let mut found_completion = false;
    for _ in 0..10 {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                if message.contains("\"id\":2") {
                    println!("*** FOUND COMPLETION RESPONSE! ***");
                    found_completion = true;
                    
                    // Save the complete response to a file for inspection
                    std::fs::write("completion_response.json", &message)?;
                    println!("Complete completion response saved to completion_response.json");
                    
                    // Also print the first 2000 characters for immediate inspection
                    let preview = if message.len() > 2000 {
                        format!("{}...", &message[..2000])
                    } else {
                        message.clone()
                    };
                    println!("Response preview:");
                    println!("{}", preview);
                    break;
                }
            }
            Err(e) => {
                println!("Failed to read completion response: {}", e);
                break;
            }
        }
    }
    
    if !found_completion {
        println!("No completion response found");
    }
    
    // Clean up
    std::env::set_current_dir(original_dir)?;
    
    Ok(())
}

#[tokio::test]
async fn test_inlay_hint_resolve_intercept() -> Result<()> {
    println!("=== Testing Inlay Hint Resolve Interception ===");
    
    // Start rust-analyzer
    let mut ra_process = TokioCommand::new("/Users/emichaelbarnettgmail.com/.rustup/toolchains/nightly-aarch64-apple-darwin/bin/rust-analyzer")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    
    let mut ra_stdin = ra_process.stdin.take().unwrap();
    let mut ra_stdout = ra_process.stdout.take().unwrap();
    let mut reader = TokioBufReader::new(ra_stdout);
    
    // Get the project directory
    let project_dir = std::env::current_dir()?.parent().unwrap().to_path_buf();
    let basic_test_path = project_dir.join("examples").join("basic_test.rs");
    let test_content = std::fs::read_to_string(&basic_test_path)?;
    
    println!("Sending initialization with inlay hint resolve capabilities...");
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
    
    // First, get the inlay hints to extract their data
    println!("Sending inlay hint request to get hint data...");
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
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&inlay_hint_request)?).await?;
    
    // Read the inlay hint response to extract data for resolve requests
    let mut inlay_hint_data = None;
    let mut messages_read = 0;
    let max_messages = 20;
    
    while messages_read < max_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                messages_read += 1;
                
                if message.contains("\"id\":2") && message.contains("Quantity") {
                    println!("*** FOUND INLAY HINT RESPONSE WITH QUANTITY! ***");
                    println!("Message: {}", message);
                    
                    // Extract the data field from the first inlay hint
                    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&message) {
                        if let Some(result) = json_value.get("result") {
                            if let Some(array) = result.as_array() {
                                if let Some(first_hint) = array.first() {
                                    if let Some(data) = first_hint.get("data") {
                                        inlay_hint_data = Some(data.clone());
                                        println!("Extracted inlay hint data: {}", data);
                                    }
                                }
                            }
                        }
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
    
    // Now send resolve requests for the inlay hints
    if let Some(data) = inlay_hint_data {
        println!("Sending inlay hint resolve request...");
        let resolve_request = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "textDocument/inlayHint/resolve",
            "params": data
        });
        send_lsp_message(&mut ra_stdin, &serde_json::to_string(&resolve_request)?).await?;
        
        // Read the resolve response
        println!("Reading resolve response...");
        let mut resolve_messages_read = 0;
        let max_resolve_messages = 10;
        
        while resolve_messages_read < max_resolve_messages {
            match read_lsp_message(&mut reader).await {
                Ok(message) => {
                    resolve_messages_read += 1;
                    println!("Resolve message {}: {}", resolve_messages_read, message);
                    
                    if message.contains("\"id\":3") {
                        println!("*** FOUND INLAY HINT RESOLVE RESPONSE! ***");
                        
                        // Check if it contains raw Quantity types
                        if message.contains("Quantity<") {
                            println!("*** FOUND RAW QUANTITY TYPES IN RESOLVE RESPONSE! ***");
                            println!("This is likely the source of the mouseover issue!");
                        }
                        
                        // Save the resolve response to a file for analysis
                        std::fs::write("inlay_hint_resolve_response.json", &message)?;
                        println!("Saved resolve response to inlay_hint_resolve_response.json");
                        break;
                    }
                }
                Err(e) => {
                    println!("Failed to read resolve message {}: {}", resolve_messages_read + 1, e);
                    break;
                }
            }
        }
    } else {
        println!("No inlay hint data found to resolve");
    }
    
    // Clean up
    ra_process.kill().await?;
    ra_process.wait().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_inlay_hint_refresh_intercept() -> Result<()> {
    println!("=== Testing Inlay Hint Refresh Interception ===");
    
    // Start rust-analyzer
    let mut ra_process = TokioCommand::new("/Users/emichaelbarnettgmail.com/.rustup/toolchains/nightly-aarch64-apple-darwin/bin/rust-analyzer")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    
    let mut ra_stdin = ra_process.stdin.take().unwrap();
    let mut ra_stdout = ra_process.stdout.take().unwrap();
    let mut reader = TokioBufReader::new(ra_stdout);
    
    // Get the project directory
    let project_dir = std::env::current_dir()?.parent().unwrap().to_path_buf();
    let basic_test_path = project_dir.join("examples").join("basic_test.rs");
    let test_content = std::fs::read_to_string(&basic_test_path)?;
    
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
    
    // Send inlay hint refresh request
    println!("Sending inlay hint refresh request...");
    let refresh_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/inlayHint/refresh",
        "params": {}
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&refresh_request)?).await?;
    
    // Read the refresh response
    println!("Reading refresh response...");
    let mut refresh_messages_read = 0;
    let max_refresh_messages = 10;
    
    while refresh_messages_read < max_refresh_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                refresh_messages_read += 1;
                println!("Refresh message {}: {}", refresh_messages_read, message);
                
                if message.contains("\"id\":2") {
                    println!("*** FOUND INLAY HINT REFRESH RESPONSE! ***");
                    
                    // Check if it contains raw Quantity types
                    if message.contains("Quantity<") {
                        println!("*** FOUND RAW QUANTITY TYPES IN REFRESH RESPONSE! ***");
                        println!("This is likely the source of the mouseover issue!");
                    }
                    
                    // Save the refresh response to a file for analysis
                    std::fs::write("inlay_hint_refresh_response.json", &message)?;
                    println!("Saved refresh response to inlay_hint_refresh_response.json");
                    break;
                }
            }
            Err(e) => {
                println!("Failed to read refresh message {}: {}", refresh_messages_read + 1, e);
                break;
            }
        }
    }
    
    // Try alternative refresh formats
    println!("Trying alternative refresh formats...");
    
    // Try as a notification instead of request
    let refresh_notification = json!({
        "jsonrpc": "2.0",
        "method": "textDocument/inlayHint/refresh",
        "params": {}
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&refresh_notification)?).await?;
    
    // Try workspace refresh
    let workspace_refresh = json!({
        "jsonrpc": "2.0",
        "method": "workspace/inlayHint/refresh",
        "params": {}
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&workspace_refresh)?).await?;
    
    // Try with specific document URI
    let document_refresh = json!({
        "jsonrpc": "2.0",
        "method": "textDocument/inlayHint/refresh",
        "params": {
            "uri": format!("file://{}", basic_test_path.to_string_lossy().replace("\\", "/"))
        }
    });
    send_lsp_message(&mut ra_stdin, &serde_json::to_string(&document_refresh)?).await?;
    
    // Read any responses to these refresh attempts
    let mut alt_refresh_messages_read = 0;
    let max_alt_refresh_messages = 15;
    
    while alt_refresh_messages_read < max_alt_refresh_messages {
        match read_lsp_message(&mut reader).await {
            Ok(message) => {
                alt_refresh_messages_read += 1;
                println!("Alt refresh message {}: {}", alt_refresh_messages_read, message);
                
                // Look for any inlay hint related messages
                if message.contains("inlayHint") || message.contains("Quantity") {
                    println!("*** FOUND INLAY HINT RELATED MESSAGE! ***");
                    std::fs::write("inlay_hint_alt_refresh_response.json", &message)?;
                    println!("Saved alt refresh response to inlay_hint_alt_refresh_response.json");
                }
            }
            Err(e) => {
                println!("Failed to read alt refresh message {}: {}", alt_refresh_messages_read + 1, e);
                break;
            }
        }
    }
    
    // Clean up
    ra_process.kill().await?;
    ra_process.wait().await?;
    
    Ok(())
}

/// Analyze document symbols message for type information
fn analyze_document_symbols_message(message: &str, source_code: &str) {
    println!("Document symbols message length: {}", message.len());
    println!("Complete document symbols JSON:");
    println!("{}", message);
    
    // Parse the JSON response
    match serde_json::from_str::<serde_json::Value>(message) {
        Ok(json_value) => {
            if let Some(result) = json_value.get("result") {
                if let Some(symbols) = result.as_array() {
                    println!("Found {} document symbols", symbols.len());
                    
                    // Look for symbols that might contain type information
                    for (i, symbol) in symbols.iter().enumerate() {
                        if let Some(name) = symbol.get("name") {
                            let name_str = name.as_str().unwrap_or("");
                            println!("  Symbol {}: '{}'", i, name_str);
                            
                            // Check for type information
                            if let Some(detail) = symbol.get("detail") {
                                let detail_str = detail.as_str().unwrap_or("");
                                if !detail_str.is_empty() {
                                    println!("    Detail: '{}'", detail_str);
                                    
                                    // Look for whippyunits types
                                    if detail_str.contains("Quantity") {
                                        println!("    *** FOUND WHIPPYUNITS TYPE IN DETAIL! ***");
                                    }
                                }
                            }
                            
                            // Check for kind (might indicate type information)
                            if let Some(kind) = symbol.get("kind") {
                                println!("    Kind: {}", kind);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to parse document symbols JSON: {}", e);
        }
    }
}

/// Analyze type definition message for type information
fn analyze_type_definition_message(message: &str, source_code: &str) {
    println!("Type definition message length: {}", message.len());
    println!("Complete type definition JSON:");
    println!("{}", message);
    
    // Parse the JSON response
    match serde_json::from_str::<serde_json::Value>(message) {
        Ok(json_value) => {
            if let Some(result) = json_value.get("result") {
                if let Some(locations) = result.as_array() {
                    println!("Found {} type definition locations", locations.len());
                    
                    for (i, location) in locations.iter().enumerate() {
                        println!("  Type definition {}: {:?}", i, location);
                        
                        // Look for URI and range information
                        if let Some(uri) = location.get("uri") {
                            println!("    URI: {}", uri);
                        }
                        if let Some(range) = location.get("range") {
                            println!("    Range: {:?}", range);
                        }
                    }
                } else if let Some(location) = result.as_object() {
                    println!("Found single type definition location: {:?}", location);
                }
            }
        }
        Err(e) => {
            println!("Failed to parse type definition JSON: {}", e);
        }
    }
}

/// Analyze completion message for type information
fn analyze_completion_message(message: &str, source_code: &str) {
    println!("Completion message length: {}", message.len());
    println!("Complete completion JSON:");
    println!("{}", message);
    
    // Parse the JSON response
    match serde_json::from_str::<serde_json::Value>(message) {
        Ok(json_value) => {
            if let Some(result) = json_value.get("result") {
                if let Some(items) = result.get("items").and_then(|v| v.as_array()) {
                    println!("Found {} completion items", items.len());
                    
                    // Look for completion items that might contain type information
                    for (i, item) in items.iter().enumerate() {
                        if let Some(label) = item.get("label") {
                            let label_str = label.as_str().unwrap_or("");
                            
                            // Check for detail (often contains type information)
                            if let Some(detail) = item.get("detail") {
                                let detail_str = detail.as_str().unwrap_or("");
                                if !detail_str.is_empty() {
                                    // Look for whippyunits types
                                    if detail_str.contains("Quantity") {
                                        println!("*** FOUND WHIPPYUNITS TYPE IN COMPLETION DETAIL! ***");
                                        println!("  Completion {}: '{}'", i, label_str);
                                        println!("    Detail: '{}'", detail_str);
                                        println!("    Full item: {:?}", item);
                                    }
                                }
                            }
                            
                            // Check for documentation
                            if let Some(docs) = item.get("documentation") {
                                if let Some(docs_str) = docs.get("value") {
                                    let docs_value = docs_str.as_str().unwrap_or("");
                                    if !docs_value.is_empty() {
                                        // Look for whippyunits types
                                        if docs_value.contains("Quantity") {
                                            println!("*** FOUND WHIPPYUNITS TYPE IN COMPLETION DOCS! ***");
                                            println!("  Completion {}: '{}'", i, label_str);
                                            println!("    Documentation: '{}'", docs_value);
                                            println!("    Full item: {:?}", item);
                                        }
                                    }
                                }
                            }
                            
                            // Also check the label itself
                            if label_str.contains("Quantity") {
                                println!("*** FOUND WHIPPYUNITS TYPE IN COMPLETION LABEL! ***");
                                println!("  Completion {}: '{}'", i, label_str);
                                println!("    Full item: {:?}", item);
                            }
                        }
                    }
                    
                    // If we didn't find any Quantity types, show a few items for context
                    let mut found_quantity = false;
                    for (i, item) in items.iter().enumerate() {
                        if let Some(label) = item.get("label") {
                            let label_str = label.as_str().unwrap_or("");
                            if let Some(detail) = item.get("detail") {
                                let detail_str = detail.as_str().unwrap_or("");
                                if detail_str.contains("Quantity") || label_str.contains("Quantity") {
                                    found_quantity = true;
                                    break;
                                }
                            }
                        }
                    }
                    
                    if !found_quantity {
                        println!("No Quantity types found in completion items. Showing first 10 items for context:");
                        for (i, item) in items.iter().take(10).enumerate() {
                            if let Some(label) = item.get("label") {
                                let label_str = label.as_str().unwrap_or("");
                                println!("  Completion {}: '{}'", i, label_str);
                                
                                if let Some(detail) = item.get("detail") {
                                    let detail_str = detail.as_str().unwrap_or("");
                                    if !detail_str.is_empty() {
                                        println!("    Detail: '{}'", detail_str);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to parse completion JSON: {}", e);
        }
    }
}
