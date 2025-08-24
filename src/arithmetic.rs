use core::ops::{Add, Div, Mul, Sub};

use crate::{
    Quantity, IsIsize,
};

// ============================================================================
// Arithmetic Operations
// ============================================================================

// ============================================================================
// Scalar-Quantity Arithmetic Operations
// ============================================================================


impl<
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE: isize,
    const MASS_EXPONENT: isize, const MASS_SCALE: isize,
    const TIME_EXPONENT: isize, const TIME_P2: isize, const TIME_P3: isize, const TIME_P5: isize, const TIME_SCALE_ORDER: isize,
>
    Mul<Quantity<
        LENGTH_EXPONENT, LENGTH_SCALE,
        MASS_EXPONENT, MASS_SCALE,
        TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
    >> for f64
{
    type Output = Quantity<
        LENGTH_EXPONENT, LENGTH_SCALE,
        MASS_EXPONENT, MASS_SCALE,
        TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
    >;

    fn mul(self: f64, other: Self::Output) -> Self::Output {
        let result_value = self * other.value;
        Self::Output::new(result_value)
    }
}

impl<
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE: isize,
    const MASS_EXPONENT: isize, const MASS_SCALE: isize,
    const TIME_EXPONENT: isize, const TIME_P2: isize, const TIME_P3: isize, const TIME_P5: isize, const TIME_SCALE_ORDER: isize,
>
    Div<Quantity<
        LENGTH_EXPONENT, LENGTH_SCALE,
        MASS_EXPONENT, MASS_SCALE,
        TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
    >> for f64
    where
        (): IsIsize<{  -LENGTH_EXPONENT }>,
        (): IsIsize<{ -MASS_EXPONENT }>,
        (): IsIsize<{ -TIME_EXPONENT }>,
{
    type Output = Quantity<
      { -LENGTH_EXPONENT }, LENGTH_SCALE,
        { -MASS_EXPONENT }, MASS_SCALE,
        { -TIME_EXPONENT }, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
    >;

    fn div(self: f64, other: Quantity<
        LENGTH_EXPONENT, LENGTH_SCALE,
        MASS_EXPONENT, MASS_SCALE,
        TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
    >) -> Self::Output {
        let result_value = self / other.value;
        Self::Output::new(result_value)
    }
}


// ============================================================================
// Quantity-Scalar Arithmetic Operations
// ============================================================================

