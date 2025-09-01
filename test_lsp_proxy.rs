use std::process::{Command, Stdio};
use std::io::{Write, Read};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start the LSP proxy
    let mut child = Command::new("./lsp-proxy/target/release/lsp-proxy")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.as_mut().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    // Create a hover response message
    let hover_response = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "contents": {
                "kind": "markdown",
                "value": "```rust\nlet result: Quantity<0, _, 1, _, 0, _, _, _>\nsize = 8, align = 0x8, no Drop\n\nRaw:\n\nlet result: Quantity<0, _, 1, _, 0, _, _, _>\nsize = 8, align = 0x8, no Drop\n```"
            }
        }
    });

    let json_str = serde_json::to_string(&hover_response)?;
    let message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);

    println!("Sending message to LSP proxy:");
    println!("{}", message);
    println!("---");

    // Send the message
    stdin.write_all(message.as_bytes())?;
    stdin.flush()?;

    // Read response
    let mut buffer = [0; 4096];
    let n = stdout.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..n]);

    println!("Response from LSP proxy:");
    println!("{}", response);

    // Read stderr for logs
    let mut stderr_buffer = [0; 4096];
    let n = stderr.read(&mut stderr_buffer)?;
    let stderr_output = String::from_utf8_lossy(&stderr_buffer[..n]);

    println!("Stderr from LSP proxy:");
    println!("{}", stderr_output);

    // Clean up
    child.kill()?;
    child.wait()?;

    Ok(())
}

