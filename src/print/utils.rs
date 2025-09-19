/// Convert any integer to Unicode superscript notation
/// Returns empty string for unity exponent (1) unless show_unity is true
/// Returns "?" for unknown values (i16::MIN)
pub fn to_unicode_superscript(num: i16, show_unity: bool) -> String {
    if num == i16::MIN {
        return "ˀ".to_string();
    }
    
    if num == 1 && !show_unity {
        return String::new();
    }
    
    num.to_string()
        .replace('-', "⁻")
        .replace('0', "⁰")
        .replace('1', "¹")
        .replace('2', "²")
        .replace('3', "³")
        .replace('4', "⁴")
        .replace('5', "⁵")
        .replace('6', "⁶")
        .replace('7', "⁷")
        .replace('8', "⁸")
        .replace('9', "⁹")
}

/// Get SI prefix based on scale
/// Complete official SI prefix spectrum as defined by BIPM/CGPM
pub fn get_si_prefix(scale_p10: i16, long_name: bool) -> Option<&'static str> {
    match scale_p10 {
        // Small prefixes (negative powers of 10) - submultiples
        -30 => Some(if long_name { "quecto" } else { "q" }),    // 10⁻³⁰ (new 2022)
        -27 => Some(if long_name { "ronto" } else { "r" }),    // 10⁻²⁷ (new 2022)
        -24 => Some(if long_name { "yocto" } else { "y" }),    // 10⁻²⁴
        -21 => Some(if long_name { "zepto" } else { "z" }),    // 10⁻²¹
        -18 => Some(if long_name { "atto" } else { "a" }),     // 10⁻¹⁸
        -15 => Some(if long_name { "femto" } else { "f" }),    // 10⁻¹⁵
        -12 => Some(if long_name { "pico" } else { "p" }),     // 10⁻¹²
        -9 => Some(if long_name { "nano" } else { "n" }),      // 10⁻⁹
        -6 => Some(if long_name { "micro" } else { "μ" }),     // 10⁻⁶
        -3 => Some(if long_name { "milli" } else { "m" }),     // 10⁻³
        -2 => Some(if long_name { "centi" } else { "c" }),     // 10⁻²
        -1 => Some(if long_name { "deci" } else { "d" }),      // 10⁻¹
        
        // No prefix for base unit
        0 => None,
        
        // Large prefixes (positive powers of 10) - multiples
        1 => Some(if long_name { "deka" } else { "da" }),      // 10¹
        2 => Some(if long_name { "hecto" } else { "h" }),      // 10²
        3 => Some(if long_name { "kilo" } else { "k" }),       // 10³
        6 => Some(if long_name { "mega" } else { "M" }),       // 10⁶
        9 => Some(if long_name { "giga" } else { "G" }),       // 10⁹
        12 => Some(if long_name { "tera" } else { "T" }),      // 10¹²
        15 => Some(if long_name { "peta" } else { "P" }),      // 10¹⁵
        18 => Some(if long_name { "exa" } else { "E" }),       // 10¹⁸
        21 => Some(if long_name { "zetta" } else { "Z" }),     // 10²¹
        24 => Some(if long_name { "yotta" } else { "Y" }),     // 10²⁴
        27 => Some(if long_name { "ronna" } else { "R" }),     // 10²⁷ (new 2022)
        30 => Some(if long_name { "quetta" } else { "Q" }),    // 10³⁰ (new 2022)
        
        _ => None, // No standard SI prefix for this scale
    }
}
