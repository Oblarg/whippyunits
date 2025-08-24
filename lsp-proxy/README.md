# WhippyUnits LSP Proxy

An LSP (Language Server Protocol) proxy that intercepts rust-analyzer's hover responses and converts verbose `whippyunits` `Quantity<...>` types into human-readable format.

## Problem

When using the `whippyunits` library, rust-analyzer displays types like:
```
Quantity<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>
```

This proxy converts them to readable format like:
```
Quantity<(meter)> Length: Exponent 1 [Scale Index 0; meter], Mass: Exponent 0 [Scale Index MAX; unused], Time: Exponent 0 [Prime Factors p2:MAX, p3:MAX, p5:MAX; unused]
```

## How It Works

The proxy sits between your editor and rust-analyzer:
```
Editor ↔ LSP Proxy ↔ rust-analyzer
```

1. **Intercepts LSP messages** between editor and rust-analyzer
2. **Detects hover responses** containing `Quantity<...>` types
3. **Converts verbose types** to human-readable format
4. **Forwards improved responses** to the editor

## Installation

### Build from Source

```bash
cd lsp-proxy
cargo build --release
```

### Install Globally

```bash
cargo install --path .
```

## Usage

### VS Code

Update your VS Code settings to use the proxy instead of rust-analyzer directly:

```json
{
    "rust-analyzer.server.path": "/path/to/whippyunits-lsp-proxy"
}
```

### Neovim (with nvim-lspconfig)

```lua
require('lspconfig').rust_analyzer.setup({
    cmd = { '/path/to/whippyunits-lsp-proxy' },
    -- ... other rust-analyzer settings
})
```

### Helix

Update your `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "rust"
language-server = { command = "/path/to/whippyunits-lsp-proxy" }
```

### Emacs (with lsp-mode)

```elisp
(setq lsp-rust-analyzer-server-command '("/path/to/whippyunits-lsp-proxy"))
```

## Demo

Run the demo to see the type conversion in action:

```bash
cargo run --example demo
```

## Architecture

### Core Components

- **`LspProxy`**: Main proxy that processes LSP messages
- **`WhippyUnitsTypeConverter`**: Converts `Quantity<...>` types to readable format
- **`main.rs`**: Async LSP message forwarding with bidirectional communication

### Message Flow

1. **Editor → Proxy**: LSP requests (hover, completion, etc.)
2. **Proxy → rust-analyzer**: Forward requests unchanged
3. **rust-analyzer → Proxy**: LSP responses with type information
4. **Proxy → Editor**: Modified responses with improved type display

### Type Conversion Logic

The proxy parses `Quantity<...>` const generic parameters and converts them using the same logic as whippyunits' `Debug` implementation:

- **Length**: millimeter, meter, kilometer
- **Mass**: milligram, gram, kilogram  
- **Time**: millisecond, second, minute
- **Exponents**: Positive in numerator, negative in denominator
- **Scales**: Human-readable names instead of numeric indices

## Features

- ✅ **Zero-config**: Works with existing rust-analyzer setup
- ✅ **Editor-agnostic**: Works with any LSP-compatible editor
- ✅ **Real-time**: Processes hover responses as you type
- ✅ **Fallback**: Forwards original messages on parsing errors
- ✅ **Logging**: Configurable logging via `RUST_LOG` environment variable

## Development

### Running Tests

```bash
cargo test
```

### Running with Debug Logging

```bash
RUST_LOG=debug cargo run
```

### Integration Testing

The proxy includes integration tests that communicate with real rust-analyzer:

```bash
cargo test integration_tests -- --nocapture
```

## Limitations

- **whippyunits-specific**: Only improves `Quantity<...>` types
- **Hover-only**: Currently only processes hover responses (not completions, diagnostics, etc.)
- **Regex-based**: Uses pattern matching rather than full Rust AST parsing

## Future Enhancements

- Support for completion item type display
- Support for diagnostic message type improvement
- Configuration file for custom type mappings
- Plugin system for other type libraries

## Troubleshooting

### rust-analyzer Not Found

The proxy automatically searches for rust-analyzer in common locations:
- `rust-analyzer` (in PATH)
- `~/.cargo/bin/rust-analyzer`
- `~/.rustup/toolchains/*/bin/rust-analyzer`

If not found, ensure rust-analyzer is installed:
```bash
rustup component add rust-analyzer
```

### No Type Improvements Visible

1. **Check hover works**: Verify rust-analyzer hover works without the proxy
2. **Check file is in project**: rust-analyzer needs proper Cargo.toml setup
3. **Check dependencies**: Ensure whippyunits is in your Cargo.toml
4. **Enable logging**: Run with `RUST_LOG=debug` to see message processing

### Performance Issues

The proxy adds minimal overhead, but if you experience slowdowns:
1. **Disable logging**: Don't use `RUST_LOG=debug` in production
2. **Check rust-analyzer**: Performance issues are usually from rust-analyzer itself
3. **Restart editor**: Sometimes LSP connections need refreshing

## License

Same as whippyunits project.
