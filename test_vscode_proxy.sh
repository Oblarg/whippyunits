#!/bin/bash

echo "Testing LSP proxy with VS Code-like messages..."

# Set up environment
export PATH="/Users/emichaelbarnettgmail.com/.rustup/toolchains/nightly-aarch64-apple-darwin/bin:$PATH"
export RUST_LOG=debug

# Create a simple LSP initialization message
INIT_MESSAGE='{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"processId": 123, "rootUri": "file:///tmp/test", "capabilities": {}}}'

# Send it through the proxy
echo "Sending initialization message..."
echo -e "Content-Length: ${#INIT_MESSAGE}\r\n\r\n$INIT_MESSAGE" | timeout 10s ./lsp-proxy/target/release/lsp-proxy

echo "Test completed"


