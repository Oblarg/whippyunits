use whippyunits::generated_api::rescale_f64;
use whippyunits::default_declarators::*;
use whippyunits::generated_quantity_type::Quantity;

// ============================================================================
// Addition and Subtraction Tests
// ============================================================================

#[test]
fn test_addition_same_scale() {
    let m1 = 5.0.amperes();
    let s1 = 30.0.seconds();
    
    // Same scale addition should work
    let result = m1 + 3.0.amperes();
    assert_eq!(result.value, 8.0);

    println!("result: {:?}", result);
    
    let result: unit!(s) = s1 + 10.0.seconds();
    assert_eq!(result.value, 40.0);
}

#[test]
fn test_add_assign() {
    let mut m1 = 5.0.meters();
    
    // Same scale addition should work
    m1 += 3.0.meters();
    assert_eq!(m1.value, 8.0);
}

#[test]
fn test_subtraction_same_scale() {
    let m1 = 5.0.meters();
    let s1 = 30.0.seconds();
    
    // Same scale subtraction should work
    let result: unit!(m) = m1 - 2.0.meters();
    assert_eq!(result.value, 3.0);
    
    let result: unit!(s) = s1 - 5.0.seconds();
    assert_eq!(result.value, 25.0);
}

// ============================================================================
// Multiplication and Division Tests
// ============================================================================

#[test]
fn test_multiplication_same_scale() {
    let m1 = 5.0.meters();
    let s1 = 30.0.seconds();
    
    // Same scale multiplication should work
    let result: unit!(m) = m1 * 2.0;
    assert_eq!(result.value, 10.0);
    
    let result: unit!(s) = s1 * 3.0;
    assert_eq!(result.value, 90.0);
}

#[test]
fn test_division_same_scale() {
    let m1 = 5.0.meters();
    let s1 = 30.0.seconds();
    
    // Same scale division should work
    let result: unit!(m) = m1 / 2.0;
    assert_eq!(result.value, 2.5);
    
    let result: unit!(s) = s1 / 3.0;
    assert_eq!(result.value, 10.0);
}

#[test]
fn test_quantity_multiplication() {
    let m1 = 5.0.amperes();
    let s1 = 30.0.seconds();
    
    // Multiplying quantities should combine dimensions
    let result= m1 * s1;
    println!("result: {:?}", result);
    // Result should be length * time = distance * time
    assert_eq!(result.value, 150.0); // 5m * 30s = 150 m·s
}

#[test]
fn test_scalar_from_radians() {
    let radians = 5.0.radians();
    let square_radians = radians * radians;
    let cube_radians = square_radians * radians;
    let inverse_radians = 1.0 / radians;
    let inverse_square_radians = 1.0 / square_radians;
    let inverse_cube_radians = 1.0 / cube_radians;
    
    let scalar: f64 = radians.into();
    assert_eq!(scalar, 5.0);
    let scalar: f64 = square_radians.into();
    assert_eq!(scalar, 25.0);
    let scalar: f64 = cube_radians.into();
    assert_eq!(scalar, 125.0);
    let scalar: f64 = inverse_radians.into();
    assert_eq!(scalar, 0.2);
    let scalar: f64 = inverse_square_radians.into();
    assert_eq!(scalar, 0.04);
    let scalar: f64 = inverse_cube_radians.into();
    assert_eq!(scalar, 0.008);
}

#[test]
fn test_radian_erasure() {
    let composite_with_radians = 5.0.radians() / 3.0.seconds();
    let composite_with_radians_erased: unit!(1 / s) = composite_with_radians.into();
    println!("composite_with_radians_erased: {:?}", composite_with_radians_erased);
    assert_eq!(composite_with_radians_erased.value, 5.0 / 3.0);
}

#[test]
fn test_quantity_division() {
    let m1 = 5.0.meters();
    let s1 = 30.0.seconds();
    
    // Dividing quantities should combine dimensions
    let result: unit!(m / s) = m1 / s1;
    // Result should be length / time = velocity
    assert_eq!(result.value, 5.0 / 30.0); // 5m / 30s = 0.166... m/s
}

