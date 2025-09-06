use crate::{
    IsIsize,
};
use std::ops::{Add, Div, Mul, Sub, AddAssign, SubAssign, MulAssign, DivAssign};
use crate::quantity_type::Quantity;

#[rustfmt::skip]
#[macro_export]
macro_rules! scalar_quantity_mul_div_interface {
    ($T:ty) => {
        impl<
            const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
        >
            Mul<Quantity<
                MASS_EXPONENT, MASS_SCALE_P10,
                LENGTH_EXPONENT, LENGTH_SCALE_P10,
                TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                $T,
            >> for $T
        {
            type Output = Quantity<
                MASS_EXPONENT, MASS_SCALE_P10,
                LENGTH_EXPONENT, LENGTH_SCALE_P10,
                TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                $T,
            >;
        
            fn mul(self: $T, other: Self::Output) -> Self::Output {
                let result_value = self * other.value;
                Self::Output::new(result_value)
            }
        }

        impl<
            const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
        >
            Div<Quantity<
                MASS_EXPONENT, MASS_SCALE_P10,
                LENGTH_EXPONENT, LENGTH_SCALE_P10,
                TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            >> for $T
            where
                (): IsIsize<{ -MASS_EXPONENT }>,
                (): IsIsize<{ -LENGTH_EXPONENT }>,
                (): IsIsize<{ -TIME_EXPONENT }>,
        {
            type Output = Quantity<
                { -MASS_EXPONENT }, MASS_SCALE_P10,
                { -LENGTH_EXPONENT }, LENGTH_SCALE_P10,
                { -TIME_EXPONENT }, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                $T,
            >;

            fn div(self: $T, other: Quantity<
                MASS_EXPONENT, MASS_SCALE_P10,
                LENGTH_EXPONENT, LENGTH_SCALE_P10,
                TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            >) -> Self::Output {
                let result_value = self / other.value;
                Self::Output::new(result_value)
            }
        }
    }
}

#[rustfmt::skip]
#[macro_export]
macro_rules! quantity_scalar_mul_div_interface {
    ($op:tt, $fn:ident, $trait:ident, $T:ty) => {
        impl<
            const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
        >
            $trait<$T> for Quantity<
                LENGTH_EXPONENT, LENGTH_SCALE_P10,
                MASS_EXPONENT, MASS_SCALE_P10,
                TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                $T,
            >
        {
            type Output = Self;

            fn $fn(self, other: f64) -> Self::Output {
                Self::new(self.value $op other)
            }
        }
    }
}

macro_rules! quantity_scalar_mul_div_assign_interface {
    ($op:tt, $fn:ident, $trait:ident, $T:ty) => {
        impl<
            const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
        >
            $trait<$T> for Quantity<
                LENGTH_EXPONENT, LENGTH_SCALE_P10,
                MASS_EXPONENT, MASS_SCALE_P10,
                TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                $T,
            >
        {
            fn $fn(&mut self, other: $T) {
                self.value $op other;
            }
        }
    }
}


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
        $T:ty,
    ) => {
        type Output = Quantity::<
            { $mass_exponent1 $log_op $mass_exponent2 }, $mass_scale_p10_1,
            { $length_exponent1 $log_op $length_exponent2 }, $length_scale_p10_1,
            { $time_exponent1 $log_op $time_exponent2 }, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1,
            $T,
        >;
    };

    (
        SmallerWins, $op:tt,
        $mass_exponent1:ident, $mass_scale_p10_1:ident, $mass_exponent2:ident, $mass_scale_p10_2:ident,
        $length_exponent1:ident, $length_scale_p10_1:ident, $length_exponent2:ident, $length_scale_p10_2:ident,
        $time_exponent1:ident, $time_scale_p2_1:ident, $time_scale_p3_1:ident, $time_scale_p5_1:ident,
        $time_exponent2:ident, $time_scale_p2_2:ident, $time_scale_p3_2:ident, $time_scale_p5_2:ident,
        $T:ty,
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
            $T,
        >
    };
}

