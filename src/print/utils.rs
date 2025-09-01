/// Convert any integer to Unicode superscript notation
/// Returns empty string for unity exponent (1) unless show_unity is true
/// Returns "?" for unknown values (isize::MIN)
pub fn to_unicode_superscript(num: isize, show_unity: bool) -> String {
    if num == isize::MIN {
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
pub fn get_si_prefix(scale_p10: isize, long_name: bool) -> Option<&'static str> {
    match scale_p10 {
        -15 => Some(if long_name { "femto" } else { "f" }),
        -12 => Some(if long_name { "pico" } else { "p" }),
        -9 => Some(if long_name { "nano" } else { "n" }),
        -6 => Some(if long_name { "micro" } else { "μ" }),
        -3 => Some(if long_name { "milli" } else { "m" }),
        -2 => Some(if long_name { "centi" } else { "c" }),
        -1 => Some(if long_name { "deci" } else { "d" }),
        0 => None, // No prefix for base unit
        1 => Some(if long_name { "deca" } else { "da" }),
        2 => Some(if long_name { "hecto" } else { "h" }),
        3 => Some(if long_name { "kilo" } else { "k" }),
        6 => Some(if long_name { "mega" } else { "M" }),
        9 => Some(if long_name { "giga" } else { "G" }),
        12 => Some(if long_name { "tera" } else { "T" }),
        15 => Some(if long_name { "peta" } else { "P" }),
        _ => None, // No standard SI prefix for this scale
    }
}
