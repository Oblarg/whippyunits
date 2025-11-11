//! Scalar Erasure Demo
//!
//! This example demonstrates how to safely erase dimensionless quantities (scalars)
//! to numeric types using `.into()`. Dimensionless quantities are created by dividing
//! quantities of the same dimension, and they can be safely converted to numeric types
//! because they have no physical dimension.
//!
//! **Key Points:**
//! - Dimensionless quantities automatically rescale to unity before erasure
//! - This ensures the numeric value matches the semantic meaning
//! - Direct access via `.unsafe_value` may give unexpected results due to residual scales
//! - Erasure works with any numeric type (f64, f32, i32, etc.)

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

#[culit::culit(whippyunits::default_declarators::literals)]
fn main() {
    println!("Scalar Erasure Demo");
    println!("==================\n");

    // 1. Basic dimensionless ratios
    println!("1. Basic Dimensionless Ratios:");
    let ratio1 = 1.0m / 1.0m;
    let scalar1: f64 = ratio1.into();
    println!("   {} → {}", ratio1, scalar1);
    assert_eq!(scalar1, 1.0);

    let ratio2 = 5.0m / 2.0m;
    let scalar2: f64 = ratio2.into();
    println!("   {} → {}", ratio2, scalar2);
    assert_eq!(scalar2, 2.5);

    // 2. Ratios with different scales (residual scale)
    println!("\n2. Ratios with Different Scales (Residual Scale):");
    println!("   When dividing quantities with different scales, a 'residual scale'");
    println!("   is stored in the type, but erasure rescales to unity:");

    let ratio3 = 1.0m / 1.0mm;
    println!("   {} (unsafe_value = {})", ratio3, ratio3.unsafe_value);
    println!("   ⚠️  Direct access gives: {}", ratio3.unsafe_value);

    let scalar3: f64 = ratio3.into();
    println!("   ✅ Erasure rescales to unity: {}", scalar3);
    assert_eq!(scalar3, 1000.0);

    let ratio4 = 100.0mm / 1.0m;
    let scalar4: f64 = ratio4.into();
    println!("   {} → {}", ratio4, scalar4);
    assert_eq!(scalar4, 0.1);

    // 3. Compound dimensionless quantities
    println!("\n3. Compound Dimensionless Quantities:");
    let area1 = 4.0m * 5.0m;
    let area2 = 2.0m * 10.0m;
    let ratio5 = area1 / area2;
    let scalar5: f64 = ratio5.into();
    println!("   {} / {} = {} → {}", area1, area2, ratio5, scalar5);
    assert_eq!(scalar5, 1.0);

    // 4. Using with standard library functions
    println!("\n4. Using with Standard Library Functions:");
    let ratio6 = 2.0m / 1.0m;
    let ratio6_scalar: f64 = ratio6.into();
    let log_value = f64::ln(ratio6_scalar);
    println!("   ln({}) = ln({}) = {}", ratio6, ratio6_scalar, log_value);
    assert!((log_value - 0.693147).abs() < 1e-5);

    let ratio7 = 3.0m / 2.0m;
    let ratio7_scalar: f64 = ratio7.into();
    let pow_value = f64::powf(ratio7_scalar, 2.5);
    println!(
        "   ({})^2.5 = {}^2.5 = {}",
        ratio7, ratio7_scalar, pow_value
    );
    assert!((pow_value - 2.75568).abs() < 1e-4);

    // 5. Different numeric types
    println!("\n5. Erasure to Different Numeric Types:");
    let ratio8 = 5.0m / 2.0m;
    let scalar_f64: f64 = ratio8.into();
    let scalar_f32: f32 = ratio8.into();
    let scalar_i32: i32 = ratio8.into();
    println!("   {} → f64: {}", ratio8, scalar_f64);
    println!("   {} → f32: {}", ratio8, scalar_f32);
    println!("   {} → i32: {}", ratio8, scalar_i32);
    assert_eq!(scalar_f64, 2.5);
    assert_eq!(scalar_f32, 2.5);
    assert_eq!(scalar_i32, 2); // rounds down

    // 6. Small ratios that require rescaling
    println!("\n6. Small Ratios Requiring Rescaling:");
    let ratio9 = 1.0mm / 1.0m;
    let scalar9: f64 = ratio9.into();
    println!("   {} → {}", ratio9, scalar9);
    assert_eq!(scalar9, 0.001);

    let ratio10 = 1.0m / 1.0km;
    let scalar10: f64 = ratio10.into();
    println!("   {} → {}", ratio10, scalar10);
    assert_eq!(scalar10, 0.001);

    // 7. Practical example: efficiency calculation
    println!("\n7. Practical Example: Efficiency Calculation:");
    let output_power = 750.0W;
    let input_power = 1000.0W;
    let efficiency = output_power / input_power;
    let efficiency_scalar: f64 = efficiency.into();
    println!(
        "   Efficiency = {} / {} = {}",
        output_power, input_power, efficiency
    );
    println!("   Efficiency as scalar: {:.1}%", efficiency_scalar * 100.0);
    assert!((efficiency_scalar - 0.75).abs() < 1e-10);

    println!("\n✅ All scalar erasure examples completed successfully!");
}
