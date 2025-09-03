use crate::{
    IsIsize,
};
use std::ops::{Add, Div, Mul, Sub};
use crate::quantity_type::Quantity;
use crate::constants::*;
use crate::scale_conversion::*;
use crate::scale_resolution::*;

// ============================================================================
// Arithmetic Operations
// ============================================================================

// ============================================================================
// Scalar-Quantity Arithmetic Operations
// ============================================================================

#[rustfmt::skip]
impl<
    const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
    const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
>
    Mul<Quantity<
        MASS_EXPONENT, MASS_SCALE_P10,
        LENGTH_EXPONENT, LENGTH_SCALE_P10,
        TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
    >> for f64
{
    type Output = Quantity<
        MASS_EXPONENT, MASS_SCALE_P10,
        LENGTH_EXPONENT, LENGTH_SCALE_P10,
        TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
    >;

    fn mul(self: f64, other: Self::Output) -> Self::Output {
        let result_value = self * other.value;
        Self::Output::new(result_value)
    }
}

#[rustfmt::skip]
impl<
    const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
    const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
>
    Div<Quantity<
        MASS_EXPONENT, MASS_SCALE_P10,
        LENGTH_EXPONENT, LENGTH_SCALE_P10,
        TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
    >> for f64
    where
        (): IsIsize<{ -MASS_EXPONENT }>,
        (): IsIsize<{ -LENGTH_EXPONENT }>,
        (): IsIsize<{ -TIME_EXPONENT }>,
{
    type Output = Quantity<
        { -MASS_EXPONENT }, MASS_SCALE_P10,
        { -LENGTH_EXPONENT }, LENGTH_SCALE_P10,
        { -TIME_EXPONENT }, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
    >;

    fn div(self: f64, other: Quantity<
        MASS_EXPONENT, MASS_SCALE_P10,
        LENGTH_EXPONENT, LENGTH_SCALE_P10,
        TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
    >) -> Self::Output {
        let result_value = self / other.value;
        Self::Output::new(result_value)
    }
}


// ============================================================================
// Quantity-Scalar Arithmetic Operations
// ============================================================================

#[rustfmt::skip]
#[macro_export]
macro_rules! quantity_scalar_mul_div_interface {
    ($op:tt, $fn:ident, $trait:ident) => {
        impl<
            const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
            T,
        >
            $trait<f64> for Quantity<
                LENGTH_EXPONENT, LENGTH_SCALE_P10,
                MASS_EXPONENT, MASS_SCALE_P10,
                TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                T,
            >
        where
            T: Copy + $trait<f64, Output = T>,
        {
            type Output = Self;

            fn $fn(self, other: f64) -> Self::Output {
                Self::new(self.value $op other)
            }
        }
    }
}

quantity_scalar_mul_div_interface!(*, mul, Mul);
quantity_scalar_mul_div_interface!(/, div, Div);


// ============================================================================
// Quantity-Quantity Arithmetic Operations
// ============================================================================

// ============================================================================
// Add/Sub Output Type
// ============================================================================

#[rustfmt::skip]
#[macro_export]
macro_rules! add_sub_output_type {

    (
        LeftHandWins,
        $mass_exponent:ident, $mass_scale_p10_1:ident, $mass_scale_p10_2:ident,
        $length_exponent:ident, $length_scale_p10_1:ident, $length_scale_p10_2:ident,
        $time_exponent:ident, $time_scale_p2_1:ident, $time_scale_p3_1:ident, $time_scale_p5_1:ident,
                              $time_scale_p2_2:ident, $time_scale_p3_2:ident, $time_scale_p5_2:ident,
    ) => {
        Quantity::<
            $mass_exponent, $mass_scale_p10_1,
            $length_exponent, $length_scale_p10_1,
            $time_exponent, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1,
        >
    };

    (
        SmallerWins,
        $mass_exponent:ident, $mass_scale_p10_1:ident, $mass_scale_p10_2:ident,
        $length_exponent:ident, $length_scale_p10_1:ident, $length_scale_p10_2:ident,
        $time_exponent:ident, $time_scale_p2_1:ident, $time_scale_p3_1:ident, $time_scale_p5_1:ident,
                            $time_scale_p2_2:ident, $time_scale_p3_2:ident, $time_scale_p5_2:ident,
    ) => {
        Quantity::<
            $length_exponent, { min_length_scale($length_exponent, $length_scale_p10_1, $length_exponent, $length_scale_p10_2) },
            $mass_exponent, { min_mass_scale($mass_exponent, $mass_scale_p10_1, $mass_exponent, $mass_scale_p10_2) },
            $time_exponent, { min_time_scale(2, $time_exponent, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, 
                                              $time_exponent, $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2) },
                            { min_time_scale(3, $time_exponent, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, 
                                              $time_exponent, $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2) },
                            { min_time_scale(5, $time_exponent, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, 
                                              $time_exponent, $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2) }
        >
    };
}

