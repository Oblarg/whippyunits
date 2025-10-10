//! Utility functions for whippyunits

use crate::dimensions::{DIMENSIONS, Dimension, Unit};
use crate::base_units::lookup_base_unit;
use crate::DimensionExponents;

/// Capitalize the first character of a string
pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Look up dimension information by name (case-insensitive)
/// This replaces the old DIMENSION_LOOKUP.iter().find() pattern
pub fn lookup_dimension_by_name(name: &str) -> Option<&'static Dimension> {
    let name_lower = name.to_lowercase();
    let name_no_spaces = name_lower.replace(' ', "");

    DIMENSIONS.iter().find(|dim| {
        let dim_name_lower = dim.name.to_lowercase();

        // Direct match
        if dim_name_lower == name_lower {
            return true;
        }

        // Match with spaces removed (for UpperCamelCase support)
        let dim_name_no_spaces = dim_name_lower.replace(' ', "");
        if dim_name_no_spaces == name_no_spaces {
            return true;
        }

        // Handle common naming variations (copied from original logic)
        match dim_name_lower.as_str() {
            "volume mass density" => {
                name_lower == "density"
                    || name_lower == "volume_mass_density"
                    || name_lower == "volumemassdensity"
            }
            "linear mass density" => {
                name_lower == "linear_mass_density" || name_lower == "linearmassdensity"
            }
            "surface mass density" => {
                name_lower == "surface_mass_density" || name_lower == "surfacemassdensity"
            }
            "wave number" => name_lower == "wavenumber" || name_lower == "wave_number",
            "kinematic viscosity" => {
                name_lower == "kinematic_viscosity" || name_lower == "kinematicviscosity"
            }
            "surface tension" => name_lower == "surface_tension" || name_lower == "surfacetension",
            "specific energy" => name_lower == "specific_energy" || name_lower == "specificenergy",
            "specific power" => name_lower == "specific_power" || name_lower == "specificpower",
            "mass flow rate" => name_lower == "mass_flow_rate" || name_lower == "massflowrate",
            "volume flow rate" => {
                name_lower == "volume_flow_rate" || name_lower == "volumeflowrate"
            }
            "power density" => name_lower == "power_density" || name_lower == "powerdensity",
            "force density" => name_lower == "force_density" || name_lower == "forcedensity",
            "heat flux" => name_lower == "heat_flux" || name_lower == "heatflux",
            "electric charge" => {
                name_lower == "electric_charge"
                    || name_lower == "electriccharge"
                    || name_lower == "charge"
            }
            "electric potential" => {
                name_lower == "electric_potential"
                    || name_lower == "electricpotential"
                    || name_lower == "potential"
            }
            "electric resistance" => {
                name_lower == "electric_resistance"
                    || name_lower == "electricresistance"
                    || name_lower == "resistance"
            }
            "electric conductance" => {
                name_lower == "electric_conductance"
                    || name_lower == "electricconductance"
                    || name_lower == "conductance"
            }
            "electric field" => name_lower == "electric_field" || name_lower == "electricfield",
            "magnetic field" => name_lower == "magnetic_field" || name_lower == "magneticfield",
            "magnetic flux" => name_lower == "magnetic_flux" || name_lower == "magneticflux",
            "linear charge density" => {
                name_lower == "linear_charge_density" || name_lower == "linearchargedensity"
            }
            "surface charge density" => {
                name_lower == "surface_charge_density" || name_lower == "surfacechargedensity"
            }
            "volume charge density" => {
                name_lower == "volume_charge_density" || name_lower == "volumechargedensity"
            }
            "magnetizing field" => {
                name_lower == "magnetizing_field" || name_lower == "magnetizingfield"
            }
            "specific heat capacity" => {
                name_lower == "specific_heat_capacity" || name_lower == "specificheatcapacity"
            }
            "molar heat capacity" => {
                name_lower == "molar_heat_capacity" || name_lower == "molarheatcapacity"
            }
            "thermal conductivity" => {
                name_lower == "thermal_conductivity" || name_lower == "thermalconductivity"
            }
            "thermal resistance" => {
                name_lower == "thermal_resistance" || name_lower == "thermalresistance"
            }
            "thermal expansion" => {
                name_lower == "thermal_expansion" || name_lower == "thermalexpansion"
            }
            "molar mass" => name_lower == "molar_mass" || name_lower == "molarmass",
            "molar volume" => name_lower == "molar_volume" || name_lower == "molarvolume",
            "molar concentration" => {
                name_lower == "molar_concentration" || name_lower == "molarconcentration"
            }
            "molal concentration" => {
                name_lower == "molal_concentration" || name_lower == "molalconcentration"
            }
            "molar flow rate" => name_lower == "molar_flow_rate" || name_lower == "molarflowrate",
            "molar flux" => name_lower == "molar_flux" || name_lower == "molarflux",
            "molar energy" => name_lower == "molar_energy" || name_lower == "molarenergy",
            "luminous exposure" => {
                name_lower == "luminous_exposure" || name_lower == "luminousexposure"
            }
            "luminous efficacy" => {
                name_lower == "luminous_efficacy" || name_lower == "luminousefficacy"
            }
            "volume mass density" => {
                name_lower == "volume_mass_density" || name_lower == "volumemassdensity" || name_lower == "density"
            }
            "linear mass density" => {
                name_lower == "linear_mass_density" || name_lower == "linearmassdensity"
            }
            "dynamic viscosity" => {
                name_lower == "dynamic_viscosity" || name_lower == "dynamicviscosity" || name_lower == "viscosity"
            }
            "kinematic viscosity" => {
                name_lower == "kinematic_viscosity" || name_lower == "kinematicviscosity"
            }
            _ => false,
        }
    })
}

