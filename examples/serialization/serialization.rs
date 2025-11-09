//! Serialization Demo
//!
//! This example demonstrates how to serialize whippyunits quantities to various formats:
//! - JSON using serde's Serialize trait
//! - JSON using the serialize_to_json function
//! - String format using the fmt formatter

use whippyunits::{quantity, serialization::serialize_to_json};
use serde_json;

fn main() {
    println!("Serialization Demo");
    println!("==================\n");

    let length = quantity!(5.0, m);
    let mass = quantity!(2.5, kg);
    let force = quantity!(10.0, N);
    let velocity = quantity!(20.0, m / s);
    let acceleration = quantity!(9.81, m / s ^ 2);

    println!("1. Serializing to JSON using serde's Serialize trait:");
    let length_json = serde_json::to_string(&length).unwrap();
    let mass_json = serde_json::to_string(&mass).unwrap();
    let force_json = serde_json::to_string(&force).unwrap();

    println!("   Length JSON: {}", length_json);
    println!("   Mass JSON: {}", mass_json);
    println!("   Force JSON: {}", force_json);

    println!("\n2. Serializing to JSON using serialize_to_json function:");
    let velocity_json = serialize_to_json(&velocity).unwrap();
    let acceleration_json = serialize_to_json(&acceleration).unwrap();

    println!("   Velocity JSON: {}", velocity_json);
    println!("   Acceleration JSON: {}", acceleration_json);

    println!("\n3. Serializing to string format using fmt formatter:");
    let length_str = format!("{}", length.fmt("m"));
    let mass_str = format!("{}", mass.fmt("kg"));
    let force_str = format!("{}", force.fmt("N"));
    let velocity_str = format!("{}", velocity.fmt("m/s"));
    let acceleration_str = format!("{}", acceleration.fmt("m/s2"));

    println!("   Length: {}", length_str);
    println!("   Mass: {}", mass_str);
    println!("   Force: {}", force_str);
    println!("   Velocity: {}", velocity_str);
    println!("   Acceleration: {}", acceleration_str);

    println!("\n4. Formatting with unit conversions:");
    let length_km = format!("{}", length.fmt("km"));
    let length_cm = format!("{}", length.fmt("cm"));
    let mass_g = format!("{}", mass.fmt("g"));
    let velocity_kmh = format!("{}", velocity.fmt("km/h"));

    println!("   5 m as km: {}", length_km);
    println!("   5 m as cm: {}", length_cm);
    println!("   2.5 kg as g: {}", mass_g);
    println!("   20 m/s as km/h: {}", velocity_kmh);

    println!("\n5. Formatting with precision:");
    println!("   Default: {}", format!("{}", length.fmt("km")));
    println!("   2 decimals: {}", format!("{:.2}", length.fmt("km")));
    println!("   4 decimals: {}", format!("{:.4}", length.fmt("km")));

    println!("\n6. JSON format details:");
    println!("   The JSON format is: {{\"value\": <number>, \"unit\": \"<UCUM_unit_string>\"}}");
    println!("   Example: {}", length_json);
}

