use strsim::damerau_levenshtein;
use whippyunits_core::{Dimension, SiPrefix};

/// Find similar scale identifiers to the given unknown scale name
/// Returns suggestions with similarity scores above the threshold
pub fn find_similar_scales(unknown_scale: &str, threshold: f64) -> Vec<(String, f64)> {
    let mut suggestions = Vec::new();
    
    // Get all available scale identifiers
    let all_scales = get_all_available_scales();
    
    for scale_name in all_scales {
        let similarity = calculate_similarity(unknown_scale, &scale_name);
        if similarity >= threshold {
            suggestions.push((scale_name, similarity));
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
    let distance = damerau_levenshtein(s1, s2);
    let max_len = s1.len().max(s2.len()) as f64;
    
    if max_len == 0.0 {
        1.0
    } else {
        1.0 - (distance as f64 / max_len)
    }
}

/// Get all available scale identifiers from the whippyunits-core data
fn get_all_available_scales() -> Vec<String> {
    let mut scales = Vec::new();
    
    // Get base unit names from all dimensions
    for dimension in Dimension::ALL {
        for unit in dimension.units {
            // Add the unit name (capitalized for scale identifiers)
            let capitalized_name = whippyunits_core::CapitalizedFmt(unit.name).to_string();
            scales.push(capitalized_name);
        }
    }
    
    // Add prefixed unit names for base units only
    for prefix in SiPrefix::ALL {
        for dimension in Dimension::BASIS {
            if let Some(base_unit) = dimension.units.first() {
                let unit_singular = base_unit.name.trim_end_matches('s');
                let combined_name = format!("{}{}", prefix.name(), unit_singular);
                let capitalized_name = whippyunits_core::CapitalizedFmt(&combined_name).to_string();
                scales.push(capitalized_name);
            }
        }
    }
    
    // Remove duplicates and sort
    scales.sort();
    scales.dedup();
    scales
}
