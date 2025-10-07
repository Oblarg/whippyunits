//! Test the expanded value! macro with different backing types

use whippyunits::*;

#[test]
fn test_value_macro_with_different_types() {
    // Test with f64 (default)
    let distance_f64 = quantity!(1.0, m);
    let val_f64_m: f64 = value!(distance_f64, m);
    let val_f64_mm: f64 = value!(distance_f64, mm);
    
    assert_eq!(val_f64_m, 1.0);
    assert_eq!(val_f64_mm, 1000.0);
    
    // Test with f32
    let distance_f32 = quantity!(1.0f32, m, f32);
    let val_f32_m: f32 = value!(distance_f32, m, f32);
    let val_f32_mm: f32 = value!(distance_f32, mm, f32);
    
    assert_eq!(val_f32_m, 1.0f32);
    assert_eq!(val_f32_mm, 1000.0f32);
    
    // Test with i32
    let distance_i32 = quantity!(1, m, i32);
    let val_i32_m: i32 = value!(distance_i32, m, i32);
    let val_i32_mm: i32 = value!(distance_i32, mm, i32);
    
    assert_eq!(val_i32_m, 1);
    assert_eq!(val_i32_mm, 1000);
    
    // Test with i64
    let distance_i64 = quantity!(1i64, m, i64);
    let val_i64_m: i64 = value!(distance_i64, m, i64);
    let val_i64_mm: i64 = value!(distance_i64, mm, i64);
    
    assert_eq!(val_i64_m, 1i64);
    assert_eq!(val_i64_mm, 1000i64);
    
    // Test with u32
    let distance_u32 = quantity!(1u32, m, u32);
    let val_u32_m: u32 = value!(distance_u32, m, u32);
    let val_u32_mm: u32 = value!(distance_u32, mm, u32);
    
    assert_eq!(val_u32_m, 1u32);
    assert_eq!(val_u32_mm, 1000u32);
    
    // Test with u64
    let distance_u64 = quantity!(1u64, m, u64);
    let val_u64_m: u64 = value!(distance_u64, m, u64);
    let val_u64_mm: u64 = value!(distance_u64, mm, u64);
    
    assert_eq!(val_u64_m, 1u64);
    assert_eq!(val_u64_mm, 1000u64);
}

#[test]
fn test_value_macro_with_compound_units() {
    // Test with energy (Joules)
    let energy_f64 = quantity!(1.0, J);
    let val_f64_kj: f64 = value!(energy_f64, kJ);
    assert_eq!(val_f64_kj, 0.001);
    
    let energy_i32 = quantity!(1000, J, i32);
    let val_i32_kj: i32 = value!(energy_i32, kJ, i32);
    assert_eq!(val_i32_kj, 1);

    // Test with power (Watts)
    let power_f64 = quantity!(1000.0, W);
    let val_f64_kw: f64 = value!(power_f64, kW);
    assert_eq!(val_f64_kw, 1.0);

    let power_i32 = quantity!(1000, W, i32);
    let val_i32_kw: i32 = value!(power_i32, kW, i32);
    assert_eq!(val_i32_kw, 1);
}

#[test]
fn test_value_macro_type_preservation() {
    // Ensure that the macro preserves the exact type
    let distance_f64 = quantity!(1.0, m);
    let distance_f32 = quantity!(1.0f32, m, f32);
    let distance_i32 = quantity!(1, m, i32);
    let distance_i64 = quantity!(1i64, m, i64);
    
    // These should compile and return the same type as the input
    let _: f64 = value!(distance_f64, mm);
    let _: f32 = value!(distance_f32, mm, f32);
    let _: i32 = value!(distance_i32, mm, i32);
    let _: i64 = value!(distance_i64, mm, i64);
    
    // These should NOT compile (type mismatch)
    // let _: f32 = value!(distance_f64, mm); // Should fail
    // let _: i32 = value!(distance_f64, mm); // Should fail
}

#[test]
fn test_value_macro_with_all_integer_types() {
    // Test all signed integer types
    // Note: i8 and u8 can't handle 1000, so we test with smaller conversions
    let distance_i8 = quantity!(1i8, m, i8);
    let val_i8: i8 = value!(distance_i8, m, i8); // Same unit, no conversion
    assert_eq!(val_i8, 1i8);

    let distance_i16 = quantity!(1i16, m, i16);
    let val_i16: i16 = value!(distance_i16, mm, i16);
    assert_eq!(val_i16, 1000i16);

    let distance_i128 = quantity!(1i128, m, i128);
    let val_i128: i128 = value!(distance_i128, mm, i128);
    assert_eq!(val_i128, 1000i128);

    // Test all unsigned integer types
    // Note: u8 can't handle 1000, so we test with smaller conversions
    let distance_u8 = quantity!(1u8, m, u8);
    let val_u8: u8 = value!(distance_u8, m, u8); // Same unit, no conversion
    assert_eq!(val_u8, 1u8);

    let distance_u16 = quantity!(1u16, m, u16);
    let val_u16: u16 = value!(distance_u16, mm, u16);
    assert_eq!(val_u16, 1000u16);

    let distance_u128 = quantity!(1u128, m, u128);
    let val_u128: u128 = value!(distance_u128, mm, u128);
    assert_eq!(val_u128, 1000u128);
}

#[test]
fn test_value_macro_with_mass_units() {
    // Test mass conversions
    let mass_f64 = quantity!(1.0, kg);
    let val_f64_g: f64 = value!(mass_f64, g);
    assert_eq!(val_f64_g, 1000.0);
    
    let mass_i32 = quantity!(1, kg, i32);
    let val_i32_g: i32 = value!(mass_i32, g, i32);
    assert_eq!(val_i32_g, 1000);
    
    // Test with milligrams
    let mass_f32 = quantity!(1.0f32, g, f32);
    let val_f32_mg: f32 = value!(mass_f32, mg, f32);
    assert_eq!(val_f32_mg, 1000.0f32);
}

#[test]
fn test_value_macro_with_time_units() {
    // Test time conversions
    let time_f64 = quantity!(1.0, s);
    let val_f64_ms: f64 = value!(time_f64, ms);
    assert_eq!(val_f64_ms, 1000.0);
    
    let time_i32 = quantity!(1, s, i32);
    let val_i32_ms: i32 = value!(time_i32, ms, i32);
    assert_eq!(val_i32_ms, 1000);
    
    // Test with minutes
    let time_f32 = quantity!(60.0f32, s, f32);
    let val_f32_min: f32 = value!(time_f32, min, f32);
    assert_eq!(val_f32_min, 1.0f32);
}
