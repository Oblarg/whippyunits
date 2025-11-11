//! Basic Arithmetic Operations Demo
//!
//! This example demonstrates basic arithmetic operations with whippyunits quantities.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::unit;

#[culit::culit(whippyunits::default_declarators::literals)]
fn main() {
    println!("Basic Arithmetic Operations Demo");
    println!("================================\n");

    println!("1. Addition:");
    let distance1 = 5.0m;
    let distance2 = 3.0m;
    let total_distance = distance1 + distance2;
    println!("   {} + {} = {}", distance1, distance2, total_distance);

    println!("\n2. Subtraction:");
    let time1 = 30.0s;
    let time2 = 5.0s;
    let elapsed: unit!(s) = time1 - time2;
    println!("   {} - {} = {}", time1, time2, elapsed);

    println!("\n3. Scalar Multiplication:");
    let length = 5.0m;
    let scaled_length: unit!(m) = length * 3.0;
    println!("   {} * 3.0 = {}", length, scaled_length);

    println!("\n4. Scalar Division:");
    let distance = 10.0m;
    let half_distance: unit!(m) = distance / 2.0;
    println!("   {} / 2.0 = {}", distance, half_distance);

    println!("\n5. Quantity Multiplication:");
    let width = 5.0m;
    let height = 4.0m;
    let area = width * height;
    println!("   {} * {} = {}", width, height, area);

    println!("\n6. Quantity Division:");
    let distance = 100.0m;
    let time = 10.0s;
    let velocity = distance / time;
    println!("   {} / {} = {}", distance, time, velocity);

    println!("\n7. Compound Operations:");
    let result = 5.0m + 3.0m - 2.0m * 3.0 / 2.0;
    println!("   5.0 m + 3.0 m - 2.0 m * 3.0 / 2.0 = {}", result);

    println!("\n8. In-place Operations:");
    let mut distance = 5.0m;
    distance += 3.0m;
    println!("   After += 3.0 m: {}", distance);

    distance *= 2.0;
    println!("   After *= 2.0: {}", distance);

    distance /= 4.0;
    println!("   After /= 4.0: {}", distance);

    println!("\n9. Negative Quantities:");
    let pos = 7.0m;
    let neg = -5.0m;
    let sum = pos + neg;
    println!("   {} + {} = {}", pos, neg, sum);

    println!("\n10. Exponentiation:");
    let side = 3.0m;
    let volume = side * side * side;
    println!("   ({})³ = {}", side, volume);
}
