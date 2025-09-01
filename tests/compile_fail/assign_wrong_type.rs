// This should fail to compile: can't assign length to mass variable
use whippyunits::*;
use whippyunits::default_declarators::*;

fn main() {
    let length = 5.0.meters();
    
    // This should fail to compile: can't assign length to mass variable
    let mass: Kilogram = length;
}
