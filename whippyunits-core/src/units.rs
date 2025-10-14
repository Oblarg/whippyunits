use crate::dimension_exponents::{self, DynDimensionExponents, TypeDimensionExponents};
use crate::scale_exponents::ScaleExponents;
use crate::prefix::SiPrefix;

/// A unit.
///
/// Each unit is assigned a storage unit. The storage unit is the unit
/// of a value stored with this unit. Storage units are always a well defined
/// multiple of an SI base unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unit<ExponentsType = DynDimensionExponents> {
    /// Name of the unit.
    pub name: &'static str,

    /// Symbols associated with the unit.
    pub symbols: &'static [&'static str],

    /// Base unit per storage unit.
    ///
    /// To convert from a value in the storage unit to a SI base unit value,
    /// multiply it by `scale`.
    ///
    /// For example, a scale of `1000` in dimension length would store
    /// the value as kilometers. If we have the value `123` stored then that
    /// means in meters it is `123 * 1000 = 123,000 meters`.
    pub scale: ScaleExponents,

    /// Storage unit per this unit.
    ///
    /// To convert from a value in this unit to a storage unit value,
    /// multiply it by `conversion_factor`.
    pub conversion_factor: f64,

    /// Dimension the unit belongs to.
    pub exponents: ExponentsType,
}

const IDENTITY: f64 = 1.0;

impl<ExponentsType> Unit<ExponentsType> {
    /// Check if the unit has a conversion factor.
    pub fn has_conversion(&self) -> bool {
        self.conversion_factor != IDENTITY
    }
}

impl<
    const MASS_EXP: i16,
    const LENGTH_EXP: i16,
    const TIME_EXP: i16,
    const CURRENT_EXP: i16,
    const TEMPERATURE_EXP: i16,
    const AMOUNT_EXP: i16,
    const LUMINOSITY_EXP: i16,
    const ANGLE_EXP: i16,
>
    Unit<
        crate::dimension_exponents!([
            MASS_EXP,
            LENGTH_EXP,
            TIME_EXP,
            CURRENT_EXP,
            TEMPERATURE_EXP,
            AMOUNT_EXP,
            LUMINOSITY_EXP,
            ANGLE_EXP,
        ]),
    >
{
    pub const fn erase(&self) -> Unit {
        Unit {
            name: self.name,
            symbols: self.symbols,
            scale: self.scale,
            conversion_factor: self.conversion_factor,
            exponents: self.exponents.value_const(),
        }
    }
}

/// Lists of units.
impl Unit {
    pub const BASES: [Self; 8] = [
        Unit::GRAM.erase(),
        Unit::METER.erase(),
        Unit::SECOND.erase(),
        Unit::AMPERE.erase(),
        Unit::KELVIN.erase(),
        Unit::MOLE.erase(),
        Unit::CANDELA.erase(),
        Unit::RADIAN.erase(),
    ];

    /// Convert multiple names to their base unit.
    ///
    /// This function takes a scale type name (like "Kilogram", "Millimeter") and returns
    /// the corresponding base unit (like "g", "m").
    pub fn multiple_to_base_unit(
        multiple: &str,
    ) -> Option<(&'static Self, Option<&'static SiPrefix>)> {
        let (prefix, name) = SiPrefix::strip_any_prefix_name(multiple)
            .map(|(prefix, name)| (Some(prefix), name))
            .unwrap_or((None, multiple));

        Self::BASES.iter().find_map(|unit| {
            if unit.name == name {
                Some((unit, prefix))
            } else {
                None
            }
        })
    }

    /// Find base unit by symbol.
    pub fn find_base(symbol: &str) -> Option<&'static Self> {
        Self::BASES
            .iter()
            .find(|unit| unit.symbols.contains(&symbol))
    }
}

/// Mass
impl Unit<crate::dimension_exponents!([1, 0, 0, 0, 0, 0, 0, 0])> {
    pub const GRAM: Self = Self {
        name: "gram",
        symbols: &["g"],
        scale: ScaleExponents::_10(-3),
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };

