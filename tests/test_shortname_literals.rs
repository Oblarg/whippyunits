//! Test shortname custom literals like 5.0m

use whippyunits::*;

#[culit::culit(whippyunits::default_declarators::literals)]
#[test]
fn test_shortname_custom_literals() {
    // Test shortname literals that delegate to unit! macro
    let distance: unit!(m) = 5.0m;
    let mass: unit!(kg) = 2.5kg;
    let time: unit!(s) = 10.0s;

    // These should create proper unit types using the unit! macro with new initialization
    // We can test that they have the correct values by accessing the .unsafe_value field
    assert_eq!(distance.unsafe_value, 5.0);
    assert_eq!(mass.unsafe_value, 2.5);
    assert_eq!(time.unsafe_value, 10.0);

    // Test that they are actually proper unit types with correct dimensions
    // distance should be length (m), mass should be mass (kg), time should be time (s)
    println!("Distance: {} (should be length)", distance.unsafe_value);
    println!("Mass: {} (should be mass)", mass.unsafe_value);
    println!("Time: {} (should be time)", time.unsafe_value);

    // Test that the shortname literals work correctly
    // Note: Integer literals like 5m would need shortname macros in the int module
}
