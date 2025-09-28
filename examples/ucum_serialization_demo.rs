//! UCUM Serialization Demo
//!
//! This example demonstrates how to use the UCUM serialization functionality
//! in whippyunits to convert quantities to and from UCUM format strings.

use whippyunits::{
    quantity,
    serialization::{serialize_to_json, to_ucum_unit},
};

fn main() {
    println!("UCUM Serialization Demo");
    println!("======================");

    // Create some quantities with different units using the quantity! macro
    let length = quantity!(5.0, m);
    let mass = quantity!(2.5, kg);
    let force = quantity!(10.0, N);
    let energy = quantity!(100.0, J);
    let pressure = quantity!(1000.0, Pa);

    // Convert to UCUM format
    println!("\nConverting quantities to UCUM format:");
    println!("Length: {} -> {}", length.value, to_ucum_unit(&length));
    println!("Mass: {} -> {}", mass.value, to_ucum_unit(&mass));
    println!("Force: {} -> {}", force.value, to_ucum_unit(&force));
    println!("Energy: {} -> {}", energy.value, to_ucum_unit(&energy));
    println!(
        "Pressure: {} -> {}",
        pressure.value,
        to_ucum_unit(&pressure)
    );

    // Demonstrate JSON serialization
    println!("\nJSON Serialization:");
    if let Ok(json) = serialize_to_json(&length) {
        println!("Length JSON: {}", json);
    }

    if let Ok(json) = serialize_to_json(&force) {
        println!("Force JSON: {}", json);
    }
}
