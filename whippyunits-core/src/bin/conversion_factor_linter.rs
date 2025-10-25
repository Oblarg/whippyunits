#!/usr/bin/env cargo +nightly -Zscript

//! Conversion factor range linter for whippyunits-core
//!
//! This linter ensures that all unit conversion factors are within the specified range
//! [1/sqrt(10), sqrt(10)] ≈ [0.316, 3.162] as per the whippyunits specification.
//!
//! The rationale for this range is to ensure that conversion factors are reasonably
//! close to 1.0, which helps maintain numerical stability and prevents extreme
//! scaling that could lead to precision issues.

use whippyunits_core::{Dimension, Unit};

/// The minimum allowed conversion factor: 1/sqrt(10) ≈ 0.3162277660168379
const MIN_CONVERSION_FACTOR: f64 = 0.3162277660168379;

/// The maximum allowed conversion factor: sqrt(10) ≈ 3.1622776601683795
const MAX_CONVERSION_FACTOR: f64 = 3.1622776601683795;

/// Represents a conversion factor that is outside the allowed range
#[derive(Debug, Clone)]
struct ConversionFactorViolation {
    unit_name: String,
    dimension_name: String,
    conversion_factor: f64,
    violation_type: ViolationType,
}

/// Type of violation for a conversion factor
#[derive(Debug, Clone)]
enum ViolationType {
    TooSmall { min_allowed: f64, actual: f64 },
    TooLarge { max_allowed: f64, actual: f64 },
    ExactlyRepresentable { 
        actual: f64, 
        prime_representation: String,
        suggestion: String 
    },
}

impl ConversionFactorViolation {
    fn new(unit: &Unit, violation_type: ViolationType) -> Self {
        Self {
            unit_name: unit.name.to_string(),
            dimension_name: "Unknown".to_string(), // Will be filled in by caller
            conversion_factor: unit.conversion_factor,
            violation_type,
        }
    }
}

/// Check if a conversion factor is within the allowed range
fn is_conversion_factor_valid(factor: f64) -> bool {
    // Handle identity conversion factor (1.0) - always valid
    if factor == 1.0 {
        return true;
    }
    
    // Check if factor is within the allowed range
    factor >= MIN_CONVERSION_FACTOR && factor <= MAX_CONVERSION_FACTOR
}

/// Check if a conversion factor is exactly representable using the prime factor scheme
/// Returns Some((prime_representation, suggestion)) if representable, None otherwise
fn is_exactly_representable(factor: f64) -> Option<(String, String)> {
    // Handle special cases
    if factor == 1.0 {
        return Some(("1".to_string(), "Use identity conversion (no conversion needed)".to_string()));
    }
    
    // Search for exact representation using powers of 2, 3, and 5
    // We'll search up to reasonable exponent limits to keep it manageable
    const MAX_EXPONENT: i32 = 20; // Reasonable limit for practical conversion factors
    
    // Try all combinations of exponents for 2, 3, and 5
    for exp2 in -MAX_EXPONENT..=MAX_EXPONENT {
        for exp3 in -MAX_EXPONENT..=MAX_EXPONENT {
            for exp5 in -MAX_EXPONENT..=MAX_EXPONENT {
                let calculated_factor = 2.0_f64.powi(exp2) * 3.0_f64.powi(exp3) * 5.0_f64.powi(exp5);
                
                // Check if this combination gives us the exact factor
                if (factor - calculated_factor).abs() < 1e-15 {
                    let prime_repr = format_prime_representation(exp2, exp3, exp5);
                    let suggestion = format!("Use exact prime factorization: {}", prime_repr);
                    return Some((prime_repr, suggestion));
                }
            }
        }
    }
    
    None
}

