# WhippyUnits

A zero-cost, pure rust units-of-measure library for applied computation.

## Features

- **Compile-time dimensional safety**: Catch dimensional and scale coherence errors at compile-time
- **Simple declarator syntaxes**: Supports declarator methods (`5.0.meters()`), macros (`quantity!(5.0, m)`), and even literals (`5.0m`)
- **Unit literal DSL**: Easily define quantities in complex/bespoke dimensionalities, e.g. `quantity!(1, V * s^2 / m)`
- **Scale-generic dimension type traits**: Write scale-generic or dimension-generic arithmetic that "just works" when given a concrete type
- **Scale-generic dimension DSL**: Define scale-generic dimension traits for bespoke dimensions as easily as you can define quantities, e.g. `define_generic_dimension!(BespokeQuantity, V * T^2 / L)`
- **Automatic unit conversion**: Type-driven generic rescaling using compile-time-computed conversion factors
- **No homotypes**: Prime-factorized scale encoding guarantees unique type representation - if two quantities represent the exact same thing, they are *guaranteed* to be the same type
- **No hidden flops**: Rescaling uses lossless log-scale arithmetic at all steps, and exponentiates by lookup table; integer types are guaranteed to use pure integer math, and floating point types use no more float math than necessary
- **Scoped base unit preferences**: Use `define_base_units!` to change the default storage units for a scope, fully decoupling storage scale (which can be chosen to satisfy numerical or software architecture constraints) from declarator syntax (which can match the natural units of the problem-space)
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

Use the `define_literals!()` macro to enable the use of unit literals in scopes tagged with the `#[culit::culit]` attribute:

```rust
whippyunits::define_literals!();

#[culit::culit]
fn example() {
    let distance = 100.0m; // 100.0 meters (f64)
    let mass = 10g;        // 10 grams (i32)
    let energy = 1.0J_f32; // 1 joule (f32)
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

## Scope-local base unit preferences

Use the `define_base_units!` macro to define a local declarator syntax that obeys a given set of base SI scale preferences for storage:

```rust
define_base_units!(
    Kilogram,
    // Local declarators will now use millimeters instead of meters
    Millimeter,
    Second,
    Ampere,
    Kelvin,
    Mole,
    Candela,
    Radian
);

#[culit::culit]
fn example() {
    let distance = 1.0.meters();      // automatically stores as 1000.0 millimeters
    let distance = quantity!(1.0, m); // so does this
    let distance = 1.0m;              // and so does this!

    // compound/derived units are "lifted" to the provided scale preferences
    let energy = 1.0J; // kg * mm^2 / s^2 yields microJoules, so this stores as 1000.0 * 1000.0 microJoules
}
```

## Human-readable display and debug format

```rust
println!("{}", 5.0.millimeters()); 
// (5) Quantity<mm; Length>
println!("{:?}", 5.0.meters()); 
// (5_f64) Quantity<meter; Length; [mass⁰, length¹, time⁰, current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2⁰, 3⁰, 5⁰, π⁰]>

// Even handles complex SI values with correct aggregate-power-of-10 prefixing:
let joule = 1.0.milligrams() * 1.0.meters() * 1.0.meters() / 1.0.seconds() / 1.0.seconds();
println!("{}", joule);
// (1) Quantity<μ(kg·m²·s⁻²); μJ; Energy>
println!("{:?}", joule);
// (1_f64) Quantity<micro(kilogram·meter²·second⁻²); microJoule; Energy; [mass¹, length², time⁻², current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2⁻⁶, 3⁰, 5⁻⁶, π⁰]>
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

The `whippyunits-pretty` tool provides type prettyprinting in compiler errors:

```bash
error[E0308]: mismatched types
  --> tests/test_incoherent_addition.rs:11:28
   |
11 |     let _result = length + time;
   |                            ^^^^ expected `1`, found `0`
   |
   = note: expected struct `L·(Mˀ·Iˀ·θˀ·Nˀ·Cdˀ·Aˀ)`
              found struct `T·(Mˀ·Iˀ·θˀ·Nˀ·Cdˀ·Aˀ)`

For more information about this error, try `rustc --explain E0308`.

// without prettifying...

error[E0308]: mismatched types
  --> tests/test_incoherent_addition.rs:11:28
   |
11 |     let _result = length + time;
   |                            ^^^^ expected `1`, found `0`
   |
   = note: expected struct `Quantity<_, 1, 0, _, _, _, _, _, _, _, _, _>`
              found struct `Quantity<_, 0, 1, _, _, _, _, _, _, _, _, _>`

For information about this error, try `rustc --explain E0308`.
```

## LSP Proxy

The `lsp-proxy/` directory contains a Language Server Protocol proxy that intercepts rust-analyzer responses to enhance type display. It:

* pretty-prints Quantity types in human-readable form matching the `debug` trait behavior...
    * verbosely in hover info, including best-effort interpretation of partially-resolved types:
        ```rust
        // fully-resolved
        let result: Quantity<meter; Length; [mass⁰, length¹, time⁰, current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2⁰, 3⁰, 5⁰, π⁰] f64>
        size = 8, align = 0x8, no Drop

        Raw:

        let result: Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>
        size = 8, align = 0x8, no Drop

        // partially-resolved - may happen intentionally in scale-generic code,
        // or may happen due to rust-analyzer being lazy about const generic evaluation
        let frequency: Quantity<secondˀ; [mass⁰, length⁰, timeˀ, current⁰, temperature⁰, amount⁰, luminosity⁰, angle⁰] [2⁰, 3⁰, 5⁰, π⁰] f64>
        size = 8, align = 0x8, no Drop

        Raw:

        let frequency: Quantity<0, 0, _, 0, 0, 0, 0, 0, 0, 0, 0, 0>
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
