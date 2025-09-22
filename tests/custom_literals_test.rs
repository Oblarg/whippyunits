//! Tests for custom literals functionality

extern crate culit;
use culit::culit;

#[culit]
#[test]
fn test_basic_custom_literals() {
    // Test basic float literals
    let distance = 100m_f64;
    let mass = 5.5kg_f64;
    let time = 30s_f64;
    
    // These are currently placeholder implementations that just return the parsed value
    // In a real implementation, they would create proper unit types
    assert_eq!(distance, 100.0);
    assert_eq!(mass, 5.5);
    assert_eq!(time, 30.0);
}

#[culit]
#[test]
fn test_integer_custom_literals() {
    // Test integer literals
    let count1 = 10m_i32;
    let count2 = 100kg_i64;
    let count3 = 5000ms_u32;
    
    assert_eq!(count1, 10);
    assert_eq!(count2, 100);
    assert_eq!(count3, 5000);
}

#[culit]
#[test]
fn test_different_numeric_types() {
    // Test different numeric types
    let float32_val = 3.14m_f32;
    let int32_val = 42kg_i32;
    let int64_val = 1000s_i64;
    let uint32_val = 500A_u32;
    let uint64_val = 2000K_u64;
    
    assert_eq!(float32_val, 3.14);
    assert_eq!(int32_val, 42);
    assert_eq!(int64_val, 1000);
    assert_eq!(uint32_val, 500);
    assert_eq!(uint64_val, 2000);
}

#[culit]
#[test]
fn test_prefixed_units() {
    // Test prefixed units
    let millimeter = 1000mm_f64;
    let kilometer = 5.5km_f64;
    let milligram = 500mg_f64;
    let microsecond = 1000us_f64;
    
    assert_eq!(millimeter, 1000.0);
    assert_eq!(kilometer, 5.5);
    assert_eq!(milligram, 500.0);
    assert_eq!(microsecond, 1000.0);
}

#[culit]
#[test]
fn test_negative_literals() {
    // Test negative literals (the minus sign should be preserved)
    let negative_distance = -100m_f64;
    let negative_mass = -5.5kg_f64;
    
    assert_eq!(negative_distance, -100.0);
    assert_eq!(negative_mass, -5.5);
}

#[culit]
#[test]
fn test_hex_literals() {
    // Test hex literals (base 16)
    let hex_distance = 0xFFm_f64;  // 255 in hex
    let hex_mass = 0x10kg_i32;     // 16 in hex
    
    assert_eq!(hex_distance, 255.0);
    assert_eq!(hex_mass, 16);
}

#[culit]
#[test]
fn test_binary_literals() {
    // Test binary literals (base 2)
    let binary_distance = 0b1010m_f64;  // 10 in binary
    let binary_mass = 0b1111kg_i32;     // 15 in binary
    
    assert_eq!(binary_distance, 10.0);
    assert_eq!(binary_mass, 15);
}

#[culit]
#[test]
fn test_octal_literals() {
    // Test octal literals (base 8)
    let octal_distance = 0o777m_f64;  // 511 in octal
    let octal_mass = 0o100kg_i32;     // 64 in octal
    
    assert_eq!(octal_distance, 511.0);
    assert_eq!(octal_mass, 64);
}

#[culit]
#[test]
fn test_various_units() {
    // Test various unit types
    let ampere = 2.5A_f64;
    let kelvin = 300K_f64;
    let mole = 1mol_f64;
    let candela = 100cd_f64;
    let radian = 1.57rad_f64;
    let degree = 90deg_f64;
    
    assert_eq!(ampere, 2.5);
    assert_eq!(kelvin, 300.0);
    assert_eq!(mole, 1.0);
    assert_eq!(candela, 100.0);
    assert_eq!(radian, 1.57);
    assert_eq!(degree, 90.0);
}

#[test]
fn test_unit_symbol_generation() {
    use whippyunits::custom_literals::{get_unit_symbols_for_literals, get_type_suffixes};
    
    let symbols = get_unit_symbols_for_literals();
    let suffixes = get_type_suffixes();
    
    // Check that we have the expected base units
    assert!(symbols.contains(&"m".to_string()));
    assert!(symbols.contains(&"kg".to_string()));
    assert!(symbols.contains(&"s".to_string()));
    assert!(symbols.contains(&"A".to_string()));
    assert!(symbols.contains(&"K".to_string()));
    assert!(symbols.contains(&"mol".to_string()));
    assert!(symbols.contains(&"cd".to_string()));
    assert!(symbols.contains(&"rad".to_string()));
    
    // Check that we have common prefixed units
    assert!(symbols.contains(&"mm".to_string()));
    assert!(symbols.contains(&"km".to_string()));
    assert!(symbols.contains(&"mg".to_string()));
    assert!(symbols.contains(&"ms".to_string()));
    
    // Check that we have the expected type suffixes
    assert!(suffixes.contains(&"f64"));
    assert!(suffixes.contains(&"f32"));
    assert!(suffixes.contains(&"i32"));
    assert!(suffixes.contains(&"i64"));
    assert!(suffixes.contains(&"u32"));
    assert!(suffixes.contains(&"u64"));
}
