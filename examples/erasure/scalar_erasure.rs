//! Scalar Erasure Demo
//!
//! This example demonstrates how to safely erase dimensionless quantities (scalars)
//! to numeric types using `.into()`.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::quantity;

fn main() {
    println!("Scalar Erasure Demo");
    println!("==================\n");

    // Basic dimensionless ratio
    // Better r-a interaction and constant folding by using `quantity!` instead of `1.0m / 1.0m`:
    let ratio = quantity!(1.0, m / m);
    let erased: f64 = ratio.into();

    println!("   {} → {}", ratio, erased);
    assert_eq!(erased, 1.0);

    // Ratio with different scales (leaves a residual scale in the type)
    let ratio = quantity!(1.0, m / mm);
    let erased: f64 = ratio.into(); // erasure rescales to unity, so the residual scale in the type is correctly applied
    let unsafe_value: f64 = ratio.unsafe_value; // direct access to the unsafe value gives the storage value, which is 1.0 (probably surprising!)

    println!("   {} → {}", ratio, erased); // 1000.0 (probably what you expect!)
    println!("   .unsafe_value = {}", unsafe_value); // 1.0 (probably surprising!)
    assert_eq!(erased, 1000.0);

    // Different numeric types
    let erased_f64: f64 = ratio.into();
    let erased_f32: f32 = ratio.into();
    let erased_i32: i32 = ratio.into();

    println!("   {} → f64: {}", ratio, erased_f64);
    println!("   {} → f32: {}", ratio, erased_f32);
    println!("   {} → i32: {}", ratio, erased_i32);
}
