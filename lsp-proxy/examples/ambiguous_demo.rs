use whippyunits_lsp_proxy::{WhippyUnitsTypeConverter, DisplayConfig};

fn main() {
    println!("🚀 WHIPPYUNITS LSP PROXY - COMPACT UNRESOLVED TYPE NOTATION");
    println!("============================================================");
    println!();

    let converter = WhippyUnitsTypeConverter::new();

    // Test the actual unresolved type from result2 (distance1 * distance2)
    println!("Test 1: Actual unresolved type from result2 (distance1 * distance2)");
    println!("Input: Quantity<_, -1, _, 9223372036854775807, _, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>");
    let test1 = "Quantity<_, -1, _, 9223372036854775807, _, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>";
    let result1 = converter.convert_types_in_text_with_config(test1, &DisplayConfig::default());
    println!("Output: {}", result1);
    println!();

    // Test with more resolved parameters
    println!("Test 2: More resolved parameters");
    println!("Input: Quantity<2, -1, 0, 9223372036854775807, _, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>");
    let test2 = "Quantity<2, -1, 0, 9223372036854775807, _, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>";
    let result2 = converter.convert_types_in_text_with_config(test2, &DisplayConfig::default());
    println!("Output: {}", result2);
    println!();

    // Test with mostly sentinel values
    println!("Test 3: Mostly sentinel values");
    println!("Input: Quantity<_, _, _, 9223372036854775807, _, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>");
    let test3 = "Quantity<_, _, _, 9223372036854775807, _, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>";
    let result3 = converter.convert_types_in_text_with_config(test3, &DisplayConfig::default());
    println!("Output: {}", result3);
    println!();

    // Test with exponent 0 (should be pruned)
    println!("Test 5: Exponent 0 should be pruned");
    println!("Input: Quantity<1, -1, 0, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>");
    let test5 = "Quantity<1, -1, 0, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>";
    let result5 = converter.convert_types_in_text_with_config(test5, &DisplayConfig::default());
    println!("Output: {}", result5);
    println!();

    // Test with verbose mode
    println!("Test 4: Unresolved type in verbose mode");
    let config = DisplayConfig { verbose: true, unicode: true, include_raw: false };
    let result4 = converter.convert_types_in_text_with_config(test1, &config);
    println!("Output: {}", result4);
    println!();

    println!("The new compact notation shows scale^exponent with _ for unresolved parts!");
    println!("This gives immediate visual feedback about what's known vs unknown.");
}

