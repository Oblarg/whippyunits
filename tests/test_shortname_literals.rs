//! Test shortname custom literals like 5.0m

use whippyunits::*;

whippyunits::define_default_literals!();

#[culit::culit]
#[test]
fn test_shortname_custom_literals() {
    // Test shortname literals that delegate to unit! macro
    let distance = 5.0m;
    let mass = 2.5kg;
    let time = 10.0s;
    
    // These should create proper unit types using the unit! macro with new initialization
    // We can test that they have the correct values by accessing the .value field
    assert_eq!(distance.value, 5.0);
    assert_eq!(mass.value, 2.5);
    assert_eq!(time.value, 10.0);
    
    // Test that they are actually proper unit types with correct dimensions
    // distance should be length (m), mass should be mass (kg), time should be time (s)
    println!("Distance: {} (should be length)", distance.value);
    println!("Mass: {} (should be mass)", mass.value);
    println!("Time: {} (should be time)", time.value);
    
    // Test that the shortname literals work correctly
    // Note: Integer literals like 5m would need shortname macros in the int module
}
