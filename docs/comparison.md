# Choosing a Rust Units Library

WhippyUnits and [uom](https://crates.io/crates/uom) are both zero-cost, type-safe units-of-measure libraries for Rust.  They share the same goal — catching dimensional errors at compile time — but make fundamentally different architectural choices.  This document lays out the trade-offs so you can decide which fits your project.

## The core difference: where does scale live?

**uom** normalizes every value to a base unit at construction time.  `1 kilometer` is stored as `1000.0` in the `meter` base unit.  Scale information is discarded; the runtime value *is* the quantity in base units.

**WhippyUnits** encodes scale in the type system.  `1 kilometer` is stored as `1.0` with `kilometer` scale carried as a type parameter.  The runtime value is exactly what you declared; conversion only happens when you explicitly ask for it.

This single decision has cascading consequences for precision, integer support, ergonomics, and safety.

## Integer storage

In uom, all values are normalized to the base unit.  For floating-point types this is invisible, but for integers it is destructive:

```rust
// uom
let length = i32::Length::new::<centimeter>(1);
// internally stored as 0 (truncated from 0.01 meters)
assert_eq!(length.get::<centimeter>(), 0); // not 1
```

Because `1 centimeter` normalizes to `0.01 meters`, which truncates to `0` in `i32`, the value is lost.  uom documents this as a limitation users must be aware of.

In WhippyUnits, scale lives in the type, so the stored value is always exactly what you declared:

```rust
// whippyunits
let length = 1_i32.centimeters();
assert_eq!(value!(length, cm, i32), 1); // exactly 1
```

Integer rescaling uses pure rational arithmetic (`value * numerator / denominator`) with no hidden floating-point at any step.  This makes WhippyUnits suitable for fixed-point control systems, embedded applications, and anywhere else integer precision matters.

## Floating-point precision

uom normalizes to base units at construction, which can introduce precision loss when values are far from the base scale:

```rust
// uom: 1 nanometer stored as 0.000000001 meters (f64)
let nm = f64::Length::new::<nanometer>(1.0);
// round-trip may accumulate representation error
```

WhippyUnits values stay at the magnitude you declared.  Conversion happens only at explicit `rescale()` boundaries, and uses log-scale arithmetic (exponent lookup tables rather than chained floating-point multiplications) to minimize error when it does occur.

## Scale safety

Because uom normalizes to base units at construction, values declared in different units (e.g. kilometers and meters) are already stored in the same base scale (meters) by the time they interact.  There is no cross-scale conversion in normal arithmetic — it happened implicitly at construction time.

uom's `autoconvert` feature (enabled by default) goes a step further: it allows interop between quantities from *different base unit sets* defined via `ISQ!` (e.g. an SI module using meters and a CGS module using centimeters).  With `autoconvert` off, these are incompatible types.

In practice, most uom users never define custom base unit sets, so `autoconvert` is invisible to them — everything is already in the same base scale.  The consequence, however, is that the conversion from declared units to base units is always implicit and always happens at construction.  There is no way to know from reading `a + b` whether a lossy normalization happened upstream.

WhippyUnits takes the opposite approach — scale is preserved in the type, so `km` and `m` are different types:

```rust
// whippyunits
let sum = 1.0.kilometers() + 1.0.meters(); // compile error

// explicit rescale makes the conversion visible
let sum = 1.0.kilometers() + rescale(1.0.meters()); // 1.001 km
let sum = rescale(1.0.kilometers()) + 1.0.meters();  // 1001.0 m
```

The `rescale()` call makes every conversion point visible in the source, and the target scale is always unambiguous from context.  For projects where everything should share a common storage scale, WhippyUnits' `define_unit_declarators!` macro provides uom-like behavior — declarators auto-normalize to chosen base units at construction — while still preserving scale in the type system.

## Angles as a dimension

uom treats angles as dimensionless.  This means angular velocity and frequency are the same type:

```rust
// uom
let angular_velocity = f64::AngularVelocity::new::<radian_per_second>(1.0);
let frequency = f64::Frequency::new::<hertz>(1.0);
// these are different named types, but the underlying dimension is identical:
// both are Quantity<..., Z0, Z0, N1, ...> (time^-1, all others zero)
```

WhippyUnits gives angle its own dimension (the eighth base dimension, alongside mass, length, time, etc.).  Angular velocity (`rad/s`) and frequency (`1/s`) are genuinely different types, and confusing them is a compile error.

For interop with trigonometric functions, angular quantities erase to their radian-scale numeric value via `.into()`:

```rust
// whippyunits
let sin_value: f64 = f64::sin(90.0.degrees().into()); // converts to radians, then erases
assert_eq!(sin_value, 1.0);
```

Compound units with an angular component (like `rad/m`) can also erase just the angular part via `.into()`, making it easy to move between angular and non-angular formulations in physics code.

## Construction syntax

uom uses an explicit constructor pattern:

```rust
// uom
let length = f64::Length::new::<kilometer>(5.0);
let velocity = f64::Velocity::new::<meter_per_second>(10.0);
```

WhippyUnits supports three declaration syntaxes:

```rust
// whippyunits — method syntax
let length = 5.0.kilometers();

// whippyunits — macro syntax (supports compound units)
let velocity = quantity!(10.0, m/s);
let energy = quantity!(1.0, kg*m^2/s^2);

// whippyunits — literal syntax (with culit attribute)
let length = 5.0km;
let energy = 1.0J;
```

All three support the full SI prefix range, compound units, and both UCUM and traditional notation.

## IDE and tooling support

WhippyUnits ships an LSP proxy that intercepts rust-analyzer to render `Quantity` types as human-readable unit expressions in hover info and inlay hints, and a CLI pretty-printer that converts raw generic type parameters in compiler errors into unit symbols.

uom does not provide comparable tooling; users work with raw generic type signatures in error messages and IDE hints.

## Where uom has the edge

uom supports storage types that WhippyUnits currently does not:

| Storage type | uom | WhippyUnits |
|---|---|---|
| `f32`, `f64` | yes | yes |
| `i8`..`i128`, `u8`..`u128` | yes | yes |
| `isize`, `usize` | yes | yes |
| `bigint`, `biguint` | yes | no |
| `rational`, `rational32`, `rational64`, `bigrational` | yes | no |
| `complex32`, `complex64` | yes | no |

uom also supports defining entirely custom systems of quantities from scratch via its `system!` macro.  WhippyUnits supports base-scale customization (`define_unit_declarators!`), but does not support arbitrary non-SI quantity systems.