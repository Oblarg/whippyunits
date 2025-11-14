//! Affine Units: Units with Zero-Point Offsets
//!
//! This example demonstrates affine units, which are units that have
//! a zero-point offset from the storage unit. These units handle
//! temperature scales and other measurements where the zero point
//! differs from the storage unit.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// For method syntax (.celsius(), .fahrenheit(), etc.)
use whippyunits::default_declarators::*;

// For macro syntax (quantity! macro)
use whippyunits::quantity;

// For accessing values with unit safety
use whippyunits::value;

// For literal syntax (0.0degC, 32.0degF, etc.) - requires culit attribute
#[culit::culit(whippyunits::default_declarators::literals)]
fn main() {
    // ========================================================================
    // DECLARATION: Declaring Quantities in Affine Units
    // ========================================================================
    //
    // Affine units can be declared using all three syntax options:
    // 1. Method syntax: 0.0.celsius()
    // 2. Macro syntax: quantity!(0.0, degC)
    // 3. Literal syntax: 0.0degC (requires culit attribute)
    //
    // All three produce the same result: a quantity that is automatically
    // converted to its storage unit at declaration time by adding the affine offset.
    // Note: The type system will show the storage unit (e.g., Kelvin, Rankine),
    // not the original affine unit (e.g., Celsius, Fahrenheit).

    // Method syntax - most familiar, requires importing default_declarators::*
    // 0°C = 0 + 273.15 = 273.15 K (stored as Kelvin, affine offset: +273.15)

    // Macro syntax - flexible, works everywhere, supports expressions
    // Same as method syntax: 0°C stored as 273.15 K

    // Literal syntax - most concise, requires culit attribute on function/module
    // Same as method and macro syntax: 0°C stored as 273.15 K

    println!("Declaration Examples:");
    println!("  Method syntax:   {}", 0.0_f64.celsius());
    println!("  Macro syntax:    {}", quantity!(0.0_f64, degC));
    println!("  Literal syntax:  {}", 0.0degC);
    println!("  {}", 32.0_f64.fahrenheit());

    // ========================================================================
    // ACCESS: Retrieving Values in Affine Units
    // ========================================================================
    //
    // To access the underlying numeric value, use the value! macro with
    // an explicit unit specification. This is unit-safe: the compiler
    // verifies dimensional compatibility and automatically rescales.

    println!("\nAccess Examples:");
    println!("  {}", value!(0.0_f64.celsius(), K));
    println!("  {}", value!(0.0_f64.celsius(), degC));
    println!("  {}", value!(32.0_f64.fahrenheit(), degR));
    println!("  {}", value!(32.0_f64.fahrenheit(), degF));
}
