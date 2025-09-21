use whippyunits::define_base_units;
use whippyunits::api::*;

#[test]
fn test_local_quantity_macro() {
    // Set up scoped preferences with different units
    define_base_units!(Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian);
    
    // Test with f64 (default)
    let energy_f64 = quantity!(100.0, J);
    println!("Energy (f64): {:?}", energy_f64);
    
    // Test with i32
    let energy_i32 = quantity!(100, J, i32);
    println!("Energy (i32): {:?}", energy_i32);
    
    // Test with i64
    let energy_i64 = quantity!(100, J, i64);
    println!("Energy (i64): value = {}", energy_i64.value);
    
    // Test with other compound units using different storage types
    let force_f64 = quantity!(50.0, N);
    println!("Force (f64): {:?}", force_f64);
    
    let force_i32 = quantity!(50, N, i32);
    println!("Force (i32): {:?}", force_i32);
    
    let power_f64 = quantity!(25.0, W);
    println!("Power (f64): {:?}", power_f64);
    
    let power_i64 = quantity!(25, W, i64);
    println!("Power (i64): value = {}", power_i64.value);
    
    // Test LocalMass trait with different storage types
    // Note: grams() converts to the local mass scale (Kilogram), so 1000 grams = 1 kilogram
    let mass_f64 = 1000.0.grams();
    assert_eq!(mass_f64.value, 1.0); // 1000 grams = 1 kilogram
    
    let mass_i32 = 1000i32.grams();
    assert_eq!(mass_i32.value, 1); // 1000 grams = 1 kilogram
    
    let mass_i64 = 1000i64.grams();
    assert_eq!(mass_i64.value, 1); // 1000 grams = 1 kilogram
    
    println!("Local quantity macro with generic storage types test passed!");
}


