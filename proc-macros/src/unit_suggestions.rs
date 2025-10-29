use strsim::damerau_levenshtein;
use whippyunits_core::{Dimension, SiPrefix};

/// Find similar units to the given unknown unit name
/// Returns suggestions with similarity scores above the threshold
pub fn find_similar_units(unknown_unit: &str, threshold: f64) -> Vec<(String, f64)> {
    let mut suggestions = Vec::new();

    // Get all available unit symbols and names
    let all_units = get_all_available_units();

    for unit_name in all_units {
        let similarity = calculate_similarity(unknown_unit, &unit_name);
        if similarity >= threshold {
            suggestions.push((unit_name, similarity));
        }
    }

    // Sort by similarity score (highest first)
    suggestions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Limit to top 3 suggestions
    suggestions.truncate(3);
    suggestions
}

/// Calculate similarity between two strings using Damerau-Levenshtein distance
fn calculate_similarity(s1: &str, s2: &str) -> f64 {
    let distance = damerau_levenshtein(&s1.to_lowercase(), &s2.to_lowercase());
    let max_len = s1.len().max(s2.len()) as f64;

    if max_len == 0.0 {
        1.0
    } else {
        1.0 - (distance as f64 / max_len)
    }
}

/// Get all available unit symbols and names from the whippyunits-core data
fn get_all_available_units() -> Vec<String> {
    let mut units = Vec::new();

    // Get all units from all dimensions
    for dimension in Dimension::ALL {
        // Add unit symbols and names
        for unit in dimension.units {
            // Add all symbols for this unit
            for symbol in unit.symbols {
                units.push(symbol.to_string());
            }
            // Add the unit name
            units.push(unit.name.to_string());
        }
    }

    // Add prefixed units ONLY for base units (first unit in each dimension)
    for prefix in SiPrefix::ALL {
        for dimension in Dimension::ALL {
            // Only prefix the first unit in each dimension (the base unit)
            if let Some(base_unit) = dimension.units.first() {
                // Add prefixed symbols (using the first symbol)
                if let Some(first_symbol) = base_unit.symbols.first() {
                    let prefixed_symbol = format!("{}{}", prefix.symbol(), first_symbol);
                    units.push(prefixed_symbol);
                }

                // Add prefixed names
                let prefixed_name = format!("{}{}", prefix.name(), base_unit.name);
                units.push(prefixed_name);
            }
        }
    }

    // Remove duplicates and sort
    units.sort();
    units.dedup();
    units
}
