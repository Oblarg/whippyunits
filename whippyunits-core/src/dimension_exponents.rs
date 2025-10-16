use core::ops::{Add, Mul, Neg, Sub};

use crate::num::N;

/// Axis of dimensions vector space.
///
/// All dimensions live in a 8 axis vector space.
/// For example, `m^2/s` would be vector `[0, 2, -1, 0, 0, 0, 0, 0]`.
/// These axis are defined by the
/// [International System of Quantities](https://en.wikipedia.org/wiki/International_System_of_Quantities). Additionally, we add angles as their own axis.
///
/// Addition of dimensioned quantities is only defined if their dimensions
/// are the same vector.
/// Multiplication of dimensioned quantities is defined by adding their
/// dimension vectors to form a new dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DimensionBasis {
    Mass,
    Length,
    Time,
    Current,
    Temperature,
    Amount,
    Luminosity,
    Angle,
}

impl DimensionBasis {
    /// List of all dimension basis.
    pub const ALL: [Self; 8] = [
        Self::Mass,
        Self::Length,
        Self::Time,
        Self::Current,
        Self::Temperature,
        Self::Amount,
        Self::Luminosity,
        Self::Angle,
    ];

    /// Convert a basis to it's exponents.
    ///
    /// The resulting exponents will always have one and only one exponent set to `1`.
    pub const fn exponents(&self) -> DynDimensionExponents {
        match self {
            Self::Mass => DynDimensionExponents::MASS,
            Self::Length => DynDimensionExponents::LENGTH,
            Self::Time => DynDimensionExponents::TIME,
            Self::Current => DynDimensionExponents::CURRENT,
            Self::Temperature => DynDimensionExponents::TEMPERATURE,
            Self::Amount => DynDimensionExponents::AMOUNT,
            Self::Luminosity => DynDimensionExponents::LUMINOSITY,
            Self::Angle => DynDimensionExponents::ANGLE,
        }
    }

    /// Get the symbol for the dimension.
    pub const fn symbol(&self) -> &'static str {
        match self {
            Self::Mass => "M",
            Self::Length => "L",
            Self::Time => "T",
            Self::Current => "I",
            Self::Temperature => "Θ",
            Self::Amount => "N",
            Self::Luminosity => "J",
            Self::Angle => "A",
        }
    }
}

/// Trait for type level or dynamic dimension exponents.
///
/// Dimension exponents are tracked at the type level most of the time
/// to provide compile time checks.
pub trait DimensionExponents: exponents_seal::Sealed + 'static {
    /// Get the exponent values.
    fn value(&self) -> DynDimensionExponents;

    /// Check if all the exponents are zero.
    ///
    /// When `true` that maps to a dimensionless quantity.
    fn is_zero(&self) -> bool {
        self.value() == DynDimensionExponents::ZERO
    }

    /// Get the basis this value represents.
    ///
    /// A basis is one of the eight axis in the space defining all dimensions.
    ///
    /// This returns `None` for negative basis.
    fn as_basis(&self) -> Option<DimensionBasis> {
        match self.value().0.iter().position(|&x| x == 1) {
            Some(0) => Some(DimensionBasis::Mass),
            Some(1) => Some(DimensionBasis::Length),
            Some(2) => Some(DimensionBasis::Time),
            Some(3) => Some(DimensionBasis::Current),
            Some(4) => Some(DimensionBasis::Temperature),
            Some(5) => Some(DimensionBasis::Amount),
            Some(6) => Some(DimensionBasis::Luminosity),
            Some(7) => Some(DimensionBasis::Angle),
            Some(_) => unreachable!(),
            None => None,
        }
    }
}

mod exponents_seal {
    use super::__exponents;

    pub trait Sealed {}

    impl Sealed for super::DynDimensionExponents {}

    super::__exponents_impl_trait!({
        impl Sealed for TypeDimensionExponents {}
    });
}

/// Type level dimension of a quantity.
///
/// If all dimension exponents are zero, the quantity is dimensionless.
pub struct TypeDimensionExponents<
    Mass = _M<0>,
    Length = _L<0>,
    Time = _T<0>,
    Current = _I<0>,
    Temperature = _Θ<0>,
    Amount = _N<0>,
    Luminosity = _J<0>,
    Angle = _A<0>,