#[rustfmt::skip]
#[macro_export]
macro_rules! quantity_quantity_add_sub_interface {
    // Strict interface (measurement scales must match) (only one set of scale parameters)
    (Strict, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident) => {
        impl<
            const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10,
                    TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                    $T,
                >,
            >
            for Quantity<
                MASS_EXPONENT, MASS_SCALE_P10,
                LENGTH_EXPONENT, LENGTH_SCALE_P10,
                TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                $T,
            >
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

    // Non-Strict interface (measurement scales can differ) (two sets of scale parameters)
    (LeftHandWins, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident) => {
        impl<
            const MASS_EXPONENT: isize, const MASS_SCALE_P10_1: isize, const MASS_SCALE_P10_2: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10_1: isize, const LENGTH_SCALE_P10_2: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2_1: isize, const TIME_SCALE_P3_1: isize, const TIME_SCALE_P5_1: isize,
                                        const TIME_SCALE_P2_2: isize, const TIME_SCALE_P3_2: isize, const TIME_SCALE_P5_2: isize,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    $T,
                >,
            >
            for Quantity<
                MASS_EXPONENT, MASS_SCALE_P10_1,
                LENGTH_EXPONENT, LENGTH_SCALE_P10_1,
                TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                $T,
            >
        {
            type Output = Self;

            fn $fn(
                self,
                other: Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    $T,
                >,
            ) -> Self::Output {
               const rescaled_other: Self = $rescale_fn(other);
               Self::Output::new(self.value $op rescaled_other.value)
            }
        }
    };

    // Non-Strict interface (measurement scales can differ) (two sets of scale parameters)
    (SmallerWins, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident) => {
        impl<
            const MASS_EXPONENT: isize, const MASS_SCALE_P10_1: isize, const MASS_SCALE_P10_2: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10_1: isize, const LENGTH_SCALE_P10_2: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2_1: isize, const TIME_SCALE_P3_1: isize, const TIME_SCALE_P5_1: isize,
                                        const TIME_SCALE_P2_2: isize, const TIME_SCALE_P3_2: isize, const TIME_SCALE_P5_2: isize,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    $T,
                >,
            >
            for Quantity<
                MASS_EXPONENT, MASS_SCALE_P10_1,
                LENGTH_EXPONENT, LENGTH_SCALE_P10_1,
                TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                $T,
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
                $T,
            >;

            fn $fn(
                self,
                other: Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    $T,
                >,
            ) -> Self::Output {
                let rescaled_self: Self::Output = $rescale_fn(self);
                let rescaled_other: Self::Output = $rescale_fn(other);

                Self::Output::new(rescaled_self.value $op rescaled_other.value)
            }
        }
    }
}

// AddAssign/SubAssign must return the same type as the left-hand side, so it only supports strict or left-hand-wins rescale semantics
#[rustfmt::skip]
#[macro_export]
macro_rules! quantity_quantity_add_sub_assign_interface {
    // Strict interface (measurement scales must match) (only one set of scale parameters)
    (Strict, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident) => {
        impl<
            const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10,
                    TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                    $T,
                >,
            > for Quantity<
                MASS_EXPONENT, MASS_SCALE_P10,
                LENGTH_EXPONENT, LENGTH_SCALE_P10,
                TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                $T,
            >
        {
            fn $fn(&mut self, other: Quantity<
                MASS_EXPONENT, MASS_SCALE_P10,
                LENGTH_EXPONENT, LENGTH_SCALE_P10,
                TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                $T,
            >) {
                self.value $op other.value;
            }
        }
    };

    // Non-Strict interface (measurement scales can differ) (two sets of scale parameters)
    // All non-strict modes adopt LeftHandWins for AddAssign/SubAssign
    ($rescale_behavior:ident, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident) => {
        impl 
            const MASS_EXPONENT: isize, const MASS_SCALE_P10_1: isize, const MASS_SCALE_P10_2: isize,
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10_1: isize, const LENGTH_SCALE_P10_2: isize,
            const TIME_EXPONENT: isize, const TIME_SCALE_P2_1: isize, const TIME_SCALE_P3_1: isize, const TIME_SCALE_P5_1: isize,
            const TIME_SCALE_P2_2: isize, const TIME_SCALE_P3_2: isize, const TIME_SCALE_P5_2: isize,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    $T,
                >,
            > for Quantity<
                MASS_EXPONENT, MASS_SCALE_P10_1,
                LENGTH_EXPONENT, LENGTH_SCALE_P10_1,
                TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                $T,
            >
        {
            fn $fn(&mut self, other: Quantity<
                MASS_EXPONENT, MASS_SCALE_P10_1,
                LENGTH_EXPONENT, LENGTH_SCALE_P10_1,
                TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                $T,
            >) {
                let rescaled_other: Self = $rescale_fn(other);
                self.value $op rescaled_other.value;
            }
        }
    };
}