/// Format the prime representation as a readable string
fn format_prime_representation(exp2: i32, exp3: i32, exp5: i32) -> String {
    let mut terms = Vec::new();
    
    if exp2 != 0 {
        if exp2 == 1 {
            terms.push("2".to_string());
        } else if exp2 == -1 {
            terms.push("1/2".to_string());
        } else if exp2 > 0 {
            terms.push(format!("2^{}", exp2));
        } else {
            terms.push(format!("1/2^{}", -exp2));
        }
    }
    
    if exp3 != 0 {
        if exp3 == 1 {
            terms.push("3".to_string());
        } else if exp3 == -1 {
            terms.push("1/3".to_string());
        } else if exp3 > 0 {
            terms.push(format!("3^{}", exp3));
        } else {
            terms.push(format!("1/3^{}", -exp3));
        }
    }
    
    if exp5 != 0 {
        if exp5 == 1 {
            terms.push("5".to_string());
        } else if exp5 == -1 {
            terms.push("1/5".to_string());
        } else if exp5 > 0 {
            terms.push(format!("5^{}", exp5));
        } else {
            terms.push(format!("1/5^{}", -exp5));
        }
    }
    
    if terms.is_empty() {
        "1".to_string()
    } else {
        terms.join(" × ")
    }
}

/// Find all units with conversion factors outside the allowed range or exactly representable
fn find_conversion_factor_violations() -> Vec<ConversionFactorViolation> {
    let mut violations = Vec::new();
    
    // Check all units across all dimensions
    for dimension in Dimension::ALL {
        for unit in dimension.units {
            // Skip units with identity conversion factor (1.0) - these are always valid
            if unit.conversion_factor == 1.0 {
                continue;
            }
            
            // First check if the conversion factor is exactly representable
            if let Some((prime_representation, suggestion)) = is_exactly_representable(unit.conversion_factor) {
                let violation_type = ViolationType::ExactlyRepresentable {
                    actual: unit.conversion_factor,
                    prime_representation,
                    suggestion,
                };
                
                let mut violation = ConversionFactorViolation::new(unit, violation_type);
                violation.dimension_name = dimension.name.to_string();
                violations.push(violation);
                continue; // Skip range check for exactly representable factors
            }
            
            // Check if factor is outside the allowed range
            if !is_conversion_factor_valid(unit.conversion_factor) {
                let violation_type = if unit.conversion_factor < MIN_CONVERSION_FACTOR {
                    ViolationType::TooSmall {
                        min_allowed: MIN_CONVERSION_FACTOR,
                        actual: unit.conversion_factor,
                    }
                } else {
                    ViolationType::TooLarge {
                        max_allowed: MAX_CONVERSION_FACTOR,
                        actual: unit.conversion_factor,
                    }
                };
                
                let mut violation = ConversionFactorViolation::new(unit, violation_type);
                violation.dimension_name = dimension.name.to_string();
                violations.push(violation);
            }
        }
    }
    
    violations
}

/// Collect statistics about conversion factors
fn collect_conversion_factor_statistics() -> (usize, usize, f64, f64, f64) {
    let mut total_units = 0;
    let mut units_with_conversion = 0;
    let mut min_factor = f64::INFINITY;
    let mut max_factor = f64::NEG_INFINITY;
    let mut sum_factors = 0.0;
    
    for dimension in Dimension::ALL {
        for unit in dimension.units {
            total_units += 1;
            
            if unit.conversion_factor != 1.0 {
                units_with_conversion += 1;
                min_factor = min_factor.min(unit.conversion_factor);
                max_factor = max_factor.max(unit.conversion_factor);
                sum_factors += unit.conversion_factor;
            }
        }
    }
    
    let avg_factor = if units_with_conversion > 0 {
        sum_factors / units_with_conversion as f64
    } else {
        1.0
    };
    
    (total_units, units_with_conversion, min_factor, max_factor, avg_factor)
}

