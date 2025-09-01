#![feature(generic_const_exprs)]

use whippyunits::*;

fn main() {
    // Example 1: Simple unit
    // Select "unit!(m)" and right-click → Refactor → Generate Unit Alias
    let distance: unit!(m) = 5.0.meters();
    
    // Example 2: Compound unit
    // Select "unit!(kg * m / s^2)" and right-click → Refactor → Generate Unit Alias
    let force: unit!(kg * m / s^2) = 10.0.newtons();
    
    // Example 3: Unit with exponent
    // Select "unit!(m^2)" and right-click → Refactor → Generate Unit Alias
    let area: unit!(m^2) = distance * distance;
    
    // Example 4: Velocity unit
    // Select "unit!(m / s)" and right-click → Refactor → Generate Unit Alias
    let velocity: unit!(m / s) = distance / 2.0.seconds();
    
    println!("Distance: {}", distance);
    println!("Force: {}", force);
    println!("Area: {}", area);
    println!("Velocity: {}", velocity);
}

// After using the refactor, you might end up with type aliases like:
// type Distance = whippyunits::Quantity<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>;
// type Force = whippyunits::Quantity<1, 0, 1, 1, -2, 0, 0, 0, 0>;
// type Area = whippyunits::Quantity<2, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>;
// type Velocity = whippyunits::Quantity<1, 0, 0, 9223372036854775807, -1, 0, 0, 0, 0>;
