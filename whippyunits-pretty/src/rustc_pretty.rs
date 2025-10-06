use anyhow::Result;
use log::debug;

use whippyunits_lsp_proxy::unit_formatter::{UnitFormatter, DisplayConfig};

/// Pretty printer for rustc output with whippyunits type formatting
pub struct RustcPrettyPrinter {
    formatter: UnitFormatter,
    display_config: DisplayConfig,
}

impl RustcPrettyPrinter {
    pub fn new() -> Self {
        Self::with_config(DisplayConfig::default())
    }

    pub fn with_config(display_config: DisplayConfig) -> Self {
        Self {
            formatter: UnitFormatter::new(),
            display_config,
        }
    }

    /// Process a complete rustc output string
    pub fn process_rustc_output(&mut self, output: &str) -> Result<String> {
        let lines: Vec<&str> = output.lines().collect();
        let mut processed_lines = Vec::new();

        for line in lines {
            let processed = self.process_line(line)?;
            processed_lines.push(processed);
        }

        Ok(processed_lines.join("\n"))
    }

    /// Process a single line of rustc output
    pub fn process_line(&mut self, line: &str) -> Result<String> {
        // Check if this line contains whippyunits types using the same logic as LSP proxy
        if self.contains_whippyunits_types(line) {
            debug!("Processing line with whippyunits types: {}", line);
            
            // Apply type conversion using the updated formatter
            let processed = self.formatter.format_types(
                line, 
                &self.display_config
            );
            
            // If we made changes, log them
            if processed != line {
                debug!("Transformed: {} -> {}", line, processed);
            }
            
            Ok(processed)
        } else {
            // No whippyunits types, pass through unchanged
            Ok(line.to_string())
        }
    }

    /// Check if a line contains whippyunits types using the same logic as LSP proxy
    fn contains_whippyunits_types(&self, line: &str) -> bool {
        // Check for the basic Quantity pattern first
        if !line.contains("Quantity") {
            return false;
        }
        
        // For new format with Scale and Dimension structs
        if line.contains("Scale") && line.contains("Dimension") {
            return true;
        }
        
        // For old format, check for Quantity< pattern
        if line.contains("Quantity<") {
            return true;
        }
        
        false
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rustc_output_processing() {
        let mut printer = RustcPrettyPrinter::new();
        
        // Test with new Scale/Dimension format
        let rustc_output = r#"error[E0308]: mismatched types
 --> src/main.rs:5:9
  |
5 |     let x: Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64> = 5.0;
  |         ^   expected `Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64>`, found `{float}`
  |
  = note: expected struct `Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64>`
             found type `{float}`"#;
        
        let processed = printer.process_rustc_output(rustc_output).unwrap();
        println!("Processed output:\n{}", processed);
        
        // Should contain pretty-printed types
        assert!(processed.contains("m"));
        assert!(!processed.contains("_L<1>"));
    }

    #[test]
    fn test_line_processing() {
        let mut printer = RustcPrettyPrinter::new();
        
        let line = "    let x: Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64> = 5.0;";
        let processed = printer.process_line(line).unwrap();
        
        println!("Original: {}", line);
        println!("Processed: {}", processed);
        
        // Should contain pretty-printed type
        assert!(processed.contains("m"));
        assert!(!processed.contains("_L<1>"));
    }

    #[test]
    fn test_contains_whippyunits_types() {
        let printer = RustcPrettyPrinter::new();
        
        // Test new format
        assert!(printer.contains_whippyunits_types("Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64>"));
        
        // Test old format
        assert!(printer.contains_whippyunits_types("Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>"));
        
        // Test non-whippyunits types
        assert!(!printer.contains_whippyunits_types("let x: String = \"hello\";"));
        assert!(!printer.contains_whippyunits_types("let x: i32 = 42;"));
    }
}