#[rustfmt::skip]
#[macro_export]
macro_rules! quantity_quantity_mul_div_interface {
    // Strict interface (measurement scales must match) (only one set of scale parameters)
    (Strict, $op:tt, $log_op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident) => {
        impl<
            const MASS_EXPONENT1: isize, const MASS_EXPONENT2: isize, const MASS_SCALE_P10: isize,
            const LENGTH_EXPONENT1: isize, const LENGTH_EXPONENT2: isize, const LENGTH_SCALE_P10: isize,
            const TIME_EXPONENT1: isize, const TIME_EXPONENT2: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT2, MASS_SCALE_P10,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10,
                    TIME_EXPONENT2, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                    $T,
                >,
            >
            for Quantity<
                MASS_EXPONENT1, MASS_SCALE_P10,
                LENGTH_EXPONENT1, LENGTH_SCALE_P10,
                TIME_EXPONENT1, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                $T,
            >
        where
            (): IsIsize<{ MASS_EXPONENT1 $log_op MASS_EXPONENT2 }>,
            (): IsIsize<{ LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }>,
            (): IsIsize<{ TIME_EXPONENT1 $log_op TIME_EXPONENT2 }>,
        {
            type Output = Quantity<
                { MASS_EXPONENT1 $log_op MASS_EXPONENT2 }, MASS_SCALE_P10,
                { LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }, LENGTH_SCALE_P10,
                { TIME_EXPONENT1 $log_op TIME_EXPONENT2 }, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                $T,
            >;

            fn $fn(
                self,
                other: Quantity<
                    MASS_EXPONENT2, MASS_SCALE_P10,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10,
                    TIME_EXPONENT2, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
                    $T,
                >,
            ) -> Self::Output {
                Self::Output::new(self.value $op other.value)
            }
        }
    };

    // Non-Strict interface (measurement scales can differ) (two sets of scale parameters)
    (LeftHandWins, $op:tt, $log_op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident) => {
        impl<
            const MASS_EXPONENT1: isize, const MASS_SCALE_P10_1: isize,
            const LENGTH_EXPONENT1: isize, const LENGTH_SCALE_P10_1: isize,
            const TIME_EXPONENT1: isize, const TIME_SCALE_P2_1: isize, const TIME_SCALE_P3_1: isize, const TIME_SCALE_P5_1: isize,
            const MASS_EXPONENT2: isize, const MASS_SCALE_P10_2: isize,
            const LENGTH_EXPONENT2: isize, const LENGTH_SCALE_P10_2: isize,
            const TIME_EXPONENT2: isize, const TIME_SCALE_P2_2: isize, const TIME_SCALE_P3_2: isize, const TIME_SCALE_P5_2: isize,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT2, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    $T,
                >,
            >
            for Quantity<
                MASS_EXPONENT1, MASS_SCALE_P10_1,
                LENGTH_EXPONENT1, LENGTH_SCALE_P10_1,
                TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                $T,
            >
        where
            (): IsIsize<{ MASS_EXPONENT1 $log_op MASS_EXPONENT2 }>,
            (): IsIsize<{ LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }>,
            (): IsIsize<{ TIME_EXPONENT1 $log_op TIME_EXPONENT2 }>,
        {
            type Output = Quantity<
                { MASS_EXPONENT1 $log_op MASS_EXPONENT2 }, MASS_SCALE_P10_1,
                { LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }, LENGTH_SCALE_P10_1,
                { TIME_EXPONENT1 $log_op TIME_EXPONENT2 }, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                $T,
            >;
            
            type OutputScaleOther = Quantity<
                MASS_EXPONENT2, MASS_SCALE_P10_1,
                LENGTH_EXPONENT2, LENGTH_SCALE_P10_1,
                TIME_EXPONENT2, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                $T,
            >;

            fn $fn(
                self,
                other: Quantity<
                    MASS_EXPONENT2, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    $T,
                >,
            ) -> Self::Output {
                let rescaled_other: OutputScaleOther = $rescale_fn(other);
                Self::Output::new(self.value $op rescaled_other.value)
            }
        }
    };

    // Non-Strict interface (measurement scales can differ) (two sets of scale parameters)
    (SmallerWins, $op:tt, $log_op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident) => {
        impl<
            const MASS_EXPONENT1: isize, const MASS_SCALE_P10_1: isize,
            const LENGTH_EXPONENT1: isize, const LENGTH_SCALE_P10_1: isize,
            const TIME_EXPONENT1: isize, const TIME_SCALE_P2_1: isize, const TIME_SCALE_P3_1: isize, const TIME_SCALE_P5_1: isize,
            const MASS_EXPONENT2: isize, const MASS_SCALE_P10_2: isize,
            const LENGTH_EXPONENT2: isize, const LENGTH_SCALE_P10_2: isize,
            const TIME_EXPONENT2: isize, const TIME_SCALE_P2_2: isize, const TIME_SCALE_P3_2: isize, const TIME_SCALE_P5_2: isize,
        >
            $trait<
                Quantity<
                    MASS_EXPONENT2, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    $T,
                >,
            >
            for Quantity<
                MASS_EXPONENT1, MASS_SCALE_P10_1,
                LENGTH_EXPONENT1, LENGTH_SCALE_P10_1,
                TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                $T,
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
                $T,
            >;

            type OutputScaleSelf = Quantity<
                MASS_EXPONENT1, { min_mass_scale(MASS_EXPONENT1, MASS_SCALE_P10_1, MASS_EXPONENT2, MASS_SCALE_P10_2) },
                LENGTH_EXPONENT1, { min_length_scale(LENGTH_EXPONENT1, LENGTH_SCALE_P10_1, LENGTH_EXPONENT2, LENGTH_SCALE_P10_2) },
                TIME_EXPONENT1, { min_time_scale(2, TIME_EXPONENT1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1, 
                                                   TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) },
                $T,
            >;

            type OutputScaleOther = Quantity<
                MASS_EXPONENT2, MASS_SCALE_P10_2,
                LENGTH_EXPONENT2, LENGTH_SCALE_P10_2,
                TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                $T,
            >;

            fn $fn(
                self,
                other: Quantity<
                    MASS_EXPONENT2, MASS_SCALE_P10_2,
                    LENGTH_EXPONENT2, LENGTH_SCALE_P10_2,
                    TIME_EXPONENT2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2,
                    $T,
                >,
            ) -> Self::Output {
                let rescaled_self: OutputScaleSelf = $rescale_fn(self);
                let rescaled_other: OutputScaleOther = $rescale_fn(other);

                Self::Output::new(rescaled_self.value $op rescaled_other.value)
            }
        }
    };
}

