// This is a proper test of the define_generic_dimension proc macro
// It actually uses the macro to generate code and verifies it compiles

use whippyunits_proc_macros::define_generic_dimension;
use whippyunits::default_declarators::*;

// Test the macro by actually calling it
define_generic_dimension!(LengthOrMass, Length, Mass);

fn main() {
    // If we get here, the macro expanded successfully and the code compiles
    println!("All proc macro tests passed!");
    
    // Test that the traits work as expected by creating some Quantity types
    let foo = 1.0.meters();
    let bar = 1.0.milligrams();
    
    // Test that both implement the trait
    fn test_trait<T: LengthOrMass>(_x: T) {}
    
    test_trait(foo);
    test_trait(bar);
    
    println!("Foo: {}", foo);
    println!("Bar: {}", bar);
}
