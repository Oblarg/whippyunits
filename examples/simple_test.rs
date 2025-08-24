use whippyunits::*;
use whippyunits::default_declarators::{LengthExt, MassExt, TimeExt};

fn main() {
    // Test 1: Single unit
    let _test1: proc_unit!(mm) = 5.millimeters();
    
    // Test 2: Single unit with exponent
    let _test2: proc_unit!(mm^2) = 5.millimeters() * 5.millimeters();
    
    // Test 3: Multiple units
    let _test3: proc_unit!(mm * s) = 5.millimeters() * 5.seconds();
    
    println!("Simple tests completed");
}