#[test]
fn test_scalar_quantity_multiplication() {
    let m1 = 5.0.meters();
    
    // Scalar * Quantity should work
    let result: unit!(m) = 3.0 * m1;
    assert_eq!(result.value, 15.0);
    
    // Quantity * Scalar should work
    let result: unit!(m) = m1 * 4.0; 
    assert_eq!(result.value, 20.0);
}

#[test]
fn test_scalar_quantity_division() {
    let m1 = 5.0.meters();
    
    // Quantity / Scalar should work
    let result: unit!(m) = m1 / 2.0;
    assert_eq!(result.value, 2.5);
    
    // Scalar / Quantity should work (inverts dimensions)
    let result: unit!(1 / m) = 10.0 / m1;
    assert_eq!(result.value, 2.0); // 10 / 5m = 2 m^-1
}

#[test]
fn test_quantity_scalar_multiplication() {
    let m1 = 5.0.meters();
    
    // Quantity * Scalar should work
    let result: unit!(m) = m1 * 4.0;
    assert_eq!(result.value, 20.0);
}

#[test]
fn test_quantity_scalar_division() {
    let m1 = 5.0.meters();
    
    // Quantity / Scalar should work
    let result: unit!(m) = m1 / 2.0;
    assert_eq!(result.value, 2.5);
}

#[test]
fn test_quantity_scalar_multiplication_assign() {
    let mut m1 = 5.0.meters();
    
    // Quantity * Scalar should work
    m1 *= 4.0;
    assert_eq!(m1.value, 20.0);
}


// ============================================================================
// Rescale Tests
// ============================================================================

#[test]
fn test_rescale_length() {
    let m1: unit!(m) = 5.0.meters();
    
    // Rescale from meters to kilometers
    let result: Kilometer = rescale_f64(m1);
    assert_eq!(result.value, 0.005); // 5m = 0.005km
    
    // Rescale from meters to millimeters
    let result: Millimeter = rescale_f64(m1);
    assert_eq!(result.value, 5000.0); // 5m = 5000mm
}

#[test]
fn test_rescale_mass() {    
    // Rescale from grams to kilograms
    let result: Kilogram = rescale_f64(100.0.grams());
    assert_eq!(result.value, 0.1); // 100g = 0.1kg
    
    // Rescale from grams to milligrams
    let result: Milligram = rescale_f64(100.0.grams());
    assert_eq!(result.value, 100000.0); // 100g = 100000mg

    println!("{:?}", 1.kilograms() * 1.meters() * 1.meters() / 1.seconds() / 1.seconds());
}

#[test]
fn test_rescale_time() {
    let s1 = 30.0.seconds();
    
    // Rescale from seconds to minutes
    let result: Minute = rescale_f64(s1);
    assert_eq!(result.value, 0.5); // 30s = 0.5min
    
    // Rescale from seconds to milliseconds
    let result: Millisecond = rescale_f64(s1);
    assert_eq!(result.value, 30000.0); // 30s = 30000ms
}

// ============================================================================
// Edge Cases and Error Handling Tests
// ============================================================================

#[test]
fn test_negative_quantities() {
    let neg_m = (-3.0).meters();
    let pos_m = 5.0.meters();
    
    // Addition with negative
    let result = neg_m + pos_m;
    assert_eq!(result.value, 2.0);
    
    // Subtraction with negative
    let result = pos_m - neg_m;
    assert_eq!(result.value, 8.0);
    
    // Multiplication with negative
    let result = neg_m * 2.0;
    assert_eq!(result.value, -6.0);
}

#[test]
fn test_large_numbers() {
    let large_m = 1000000.0.meters();
    let small_m = 0.000001.meters();
    
    // Addition with large numbers
    let result = large_m + small_m;
    assert_eq!(result.value, 1000000.000001);
    
    // Multiplication with large numbers
    let result = large_m * 2.0;
    assert_eq!(result.value, 2000000.0);
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_chain_operations() {
    let m1 = 5.0.meters();
    
    // Chain multiple operations
    let result = m1 + m1 - 2.0.meters() * 3.0 / 2.0;
    // 5m + 5m - (2m * 3) / 2 = 10m - 3m = 7m
    assert_eq!(result.value, 7.0);
}

