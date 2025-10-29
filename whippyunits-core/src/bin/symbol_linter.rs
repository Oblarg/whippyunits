#!/usr/bin/env cargo +nightly -Zscript

//! Symbol uniqueness linter for whippyunits-core
//!
//! This linter ensures that all unit symbols are uniquely readable by checking
//! that no concrete unit symbol conflicts with a combination of an SI prefix
//! and a base unit symbol.

use std::collections::HashMap;
use whippyunits_core::{Dimension, SiPrefix, Unit};

/// Represents a symbol conflict between a concrete unit and a prefix+base combination
#[derive(Debug, Clone)]
struct SymbolConflict {
    symbol: String,
    concrete_unit: String,
    concrete_dimension: String,
    conflicting_prefix: String,
    conflicting_base: String,
    conflicting_base_dimension: String,
}

/// Known exceptions - symbols that are allowed to conflict with prefix+base combinations
/// These are cases where the conflict is acceptable (e.g., legitimate SI units)
const KNOWN_EXCEPTIONS: &[&str] = &[
    "ft",   // foot vs femto + tesla (fT) - imperial unit conflict
    "pa",   // pascal vs pico + ampere (pA) - legitimate SI unit ambiguity
    "grad", // gradian vs grad + tesla (gradT) - legitimate SI unit ambiguity
    "ct",   // carat vs centitesla (cT)
    "nm",   // nanometer vs Newton-meter (Nm)
    "pc",   // parsec vs picocoulomb (pC)
    "ev",   // electron-volt (eV) vs exavolt (EV)
];

/// Collect all concrete unit symbols from the codebase (case-insensitive)
fn collect_concrete_symbols() -> HashMap<String, (String, String)> {
    let mut symbols = HashMap::new();

    // Collect symbols from all dimensions
    for dimension in Dimension::ALL {
        for unit in dimension.units {
            for symbol in unit.symbols {
                symbols.insert(
                    symbol.to_lowercase(),
                    (unit.name.to_string(), dimension.name.to_string()),
                );
            }
        }
    }

    symbols
}

/// Generate all possible prefix + base unit symbol combinations (case-insensitive)
fn generate_prefixed_symbols() -> HashMap<String, (String, String, String)> {
    let mut prefixed_symbols = HashMap::new();

    // Only consider base units (first unit in array for each dimension)
    for dimension in Dimension::ALL {
        // Find the first unit that is prefixable (no conversion factor)
        if let Some(prefixable_unit) = dimension
            .units
            .iter()
            .filter(|unit| !unit.has_conversion())
            .next()
        {
            for symbol in prefixable_unit.symbols {
                // Generate all prefix + symbol combinations
                for prefix in SiPrefix::ALL {
                    let prefixed_symbol = format!("{}{}", prefix.symbol(), symbol).to_lowercase();
                    prefixed_symbols.insert(
                        prefixed_symbol,
                        (
                            prefix.symbol().to_string(),
                            symbol.to_string(),
                            dimension.name.to_string(),
                        ),
                    );
                }
            }
        }
    }

    prefixed_symbols
}

/// Find conflicts between concrete symbols and prefix+base combinations
fn find_conflicts() -> Vec<SymbolConflict> {
    let concrete_symbols = collect_concrete_symbols();
    let prefixed_symbols = generate_prefixed_symbols();
    let mut conflicts = Vec::new();

    // Check for conflicts: concrete symbols that match prefix+base combinations
    for (symbol, (concrete_unit, concrete_dimension)) in &concrete_symbols {
        if let Some((prefix, base, base_dimension)) = prefixed_symbols.get(symbol) {
            // Skip known exceptions
            if !KNOWN_EXCEPTIONS.contains(&symbol.as_str()) {
                conflicts.push(SymbolConflict {
                    symbol: symbol.clone(),
                    concrete_unit: concrete_unit.clone(),
                    concrete_dimension: concrete_dimension.clone(),
                    conflicting_prefix: prefix.clone(),
                    conflicting_base: base.clone(),
                    conflicting_base_dimension: base_dimension.clone(),
                });
            }
        }
    }

    conflicts
}

/// Check for duplicate symbols within concrete units
fn find_duplicate_symbols() -> Vec<(String, Vec<(String, String)>)> {
    let concrete_symbols = collect_concrete_symbols();
    let mut symbol_to_units: HashMap<String, Vec<(String, String)>> = HashMap::new();

    // Group symbols by their string value
    for (symbol, (unit_name, dimension_name)) in concrete_symbols {
        symbol_to_units
            .entry(symbol)
            .or_default()
            .push((unit_name, dimension_name));
    }

    // Find symbols that appear in multiple units
    symbol_to_units
        .into_iter()
        .filter(|(_, units)| units.len() > 1)
        .collect()
}

