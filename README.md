# WhippyUnits

A pure rust units-of-measure library for applied numerical analysis.

## Features

- **Simple syntax**: `5.meters()`, `2.0.kilograms()`, `1.hours()`
- **Compile-time dimensional safety**: Catch dimensionally-incoherent expressions at compile time
- **Scale-generic dimension type traits**: Write scale-generic or dimension-generic arithmetic that "just works" when given a concrete type
- **Automatic unit conversion**: Implicit and explicit rescaling using compile-time-computed conversion factors
- **No hidden/unnecessary flops**: Rescaling uses lossless log-scale arithmetic at all steps *except* final data-value rescaling; since exponentiation is by lookup-table, fixed-point computations can remain fully fixed-point
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
    // scoped preferences lets you control the native units of your API
    // by generating traits and declarators fixed to scale preferences of your choosing
    set_unit_preferences!(
        Millimeter,
        Milligram,
        Second
    );

    // automatically stored in millimeters, but we can declare in whatever unit is convenient/readable
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
let sumInMillimeters = meters + rescale(millimeters);

// The `unit!` macro provides declarative syntax for unit types...
type Joule = unit!(kg * m^2 / s^2);
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

## Human-readable display and debug format

```rust
set_unit_preferences!(
    Millimeter,
    Milligram,
    Second
);

println!("{}", 5.0.meters()); // (5000) Quantity<mm; Length>
println!("{:?}", 5.0.meters()); // (5000) Quantity<millimeter; Length; [mass⁰(10⁻⁶), length¹(10⁻³), time⁰(2⁰, 3⁰, 5⁰)]>

// Even handles complex SI values with correct aggregate-power-of-10 prefixing:
let joule = 1.0.kilograms() * 1.0.meters() * 1.0.meters() / 1.0.seconds() / 1.0.seconds();
println!("{}", joule); // (1000000000000) Quantity<mg·mm²·s⁻²; pJ; Energy>
println!("{:?}", joule); 
// (1000000000000) Quantity<milligram·millimeter²·second⁻²; picoJoule; Energy; [mass¹(10⁻⁶), length²(10⁻³), time⁻²(2⁰, 3⁰, 5⁰)]>
```

## Scale-generic dimensional traits

Define custom dimensional type traits using intuitive algebraic expressions:

```rust
use whippyunits::define_generic_dimension;

// Define traits for derived physical quantities
define_generic_dimension!(Energy, Mass * Length^2 / Time^2);

// Use the generated traits in scale-generic algorithms
fn accept_generic_energy<E: Energy>(energy: E) {
    // this works on any energy, regardless of storage scale!
}

// so we can send it joules...
accept_generic_energy(1.0.kilograms() * 1.0.meters() * 1.0.meters() / 1.0.seconds() / 1.0.seconds());
// or any other unit of energy (standard or not!)
accept_generic_energy(1.0.milligrams() * 1.0.millimeters() * 1.0.meters() / 1.0.hours() / 1.0.days());
```

The DSL supports:
- **Basic operations**: `*`, `/`, `^` (exponentiation)
- **Operator precedence**: `^` has higher precedence than `*` and `/`
- **Parentheses**: For grouping complex expressions
- **Automatic scale parameter generation**: Only generates scale constants for non-zero dimensions

This creates a **dimensional type system** that's as expressive as mathematical notation while maintaining compile-time safety and zero runtime cost.

## LSP Proxy

The `lsp-proxy/` directory contains a Language Server Protocol proxy that intercepts rust-analyzer responses to enhance type display. It:

* pretty-prints Quantity types in human-readable form matching the `debug` trait behavior...
    * verbosely in hover info, including best-effort interpretation of partially-resolved types:
        ```rust
        // fully-resolved
        let distance1: Quantity<millimeter; Length; [mass⁰(unused), length¹(10⁻³), time⁰(unused)]>
        size = 8, align = 0x8, no Drop

        Raw:

        let distance1: Quantity<0, 9223372036854775807, 1, -3, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>

        // partially-resolved - may happen intentionally in scale-generic code,
        // or may happen due to rust-analyzer being lazy about const generic evaluation
        let distance2: Quantity<?; Length; [mass⁰(10ˀ), length¹(10ˀ), time⁰(2ˀ, 3ˀ, 5ˀ)]>
        size = 8, align = 0x8, no Drop

        Raw:

        let distance2: Quantity<0, _, 1, _, 0, _, _, _>
        ```
    * tersely in inlay hints (also including best-effort interpretation...):
        ```rust
        //          v fully resolved inlay hint
        let distance: mm = 5.0.millimeters();
        //        v partially resolved inlay hint
        let length: Length = getLength();
        ```
* replaces literal Quantity text completions with equivalent `unit!` macro declarations
    ```rust
    //          v inlay hint (like above)
    let distance: mm = 5.0.millimeters();
    //          double-clicks into correct macro declaration syntax for the unit!
    let distance: unit!(mm) = 5.0.millimeters();
    ```

See `lsp-proxy/README.md` for setup instructions.

## Requirements

Several nightly rust features:

```rust
#![feature(generic_const_exprs)] // compile-time dimensional arithmetic
#![feature(trait_alias)] // easier dimension trait name overloads
```
### Usage

```bash
# Standard cargo commands (will use nightly automatically)
cargo check
cargo build
cargo test
```
