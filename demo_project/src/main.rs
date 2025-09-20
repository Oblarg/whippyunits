use whippyunits::*;
use whippyunits::quantity_type::Quantity;
use whippyunits::default_declarators::{SILength, SIMass};

fn main() {
    // This will cause a dimensionally incoherent addition error
    let length: Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64> = 5.0_f64.meters();
    let mass: Quantity<1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64> = 10.0_f64.kilograms();
    
    // This should fail - can't add length + mass
    let result = length + mass;
    
    println!("Result: {}", result);
}