#[macro_export]
macro_rules! generate_arithmetic_ops {
    ($rescale_behavior:ident, $T:ty, $rescale_fn:ident) => {
        // scalar-quantity arithmetic operations
        scalar_quantity_mul_div_interface!($T);

        quantity_scalar_mul_div_interface!(*, mul, Mul, $T);
        quantity_scalar_mul_div_interface!(/, div, Div, $T);

        quantity_scalar_mul_div_assign_interface!(*=, mul_assign, MulAssign, $T);
        quantity_scalar_mul_div_assign_interface!(/=, div_assign, DivAssign, $T);

        // quantity-quantity arithmetic operations
        quantity_quantity_add_sub_interface!($rescale_behavior, +, add, Add, $T, $rescale_fn);
        quantity_quantity_add_sub_interface!($rescale_behavior, -, sub, Sub, $T, $rescale_fn);

        quantity_quantity_add_sub_assign_interface!($rescale_behavior, +=, add_assign, AddAssign, $T, $rescale_fn);
        quantity_quantity_add_sub_assign_interface!($rescale_behavior, -=, sub_assign, SubAssign, $T, $rescale_fn);

        quantity_quantity_mul_div_interface!($rescale_behavior, *, +, mul, Mul, $T, $rescale_fn);
        quantity_quantity_mul_div_interface!($rescale_behavior, /, -, div, Div, $T, $rescale_fn);
    };
}

#[cfg(feature = "strict")]
generate_arithmetic_ops!(Strict, f64, rescale_f64);

// Default if no feature is specified
#[cfg(not(any(feature = "strict", feature = "smaller_wins", feature = "left_hand_wins")))]
generate_arithmetic_ops!(Strict);

