use crate::dimension_exponents::{DynDimensionExponents, TypeDimensionExponents};
use crate::prefix::SiPrefix;
use crate::scale_exponents::ScaleExponents;

/// Unit system classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum System {
    /// Metric system (SI and derived units)
    Metric,
    /// Imperial system (US customary units)
    Imperial,
    /// Astronomical system (For extremely )
    Astronomical,
}

impl System {
    /// Get the string identifier for this system.
    pub const fn as_str(&self) -> &'static str {
        match self {
            System::Metric => "Metric",
            System::Imperial => "Imperial",
            System::Astronomical => "Astronomical"
        }
    }
}

/// A unit.
///
/// Each unit is assigned a "storage unit". The storage unit is the unit
/// of a value stored with this unit. Storage units are always a well defined
/// multiple of an SI base unit.
/// 
/// The logarithmic scale encoding in the type system uses only powers of
/// 2, 3, 5, and pi.  This means that the storage unit must be a multiple of
/// an SI base unit and a product of powers of 2, 3, 5, and pi.  For example,
/// 
/// - "kilometer" has a scale factor of 10^3 = 2^3 * 5^3
/// - "degree" has a scale factor of π/180 = 2^-2 * 3^-2 * 5^-1 * pi^1
/// 
/// Units that differ from identity in their `conversion_factor` are "non-storage"
/// units.  Non-storage units are not stored in their native scale; upon declaration
/// they are converted to their "nearest neighbor" power-of-10 multiple of a SI
/// base unit.  For example
/// 
/// - "inch" is multiplied by 2.54 and stored as "centimeters"
/// - "yard" is multiplied by 0.9144 and stored as "meters"
/// - "mile" is multiplied by 1.609344 and stored as "kilometers"
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unit<ExponentsType = DynDimensionExponents> {
    /// Name of the unit.
    pub name: &'static str,

    /// Symbols associated with the unit.
    /// 
    /// Symbols are also used for lookup, so they must be unique within
    /// the unit system.
    /// 
    /// Because SI has a systematic prefixing semantics, symbols must be kept
    /// from colliding not just with the SI base symbols, but with any legal
    /// prefixing thereof.  "SI base symbols" are defined as the first symbol
    /// of the first unit in each dimension, by declaration order.
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
    /// Difference from identity canonically identifies a unit as a
    /// "non-storage" unit.  Non-storage units are not stored in their
    /// native scale, as arbitrary float scaling factors are not part 
    /// of the logarithmic scale encoding in the type system.
    ///
    /// To convert from a value in this unit to a storage unit value,
    /// multiply it by `conversion_factor`.
    /// 
    /// For example, the "inch" unit has a `conversion_factor` of `2.54`, 
    /// which means that a value of `1` in inches is stored as `2.54` 
    /// (accordingly, the `scale` is `10^-2`).
    /// 
    /// Non-storage units are always given a storage scale of their
    /// "nearest neighbor" power-of-10 multiple of a SI base unit.
    /// For example,
    /// 
    /// - "inch" is multiplied by 2.54 and stored as "centimeters"
    /// - "yard" is multiplied by 0.9144 and stored as "meters"
    /// - "mile" is multiplied by 1.609344 and stored as "kilometers"
    pub conversion_factor: f64,

    /// The "zero point offset" of this unit's measurement scale from
    /// the numerical zero of the storage unit.
    /// 
    /// To convert from a value in the unit to the storage unit, add 
    /// the affine offset to the value.
    /// 
    /// For example, the "celsius" unit has an affine offset of `273.15`,
    /// which means that a value of `0` in celsius is stored as `273.15` 
    /// in kelvin.
    pub affine_offset: f64,

    /// Dimensional exponent vector of the [dimension](crate::dimension_exponents::DimensionBasis) 
    /// this unit belongs to.
    pub exponents: ExponentsType,

    /// Which "unit system" this unit belongs to.  This determines the name
    /// of the declarator trait in which this unit's nominal declarators will
    /// live (e.g. "ImperialLength" or "MetricMass").
    pub system: System,
}

const IDENTITY: f64 = 1.0;
const NONE: f64 = 0.0;

impl<ExponentsType> Unit<ExponentsType> {
    /// Check if the unit has a conversion factor.
    pub fn has_conversion(&self) -> bool {
        self.conversion_factor != IDENTITY
    }

