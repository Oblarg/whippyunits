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

/// Find all units with conversion factors outside the allowed range
fn find_conversion_factor_violations() -> Vec<ConversionFactorViolation> {
    let mut violations = Vec::new();
    
    // Check all units across all dimensions
    for dimension in Dimension::ALL {
        for unit in dimension.units {
            // Skip units with identity conversion factor (1.0) - these are always valid
            if unit.conversion_factor == 1.0 {
                continue;
            }
            
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
    println!("🔍 WhippyUnits Conversion Factor Range Linter");
    println!("==============================================\n");
    
    println!("📏 SPECIFICATION:");
    println!("  All conversion factors must be in range [1/√10, √10]");
    println!("  Range: [{:.6}, {:.6}]", MIN_CONVERSION_FACTOR, MAX_CONVERSION_FACTOR);
    println!("  Rationale: Maintain numerical stability and prevent extreme scaling\n");
    
    let violations = find_conversion_factor_violations();
    
    if !violations.is_empty() {
        println!("❌ CONVERSION FACTOR VIOLATIONS FOUND:");
        println!("These units have conversion factors outside the allowed range:\n");
        
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
            }
            println!();
        }
        
        println!("🚨 ACTION REQUIRED: Conversion factor violations must be resolved!");
        println!("   These violations could lead to numerical instability or precision issues.\n");
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
        println!("\n📋 SUMMARY:");
        println!("  - {} conversion factor violations found", violations.len());
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