// ============================================================================
// Mul/Div Output Type
// ============================================================================

#[rustfmt::skip]
#[macro_export]
macro_rules! mul_div_output_type {
    
    (
        LeftHandWins, $log_op:tt,
        $mass_exponent1:ident, $mass_scale_p10_1:ident, $mass_exponent2:ident, $mass_scale_p10_2:ident,
        $length_exponent1:ident, $length_scale_p10_1:ident, $length_exponent2:ident, $length_scale_p10_2:ident,
        $time_exponent1:ident, $time_scale_p2_1:ident, $time_scale_p3_1:ident, $time_scale_p5_1:ident,
        $time_exponent2:ident, $time_scale_p2_2:ident, $time_scale_p3_2:ident, $time_scale_p5_2:ident,
    ) => {
        type Output = Quantity::<
            { $mass_exponent1 $log_op $mass_exponent2 }, $mass_scale_p10_1,
            { $length_exponent1 $log_op $length_exponent2 }, $length_scale_p10_1,
            { $time_exponent1 $log_op $time_exponent2 }, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1,
        >;
    };

    (
        SmallerWins, $op:tt,
        $mass_exponent1:ident, $mass_scale_p10_1:ident, $mass_exponent2:ident, $mass_scale_p10_2:ident,
        $length_exponent1:ident, $length_scale_p10_1:ident, $length_exponent2:ident, $length_scale_p10_2:ident,
        $time_exponent1:ident, $time_scale_p2_1:ident, $time_scale_p3_1:ident, $time_scale_p5_1:ident,
        $time_exponent2:ident, $time_scale_p2_2:ident, $time_scale_p3_2:ident, $time_scale_p5_2:ident,
    ) => {
        Quantity::<
                        { $mass_exponent1 $op $mass_exponent2 }, { min_mass_scale($mass_exponent1, $mass_scale_p10_1, $mass_exponent2, $mass_scale_p10_2) },
            { $length_exponent1 $op $length_exponent2 }, { min_length_scale($length_exponent1, $length_scale_p10_1, $length_exponent2, $length_scale_p10_2) },
            { $time_exponent1 $op $time_exponent2 }, { min_time_scale(2, $time_exponent1, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, 
                                                      $time_exponent2, $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2) },
                                             { min_time_scale(3, $time_exponent1, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, 
                                                      $time_exponent2, $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2) },
                                             { min_time_scale(5, $time_exponent1, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, 
                                                      $time_exponent2, $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2) },
                                             { min_time_scale(0, $time_exponent1, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, 
                                                      $time_exponent2, $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2) },
        >
    };
}

// ============================================================================
// Add/Sub Interface
// ============================================================================