> {
    #[allow(clippy::type_complexity)]
    _phantom: core::marker::PhantomData<(
        Mass,
        Length,
        Time,
        Current,
        Temperature,
        Amount,
        Luminosity,
        Angle,
    )>,
}

pub type Zero = dimension_exponents!([0, 0, 0, 0, 0, 0, 0, 0]);
pub type Mass = dimension_exponents!([1, 0, 0, 0, 0, 0, 0, 0]);
pub type Length = dimension_exponents!([0, 1, 0, 0, 0, 0, 0, 0]);
pub type Time = dimension_exponents!([0, 0, 1, 0, 0, 0, 0, 0]);
pub type Current = dimension_exponents!([0, 0, 0, 1, 0, 0, 0, 0]);
pub type Temperature = dimension_exponents!([0, 0, 0, 0, 1, 0, 0, 0]);
pub type Amount = dimension_exponents!([0, 0, 0, 0, 0, 1, 0, 0]);
pub type Luminosity = dimension_exponents!([0, 0, 0, 0, 0, 0, 1, 0]);
pub type Angle = dimension_exponents!([0, 0, 0, 0, 0, 0, 0, 1]);

/// Construct [`TypeDimensionExponents`] type from an array of exponents.
#[macro_export]
macro_rules! dimension_exponents {
    ([
        $m:expr,
        $l:expr,
        $t:expr,
        $i:expr,
        $h:expr,
        $n:expr,
        $j:expr,
        $a:expr $(,)?
    ]) => {
        $crate::dimension_exponents::TypeDimensionExponents::<
            $crate::dimension_exponents::_M<$m>,
            $crate::dimension_exponents::_L<$l>,
            $crate::dimension_exponents::_T<$t>,
            $crate::dimension_exponents::_I<$i>,
            $crate::dimension_exponents::_Θ<$h>,
            $crate::dimension_exponents::_N<$n>,
            $crate::dimension_exponents::_J<$j>,
            $crate::dimension_exponents::_A<$a>,
        >
    };
}
use dimension_exponents;

__exponents_impl!({
    impl TypeDimensionExponents {
        pub const fn new() -> Self {
            Self {
                _phantom: core::marker::PhantomData,
            }
        }

        pub const fn value_const(&self) -> DynDimensionExponents {
            DynDimensionExponents([
                MASS_EXP,
                LENGTH_EXP,
                TIME_EXP,
                CURRENT_EXP,
                TEMPERATURE_EXP,
                AMOUNT_EXP,
                LUMINOSITY_EXP,
                ANGLE_EXP,
            ])
        }
    }
});

__exponents_impl_trait!({
    impl Default for TypeDimensionExponents {
        fn default() -> __exponents!() {
            Self::new()
        }
    }
});

__exponents_impl_trait!({
    impl DimensionExponents for TypeDimensionExponents {
        fn value(&self) -> DynDimensionExponents {
            self.value_const()
        }
    }
});

__exponents_impl_op!({
    impl Add for TypeDimensionExponents {
        type Output = __exponents!();

        fn add(self, _: __exponents!(@rhs)) -> Self::Output {
            TypeDimensionExponents {
                _phantom: core::marker::PhantomData,
            }
        }
    }
});

__exponents_impl_op!({
    impl Sub for TypeDimensionExponents {
        type Output = __exponents!();

        fn sub(self, _: __exponents!(@rhs)) -> Self::Output {
            TypeDimensionExponents {
                _phantom: core::marker::PhantomData,
            }
        }
    }
});

/// The mass dimension exponent of a quantity.
pub struct _M<const EXP: i16 = 0>;

/// The length dimension exponent of a quantity.
pub struct _L<const EXP: i16 = 0>;

/// The time dimension exponent of a quantity.
pub struct _T<const EXP: i16 = 0>;

/// The current dimension exponent of a quantity.
pub struct _I<const EXP: i16 = 0>;

/// The temperature dimension exponent of a quantity.
pub struct _Θ<const EXP: i16 = 0>;

/// The amount dimension exponent of a quantity.
pub struct _N<const EXP: i16 = 0>;

/// The luminosity dimension exponent of a quantity.
pub struct _J<const EXP: i16 = 0>;

/// The angle dimension exponent of a quantity.
pub struct _A<const EXP: i16 = 0>;

