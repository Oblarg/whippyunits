# WhippyUnits

A pure rust units-of-measure library for applied numerical analysis.

## Features

- **Simple declarator syntaxes**: Supports declarator methods (`5.0.meters()`), macros (`quantity!(5.0, m)`), and even literals (`5.0m_f64`)
- **Powerful unit literal DSL**: Easily define quantities in complex/bespoke dimensionalities, e.g. `quantity!(1, V * s^2 / m)`
- **Compile-time dimensional safety**: Catch dimensionally-incoherent expressions at compile time
- **Scale-generic dimension type traits**: Write scale-generic or dimension-generic arithmetic that "just works" when given a concrete type
- **Scale-generic dimension DSL**: Define scale-generic dimension traits for bespoke dimensions as easily as you can define quantities, e.g. `define_generic_dimension(BespokeQuantity, V * T^2 / L)`
- **Automatic unit conversion**: Type-driven generic rescaling using compile-time-computed conversion factors
- **No hidden/unnecessary flops**: Rescaling uses lossless log-scale arithmetic at all steps prior to exponentiation
- **Scoped storage preferences**: Set the storage scale individually for each scope
- **Language server integration**: Customized type rendering and text completion for unit types

## Example

```rust
// simple declarative syntax for standard quantities
let distance = 5.0.meters();
// or...
let distance = quantity!(5.0, m);
// or, in annotated context (see section on literals below)...
let distance = 5.0m;

// multiplication tracks dimensions
let area = 5.0.meters() * 5.0.meters();
// or...
let area = quantity!(5.0 * 5.0, m^2);

// dimensionally coherent operations permitted
let legal = area + area;
// dimensionally incoherent operations generate compile-time error
let illegal = area + distance;

// in some other source...

fn foo() {
    // scoped preferences lets you control the native units of your API
    // by generating traits and declarators fixed to scale preferences of your choosing
    define_base_units!(
        Kilogram,
        // Local declarators will now use millimeters
        Millimeter,
        Second,
        Ampere,
        Kelvin,
        Mole,
        Candela,
        Radian
    );

    // automatically stored as 5000.0 millimeters
    let distance_in_millimeters = 5.0.meters();

    ...
}

// The generic `rescale` function makes multiscale addition both ergonomic and safe:

// result: 1.001 meters
let sum_in_meters = 1.0.meters() + rescale(1.0.millimeters());
// result: 1001.0 millimeters
let sum_in_millimeters = rescale(1.0.meters()) + 1.0.millimeters();
// result: compilation error (scale incoherence)
let illegal_sum = 1.0.meters() + 1.0.millimeters();
```

## Scale-Generic Calculations

The `define_generic_dimension!` macro makes it easy to write contracts that are generic over scale while maintaining dimensional coherence guarantees.  Generic dimension traits for each "primitive" dimension (mass, length, etc) are pre-defined for convenience in the default declarator package.

```rust
use core::ops::Mul;
use whippyunits::define_generic_dimension;

// define scale-generic dimension trait Length, representing primitive dimension Length
define_generic_dimension!(Length, Length);
// define scale-generic dimension trait Area, representing a product of lengths
define_generic_dimension!(Area, Length^2);

// calculates an area from two lengths of generic dimension
fn area<D1: Length, D2: Length>(d1: D1, d2: D2) -> <D1 as Mul<D2>>::Output
where
    D1: Mul<D2>,
{
    d1 * d2
}

// regardless of input/output scales, the return type will always satisfy the Area trait
let millimeters_squared: impl Area = area(1.0.millimeters(), 1.0.millimeters());
let meters_squared: impl Area = area(1.0.meters(), 1.0.meters());

// the generic dimension DSL expresses arbitrary dimensional algebra, so all derived units can be covered
define_generic_dimension!(Energy, Mass * Length^2 / Time^2)
```

## Declarator Literals

Use the `#[whippy_literals]` attribute to enable custom literals with unit suffixes:

```rust
#[whippy_literals]
fn example() {
    let distance = 100.0m_f64;    // 100.0 meters (f64)
    let mass = 10g_i32;       // 10 grams (i32)
}
```

Supports all SI base units and prefixed units (mm, kg, μs, etc.) with type suffixes (f64, f32, i32, i64, u32, u64).

## Imperial and Affine Declarators

