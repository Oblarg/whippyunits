use whippyunits::set_unit_preferences;
use whippyunits::api::*;

#[test]
fn test_local_quantity_macro() {
    // Set up scoped preferences with different units
    set_unit_preferences!(Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian);
    
    // Test with aggregate quantities like joules
    let energy = quantity!(100.0, J);
    println!("Energy: {:?}", energy);
    
    // Test with other compound units
    let force = quantity!(50.0, N);
    println!("Force: {:?}", force);
    
    let power = quantity!(25.0, W);
    println!("Power: {:?}", power);
}