/// Dimension vector.
///
/// We choose this labelling of the space's axes.
/// ```text
/// [mass, length, time, current, temperature, amount, luminosity, angle]
/// ```
/// We consider angles to be a seperate axis to distingish them from dimensionless.
///
/// Each dimensional quantity exists at some unique integer lattice point in this space.
/// For example `m/s` would be vector `[0, 1, -1, 0, 0, 0, 0, 0]`. The numbers are the
/// exponents found in the written units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DynDimensionExponents(pub [i16; 8]);

impl DimensionExponents for DynDimensionExponents {
    fn value(&self) -> Self {
        *self
    }
}

impl DynDimensionExponents {
    pub const ZERO: Self = Zero::new().value_const();

    pub const MASS: Self = Mass::new().value_const();
    pub const LENGTH: Self = Length::new().value_const();
    pub const TIME: Self = Time::new().value_const();
    pub const CURRENT: Self = Current::new().value_const();
    pub const TEMPERATURE: Self = Temperature::new().value_const();
    pub const AMOUNT: Self = Amount::new().value_const();
    pub const LUMINOSITY: Self = Luminosity::new().value_const();
    pub const ANGLE: Self = Angle::new().value_const();
}

impl Mul<i16> for DynDimensionExponents {
    type Output = DynDimensionExponents;

    fn mul(self, rhs: i16) -> Self {
        Self([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
            self.0[4] * rhs,
            self.0[5] * rhs,
            self.0[6] * rhs,
            self.0[7] * rhs,
        ])
    }
}

impl<T: DimensionExponents> Add<T> for DynDimensionExponents {
    type Output = DynDimensionExponents;

    fn add(self, rhs: T) -> Self {
        let rhs = rhs.value();

        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
            self.0[4] + rhs.0[4],
            self.0[5] + rhs.0[5],
            self.0[6] + rhs.0[6],
            self.0[7] + rhs.0[7],
        ])
    }
}

impl Neg for DynDimensionExponents {
    type Output = DynDimensionExponents;

    fn neg(self) -> Self {
        Self([
            -self.0[0], -self.0[1], -self.0[2], -self.0[3], -self.0[4], -self.0[5], -self.0[6],
            -self.0[7],
        ])
    }
}

impl core::fmt::Display for DynDimensionExponents {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut first = true;
        for (d, x) in DimensionBasis::ALL.iter().zip(self.0) {
            if x == 0 {
                continue;
            }

            if first {
                first = false;
            } else {
                write!(f, " * ")?;
            }

            write!(f, "{}^{x}", d.symbol())?;
        }

        if first {
            write!(f, "1")?;
        }

        Ok(())
    }
}

macro_rules! __exponents_impl {
    ({
        impl TypeDimensionExponents {
            $($t:tt)*
        }
    }) => {
        impl<
            const MASS_EXP: i16,
            const LENGTH_EXP: i16,
            const TIME_EXP: i16,
            const CURRENT_EXP: i16,
            const TEMPERATURE_EXP: i16,
            const AMOUNT_EXP: i16,
            const LUMINOSITY_EXP: i16,
            const ANGLE_EXP: i16,
        > __exponents!() {
            $($t)*
        }
    };
}
use __exponents_impl;

macro_rules! __exponents_impl_trait {
    ({
        impl $trait:ident for TypeDimensionExponents {
            $($t:tt)*
        }
    }) => {
        impl<
            const MASS_EXP: i16,
            const LENGTH_EXP: i16,
            const TIME_EXP: i16,
            const CURRENT_EXP: i16,
            const TEMPERATURE_EXP: i16,
            const AMOUNT_EXP: i16,
            const LUMINOSITY_EXP: i16,
            const ANGLE_EXP: i16,
        > $trait for __exponents!() {
            $($t)*
        }
    };
}
use __exponents_impl_trait;

