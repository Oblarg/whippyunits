# WhippyUnits

A pure rust units-of-measure library for applied numerical analysis.

## Features

- **Simple syntax**: `5.meters()`, `2.0.kilograms()`, `1.hours()`
- **Compile-time dimensional safety**: Catch dimensionally-incoherent expressions at compile time
- **Automatic unit conversion**: Implicit and explicit rescaling using compile-time-computed conversion factors
- **No compile-time flops**: Rescaling implemented with log-space integers and a statically pre-generated lookup table
- **Scoped storage preferences**: Set the storage scale (eventually: backing datatype) individually for each scope
- **Language server integration**: Customized type rendering and text completion for unit types

## Example

```rust
// simple declarative syntax for standard quantities
let distance = 5.0.meters();

// multiplication tracks dimensions
let area = distance * distance;

// dimensionally coherent operations permitted
let legal = area + area;
// dimensionally incoherent operations generate compile-time error
let illegal = area + distance;

// in some other source...

fn foo() {
    // scoped preferences lets you control the native units of your declarator API
    // TODO: this will eventually be a proc macro method annotation that does not rely on expansion order
    set_unit_preferences!(
        MILLIMETER_SCALE,
        MILLIGRAM_SCALE,
        MILLISECOND_SCALE_ORDER
    );

    // automatically stored in millimeters, but we can declare in whatever unit is cnonvenient/readable
    let scopedDistance = 5.0.meters();

    ...
}

// Across the compilation unit you can control rescale behavior

// You may allow an implicit rescaling with one of several semantics...

// "Left-hand wins"
let sumInMeters = meters + millimeters;
// "Largest wins"
let sumInMeters = meters + millimeters;
// "Smallest wins"
let sumInMillimeters = meters + millimeters;
// or use "strict" semantics to require an explicit rescale (else compile error)
let sumInMillimeters = meters + millimeters.rescale();

// The `unit!` macro provides declarative syntax for unit types...
type Energy = unit!(kg * m^2 / s^2);
```

## Scale-Generic Calculations

Functions can be generic over unit scales while maintaining dimensional safety:

```rust
use core::ops::Mul;
use whippyunits::dimension_traits::*;

// dimension trait Length indicates a scale-generic length value
fn Area<D1: Length, D2: Length>(d1: D1, d2: D2) -> <D1 as Mul<D2>>::Output
where
    D1: Mul<D2>,
{
    d1 * d2
}

// Works with any length units
let area = Area(5.0.meters(), 3.0.meters()); // 15.0 m²
// deterministically autorescales if units mismatch and semantics allow it
// e.g., under left-hand-wins...
let area = Area(5.0.meters(), 3000.0.millimeters()); // 15.0 m² (converted)
// under 'strict' (compile error if rescale omitted)...
let area = Area(5.0.meters(), rescale(3000.0.millimeters())); // 15.0 m² (converted)
```

## Requirements

- Rust nightly (for const generics support)
- Features: `adt_const_params`, `generic_const_exprs`

## LSP Proxy

The `lsp-proxy/` directory contains a Language Server Protocol proxy that intercepts rust-analyzer responses to enhance type display. It:

    * pretty-prints Quantity types in human-readable form matching the `debug` trait behavior
    * replaces literal Quantity text completions with equivalent `unit!` macro declarations

See `lsp-proxy/README.md` for setup instructions.

## Development Setup

This project is configured to automatically use the nightly Rust toolchain from rustup, overriding any Homebrew Rust installation.

### Automatic Toolchain Selection

The project includes:
- `.cargo/config.toml` - Configures Cargo to use rustup's nightly toolchain
- `cargo-nightly` script - Alternative way to run cargo with nightly toolchain

### Usage

You can use either:
```bash
# Standard cargo commands (will use nightly automatically)
cargo check
cargo build
cargo test

# Or explicitly use the nightly script
./cargo-nightly check
./cargo-nightly build
./cargo-nightly test
```
