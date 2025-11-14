//! Type Assertions for Rescale Operations
//!
//! This example shows how to use unit!() to specify target types
//! for rescale operations, ensuring type safety and clarity.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::{rescale, unit};

#[culit::culit(whippyunits::default_declarators::literals)]
fn main() {
    println!("Type Assertions for Rescale Operations");
    println!("======================================\n");

    // You can use unit!() to specify the target type for a rescale operation:
    let distance_mm: unit!(mm) = rescale!(1.0m, mm);
    println!("   Result: {}", distance_mm);

    // dimensionally invalid rescale operation will compile error:
    // let distance_km: unit!(km) = rescale!(1.0m, km); // ❌ Compile error!
}
