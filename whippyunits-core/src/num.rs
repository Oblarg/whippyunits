//! Polyfill for generic const expressions.
//!
//! Because [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560)
//! is an untable feature (along with being incomplete), this polyfill exists to
//! allow a small subset of those operations on stable.
//!
//! Because dimensional analysis usually has small dimension exponents we constrain
//! this polyfill to working with input integers in the range -200 to 200. If the compiler
//! supports `generic_const_exprs` (nightly), it is automatically detected and used to
//! provide implementations for all `i16` integers.

/// Const generic with type level math operations.
///
/// Supported operations:
/// - Addition: `<A as Add<B>>::Output`
/// - Subtraction: `<A as Sub<B>>::Output`
/// - Negation: `<X as Neg>::Output`
///
/// On stable Rust, the operations are limited to inputs in the range -200 to 200.
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

#[cfg(not(has_generic_const_exprs))]
#[doc(hidden)]
pub trait __AsTypenum {
    type Repr: typenum::Integer;
}

#[cfg(not(has_generic_const_exprs))]
#[doc(hidden)]
pub trait __IntoNum {
    type Num;

    fn into_num() -> Self::Num;
}

#[cfg(has_generic_const_exprs)]
mod cge {
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

// Stable polyfill backed by typenum.
#[cfg(not(has_generic_const_exprs))]
mod stable {
    use super::{__AsTypenum, __IntoNum, N};
    use core::ops::{Add, Neg, Sub};
    use seq_macro::seq;

    impl __IntoNum for typenum::Z0 {
        type Num = N<0>;

        fn into_num() -> Self::Num {
            N
        }
    }

    impl __AsTypenum for N<0> {
        type Repr = typenum::Z0;
    }

    seq!(I in 1..=400 {
        impl __IntoNum for typenum::P~I {
            type Num = N<I>;

            fn into_num() -> Self::Num {
                N
            }
        }

        impl __IntoNum for typenum::N~I {
            type Num = N<{ -I }>;

            fn into_num() -> Self::Num {
                N
            }
        }
    });

    seq!(I in 1..=200 {
        impl __AsTypenum for N<I> {
            type Repr = typenum::P~I;
        }

        impl __AsTypenum for N<{ -I }> {
            type Repr = typenum::N~I;
        }
    });

    impl<const A: i16, const B: i16> Add<N<B>> for N<A>
    where
        N<A>: __AsTypenum,
        N<B>: __AsTypenum,
        <N<A> as __AsTypenum>::Repr: Add<<N<B> as __AsTypenum>::Repr>,
        <<N<A> as __AsTypenum>::Repr as Add<<N<B> as __AsTypenum>::Repr>>::Output: __IntoNum,
    {
        type Output =
            <<<N<A> as __AsTypenum>::Repr as Add<<N<B> as __AsTypenum>::Repr>>::Output as __IntoNum>::Num;

        fn add(self, _: N<B>) -> Self::Output {
            <<N<A> as __AsTypenum>::Repr as Add<<N<B> as __AsTypenum>::Repr>>::Output::into_num()
        }
    }

    impl<const A: i16, const B: i16> Sub<N<B>> for N<A>
    where
        N<A>: __AsTypenum,
        N<B>: __AsTypenum,
        <N<A> as __AsTypenum>::Repr: Sub<<N<B> as __AsTypenum>::Repr>,
        <<N<A> as __AsTypenum>::Repr as Sub<<N<B> as __AsTypenum>::Repr>>::Output: __IntoNum,
    {
        type Output =
            <<<N<A> as __AsTypenum>::Repr as Sub<<N<B> as __AsTypenum>::Repr>>::Output as __IntoNum>::Num;

        fn sub(self, _: N<B>) -> Self::Output {
            <<N<A> as __AsTypenum>::Repr as Sub<<N<B> as __AsTypenum>::Repr>>::Output::into_num()
        }
    }

    impl<const X: i16> Neg for N<X>
    where
        N<X>: __AsTypenum,
        <N<X> as __AsTypenum>::Repr: Neg,
        <<N<X> as __AsTypenum>::Repr as Neg>::Output: __IntoNum,
    {
        type Output = <<<N<X> as __AsTypenum>::Repr as Neg>::Output as __IntoNum>::Num;

        fn neg(self) -> Self::Output {
            <<N<X> as __AsTypenum>::Repr as Neg>::Output::into_num()
        }
    }
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

        // Stable range boundary checks
        assert::<N<200>, N<200>, N<400>>();
        assert::<N<-200>, N<-200>, N<-400>>();
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

        // Stable range boundary checks
        assert::<N<200>, N<-200>, N<400>>();
        assert::<N<-200>, N<200>, N<-400>>();
    }

    #[test]
    fn can_negate_const_numbers() {
        fn assert<N: Neg<Output = O>, O>() {}

        assert::<N<-1>, N<1>>();
        assert::<N<0>, N<0>>();
        assert::<N<1>, N<-1>>();
        assert::<N<200>, N<-200>>();
        assert::<N<-200>, N<200>>();
    }
}