    /// Check if the unit has an affine offset.
    pub fn has_affine_offset(&self) -> bool {
        self.affine_offset != NONE
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
            affine_offset: self.affine_offset,
            exponents: self.exponents.value_const(),
            system: self.system,
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
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const GRAIN: Self = Self {
        name: "grain",
        symbols: &["gr"],
        scale: ScaleExponents::_10(-4),
        conversion_factor: 0.6479891,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const CARAT: Self = Self {
        name: "carat",
        symbols: &["ct"],
        scale: ScaleExponents::_10(-4).mul(ScaleExponents::_2(1)),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const OUNCE: Self = Self {
        name: "ounce",
        symbols: &["oz"],
        scale: ScaleExponents::_10(-2),
        conversion_factor: 2.8349523125,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const TROY_OUNCE: Self = Self {
        name: "troy_ounce",
        symbols: &["ozt"],
        scale: ScaleExponents::_10(-2),
        conversion_factor: 3.11034768,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };
    pub const TROY_POUND: Self = Self {
        name: "troy_pound",
        symbols: &["lbt"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: 0.3732417216,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };
    
    pub const POUND: Self = Self {
        name: "pound",
        symbols: &["lb"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: 0.45359237,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const STONE: Self = Self {
        name: "stone",
        symbols: &["st"],
        scale: ScaleExponents::_10(1),
        conversion_factor: 0.635029318,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const SLUG: Self = Self {
        name: "slug",
        symbols: &["slg"],
        scale: ScaleExponents::_10(1),
        conversion_factor: 1.4593902937206365,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const TON: Self = Self {
        name: "ton",
        symbols: &["t"],
        scale: ScaleExponents::_10(3),
        conversion_factor: 1.0160469088,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };
    // To add: Earth mass, Jupiter mass, Sol mass
}

/// Length
impl Unit<crate::dimension_exponents!([0, 1, 0, 0, 0, 0, 0, 0])> {
    pub const METER: Self = Self {
        name: "meter",
        symbols: &["m"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const INCH: Self = Self {
        name: "inch",
        symbols: &["in"],
        scale: ScaleExponents::_10(-2),
        conversion_factor: 2.54,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const FOOT: Self = Self {
        name: "foot",
        symbols: &["ft"],
        scale: ScaleExponents::_10(-1),
        conversion_factor: 3.048,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const YARD: Self = Self {
        name: "yard",
        symbols: &["yd"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: 0.9144,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const FATHOM: Self = Self {
        name: "fathom",
        symbols: &["ftm"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: 1.8288,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const FURLONG: Self = Self {
        name: "furlong",
        symbols: &["fur"],
        scale: ScaleExponents::_10(2),
        conversion_factor: 2.01168,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const MILE: Self = Self {
        name: "mile",
        symbols: &["mi"],
        scale: ScaleExponents::_10(3),
        conversion_factor: 1.609344,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const NAUTICAL_MILE: Self = Self {
        name: "nautical_mile",
        symbols: &["nmi"],
        scale: ScaleExponents::_10(3),
        conversion_factor: 1.852,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const ASTRONOMICAL_UNIT: Self = Self {
        name: "astronomical_unit",
        symbols: &["AU"],
        scale: ScaleExponents::_10(11),
        conversion_factor: 1.495978707,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Astronomical,
    };

    pub const LIGHT_YEAR: Self = Self {
        name: "light_year",
        symbols: &["ly"],
        scale: ScaleExponents::_10(16),
        conversion_factor: 0.94607304725808,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Astronomical,
    };

    pub const PARSEC: Self = Self {
        name: "parsec",
        symbols: &["pc"],
        scale: ScaleExponents::_10(16),
        conversion_factor: 3.08567758128,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Astronomical,
    };
}

/// Time
impl Unit<crate::dimension_exponents!([0, 0, 1, 0, 0, 0, 0, 0])> {
    pub const SECOND: Self = Self {
        name: "second",
        symbols: &["s"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const MINUTE: Self = Self {
        name: "minute",
        symbols: &["min"],
        scale: ScaleExponents::_10(1).mul(ScaleExponents::_6(1)),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const HOUR: Self = Self {
        name: "hour",
        symbols: &["h", "hr"],
        scale: ScaleExponents::_10(2).mul(ScaleExponents::_6(2)),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const DAY: Self = Self {
        name: "day",
        symbols: &["d"],
        scale: ScaleExponents::_10(2)
            .mul(ScaleExponents::_6(3))
            .mul(ScaleExponents::_2(2)),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const WEEK: Self = Self {
        name: "week",
        symbols: &["wk"],
        scale: ScaleExponents::_10(3)
        .mul(ScaleExponents::_6(3))
        .mul(ScaleExponents::_2(2)),
        conversion_factor: 0.7,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const MONTH: Self = Self { // 30 days
        name: "month",
        symbols: &["mo"],
        scale: ScaleExponents::_10(3)
        .mul(ScaleExponents::_6(4))
        .mul(ScaleExponents::_2(1)),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const YEAR: Self = Self { // solar year, not calendar year
        name: "year",
        symbols: &["yr"],
        scale: ScaleExponents::_10(7),
        conversion_factor: 3.1556926,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Current
impl Unit<crate::dimension_exponents!([0, 0, 0, 1, 0, 0, 0, 0])> {
    pub const AMPERE: Self = Self {
        name: "ampere",
        symbols: &["A"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Temperature
impl Unit<crate::dimension_exponents!([0, 0, 0, 0, 1, 0, 0, 0])> {
    pub const KELVIN: Self = Self {
        name: "kelvin",
        symbols: &["K"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const CELSIUS: Self = Self {
        name: "celsius",
        symbols: &["C"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: 273.15,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const RANKINE: Self = Self {
        name: "rankine",
        symbols: &["R"],
        scale: ScaleExponents([0, -2, 1, 0]),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const FAHRENHEIT: Self = Self {
        name: "fahrenheit",
        symbols: &["degF"],
        scale: ScaleExponents([0, -2, 1, 0]),
        conversion_factor: IDENTITY,
        affine_offset: 459.7,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };
}

/// Amount
impl Unit<crate::dimension_exponents!([0, 0, 0, 0, 0, 1, 0, 0])> {
    pub const MOLE: Self = Self {
        name: "mole",
        symbols: &["mol"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Angle
impl Unit<crate::dimension_exponents!([0, 0, 0, 0, 0, 0, 0, 1])> {
    pub const RADIAN: Self = Self {
        name: "radian",
        symbols: &["rad"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const DEGREE: Self = Self {
        name: "degree",
        symbols: &["deg"],
        scale: ScaleExponents([-2, -2, -1, 1]),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const GRADIAN: Self = Self {
        name: "gradian",
        symbols: &["grad"],
        scale: ScaleExponents([-3, -1, -1, 1]),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const TURN: Self = Self {
        name: "turn",
        symbols: &["rot", "turn"],
        scale: ScaleExponents([1, 0, 0, 1]),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const ARCMINUTE: Self = Self {
        name: "arcminute",
        symbols: &["arcmin"],
        scale: ScaleExponents([-4, -2, -2, 1]),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const ARCSECOND: Self = Self {
        name: "arcsecond",
        symbols: &["arcsec"],
        scale: ScaleExponents([-6, -2, -2, 1]),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Frequency
impl Unit<crate::dimension_exponents!([0, 0, -1, 0, 0, 0, 0, 0])> {
    pub const HERTZ: Self = Self {
        name: "hertz",
        symbols: &["Hz"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Force
impl Unit<crate::dimension_exponents!([1, 1, -2, 0, 0, 0, 0, 0])> {
    pub const NEWTON: Self = Self {
        name: "newton",
        symbols: &["N"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Energy and work
impl Unit<crate::dimension_exponents!([1, 2, -2, 0, 0, 0, 0, 0])> {
    pub const JOULE: Self = Self {
        name: "joule",
        symbols: &["J"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const NEWTON_METER: Self = Self {
        name: "newton_meter",
        symbols: &["Nm"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const ELECTRONVOLT: Self = Self {
        name: "electron_volt",
        symbols: &["eV"],
        scale: ScaleExponents::_10(-19),
        conversion_factor: 1.602176634,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const ERG: Self = Self {
        name: "erg",
        symbols: &["erg"],
        scale: ScaleExponents::_10(-7),
        conversion_factor: 1.0,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const CALORIE: Self = Self {
        name: "calorie",
        symbols: &["cal"],
        scale: ScaleExponents::_10(1),
        conversion_factor: 0.4184,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const FOOT_POUND: Self = Self {
        name: "foot_pound",
        symbols: &["ft_lb"],
        scale: ScaleExponents::_10( 1),
        conversion_factor: 1.3558179483314004,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const KILOWATT_HOUR: Self = Self {
        name: "kilowatt_hour",
        symbols: &["kWh"],
        scale: ScaleExponents::_10(5).mul(ScaleExponents::_6(2)),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const THERM: Self = Self {
        name: "therm",
        symbols: &["thm"],
        scale: ScaleExponents::_10(8),
        conversion_factor: 1.05505585262,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };
}

/// Power
impl Unit<crate::dimension_exponents!([1, 2, -3, 0, 0, 0, 0, 0])> {
    pub const WATT: Self = Self {
        name: "watt",
        symbols: &["W"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const HORSEPOWER: Self = Self {
        name: "horsepower",
        symbols: &["hp"],
        scale: ScaleExponents::_10(3),
        conversion_factor: 0.7456998715822702,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };
}

/// Pressure
impl Unit<crate::dimension_exponents!([1, -1, -2, 0, 0, 0, 0, 0])> {
    pub const PASCAL: Self = Self {
        name: "pascal",
        symbols: &["Pa"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const TORR: Self = Self {
        name: "torr",
        symbols: &["Torr"],
        scale: ScaleExponents::_10(2),
        conversion_factor: 1.3332236842105263,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const PSI: Self = Self {
        name: "psi",
        symbols: &["psi"],
        scale: ScaleExponents::_10(4),
        conversion_factor: 0.6894757293168361,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const BAR: Self = Self {
        name: "bar",
        symbols: &["bar"],
        scale: ScaleExponents::_10(5),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const ATMOSPHERE: Self = Self {
        name: "atmosphere",
        symbols: &["atm"],
        scale: ScaleExponents::_10(5),
        conversion_factor: 1.01325,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Electric charge
impl Unit<crate::dimension_exponents!([0, 0, 1, 1, 0, 0, 0, 0])> {
    pub const COULOMB: Self = Self {
        name: "coulomb",
        symbols: &["C"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Electric potential
impl Unit<crate::dimension_exponents!([1, 2, -3, -1, 0, 0, 0, 0])> {
    pub const VOLT: Self = Self {
        name: "volt",
        symbols: &["V"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Capacitance
impl Unit<crate::dimension_exponents!([-1, -2, 4, 2, 0, 0, 0, 0])> {
    pub const FARAD: Self = Self {
        name: "farad",
        symbols: &["F"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

// Electric resistance
impl Unit<crate::dimension_exponents!([1, 2, -3, -2, 0, 0, 0, 0])> {
    pub const OHM: Self = Self {
        name: "ohm",
        symbols: &["Ω"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Electric conductance
impl Unit<crate::dimension_exponents!([-1, -2, 3, 2, 0, 0, 0, 0])> {
    pub const SIEMENS: Self = Self {
        name: "siemens",
        symbols: &["S"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Inductance
impl Unit<crate::dimension_exponents!([1, 2, -2, -2, 0, 0, 0, 0])> {
    pub const HENRY: Self = Self {
        name: "henry",
        symbols: &["H"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Magnetic field
impl Unit<crate::dimension_exponents!([1, 0, -2, -1, 0, 0, 0, 0])> {
    pub const TESLA: Self = Self {
        name: "tesla",
        symbols: &["T"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const GAUSS: Self = Self {
        name: "gauss",
        symbols: &["G"],
        scale: ScaleExponents::_10(-4),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Magnetic flex
impl Unit<crate::dimension_exponents!([1, 2, -2, -1, 0, 0, 0, 0])> {
    pub const WEBER: Self = Self {
        name: "weber",
        symbols: &["Wb"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Illuminance
impl Unit<crate::dimension_exponents!([0, -2, 0, 0, 0, 0, 1, 0])> {
    pub const LUX: Self = Self {
        name: "lux",
        symbols: &["lx"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const LUMEN: Self = Self {
        name: "lumen",
        symbols: &["lm"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Luminous intensity
impl Unit<crate::dimension_exponents!([0, 0, 0, 0, 0, 0, 1, 0])> {
    pub const CANDELA: Self = Self {
        name: "candela",
        symbols: &["cd"],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Kinematic viscosity
impl Unit<crate::dimension_exponents!([0, 2, -1, 0, 0, 0, 0, 0])> {
    pub const STOKES: Self = Self {
        name: "stokes",
        symbols: &["St"],
        scale: ScaleExponents::_10(-4),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };
}

/// Area
impl Unit<crate::dimension_exponents!([0, 2, 0, 0, 0, 0, 0, 0])> {
    pub const HECTARE: Self = Self {
        name: "hectare",
        symbols: &["hect"],
        scale: ScaleExponents::_10(4),
        conversion_factor: IDENTITY, // 1 hectare = 10,000 m²
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const ACRE: Self = Self {
        name: "acre",
        symbols: &["acre"],
        scale: ScaleExponents::_10(3),
        conversion_factor: 0.40468564224, // 1 acre = 4046.8564224 m²
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };
}

/// Volume
impl Unit<crate::dimension_exponents!([0, 3, 0, 0, 0, 0, 0, 0])> {
    // Metric volume units
    pub const LITER: Self = Self {
        name: "liter",
        symbols: &["L", "l"],
        scale: ScaleExponents::_10(-3),
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
    };

    pub const GALLON_US: Self = Self {
        name: "gallon",
        symbols: &["gal", "gallon"],
        scale: ScaleExponents::_10(-2),
        conversion_factor: 0.3785411784, // 1 US gallon = 3.785411784 L
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const GALLON_UK: Self = Self {
        name: "uk_gallon",
        symbols: &["uk_gal"],
        scale: ScaleExponents::_10(-2),
        conversion_factor: 0.454609, // 1 UK gallon = 4.54609 L
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const QUART_US: Self = Self {
        name: "quart",
        symbols: &["qrt"],
        scale: ScaleExponents::_10(-3),
        conversion_factor: 0.946352946, // 1 US quart = 0.946352946 L
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const QUART_UK: Self = Self {
        name: "uk_quart",
        symbols: &["uk_qrt"],
        scale: ScaleExponents::_10(-3),
        conversion_factor: 1.1365225, // 1 UK quart = 1.1365225 L
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const PINT_US: Self = Self {
        name: "pint",
        symbols: &["pnt"],
        scale: ScaleExponents::_10(-3),
        conversion_factor: 0.473176473, // 1 US pint = 0.473176473 L
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const PINT_UK: Self = Self {
        name: "uk_pint",
        symbols: &["uk_pnt"],
        scale: ScaleExponents::_10(-3),
        conversion_factor: 0.56826125, // 1 UK pint = 0.56826125 L
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const CUP_US: Self = Self {
        name: "cup",
        symbols: &["cup"],
        scale: ScaleExponents::_10(-4),
        conversion_factor: 2.365882365, // 1 US cup = 0.2365882365 L
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const CUP_UK: Self = Self {
        name: "uk_cup",
        symbols: &["uk_cup"],
        scale: ScaleExponents::_10(-4),
        conversion_factor: 2.84130625, // 1 UK cup = 0.284130625 L
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const FLUID_OUNCE_US: Self = Self {
        name: "fluid_ounce",
        symbols: &["fl_oz"],
        scale: ScaleExponents::_10(-5),
        conversion_factor: 2.95735295625,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const FLUID_OUNCE_UK: Self = Self {
        name: "uk_fluid_ounce",
        symbols: &["uk_fl_oz"],
        scale: ScaleExponents::_10(-5),
        conversion_factor: 2.84130625,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const TABLESPOON_US: Self = Self {
        name: "tablespoon",
        symbols: &["tbsp"],
        scale: ScaleExponents::_10(-5),
        conversion_factor: 1.478676478125,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const TABLESPOON_UK: Self = Self {
        name: "uk_tablespoon",
        symbols: &["uk_tbsp"],
        scale: ScaleExponents::_10(-5),
        conversion_factor: 1.77581640625,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const TEASPOON_US: Self = Self {
        name: "teaspoon",
        symbols: &["tsp"],
        scale: ScaleExponents::_10(-5),
        conversion_factor: 0.492892159375,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const TEASPOON_UK: Self = Self {
        name: "uk_teaspoon",
        symbols: &["uk_tsp"],
        scale: ScaleExponents::_10(-5),
        conversion_factor: 0.59193880208333,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };

    pub const BUSHEL: Self = Self {
        name: "bushel",
        symbols: &["bu"],
        scale: ScaleExponents::_10(-1),
        conversion_factor: 0.3523907016688,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Imperial,
    };
}

/// Dimensionless
impl Unit<crate::dimension_exponents!([0, 0, 0, 0, 0, 0, 0, 0])> {
    pub const DIMENSIONLESS: Self = Self {
        name: "dimensionless",
        symbols: &[],
        scale: ScaleExponents::IDENTITY,
        conversion_factor: IDENTITY,
        affine_offset: NONE,
        exponents: TypeDimensionExponents::new(),
        system: System::Metric,
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
