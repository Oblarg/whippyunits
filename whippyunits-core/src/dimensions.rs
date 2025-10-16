use crate::dimension_exponents::{DimensionExponents, DynDimensionExponents, TypeDimensionExponents};
use crate::prefix::SiPrefix;
use crate::units::Unit;

/// A dimension and its associated units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimension<ExponentsType: 'static = DynDimensionExponents> {
    /// Name of the dimension.
    pub name: &'static str,

    /// Symbol for the dimension.
    pub symbol: &'static str,

    /// Point in dimension space this dimension is for.
    pub exponents: ExponentsType,

    /// All supported units for this dimension.
    pub units: &'static [Unit<ExponentsType>],
    units_erased: &'static [Unit],
}

impl Dimension {
    /// Find a dimension by its name or symbol.
    pub fn find_dimension(name_or_symbol: &str) -> Option<&'static Self> {
        Self::ALL
            .iter()
            .find(|dim| dim.symbol == name_or_symbol || dim.name == name_or_symbol)
    }

    /// Find a unit by its symbol across all dimensions.
    pub fn find_unit_by_name(name: &str) -> Option<(&'static Unit, &'static Self)> {
        Self::ALL.iter().find_map(|dimension| {
            dimension
                .units
                .iter()
                .find(|unit| unit.name == name)
                .map(|unit| (unit, dimension))
        })
    }

    /// Find a unit by its symbol across all dimensions.
    pub fn find_unit_by_symbol(symbol: &str) -> Option<(&'static Unit, &'static Dimension)> {
        Self::ALL.iter().find_map(|dimension| {
            dimension
                .units
                .iter()
                .find(|unit| unit.symbols.contains(&symbol))
                .map(|unit| (unit, dimension))
        })
    }

    /// Find a unit by its symbol across all dimensions.
    pub fn find_si_unit_by_name(name: &str) -> Option<(&'static Unit, &'static Dimension)> {
        Self::ALL.iter().find_map(|dimension| {
            dimension
                .units
                .iter()
                .find(|unit| {
                    unit.name == name
                        && unit.exponents.as_basis().is_some()
                        && !unit.has_conversion()
                })
                .map(|unit| (unit, dimension))
        })
    }

    /// Find a unit by its symbol across all dimensions.
    pub fn find_si_unit_by_symbol(symbol: &str) -> Option<(&'static Unit, &'static Dimension)> {
        Self::ALL.iter().find_map(|dimension| {
            dimension
                .units
                .iter()
                .find(|unit| {
                    unit.symbols.contains(&symbol)
                        && unit.exponents.as_basis().is_some()
                        && !unit.has_conversion()
                })
                .map(|unit| (unit, dimension))
        })
    }

    /// Find a dimension by its exponents.
    pub fn find_dimension_by_exponents(exponents: DynDimensionExponents) -> Option<&'static Dimension> {
        Self::ALL.iter().find(|dim| dim.exponents == exponents)
    }

    /// Iterator over all unit names.
    pub fn names<'r>(
        dims: impl IntoIterator<Item = &'r Self>,
    ) -> impl Iterator<Item = &'static str> {
        dims.into_iter()
            .flat_map(|dimension| dimension.units.iter().map(move |unit| unit.name))
    }

    /// Iterator over all unit dymbols.
    pub fn symbols<'r>(
        dims: impl IntoIterator<Item = &'r Self>,
    ) -> impl Iterator<Item = &'static str> {
        dims.into_iter().flat_map(|dimension| {
            dimension
                .units
                .iter()
                .flat_map(move |unit| unit.symbols.iter().copied())
        })
    }

    /// Get all non SI base units.
    pub fn non_si_base_units() -> impl Iterator<Item = (&'static Unit, &'static Dimension)> {
        Self::ALL.iter().flat_map(|dimension| {
            dimension.units.iter().filter_map(move |unit| {
                if unit.exponents.as_basis().is_some() {
                    None
                } else {
                    Some((unit, dimension))
                }
            })
        })
    }

    /// Get all pure SI base units.
    pub fn si_base_units() -> impl Iterator<Item = (&'static Unit, &'static Dimension)> {
        Self::ALL.iter().flat_map(|dimension| {
            dimension.units.iter().filter_map(move |unit| {
                if unit.exponents.as_basis().is_some() {
                    Some((unit, dimension))
                } else {
                    None
                }
            })
        })
    }

    /// Find a unit by name or symbol across all dimensions.
    pub fn find_by_literal(
        literal: &str,
    ) -> Option<(&'static Unit, &'static Dimension, Option<&'static SiPrefix>)> {
        let literal = literal.trim_end_matches("s");

        if let Some((prefix, base)) = SiPrefix::strip_any_prefix_name(literal)
            && let Some((unit, dimension)) = Self::find_si_unit_by_name(base)
        {
            return Some((unit, dimension, Some(prefix)));
        }

        if let Some((prefix, base)) = SiPrefix::strip_any_prefix_symbol(literal)
            && let Some((unit, dimension)) = Self::find_si_unit_by_symbol(base)
        {
            return Some((unit, dimension, Some(prefix)));
        }

        if let Some((unit, dimension)) = Self::find_unit_by_name(literal) {
            Some((unit, dimension, None))
        } else if let Some((unit, dimension)) = Self::find_unit_by_symbol(literal) {
            Some((unit, dimension, None))
        } else {
            None
        }
    }
}

