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
let area = 5.0m * 5.0m;
// or...
let area = quantity!(5.0 * 5.0, m^2);

// dimensionally coherent operations permitted
let legal = area + area;
// dimensionally incoherent operations generate compile-time error
let illegal = area + distance;

// The generic `rescale` function makes multiscale addition both ergonomic and safe:

// result: 1.001 meters
let sum_in_meters = 1.0m + rescale(1.0mm);
// result: 1001.0 millimeters
let sum_in_millimeters = rescale(1.0m) + 1.0mm;
// result: compilation error (scale incoherence)
let illegal_sum = 1.0m + 1.0mm;
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
let millimeters_squared: impl Area = area(1.0mm, 1.0mm);
let meters_squared: impl Area = area(1.0m, 1.0m);
// This works too; it will return a concrete type of milli(meters^2), since mm * m = (1/1000)(meters^2)
let mixed_units: impl Area = area(1.0mm, 1.0m);

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
println!("{}", 5.0m); 
// (5) Quantity<m, f64>
println!("{:?}", 5.0m); 
// (5_f64) Quantity<meter [length¹]>

// Even handles complex SI values with correct aggregate-power-of-10 prefixing:
let microjoule = quantity!(1.0, kg * mm^2 / s^2);
println!("{}", microjoule);
// (1) Quantity<μJ, f64>
println!("{:?}", microjoule);
// (1_f64) Quantity<microJoule (Energy) [2⁻⁶, 5⁻⁶] [mass¹, length², time⁻²], f64>
```

## Print Format with Rescaling

Format quantities in any compatible unit with automatic conversion:

```rust
println!("{}", 5.0km.fmt("miles"));     // "3.1068559611866697 mi"
println!("{}", 5.0ft.fmt("feet"));      // "16404.199475065616 ft"

// With precision control using format specifiers
println!("{:.2}", 5.0km.fmt("miles")); // "3.11 mi"
```

## CLI Pretty Printer

The `whippyunits-pretty` tool provides type prettyprinting in compiler errors:

```bash
# Example output after prettifying:
error[E0308]: mismatched types
  --> tests/compile_fail/add_length_to_time.rs:10:28
   |
10 |     let _result = length + time;
   |                            ^^^^ expected `1`, found `0`
   |
   = note: expected struct `Quantity<m, f64>`
              found struct `Quantity<s, f64>`

# Without prettifying (raw compiler output):
error[E0308]: mismatched types
  --> tests/compile_fail/add_length_to_time.rs:10:28
   |
10 |     let _result = length + time;
   |                            ^^^^ expected `1`, found `0`
   |
   = note: expected struct `Quantity<Scale, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>>`
              found struct `Quantity<Scale, Dimension<_M<0>, _L<0>, _T<1>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>>`
```

The tool converts complex generic type parameters into human-readable unit symbols, making error messages much clearer.

## LSP Proxy

The `lsp-proxy/` directory contains a Language Server Protocol proxy that intercepts rust-analyzer responses to enhance type display. It:

* pretty-prints Quantity types in human-readable form matching the `debug` trait behavior...
    * verbosely in hover info, including best-effort interpretation of partially-resolved types:
        ```rust
        let result: Quantity<meter [length¹], f64>

        ---
        Raw:

        result: Quantity<Scale, Dimension<_M<0>, _L<1>>>
        ---

        size = 8, align = 0x8, no Drop
        ```
    * tersely in inlay hints with backing datatype suffix (also including best-effort interpretation...):
        ```rust
        //          v inlay hint
        let distance: Quantity<m, f64> = 5.0mm;
        ```
* replaces literal Quantity text completions with equivalent `unit!` macro declarations
    ```rust
    //          v inlay hint (like above)
    let distance: Quantity<m, f64> = 5.0mm;
    //          double-clicks into correct macro declaration syntax for the unit!
    let distance: unit!(m) = 5.0mm;
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