#[macro_export]
macro_rules! quantity_scalar_mul_div_interface {
    ($op:tt, $fn:ident, $trait:ident) => {
        impl<
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE: isize,
            const MASS_EXPONENT: isize, const MASS_SCALE: isize,
            const TIME_EXPONENT: isize, const TIME_P2: isize, const TIME_P3: isize, const TIME_P5: isize, const TIME_SCALE_ORDER: isize,
        >
            $trait<f64> for Quantity<
                LENGTH_EXPONENT, LENGTH_SCALE,
                MASS_EXPONENT, MASS_SCALE,
                TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER,
            >
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

#[macro_export]
macro_rules! add_sub_output_type {

    (
        LeftHandWins,
        $length_exponent:ident, $length_scale_1:ident, $length_scale_2:ident,
        $mass_exponent:ident, $mass_scale_1:ident, $mass_scale_2:ident,
        $time_exponent:ident, $time_scale_p2_1:ident, $time_scale_p3_1:ident, $time_scale_p5_1:ident, $time_scale_order_1:ident,
                              $time_scale_p2_2:ident, $time_scale_p3_2:ident, $time_scale_p5_2:ident, $time_scale_order_2:ident,
    ) => {
        Quantity::<
            $length_exponent, $length_scale_1,
            $mass_exponent, $mass_scale_1,
            $time_exponent, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, $time_scale_order_1,
        >
    };

    (
        SmallerWins,
        $length_exponent:ident, $length_scale_1:ident, $length_scale_2:ident,
        $mass_exponent:ident, $mass_scale_1:ident, $mass_scale_2:ident,
        $time_exponent:ident, $time_scale_p2_1:ident, $time_scale_p3_1:ident, $time_scale_p5_1:ident, $time_scale_order_1:ident,
                            $time_scale_p2_2:ident, $time_scale_p3_2:ident, $time_scale_p5_2:ident, $time_scale_order_2:ident,
    ) => {
        Quantity::<
            $length_exponent, { min_length_scale($length_scale_1, $length_scale_2) },
            $mass_exponent, { min_mass_scale($mass_scale_1, $mass_scale_2) },
            $time_exponent, { min_time_scale(2, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, $time_scale_order_1, 
                                              $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2, $time_scale_order_2) },
                            { min_time_scale(3, $time_scale_p3_1, $time_scale_p3_1, $time_scale_p5_1, $time_scale_order_1,
                                              $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2, $time_scale_order_2) },
                            { min_time_scale(5, $time_scale_p5_1, $time_scale_p5_1, $time_scale_p5_1, $time_scale_order_1,
                                              $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2, $time_scale_order_2) },
                            { min_time_scale(0, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, $time_scale_order_1,
                                              $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2, $time_scale_order_2) },
        >
    };
}

// ============================================================================
// Mul/Div Output Type
// ============================================================================

#[macro_export]
macro_rules! mul_div_output_type {
    
    (
        LeftHandWins, $log_op:tt,
        $length_exponent1:ident, $length_scale_1:ident, $length_exponent2:ident, $length_scale_2:ident,
        $mass_exponent1:ident, $mass_scale_1:ident, $mass_exponent2:ident, $mass_scale_2:ident,
        $time_exponent1:ident, $time_scale_p2_1:ident, $time_scale_p3_1:ident, $time_scale_p5_1:ident, $time_scale_order_1:ident,
                               $time_exponent2:ident, $time_scale_p2_2:ident, $time_scale_p3_2:ident, $time_scale_p5_2:ident, $time_scale_order_2:ident,
    ) => {
        type Output = Quantity::<
            { $length_exponent1 $log_op $length_exponent2 }, $length_scale_1,
            { $mass_exponent1 $log_op $mass_exponent2 }, $mass_scale_1,
            { $time_exponent1 $log_op $time_exponent2 }, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, $time_scale_order_1,
        >;
    };

    (
        SmallerWins, $op:tt,
        $length_exponent1:ident, $length_scale_1:ident, $length_exponent2:ident, $length_scale_2:ident,
        $mass_exponent1:ident, $mass_scale_1:ident, $mass_exponent2:ident, $mass_scale_2:ident,
        $time_exponent1:ident, $time_scale_p2_1:ident, $time_scale_p3_1:ident, $time_scale_p5_1:ident, $time_scale_order_1:ident,
        $time_exponent2:ident, $time_scale_p2_2:ident, $time_scale_p3_2:ident, $time_scale_p5_2:ident, $time_scale_order_2:ident,
    ) => {
        Quantity::<
            { $length_exponent1 $op $length_exponent2 }, { min_length_scale($length_scale_1, $length_scale_2) },
            { $mass_exponent1 $op $mass_exponent2 }, { min_mass_scale($mass_scale_1, $mass_scale_2) },
            { $time_exponent1 $op $time_exponent2 }, { min_time_scale(2, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, $time_scale_order_1, 
                                                              $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2, $time_scale_order_2) },
                                               { min_time_scale(3, $time_scale_p3_1, $time_scale_p3_1, $time_scale_p5_1, $time_scale_order_1,
                                                              $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2, $time_scale_order_2) },
                                               { min_time_scale(5, $time_scale_p5_1, $time_scale_p5_1, $time_scale_p5_1, $time_scale_order_1,
                                                              $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2, $time_scale_order_2) },
                                               { min_time_scale(0, $time_scale_p2_1, $time_scale_p3_1, $time_scale_p5_1, $time_scale_order_1,
                                                              $time_scale_p2_2, $time_scale_p3_2, $time_scale_p5_2, $time_scale_order_2) },
        >
    };
}

// ============================================================================
// Add/Sub Interface
// ============================================================================

#[macro_export]
macro_rules! add_sub_interface {

    // ============================================================================
    // Strict Interface (Measurement Scales Must Match)
    // ============================================================================

    (Strict, $op:tt, $fn:ident, $trait:ident) => {
        impl<
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE: isize,
            const MASS_EXPONENT: isize, const MASS_SCALE: isize,
            const TIME_EXPONENT: isize, const TIME_P2: isize, const TIME_P3: isize, const TIME_P5: isize, const TIME_SCALE_ORDER: isize,
        >
            $trait<
                Quantity<
                    LENGTH_EXPONENT, LENGTH_SCALE,
                    MASS_EXPONENT, MASS_SCALE,
                    TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER
                >,
            >
            for Quantity<
                LENGTH_EXPONENT, LENGTH_SCALE,
                MASS_EXPONENT, MASS_SCALE,
                TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER
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

    // ============================================================================
    // Non-Strict Interface (Measurement Scales Can Differ)
    // ============================================================================

    (LeftHandWins, $op:tt, $fn:ident, $trait:ident) => {
        impl<
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE1: isize, const LENGTH_SCALE2: isize,
            const MASS_EXPONENT: isize, const MASS_SCALE1: isize, const MASS_SCALE2: isize,
            const TIME_EXPONENT: isize, const TIME_P2_1: isize, const TIME_P3_1: isize, const TIME_P5_1: isize, const TIME_SCALE_ORDER1: isize,
                                        const TIME_P2_2: isize, const TIME_P3_2: isize, const TIME_P5_2: isize, const TIME_SCALE_ORDER2: isize,
        >
            $trait<
                Quantity<
                    LENGTH_EXPONENT, LENGTH_SCALE2,
                    MASS_EXPONENT, MASS_SCALE2,
                    TIME_EXPONENT, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2
                >,
            >
            for Quantity<
                LENGTH_EXPONENT, LENGTH_SCALE1,
                MASS_EXPONENT, MASS_SCALE1,
                TIME_EXPONENT, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1
            >
        {
            type Output = Self;

            fn $fn(
                self,
                other: Quantity<
                    LENGTH_EXPONENT, LENGTH_SCALE2,
                    MASS_EXPONENT, MASS_SCALE2,
                    TIME_EXPONENT, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2,
                >,
            ) -> Self::Output {
               let factor = aggregate_conversion_factor(
                    LENGTH_EXPONENT, LENGTH_SCALE1, LENGTH_SCALE2,
                    MASS_EXPONENT, MASS_SCALE1, MASS_SCALE2,
                    TIME_EXPONENT, TIME_P2_1, TIME_P3_1, TIME_P5_1,
                    TIME_P2_2, TIME_P3_2, TIME_P5_2,
               );
               Self::Output::new(self.value $op other.value * factor)
            }
        }
    };

    (SmallerWins, $op:tt, $fn:ident, $trait:ident) => {
        impl<
            const LENGTH_EXPONENT: isize, const LENGTH_SCALE1: isize, const LENGTH_SCALE2: isize,
            const MASS_EXPONENT: isize, const MASS_SCALE1: isize, const MASS_SCALE2: isize,
            const TIME_EXPONENT: isize, const TIME_P2_1: isize, const TIME_P3_1: isize, const TIME_P5_1: isize, const TIME_SCALE_ORDER1: isize,
                                        const TIME_P2_2: isize, const TIME_P3_2: isize, const TIME_P5_2: isize, const TIME_SCALE_ORDER2: isize,
        >
            $trait<
                Quantity<
                    LENGTH_EXPONENT, LENGTH_SCALE2,
                    MASS_EXPONENT, MASS_SCALE2,
                    TIME_EXPONENT, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2
                >,
            >
            for Quantity<
                LENGTH_EXPONENT, LENGTH_SCALE1,
                MASS_EXPONENT, MASS_SCALE1,
                TIME_EXPONENT, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1
            >
        where
            (): IsIsize<{ min_length_scale(LENGTH_SCALE1, LENGTH_SCALE2) }>,
            (): IsIsize<{ min_mass_scale(MASS_SCALE1, MASS_SCALE2) }>,
            (): IsIsize<{ min_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                               TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) }>,
            (): IsIsize<{ min_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                               TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) }>,
            (): IsIsize<{ min_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                               TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) }>,
            (): IsIsize<{ min_time_scale(0, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                  TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) }>,
        {
            type Output = Quantity<
                LENGTH_EXPONENT, { min_length_scale(LENGTH_SCALE1, LENGTH_SCALE2) },
                MASS_EXPONENT, { min_mass_scale(MASS_SCALE1, MASS_SCALE2) },
                TIME_EXPONENT, { min_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                   TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) },
                               { min_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                   TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) },
                               { min_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                   TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) },
                               { min_time_scale(0, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                              TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) },
                            >;

            fn $fn(
                self,
                other: Quantity<
                    LENGTH_EXPONENT, LENGTH_SCALE2,
                    MASS_EXPONENT, MASS_SCALE2,
                    TIME_EXPONENT, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2,
                >,
            ) -> Self::Output {
                let result_length_scale = min_length_scale(LENGTH_SCALE1, LENGTH_SCALE2);
                let result_mass_scale = min_mass_scale(MASS_SCALE1, MASS_SCALE2);
                let result_time_p2 = min_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                    TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2);
                let result_time_p3 = min_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                    TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2);
                let result_time_p5 = min_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                    TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2);
                
                let factor1 = aggregate_conversion_factor(
                    LENGTH_EXPONENT, LENGTH_SCALE1, result_length_scale,
                    MASS_EXPONENT, MASS_SCALE1, result_mass_scale,
                    TIME_EXPONENT, TIME_P2_1, TIME_P3_1, TIME_P5_1,
                                    result_time_p2, result_time_p3, result_time_p5,
                );
                let factor2 = aggregate_conversion_factor(
                    LENGTH_EXPONENT, LENGTH_SCALE2, result_length_scale,
                    MASS_EXPONENT, MASS_SCALE2, result_mass_scale,
                    TIME_EXPONENT, TIME_P2_2, TIME_P3_2, TIME_P5_2,
                                    result_time_p2, result_time_p3, result_time_p5,
                );

                Self::Output::new(self.value * factor1 $op other.value * factor2)
            }
        }
    }
}