impl Dimension {
    /// All dimensions and their units.
    pub const ALL: &[Self] = &Self::ALL_FIXED;

    /// Atomic dimensions.
    pub const BASIS: [Self; 8] = [
        Dimension::MASS.erase(),
        Dimension::LENGTH.erase(),
        Dimension::TIME.erase(),
        Dimension::CURRENT.erase(),
        Dimension::TEMPERATURE.erase(),
        Dimension::AMOUNT.erase(),
        Dimension::LUMINOSITY.erase(),
        Dimension::ANGLE.erase(),
    ];

    const ALL_FIXED: [Self; 28] = [
        Dimension::MASS.erase(),
        Dimension::LENGTH.erase(),
        Dimension::TIME.erase(),
        Dimension::CURRENT.erase(),
        Dimension::TEMPERATURE.erase(),
        Dimension::AMOUNT.erase(),
        Dimension::LUMINOSITY.erase(),
        Dimension::ANGLE.erase(),
        Dimension::AREA.erase(),
        Dimension::VOLUME.erase(),
        Dimension::FREQUENCY.erase(),
        Dimension::FORCE.erase(),
        Dimension::ENERGY.erase(),
        Dimension::POWER.erase(),
        Dimension::PRESSURE.erase(),
        Dimension::ELECTRIC_CHARGE.erase(),
        Dimension::ELETRIC_POTENTIAL.erase(),
        Dimension::CAPACITANCE.erase(),
        Dimension::ELETRIC_RESISTANCE.erase(),
        Dimension::ELETRIC_CONDUCTANCE.erase(),
        Dimension::INDUCTANCE.erase(),
        Dimension::MAGNETIC_FIELD.erase(),
        Dimension::MAGNETIC_FLUX.erase(),
        Dimension::ILLUMINANCE.erase(),
        Dimension::VOLUME_MASS_DENSITY.erase(),
        Dimension::LINEAR_MASS_DENSITY.erase(),
        Dimension::DYNAMIC_VISCOSITY.erase(),
        Dimension::KINEMATIC_VISCOSITY.erase(),
    ];
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
    Dimension<
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
    pub const fn erase(&self) -> Dimension {
        Dimension {
            name: self.name,
            symbol: self.symbol,
            exponents: self.exponents.value_const(),
            units: self.units_erased,
            units_erased: self.units_erased,
        }
    }
}

impl Dimension<crate::dimension_exponents!([1, 0, 0, 0, 0, 0, 0, 0])> {
    pub const MASS: Self = __dim!(Self {
        name: "Mass",
        symbol: "M",
        units: &[Unit::GRAM, Unit::OUNCE, Unit::POUND, Unit::STONE, Unit::TON],
    });
}

