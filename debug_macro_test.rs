use whippyunits::*;

fn main() {
    // Test 1: Single unit with non-zero exponent
    let _test1: unit!(mm) = ();
    
    // Test 2: Single unit with explicit exponent
    let _test2: unit!(mm^1) = ();
    
    // Test 3: Single unit with zero exponent
    let _test3: unit!(mm^0) = ();
    
    // Test 4: Multiple units, one dimension
    let _test4: unit!(mm * mm) = ();
    
    // Test 5: Multiple dimensions
    let _test5: unit!(mm * s) = ();
    
    // Test 6: Negative exponent
    let _test6: unit!(s^-1) = ();
    
    println!("Debug tests completed");
}
