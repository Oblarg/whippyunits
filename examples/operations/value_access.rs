//! Safe Value Access Demo
//!
//! This example demonstrates safe value access using the `value!` macro and
//! explains the dangers of using `.unsafe_value` directly.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::quantity;
use whippyunits::value;

#[culit::culit(whippyunits::default_declarators::literals)]
fn main() {
    println!("Safe Value Access Demo");
    println!("=====================\n");

    // Safe Value Access with value! macro
    // The value! macro is unit-safe: it rescales to the specified unit and
    // checks dimensions at compile time. This is the recommended way to access
    // numeric values from quantities.
    println!("Safe Value Access with value! macro:");
    let distance = 1.0m;
    
    let value_m: f64 = value!(distance, m);
    let value_mm: f64 = value!(distance, mm);
    let value_km: f64 = value!(distance, km);
    
    println!("   {} = {} m", distance, value_m);
    println!("   {} = {} mm", distance, value_mm);
    println!("   {} = {} km", distance, value_km);
    println!();
    
    assert_eq!(value_m, 1.0);
    assert_eq!(value_mm, 1000.0);
    assert_eq!(value_km, 0.001);

    // Compile-time dimension checking
    // The value! macro ensures dimensional correctness at compile time.
    // Attempting to access a quantity in incompatible units will fail to compile.
    println!("Compile-time Dimension Checking:");
    let distance = 1.0m;
    let time = 1.0s;
    
    println!("   distance = {}", distance);
    println!("   time = {}", time);
    println!("   value!(distance, m) = {}", value!(distance, m));
    println!("   value!(time, s) = {}", value!(time, s));
    println!("   // value!(distance, s) would fail to compile ❌");
    println!("   // value!(time, m) would fail to compile ❌");
    println!();

    // Different numeric types
    // The value! macro supports different numeric types by specifying the type.
    // The quantity must already be of the target numeric type.
    println!("Different Numeric Types:");
    let distance_f64 = quantity!(1.0, m, f64);
    let distance_f32 = quantity!(1.0, m, f32);
    let distance_i32 = quantity!(1, m, i32);
    
    let value_f64: f64 = value!(distance_f64, m);
    let value_f32: f32 = value!(distance_f32, m, f32);
    let value_i32: i32 = value!(distance_i32, m, i32);
    
    println!("   {} → f64: {}", distance_f64, value_f64);
    println!("   {} → f32: {}", distance_f32, value_f32);
    println!("   {} → i32: {}", distance_i32, value_i32);
    println!();
    
    assert_eq!(value_f64, 1.0);
    assert_eq!(value_f32, 1.0);
    assert_eq!(value_i32, 1);

    // Dangers of unsafe_value
    // Direct access to .unsafe_value bypasses the type system's unit safety
    // guarantees. Because whippyunits represents storage scales as part of the
    // type system, the actual numeric value may not match the user's intent.
    println!("Dangers of unsafe_value:");
    let distance = 1.0m;
    
    println!("   distance = {}", distance);
    println!("   distance.unsafe_value = {}", distance.unsafe_value);
    println!("   ⚠️  This gives the storage value (1.0), not necessarily what you expect!");
    println!();
    
    // The problem becomes more apparent with different scales
    let distance_mm = 1000.0mm;
    println!("   distance_mm = {}", distance_mm);
    println!("   distance_mm.unsafe_value = {}", distance_mm.unsafe_value);
    println!("   ⚠️  This gives 1000.0, but if you expected meters, you're wrong!");
    println!();
    
    // Safe access gives the correct value in the unit you specify
    println!("   ✅ value!(distance_mm, m) = {}", value!(distance_mm, m));
    println!("   ✅ value!(distance_mm, mm) = {}", value!(distance_mm, mm));
    println!();

    // Dimensionless quantities: unsafe_value vs erasure
    // For dimensionless quantities, .unsafe_value can give surprising results
    // due to residual scales. Use .into() for safe erasure instead.
    println!("Dimensionless Quantities:");
    let ratio = 1.0m / 1.0mm;
    
    println!("   ratio = {} (1 m / 1 mm)", ratio);
    println!("   ratio.unsafe_value = {}", ratio.unsafe_value);
    println!("   ⚠️  This gives 1.0, but the actual ratio is 1000.0!");
    
    let ratio_scalar: f64 = ratio.into();
    println!("   ✅ ratio.into() = {} (correct!)", ratio_scalar);
    println!();
    
    assert_eq!(ratio.unsafe_value, 1.0);
    assert_eq!(ratio_scalar, 1000.0);

    // Angular quantities: unsafe_value vs erasure
    // For angular quantities, .unsafe_value gives the storage value, not the
    // radian-scale value that trigonometric functions expect.
    println!("Angular Quantities:");
    let angle = 90.0deg;
    
    println!("   angle = {}", angle);
    println!("   angle.unsafe_value = {}", angle.unsafe_value);
    println!("   ⚠️  sin({}) = {:.2} (wrong! expects radians)", 
             angle.unsafe_value, f64::sin(angle.unsafe_value));
    
    let angle_radians: f64 = angle.into();
    println!("   ✅ angle.into() = {} (radians)", angle_radians);
    println!("   ✅ sin({}) = {} (correct!)", 
             angle_radians, f64::sin(angle_radians));
    println!();
    
    assert!((f64::sin(angle_radians) - 1.0).abs() < 1e-10);
}