impl Dimension<crate::dimension_exponents!([0, 1, 0, 0, 0, 0, 0, 0])> {
    pub const LENGTH: Self = __dim!(Self {
        name: "Length",
        symbol: "L",
        units: &[Unit::METER, Unit::INCH, Unit::FOOT, Unit::YARD, Unit::MILE],
    });
}

impl Dimension<crate::dimension_exponents!([0, 0, 1, 0, 0, 0, 0, 0])> {
    pub const TIME: Self = __dim!(Self {
        name: "Time",
        symbol: "T",
        units: &[Unit::SECOND, Unit::MINUTE, Unit::HOUR, Unit::DAY],
    });
}

impl Dimension<crate::dimension_exponents!([0, 0, 0, 1, 0, 0, 0, 0])> {
    pub const CURRENT: Self = __dim!(Self {
        name: "Current",
        symbol: "I",
        units: &[Unit::AMPERE],
    });
}

impl Dimension<crate::dimension_exponents!([0, 0, 0, 0, 1, 0, 0, 0])> {
    pub const TEMPERATURE: Self = __dim!(Self {
        name: "Temperature",
        symbol: "θ",
        units: &[Unit::KELVIN, Unit::CELSIUS, Unit::RANKINE, Unit::FAHRENHEIT],
    });
}

impl Dimension<crate::dimension_exponents!([0, 0, 0, 0, 0, 1, 0, 0])> {
    pub const AMOUNT: Self = __dim!(Self {
        name: "Amount",
        symbol: "N",
        units: &[Unit::MOLE],
    });
}

impl Dimension<crate::dimension_exponents!([0, 0, 0, 0, 0, 0, 1, 0])> {
    pub const LUMINOSITY: Self = __dim!(Self {
        name: "Luminosity",
        symbol: "Cd",
        units: &[Unit::CANDELA],
    });
}

impl Dimension<crate::dimension_exponents!([0, 0, 0, 0, 0, 0, 0, 1])> {
    pub const ANGLE: Self = __dim!(Self {
        name: "Angle",
        symbol: "A",
        units: &[
            Unit::RADIAN,
            Unit::DEGREE,
            Unit::GRADIAN,
            Unit::TURN,
            Unit::ARCMINUTE,
            Unit::ARCSECOND,
        ]
    });
}

impl Dimension<crate::dimension_exponents!([0, 2, 0, 0, 0, 0, 0, 0])> {
    pub const AREA: Self = __dim!(Self {
        name: "Area",
        symbol: "L²",
        units: &[
            // Metric area units (canonical display unit)
            Unit::SQUARE_METER,
            // Imperial area units
            Unit::ACRE,
            // Metric area units
            Unit::HECTARE,
        ],
    });
}

impl Dimension<crate::dimension_exponents!([0, 3, 0, 0, 0, 0, 0, 0])> {
    pub const VOLUME: Self = __dim!(Self {
        name: "Volume",
        symbol: "L³",
        units: &[
            // Metric volume units
            Unit::LITER,
            // Imperial volume units
            Unit::GALLON_US,
            Unit::QUART_US,
            Unit::PINT_US,
            Unit::CUP_US,
            Unit::FLUID_OUNCE_US,
            Unit::TABLESPOON_US,
            Unit::TEASPOON_US,
            // UK Imperial volume units
            Unit::GALLON_UK,
            Unit::QUART_UK,
            Unit::PINT_UK,
            Unit::CUP_UK,
            Unit::FLUID_OUNCE_UK,
            Unit::TABLESPOON_UK,
            Unit::TEASPOON_UK,
        ],
    });
}

impl Dimension<crate::dimension_exponents!([0, 0, -1, 0, 0, 0, 0, 0])> {
    pub const FREQUENCY: Self = __dim!(Self {
        name: "Frequency",
        symbol: "T⁻¹",
        units: &[Unit::HERTZ],
    });
}

impl Dimension<crate::dimension_exponents!([1, 1, -2, 0, 0, 0, 0, 0])> {
    pub const FORCE: Self = __dim!(Self {
        name: "Force",
        symbol: "MLT⁻²",
        units: &[Unit::NEWTON],
    });
}

