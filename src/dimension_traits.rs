//! This module contains scale-generic dimension traits for the atomic dimensions.
//! 
//! These traits can be used to write code that enforces dimensional coherence while
//! leaving the scale unspecified:
//! 
//! ```rust
//! # #![feature(impl_trait_in_bindings)]
//! # #[culit::culit(whippyunits::default_declarators::literals)]
//! # fn main() {
//! # use whippyunits::dimension_traits::Length;
//! let length: impl Length = 1.0m;
//! let length: impl Length = 1.0mm;
//! // let length: impl Length = 1.0s; // 🚫 Compile error (dimension mismatch)
//! # }
//! ```
//! 
//! For non-atomic dimensions, use the [define_generic_dimension](crate::define_generic_dimension)
//! macro.
//! 
//! ### Scale-generic arithmetic
//! 
//! When writing functions that work with any scale, you need to add a `where` clause to check that
//! the two operands are valid for the arithmetic used in the function body.  Scale genericity does *not*
//! introduce any auto-rescaling semantics; addition is still a scale-safe operation, even if the scale
//! is generic:
//! 
//! ```rust
//! # #![feature(impl_trait_in_bindings)]
//! # #[culit::culit(whippyunits::default_declarators::literals)]
//! # fn main() {
//! # use whippyunits::dimension_traits::Length;
//! # use whippyunits::rescale;
//! # use core::ops::Add;
//! fn add_lengths<D1: Length, D2: Length>(d1: D1, d2: D2) -> <D1 as Add<D2>>::Output
//! where
//!     D1: Add<D2>,
//! {
//!     d1 + d2
//! }
//! 
//! let length: impl Length = add_lengths(1.0m, 1.0m); // ✅ 2.0 Quantity<m, f64>
//! let length: impl Length = add_lengths(1.0mm, 1.0mm); // ✅ 2.0 Quantity<mm, f64>
//! let length: impl Length = add_lengths(1.0m, rescale(1.0mm)); // ✅ 1.001 Quantity<m, f64>
//! // let length: impl Length = add_lengths(1.0m, 1.0mm); // 🚫 Compile error (scale mismatch)
//! // let length: impl Length = add_lengths(1.0m, 1.0s); // 🚫 Compile error (dimension mismatch)
//! # }
//! ```
//! 
//! The `Length` trait can only tell you "this type represents a length", but it can't tell you whether two
//! specific types can actually be added together (or multiplied, etc.). That check requires both types 
//! (`D1` and `D2`), and so must be done in the function. There is no way to assert on the trait itself
//! that "this type can be added to any other type that also represents a length".

use crate::quantity_type::Quantity;
use crate::{_2, _3, _5, _A, _I, _J, _L, _M, _N, _Pi, _T, _Θ, Dimension, Scale};

/// Expands to a trait and its implementation for a specific atomic dimension.
/// It follows the same pattern as the default declarators but focuses only on the
/// trait definition and implementation for scale-generic quantities.
#[macro_export]
#[doc(hidden)]
macro_rules! define_atomic_dimension_trait {
    (
        $mass_exp:expr, $length_exp:expr, $time_exp:expr, $current_exp:expr,
        $temperature_exp:expr, $amount_exp:expr, $luminosity_exp:expr, $angle_exp:expr,
        $trait_name:ident
    ) => {
        /// Trait for quantities with the specified atomic dimension
        pub trait $trait_name {
            type Unit;
        }

        impl<const SCALE_P2: i16, const SCALE_P3: i16, const SCALE_P5: i16, const SCALE_PI: i16, T>
            $trait_name
            for Quantity<
                Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>,
                Dimension<
                    _M<$mass_exp>,
                    _L<$length_exp>,
                    _T<$time_exp>,
                    _I<$current_exp>,
                    _Θ<$temperature_exp>,
                    _N<$amount_exp>,
                    _J<$luminosity_exp>,
                    _A<$angle_exp>,
                >,
                T,
            >
        {
            type Unit = Self;
        }
    };
}

// Define traits for all 8 atomic dimensions (SI base quantities)
define_atomic_dimension_trait!(1, 0, 0, 0, 0, 0, 0, 0, Mass);
define_atomic_dimension_trait!(0, 1, 0, 0, 0, 0, 0, 0, Length);
define_atomic_dimension_trait!(0, 0, 1, 0, 0, 0, 0, 0, Time);
define_atomic_dimension_trait!(0, 0, 0, 1, 0, 0, 0, 0, Current);
define_atomic_dimension_trait!(0, 0, 0, 0, 1, 0, 0, 0, Temperature);
define_atomic_dimension_trait!(0, 0, 0, 0, 0, 1, 0, 0, Amount);
define_atomic_dimension_trait!(0, 0, 0, 0, 0, 0, 1, 0, Luminosity);
define_atomic_dimension_trait!(0, 0, 0, 0, 0, 0, 0, 1, Angle);
