/// SI prefix information.
///
/// These SI prefixes are defined by
/// [BIPM/CGPM](https://www.bipm.org/en/measurement-units/si-prefixes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SiPrefix {
    // We make these private since the constants define all instances.
    factor_log10: i16,
    name: &'static str,
    symbol: &'static str,
}

impl SiPrefix {
    /// The multiplying factor exponent.
    ///
    /// For example, a multiplying factor of `10^12` would make this field `12`.
    pub const fn factor_log10(&self) -> i16 {
        self.factor_log10
    }

    /// Name of the prefix.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Prefix symbol.
    ///
    /// A symbol is not always a single character. Deca's symbol `da` is the exception.
    pub const fn symbol(&self) -> &'static str {
        self.symbol
    }

    /// List of all SI prefix definitions.
    pub const ALL: &[SiPrefix] = &Self::ALL_ARRAY;

    /// This is seperate from `ALL` so we can make sure all the prefixes are there.
    /// If we included the array size in the public constant then it would be a
    /// breaking change to add prefixes.
    const ALL_ARRAY: [SiPrefix; 24] = [
        // Small prefixes (negative powers of 10) - submultiple
        Self::QUECTO,
        Self::RONTO,
        Self::YOCTO,
        Self::ZEPTO,
        Self::ATTO,
        Self::FEMTO,
        Self::PICO,
        Self::NANO,
        Self::MICRO,
        Self::MILLI,
        Self::CENTI,
        Self::DECI,
        // Large prefixes (positive powers of 10) - multiple
        Self::DECA,
        Self::HECTO,
        Self::KILO,
        Self::MEGA,
        Self::GIGA,
        Self::TERA,
        Self::PETA,
        Self::EXA,
        Self::ZETTA,
        Self::YOTTA,
        Self::RONNA,
        Self::QUETTA,
    ];

    /// Look up SI prefix by symbol.
    pub fn from_symbol(symbol: &str) -> Option<&'static Self> {
        Self::ALL.iter().find(|prefix| prefix.symbol == symbol)
    }

    /// Strip the prefix name from a string.
    pub fn strip_prefix_name<'r>(&self, s: &'r str) -> Option<&'r str> {
        // Bail out if the string isnt even long enough to have the prefix.
        if s.len() < self.name.len() {
            return None;
        }

        // Check the first character. We know all prefixes have ASCII names.
        // We allow the first character to be uppercase or lowercase.
        let first_char = self.name.as_bytes()[0];
        if !s.starts_with([first_char as char, first_char.to_ascii_uppercase() as char]) {
            return None;
        }

        // We then check the rest of the prefix name to be lowercase.
        if s.as_bytes()[1..self.name.len()] == self.name.as_bytes()[1..] {
            // Since we now know that the multiple starts with a prefix name,
            // we know that we can index directly after without a panic.
            Some(&s[self.name.len()..])
        } else {
            None
        }
    }

    /// Strip the prefix symbol from a string.
    pub fn strip_prefix_symbol<'r>(&self, s: &'r str) -> Option<&'r str> {
        // Bail out if the string isnt even long enough to have the prefix.
        if s.len() < self.symbol.len() {
            return None;
        }

        // We then check the rest of the prefix symbol to be lowercase.
        if &s.as_bytes()[..self.symbol.len()] == self.symbol.as_bytes() {
            // Since we now know that the multiple starts with a prefix symbol,
            // we know that we can index directly after without a panic.
            Some(&s[self.symbol.len()..])
        } else {
            None
        }
    }

    /// Strip any prefix name from a string.
    ///
    /// The stripped prefix is returned along with the base unit string.
    pub fn strip_any_prefix_name(s: &str) -> Option<(&'static Self, &str)> {
        Self::ALL
            .iter()
            .find_map(|prefix| prefix.strip_prefix_name(s).map(|s| (prefix, s)))
    }

    /// Strip any prefix symbol from a string.
    ///
    /// The stripped prefix is returned along with the base unit string.
    pub fn strip_any_prefix_symbol(s: &str) -> Option<(&'static Self, &str)> {
        Self::ALL.iter().find_map(|prefix| {
            prefix.strip_prefix_symbol(s).and_then(|base| {
                // Only return a prefix if there's actually a base unit after the prefix
                if !base.is_empty() {
                    Some((prefix, base))
                } else {
                    None
                }
            })
        })
    }
}

impl SiPrefix {
    /// 10⁻³⁰
    pub const QUECTO: Self = Self {
        symbol: "q",
        factor_log10: -30,
        name: "quecto",
    };

    /// 10⁻²⁷
    pub const RONTO: Self = Self {
        symbol: "r",
        factor_log10: -27,
        name: "ronto",
    };

    /// 10⁻²⁴
    pub const YOCTO: Self = Self {
        symbol: "y",
        factor_log10: -24,
        name: "yocto",
    };