macro_rules! __exponents_impl_op {
    ({
        impl $op:ident for TypeDimensionExponents {
            $($t:tt)*
        }
    }) => {
        impl<
            const MASS_EXP: i16,
            const LENGTH_EXP: i16,
            const TIME_EXP: i16,
            const CURRENT_EXP: i16,
            const TEMPERATURE_EXP: i16,
            const AMOUNT_EXP: i16,
            const LUMINOSITY_EXP: i16,
            const ANGLE_EXP: i16,
            const MASS_EXP1: i16,
            const LENGTH_EXP1: i16,
            const TIME_EXP1: i16,
            const CURRENT_EXP1: i16,
            const TEMPERATURE_EXP1: i16,
            const AMOUNT_EXP1: i16,
            const LUMINOSITY_EXP1: i16,
            const ANGLE_EXP1: i16,
            const MASS_EXP2: i16,
            const LENGTH_EXP2: i16,
            const TIME_EXP2: i16,
            const CURRENT_EXP2: i16,
            const TEMPERATURE_EXP2: i16,
            const AMOUNT_EXP2: i16,
            const LUMINOSITY_EXP2: i16,
            const ANGLE_EXP2: i16,
        > $op<__exponents!(@rhs)> for __exponents!(@lhs)
where
    N<MASS_EXP1>: Add<N<MASS_EXP2>, Output = N<MASS_EXP>>,
    N<LENGTH_EXP1>: Add<N<LENGTH_EXP2>, Output = N<LENGTH_EXP>>,
    N<TIME_EXP1>: Add<N<TIME_EXP2>, Output = N<TIME_EXP>>,
    N<CURRENT_EXP1>: Add<N<CURRENT_EXP2>, Output = N<CURRENT_EXP>>,
    N<TEMPERATURE_EXP1>: Add<N<TEMPERATURE_EXP2>, Output = N<TEMPERATURE_EXP>>,
    N<AMOUNT_EXP1>: Add<N<AMOUNT_EXP2>, Output = N<AMOUNT_EXP>>,
    N<LUMINOSITY_EXP1>: Add<N<LUMINOSITY_EXP2>, Output = N<LUMINOSITY_EXP>>,
    N<ANGLE_EXP1>: Add<N<ANGLE_EXP2>, Output = N<ANGLE_EXP>>,
{
            $($t)*
        }
    };
}
use __exponents_impl_op;

macro_rules! __exponents {
    () => {
        $crate::dimension_exponents!([
            MASS_EXP,
            LENGTH_EXP,
            TIME_EXP,
            CURRENT_EXP,
            TEMPERATURE_EXP,
            AMOUNT_EXP,
            LUMINOSITY_EXP,
            ANGLE_EXP,
        ])
    };
    (@lhs) => {
        $crate::dimension_exponents!([
            MASS_EXP1,
            LENGTH_EXP1,
            TIME_EXP1,
            CURRENT_EXP1,
            TEMPERATURE_EXP1,
            AMOUNT_EXP1,
            LUMINOSITY_EXP1,
            ANGLE_EXP1,
        ])
    };
    (@rhs) => {
        $crate::dimension_exponents!([
            MASS_EXP2,
            LENGTH_EXP2,
            TIME_EXP2,
            CURRENT_EXP2,
            TEMPERATURE_EXP2,
            AMOUNT_EXP2,
            LUMINOSITY_EXP2,
            ANGLE_EXP2,
        ])
    };
}
use __exponents;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponent_type_to_value() {
        let exp = <dimension_exponents!([1, -2, 3, -4, 5, -6, 7, -8])>::new().value();
        assert_eq!(
            exp.value(),
            DynDimensionExponents([1, -2, 3, -4, 5, -6, 7, -8])
        );
    }

    #[test]
    fn exponents_can_be_types() {
        let exp = <dimension_exponents!([0, 1, 0, 0, 0, 0, 0, 0])>::new();
        assert_eq!(exp.as_basis().unwrap(), DimensionBasis::Length);

        let exp = exp + <dimension_exponents!([0, 0, 2, 0, 0, 0, 0, 0])>::new();
        assert_eq!(format!("{}", exp.value()), "L^1 * T^2");
    }

    #[test]
    fn exponents_can_be_values() {
        let exp = DynDimensionExponents([1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(exp.as_basis().unwrap(), DimensionBasis::Mass);

        let exp = exp + DynDimensionExponents([0, 0, 2, 0, 0, 0, 0, 0]);
        assert_eq!(format!("{exp}"), "M^1 * T^2");
    }

    #[test]
    fn can_add_types_to_values() {
        let a = Mass::new();
        let b = DynDimensionExponents::LENGTH;
        let c = b + a;

        assert_eq!(c, DynDimensionExponents([1, 1, 0, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn dimensionless_formats_as_1() {
        assert_eq!(format!("{}", DynDimensionExponents::ZERO), "1");
    }

    #[test]
    fn basis_to_exponents_to_basis_is_consistent() {
        for basis in DimensionBasis::ALL {
            assert_eq!(basis.exponents().as_basis().unwrap(), basis,);
        }
    }
}
