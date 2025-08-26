#![feature(generic_const_exprs)]

use whippyunits::*;
use whippyunits::generated_constants::*;
use whippyunits::arithmetic::*;
use core::ops::{Add, Div, Mul, Sub};

// Include the generated declarative macro
mod generated_unit_macro {
    include!("../src/generated_unit_macro.rs");
}

fn main() {
    set_unit_preferences!(
        MILLIMETER_SCALE,
        MILLIGRAM_SCALE,
        MILLISECOND_SCALE_ORDER
    );
    

    // Test basic quantity creation and display
    let distance1 = 5.0.meters();
    let distance2 = 3.0.meters();

    println!("Distance1: {}", distance1);
    println!("Distance2: {}", distance2);
    println!("Distance1 debug: {:?}", distance1);
    println!("Distance2 debug: {:?}", distance2);

    // Try to add them
    println!("Attempting to add distances...");
    let result = distance1 + distance2;

    println!("Result: {}", result);
    println!("Expected: 8.0 m");
    println!("Result debug: {:?}", result);

    // Try to multiply them
    println!("Attempting to multiply distances...");
    let result2 = distance1 * distance2;
    println!("Result2: {}", result2);
    println!("Expected: 15.0 m^2");
    println!("Result2 debug: {:?}", result2);

    // Try to divide the result by the second distance
    println!("Attempting to divide the result by the second distance...");
    let result3= result2 / distance2;
    println!("Result3: {}", result3);
    println!("Expected: 5.0 m");
    println!("Result3 debug: {:?}", result3);

    // Try to divide the first distance by the result
    println!("Attempting to divide the first distance by the result...");
    let result4 = distance1 / result2;
    println!("Result4: {}", result4);
    println!("Expected: 0.3333333333333333 m^-1");
    println!("Result4 debug: {:?}", result4);
}
