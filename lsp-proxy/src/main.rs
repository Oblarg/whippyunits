use std::process::{Command, Stdio};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, AsyncReadExt};
use tokio::process::{Child, Command as TokioCommand};
use anyhow::Result;
use log::{info, warn, error};

use whippyunits_lsp_proxy::{LspProxy, DisplayConfig};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    info!("🚀 WHIPPYUNITS LSP PROXY STARTING - This means VS Code is using our proxy!");
    
    // Read display configuration from environment variables
    let display_config = read_display_config();
    info!("Display config: verbose={}, unicode={}", display_config.verbose, display_config.unicode);
    
    // Find rust-analyzer
    let rust_analyzer_path = find_rust_analyzer()?;
    info!("Found rust-analyzer at: {}", rust_analyzer_path);
    
    // Get command line arguments (excluding program name)
    let args: Vec<String> = std::env::args().skip(1).collect();
    
    // Spawn rust-analyzer
    let mut rust_analyzer = spawn_rust_analyzer(&rust_analyzer_path, &args).await?;
    
    // Create proxy with configuration
    let proxy = LspProxy::with_config(display_config);
    
    // Set up bidirectional communication
    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    
    let mut ra_stdin = rust_analyzer.stdin.take().expect("Failed to get rust-analyzer stdin");
    let ra_stdout = rust_analyzer.stdout.take().expect("Failed to get rust-analyzer stdout");
    let ra_stderr = rust_analyzer.stderr.take().expect("Failed to get rust-analyzer stderr");
    
    // Spawn tasks for bidirectional communication
    let proxy_clone = proxy.clone();
    let editor_to_ra = tokio::spawn(async move {
        let mut reader = BufReader::new(stdin);
        let mut buffer = String::new();
        
        loop {
            buffer.clear();
            match read_lsp_message(&mut reader, &mut buffer).await {
                Ok(Some(message)) => {
                    // Process outgoing message (editor to rust-analyzer)
                    match proxy_clone.process_outgoing(&message) {
                        Ok(processed) => {
                            if let Err(e) = ra_stdin.write_all(processed.as_bytes()).await {
                                error!("Failed to write to rust-analyzer: {}", e);
                                break;
                            }
                            if let Err(e) = ra_stdin.flush().await {
                                error!("Failed to flush rust-analyzer stdin: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            warn!("Failed to process outgoing message: {}", e);
                            // Forward original message on error
                            if let Err(e) = ra_stdin.write_all(message.as_bytes()).await {
                                error!("Failed to write to rust-analyzer: {}", e);
                                break;
                            }
                        }
                    }
                }
                Ok(None) => break, // EOF
                Err(e) => {
                    error!("Failed to read from editor: {}", e);
                    break;
                }
            }
        }
    });
    
    let proxy_clone = proxy.clone();
    let ra_to_editor = tokio::spawn(async move {
        let mut reader = BufReader::new(ra_stdout);
        let mut buffer = String::new();
        
        loop {
            buffer.clear();
            match read_lsp_message(&mut reader, &mut buffer).await {
                Ok(Some(message)) => {
                    // Process incoming message (rust-analyzer to editor)
                    match proxy_clone.process_incoming(&message) {
                        Ok(processed) => {
                            if let Err(e) = stdout.write_all(processed.as_bytes()).await {
                                error!("Failed to write to editor: {}", e);
                                break;
                            }
                            if let Err(e) = stdout.flush().await {
                                error!("Failed to flush editor stdout: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            warn!("Failed to process incoming message: {}", e);
                            // Forward original message on error
                            if let Err(e) = stdout.write_all(message.as_bytes()).await {
                                error!("Failed to write to editor: {}", e);
                                break;
                            }
                        }
                    }
                }
                Ok(None) => break, // EOF
                Err(e) => {
                    error!("Failed to read from rust-analyzer: {}", e);
                    break;
                }
            }
        }
    });
    
    // Forward stderr from rust-analyzer to our stderr
    let stderr_forwarder = tokio::spawn(async move {
        let mut reader = BufReader::new(ra_stderr);
        let mut line = String::new();
        
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    eprint!("{}", line);
                }
                Err(e) => {
                    error!("Failed to read rust-analyzer stderr: {}", e);
                    break;
                }
            }
        }
    });
    
    // Wait for any task to complete (which means something went wrong or EOF)
    tokio::select! {
        _ = editor_to_ra => info!("Editor to rust-analyzer task completed"),
        _ = ra_to_editor => info!("Rust-analyzer to editor task completed"),
        _ = stderr_forwarder => info!("Stderr forwarder task completed"),
    }
    
    // Clean up
    if let Err(e) = rust_analyzer.kill().await {
        warn!("Failed to kill rust-analyzer: {}", e);
    }
    
    info!("LSP proxy shutting down");
    Ok(())
}

