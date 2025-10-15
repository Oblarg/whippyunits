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

use whippyunits_core::SiPrefix;

/// Get SI prefix based on scale using whippyunits-core as source of truth
/// Complete official SI prefix spectrum as defined by BIPM/CGPM
pub fn get_si_prefix(scale_p10: i16, long_name: bool) -> Option<&'static str> {
    SiPrefix::ALL
        .iter()
        .find(|prefix| prefix.factor_log10() == scale_p10)
        .map(|prefix: &SiPrefix| if long_name { prefix.name() } else { prefix.symbol() })
}
