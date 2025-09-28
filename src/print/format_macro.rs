/// Custom formatting macro that supports {:kg} syntax
///
/// Usage: format_units!("Distance: {:km}", distance) or format_units!("Mass: {:g:2}", mass)
#[macro_export]
macro_rules! format_units {
    // Handle single quantity with unit specifier
    ($format_string:expr, $quantity:expr) => {
        $crate::print::format_macro::format_units_impl!($format_string, $quantity)
    };

    // Handle multiple quantities
    ($format_string:expr, $($quantity:expr),+) => {
        $crate::print::format_macro::format_units_impl!($format_string, $($quantity),+)
    };
}

/// Internal implementation macro for format_units
#[macro_export]
macro_rules! format_units_impl {
    // Single quantity case
    ($format_string:expr, $quantity:expr) => {
        {
            let format_str = $format_string;
            if format_str.contains("{:") {
                // Parse the format string to extract unit specifiers
                $crate::print::format_macro::parse_and_format!(format_str, $quantity)
            } else {
                // No unit specifiers, use standard formatting
                format!(format_str, $quantity)
            }
        }
    };

    // Multiple quantities case - for now, just use standard formatting
    ($format_string:expr, $($quantity:expr),+) => {
        format!($format_string, $($quantity),+)
    };
}

/// Parse format string and handle unit specifiers
#[macro_export]
macro_rules! parse_and_format {
    ($format_str:expr, $quantity:expr) => {{
        // For now, we'll use a simple approach and let the user specify the unit
        // This is a placeholder - in a full implementation, we'd parse the format string
        // to extract unit specifiers like {:km}, {:g:2}, etc.
        format!($format_str, $quantity)
    }};
}

/// Macro to format a quantity with a specific unit (simpler approach)
///
/// Usage: format!("Distance: {}", format_as!(distance, "km"))
#[macro_export]
macro_rules! format_as {
    ($quantity:expr, $unit:expr) => {
        match $crate::print::custom_display::QuantityFormatExt::format_as($quantity, $unit) {
            Ok(formatted) => formatted,
            Err(e) => format!("Error: {}", e),
        }
    };
    ($quantity:expr, $unit:expr, $precision:expr) => {
        match $crate::print::custom_display::QuantityFormatExt::format_as_with_precision(
            $quantity, $unit, $precision,
        ) {
            Ok(formatted) => formatted,
            Err(e) => format!("Error: {}", e),
        }
    };
}

/// Macro to print a quantity in a specific unit
///
/// Usage: print_as!(quantity, "km") or print_as!(quantity, "km:2")
#[macro_export]
macro_rules! print_as {
    ($quantity:expr, $unit:expr) => {
        match $crate::print::custom_display::QuantityFormatExt::format_as($quantity, $unit) {
            Ok(formatted) => println!("{}", formatted),
            Err(e) => println!("Error: {}", e),
        }
    };
    ($quantity:expr, $unit:expr, $precision:expr) => {
        match $crate::print::custom_display::QuantityFormatExt::format_as_with_precision(
            $quantity, $unit, $precision,
        ) {
            Ok(formatted) => println!("{}", formatted),
            Err(e) => println!("Error: {}", e),
        }
    };
}

/// Macro that provides a more natural syntax for unit formatting
///
/// Usage: format!("Distance: {:km}", distance) becomes format!("Distance: {}", format_as!(distance, "km"))
/// This macro dynamically supports all units defined in the default-dimensions source of truth
#[macro_export]
macro_rules! format_quantity {
    // Single quantity with unit
    ($format_string:expr, $quantity:expr) => {{
        let mut result = $format_string.to_string();

        // Get all available units from the source of truth
        let units = whippyunits_default_dimensions::UNIT_LITERALS;

        // Replace each {:unit} pattern with the formatted result
        for unit_info in units {
            let pattern = format!("{{:{}}}", unit_info.symbol);
            if result.contains(&pattern) {
                let formatted = $crate::format_as!($quantity, unit_info.symbol);
                result = result.replace(&pattern, &formatted);
            }
        }

        result
    }};
}