impl Dimension<crate::dimension_exponents!([1, 2, -2, 0, 0, 0, 0, 0])> {
    pub const ENERGY: Self = __dim!(Self {
        name: "Energy",
        symbol: "ML²T⁻²",
        units: &[Unit::JOULE],
    });
}

impl Dimension<crate::dimension_exponents!([1, 2, -3, 0, 0, 0, 0, 0])> {
    pub const POWER: Self = __dim!(Self {
        name: "Power",
        symbol: "ML²T⁻³",
        units: &[Unit::WATT],
    });
}

impl Dimension<crate::dimension_exponents!([1, -1, -2, 0, 0, 0, 0, 0])> {
    pub const PRESSURE: Self = __dim!(Self {
        name: "Pressure",
        symbol: "ML⁻¹T⁻²",
        units: &[Unit::PASCAL],
    });
}

impl Dimension<crate::dimension_exponents!([0, 0, 1, 1, 0, 0, 0, 0])> {
    pub const ELECTRIC_CHARGE: Self = __dim!(Self {
        name: "Electric Charge",
        symbol: "TI",
        units: &[Unit::COULOMB],
    });
}

impl Dimension<crate::dimension_exponents!([1, 2, -3, -1, 0, 0, 0, 0])> {
    pub const ELETRIC_POTENTIAL: Self = __dim!(Self {
        name: "Electric Potential",
        symbol: "ML²T⁻³I⁻¹",
        units: &[Unit::VOLT],
    });
}

impl Dimension<crate::dimension_exponents!([-1, -2, 4, 2, 0, 0, 0, 0])> {
    pub const CAPACITANCE: Self = __dim!(Self {
        name: "Capacitance",
        symbol: "M⁻¹L⁻²T⁴I²",
        units: &[Unit::FARAD],
    });
}

impl Dimension<crate::dimension_exponents!([1, 2, -3, -2, 0, 0, 0, 0])> {
    pub const ELETRIC_RESISTANCE: Self = __dim!(Self {
        name: "Electric Resistance",
        symbol: "ML²T⁻³I⁻²",
        units: &[Unit::OHM],
    });
}

impl Dimension<crate::dimension_exponents!([-1, -2, 3, 2, 0, 0, 0, 0])> {
    pub const ELETRIC_CONDUCTANCE: Self = __dim!(Self {
        name: "Electric Conductance",
        symbol: "M⁻¹L⁻²T³I²",
        units: &[Unit::SIEMENS],
    });
}

impl Dimension<crate::dimension_exponents!([1, 2, -2, -2, 0, 0, 0, 0])> {
    pub const INDUCTANCE: Self = __dim!(Self {
        name: "Inductance",
        symbol: "ML²T⁻²I⁻²",
        units: &[Unit::HENRY],
    });
}

impl Dimension<crate::dimension_exponents!([1, 0, -2, -1, 0, 0, 0, 0])> {
    pub const MAGNETIC_FIELD: Self = __dim!(Self {
        name: "Magnetic Field",
        symbol: "MT⁻²I⁻¹",
        units: &[Unit::TESLA],
    });
}

impl Dimension<crate::dimension_exponents!([1, 2, -2, -1, 0, 0, 0, 0])> {
    pub const MAGNETIC_FLUX: Self = __dim!(Self {
        name: "Magnetic Flux",
        symbol: "ML²T⁻²I⁻¹",
        units: &[Unit::WEBER],
    });
}

impl Dimension<crate::dimension_exponents!([0, -2, 0, 0, 0, 0, 1, 0])> {
    pub const ILLUMINANCE: Self = __dim!(Self {
        name: "Illuminance",
        symbol: "L⁻²Cd",
        units: &[Unit::LUX],
    });
}

impl Dimension<crate::dimension_exponents!([1, -3, 0, 0, 0, 0, 0, 0])> {
    pub const VOLUME_MASS_DENSITY: Self = __dim!(Self {
        name: "Volume Mass Density",
        symbol: "ML⁻³",
        units: &[
            // No atomic units for volume mass density - it's a derived dimension
        ],
    });
}

