//! Setup and Installation Guide
//!
//! This example shows how to set up whippyunits in your project.
//! Run this example to verify your setup is working correctly.

// Step 1: Add whippyunits to your Cargo.toml
// 
// [dependencies]
// whippyunits = "0.1.0"
//
// Note: whippyunits requires nightly Rust due to generic_const_exprs feature.
// Create a rust-toolchain.toml file in your project root:
//
// [toolchain]
// channel = "nightly"

// Step 2: Enable the required feature in your main.rs or lib.rs
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// Step 3: Import whippyunits
use whippyunits::default_declarators::*;
use whippyunits::quantity;

fn main() {
    // If you can compile and run this, your setup is correct!
    
    println!("✅ WhippyUnits Setup Verification");
    println!("================================\n");
    
    // Create a simple quantity
    let distance = quantity!(5.0, m);
    println!("Created a quantity: {:?}", distance);
    
    // Try method syntax
    let time = 10.0.seconds();
    println!("Created with method syntax: {:?}", time);
    
    println!("\n🎉 Setup complete!");
    println!("\nNext: cargo run --example hello_world");
}

