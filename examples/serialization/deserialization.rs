//! Deserialization Demo
//!
//! This example demonstrates how to deserialize whippyunits quantities from various formats:
//! - String format using from_string! macro
//! - JSON format using from_json! macro
//! - Error handling for dimension mismatches and invalid formats

use whippyunits::{from_json, from_string, serialization::SerializationError};

fn main() {
    println!("Deserialization Demo");
    println!("====================\n");

    println!("1. Deserializing from string format:");
    let length = from_string!("5.0 m", m).unwrap();
    let mass = from_string!("2.5 kg", kg).unwrap();
    let time = from_string!("10.0 s", s).unwrap();

    println!("   Length: {}", length);
    println!("   Mass: {}", mass);
    println!("   Time: {}", time);

    println!("\n2. Unit conversions during deserialization:");
    let km_from_m = from_string!("1000.0 m", km).unwrap();
    let cm_from_m = from_string!("1.0 m", cm).unwrap();
    let kg_from_g = from_string!("1000.0 g", kg).unwrap();

    println!("   1000 m as km: {}", km_from_m);
    println!("   1 m as cm: {}", cm_from_m);
    println!("   1000 g as kg: {}", kg_from_g);

    println!("\n3. Deserializing compound units from strings:");
    let velocity = from_string!("10.0 m/s", m / s).unwrap();
    let acceleration = from_string!("9.81 m/s2", m / s ^ 2).unwrap();
    let force = from_string!("100.0 kg.m/s2", kg * m / s ^ 2).unwrap();

    println!("   Velocity: {}", velocity);
    println!("   Acceleration: {}", acceleration);
    println!("   Force: {}", force);

    println!("\n4. Deserializing from JSON format:");
    let length_json = r#"{"value": 5.0, "unit": "m"}"#;
    let mass_json = r#"{"value": 2.5, "unit": "kg"}"#;
    let force_json = r#"{"value": 10.0, "unit": "N"}"#;

    let length_from_json = from_json!(length_json, m).unwrap();
    let mass_from_json = from_json!(mass_json, kg).unwrap();
    let force_from_json = from_json!(force_json, N).unwrap();

    println!("   Length: {}", length_from_json);
    println!("   Mass: {}", mass_from_json);
    println!("   Force: {}", force_from_json);

    println!("\n5. Unit conversion during JSON deserialization:");
    let km_json = r#"{"value": 1000.0, "unit": "m"}"#;
    let km_from_json = from_json!(km_json, km).unwrap();
    println!("   1000 m converted to km: {}", km_from_json);

    println!("\n6. Error handling - Dimension mismatch:");
    let result = from_string!("5.0 m", kg);
    match result {
        Ok(_) => println!("   ❌ ERROR: Dimension check failed!"),
        Err(e @ SerializationError::DimensionMismatch { .. }) => {
            println!("   ✓ Correctly caught dimension mismatch");
            println!("     Error message: {}", e);
        }
        Err(e) => println!("   ❌ Unexpected error: {}", e),
    }

    let result = from_json!(length_json, kg);
    match result {
        Ok(_) => println!("   ❌ ERROR: Dimension check failed!"),
        Err(e @ SerializationError::DimensionMismatch { .. }) => {
            println!("   ✓ Correctly caught dimension mismatch in JSON");
            println!("     Error message: {}", e);
        }
        Err(e) => println!("   ✓ Correctly caught dimension mismatch: {}", e),
    }

    println!("\n7. Error handling - Invalid format:");
    let result = from_string!("5.0", m);
    match result {
        Ok(_) => println!("   ❌ ERROR: Format validation failed!"),
        Err(SerializationError::InvalidFormat(msg)) => {
            println!("   ✓ Correctly caught invalid format: {}", msg);
        }
        Err(e) => println!("   ❌ Unexpected error: {}", e),
    }

    let invalid_json = r#"{"value": 5.0}"#;
    let result = from_json!(invalid_json, m);
    match result {
        Ok(_) => println!("   ❌ ERROR: Format validation failed!"),
        Err(e) => println!("   ✓ Correctly caught invalid JSON format: {}", e),
    }
}

