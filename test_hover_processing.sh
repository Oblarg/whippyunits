#!/bin/bash

# Test the LSP proxy with proper LSP message format
JSON_PAYLOAD='{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet result: Quantity<0, _, 1, _, 0, _, _, _>\nsize = 8, align = 0x8, no Drop\n\nRaw:\n\nlet result: Quantity<0, _, 1, _, 0, _, _, _>\nsize = 8, align = 0x8, no Drop\n```"}}}'

# Calculate content length
CONTENT_LENGTH=${#JSON_PAYLOAD}

# Create proper LSP message format
LSP_MESSAGE="Content-Length: $CONTENT_LENGTH\r\n\r\n$JSON_PAYLOAD"

echo "Testing LSP proxy with hover message..."
echo "Input: $JSON_PAYLOAD"
echo "---"

# Send to LSP proxy and capture output
echo -e "$LSP_MESSAGE" | RUST_LOG=debug ./lsp-proxy/target/release/lsp-proxy 2>&1 | head -20

