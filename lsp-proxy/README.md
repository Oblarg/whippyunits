# WhippyUnits LSP Proxy

This package provides the LSP proxy for enhancing the developer experience when working with whippyunits in IDEs.

The LSP proxy intercepts and enhances LSP messages for VS Code and other editors, providing readable type information for whippyunits types.

**Note**: The `whippyunits-pretty` CLI tool is a separate crate (`whippyunits-pretty`) that uses the same core type conversion logic from this library, ensuring consistent behavior across all development environments.

## Installation

```bash
cargo build -p whippyunits-lsp-proxy --release
```

This builds the LSP proxy:
- `target/release/lsp-proxy` - LSP integration

## LSP Proxy

The LSP proxy intercepts communication between your editor and rust-analyzer, transforming whippyunits types in:
- **Hover tooltips** - Shows readable type information
- **Inlay hints** - Displays clean type annotations
- **Error messages** - Pretty-prints complex type signatures

### Setup

Configure your editor to use the LSP proxy instead of rust-analyzer directly:

```json
// VS Code settings.json
{
    "rust-analyzer.server.path": "/path/to/target/release/lsp-proxy"
}
```

### Configuration

Set environment variables to control the display format:

- `WHIPPYUNITS_VERBOSE=true` - Enable verbose output mode
- `WHIPPYUNITS_UNICODE=false` - Disable Unicode symbols
- `WHIPPYUNITS_INCLUDE_RAW=true` - Include raw type information

## Related: Pretty Printer CLI

The `whippyunits-pretty` CLI tool is a separate crate that uses the same core type conversion logic from this library. See the `whippyunits-pretty` crate for documentation on using the CLI tool to pretty-print rustc compiler output.

## Type Format Examples

| Original | Clean Mode | Verbose Mode |
|----------|------------|--------------|
| `Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>` | `Quantity<m; Length>` | `Quantity<meter; Length; [mass⁰, length¹, time⁰, current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2³²⁷⁶⁷, 3³²⁷⁶⁷, 5³²⁷⁶⁷, 10³²⁷⁶⁷, π⁰] f64>` |
| `Quantity<1, 0, 0, 0, 0, 0, 0, 0>` | `Quantity<kg; Mass>` | `Quantity<kilogram; Mass; [mass¹, length⁰, time⁰, current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2⁰, 3⁰, 5⁰, 10⁰, π⁰] f64>` |

## Architecture

The library provides:

- **`WhippyUnitsTypeConverter`** - Converts complex `Quantity<...>` types to readable formats
- **`DisplayConfig`** - Configuration for output modes (verbose, unicode, raw)
- **`LspProxy`** - Handles LSP message interception and transformation

## Performance

- **Zero Runtime Cost**: All type transformations are compile-time only
- **Stream Processing**: Processes LSP messages efficiently with minimal memory usage
- **Fast Pattern Matching**: Uses efficient regex patterns for type detection

## Related Tools

- **Core Library**: `whippyunits` - The underlying units library with prettyprint capabilities
- **Proc Macros**: `whippyunits-proc-macros` - Compile-time unit definitions
- **Pretty Printer**: `whippyunits-pretty` - CLI tool for pretty-printing compiler output (uses this library)