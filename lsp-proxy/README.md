# WhippyUnits Developer Experience Tools

This package provides two complementary tools for enhancing the developer experience when working with whippyunits:

1. **LSP Proxy** (`lsp-proxy`) - Intercepts and enhances LSP messages for VS Code and other editors
2. **Pretty Printer** (`whippyunits-pretty`) - CLI tool for pretty-printing rustc compiler output

Both tools share the same core type conversion logic, ensuring consistent behavior across all development environments.

## Installation

```bash
cargo build -p whippyunits-lsp-proxy --release
```

This builds both tools:
- `target/release/lsp-proxy` - LSP integration
- `target/release/whippyunits-pretty` - CLI pretty printer

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

## Pretty Printer CLI

The `whippyunits-pretty` CLI tool processes rustc compiler output, transforming complex whippyunits type signatures into readable formats.

### Usage

#### Basic Usage (stdin)
```bash
rustc --crate-type lib src/main.rs 2>&1 | whippyunits-pretty
```

#### File Input
```bash
whippyunits-pretty --input compiler_output.txt
```

#### Options
- `-v, --verbose`: Enable verbose output mode (shows full dimension and scale information)
- `-u, --no-unicode`: Disable Unicode symbols in output
- `-r, --include-raw`: Include raw type information alongside pretty-printed types
- `-d, --debug`: Enable debug logging
- `-f, --input <FILE>`: Read from file instead of stdin

### Examples

#### Clean Mode (Default)
```bash
$ echo "error: expected \`Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>\`, found \`{float}\`" | whippyunits-pretty
error: expected `Quantity<m; Length>`, found `{float}`
```

#### Verbose Mode
```bash
$ echo "error: expected \`Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>\`, found \`{float}\`" | whippyunits-pretty --verbose
error: expected `Quantity<meter; Length; [mass⁰, length¹, time⁰, current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2³²⁷⁶⁷, 3³²⁷⁶⁷, 5³²⁷⁶⁷, 10³²⁷⁶⁷, π⁰] f64>`, found `{float}`
```

#### With Raw Information
```bash
$ echo "error: expected \`Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>\`, found \`{float}\`" | whippyunits-pretty --include-raw
error: expected `Quantity<m; Length>`, found `{float}`
    Raw: Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>
```

### Integration with Build Systems

#### Cargo
```bash
# Add to your build script or Makefile
cargo check 2>&1 | whippyunits-pretty
```

#### Make
```makefile
check-pretty:
	rustc --crate-type lib src/main.rs 2>&1 | whippyunits-pretty
```

#### CI/CD
```yaml
- name: Check with pretty output
  run: cargo check 2>&1 | whippyunits-pretty
```

## Type Format Examples

| Original | Clean Mode | Verbose Mode |
|----------|------------|--------------|
| `Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>` | `Quantity<m; Length>` | `Quantity<meter; Length; [mass⁰, length¹, time⁰, current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2³²⁷⁶⁷, 3³²⁷⁶⁷, 5³²⁷⁶⁷, 10³²⁷⁶⁷, π⁰] f64>` |
| `Quantity<1, 0, 0, 0, 0, 0, 0, 0>` | `Quantity<kg; Mass>` | `Quantity<kilogram; Mass; [mass¹, length⁰, time⁰, current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2⁰, 3⁰, 5⁰, 10⁰, π⁰] f64>` |

## Architecture

Both tools share the same core components:

- **`WhippyUnitsTypeConverter`** - Converts complex `Quantity<...>` types to readable formats
- **`DisplayConfig`** - Configuration for output modes (verbose, unicode, raw)
- **`LspProxy`** - Handles LSP message interception and transformation
- **`RustcPrettyPrinter`** - Processes rustc output line-by-line

## Performance

- **Zero Runtime Cost**: All type transformations are compile-time only
- **Stream Processing**: Both tools process output line-by-line for minimal memory usage
- **Fast Pattern Matching**: Uses efficient regex patterns for type detection

## Related Tools

- **Core Library**: `whippyunits` - The underlying units library with prettyprint capabilities
- **Proc Macros**: `whippyunits-proc-macros` - Compile-time unit definitions
- **Default Dimensions**: `whippyunits-default-dimensions` - Standard unit definitions