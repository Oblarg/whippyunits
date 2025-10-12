use whippyunits::api::*;
use whippyunits::define_base_units;
use whippyunits::{rescale_f64, rescale_i32, rescale_i64};

// Set up scoped preferences with different units
define_base_units!(Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian);

// Define custom literals separately
whippyunits::define_literals!();

#[culit::culit]
#[test]
fn test_local_quantity_macro() {
    // Test with f64 (default)
    let energy_f64 = quantity!(100.0, J);
    println!("Energy (f64): {:?}", energy_f64);

    let local_watt = quantity!(100.0, J / s);

    // Test with i32
    let energy_i32 = quantity!(100, J, i32);
    println!("Energy (i32): {:?}", energy_i32);

    // Test with i64
    let energy_i64 = quantity!(100, J, i64);
    println!("Energy (i64): value = {}", energy_i64.unsafe_value);

    // Test with other compound units using different storage types
    let force_f64 = quantity!(50.0, N);
    println!("Force (f64): {:?}", force_f64);

    let force_i32 = quantity!(50, N, i32);
    println!("Force (i32): {:?}", force_i32);

    let power_f64 = quantity!(25.0, W);
    println!("Power (f64): {:?}", power_f64);

    let power_i64 = quantity!(25, W, i64);
    println!("Power (i64): value = {}", power_i64.unsafe_value);

    // Test LocalMass trait with different storage types
    // Note: grams() converts to the local mass scale (Kilogram), so 1000 grams = 1 kilogram
    let mass_f64 = 1000.0.grams();
    assert_eq!(mass_f64.unsafe_value, 1.0); // 1000 grams = 1 kilogram

    let mass_i32 = 1000i32.grams();
    assert_eq!(mass_i32.unsafe_value, 1); // 1000 grams = 1 kilogram

    let mass_i64 = 1000i64.grams();
    assert_eq!(mass_i64.unsafe_value, 1); // 1000 grams = 1 kilogram

    println!("Local quantity macro with generic storage types test passed!");
}

#[culit::culit]
#[test]
fn test_compound_unit_literals_with_local_scale() {
    // Test compound unit literals with local scale preferences
    let energy_f64 = 1.0J;  // 1 joule
    let energy_i32 = 1J;    // 1 joule (integer)
    
    // Test other compound units
    let power_f64 = 2.0W;   // 2 watts
    let force_f64 = 10.0N;  // 10 newtons
    let pressure_f64 = 100.0Pa; // 100 pascals
    
    // Test prefixed compound units
    let kilojoule_f64 = 1.5kJ;  // 1.5 kilojoules
    let milliwatt_f64 = 100.0mW; // 100 milliwatts
    
    println!("Energy (f64): {}", energy_f64);
    println!("Energy (i32): {}", energy_i32);
    println!("Power (f64): {}", power_f64);
    println!("Force (f64): {}", force_f64);
    println!("Pressure (f64): {}", pressure_f64);
    println!("Kilojoule (f64): {}", kilojoule_f64);
    println!("Milliwatt (f64): {}", milliwatt_f64);
    
    println!("Compound unit literals with local scale preferences test passed!");
}

#[test]
fn test_compound_unit_literal_detection() {
    let test = quantity!(1.0, kW * h);
    println!("Test: {:?}", test);
}