/// Check for symbols that could be ambiguous with prefix+base combinations
fn find_potential_ambiguities() -> Vec<(String, String, String, String)> {
    let concrete_symbols = collect_concrete_symbols();
    let mut potential_ambiguities = Vec::new();

    // Check if any concrete symbol could be interpreted as a prefix+base combination
    for (symbol, (unit_name, dimension_name)) in &concrete_symbols {
        // Try to parse the symbol as prefix+base
        if let Some((prefix, base)) = SiPrefix::strip_any_prefix_symbol(symbol) {
            // Check if the base is a valid prefixable unit symbol (first in lexical order)
            if let Some((base_unit, _base_dimension)) = find_prefixable_unit_by_symbol(base) {
                // Skip known exceptions
                if !KNOWN_EXCEPTIONS.contains(&symbol.as_str()) {
                    potential_ambiguities.push((
                        symbol.clone(),
                        unit_name.clone(),
                        dimension_name.clone(),
                        format!(
                            "{}{} ({} prefix + {} base)",
                            prefix.symbol(),
                            base,
                            prefix.name(),
                            base_unit.name
                        ),
                    ));
                }
            }
        }
    }

    potential_ambiguities
}

/// Find a prefixable unit by its symbol (first unit in array for each dimension, case-insensitive)
fn find_prefixable_unit_by_symbol(symbol: &str) -> Option<(&'static Unit, &'static Dimension)> {
    let symbol_lower = symbol.to_lowercase();
    Dimension::ALL.iter().find_map(|dimension| {
        dimension
            .units
            .iter()
            .filter(|unit| !unit.has_conversion())
            .next()
            .and_then(|unit| {
                if unit
                    .symbols
                    .iter()
                    .any(|s| s.to_lowercase() == symbol_lower)
                {
                    Some((unit, dimension))
                } else {
                    None
                }
            })
    })
}

/// Print a detailed report of all symbol issues
fn print_report() {
    println!("🔍 WhippyUnits Symbol Uniqueness Linter");
    println!("==========================================\n");

    let conflicts = find_conflicts();
    let duplicates = find_duplicate_symbols();
    let ambiguities = find_potential_ambiguities();

    // Report conflicts (most serious issue)
    if !conflicts.is_empty() {
        println!("❌ SYMBOL CONFLICTS FOUND:");
        println!("These concrete unit symbols conflict with prefix+base combinations:\n");

        for conflict in &conflicts {
            println!("  Symbol: '{}'", conflict.symbol);
            println!(
                "    Concrete unit: {} ({})",
                conflict.concrete_unit, conflict.concrete_dimension
            );
            println!(
                "    Conflicts with: {} prefix + {} base ({})",
                conflict.conflicting_prefix,
                conflict.conflicting_base,
                conflict.conflicting_base_dimension
            );
            println!();
        }
    }

    // Report duplicate symbols
    if !duplicates.is_empty() {
        println!("⚠️  DUPLICATE SYMBOLS FOUND:");
        println!("These symbols are used by multiple units:\n");

        for (symbol, units) in &duplicates {
            println!("  Symbol: '{}'", symbol);
            for (unit_name, dimension_name) in units {
                println!("    - {} ({})", unit_name, dimension_name);
            }
            println!();
        }
    }

    // Report potential ambiguities
    if !ambiguities.is_empty() {
        println!("💡 POTENTIAL AMBIGUITIES:");
        println!("These concrete symbols could be interpreted as prefix+base combinations:\n");

        for (symbol, unit_name, dimension_name, interpretation) in &ambiguities {
            println!("  Symbol: '{}'", symbol);
            println!("    Used by: {} ({})", unit_name, dimension_name);
            println!("    Could be interpreted as: {}", interpretation);
            println!();
        }
    }

    // Show known exceptions
    if !KNOWN_EXCEPTIONS.is_empty() {
        println!("ℹ️  KNOWN EXCEPTIONS:");
        println!("These symbols are allowed to conflict with prefix+base combinations:\n");

        for exception in KNOWN_EXCEPTIONS {
            println!("  - '{}' (accepted conflict)", exception);
        }
        println!();
    }

    // Summary
    let total_issues = conflicts.len() + duplicates.len() + ambiguities.len();
    if total_issues == 0 {
        println!("✅ No symbol conflicts found! All symbols are uniquely readable.");
    } else {
        println!("📊 SUMMARY:");
        println!("  - {} symbol conflicts", conflicts.len());
        println!("  - {} duplicate symbols", duplicates.len());
        println!("  - {} potential ambiguities", ambiguities.len());
        println!("  - {} total issues", total_issues);

        if !conflicts.is_empty() {
            println!("\n🚨 ACTION REQUIRED: Symbol conflicts must be resolved!");
            std::process::exit(1);
        }
    }
}

/// Print statistics about the symbol space
fn print_statistics() {
    println!("\n📈 SYMBOL STATISTICS:");
    println!("====================\n");

    let concrete_symbols = collect_concrete_symbols();
    let prefixed_symbols = generate_prefixed_symbols();

    println!("  Total concrete unit symbols: {}", concrete_symbols.len());
    println!(
        "  Total possible prefix+base combinations: {}",
        prefixed_symbols.len()
    );

    // Count symbols by dimension
    let mut dimension_counts: HashMap<String, usize> = HashMap::new();
    for dimension in Dimension::ALL {
        let mut count = 0;
        for unit in dimension.units {
            count += unit.symbols.len();
        }
        dimension_counts.insert(dimension.name.to_string(), count);
    }

    println!("\n  Symbols by dimension:");
    for (dimension, count) in dimension_counts {
        println!("    {}: {}", dimension, count);
    }

    // Count prefix usage
    println!("\n  SI Prefixes available: {}", SiPrefix::ALL.len());
    println!("  SI Base units: {}", Dimension::BASIS.len());
}

fn main() {
    print_report();
    print_statistics();
}