    /// 10⁻²¹
    pub const ZEPTO: Self = Self {
        symbol: "z",
        factor_log10: -21,
        name: "zepto",
    };

    /// 10⁻¹⁸
    pub const ATTO: Self = Self {
        symbol: "a",
        factor_log10: -18,
        name: "atto",
    };

    /// 10⁻¹⁵
    pub const FEMTO: Self = Self {
        symbol: "f",
        factor_log10: -15,
        name: "femto",
    };

    /// 10⁻¹²
    pub const PICO: Self = Self {
        symbol: "p",
        factor_log10: -12,
        name: "pico",
    };

    /// 10⁻⁹
    pub const NANO: Self = Self {
        symbol: "n",
        factor_log10: -9,
        name: "nano",
    };

    /// 10⁻⁶
    pub const MICRO: Self = Self {
        symbol: "µ",
        factor_log10: -6,
        name: "micro",
    };

    /// 10⁻³
    pub const MILLI: Self = Self {
        symbol: "m",
        factor_log10: -3,
        name: "milli",
    };

    /// 10⁻²
    pub const CENTI: Self = Self {
        symbol: "c",
        factor_log10: -2,
        name: "centi",
    };

    /// 10⁻¹
    pub const DECI: Self = Self {
        symbol: "d",
        factor_log10: -1,
        name: "deci",
    };

    /// 10¹
    pub const DECA: Self = Self {
        symbol: "da",
        factor_log10: 1,
        name: "deca",
    };

    /// 10²
    pub const HECTO: Self = Self {
        symbol: "h",
        factor_log10: 2,
        name: "hecto",
    };

    /// 10³
    pub const KILO: Self = Self {
        symbol: "k",
        factor_log10: 3,
        name: "kilo",
    };

    /// 10⁶
    pub const MEGA: Self = Self {
        symbol: "M",
        factor_log10: 6,
        name: "mega",
    };

    /// 10⁹
    pub const GIGA: Self = Self {
        symbol: "G",
        factor_log10: 9,
        name: "giga",
    };

    /// 10¹²
    pub const TERA: Self = Self {
        symbol: "T",
        factor_log10: 12,
        name: "tera",
    };

    /// 10¹⁵
    pub const PETA: Self = Self {
        symbol: "P",
        factor_log10: 15,
        name: "peta",
    };

    /// 10¹⁸
    pub const EXA: Self = Self {
        symbol: "E",
        factor_log10: 18,
        name: "exa",
    };

    /// 10²¹
    pub const ZETTA: Self = Self {
        symbol: "Z",
        factor_log10: 21,
        name: "zetta",
    };

    /// 10²⁴
    pub const YOTTA: Self = Self {
        symbol: "Y",
        factor_log10: 24,
        name: "yotta",
    };

    /// 10²⁷
    pub const RONNA: Self = Self {
        symbol: "R",
        factor_log10: 27,
        name: "ronna",
    };

    /// 10³⁰
    pub const QUETTA: Self = Self {
        symbol: "Q",
        factor_log10: 30,
        name: "quetta",
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_prefix_from_symbol() {
        for prefix in SiPrefix::ALL {
            assert_eq!(SiPrefix::from_symbol(prefix.symbol).unwrap(), prefix);
        }

        assert_eq!(SiPrefix::from_symbol("?"), None);
    }

    #[test]
    fn can_strip_prefix_name_from_str() {
        assert_eq!(
            SiPrefix::MILLI.strip_prefix_name("millimeter"),
            Some("meter")
        );
        assert_eq!(
            SiPrefix::MILLI.strip_prefix_name("Millimeter"),
            Some("meter")
        );
        assert_eq!(SiPrefix::MILLI.strip_prefix_name("abc"), None);
    }

    #[test]
    fn can_strip_any_prefix_name_from_str() {
        assert_eq!(
            SiPrefix::strip_any_prefix_name("megameter"),
            Some((&SiPrefix::MEGA, "meter"))
        );
        assert_eq!(
            SiPrefix::strip_any_prefix_name("Megameter"),
            Some((&SiPrefix::MEGA, "meter"))
        );
        assert_eq!(SiPrefix::strip_any_prefix_name("abc"), None);
    }

    #[test]
    fn can_strip_prefix_symbol_from_str() {
        assert_eq!(SiPrefix::MILLI.strip_prefix_symbol("mm"), Some("m"));
        assert_eq!(SiPrefix::MILLI.strip_prefix_name("?"), None);
    }

    #[test]
    fn can_strip_any_prefix_symbol_from_str() {
        assert_eq!(
            SiPrefix::strip_any_prefix_symbol("Mm"),
            Some((&SiPrefix::MEGA, "m"))
        );
        assert_eq!(SiPrefix::strip_any_prefix_name("?"), None);
    }
}
