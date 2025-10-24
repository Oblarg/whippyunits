use strsim::damerau_levenshtein;
use whippyunits_core::Dimension;

/// Find similar dimensions to the given unknown dimension name
/// Returns suggestions with similarity scores above the threshold
pub fn find_similar_dimensions(unknown_dimension: &str, threshold: f64) -> Vec<(String, f64)> {
    let mut suggestions = Vec::new();
    
    // Get all available dimension names and symbols
    let all_dimensions = get_all_available_dimensions();
    
    for dimension_name in all_dimensions {
        let similarity = calculate_similarity(unknown_dimension, &dimension_name);
        if similarity >= threshold {
            suggestions.push((dimension_name, similarity));
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

/// Get all available dimension names and symbols from the whippyunits-core data
fn get_all_available_dimensions() -> Vec<String> {
    let mut dimensions = Vec::new();
    
    // Get all dimensions from the whippyunits-core data
    for dimension in Dimension::ALL {
        // Add dimension name
        dimensions.push(dimension.name.to_string());
        
        // Add dimension symbol
        dimensions.push(dimension.symbol.to_string());
    }
    
    // Remove duplicates and sort
    dimensions.sort();
    dimensions.dedup();
    dimensions
}
