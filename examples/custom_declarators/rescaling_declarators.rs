//! Rescaling Declarators
//!
//! Rescaling declarators automatically convert all quantities to specified
//! base unit scales for storage, while allowing declaration in any unit.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::define_unit_declarators;
use whippyunits::value;

// Create rescaling declarators: all quantities stored in mm, kg, s, etc.
define_unit_declarators!(
    mm_scale,
    Kilogram,  // Mass: kg
    Millimeter, // Length: mm
    Second,    // Time: s
    Ampere,    // Current: A
    Kelvin,    // Temperature: K
    Mole,      // Amount: mol
    Candela,  // Luminosity: cd
    Radian     // Angle: rad
);

fn main() {
    println!("Rescaling Declarators");
    println!("=====================\n");

    use mm_scale::*;

    // Declare in any unit, but stored in base scale (mm for length)
    let distance_km = quantity!(1.0, km);
    let distance_m = quantity!(1.0, m);
    let distance_cm = quantity!(1.0, cm);

    // All stored as mm internally
    println!("Declared in different units, stored as mm:");
    println!("   {} km → {} mm", value!(distance_km, km), value!(distance_km, mm));
    println!("   {} m → {} mm", value!(distance_m, m), value!(distance_m, mm));
    println!("   {} cm → {} mm\n", value!(distance_cm, cm), value!(distance_cm, mm));

    // Can add directly - all same scale internally
    let sum = distance_km + distance_m + distance_cm;
    println!("Can add directly (same internal scale):");
    println!("   {} km + {} m + {} cm = {} mm",
             value!(distance_km, km),
             value!(distance_m, m),
             value!(distance_cm, cm),
             value!(sum, mm));
}

