use serde_json::Value;
use anyhow::Result;

pub mod inlay_hint_processor;
pub mod rustc_pretty;
pub mod quantity_detection;
pub mod unit_formatter;
pub mod lsp_structures;
pub mod hover_processor;

#[cfg(test)]
mod tests;

use unit_formatter::UnitFormatter;
use lsp_structures::LspMessage;
use hover_processor::HoverProcessor;
use inlay_hint_processor::InlayHintProcessor;

// Re-export for public API
pub use unit_formatter::DisplayConfig;

/// LSP Proxy that intercepts and modifies hover responses
#[derive(Clone)]
pub struct LspProxy {
    unit_formatter: UnitFormatter,
    display_config: DisplayConfig,
    hover_processor: HoverProcessor,
    inlay_hint_processor: InlayHintProcessor,
}

impl LspProxy {
    pub fn new() -> Self {
        let display_config = DisplayConfig::default();
        // Create a non-verbose config for inlay hints
        let inlay_hint_config = DisplayConfig {
            verbose: false,
            unicode: true,
            include_raw: false,
        };
        Self {
            unit_formatter: UnitFormatter::new(),
            display_config: display_config.clone(),
            hover_processor: HoverProcessor::new(display_config),
            inlay_hint_processor: InlayHintProcessor::with_config(inlay_hint_config),
        }
    }

    pub fn with_config(display_config: DisplayConfig) -> Self {
        // Create a non-verbose config for inlay hints
        let inlay_hint_config = DisplayConfig {
            verbose: false,
            unicode: display_config.unicode,
            include_raw: false,
        };
        Self {
            unit_formatter: UnitFormatter::new(),
            display_config: display_config.clone(),
            hover_processor: HoverProcessor::new(display_config),
            inlay_hint_processor: InlayHintProcessor::with_config(inlay_hint_config),
        }
    }

    /// Process an incoming LSP message (from rust-analyzer to editor)
    /// This expects a complete LSP message with Content-Length header
    pub fn process_incoming(&self, message: &str) -> Result<String> {
        // Parse the LSP message format
        let json_payload = self.extract_json_payload(message)?;
        
        // Fast string search to detect if this message contains Quantity types
        if !self.contains_quantity_types_fast(&json_payload) {
            // No Quantity types detected, return original message unchanged
            return Ok(message.to_string());
        }
        
        // Parse the JSON payload only if we detected Quantity types
        let mut lsp_msg: LspMessage = serde_json::from_str(&json_payload)?;
        
        // Check if this is a hover response
        if let Some(result) = &lsp_msg.result {
            if let Some(hover_content) = self.hover_processor.extract_hover_content(result) {
                let improved_content = self.hover_processor.improve_hover_content(hover_content);
                lsp_msg.result = Some(serde_json::to_value(improved_content)?);
            }
        }
        
        // Check if this is a refresh notification
        if self.is_refresh_notification(&lsp_msg) {
            // Pass through refresh notifications unchanged - they're notifications, not requests
            // The client should respond to this by re-requesting inlay hints
        }
        
        // Check if this is a resolve request
        if self.is_resolve_request(&lsp_msg) {
            // Pass through resolve requests unchanged - we'll intercept the response
        }
        
        // Check if this is an inlay hint response (including resolve responses)
        if let Some(result) = &lsp_msg.result {
            if self.is_inlay_hint_response(&lsp_msg) {
                eprintln!("*** INTERCEPTING INLAY HINT RESPONSE ***");
                let improved_result = self.process_inlay_hint_result(result)?;
                lsp_msg.result = Some(improved_result);
            }
        }
        
        // Reconstruct the LSP message format
        let new_json = serde_json::to_string(&lsp_msg)?;
        let content_length = new_json.len();
        Ok(format!("Content-Length: {}\r\n\r\n{}", content_length, new_json))
    }