impl Dimension<crate::dimension_exponents!([1, -1, 0, 0, 0, 0, 0, 0])> {
    pub const LINEAR_MASS_DENSITY: Self = __dim!(Self {
        name: "Linear Mass Density",
        symbol: "ML⁻¹",
        units: &[
            // No atomic units for linear mass density - it's a derived dimension
        ],
    });
}

impl Dimension<crate::dimension_exponents!([1, -1, 1, 0, 0, 0, 0, 0])> {
    pub const DYNAMIC_VISCOSITY: Self = __dim!(Self {
        name: "Dynamic Viscosity",
        symbol: "ML⁻¹T⁻¹",
        units: &[
            // No atomic units for dynamic viscosity - it's a derived dimension
        ],
    });
}

impl Dimension<crate::dimension_exponents!([0, 2, -1, 0, 0, 0, 0, 0])> {
    pub const KINEMATIC_VISCOSITY: Self = __dim!(Self {
        name: "Kinematic Viscosity",
        symbol: "L²T⁻¹",
        units: &[Unit::STOKES],
    });
}

impl Dimension<crate::dimension_exponents!([0, 0, 0, 0, 0, 0, 0, 0])> {
    pub const NONE: Self = __dim!(Self {
        name: "dimensionless",
        symbol: "1",
        units: &[Unit::DIMENSIONLESS],
    });
}

pub struct DimensionExponentsFmt(DynDimensionExponents);

impl core::fmt::Display for DimensionExponentsFmt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut first = true;
        for (d, x) in Dimension::BASIS.iter().zip(self.0.0) {
            if x == 0 {
                continue;
            }

            if first {
                first = false;
            } else {
                write!(f, " * ")?;
            }

            write!(f, "{}^{x}", d.units[0].symbols[0])?;
        }

        if first {
            write!(f, "1")?;
        }

        Ok(())
    }
}

macro_rules! __dim {
    (
        Self {
            name: $name:expr,
            symbol: $symbol:expr,
            units: &[$($unit:path),* $(,)?] $(,)?
        }
    ) => {
        Self {
            name: $name,
            symbol: $symbol,
            exponents: TypeDimensionExponents::new(),
            units: &[$($unit),*],
            units_erased: &[$($unit.erase()),*],
        }
    }
}
use __dim;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_dimension_by_literal_string() {
        assert_eq!(
            Dimension::find_by_literal("rad").unwrap(),
            (&Unit::RADIAN.erase(), &Dimension::ANGLE.erase(), None,)
        );

        assert_eq!(
            Dimension::find_by_literal("mrad").unwrap(),
            (
                &Unit::RADIAN.erase(),
                &Dimension::ANGLE.erase(),
                Some(&SiPrefix::MILLI),
            )
        );

        assert_eq!(
            Dimension::find_by_literal("megameter").unwrap(),
            (
                &Unit::METER.erase(),
                &Dimension::LENGTH.erase(),
                Some(&SiPrefix::MEGA),
            )
        );

        assert_eq!(
            Dimension::find_by_literal("Megameter").unwrap(),
            (
                &Unit::METER.erase(),
                &Dimension::LENGTH.erase(),
                Some(&SiPrefix::MEGA),
            )
        );

        assert_eq!(
            Dimension::find_by_literal("gram").unwrap(),
            (&Unit::GRAM.erase(), &Dimension::MASS.erase(), None,)
        );

        assert_eq!(
            Dimension::find_by_literal("grams").unwrap(),
            (&Unit::GRAM.erase(), &Dimension::MASS.erase(), None,)
        );

        assert_eq!(Dimension::find_by_literal("millipound"), None,);

        assert_eq!(Dimension::find_by_literal("abc"), None,);
    }

    #[test]
    fn can_format_dimension_exponents() {
        assert_eq!(
            format!(
                "{}",
                DimensionExponentsFmt(Dimension::ENERGY.exponents.value())
            ),
            "g^1 * m^2 * s^-2"
        );

        assert_eq!(
            format!(
                "{}",
                DimensionExponentsFmt(Dimension::NONE.exponents.value())
            ),
            "1"
        );
    }
}