Imperial units with automatic SI conversion:

```rust
let length = 12.0.inches();     // converts to centimeters
let mass = 2.0.pounds();        // converts to kilograms
let temp = 32.0.fahrenheit();   // converts to kelvin (affine)
```

Affine quantities (like temperature) handle zero-point offsets automatically. Celsius and Fahrenheit are stored as Kelvin internally with proper conversion factors.

## Human-readable display and debug format

```rust
println!("{}", 5.0.millimeters()); 
// (5) Quantity<mm; Length>
println!("{:?}", 5.0.meters()); 
// (5_f64) Quantity<meter; Length; [mass⁰, length¹, time⁰, current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2⁰, 3⁰, 5⁰, 10⁰, π⁰]>

// Even handles complex SI values with correct aggregate-power-of-10 prefixing:
let joule = 1.0.milligrams() * 1.0.meters() * 1.0.meters() / 1.0.seconds() / 1.0.seconds();
println!("{}", joule);
// (1) Quantity<μ(kg·m²·s⁻²); μJ; Energy>
println!("{:?}", joule);
// (1_f64) Quantity<micro(kilogram·meter²·second⁻²); microJoule; Energy; [mass¹, length², time⁻², current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2⁰, 3⁰, 5⁰, 10⁻⁶, π⁰]>
```

## Print Format with Rescaling

Format quantities in any compatible unit with automatic conversion:

```rust
let distance = 5.0.kilometers();
println!("{}", distance.format_as("miles").unwrap());     // "3.1068559611866697 mi"
println!("{}", distance.format_as("feet").unwrap());      // "16404.199475065616 ft"

// With precision control
println!("{}", distance.format_as_with_precision("miles", 2).unwrap()); // "3.11 mi"
```

Use the `format_as!` macro for inline formatting: `format!("Distance: {}", format_as!(distance, "km"))`

## CLI Pretty Printer

The `whippyunits-pretty` tool transforms complex compiler error messages into readable formats:

```bash
# Pipe rustc output through the pretty printer
cargo check 2>&1 | whippyunits-pretty
# Converts: Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>
# Into:     Quantity<m; Length>

# Verbose mode with full dimension info
cargo check 2>&1 | whippyunits-pretty --verbose
# Converts: Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>
# Into:     Quantity<meter; Length; [mass⁰, length¹, time⁰, current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2⁰, 3⁰, 5⁰, 10⁰, π⁰] f64>
```

## LSP Proxy

The `lsp-proxy/` directory contains a Language Server Protocol proxy that intercepts rust-analyzer responses to enhance type display. It:

* pretty-prints Quantity types in human-readable form matching the `debug` trait behavior...
    * verbosely in hover info, including best-effort interpretation of partially-resolved types:
        ```rust
        // fully-resolved
        let result: Quantity<meter; Length; [mass⁰, length¹, time⁰, current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2⁰, 3⁰, 5⁰, 10⁰, π⁰] f64>
        size = 8, align = 0x8, no Drop

        Raw:

        let result: Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>
        size = 8, align = 0x8, no Drop

        // partially-resolved - may happen intentionally in scale-generic code,
        // or may happen due to rust-analyzer being lazy about const generic evaluation
        let frequency: Quantity<secondˀ; [mass⁰, length⁰, timeˀ, current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2⁰, 3⁰, 5⁰, 10⁰, π⁰] f64>
        size = 8, align = 0x8, no Drop

        Raw:

        let frequency: Quantity<0, 0, _, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>
        size = 8, align = 0x8, no Drop
        ```
    * tersely in inlay hints with backing datatype suffix (also including best-effort interpretation...):
        ```rust
        //          v inlay hint
        let distance: mm_f64 = 5.0.millimeters();
        ```
* replaces literal Quantity text completions with equivalent `unit!` macro declarations
    ```rust
    //          v inlay hint (like above)
    let distance: mm_f64 = 5.0.millimeters();
    //          double-clicks into correct macro declaration syntax for the unit!
    let distance: unit!(mm) = 5.0.millimeters();
    ```

See `lsp-proxy/README.md` for setup instructions.

## Requirements

```rust
#![feature(generic_const_exprs)] // compile-time dimensional arithmetic
```

### Usage

```bash
# Standard cargo commands (will use nightly automatically)
cargo check
cargo build
cargo test
```
