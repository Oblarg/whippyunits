# WhippyUnits LSP Proxy Setup Guide

This guide shows how to set up the WhippyUnits LSP Proxy to get improved hover information in VS Code.

## What It Does

The LSP proxy intercepts hover responses from rust-analyzer and converts verbose `Quantity<...>` types into readable formats:

**Before:**
```
Quantity<1, -1, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>
```

**After:**
```
Quantity<(millimeter)> Length: Exponent 1 [Scale Index -1; millimeter], Mass: Exponent 0 [Scale Index MAX; unused], Time: Exponent 0 [Prime Factors p2:MAX, p3:MAX, p5:MAX; unused]
```

## Setup Steps

### 1. Build the LSP Proxy

```bash
cd lsp-proxy
cargo build --release
```

### 2. Configure VS Code

The `.vscode/settings.json` file is already configured to use the proxy:

```json
{
    "rust-analyzer.server.path": "./lsp-proxy/target/release/lsp-proxy",
    "rust-analyzer.server.extraEnv": {
        "RUST_LOG": "debug"
    }
}
```

### 3. Test the Setup

1. **Restart VS Code** (or reload the window with `Cmd+Shift+P` → "Developer: Reload Window")
2. **Open a Rust file** with whippyunits code (like `test_hover.rs`)
3. **Hover over variables** with `Quantity` types to see the improved hover information

### 4. Verify It's Working

- Hover over variables like `distance`, `mass`, `velocity`, etc. in `test_hover.rs`
- You should see readable type information instead of verbose const generics
- Check the VS Code Output panel (View → Output → rust-analyzer) for proxy logs

## Troubleshooting

### If hover information doesn't improve:

1. **Check the Output panel**: View → Output → Select "rust-analyzer" to see logs
2. **Verify the binary exists**: `ls -la lsp-proxy/target/release/lsp-proxy`
3. **Restart VS Code completely**
4. **Check rust-analyzer is installed**: `rustup component add rust-analyzer`

### If rust-analyzer stops working entirely:

1. **Revert the settings**: Remove the `rust-analyzer.server.path` setting
2. **Restart VS Code**
3. **Check the proxy logs** for error messages

## How It Works

1. VS Code sends LSP requests to our proxy instead of rust-analyzer directly
2. The proxy forwards requests to the real rust-analyzer
3. When rust-analyzer sends back hover responses, the proxy intercepts them
4. The proxy converts `Quantity<...>` types to readable formats
5. The improved response is sent back to VS Code

## Files

- `lsp-proxy/src/main.rs` - Main LSP proxy server
- `lsp-proxy/src/lib.rs` - Type conversion logic
- `.vscode/settings.json` - VS Code configuration
- `test_hover.rs` - Test file for verifying hover improvements