/// Look up dimension information by exponents
/// This replaces the old DIMENSION_LOOKUP.iter().find() pattern
pub fn lookup_dimension_by_exponents(exponents: DimensionExponents) -> Option<&'static Dimension> {
    DIMENSIONS.iter().find(|dim| dim.exponents == exponents)
}

/// Look up dimension by symbol
/// This replaces the old DIMENSION_LOOKUP.iter().find() pattern
pub fn lookup_dimension_by_symbol(symbol: &str) -> Option<&'static Dimension> {
    DIMENSIONS.iter().find(|dim| dim.symbol == symbol)
}

/// Get all dimension names - for error message generation
/// This replaces the old DIMENSION_LOOKUP.iter().map(|info| info.name).collect() pattern
pub fn get_all_dimension_names() -> Vec<&'static str> {
    DIMENSIONS.iter().map(|dim| dim.name).collect()
}

/// Get all dimension symbols - for error message generation  
/// This replaces the old DIMENSION_LOOKUP.iter().filter_map(|info| info.symbol).collect() pattern
pub fn get_all_dimension_symbols() -> Vec<&'static str> {
    DIMENSIONS.iter().map(|dim| dim.symbol).collect()
}

/// Get the first 8 atomic dimensions - for symbol generation
/// This replaces the old DIMENSION_LOOKUP.iter().take(8) pattern
pub fn get_atomic_dimensions() -> Vec<&'static Dimension> {
    DIMENSIONS.iter().take(8).collect()
}

/// Get all dimensions
pub fn get_all_dimensions() -> Vec<&'static Dimension> {
    DIMENSIONS.iter().collect()
}

/// Get atomic dimension symbols for symbol generation
/// This replaces the old DIMENSION_LOOKUP.iter().take(8).map(|info| info.symbol.unwrap_or("?")).collect() pattern
pub fn get_atomic_dimension_symbols() -> Vec<&'static str> {
    DIMENSIONS.iter().take(8).map(|dim| dim.symbol).collect()
}

/// Look up a unit by symbol across all dimensions (replaces UNIT_LITERALS lookup)
/// Returns the dimension and unit information
pub fn lookup_unit_literal(unit_symbol: &str) -> Option<(&'static Dimension, &'static Unit)> {
    // First try to find exact matches in the dimensions data
    for dimension in DIMENSIONS {
        for unit in dimension.units {
            if unit.symbols.contains(&unit_symbol) {
                return Some((dimension, unit));
            }
        }
    }
    
    // If not found, try to handle prefixed units (short form like "km", "cm")
    if let Some((base_symbol, _prefix)) = is_prefixed_base_unit(unit_symbol) {
        // Find the base unit in the dimensions data
        for dimension in DIMENSIONS {
            if let Some(unit) = dimension.units.iter().find(|unit| unit.symbols.contains(&base_symbol)) {
                return Some((dimension, unit));
            }
        }
    }
    
    // If still not found, try to handle long prefixed unit names (like "kilometer", "centimeter")
    if let Some(short_form) = scale_type_to_actual_unit_symbol(unit_symbol) {
        // Recursively call this function with the short form
        return lookup_unit_literal(&short_form);
    }
    
    None
}

