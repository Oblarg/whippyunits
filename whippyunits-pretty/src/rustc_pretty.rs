use regex::Regex;
use anyhow::Result;
use log::debug;

use whippyunits_lsp_proxy::{unit_formatter::{UnitFormatter, DisplayConfig}};

/// Pretty printer for rustc output with whippyunits type formatting
pub struct RustcPrettyPrinter {
    formatter: UnitFormatter,
    display_config: DisplayConfig,
    quantity_regex: Regex,
    error_regex: Regex,
    warning_regex: Regex,
    note_regex: Regex,
}

impl RustcPrettyPrinter {
    pub fn new() -> Self {
        Self::with_config(DisplayConfig::default())
    }

    pub fn with_config(display_config: DisplayConfig) -> Self {
        Self {
            formatter: UnitFormatter::new(),
            display_config,
            quantity_regex: Regex::new(r"Quantity<([^>]+)>").unwrap(),
            error_regex: Regex::new(r"^error\[([^\]]+)\]: (.+)$").unwrap(),
            warning_regex: Regex::new(r"^warning\[([^\]]+)\]: (.+)$").unwrap(),
            note_regex: Regex::new(r"^(\s+)note: (.+)$").unwrap(),
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
        // Check if this line contains whippyunits types (both old and new format)
        if self.quantity_regex.is_match(line) || line.contains("Scale<") && line.contains("Dimension<") {
            debug!("Processing line with whippyunits types: {}", line);
            
            // Apply type conversion
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

    /// Check if a line is an error message
    pub fn is_error_line(&self, line: &str) -> bool {
        self.error_regex.is_match(line)
    }

    /// Check if a line is a warning message
    pub fn is_warning_line(&self, line: &str) -> bool {
        self.warning_regex.is_match(line)
    }

    /// Check if a line is a note message
    pub fn is_note_line(&self, line: &str) -> bool {
        self.note_regex.is_match(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rustc_output_processing() {
        let mut printer = RustcPrettyPrinter::new();
        
        let rustc_output = r#"error[E0308]: mismatched types
 --> src/main.rs:5:9
  |
5 |     let x: Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807> = 5.0;
  |         ^   expected `Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>`, found `{float}`
  |
  = note: expected struct `Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>`
             found type `{float}`"#;
        
        let processed = printer.process_rustc_output(rustc_output).unwrap();
        println!("Processed output:\n{}", processed);
        
        // Should contain pretty-printed types
        assert!(processed.contains("m"));
        assert!(!processed.contains("9223372036854775807"));
    }

    #[test]
    fn test_line_processing() {
        let mut printer = RustcPrettyPrinter::new();
        
        let line = "    let x: Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807> = 5.0;";
        let processed = printer.process_line(line).unwrap();
        
        println!("Original: {}", line);
        println!("Processed: {}", processed);
        
        // Should contain pretty-printed type
        assert!(processed.contains("m"));
        assert!(!processed.contains("9223372036854775807"));
    }
}
