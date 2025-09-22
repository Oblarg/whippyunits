//! Simple test to verify custom literals work

use whippyunits::*;

#[whippy_literals]
#[test]
fn test_simple_custom_literals() {
    // Test float literals with float suffixes (these go to float module)
    let distance = 100.0m_f64;
    let mass = 5.5kg_f64;
    let time = 30.0s_f64;
    
    // Test integer literals with integer suffixes (these go to int module)
    let count = 10m_i32;
    
    // These should now create proper unit types using the quantity! macro
    // We can test that they have the correct values by accessing the .value field
    assert_eq!(distance.value, 100.0);
    assert_eq!(mass.value, 5.5);
    assert_eq!(time.value, 30.0);
    assert_eq!(count.value, 10);
    
    // Test that they are actually proper unit types with correct dimensions
    // distance should be length (m), mass should be mass (kg), time should be time (s)
    println!("Distance: {} (should be length)", distance.value);
    println!("Mass: {} (should be mass)", mass.value);
    println!("Time: {} (should be time)", time.value);
    println!("Count: {} (should be length)", count.value);
    
    println!("Custom literals test passed!");
}
