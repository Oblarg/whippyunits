//! Comparison Operations Demo
//!
//! This example demonstrates comparison operators (<, <=, >, >=, ==, !=) with whippyunits quantities.
//!
//! **Important**: Comparison operators are **scale-strict** - both operands must have the same scale.
//! To compare quantities with different scales, use `rescale()` to convert one to match the other.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::api::rescale;

#[culit::culit(whippyunits::default_declarators::literals)]
fn main() {
    println!("Comparison Operations Demo");
    println!("==========================\n");

    // Basic comparisons with same scale
    println!("1. Same Scale Comparisons:");
    let distance1 = 5.0m;
    let distance2 = 10.0m;

    println!(
        "   {} < {}: {}",
        distance1,
        distance2,
        distance1 < distance2
    );
    println!(
        "   {} > {}: {}",
        distance2,
        distance1,
        distance2 > distance1
    );
    println!(
        "   {} == {}: {}",
        distance1,
        distance1,
        distance1 == distance1
    );

    // Scale-strict: must rescale to compare different scales
    println!("\n2. Cross-Scale Comparisons:");
    let meters = 1.0m;
    let millimeters = 500.0mm;

    // ✅ Correct: rescale one side to match the other
    println!(
        "   rescale({}) > {}: {}",
        meters,
        millimeters,
        rescale(meters) > millimeters
    );
    println!(
        "   {} > rescale({}): {}",
        meters,
        millimeters,
        meters > rescale(millimeters)
    );

    // ❌ This would fail to compile (scale mismatch):
    // let _result = meters > millimeters;  // Compile error!

    // Equality with rescale
    println!("\n3. Equality Across Scales:");
    println!(
        "   rescale({}) == {}: {}",
        meters,
        1000.0mm,
        rescale(meters) == 1000.0mm
    );
    println!(
        "   {} == rescale({}): {}",
        meters,
        1000.0mm,
        meters == rescale(1000.0mm)
    );

    // Comparisons in conditional logic
    println!("\n4. Using Comparisons in Logic:");
    let max_distance = 100.0m;
    let current_distance = 150.0m;

    if current_distance > max_distance {
        println!(
            "   Warning: {} exceeds limit of {}",
            current_distance, max_distance
        );
    }

    // Sorting quantities
    println!("\n5. Sorting Quantities:");
    let mut distances: Vec<_> = vec![20.0m, 5.0m, 15.0m, 10.0m];

    println!("   Before sorting:");
    for d in &distances {
        print!("   {}  ", d);
    }
    println!();

    distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("   After sorting:");
    for d in &distances {
        print!("   {}  ", d);
    }
    println!();

    // Zero and negative values
    println!("\n6. Zero and Negative Comparisons:");
    let zero = 0.0m;
    let positive = 5.0m;
    let negative = -3.0m;

    println!("   {} < {}: {}", negative, zero, negative < zero);
    println!("   {} < {}: {}", zero, positive, zero < positive);
    println!("   {} == {}: {}", zero, zero, zero == zero);
}