    /// Process an outgoing LSP message (from editor to rust-analyzer)
    /// This expects a complete LSP message with Content-Length header
    pub fn process_outgoing(&self, message: &str) -> Result<String> {
        // Parse the LSP message format for logging purposes
        let json_payload = self.extract_json_payload(message)?;
        let lsp_msg: LspMessage = serde_json::from_str(&json_payload)?;
        
        // Log outgoing requests for debugging
        if let Some(method) = &lsp_msg.method {
            eprintln!("*** OUTGOING REQUEST: {} ***", method);
        }
        
        // Check if this is a refresh notification (from client to server)
        if self.is_refresh_notification(&lsp_msg) {
            eprintln!("*** INTERCEPTING OUTGOING REFRESH NOTIFICATION ***");
            // Pass through refresh notifications unchanged
        }
        
        // Check if this is a resolve request (from client to server)
        if self.is_resolve_request(&lsp_msg) {
            eprintln!("*** INTERCEPTING OUTGOING RESOLVE REQUEST ***");
            // Pass through resolve requests unchanged
        }
        
        // For outgoing messages, we just pass through unchanged
        // No content transformation needed - these are requests, not responses
        Ok(message.to_string())
    }

    /// Fast string search to detect Quantity types without deserialization
    /// This performs a performant string search for "Quantity<" patterns
    fn contains_quantity_types_fast(&self, json_payload: &str) -> bool {
        quantity_detection::contains_quantity_types_fast(json_payload)
    }

    /// Extract JSON payload from LSP message format
    fn extract_json_payload(&self, message: &str) -> Result<String> {
        let lines: Vec<&str> = message.lines().collect();
        
        // Find the empty line that separates headers from JSON
        let mut json_start = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                json_start = i + 1;
                break;
            }
        }
        
        if json_start >= lines.len() {
            return Err(anyhow::anyhow!("No JSON payload found in LSP message"));
        }
        
        // Join the remaining lines as JSON
        Ok(lines[json_start..].join("\n"))
    }

    /// Check if this is an inlay hint response (has result with inlay hint data)
    fn is_inlay_hint_response(&self, lsp_msg: &LspMessage) -> bool {
        // Check if the result contains inlay hint data structure
        if let Some(result) = &lsp_msg.result {
            // Check if result is an array (typical for inlay hint requests)
            if result.is_array() {
                // Check if any item in the array has inlay hint structure
                if let Some(array) = result.as_array() {
                    for item in array {
                        if let Some(item_obj) = item.as_object() {
                            if item_obj.contains_key("position") && item_obj.contains_key("label") {
                                return true;
                            }
                        }
                    }
                }
            }
            
            // Check if result is an object (typical for inlay hint resolve responses)
            if result.is_object() {
                if let Some(obj) = result.as_object() {
                    if obj.contains_key("position") && obj.contains_key("label") {
                        return true;
                    }
                }
            }
        }
        
        false
    }

    /// Check if this is a refresh notification
    fn is_refresh_notification(&self, lsp_msg: &LspMessage) -> bool {
        if let Some(method) = &lsp_msg.method {
            method == "workspace/inlayHint/refresh"
        } else {
            false
        }
    }

    /// Check if this is a resolve request
    fn is_resolve_request(&self, lsp_msg: &LspMessage) -> bool {
        if let Some(method) = &lsp_msg.method {
            method == "inlayHint/resolve"
        } else {
            false
        }
    }

    /// Process inlay hint result to pretty-print whippyunits types
    fn process_inlay_hint_result(&self, result: &Value) -> Result<Value> {
        // Create a full message structure for the inlay hint processor
        let full_message = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": result
        });
        
        // Convert to string for processing
        let message_str = serde_json::to_string(&full_message)?;
        
        // Process the inlay hint response using our instance processor
        let processed_str = self.inlay_hint_processor.process_inlay_hint_response(&message_str)?;
        
        // Parse back to Value
        let processed_value: Value = serde_json::from_str(&processed_str)?;
        
        // Extract just the result part (remove the jsonrpc wrapper)
        if let Some(processed_result) = processed_value.get("result") {
            Ok(processed_result.clone())
        } else {
            // If no result field, return the original
            Ok(result.clone())
        }
    }
}