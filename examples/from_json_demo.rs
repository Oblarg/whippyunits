//! from_json! Macro Demo
//!
//! This example demonstrates how to use the from_json! macro to deserialize
//! JSON representations of whippyunits quantities back to typed quantities.

use whippyunits::{from_json, quantity, serialization::serialize_to_json};

fn main() {
    println!("from_json! Macro Demo");
    println!("=====================");

    // Create some quantities with different units
    let length = quantity!(5.0, m);
    let mass = quantity!(2.5, kg);
    let force = quantity!(10.0, N);

    // Serialize to JSON
    println!("\nSerializing quantities to JSON:");
    let length_json = serialize_to_json(&length).unwrap();
    let mass_json = serialize_to_json(&mass).unwrap();
    let force_json = serialize_to_json(&force).unwrap();

    println!("Length JSON: {}", length_json);
    println!("Mass JSON: {}", mass_json);
    println!("Force JSON: {}", force_json);

    // Deserialize back using from_json! macro
    println!("\nDeserializing JSON back to quantities:");

    // Use the from_json! macro to deserialize
    let length_from_json = from_json!(&length_json, m);
    let mass_from_json = from_json!(&mass_json, kg);
    let force_from_json = from_json!(&force_json, N);

    println!("Length from JSON: {:?}", length_from_json);
    println!("Mass from JSON: {:?}", mass_from_json);
    println!("Force from JSON: {:?}", force_from_json);

    // Test dimension mismatch (should fail)
    println!("\nTesting dimension mismatch (should fail):");
    let result = from_json!(&length_json, kg); // Trying to parse length as mass
    match result {
        Ok(_) => println!("ERROR: Dimension check failed!"),
        Err(e) => println!("✓ Correctly caught dimension mismatch: {}", e),
    }

    // Test unit conversion
    println!("\nTesting unit conversion:");
    let km_json = r#"{"value": 1000.0, "unit": "m"}"#;
    let km_from_json = from_json!(km_json, km);
    println!("1000 meters converted to km: {:?}", km_from_json);
}
