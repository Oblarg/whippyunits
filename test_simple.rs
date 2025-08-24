extern crate whippyunits;
extern crate whippyunits_unit_macro;

use whippyunits_unit_macro::unit;

fn main() {
    // Test 1: Single unit
    let _test1: unit!(mm) = ();
    
    // Test 2: Single unit with exponent
    let _test2: unit!(mm^2) = ();
    
    // Test 3: Multiple units
    let _test3: unit!(mm * s) = ();
    
    println!("Simple tests completed");
}
