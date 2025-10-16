//! Default declarators for [Quantity] instances.
//! 
//! By default, declarators are generated for all standard SI units, base and derived.
//! Quantities in bespoke units can be declared using the [quantity!](crate::quantity!) macro,
//! which offers a simple unit literal syntax.
//! 
//! Declarator methods are technically non-const, since const traits are not yet generally available.
//! If const declaration is required, use the [quantity!](crate::quantity!) macro.
//! 
//! [Literal declarators](crate::define_literals!) can also be generated that sugar the
//! trait declarators, e.g. `1.0m` is equivalent to `1.0.meters()`.
//! 
//! ## Usage
//! 
//! ```rust
//! use whippyunits::default_declarators::*;
//! 
//! // atomic units...
//! let distance = 1.0.meters();
//! let distance = quantity!(1.0, m);
//! let distance = 1.0m; // (only available in scopes tagged with #[culit::culit])
//! 
//! // named derived units...
//! let energy = 1.0.joules();
//! let energy = quantity!(1.0, J);
//! let energy = 1.0J; // (only available in scopes tagged with #[culit::culit])
//! 
//! // bespoke units...
//! let bespoke = quantity!(1.0, V * s^2 / m);
//! ```

use crate::quantity_type::Quantity;
use crate::{Scale, Dimension, _2, _3, _5, _Pi, _M, _L, _T, _I, _Θ, _N, _J, _A};
use whippyunits_proc_macros::generate_default_declarators;

#[doc(hidden)]
macro_rules! define_quantity {
    (
        $mass_exp:expr, $length_exp:expr, $time_exp:expr, $current_exp:expr, $temperature_exp:expr, $amount_exp:expr, $luminosity_exp:expr, $angle_exp:expr,
        $trait_name:ident,
        $(($scale_name:ident, $fn_name:ident, $scale_p2:expr, $scale_p3:expr, $scale_p5:expr, $scale_pi:expr)),* $(,)?
    ) => {
        // Generate the trait definition (generic over storage type)
        pub trait $trait_name<T = f64> {
            $(
                fn $fn_name(self) -> $scale_name<T>;
            )*
        }

        // Generate the type definitions (generic with f64 default)
        $(
            pub type $scale_name<T = f64> = Quantity<
                Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>,
                Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>,
                T,
            >;
        )*

        // Generate default extension trait implementation (uses default f64)
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::<Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>, Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>, f64>::new(self)
                }
            )*
        }

        // Generate extension trait implementations for i32
        impl $trait_name<i32> for i32 {
            $(
                fn $fn_name(self) -> $scale_name<i32> {
                    Quantity::<Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>, Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>, i32>::new(self)
                }
            )*
        }
    };
}

#[doc(hidden)]
macro_rules! define_nonstorage_quantity {
    (
        $mass_exp:expr, $length_exp:expr, $time_exp:expr, $current_exp:expr, $temperature_exp:expr, $amount_exp:expr, $luminosity_exp:expr, $angle_exp:expr,
        $trait_name:ident,
        $(($fn_name:ident, $conversion_factor:expr, $scale_p2:expr, $scale_p3:expr, $scale_p5:expr, $scale_pi:expr)),* $(,)?
    ) => {
        // Generate the trait definition (generic over storage type)
        pub trait $trait_name<T = f64> {
            $(
                fn $fn_name(self) -> Quantity<
                    Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>,
                    Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>,
                    T,
                >;
            )*
        }

        // Generate extension trait implementations for f64 (default)
        impl $trait_name<f64> for f64 {
            $(
                fn $fn_name(self) -> Quantity<
                    Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>,
                    Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>,
                    f64,
                > {
                    Quantity::<Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>, Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>, f64>::new(self * $conversion_factor)
                }
            )*
        }

        // Generate extension trait implementations for i32
        impl $trait_name<i32> for i32 {
            $(
                fn $fn_name(self) -> Quantity<
                    Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>,
                    Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>,
                    i32,
                > {
                    Quantity::<Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>, Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>, i32>::new((self as f64 * $conversion_factor) as i32)
                }
            )*
        }

        // Generate extension trait implementations for i64
        impl $trait_name<i64> for i64 {
            $(
                fn $fn_name(self) -> Quantity<
                    Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>,
                    Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>,
                    i64,
                > {
                    Quantity::<Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>, Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>, i64>::new((self as f64 * $conversion_factor) as i64)
                }
            )*
        }
    };
}