/// Check if a unit symbol is a prefixed base unit (e.g., "cm", "km", "mm")
/// Returns (base_symbol, prefix) if it's a prefixed unit, None otherwise
pub fn is_prefixed_base_unit(unit_symbol: &str) -> Option<(&'static str, &'static str)> {
    use crate::SI_PREFIXES;
    
    // First check if this is an exact match for a unit literal by checking dimensions data directly
    // If it is, don't treat it as a prefixed unit (e.g., "min" should not be "m" + "in")
    for dimension in DIMENSIONS {
        for unit in dimension.units {
            if unit.symbols.contains(&unit_symbol) {
                return None; // This is an exact match, not a prefixed unit
            }
        }
    }
    
    // Try each SI prefix
    for prefix_info in SI_PREFIXES {
        if unit_symbol.starts_with(prefix_info.symbol) {
            let base_symbol = &unit_symbol[prefix_info.symbol.len()..];
            // Check if the remaining part is a valid base unit
            if let Some(base_unit_info) = lookup_base_unit(base_symbol) {
                return Some((base_unit_info.symbol, prefix_info.symbol));
            } else {
                // Only check the first unit in each dimension (the SI base unit)
                // This prevents applying prefixes to imperial units or other non-SI units
                for dimension in DIMENSIONS {
                    if let Some(first_unit) = dimension.units.first() {
                        if first_unit.symbols.contains(&base_symbol) {
                            // Use the first symbol from the unit's symbols array
                            if let Some(&unit_symbol) = first_unit.symbols.first() {
                                return Some((unit_symbol, prefix_info.symbol));
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Get unit dimensions for a unit symbol
/// Returns the dimension exponents as a tuple
pub fn get_unit_dimensions(unit_symbol: &str) -> Option<DimensionExponents> {
    // First try to find in the unified dimensions data
    if let Some((dimension, _)) = lookup_unit_literal(unit_symbol) {
        return Some(dimension.exponents);
    }
    
    // Try prefixed base units
    if let Some((base_symbol, _)) = is_prefixed_base_unit(unit_symbol) {
        if let Some(base_unit) = lookup_base_unit(base_symbol) {
            return Some(base_unit.dimension_exponents);
        }
    }
    
    None
}

/// Check if a unit symbol is a valid unit literal
pub fn is_valid_unit_literal(unit_symbol: &str) -> bool {
    lookup_unit_literal(unit_symbol).is_some()
}

/// Get all unit symbols from all dimensions
pub fn get_all_unit_symbols() -> Vec<&'static str> {
    let mut symbols = Vec::new();
    for dimension in DIMENSIONS {
        for unit in dimension.units {
            symbols.extend(unit.symbols.iter());
        }
    }
    symbols
}

/// Get units by dimension exponents
pub fn get_units_by_exponents(exponents: DimensionExponents) -> Vec<(&'static Dimension, &'static Unit)> {
    let mut result = Vec::new();
    for dimension in DIMENSIONS {
        if dimension.exponents == exponents {
            for unit in dimension.units {
                result.push((dimension, unit));
            }
        }
    }
    result
}

/// Convert dimension exponents to a unit expression string
/// This function takes dimension exponents and base unit symbols and generates a unit expression
pub fn dimension_exponents_to_unit_expression(
    exponents: DimensionExponents,
    base_units: &[(&str, &str)],
) -> String {
    let (mass, length, time, current, temp, amount, lum, angle) = exponents;
    let mut parts = Vec::new();
    
    // Add each dimension if it has a non-zero exponent
    if mass != 0 {
        parts.push(format!("{}^{}", base_units[0].1, mass));
    }
    if length != 0 {
        parts.push(format!("{}^{}", base_units[1].1, length));
    }
    if time != 0 {
        parts.push(format!("{}^{}", base_units[2].1, time));
    }
    if current != 0 {
        parts.push(format!("{}^{}", base_units[3].1, current));
    }
    if temp != 0 {
        parts.push(format!("{}^{}", base_units[4].1, temp));
    }
    if amount != 0 {
        parts.push(format!("{}^{}", base_units[5].1, amount));
    }
    if lum != 0 {
        parts.push(format!("{}^{}", base_units[6].1, lum));
    }
    if angle != 0 {
        parts.push(format!("{}^{}", base_units[7].1, angle));
    }
    
    if parts.is_empty() {
        "1".to_string()
    } else {
        parts.join(" * ")
    }
}

/// Convert scale type name to actual unit symbol
/// This function maps scale type names (like "Kilogram", "Millimeter") to their actual unit symbols
/// Uses a systematic, data-driven approach based on the existing data structures
pub fn scale_type_to_actual_unit_symbol(scale_type: &str) -> Option<String> {
    use crate::{SI_PREFIXES, BASE_UNITS};
    
    // Convert to lowercase for case-insensitive matching
    let scale_type_lower = scale_type.to_lowercase();
    
    // Try to find a matching prefix in the scale type name
    for prefix_info in SI_PREFIXES {
        if scale_type_lower.starts_with(prefix_info.long_name) {
            // Found a prefix, now find the base unit
            let base_unit_name = &scale_type_lower[prefix_info.long_name.len()..];
            
            // Look for a base unit that matches the remaining part
            for base_unit in BASE_UNITS {
                if base_unit.long_name == base_unit_name {
                    // Found both prefix and base unit, combine them
                    return Some(format!("{}{}", prefix_info.symbol, base_unit.symbol));
                }
            }
        }
    }
    
    // No prefix found, try to match directly with base unit names
    for base_unit in BASE_UNITS {
        if base_unit.long_name == scale_type_lower {
            return Some(base_unit.symbol.to_string());
        }
    }
    
    None
}

/// Convert scale type name to unit symbol
/// This function maps scale type names (like "Kilogram", "Millimeter") to their base unit symbols
pub fn scale_type_to_unit_symbol(scale_type: &str) -> Option<&'static str> {
    // Map the scale type names to their base unit symbols
    match scale_type {
        // Mass scales
        "Kilogram" | "Gram" | "Milligram" | "Microgram" | "Nanogram" | "Picogram" | "Femtogram"
        | "Attogram" | "Zeptogram" | "Yoctogram" | "Megagram" | "Gigagram" | "Teragram"
        | "Petagram" | "Exagram" | "Zettagram" | "Yottagram" => Some("g"),

        // Length scales
        "Meter" | "Millimeter" | "Micrometer" | "Nanometer" | "Picometer" | "Femtometer"
        | "Attometer" | "Zeptometer" | "Yoctometer" | "Kilometer" | "Megameter" | "Gigameter"
        | "Terameter" | "Petameter" | "Exameter" | "Zettameter" | "Yottameter" => Some("m"),

        // Time scales
        "Second" | "Millisecond" | "Microsecond" | "Nanosecond" | "Picosecond" | "Femtosecond"
        | "Attosecond" | "Zeptosecond" | "Yoctosecond" | "Kilosecond" | "Megasecond"
        | "Gigasecond" | "Terasecond" | "Petasecond" | "Exasecond" | "Zettasecond"
        | "Yottasecond" => Some("s"),

        // Current scales
        "Ampere" | "Milliampere" | "Microampere" | "Nanoampere" | "Picoampere" | "Femtoampere"
        | "Attoampere" | "Zeptoampere" | "Yoctoampere" | "Kiloampere" | "Megaampere"
        | "Gigaampere" | "Teraampere" | "Petaampere" | "Exaampere" | "Zettaampere"
        | "Yottaampere" => Some("A"),

        // Temperature scales
        "Kelvin" | "Millikelvin" | "Microkelvin" | "Nanokelvin" | "Picokelvin" | "Femtokelvin"
        | "Attokelvin" | "Zeptokelvin" | "Yoctokelvin" | "Kilokelvin" | "Megakelvin"
        | "Gigakelvin" | "Terakelvin" | "Petakelvin" | "Exakelvin" | "Zettakelvin"
        | "Yottakelvin" => Some("K"),

        // Amount scales
        "Mole" | "Millimole" | "Micromole" | "Nanomole" | "Picomole" | "Femtomole" | "Attomole"
        | "Zeptomole" | "Yoctomole" | "Kilomole" | "Megamole" | "Gigamole" | "Teramole"
        | "Petamole" | "Examole" | "Zettamole" | "Yottamole" => Some("mol"),

        // Luminosity scales
        "Candela" | "Millicandela" | "Microcandela" | "Nanocandela" | "Picocandela"
        | "Femtocandela" | "Attocandela" | "Zeptocandela" | "Yoctocandela" | "Kilocandela"
        | "Megacandela" | "Gigacandela" | "Teracandela" | "Petacandela" | "Exacandela"
        | "Zettacandela" | "Yottacandela" => Some("cd"),

        // Angle scales
        "Radian" | "Milliradian" | "Microradian" | "Nanoradian" | "Picoradian" | "Femtoradian"
        | "Attoradian" | "Zeptoradian" | "Yoctoradian" | "Kiloradian" | "Megaradian"
        | "Gigaradian" | "Teraradian" | "Petaradian" | "Exaradian" | "Zettaradian"
        | "Yottaradian" => Some("rad"),

        _ => None,
    }
}

/// Convert long unit name to short symbol
/// This is a simple mapping function for common unit names
pub fn convert_long_name_to_short(long_name: &str) -> Option<&'static str> {
    match long_name {
        "meter" => Some("m"),
        "gram" => Some("g"),
        "second" => Some("s"),
        "ampere" => Some("A"),
        "kelvin" => Some("K"),
        "mole" => Some("mol"),
        "candela" => Some("cd"),
        "radian" => Some("rad"),
        "newton" => Some("N"),
        "joule" => Some("J"),
        "watt" => Some("W"),
        "pascal" => Some("Pa"),
        "hertz" => Some("Hz"),
        "coulomb" => Some("C"),
        "volt" => Some("V"),
        "farad" => Some("F"),
        "ohm" => Some("Ω"),
        "siemens" => Some("S"),
        "henry" => Some("H"),
        "tesla" => Some("T"),
        "weber" => Some("Wb"),
        "lumen" => Some("lm"),
        "lux" => Some("lx"),
        _ => None,
    }
}

