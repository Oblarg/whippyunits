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
    // By default, quantities use f64 for storage. This provides good precision
    // and range for most use cases.
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

    println!("   quantity!(5.0, m) = {} (f64, default)", distance_f64);
    println!("   quantity!(5.0, m, f32) = {} (f32)", distance_f32);
    println!("   quantity!(5, m, i32) = {} (i32)", distance_i32);
    println!();

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

    // Operations preserve storage type
    // Arithmetic operations preserve the storage type of the operands.
    println!("Operations Preserve Storage Type:");
    let distance1 = quantity!(3, m, i32);
    let distance2 = quantity!(2, m, i32);
    let sum: unit!(m, i32) = distance1 + distance2;

    println!("   {} + {} = {}", distance1, distance2, sum);
    println!("   Result is also i32");
    println!();

    // When to use different storage types
    // - f64: Default, good precision and range (most use cases)
    // - f32: Lower memory usage, sufficient precision for many applications
    // - i32/i64: Integer quantities, exact values, no floating point errors
    println!("When to Use Different Storage Types:");
    println!("   f64: Default choice, good precision and range");
    println!("   f32: Lower memory usage, sufficient for many applications");
    println!("   i32/i64: Integer quantities, exact values, no floating point errors");
    println!("   u32/u64: Unsigned quantities (e.g., counts, indices)");
    println!();

    // Integer storage types
    // Integer types are useful for exact quantities like counts or discrete measurements.
    println!("Integer Storage Types:");
    let count = quantity!(42, 1, i32);
    let items = quantity!(100, 1, u32);

    println!("   count = {} (i32)", count);
    println!("   items = {} (u32)", items);
    println!("   Integer types provide exact values without floating point errors");
    println!();

    // Floating point precision considerations
    // f32 has less precision than f64, which can matter for very large or very small values.
    println!("Floating Point Precision:");
    let large_f64 = quantity!(1e10, m, f64);
    let large_f32 = quantity!(1e10, m, f32);

    println!("   large_f64 = {}", large_f64);
    println!("   large_f32 = {}", large_f32);
    println!("   f32 has ~7 decimal digits of precision");
    println!("   f64 has ~15 decimal digits of precision");
    println!();
}
