#![feature(generic_const_exprs)]
#![feature(impl_trait_in_bindings)]

// To run with different rescale behaviors:
// cargo run --example basic_test --features strict
// cargo run --example basic_test --features smaller_wins  
// cargo run --example basic_test --features left_hand_wins

use core::ops::{Add, Div, Mul, Sub};
use whippyunits::default_declarators::Millimeter;
use whippyunits::*;
use whippyunits::dimension_traits::*;
use whippyunits::scale_conversion::*;
use whippyunits::constants::*;
use whippyunits::quantity_type::Quantity;

fn main() {
    set_unit_preferences!(Milligram, Millimeter, Second);

    fn area<D1: Length, D2: Length>(d1: D1, d2: D2) -> <D1 as Mul<D2>>::Output
    where
        D1: Mul<D2>,
    {
        d1 * d2
    }

    // Test basic quantity creation and display
    let distance1 = 5.0.meters();
    let distance2: impl Length = 3.0.meters();

    println!("Distance1: {}", distance1);
    println!("Distance2: {}", distance2);
    println!("Distance1 debug: {:?}", distance1);
    println!("Distance2 debug: {:?}", distance2);

    // Try to add them
    println!("Attempting to add distances...");
    let result = distance1 + distance2;

    println!("Expected: 8.0 m");
    println!("Result: {}", result);
    println!("Result debug: {:?}", result);

    // Try to multiply them
    println!("Attempting to multiply distances...");
    let result2 = area(distance1, distance2);
    println!("Expected: 15.0 m^2");
    println!("Result2: {}", result2);
    println!("Result2 debug: {:?}", result2);

    // Try to divide the result by the second distance
    println!("Attempting to divide the result by the second distance...");
    let result3 = result2 / distance2;
    println!("Expected: 5.0 m");
    println!("Result3: {}", result3);
    println!("Result3 debug: {:?}", result3);

    // Try to divide the first distance by the result
    println!("Attempting to divide the first distance by the result...");
    let result4 = distance1 / result2;
    println!("Expected: 0.3333333333333333 m^-1");
    println!("Result4: {}", result4);
    println!("Result4 debug: {:?}", result4);

    let joule = 1.0.kilograms() * 1.0.meters() * 1.0.meters() / 1.0.seconds() / 1.0.seconds();
    println!("Joule: {}", joule);
    println!("Joule debug: {:?}", joule);
    
    // Test unrecognized dimension (mass^2 * length^3 / time^4)
    let complex_quantity = 1.0.kilograms() * 1.0.kilograms() * 1.0.meters() * 1.0.meters() * 1.0.meters() / 1.0.seconds() / 1.0.seconds() / 1.0.seconds() / 1.0.seconds();
    println!("Complex quantity: {}", complex_quantity);
    println!("Complex quantity debug: {:?}", complex_quantity);
}
