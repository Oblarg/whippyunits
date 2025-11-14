//! Storage Types Demo
//!
//! This example demonstrates the storage type parameter, which controls the
//! underlying numeric type used to store quantity values.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::quantity;
use whippyunits::unit;

fn main() {
    println!("Storage Types Demo");
    println!("==================\n");

    // Default storage type (f64)
    println!("Default Storage Type (f64):");
    let distance = quantity!(5.0, m);
    let time = quantity!(10.0, s);

    println!("   distance = {}", distance);
    println!("   time = {}", time);
    println!("   Both use f64 storage by default");
    println!();

    // Explicit storage types with quantity! macro
    // You can specify the storage type as the third parameter to quantity!
    println!("Explicit Storage Types with quantity! macro:");
    let distance_f64 = quantity!(5.0, m, f64);
    let distance_f32 = quantity!(5.0, m, f32);
    let distance_i32 = quantity!(5, m, i32);

    // Type safety: cannot mix different storage types
    // Quantities with different storage types cannot be directly operated on,
    // even if they have the same dimension and scale.
    println!("Type Safety - Storage Type Mismatch:");
    let distance_f64 = quantity!(5.0, m, f64);
    let distance_f32 = quantity!(5.0, m, f32);

    println!("   distance_f64 = {}", distance_f64);
    println!("   distance_f32 = {}", distance_f32);
    println!("   // distance_f64 + distance_f32 would fail to compile ❌");
    println!("   // Different storage types cannot be mixed");
    println!();
}