/// Print a detailed report of conversion factor violations
fn print_report() {
    println!("🔍 WhippyUnits Conversion Factor Linter");
    println!("========================================\n");
    
    println!("📏 SPECIFICATION:");
    println!("  All conversion factors must be in range [1/√10, √10]");
    println!("  Range: [{:.6}, {:.6}]", MIN_CONVERSION_FACTOR, MAX_CONVERSION_FACTOR);
    println!("  Rationale: Maintain numerical stability and prevent extreme scaling");
    println!("  Additionally detects factors exactly representable with prime factorization\n");
    
    let violations = find_conversion_factor_violations();
    
    if !violations.is_empty() {
        println!("❌ CONVERSION FACTOR ISSUES FOUND:");
        println!("These units have conversion factors that need attention:\n");
        
        for violation in &violations {
            println!("  Unit: {} ({})", violation.unit_name, violation.dimension_name);
            println!("    Conversion factor: {:.10}", violation.conversion_factor);
            
            match &violation.violation_type {
                ViolationType::TooSmall { min_allowed, actual } => {
                    println!("    ❌ TOO SMALL: {:.10} < {:.10} (minimum allowed)", actual, min_allowed);
                    println!("    💡 Suggestion: Consider adjusting the scale or conversion factor");
                }
                ViolationType::TooLarge { max_allowed, actual } => {
                    println!("    ❌ TOO LARGE: {:.10} > {:.10} (maximum allowed)", actual, max_allowed);
                    println!("    💡 Suggestion: Consider adjusting the scale or conversion factor");
                }
                ViolationType::ExactlyRepresentable { actual, prime_representation, suggestion } => {
                    println!("    ⚠️  EXACTLY REPRESENTABLE: {:.10} can be exactly represented", actual);
                    println!("    🔢 Prime factorization: {}", prime_representation);
                    println!("    💡 Suggestion: {}", suggestion);
                }
            }
            println!();
        }
        
        println!("🚨 ACTION REQUIRED: Conversion factor issues must be addressed!");
        println!("   Range violations could lead to numerical instability or precision issues.");
        println!("   Exactly representable factors should be converted to use prime factorization.\n");
    } else {
        println!("✅ All conversion factors are within the allowed range!");
        println!("   No violations found.\n");
    }
    
    // Print statistics
    let (total_units, units_with_conversion, min_factor, max_factor, avg_factor) = 
        collect_conversion_factor_statistics();
    
    println!("📊 STATISTICS:");
    println!("  Total units: {}", total_units);
    println!("  Units with non-identity conversion factors: {}", units_with_conversion);
    
    if units_with_conversion > 0 {
        println!("  Conversion factor range: [{:.10}, {:.10}]", min_factor, max_factor);
        println!("  Average conversion factor: {:.10}", avg_factor);
        
        // Check if all factors are within range
        let all_valid = min_factor >= MIN_CONVERSION_FACTOR && max_factor <= MAX_CONVERSION_FACTOR;
        if all_valid {
            println!("  ✅ All conversion factors are within specification range");
        } else {
            println!("  ❌ Some conversion factors are outside specification range");
        }
    } else {
        println!("  All units use identity conversion factors (1.0)");
    }
    
    // Summary
    if !violations.is_empty() {
        let range_violations = violations.iter().filter(|v| matches!(v.violation_type, ViolationType::TooSmall { .. } | ViolationType::TooLarge { .. })).count();
        let exactly_representable = violations.iter().filter(|v| matches!(v.violation_type, ViolationType::ExactlyRepresentable { .. })).count();
        
        println!("\n📋 SUMMARY:");
        println!("  - {} total conversion factor issues found", violations.len());
        println!("    - {} range violations (too small/large)", range_violations);
        println!("    - {} exactly representable factors", exactly_representable);
        println!("  - {} total units checked", total_units);
        println!("  - {} units with non-identity conversion factors", units_with_conversion);
        
        std::process::exit(1);
    }
}

/// Print detailed information about the allowed range
fn print_range_info() {
    println!("\n📐 RANGE DETAILS:");
    println!("================\n");
    
    println!("  Minimum allowed: 1/√10 = {:.10}", MIN_CONVERSION_FACTOR);
    println!("  Maximum allowed: √10   = {:.10}", MAX_CONVERSION_FACTOR);
    println!("  Range width:     {:.10}", MAX_CONVERSION_FACTOR - MIN_CONVERSION_FACTOR);
    println!("  Center point:    {:.10}", (MIN_CONVERSION_FACTOR + MAX_CONVERSION_FACTOR) / 2.0);
    
    println!("\n  Common conversion factors within range:");
    println!("    - 1.0 (identity) ✅");
    println!("    - 2.54 (inch to cm) ✅");
    println!("    - 0.3048 (foot to m) ✅");
    println!("    - 0.9144 (yard to m) ✅");
    println!("    - 1.609344 (mile to km) ✅");
    
    println!("\n  Examples of factors that would be OUTSIDE range:");
    println!("    - 0.1 (too small) ❌");
    println!("    - 10.0 (too large) ❌");
    println!("    - 0.01 (too small) ❌");
    println!("    - 100.0 (too large) ❌");
}

fn main() {
    print_report();
    print_range_info();
}
