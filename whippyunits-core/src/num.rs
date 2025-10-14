//! Polyfill for generic const expressions.
//!
//! Because [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560)
//! is an untable feature (along with being incomplete), this polyfill exists to
//! allow a small subset of those operations on stable.
//!
//! Because dimensional analysis usually has small dimension exponents we constrain
//! this polyfill to working with input integers in the range -7 to 7. If the `nightly`
//! feature is enable then `generic_const_exprs` is used to provide implementations for
//! all `i16` integers.

/// Const generic with type level math operations.
///
/// Supported operations:
/// - Addition: `<A as Add<B>>::Output`
/// - Subtraction: `<A as Sub<B>>::Output`
/// - Negation: `<X as Neg>::Output`
///
/// Without the `nightly` feature the operations are limited to inputs in the range -7 to 7.
///
/// The [`Num`] trait can be used to constrain a generic to only this type.
pub struct N<const X: i16>;

/// Higher order type of [`N`] numbers.
///
/// This trait is sealed and cannot be implemented.
pub trait Num: num_seal::Sealed {
    /// The value of this number.
    const N: i16;
}

impl<const X: i16> Num for N<X> {
    const N: i16 = X;
}

mod num_seal {
    pub trait Sealed {}

    impl<const X: i16> Sealed for super::N<X> {}
}

#[cfg(feature = "nightly")]
mod nightly {
    use super::N;

    impl<const A: i16, const B: i16> core::ops::Add<N<B>> for N<A>
    where
        [(); { A + B } as usize]:,
    {
        type Output = N<{ A + B }>;

        fn add(self, _: N<B>) -> Self::Output {
            N
        }
    }

    impl<const A: i16, const B: i16> core::ops::Sub<N<B>> for N<A>
    where
        [(); { A - B } as usize]:,
    {
        type Output = N<{ A - B }>;

        fn sub(self, _: N<B>) -> Self::Output {
            N
        }
    }

    impl<const X: i16> core::ops::Neg for N<X>
    where
        [(); { -X } as usize]:,
    {
        type Output = N<{ -X }>;

        fn neg(self) -> Self::Output {
            N
        }
    }
}

// Here is our polyfill which manually implements every case.
// It isnt "nice" but it works well enough.
#[cfg(not(feature = "nightly"))]
mod stable {
    use super::N;

    __cartesian_impls!(-7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7);

    macro_rules! __pair_impls {
        ($a:literal, $b:literal) => {
            impl core::ops::Add<N<$b>> for N<$a> {
                type Output = N<{ $a + $b }>;

                fn add(self, _: N<$b>) -> Self::Output {
                    N
                }
            }

            impl core::ops::Sub<N<$b>> for N<$a> {
                type Output = N<{ $a - $b }>;

                fn sub(self, _: N<$b>) -> Self::Output {
                    N
                }
            }
        };
    }
    use __pair_impls;

    macro_rules! __single_impls {
        ($n:literal) => {
            impl core::ops::Neg for N<$n> {
                type Output = N<{ -$n }>;

                fn neg(self) -> Self::Output {
                    N
                }
            }
        };
    }
    use __single_impls;

    macro_rules! __cartesian_impls {
        (@inner $a:literal []) => {
            // Base case, we do nothing here.
        };
        (@double [] $t:tt) => {
            // Base case, we do nothing here.
        };
        (@inner $a:literal [$b:literal $(, $b_:literal)*]) => {
            __pair_impls! { $a, $b }

            __cartesian_impls! {
                @inner
                $a
                [$($b_),*]
            }
        };
        (@double [$a:literal $(, $a_:tt)*] [$($b:literal),*]) => {
            __single_impls! { $a }

            __cartesian_impls! {
                @inner
                $a
                [$($b),*]
            }

            __cartesian_impls! {
                @double
                [$($a_),*]
                [$($b),*]
            }
        };
        ($($x:literal),*) => {
            __cartesian_impls! {
                @double
                [$($x),*]
                [$($x),*]
            }
        };
    }
    use __cartesian_impls;
}

#[cfg(test)]
mod tests {
    use core::ops::{Add, Neg, Sub};

    use super::*;

    #[test]
    fn can_add_const_numbers() {
        fn assert<A: Add<B, Output = O>, B, O>() {}

        assert::<N<0>, N<0>, N<0>>();

        assert::<N<0>, N<1>, N<1>>();
        assert::<N<1>, N<0>, N<1>>();

        assert::<N<0>, N<-1>, N<-1>>();
        assert::<N<-1>, N<0>, N<-1>>();

        assert::<N<1>, N<2>, N<3>>();
        assert::<N<2>, N<1>, N<3>>();

        assert::<N<1>, N<-2>, N<-1>>();
        assert::<N<2>, N<-1>, N<1>>();
    }

    #[test]
    fn can_sub_const_numbers() {
        fn assert<A: Sub<B, Output = O>, B, O>() {}

        assert::<N<0>, N<0>, N<0>>();

        assert::<N<0>, N<1>, N<-1>>();
        assert::<N<1>, N<0>, N<1>>();

        assert::<N<0>, N<-1>, N<1>>();
        assert::<N<-1>, N<0>, N<-1>>();

        assert::<N<1>, N<2>, N<-1>>();
        assert::<N<2>, N<1>, N<1>>();

        assert::<N<1>, N<-2>, N<3>>();
        assert::<N<2>, N<-1>, N<3>>();
    }

    #[test]
    fn can_negate_const_numbers() {
        fn assert<N: Neg<Output = O>, O>() {}

        assert::<N<-1>, N<1>>();
        assert::<N<0>, N<0>>();
        assert::<N<1>, N<-1>>();
    }
}
