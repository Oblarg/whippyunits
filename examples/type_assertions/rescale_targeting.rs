//! Type Assertions for Rescale Operations
//!
//! This example shows how to use unit!() to specify target types
//! for rescale operations, ensuring type safety and clarity.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::{api::rescale, quantity, unit};

#[culit::culit(whippyunits::default_declarators::literals)]
fn main() {
    println!("Type Assertions for Rescale Operations");
    println!("======================================\n");

    println!("1. When unit!() is Required:");
    let distance_m = 1.0m;

    // let distance_mm = rescale(distance_m);  // ❌ Compile error!
    println!("   // let distance_mm = rescale(distance_m);  // ❌ Compile error!");

    let distance_mm: unit!(mm) = rescale(distance_m);
    let distance_km: unit!(km) = rescale(distance_m);

    println!("   let distance_mm: unit!(mm) = rescale(distance_m);  // ✅ Required");
    println!("   Result: {}", distance_mm);
    println!("   let distance_km: unit!(km) = rescale(distance_m);  // ✅ Required");
    println!("   Result: {}", distance_km);

    println!("\n2. When unit!() is Optional:");
    let distance_m = 1.0m;
    let distance_mm = 500.0mm;

    let sum_mm = rescale(distance_m) + distance_mm;
    println!("   let sum_mm = rescale(distance_m) + distance_mm;  // ✅ Works without unit!()");
    println!("   Result: {}", sum_mm);

    let sum_mm_explicit: unit!(mm) = rescale(distance_m) + distance_mm;
    println!("   let sum_mm_explicit: unit!(mm) = rescale(distance_m) + distance_mm;");
    println!("   Result: {}", sum_mm_explicit);

    println!("\n3. Rescaling Compound Units:");
    let velocity_kmh = quantity!(100.0, km / h);
    let velocity_ms: unit!(m / s) = rescale(velocity_kmh);
    println!("   let velocity_ms: unit!(m / s) = rescale(velocity_kmh);");
    println!("   Result: {}", velocity_ms);
}
