// Re-export the function from whippyunits-core for backward compatibility
pub use whippyunits_core::to_unicode_superscript;

use whippyunits_core::SiPrefix;

/// Get SI prefix based on scale using whippyunits-core as source of truth
/// Complete official SI prefix spectrum as defined by BIPM/CGPM
pub fn get_si_prefix(scale_p10: i16, long_name: bool) -> Option<&'static str> {
    SiPrefix::ALL
        .iter()
        .find(|prefix| prefix.factor_log10() == scale_p10)
        .map(|prefix: &SiPrefix| {
            if long_name {
                prefix.name()
            } else {
                prefix.symbol()
            }
        })
}