/// Read a complete LSP message from the reader
async fn read_lsp_message<R: AsyncBufReadExt + Unpin>(
    reader: &mut R,
    buffer: &mut String,
) -> Result<Option<String>, anyhow::Error> {
    // Read the Content-Length header
    buffer.clear();
    match reader.read_line(buffer).await {
        Ok(0) => return Ok(None), // EOF
        Ok(_) => {}
        Err(e) => return Err(anyhow::anyhow!("Failed to read Content-Length: {}", e)),
    }
    
    let content_length_line = buffer.trim();
    if content_length_line.is_empty() {
        return Ok(None);
    }
    
    // Parse Content-Length
    let content_length: usize = content_length_line
        .strip_prefix("Content-Length: ")
        .ok_or_else(|| anyhow::anyhow!("Invalid Content-Length header: {}", content_length_line))?
        .parse()?;
    
    // Read the empty line after headers
    buffer.clear();
    reader.read_line(buffer).await?;
    
    // Read the JSON payload
    buffer.clear();
    let mut json_buffer = vec![0u8; content_length];
    reader.read_exact(&mut json_buffer).await?;
    
    let json_payload = String::from_utf8(json_buffer)?;
    
    // Reconstruct the complete LSP message
    let message = format!("Content-Length: {}\r\n\r\n{}", content_length, json_payload);
    Ok(Some(message))
}

/// Read display configuration from environment variables
fn read_display_config() -> DisplayConfig {
    let verbose = std::env::var("WHIPPYUNITS_VERBOSE")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);
    
    let unicode = std::env::var("WHIPPYUNITS_UNICODE")
        .map(|v| v != "false" && v != "0") // Default to true unless explicitly false
        .unwrap_or(true);
    
    let include_raw = std::env::var("WHIPPYUNITS_INCLUDE_RAW")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);
    
    DisplayConfig { verbose, unicode, include_raw }
}

fn find_rust_analyzer() -> Result<String> {
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
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
        {
            return Ok(path);
        }
    }
    
    Err(anyhow::anyhow!("rust-analyzer not found. Please ensure it's installed and in your PATH."))
}

async fn spawn_rust_analyzer(path: &str, args: &[String]) -> Result<Child> {
    let mut cmd = TokioCommand::new(path);
    cmd.args(args)
       .stdin(Stdio::piped())
       .stdout(Stdio::piped())
       .stderr(Stdio::piped());
    
    // Explicitly set critical environment variables for rust-analyzer
    if let Ok(path) = std::env::var("PATH") {
        cmd.env("PATH", path);
    }
    if let Ok(developer_dir) = std::env::var("DEVELOPER_DIR") {
        cmd.env("DEVELOPER_DIR", developer_dir);
    }
    if let Ok(sdkroot) = std::env::var("SDKROOT") {
        cmd.env("SDKROOT", sdkroot);
    }
    if let Ok(rust_log) = std::env::var("RUST_LOG") {
        cmd.env("RUST_LOG", rust_log);
    }
    
    // Log the environment variables we're setting
    info!("Setting environment for rust-analyzer:");
    info!("  PATH: {}", std::env::var("PATH").unwrap_or_default());
    info!("  DEVELOPER_DIR: {}", std::env::var("DEVELOPER_DIR").unwrap_or_default());
    info!("  SDKROOT: {}", std::env::var("SDKROOT").unwrap_or_default());
    
    info!("Spawning rust-analyzer: {:?}", cmd);
    
    let child = cmd.spawn()?;
    Ok(child)
}
