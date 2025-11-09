//! Core Concepts: Understanding Quantities, Dimensions, and Scales
//!
//! This example explains the fundamental concepts you need to understand
//! before using whippyunits effectively.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::quantity;
use whippyunits::value;

fn main() {
    // A Quantity is a type-safe wrapper that has three components:
    // - A UNIT: The specific unit (e.g., kilometers, millimeters, seconds)
    // - A STORAGE TYPE: The numeric type (f64, f32, i32, etc.)
    // - A BRAND: Optional type-level marker for type safety
    //
    // The UNIT is determined by:
    // - DIMENSION: What kind of physical quantity (length, time, mass)
    // - SCALE: The scale factor (typically maps to a prefix like 'kilo', 'milli')
    //
    // For example:
    // - Dimension: Length + Scale: 10³ → Unit: kilometers (km)
    // - Dimension: Length + Scale: 10⁻³ → Unit: millimeters (mm)
    // - Dimension: Length + Scale: 1 → Unit: meters (m)
    // Note: Scale typically (but not always) maps to a prefix in the unit name

    println!("1. What is a Quantity?");
    let distance = quantity!(5.0, m);
    println!("   Example: {:?}", distance);
    println!("   - Unit: meters (m), Storage type: f64, Brand: (default)\n");


    // The UNIT name (e.g., 'kilometers', 'millimeters') comes from:
    // - DIMENSION: The base physical quantity (Length, Time, Mass, etc.)
    // - SCALE: The scale factor (typically corresponds to a prefix)
    //
    // length1 and length2:
    // - Same dimension (Length) ✅
    // - Different scales → different units (m vs mm) ⚠️
    // - Cannot add directly (need rescale)
    //
    // length1 and time:
    // - Different dimensions (Length vs Time) ❌
    // - Different units (meters vs seconds)
    // - Cannot add, subtract, or compare

    println!("2. How Units Are Determined: Dimension + Scale");
    let length1 = quantity!(1.0, m);    // Unit: meters (Length dimension, scale 1)
    let length2 = quantity!(1000.0, mm); // Unit: millimeters (Length dimension, scale 10⁻³)
    let time = quantity!(1.0, s);        // Unit: seconds (Time dimension, scale 1)

    println!("   length1: {:?} (Length, scale 1)", length1);
    println!("   length2: {:?} (Length, scale 10⁻³)", length2);
    println!("   time: {:?} (Time, scale 1)\n", time);


    // The Rust type system prevents dimensional errors at compile time.
    //
    // This works: same dimension and scale
    // This would fail: different dimensions (m + s)
    // This would also fail: same dimension, different scales (m + mm)

    println!("3. Type Safety");
    let sum = quantity!(5.0, m) + quantity!(3.0, m);
    println!("   ✅ Adding 5.0m + 3.0m = {:?}", sum);
    
    // This would fail to compile:
    // let bad = quantity!(5.0, m) + quantity!(3.0, s);
    // Error: cannot add Quantity<m, f64> and Quantity<s, f64>
    
    // This would also fail to compile:
    // let bad = quantity!(5.0, m) + quantity!(3.0, mm);
    // Error: cannot add Quantity<m, f64> and Quantity<mm, f64>
    // (same dimension, but different scales - need rescale!)
    println!("   ❌ Adding 5.0m + 3.0s would be a compile error (different dimensions)");
    println!("   ❌ Adding 5.0m + 3.0mm would be a compile error (different scales)\n");

    // Even though 1.0m and 1000.0mm represent the same physical distance,
    // they have different UNITS because they have different SCALES.
    // The scale typically (but not always) maps to a prefix in the unit name.
    //
    // The unit name comes from: Dimension (Length) + Scale (10⁻³) = millimeters

    println!("4. Scale Determines the Unit");
    let meters = quantity!(1.0, m);
    let millimeters = quantity!(1000.0, mm);

    println!("   meters: {:?} (scale 1, no prefix)", meters);
    println!("   millimeters: {:?} (scale 10⁻³, milli- prefix)", millimeters);
    println!("   Same physical quantity: {} m = {} mm", value!(meters, m), value!(millimeters, mm));
    println!("   But different TYPES with different UNITS!\n");

    // When you multiply or divide quantities, you create new dimensions.
    // Area is a new dimension: Length × Length = Length²
    // Velocity is a new dimension: Length / Time

    println!("5. Operations Create New Types");
    let width = quantity!(5.0, m);
    let height = quantity!(4.0, m);
    let area = width * height; // Creates Area dimension (Length²)
    
    println!("   width: {:?}", width);
    println!("   height: {:?}", height);
    println!("   area = width * height: {:?} (Length²)\n", area);

    let distance = quantity!(100.0, m);
    let time = quantity!(10.0, s);
    let velocity = distance / time; // Creates Velocity dimension (Length/Time)
    
    println!("   distance: {:?}", distance);
    println!("   time: {:?}", time);
    println!("   velocity = distance / time: {:?} (Length/Time)\n", velocity);
}

