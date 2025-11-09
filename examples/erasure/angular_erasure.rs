//! Angular Erasure Demo
//!
//! This example demonstrates the key properties of angular erasure in whippyunits.
//! Angular quantities are always erased in radian scale, regardless of their original
//! unit. This ensures semantic correctness and compatibility with standard library
//! functions that expect radian-scale values.
//!
//! **Key Properties:**
//! 1. Automatic rescaling to radian scale before erasure
//! 2. Direct access (`.unsafe_value`) vs safe erasure (`.into()`) behavior
//! 3. Compound units can erase radian component while retaining other dimensions
//! 4. Works with any numeric type

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::quantity;
use whippyunits::unit;
use whippyunits::value;

#[culit::culit(whippyunits::default_declarators::literals)]
fn main() {
    println!("Angular Erasure Demo");
    println!("====================\n");

    // Radian Scale Conversion
    // Angular quantities are always erased in radian scale, regardless of their
    // original unit. The erasure mechanism automatically rescales non-radian
    // angular units to radian scale before conversion.
    println!("Radian Scale Conversion:");
    let angle_rad = 1.0rad;
    let angle_deg = 90.0deg;
    
    let scalar_rad: f64 = angle_rad.into();
    let scalar_deg: f64 = angle_deg.into();
    
    println!("   {} → {}", angle_rad, scalar_rad);
    println!("   {} → {}", angle_deg, scalar_deg);
    println!();
    
    assert_eq!(scalar_rad, 1.0);
    assert!((scalar_deg - std::f64::consts::PI / 2.0).abs() < 1e-10);

    // Safe vs Unsafe Access
    // Direct access via `.unsafe_value` may give unexpected results because it
    // doesn't account for unit scale. Safe erasure via `.into()` automatically
    // rescales to radian scale, ensuring semantic correctness.
    println!("Safe vs Unsafe Access:");
    let angle = 90.0deg;
    println!("   Angle: {}", angle);
    println!("   .unsafe_value = {}", angle.unsafe_value);
    println!("   sin({}) = {:.2}", angle.unsafe_value, f64::sin(angle.unsafe_value));
    
    let angle_radians: f64 = angle.into();
    println!("   .into() = {}", angle_radians);
    println!("   sin({}) = {}", angle_radians, f64::sin(angle_radians));
    println!();
    
    assert!((f64::sin(angle_radians) - 1.0).abs() < 1e-10);

    // Compound Unit Erasure
    // Compound units can erase their radian component while retaining other
    // dimensional components. Only pure radian powers are erased.
    println!("Compound Unit Erasure:");
    
    // Example: Angular velocity (rad/s) → frequency (1/s)
    let angular_velocity = 5.0rad / 3.0s;
    println!("   Angular velocity: {}", angular_velocity);
    let frequency: unit!(1 / s) = angular_velocity.into();
    println!("   After erasure: {}", frequency);
    assert_eq!(value!(frequency, 1 / s), 5.0 / 3.0);
    
    // Example: Curvature in centripetal acceleration calculation
    // The radian component is erased, leaving m/s²
    let curvature = quantity!(1.0, rad / m);
    let velocity = quantity!(1.0, m / s);
    let centripetal_acceleration: unit!(m / s^2) = 
        (curvature * velocity * velocity).into();
    println!("   Curvature: {}", curvature);
    println!("   Velocity: {}", velocity);
    println!("   Centripetal acceleration: {}", centripetal_acceleration);
    assert_eq!(value!(centripetal_acceleration, m / s^2), 1.0);
    println!();

    // Numeric Type Polymorphism
    // Erasure can convert to any numeric type (f64, f32, i32, etc.).
    println!("Numeric Type Polymorphism:");
    let angle = 1.0rad;
    let scalar_f64: f64 = angle.into();
    let scalar_f32: f32 = angle.into();
    let scalar_i32: i32 = angle.into();
    
    println!("   {} → f64: {}", angle, scalar_f64);
    println!("   {} → f32: {}", angle, scalar_f32);
    println!("   {} → i32: {}", angle, scalar_i32);
    println!();
    
    assert_eq!(scalar_f64, 1.0);
    assert_eq!(scalar_f32, 1.0);
    assert_eq!(scalar_i32, 1);

    // Standard Library Integration
    // Because erasure always produces radian-scale values, angular quantities
    // can be safely used with standard library trig functions.
    println!("Standard Library Integration:");
    let angle = 90.0deg;
    let angle_radians: f64 = angle.into();
    let sin_value = f64::sin(angle_radians);
    let cos_value = f64::cos(angle_radians);
    
    println!("   Angle: {}", angle);
    println!("   Erased to radians: {}", angle_radians);
    println!("   sin({}) = {}", angle_radians, sin_value);
    println!("   cos({}) = {}", angle_radians, cos_value);
    println!();
    
    assert!((sin_value - 1.0).abs() < 1e-10);
    assert!(cos_value.abs() < 1e-10);

    println!("✅ All angular erasure properties demonstrated successfully!");
}