#[rustfmt::skip]
#[macro_export]
macro_rules! add_sub_interface {

    // ============================================================================
    // Strict Interface (Measurement Scales Must Match)
    // ============================================================================

    (Strict, $op:tt, $fn:ident, $trait:ident) => {
        impl<
            const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
            T,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10,
                    TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                    T,
                >,
            >
            for Quantity<
                MASS_EXPONENT, MASS_SCALE_P10,
                LENGTH_EXPONENT, LENGTH_SCALE_P10,
                TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                T,
            >
        where
            T: Copy + $trait<T, Output = T>,
        {
            type Output = Self;

            fn $fn(
                self,
                other: Self,
            ) -> Self::Output {
                Self::new(self.value $op other.value)
            }
        }
    };

    // ============================================================================
    // Non-Strict Interface (Measurement Scales Can Differ)
    // ============================================================================

    (LeftHandWins, $op:tt, $fn:ident, $trait:ident) => {
        impl<
            const MASS_EXPONENT: isize, const MASS_SCALE_P10_1: isize, const MASS_SCALE_P10_2: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10_1: isize, const LENGTH_SCALE_P10_2: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2_1: isize, const TIME_SCALE_P3_1: isize, const TIME_SCALE_P5_1: isize,
                                        const TIME_SCALE_P2_2: isize, const TIME_SCALE_P3_2: isize, const TIME_SCALE_P5_2: isize,
            T,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    T,
                >,
            >
            for Quantity<
                MASS_EXPONENT, MASS_SCALE_P10_1,
                LENGTH_EXPONENT, LENGTH_SCALE_P10_1,
                TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                T,
            >
        where
            T: Copy + $trait<T, Output = T> + Mul<f64, Output = T>,
        {
            type Output = Self;

            fn $fn(
                self,
                other: Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    T,
                >,
            ) -> Self::Output {
               let factor = aggregate_conversion_factor(
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_1, LENGTH_SCALE_P10_2,
                    MASS_EXPONENT, MASS_SCALE_P10_1, MASS_SCALE_P10_2,
                    TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                    TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
               );
               Self::Output::new(self.value $op (other.value * factor))
            }
        }
    };

    (SmallerWins, $op:tt, $fn:ident, $trait:ident) => {
        impl<
            const MASS_EXPONENT: isize, const MASS_SCALE_P10_1: isize, const MASS_SCALE_P10_2: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10_1: isize, const LENGTH_SCALE_P10_2: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2_1: isize, const TIME_SCALE_P3_1: isize, const TIME_SCALE_P5_1: isize,
                                        const TIME_SCALE_P2_2: isize, const TIME_SCALE_P3_2: isize, const TIME_SCALE_P5_2: isize,
            T,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    T,
                >,
            >
            for Quantity<
                MASS_EXPONENT, MASS_SCALE_P10_1,
                LENGTH_EXPONENT, LENGTH_SCALE_P10_1,
                TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                T,
            >
        where
            (): IsIsize<{ min_mass_scale(MASS_EXPONENT, MASS_SCALE_P10_1, MASS_EXPONENT, MASS_SCALE_P10_2) }>,
            (): IsIsize<{ min_length_scale(LENGTH_EXPONENT, LENGTH_SCALE_P10_1, LENGTH_EXPONENT, LENGTH_SCALE_P10_2) }>,
            (): IsIsize<{ min_time_scale(2, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                               TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsIsize<{ min_time_scale(3, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                               TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsIsize<{ min_time_scale(5, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                               TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            T: Copy + $trait<T, Output = T> + Mul<f64, Output = T>,
        {
            type Output = Quantity<
                MASS_EXPONENT, { min_mass_scale(MASS_EXPONENT, MASS_SCALE_P10_1, MASS_EXPONENT, MASS_SCALE_P10_2) },
                LENGTH_EXPONENT, { min_length_scale(LENGTH_EXPONENT, LENGTH_SCALE_P10_1, LENGTH_EXPONENT, LENGTH_SCALE_P10_2) },
                TIME_EXPONENT, { min_time_scale(2, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                   TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                               { min_time_scale(3, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                   TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                               { min_time_scale(5, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                   TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                            >;

            fn $fn(
                self,
                other: Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    T,
                >,
            ) -> Self::Output {
                let result_mass_scale_p10 = min_mass_scale(MASS_EXPONENT, MASS_SCALE_P10_1, MASS_EXPONENT, MASS_SCALE_P10_2);
                let result_length_scale_p10 = min_length_scale(LENGTH_EXPONENT, LENGTH_SCALE_P10_1, LENGTH_EXPONENT, LENGTH_SCALE_P10_2);
                let result_time_scale_p2 = min_time_scale(2, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2);
                let result_time_scale_p3 = min_time_scale(3, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2);
                let result_time_scale_p5 = min_time_scale(5, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2);
                
                let factor1 = aggregate_conversion_factor(
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_1, result_length_scale_p10,
                    MASS_EXPONENT, MASS_SCALE_P10_1, result_mass_scale_p10,
                    TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                    result_time_scale_p2, result_time_scale_p3, result_time_scale_p5,
                );
                let factor2 = aggregate_conversion_factor(
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_2, result_length_scale_p10,
                    MASS_EXPONENT, MASS_SCALE_P10_2, result_mass_scale_p10,
                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                                    result_time_scale_p2, result_time_scale_p3, result_time_scale_p5,
                );

                Self::Output::new((self.value * factor1) $op (other.value * factor2))
            }
        }
    }
}

// ============================================================================
// Mul/Div Interface
// ============================================================================

#[rustfmt::skip]
#[macro_export]
macro_rules! mul_div_interface {

    // ============================================================================
    // Strict Interface (Measurement Scales Must Match)
    // ============================================================================

    (Strict, $op:tt, $log_op:tt, $fn:ident, $trait:ident) => {
        impl<
            const MASS_EXPONENT1: isize, const MASS_EXPONENT2: isize, const MASS_SCALE_P10: isize,
            const LENGTH_EXPONENT1: isize, const LENGTH_EXPONENT2: isize, const LENGTH_SCALE_P10: isize,
            const TIME_EXPONENT1: isize, const TIME_EXPONENT2: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
            T,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT2, MASS_SCALE_P10,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10,
                    TIME_EXPONENT2, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                    T,
                >,
            >
            for Quantity<
                MASS_EXPONENT1, MASS_SCALE_P10,
                LENGTH_EXPONENT1, LENGTH_SCALE_P10,
                TIME_EXPONENT1, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                T,
            >
        where
            (): IsIsize<{ MASS_EXPONENT1 $log_op MASS_EXPONENT2 }>,
            (): IsIsize<{ LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }>,
            (): IsIsize<{ TIME_EXPONENT1 $log_op TIME_EXPONENT2 }>,
            T: Copy + $trait<T, Output = T>,
        {
            type Output = Quantity<
                { MASS_EXPONENT1 $log_op MASS_EXPONENT2 }, MASS_SCALE_P10,
                { LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }, LENGTH_SCALE_P10,
                { TIME_EXPONENT1 $log_op TIME_EXPONENT2 }, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                T,
            >;

            fn $fn(
                self,
                other: Quantity<
                    MASS_EXPONENT2, MASS_SCALE_P10,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10,
                    TIME_EXPONENT2, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                    T,
                >,
            ) -> Self::Output {
                Self::Output::new(self.value $op other.value)
            }
        }
    };

    // ============================================================================
    // Non-Strict Interface (Measurement Scales Can Differ)
    // ============================================================================

    (LeftHandWins, $op:tt, $log_op:tt, $fn:ident, $trait:ident) => {
        impl<
            const MASS_EXPONENT1: isize, const MASS_SCALE_P10_1: isize,
            const LENGTH_EXPONENT1: isize, const LENGTH_SCALE_P10_1: isize,
            const TIME_EXPONENT1: isize, const TIME_SCALE_P2_1: isize, const TIME_SCALE_P3_1: isize, const TIME_SCALE_P5_1: isize,
            const MASS_EXPONENT2: isize, const MASS_SCALE_P10_2: isize,
            const LENGTH_EXPONENT2: isize, const LENGTH_SCALE_P10_2: isize,
            const TIME_EXPONENT2: isize, const TIME_SCALE_P2_2: isize, const TIME_SCALE_P3_2: isize, const TIME_SCALE_P5_2: isize,
            T,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT2, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    T,
                >,
            >
            for Quantity<
                MASS_EXPONENT1, MASS_SCALE_P10_1,
                LENGTH_EXPONENT1, LENGTH_SCALE_P10_1,
                TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                T,
            >
        where
            (): IsIsize<{ MASS_EXPONENT1 $log_op MASS_EXPONENT2 }>,
            (): IsIsize<{ LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }>,
            (): IsIsize<{ TIME_EXPONENT1 $log_op TIME_EXPONENT2 }>,
            (): IsIsize<{ left_hand_wins_scale(LENGTH_SCALE_P10_1, LENGTH_SCALE_P10_2) }>,
            (): IsIsize<{ left_hand_wins_scale(MASS_SCALE_P10_1, MASS_SCALE_P10_2) }>,
            (): IsIsize<{ left_hand_wins_scale(TIME_SCALE_P2_1, TIME_SCALE_P2_2) }>,
            (): IsIsize<{ left_hand_wins_scale(TIME_SCALE_P3_1, TIME_SCALE_P3_2) }>,
            (): IsIsize<{ left_hand_wins_scale(TIME_SCALE_P5_1, TIME_SCALE_P5_2) }>,
            T: Copy + $trait<T, Output = T> + Mul<f64, Output = T>,
        {
            type Output = Quantity<
                { MASS_EXPONENT1 $log_op MASS_EXPONENT2 }, { left_hand_wins_scale(MASS_SCALE_P10_1, MASS_SCALE_P10_2) },
                { LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }, { left_hand_wins_scale(LENGTH_SCALE_P10_1, LENGTH_SCALE_P10_2) },
                { TIME_EXPONENT1 $log_op TIME_EXPONENT2 }, 
                { left_hand_wins_scale(TIME_SCALE_P2_1, TIME_SCALE_P2_2) },
                { left_hand_wins_scale(TIME_SCALE_P3_1, TIME_SCALE_P3_2) },
                { left_hand_wins_scale(TIME_SCALE_P5_1, TIME_SCALE_P5_2) },
                T,
            >;

            fn $fn(
                self,
                other: Quantity<
                    MASS_EXPONENT2, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    T,
                >,
            ) -> Self::Output {
                Self::Output::new(self.value $op (other.value * aggregate_conversion_factor(
                    MASS_EXPONENT2, MASS_SCALE_P10_2, MASS_SCALE_P10_1,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10_2, LENGTH_SCALE_P10_1,
                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P2_1, TIME_SCALE_P3_2, TIME_SCALE_P5_2, 
                                    TIME_SCALE_P3_1, TIME_SCALE_P3_2, TIME_SCALE_P5_1, TIME_SCALE_P5_2,
                )))
            }
        }
    };

    (SmallerWins, $op:tt, $log_op:tt, $fn:ident, $trait:ident) => {
        impl<
            const MASS_EXPONENT1: isize, const MASS_SCALE_P10_1: isize,
            const LENGTH_EXPONENT1: isize, const LENGTH_SCALE_P10_1: isize,
            const TIME_EXPONENT1: isize, const TIME_SCALE_P2_1: isize, const TIME_SCALE_P3_1: isize, const TIME_SCALE_P5_1: isize,
            const MASS_EXPONENT2: isize, const MASS_SCALE_P10_2: isize,
            const LENGTH_EXPONENT2: isize, const LENGTH_SCALE_P10_2: isize,
            const TIME_EXPONENT2: isize, const TIME_SCALE_P2_2: isize, const TIME_SCALE_P3_2: isize, const TIME_SCALE_P5_2: isize,
            T,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT2, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    T,
                >,
            >
            for Quantity<
                MASS_EXPONENT1, MASS_SCALE_P10_1,
                LENGTH_EXPONENT1, LENGTH_SCALE_P10_1,
                TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                T,
            >
        where
            (): IsIsize<{ MASS_EXPONENT1 $log_op MASS_EXPONENT2 }>,
            (): IsIsize<{ LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }>,
            (): IsIsize<{ TIME_EXPONENT1 $log_op TIME_EXPONENT2 }>,
            (): IsIsize<{ min_length_scale(LENGTH_EXPONENT1, LENGTH_SCALE_P10_1, LENGTH_EXPONENT2, LENGTH_SCALE_P10_2) }>,
            (): IsIsize<{ min_mass_scale(MASS_EXPONENT1, MASS_SCALE_P10_1, MASS_EXPONENT2, MASS_SCALE_P10_2) }>,
            (): IsIsize<{ min_time_scale(2, TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                            TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsIsize<{ min_time_scale(3, TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                            TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsIsize<{ min_time_scale(5, TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                            TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            (): IsIsize<{ min_time_scale(0, TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                            TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
            T: Copy + $trait<T, Output = T> + Mul<f64, Output = T>,
        {
            type Output = Quantity<
                { MASS_EXPONENT1 $log_op MASS_EXPONENT2 }, { min_mass_scale(MASS_EXPONENT1, MASS_SCALE_P10_1, MASS_EXPONENT2, MASS_SCALE_P10_2) },
                { LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }, { min_length_scale(LENGTH_EXPONENT1, LENGTH_SCALE_P10_1, LENGTH_EXPONENT2, LENGTH_SCALE_P10_2) },
                { TIME_EXPONENT1 $log_op TIME_EXPONENT2 }, { min_time_scale(2, TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                                               TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                                                           { min_time_scale(3, TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                                               TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                                                           { min_time_scale(5, TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                                               TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                                                           { min_time_scale(0, TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                                               TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                T,
            >;

            fn $fn(
                self,
                other: Quantity<
                    MASS_EXPONENT2, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    T,
                >,
            ) -> Self::Output {
                let result_mass_scale_p10 = min_mass_scale(MASS_EXPONENT1, MASS_SCALE_P10_1, MASS_EXPONENT2, MASS_SCALE_P10_2);
                let result_length_scale_p10 = min_length_scale(LENGTH_EXPONENT1, LENGTH_SCALE_P10_1, LENGTH_EXPONENT2, LENGTH_SCALE_P10_2);
                let result_time_scale_p2 = min_time_scale(2, TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2);
                let result_time_scale_p3 = min_time_scale(3, TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2);
                let result_time_scale_p5 = min_time_scale(5, TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2);
                
                let factor1 = aggregate_conversion_factor(
                    MASS_EXPONENT1, MASS_SCALE_P10_1, result_mass_scale_p10,
                    LENGTH_EXPONENT1, LENGTH_SCALE_P10_1, result_length_scale_p10,
                    TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                    result_time_scale_p2, result_time_scale_p3, result_time_scale_p5,
                );
                let factor2 = aggregate_conversion_factor(
                    MASS_EXPONENT2, MASS_SCALE_P10_2, result_mass_scale_p10,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10_2, result_length_scale_p10,
                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                                    result_time_scale_p2, result_time_scale_p3, result_time_scale_p5,
                );

                Self::Output::new((self.value * factor1) $op (other.value * factor2))
            }
        }
    }
}


// ============================================================================
// Generate Arithmetic Implementations
// ============================================================================

#[macro_export]
macro_rules! generate_arithmetic_ops {
    (Strict) => {
        // ============================================================================
        // Addition/Subtraction
        // ============================================================================

        add_sub_interface!(Strict, +, add, Add);
        add_sub_interface!(Strict, -, sub, Sub);

        // ============================================================================
        // Multiplication/Division
        // ============================================================================

        mul_div_interface!(Strict, *, +, mul, Mul);
        mul_div_interface!(Strict, /, -, div, Div);
    };

    ($rescale_behavior:ident) => {
        // ============================================================================
        // Addition/Subtraction
        // ============================================================================

        add_sub_interface!($rescale_behavior, +, add, Add);
        add_sub_interface!($rescale_behavior, -, sub, Sub);

        // ============================================================================
        // Multiplication/Division
        // ============================================================================

        mul_div_interface!($rescale_behavior, *, +, mul, Mul);
        mul_div_interface!($rescale_behavior, /, -, div, Div);
    };
}

#[cfg(feature = "strict")]
generate_arithmetic_ops!(Strict);

// Default if no feature is specified
#[cfg(not(any(feature = "strict", feature = "smaller_wins", feature = "left_hand_wins")))]
generate_arithmetic_ops!(Strict);