#[doc(hidden)]
macro_rules! define_affine_quantity {
    (
        $mass_exp:expr, $length_exp:expr, $time_exp:expr, $current_exp:expr, $temperature_exp:expr, $amount_exp:expr, $luminosity_exp:expr, $angle_exp:expr,
        $trait_name:ident,
        $storage_scale:ident,
        $(($scale_name:ident, $fn_name:ident, $offset:expr)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $scale_name;
            )*
        }

        // Generate the type definitions (all stored in the same scale)
        $(
            pub type $scale_name = $storage_scale;
        )*

        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    $storage_scale::new(self + $offset)
                }
            )*
        }

        // Generate extension trait implementations for i32
        impl $trait_name for i32 {
            $(
                fn $fn_name(self) -> $scale_name {
                    $storage_scale::new((self as f64) + $offset)
                }
            )*
        }
    };
}

#[doc(hidden)]
macro_rules! define_nonstorage_affine_quantity {
    (
        $mass_exp:expr, $length_exp:expr, $time_exp:expr, $current_exp:expr, $temperature_exp:expr, $amount_exp:expr, $luminosity_exp:expr, $angle_exp:expr,
        $trait_name:ident,
        $(($fn_name:ident, $conversion_factor:expr, $affine_offset:expr, $scale_p2:expr, $scale_p3:expr, $scale_p5:expr, $scale_pi:expr)),* $(,)?
    ) => {
        // Generate the trait definition (generic over storage type)
        pub trait $trait_name<T = f64> {
            $(
                fn $fn_name(self) -> Quantity<
                    Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>,
                    Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>,
                    T,
                >;
            )*
        }

        // Generate extension trait implementations for f64 (default)
        impl $trait_name<f64> for f64 {
            $(
                fn $fn_name(self) -> Quantity<
                    Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>,
                    Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>,
                    f64,
                > {
                    Quantity::<Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>, Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>, f64>::new(self * $conversion_factor + $affine_offset)
                }
            )*
        }

        // Generate extension trait implementations for i32
        impl $trait_name<i32> for i32 {
            $(
                fn $fn_name(self) -> Quantity<
                    Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>,
                    Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>,
                    i32,
                > {
                    Quantity::<Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>, Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>, i32>::new((self as f64 * $conversion_factor + $affine_offset) as i32)
                }
            )*
        }

        // Generate extension trait implementations for i64
        impl $trait_name<i64> for i64 {
            $(
                fn $fn_name(self) -> Quantity<
                    Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>,
                    Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>,
                    i64,
                > {
                    Quantity::<Scale<_2<$scale_p2>, _3<$scale_p3>, _5<$scale_p5>, _Pi<$scale_pi>>, Dimension<_M<$mass_exp>, _L<$length_exp>, _T<$time_exp>, _I<$current_exp>, _Θ<$temperature_exp>, _N<$amount_exp>, _J<$luminosity_exp>, _A<$angle_exp>>, i64>::new((self as f64 * $conversion_factor + $affine_offset) as i64)
                }
            )*
        }
    };
}

// Generate all default declarators using the source of truth from default-dimensions
generate_default_declarators!();

/// Creates a Quantity instance with the specified value, units, and storage type.
///
/// ## Syntax
///
/// ```rust
/// quantity!(value, unit_expression)
/// quantity!(value, unit_expression, storage_type)
/// ```
/// 
/// where:
/// - `value`: The value of the quantity
/// - `unit_expression`: A "unit literal expression"
///     - A "unit literal expression" is either:
///         - An atomic unit: 
///             - `m`, `kg`, `s`, `A`, `K`, `mol`, `cd`, `rad`
///         - A multiplication of two or more atomic units: 
///             - `m * kg`
///         - A division of two or more atomic units: 
///             - `m / s`
///         - An exponentiation of an atomic unit: 
///             - `m^2`, `s^-1`
///         - A combination of the above:
///             - `m * kg / s^2`
/// - `storage_type`: An optional storage type for the quantity. Defaults to `f64`.
///
/// ## Examples
///
/// ```rust
/// use whippyunits::quantity;
///
/// // Basic quantities
/// let distance = quantity!(5.0, m);
/// let mass = quantity!(2.5, kg);
/// let time = quantity!(10.0, s);
///
/// // Compound units
/// let velocity = quantity!(10.0, m / s);
/// let acceleration = quantity!(9.81, m / s^2);
/// let force = quantity!(100.0, kg * m / s^2);
/// let energy = quantity!(50.0, kg * m^2 / s^2);
///
/// // With explicit storage type
/// let distance_f32 = quantity!(5.0, m, f32);
/// let mass_i32 = quantity!(2, kg, i32);
///
/// // Complex expressions
/// let power = quantity!(1000.0, kg * m^2 / s^3);
/// let pressure = quantity!(101325.0, kg / m / s^2);
/// ```
#[macro_export]
macro_rules! quantity {
    ($value:expr, $unit:expr) => {
        <$crate::unit!($unit)>::new($value)
    };
    ($value:expr, $unit:expr, $storage_type:ty) => {
        <$crate::unit!($unit, $storage_type)>::new($value)
    };
}
