//! Type Assertions for Safe Multiplication and Division
//!
//! This example shows how to use unit!() to verify that multiplication
//! and division operations produce the expected dimensions, catching
//! errors at compile time instead of runtime.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::{quantity, unit};

#[culit::culit(whippyunits::default_declarators::literals)]
fn main() {
    println!("Type Assertions for Safe Multiplication and Division");
    println!("====================================================\n");

    println!("1. The Problem:");
    let width = 5.0m;
    let height = 4.0m;
    let _time = 10.0s;

    let _unchecked_area = width * height;
    println!("   let unchecked_area = width * height;  // ⚠️ Compiles, but unchecked");
    println!("   // let bad_area: unit!(m^2) = width * time;  // ❌ Compile error!");

    println!("\n2. The Solution:");
    let area: unit!(m ^ 2) = width * height;
    println!("   let area: unit!(m^2) = width * height;  // ✅ Verified");
    println!("   Result: {}", area);

    println!("\n3. Verifying Area:");
    let width = 5.0m;
    let height = 4.0m;
    let area: unit!(m ^ 2) = width * height;
    println!("   let area: unit!(m^2) = width * height;");
    println!("   Result: {}", area);

    println!("\n4. Verifying Volume:");
    let length = 3.0m;
    let width = 2.0m;
    let height = 1.0m;
    let volume: unit!(m ^ 3) = length * width * height;
    println!("   let volume: unit!(m^3) = length * width * height;");
    println!("   Result: {}", volume);

    println!("\n5. Verifying Velocity:");
    let distance = 100.0m;
    let time = 10.0s;
    let velocity: unit!(m / s) = distance / time;
    println!("   let velocity: unit!(m / s) = distance / time;");
    println!("   Result: {}", velocity);

    println!("\n6. Verifying Acceleration:");
    let velocity = quantity!(20.0, m / s);
    let time = 5.0s;
    let acceleration: unit!(m / s ^ 2) = velocity / time;
    println!("   let acceleration: unit!(m / s^2) = velocity / time;");
    println!("   Result: {}", acceleration);

    println!("\n7. Verifying Force:");
    let mass = 10.0kg;
    let acceleration = quantity!(9.81, m / s ^ 2);
    let force: unit!(N) = mass * acceleration;
    println!("   let force: unit!(N) = mass * acceleration;");
    println!("   Result: {}", force);

    println!("\n8. Verifying Energy:");
    let force = 10.0N;
    let distance = 5.0m;
    let energy: unit!(J) = force * distance;
    println!("   let energy: unit!(J) = force * distance;");
    println!("   Result: {}", energy);
}
