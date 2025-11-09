//! Common Errors and How to Fix Them
//!
//! This example shows the most common compile errors you'll encounter
//! and how to resolve them.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::api::rescale;
use whippyunits::define_unit_declarators;
use whippyunits::quantity;
use whippyunits::unit;
use whippyunits::value;

// Create a branded declarator module for brand mismatch examples
define_unit_declarators!(custom_brand, CustomBrand);

fn main() {
    println!("Common Errors and Solutions");
    println!("==========================\n");

    // ============================================================
    // ERROR 1: Dimension Mismatch
    // ============================================================
    // Problem: Trying to operate on quantities with different dimensions
    // Error message: expected struct `Quantity<m, f64>`, found struct `Quantity<s, f64>`
    // Solution: This is a logic error - check your calculation
    // You cannot add, subtract, or compare quantities with different dimensions

    println!("ERROR 1: Dimension Mismatch");
    let _distance = quantity!(5.0, m);
    let _time = quantity!(10.0, s);
    
    println!("   let distance = quantity!(5.0, m);");
    println!("   let time = quantity!(10.0, s);");
    println!("   let result = distance + time;  // ❌ Compile error!\n");

    // ============================================================
    // ERROR 2: Scale Mismatch
    // ============================================================
    // Problem: Trying to add quantities with different scales (same dimension)
    // Error message: expected struct `Quantity<m, f64>`, found struct `Quantity<mm, f64>`
    // Solution: Use rescale() to convert to the same scale

    println!("ERROR 2: Scale Mismatch");
    let meters = quantity!(1.0, m);
    let millimeters = quantity!(1000.0, mm);
    
    println!("   let meters = quantity!(1.0, m);");
    println!("   let millimeters = quantity!(1000.0, mm);");
    println!("   let sum = meters + millimeters;  // ❌ Compile error!\n");
    
    // ✅ Correct way:
    let sum1: unit!(m) = meters + rescale(millimeters);
    let sum2: unit!(mm) = rescale(meters) + millimeters;
    
    println!("   ✅ Correct:");
    println!("   let sum1: unit!(m) = meters + rescale(millimeters);");
    println!("   // Result: {} m", value!(sum1, m));
    println!("   let sum2: unit!(mm) = rescale(meters) + millimeters;");
    println!("   // Result: {} mm\n", value!(sum2, mm));

    // ============================================================
    // ERROR 3: Storage Type Mismatch
    // ============================================================
    // Problem: Trying to operate on quantities with different storage types
    // Error message: cannot add `Quantity<m, f64>` and `Quantity<m, i32>`
    // Solution: Convert to the same storage type

    println!("ERROR 3: Storage Type Mismatch");
    let distance_f64 = quantity!(5.0, m);  // f64
    let distance_i32 = quantity!(5, m, i32); // i32
    
    println!("   let distance_f64 = quantity!(5.0, m);     // f64");
    println!("   let distance_i32 = quantity!(5, m, i32);  // i32");
    println!("   let sum = distance_f64 + distance_i32;    // ❌ Compile error!\n");
    
    // ✅ Correct way: Create new quantity with same storage type
    let distance_i32_as_f64 = quantity!(distance_i32.unsafe_value as f64, m);
    let sum_f64: unit!(m) = distance_f64 + distance_i32_as_f64;
    println!("   ✅ Correct:");
    println!("   // Convert to same storage type, then add");
    println!("   // Result: {} m\n", value!(sum_f64, m));

    // ============================================================
    // ERROR 4: Brand Mismatch
    // ============================================================
    // Problem: Trying to operate on quantities with different brands
    // Error message: cannot add `Quantity<m, f64, ()>` and `Quantity<m, f64, CustomBrand>`
    // Solution: Quantities must have the same brand to operate together
    // Brands are used to prevent mixing quantities from different contexts
    // (e.g., different coordinate systems, different physical meanings)

    println!("ERROR 4: Brand Mismatch");
    let _default_distance = quantity!(5.0, m);  // Default brand: ()
    let _branded_distance = custom_brand::quantity!(5.0, m);  // Custom brand: CustomBrand
    
    println!("   let default_distance = quantity!(5.0, m);  // Brand: ()");
    println!("   let branded_distance = custom_brand::quantity!(5.0, m);  // Brand: CustomBrand");
    println!("   let sum = default_distance + branded_distance;  // ❌ Compile error!\n");
    
    // ✅ Correct way: Use quantities from the same brand
    let branded1 = custom_brand::quantity!(5.0, m);
    let branded2 = custom_brand::quantity!(3.0, m);
    let _sum_branded = branded1 + branded2;
    println!("   ✅ Correct:");
    println!("   let branded1 = custom_brand::quantity!(5.0, m);");
    println!("   let branded2 = custom_brand::quantity!(3.0, m);");
    println!("   let sum = branded1 + branded2;  // ✅ Works!\n");

    // ============================================================
    // Tips for Debugging
    // ============================================================
    // 1. Read the error message carefully: look for 'expected' vs 'found'
    // 2. Check which component differs: dimension, scale, storage type, or brand
    // 3. Dimension mismatch → logic error, check your calculation
    // 4. Scale mismatch → use rescale() to convert to same scale
    // 5. Storage type mismatch → convert to same numeric type
    // 6. Brand mismatch → use quantities from the same brand
}
