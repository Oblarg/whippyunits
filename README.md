# WhippyUnits

A minimal units-of-measure library using const generics for dimensional quantities in Rust.

## Features

- **Const generic dimensional analysis**: Compile-time dimensional safety using const generics
- **Clean declaration syntax**: `5.meters()`, `2.0.kilograms()`, `1.hours()`
- **Multiple unit scales**: Kilometer, meter, millimeter; kilogram, gram, milligram; hour, minute, second
- **Automatic unit conversion**: Values stored in base units (SI) with display in chosen scale
- **Type safety**: Dimensional mismatches caught at compile time

## Example

```rust
use whippyunits::quantity::{LengthExt, MassExt, TimeExt};

fn main() {
    // Basic unit declarations
    let distance = 5.0.meters();
    let mass = 2.0.kilograms();
    let time = 1.0.hours();

    println!("Distance: {}", distance);  // "5 m"
    println!("Mass: {}", mass);          // "2 kg"
    println!("Time: {}", time);          // "1 h"

    // Unit conversions
    let distance_km = distance.with_unit(whippyunits::unit::kilometers());
    let mass_g = mass.with_unit(whippyunits::unit::grams());
    let time_min = time.with_unit(whippyunits::unit::minutes());

    println!("Distance in km: {}", distance_km);  // "0.005 km"
    println!("Mass in grams: {}", mass_g);        // "2000 g"
    println!("Time in minutes: {}", time_min);    // "60 min"

    // Integer literals work too
    let int_distance = 10.meters();
    let int_mass = 5.kilograms();
    let int_time = 30.seconds();
}
```

## Design

### Dimensional Analysis

The library uses const generics to represent dimensional powers:

```rust
pub struct Dimension<const M: i32, const L: i32, const T: i32>;
```

Where:
- `M` = Mass exponent
- `L` = Length exponent  
- `T` = Time exponent

Common dimensions are provided as type aliases:
- `Mass = Dimension<1, 0, 0>`
- `Length = Dimension<0, 1, 0>`
- `Time = Dimension<0, 0, 1>`
- `Velocity = Dimension<0, 1, -1>`
- `Force = Dimension<1, 1, -2>`

### Unit Scales

Each dimension supports multiple scales:

**Length:**
- Kilometer (1000 m)
- Meter (1 m) - base unit
- Millimeter (0.001 m)

**Mass:**
- Kilogram (1 kg) - base unit
- Gram (0.001 kg)
- Milligram (0.000001 kg)

**Time:**
- Hour (3600 s)
- Minute (60 s)
- Second (1 s) - base unit

### Storage and Display

Quantities are stored internally in base units (SI) but can be displayed in any compatible scale:

```rust
let distance = 5.0.meters();           // stored as 5.0 m
let distance_km = distance.with_unit(whippyunits::unit::kilometers());
println!("{}", distance_km);           // "0.005 km"
```

## Requirements

- Rust nightly (for const generics support)
- Features: `adt_const_params`, `generic_const_exprs`

## Future Enhancements

- Arithmetic operations between quantities
- More dimensions (current, temperature, etc.)
- Scoped storage preferences
- Nalgebra integration for vectors/matrices
- Strictness levels for implicit conversions 