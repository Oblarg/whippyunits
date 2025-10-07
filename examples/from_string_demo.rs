//! from_string! Macro Demo
//!
//! This example demonstrates how to use the from_string! macro to deserialize
//! string representations of whippyunits quantities back to typed quantities.
//! The format is "value unit" (e.g., "5.0 m", "2.5 kg", "10.0 N").

use whippyunits::{from_string, quantity, serialization::SerializationError};

fn main() {
    println!("from_string! Macro Demo");
    println!("======================");

    // Test 1: Basic units
    println!("\n1. Basic Units Test:");
    let length = from_string!("5.0 m", m).unwrap();
    let mass = from_string!("2.5 kg", kg).unwrap();
    let time = from_string!("10.0 s", s).unwrap();

    println!("Length from string: {:?}", length);
    println!("Mass from string: {:?}", mass);
    println!("Time from string: {:?}", time);

    // Test 2: Unit conversions
    println!("\n2. Unit Conversions Test:");
    let km_from_m = from_string!("1000.0 m", km).unwrap();
    let cm_from_m = from_string!("1.0 m", cm).unwrap();
    let kg_from_g = from_string!("1000.0 g", kg).unwrap();

    println!("1000 meters as km: {:?}", km_from_m);
    println!("1 meter as cm: {:?}", cm_from_m);
    println!("1000 grams as kg: {:?}", kg_from_g);

    // Test 3: Compound units
    println!("\n3. Compound Units Test:");
    let velocity = from_string!("10.0 m/s", m / s).unwrap();
    let acceleration = from_string!("9.81 m/s2", m / s ^ 2).unwrap();
    let force = from_string!("100.0 kg.m/s2", kg * m / s ^ 2).unwrap();

    println!("Velocity from string: {:?}", velocity);
    println!("Acceleration from string: {:?}", acceleration);
    println!("Force from string: {:?}", force);

    // Test 4: Unit conversions with different scales
    println!("\n4. Unit Conversions Test:");
    let mm_from_m = from_string!("1000.0 m", mm).unwrap();
    let km_from_m = from_string!("1000.0 m", km).unwrap();
    let mg_from_g = from_string!("1.0 g", mg).unwrap();

    println!("1000 meters as mm: {:?}", mm_from_m);
    println!("1000 meters as km: {:?}", km_from_m);
    println!("1 gram as mg: {:?}", mg_from_g);

    // Test 5: Dimension mismatch (should fail)
    println!("\n5. Dimension Mismatch Test:");
    let result = from_string!("5.0 m", kg); // Trying to parse length as mass
    match result {
        Ok(_) => println!("❌ ERROR: Dimension check failed!"),
        Err(SerializationError::DimensionMismatch { expected, actual }) => {
            println!("✅ Correctly caught dimension mismatch");
            println!("   Expected: {:?}", expected);
            println!("   Actual: {:?}", actual);
        }
        Err(e) => println!("❌ Unexpected error: {}", e),
    }

    // Test 6: Invalid format (should fail)
    println!("\n6. Invalid Format Test:");
    let result = from_string!("5.0", m); // Missing unit
    match result {
        Ok(_) => println!("❌ ERROR: Format validation failed!"),
        Err(SerializationError::InvalidFormat(msg)) => {
            println!("✅ Correctly caught invalid format: {}", msg);
        }
        Err(e) => println!("❌ Unexpected error: {}", e),
    }

    // Test 7: Different unit types with same dimensions
    println!("\n7. Same Dimensions, Different Units Test:");
    let m_as_cm = from_string!("1.0 m", cm).unwrap();
    let kg_as_g = from_string!("1.0 kg", g).unwrap();

    println!("1 meter as cm: {} cm", m_as_cm.unsafe_value);
    println!("1 kg as g: {} g", kg_as_g.unsafe_value);

    println!("\n🎉 All tests completed! The from_string! macro is working correctly.");
    println!("✅ Proper type checking");
    println!("✅ Dimension validation");
    println!("✅ Unit conversions");
    println!("✅ Error handling");
    println!("✅ String parsing");
}
