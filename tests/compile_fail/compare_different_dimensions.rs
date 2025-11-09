// This test verifies that comparison operators require same dimension:
// quantities with different dimensions cannot be compared.

use whippyunits::default_declarators::*;

fn main() {
    let distance = 1.0.meters();
    let time = 1.0.seconds();
    
    // This should fail to compile - different dimensions
    let _result = distance > time;
}

