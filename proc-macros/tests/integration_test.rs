// Integration test for the define_generic_dimension proc macro
// This test verifies that the macro generates the expected code

use whippyunits_proc_macros::define_generic_dimension;

// Test the macro by actually calling it
define_generic_dimension!(LengthOrMass, Length, Mass);

// Test with time dimension (which has p2, p3, p5 parameters)
define_generic_dimension!(TimeOrLength, Time, Length);

// Test with all three dimensions
define_generic_dimension!(AnyDimension, Mass, Length, Time);

#[test]
fn test_macro_expansion() {
    // If we get here, the macro expanded successfully and the code compiles
    // This test verifies that the macro generates valid Rust code
    
    // We could add more specific assertions here if needed
    // For now, just checking that compilation succeeds
    assert!(true);
}

#[test]
fn test_trait_definitions() {
    // Test that the traits were generated
    // This is a compile-time test - if the traits don't exist, this won't compile
    
    // The LengthOrMass trait should exist
    fn _test_length_or_mass<T: LengthOrMass>(_x: T) {}
    
    // The TimeOrLength trait should exist  
    fn _test_time_or_length<T: TimeOrLength>(_x: T) {}
    
    // The AnyDimension trait should exist
    fn _test_any_dimension<T: AnyDimension>(_x: T) {}
    
    assert!(true);
}
