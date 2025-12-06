# whippyunits-pretty

CLI tool for pretty-printing whippyunits types in rustc compiler output.

## Installation

```bash
cargo install whippyunits-pretty
```

Or build from source:

```bash
cargo build -p whippyunits-pretty --release
```

## Usage

### Basic (stdin)

```bash
cargo check 2>&1 | whippyunits-pretty
```

### From file

```bash
whippyunits-pretty -f compiler_output.txt
```

### Options

- `-v, --verbose`: Enable verbose output mode
- `-u, --no-unicode`: Disable Unicode symbols
- `-r, --include-raw`: Include raw type information
- `-d, --debug`: Enable debug logging
- `-f, --input <FILE>`: Read from file instead of stdin

## Example

**Before:**
```
error: expected `Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>`
```

**After:**
```
error: expected `Quantity<m; Length>`
```

## Integration

```bash
# Cargo
cargo check 2>&1 | whippyunits-pretty

# Makefile
check-pretty:
	cargo check 2>&1 | whippyunits-pretty
```