// ============================================================================
// Mul/Div Interface
// ============================================================================

#[macro_export]
macro_rules! mul_div_interface {

    // ============================================================================
    // Strict Interface (Measurement Scales Must Match)
    // ============================================================================

    (Strict, $op:tt, $log_op:tt, $fn:ident, $trait:ident) => {
        impl<
            const LENGTH_EXPONENT1: isize, const LENGTH_EXPONENT2: isize, const LENGTH_SCALE: isize,
            const MASS_EXPONENT1: isize, const MASS_EXPONENT2: isize, const MASS_SCALE: isize,
            const TIME_EXPONENT1: isize, const TIME_EXPONENT2: isize, const TIME_P2: isize, const TIME_P3: isize, const TIME_P5: isize, const TIME_SCALE_ORDER: isize,
        >
            $trait<
                Quantity<
                    LENGTH_EXPONENT2, LENGTH_SCALE,
                    MASS_EXPONENT2, MASS_SCALE,
                    TIME_EXPONENT2, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER
                >,
            >
            for Quantity<
                LENGTH_EXPONENT1, LENGTH_SCALE,
                MASS_EXPONENT1, MASS_SCALE,
                TIME_EXPONENT1, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER
            >
        where
            (): IsIsize<{ LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }>,
            (): IsIsize<{ MASS_EXPONENT1 $log_op MASS_EXPONENT2 }>,
            (): IsIsize<{ TIME_EXPONENT1 $log_op TIME_EXPONENT2 }>,
        {
            type Output = Quantity<
                { LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }, LENGTH_SCALE,
                { MASS_EXPONENT1 $log_op MASS_EXPONENT2 }, MASS_SCALE,
                { TIME_EXPONENT1 $log_op TIME_EXPONENT2 }, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER
            >;

            fn $fn(
                self,
                other: Quantity<
                    LENGTH_EXPONENT2, LENGTH_SCALE,
                    MASS_EXPONENT2, MASS_SCALE,
                    TIME_EXPONENT2, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER
                >,
            ) -> Self::Output {
                Quantity::<
                    { LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }, LENGTH_SCALE,
                    { MASS_EXPONENT1 $log_op MASS_EXPONENT2 }, MASS_SCALE,
                    { TIME_EXPONENT1 $log_op TIME_EXPONENT2 }, TIME_P2, TIME_P3, TIME_P5, TIME_SCALE_ORDER
                >::new(self.value $op other.value)
            }
        }
    };

    // ============================================================================
    // Non-Strict Interface (Measurement Scales Can Differ)
    // ============================================================================

    (LeftHandWins, $op:tt, $log_op:tt, $fn:ident, $trait:ident) => {
        impl<
            const LENGTH_EXPONENT1: isize, const LENGTH_SCALE1: isize,
            const MASS_EXPONENT1: isize, const MASS_SCALE1: isize,
            const TIME_EXPONENT1: isize, const TIME_P2_1: isize, const TIME_P3_1: isize, const TIME_P5_1: isize, const TIME_SCALE_ORDER1: isize,
            const LENGTH_EXPONENT2: isize, const LENGTH_SCALE2: isize,
            const MASS_EXPONENT2: isize, const MASS_SCALE2: isize,
            const TIME_EXPONENT2: isize, const TIME_P2_2: isize, const TIME_P3_2: isize, const TIME_P5_2: isize, const TIME_SCALE_ORDER2: isize,
        >
            $trait<
                Quantity<
                    LENGTH_EXPONENT2, LENGTH_SCALE2,
                    MASS_EXPONENT2, MASS_SCALE2,
                    TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2,
                >,
            >
            for Quantity<
                LENGTH_EXPONENT1, LENGTH_SCALE1,
                MASS_EXPONENT1, MASS_SCALE1,
                TIME_EXPONENT1, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1,
            >
        where
            (): IsIsize<{ LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }>,
            (): IsIsize<{ MASS_EXPONENT1 $log_op MASS_EXPONENT2 }>,
            (): IsIsize<{ TIME_EXPONENT1 $log_op TIME_EXPONENT2 }>,
        {
            type Output = Quantity<
                { LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }, LENGTH_SCALE1,
                { MASS_EXPONENT1 $log_op MASS_EXPONENT2 }, MASS_SCALE1,
                { TIME_EXPONENT1 $log_op TIME_EXPONENT2 }, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1,
            >;

            fn $fn(
                self,
                other: Quantity<
                    LENGTH_EXPONENT2, LENGTH_SCALE2,
                    MASS_EXPONENT2, MASS_SCALE2,
                    TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2,
                >,
            ) -> Self::Output {
                Quantity::<
                    { LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }, LENGTH_SCALE1,
                    { MASS_EXPONENT1 $log_op MASS_EXPONENT2 }, MASS_SCALE1,
                    { TIME_EXPONENT1 $log_op TIME_EXPONENT2 }, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1,
                >::new(self.value $op (other.value * aggregate_conversion_factor(
                    LENGTH_EXPONENT2, LENGTH_SCALE2, LENGTH_SCALE1,
                    MASS_EXPONENT2, MASS_SCALE2, MASS_SCALE1,
                    TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, 
                                    TIME_P2_1, TIME_P3_1, TIME_P5_1,
                )))
            }
        }
    };

    (SmallerWins, $op:tt, $log_op:tt, $fn:ident, $trait:ident) => {
        impl<
            const LENGTH_EXPONENT1: isize, const LENGTH_SCALE1: isize,
            const MASS_EXPONENT1: isize, const MASS_SCALE1: isize,
            const TIME_EXPONENT1: isize, const TIME_P2_1: isize, const TIME_P3_1: isize, const TIME_P5_1: isize, const TIME_SCALE_ORDER1: isize,
            const LENGTH_EXPONENT2: isize, const LENGTH_SCALE2: isize,
            const MASS_EXPONENT2: isize, const MASS_SCALE2: isize,
            const TIME_EXPONENT2: isize, const TIME_P2_2: isize, const TIME_P3_2: isize, const TIME_P5_2: isize, const TIME_SCALE_ORDER2: isize,
        >
            $trait<
                Quantity<
                    LENGTH_EXPONENT2, LENGTH_SCALE2,
                    MASS_EXPONENT2, MASS_SCALE2,
                    TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2,
                >,
            >
            for Quantity<
                LENGTH_EXPONENT1, LENGTH_SCALE1,
                MASS_EXPONENT1, MASS_SCALE1,
                TIME_EXPONENT1, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1,
            >
        where
            (): IsIsize<{ LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }>,
            (): IsIsize<{ MASS_EXPONENT1 $log_op MASS_EXPONENT2 }>,
            (): IsIsize<{ TIME_EXPONENT1 $log_op TIME_EXPONENT2 }>,
            (): IsIsize<{ min_length_scale(LENGTH_SCALE1, LENGTH_SCALE2) }>,
            (): IsIsize<{ min_mass_scale(MASS_SCALE1, MASS_SCALE2) }>,
            (): IsIsize<{ min_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                               TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) }>,
            (): IsIsize<{ min_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                               TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) }>,
            (): IsIsize<{ min_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                               TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) }>,
            (): IsIsize<{ min_time_scale(0, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                  TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) }>,
        {
            type Output = Quantity<
                { LENGTH_EXPONENT1 $log_op LENGTH_EXPONENT2 }, { min_length_scale(LENGTH_SCALE1, LENGTH_SCALE2) },
                { MASS_EXPONENT1 $log_op MASS_EXPONENT2 }, { min_mass_scale(MASS_SCALE1, MASS_SCALE2) },
                { TIME_EXPONENT1 $log_op TIME_EXPONENT2 }, { min_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                                               TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) },
                                                           { min_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                                               TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) },
                                                           { min_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                                               TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) },
                                                           { min_time_scale(0, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                                               TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2) },
            >;

            fn $fn(
                self,
                other: Quantity<
                    LENGTH_EXPONENT2, LENGTH_SCALE2,
                    MASS_EXPONENT2, MASS_SCALE2,
                    TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2,
                >,
            ) -> Self::Output {
                let result_length_scale = min_length_scale(LENGTH_SCALE1, LENGTH_SCALE2);
                let result_mass_scale = min_mass_scale(MASS_SCALE1, MASS_SCALE2);
                let result_time_p2 = min_time_scale(2, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                    TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2);
                let result_time_p3 = min_time_scale(3, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                    TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2);
                let result_time_p5 = min_time_scale(5, TIME_P2_1, TIME_P3_1, TIME_P5_1, TIME_SCALE_ORDER1, 
                                                    TIME_P2_2, TIME_P3_2, TIME_P5_2, TIME_SCALE_ORDER2);
                
                let factor1 = aggregate_conversion_factor(
                    LENGTH_EXPONENT1, LENGTH_SCALE1, result_length_scale,
                    MASS_EXPONENT1, MASS_SCALE1, result_mass_scale,
                    TIME_EXPONENT1, TIME_P2_1, TIME_P3_1, TIME_P5_1,
                                    result_time_p2, result_time_p3, result_time_p5,
                );
                let factor2 = aggregate_conversion_factor(
                    LENGTH_EXPONENT2, LENGTH_SCALE2, result_length_scale,
                    MASS_EXPONENT2, MASS_SCALE2, result_mass_scale,
                    TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2,
                                    result_time_p2, result_time_p3, result_time_p5,
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