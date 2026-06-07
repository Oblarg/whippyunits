// This test verifies that comparison operators are scale-strict:
// quantities with different scales cannot be compared directly.

use whippyunits::default_declarators::*;

fn main() {
    let meters = 1.0.meters();
    let millimeters = 1000.0.millimeters();
    
    // This should fail to compile - different scales
    let _result = meters > millimeters;
}