    pub const OUNCE: Self = Self {
        name: "ounce",
        symbols: &["oz"],
        scale: ScaleExponents::_10(-2),
        conversion_factor: 2.8349523125,
        exponents: TypeDimensionExponents::new(),
    };

    pub const POUND: Self = Self {
        name: "pound",
        symbols: &["lb"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: 0.45359237,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Length
impl Unit<crate::dimension_exponents!([0, 1, 0, 0, 0, 0, 0, 0])> {
    pub const METER: Self = Self {
        name: "meter",
        symbols: &["m"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };

    pub const INCH: Self = Self {
        name: "inch",
        symbols: &["in"],
        scale: ScaleExponents::_10(-2),
        conversion_factor: 0.0254,
        exponents: TypeDimensionExponents::new(),
    };

    pub const FOOT: Self = Self {
        name: "foot",
        symbols: &["ft"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: 0.3048,
        exponents: TypeDimensionExponents::new(),
    };

    pub const YARD: Self = Self {
        name: "yard",
        symbols: &["yd"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: 0.9144,
        exponents: TypeDimensionExponents::new(),
    };

    pub const MILE: Self = Self {
        name: "mile",
        symbols: &["mi"],
        scale: ScaleExponents::_10(3),
        conversion_factor: 1.609344,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Time
impl Unit<crate::dimension_exponents!([0, 0, 1, 0, 0, 0, 0, 0])> {
    pub const SECOND: Self = Self {
        name: "second",
        symbols: &["s"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };

    pub const MINUTE: Self = Self {
        name: "minute",
        symbols: &["min"],
        scale: ScaleExponents::_10(1).mul(ScaleExponents::_6(1)),
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };

    pub const HOUR: Self = Self {
        name: "hour",
        symbols: &["h", "hr"],
        scale: ScaleExponents::_10(2).mul(ScaleExponents::_6(2)),
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };

    pub const DAY: Self = Self {
        name: "day",
        symbols: &["d"],
        scale: ScaleExponents::_10(2)
            .mul(ScaleExponents::_6(3))
            .mul(ScaleExponents::_2(2)),
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Current
impl Unit<crate::dimension_exponents!([0, 0, 0, 1, 0, 0, 0, 0])> {
    pub const AMPERE: Self = Self {
        name: "ampere",
        symbols: &["A"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Temperature
impl Unit<crate::dimension_exponents!([0, 0, 0, 0, 1, 0, 0, 0])> {
    pub const KELVIN: Self = Self {
        name: "kelvin",
        symbols: &["K"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Amount
impl Unit<crate::dimension_exponents!([0, 0, 0, 0, 0, 1, 0, 0])> {
    pub const MOLE: Self = Self {
        name: "mole",
        symbols: &["mol"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Angle
impl Unit<crate::dimension_exponents!([0, 0, 0, 0, 0, 0, 0, 1])> {
    pub const RADIAN: Self = Self {
        name: "radian",
        symbols: &["rad"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };

    pub const DEGREE: Self = Self {
        name: "degree",
        symbols: &["deg"],
        scale: ScaleExponents([-2, -2, -1, 1]),
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };

    pub const GRADIAN: Self = Self {
        name: "gradian",
        symbols: &["grad", "gon"],
        scale: ScaleExponents([-3, -1, -1, 1]),
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };

    pub const TURN: Self = Self {
        name: "turn",
        symbols: &["rot", "turn"],
        scale: ScaleExponents([1, 0, 0, 1]),
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };

    pub const ARCMINUTE: Self = Self {
        name: "arcminute",
        symbols: &["arcmin"],
        scale: ScaleExponents([-4, -2, -2, 1]),
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };

    pub const ARCSECOND: Self = Self {
        name: "arcsecond",
        symbols: &["arcsec"],
        scale: ScaleExponents([-6, -2, -2, 1]),
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Frequency
impl Unit<crate::dimension_exponents!([0, 0, -1, 0, 0, 0, 0, 0])> {
    pub const HERTZ: Self = Self {
        name: "hertz",
        symbols: &["Hz"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Force
impl Unit<crate::dimension_exponents!([1, 1, -2, 0, 0, 0, 0, 0])> {
    pub const NEWTON: Self = Self {
        name: "newton",
        symbols: &["N"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Energy
impl Unit<crate::dimension_exponents!([1, 2, -2, 0, 0, 0, 0, 0])> {
    pub const JOULE: Self = Self {
        name: "joule",
        symbols: &["J"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Power
impl Unit<crate::dimension_exponents!([1, 2, -3, 0, 0, 0, 0, 0])> {
    pub const WATT: Self = Self {
        name: "watt",
        symbols: &["W"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Pressure
impl Unit<crate::dimension_exponents!([1, -1, -2, 0, 0, 0, 0, 0])> {
    pub const PASCAL: Self = Self {
        name: "pascal",
        symbols: &["Pa"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Electric charge
impl Unit<crate::dimension_exponents!([0, 0, 1, 1, 0, 0, 0, 0])> {
    pub const COULOMB: Self = Self {
        name: "coulomb",
        symbols: &["C"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Electric potential
impl Unit<crate::dimension_exponents!([1, 2, -3, -1, 0, 0, 0, 0])> {
    pub const VOLT: Self = Self {
        name: "volt",
        symbols: &["V"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Capacitance
impl Unit<crate::dimension_exponents!([-1, -2, 4, 2, 0, 0, 0, 0])> {
    pub const FARAD: Self = Self {
        name: "farad",
        symbols: &["F"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

// Eletric resistence
impl Unit<crate::dimension_exponents!([1, 2, -3, -2, 0, 0, 0, 0])> {
    pub const OHM: Self = Self {
        name: "ohm",
        symbols: &["Ω"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Electric conductance
impl Unit<crate::dimension_exponents!([-1, -2, 3, 2, 0, 0, 0, 0])> {
    pub const SIEMENS: Self = Self {
        name: "siemens",
        symbols: &["S"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Inductance
impl Unit<crate::dimension_exponents!([1, 2, -2, -2, 0, 0, 0, 0])> {
    pub const HENRY: Self = Self {
        name: "henry",
        symbols: &["H"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Magnetic field
impl Unit<crate::dimension_exponents!([1, 0, -2, -1, 0, 0, 0, 0])> {
    pub const TESLA: Self = Self {
        name: "tesla",
        symbols: &["T"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Magnetic flex
impl Unit<crate::dimension_exponents!([1, 2, -2, -1, 0, 0, 0, 0])> {
    pub const WEBER: Self = Self {
        name: "weber",
        symbols: &["Wb"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Illuminance
impl Unit<crate::dimension_exponents!([0, -2, 0, 0, 0, 0, 1, 0])> {
    pub const LUX: Self = Self {
        name: "lux",
        symbols: &["lx"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Luminous intensity
impl Unit<crate::dimension_exponents!([0, 0, 0, 0, 0, 0, 1, 0])> {
    pub const CANDELA: Self = Self {
        name: "candela",
        symbols: &["cd"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Kinematic viscosity
impl Unit<crate::dimension_exponents!([0, 2, -1, 0, 0, 0, 0, 0])> {
    pub const STOKES: Self = Self {
        name: "stokes",
        symbols: &["St"],
        scale: ScaleExponents::_10(-4),
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

/// Dimensionless
impl Unit<crate::dimension_exponents!([0, 0, 0, 0, 0, 0, 0, 0])> {
    pub const DIMENSIONLESS: Self = Self {
        name: "dimensionless",
        symbols: &[],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        exponents: TypeDimensionExponents::new(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_prefix_and_unit_from_multiple_string() {
        assert_eq!(
            Unit::multiple_to_base_unit("kilometer").unwrap(),
            (&Unit::METER.erase(), Some(&SiPrefix::KILO),)
        );

        assert_eq!(
            Unit::multiple_to_base_unit("millisecond").unwrap(),
            (&Unit::SECOND.erase(), Some(&SiPrefix::MILLI),)
        );

        assert_eq!(
            Unit::multiple_to_base_unit("gram").unwrap(),
            (&Unit::GRAM.erase(), None,)
        );

        assert_eq!(Unit::multiple_to_base_unit("abc"), None,);
    }
}
