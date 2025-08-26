use crate::{LspProxy, DisplayConfig};
use serde_json::json;

#[test]
fn test_refresh_notification_handling() {
    let proxy = LspProxy::new();
    
    // Create a refresh notification message
    let refresh_notification = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "workspace/inlayHint/refresh"
    });
    
    // Convert to LSP message format
    let json_str = serde_json::to_string(&refresh_notification).unwrap();
    let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
    
    // Process the message
    let processed = proxy.process_incoming(&lsp_message).unwrap();
    
    // The refresh notification should be passed through unchanged
    // Note: The serialization might add null fields, so we check the JSON content instead
    let processed_lines: Vec<&str> = processed.lines().collect();
    let original_lines: Vec<&str> = lsp_message.lines().collect();
    let processed_json = processed_lines[2..].join("\n");
    let original_json = original_lines[2..].join("\n");
    
    // Parse both JSONs and compare the essential fields
    let processed_value: serde_json::Value = serde_json::from_str(&processed_json).unwrap();
    let original_value: serde_json::Value = serde_json::from_str(&original_json).unwrap();
    
    assert_eq!(processed_value["jsonrpc"], original_value["jsonrpc"]);
    assert_eq!(processed_value["id"], original_value["id"]);
    assert_eq!(processed_value["method"], original_value["method"]);
}

#[test]
fn test_resolve_request_handling() {
    let proxy = LspProxy::new();
    
    // Create a resolve request message
    let resolve_request = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "inlayHint/resolve",
        "params": {
            "position": {"line": 12, "character": 17},
            "label": [
                {"value": ": "},
                {"value": "Quantity"},
                {"value": "<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>"}
            ],
            "kind": 1,
            "data": {"file_id": 0, "hash": "123", "version": 1}
        }
    });
    
    // Convert to LSP message format
    let json_str = serde_json::to_string(&resolve_request).unwrap();
    let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
    
    // Process the message
    let processed = proxy.process_outgoing(&lsp_message).unwrap();
    
    // The resolve request should be passed through unchanged
    // Note: The serialization might add null fields, so we check the JSON content instead
    let processed_lines: Vec<&str> = processed.lines().collect();
    let original_lines: Vec<&str> = lsp_message.lines().collect();
    let processed_json = processed_lines[2..].join("\n");
    let original_json = original_lines[2..].join("\n");
    
    // Parse both JSONs and compare the essential fields
    let processed_value: serde_json::Value = serde_json::from_str(&processed_json).unwrap();
    let original_value: serde_json::Value = serde_json::from_str(&original_json).unwrap();
    
    assert_eq!(processed_value["jsonrpc"], original_value["jsonrpc"]);
    assert_eq!(processed_value["id"], original_value["id"]);
    assert_eq!(processed_value["method"], original_value["method"]);
    assert_eq!(processed_value["params"], original_value["params"]);
}

#[test]
fn test_resolve_response_handling() {
    let proxy = LspProxy::new();
    
    // Create a resolve response message with whippyunits types
    let resolve_response = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "result": {
            "position": {"line": 12, "character": 17},
            "label": [
                {"value": ": "},
                {"value": "Quantity"},
                {"value": "<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>"}
            ],
            "kind": 1,
            "tooltip": {
                "kind": "markdown",
                "value": "Type: `Quantity<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>`"
            },
            "textEdits": [
                {
                    "range": {"start": {"line": 12, "character": 8}, "end": {"line": 12, "character": 17}},
                    "newText": "let distance1: Quantity<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>"
                }
            ]
        }
    });
    
    // Convert to LSP message format
    let json_str = serde_json::to_string(&resolve_response).unwrap();
    let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
    
    // Process the message
    let processed = proxy.process_incoming(&lsp_message).unwrap();
    
    // Extract the JSON payload from the processed message
    let lines: Vec<&str> = processed.lines().collect();
    let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
    let processed_json = lines[json_start..].join("\n");
    
    // Parse and verify the result
    let processed_value: serde_json::Value = serde_json::from_str(&processed_json).unwrap();
    let result = &processed_value["result"];
    let label_array = result["label"].as_array().unwrap();
    
    // Should have 2 parts now (removed generic params)
    assert_eq!(label_array.len(), 2);
        
    // First part should be ": "
    assert_eq!(label_array[0]["value"], ": ");
        
    // Second part should be pretty-printed
    let second_part = &label_array[1];
    let pretty_value = second_part["value"].as_str().unwrap();
    assert!(pretty_value.contains("m"));
    
    // Should preserve additional properties from resolve response
    assert!(result.get("tooltip").is_some());
    assert!(result.get("textEdits").is_some());
    
    println!("Resolve response converted to: '{}'", pretty_value);
}

#[test]
fn test_message_type_detection() {
    let proxy = LspProxy::new();
    
    // Test refresh notification detection
    let refresh_msg = serde_json::from_str::<crate::LspMessage>(&serde_json::to_string(&json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "workspace/inlayHint/refresh"
    })).unwrap()).unwrap();
    
    assert!(proxy.is_refresh_notification(&refresh_msg));
    assert!(!proxy.is_resolve_request(&refresh_msg));
    assert!(!proxy.is_inlay_hint_response(&refresh_msg));
    
    // Test resolve request detection
    let resolve_msg = serde_json::from_str::<crate::LspMessage>(&serde_json::to_string(&json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "inlayHint/resolve",
        "params": {}
    })).unwrap()).unwrap();
    
    assert!(!proxy.is_refresh_notification(&resolve_msg));
    assert!(proxy.is_resolve_request(&resolve_msg));
    assert!(!proxy.is_inlay_hint_response(&resolve_msg));
    
    // Test inlay hint response detection
    let response_msg = serde_json::from_str::<crate::LspMessage>(&serde_json::to_string(&json!({
        "jsonrpc": "2.0",
        "id": 2,
        "result": [
            {
                "position": {"line": 12, "character": 17},
                "label": [{"value": ": "}, {"value": "Quantity"}],
                "kind": 1
            }
        ]
    })).unwrap()).unwrap();
    
    assert!(!proxy.is_refresh_notification(&response_msg));
    assert!(!proxy.is_resolve_request(&response_msg));
    assert!(proxy.is_inlay_hint_response(&response_msg));
}
