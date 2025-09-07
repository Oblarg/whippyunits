use crate::quantity_type::Quantity;
use crate::IsIsize;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[macro_export]
macro_rules! scalar_quantity_mul_div_interface {
    (($($single_dimension_single_scale_params:tt)*), ($($inversion_where_clauses:tt)*), $T:ty) => {
        impl<
            $($single_dimension_single_scale_params)*,
        >
            Mul<quantity_type!($T)> for $T
        {
            type Output = quantity_type!($T);

            fn mul(self: $T, other: Self::Output) -> Self::Output {
                let result_value = self * other.value;
                Self::Output::new(result_value)
            }
        }

        impl<
            $($single_dimension_single_scale_params)*,
        >
            Div<quantity_type!($T)> for $T
            where
                $($inversion_where_clauses)*
        {
            type Output = inverse_quantity_type!($T);

            fn div(self: $T, other: quantity_type!($T)) -> Self::Output {
                let result_value = self / other.value;
                Self::Output::new(result_value)
            }
        }
    }
}

#[macro_export]
macro_rules! quantity_scalar_mul_div_interface {
    (($($single_dimension_single_scale_params:tt)*), $op:tt, $fn:ident, $trait:ident, $T:ty) => {
        impl<
            $($single_dimension_single_scale_params)*,
        >
            $trait<$T> for quantity_type!($T)
        {
            type Output = Self;

            fn $fn(self, other: f64) -> Self::Output {
                Self::new(self.value $op other)
            }
        }
    }
}

macro_rules! quantity_scalar_mul_div_assign_interface {
    (($($single_dimension_single_scale_params:tt)*), $op:tt, $fn:ident, $trait:ident, $T:ty) => {
        impl<
            $($single_dimension_single_scale_params)*,
        >
            $trait<$T> for quantity_type!($T)
        {
            fn $fn(&mut self, other: $T) {
                self.value $op other;
            }
        }
    }
}

#[macro_export]
macro_rules! quantity_quantity_add_sub_interface {
    // Strict interface (measurement scales must match) (only one set of scale parameters)
    (
        ($($single_dimension_single_scale_params:tt)*), ($($single_dimension_multiple_scale_params:tt)*),
        ($($min_scale_where_clauses:tt)*),
        Strict, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($single_dimension_single_scale_params)*,
        >
            $trait<quantity_type!($T)>
            for quantity_type!($T)
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
    (
        ($($single_dimension_single_scale_params:tt)*), ($($single_dimension_multiple_scale_params:tt)*),
        ($($min_scale_where_clauses:tt)*),
        LeftHandWins, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($single_dimension_multiple_scale_params)*,
        >
            $trait<
                addition_input!(RightHand, $T),
            >
            for addition_input!(LeftHand, $T)
        {
            type Output = Self;

            fn $fn(
                self,
                other: addition_input!(RightHand, $T),
            ) -> Self::Output {
               const rescaled_other: Self = $rescale_fn(other);
               Self::Output::new(self.value $op rescaled_other.value)
            }
        }
    };

    // Non-Strict interface (measurement scales can differ) (two sets of scale parameters)
    (
        ($($single_dimension_single_scale_params:tt)*), ($($single_dimension_multiple_scale_params:tt)*),
        ($($min_scale_where_clauses:tt)*),
        SmallerWins, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($single_dimension_multiple_scale_params)*,
        >
            $trait<
                addition_input!(RightHand, $T),
            >
            for addition_input!(LeftHand, $T)
        where
            $($min_scale_where_clauses)*
        {
            type Output = addition_output!(SmallerWins, $T);

            fn $fn(
                self,
                other: addition_input!(RightHand, $T),
            ) -> Self::Output {
                let rescaled_self: Self::Output = $rescale_fn(self);
                let rescaled_other: Self::Output = $rescale_fn(other);

                Self::Output::new(rescaled_self.value $op rescaled_other.value)
            }
        }
    }
}

// AddAssign/SubAssign must return the same type as the left-hand side, so it only supports strict or left-hand-wins rescale semantics
#[macro_export]
macro_rules! quantity_quantity_add_sub_assign_interface {
    // Strict interface (measurement scales must match) (only one set of scale parameters)
    (
        ($($single_dimension_single_scale_params:tt)*), ($($single_dimension_multiple_scale_params:tt)*),
        Strict, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($single_dimension_single_scale_params)*,
        >
            $trait<
                quantity_type!($T),
            > for quantity_type!($T)
        {
            fn $fn(&mut self, other: quantity_type!($T)) {
                self.value $op other.value;
            }
        }
    };

    // Non-Strict interface (measurement scales can differ) (two sets of scale parameters)
    // All non-strict modes adopt LeftHandWins for AddAssign/SubAssign
    (
        ($($single_dimension_single_scale_params:tt)*), ($($single_dimension_multiple_scale_params:tt)*),
        $rescale_behavior:ident, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl
            $($single_dimension_multiple_scale_params)*,
        >
            $trait<
                addition_input!(RightHand, $T),
            > for addition_input!(LeftHand, $T)
        {
            fn $fn(&mut self, other: addition_input!(RightHand, $T)) {
                let rescaled_other: Self = $rescale_fn(other);
                self.value $op rescaled_other.value;
            }
        }
    };
}

#[macro_export]
macro_rules! quantity_quantity_mul_div_interface {
    // Strict interface (measurement scales must match) (only one set of scale parameters)
    (
        ($($multiple_dimension_single_scale_params:tt)*), ($($multiple_dimension_multiple_scale_params:tt)*),
        ($($mul_result_where_clauses:tt)*), ($($mul_min_scale_where_clauses:tt)*),
        Strict, $op:tt, $log_op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($multiple_dimension_single_scale_params)*,
        >
            $trait<
                multiplication_input!(Strict, RightHand, $T),
            >
            for multiplication_input!(Strict, LeftHand, $T)
        where
            $($mul_result_where_clauses)*
        {
            type Output = multiplication_output!(Strict, $T, $log_op);

            fn $fn(
                self,
                other: multiplication_input!(Strict, RightHand, $T),
            ) -> Self::Output {
                Self::Output::new(self.value $op other.value)
            }
        }
    };

    // Non-Strict interface (measurement scales can differ) (two sets of scale parameters)
    (
        ($($multiple_dimension_single_scale_params:tt)*), ($($multiple_dimension_multiple_scale_params:tt)*),
        ($($mul_result_where_clauses:tt)*), ($($mul_min_scale_where_clauses:tt)*),
        LeftHandWins, $op:tt, $log_op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($multiple_dimension_multiple_scale_params)*,
        >
            $trait<
                multiplication_input!(LeftHandWins, RightHand, $T),
            >
            for multiplication_input!(LeftHandWins, LeftHand, $T)
        where
            $($mul_result_where_clauses)*
        {
            type Output = multiplication_output!(LeftHandWins, $T, $log_op);

            type OutputScaleOther = multiplication_output_scale_input!(LeftHandWins, RightHand, $T);

            fn $fn(
                self,
                other: multiplication_input!(LeftHandWins, RightHand, $T),
            ) -> Self::Output {
                let rescaled_other: OutputScaleOther = $rescale_fn(other);
                Self::Output::new(self.value $op rescaled_other.value)
            }
        }
    };

    // Non-Strict interface (measurement scales can differ) (two sets of scale parameters)
    (
        ($($multiple_dimension_single_scale_params:tt)*), ($($multiple_dimension_multiple_scale_params:tt)*),
        ($($mul_result_where_clauses:tt)*), ($($mul_min_scale_where_clauses:tt)*),
        SmallerWins, $op:tt, $log_op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($multiple_dimension_multiple_scale_params)*,
        >
            $trait<
                multiplication_input!(SmallerWins, RightHand, $T),
            >
            for multiplication_input!(SmallerWins, LeftHand, $T)
        where
            $($mul_result_where_clauses)*,
            $($mul_min_scale_where_clauses)*
        {
            type Output = multiplication_output!(SmallerWins, $T, $log_op);

            type OutputScaleSelf = multiplication_output_scale_input!(SmallerWins, LeftHand, $T);

            type OutputScaleOther = multiplication_output_scale_input!(SmallerWins, RightHand, $T);

            fn $fn(
                self,
                other: multiplication_input!(SmallerWins, RightHand, $T),
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
    (($($single_dimension_single_scale_params:tt)*),
     ($($single_dimension_multiple_scale_params:tt)*),
     ($($multiple_dimension_single_scale_params:tt)*),
     ($($multiple_dimension_multiple_scale_params:tt)*),
     ($($inversion_where_clauses:tt)*),
     ($($add_min_scale_where_clauses:tt)*),
     ($($mul_min_scale_where_clauses:tt)*),
     ($($mul_result_where_clauses:tt)*),
     ($($div_result_where_clauses:tt)*),
    $rescale_behavior:ident, $T:ty, $rescale_fn:ident) => {
        // scalar-quantity arithmetic operations
        scalar_quantity_mul_div_interface!(($($single_dimension_single_scale_params)*), ($($inversion_where_clauses)*), $T);

        quantity_scalar_mul_div_interface!(($($single_dimension_single_scale_params)*), *, mul, Mul, $T);
        quantity_scalar_mul_div_interface!(($($single_dimension_single_scale_params)*), /, div, Div, $T);

        quantity_scalar_mul_div_assign_interface!(($($single_dimension_single_scale_params)*), *=, mul_assign, MulAssign, $T);
        quantity_scalar_mul_div_assign_interface!(($($single_dimension_single_scale_params)*), /=, div_assign, DivAssign, $T);

        // quantity-quantity arithmetic operations
        quantity_quantity_add_sub_interface!(
            ($($single_dimension_single_scale_params)*), ($($single_dimension_multiple_scale_params)*), 
            ($($add_min_scale_where_clauses)*), $rescale_behavior, +, add, Add, $T, $rescale_fn
        );
        quantity_quantity_add_sub_interface!(
            ($($single_dimension_single_scale_params)*), ($($single_dimension_multiple_scale_params)*),
            ($($add_min_scale_where_clauses)*), $rescale_behavior, -, sub, Sub, $T, $rescale_fn
        );

        quantity_quantity_add_sub_assign_interface!(
            ($($single_dimension_single_scale_params)*), ($($single_dimension_multiple_scale_params)*), 
            $rescale_behavior, +=, add_assign, AddAssign, $T, $rescale_fn
        );
        quantity_quantity_add_sub_assign_interface!(
            ($($single_dimension_single_scale_params)*), ($($single_dimension_multiple_scale_params)*), 
            $rescale_behavior, -=, sub_assign, SubAssign, $T, $rescale_fn
        );

        quantity_quantity_mul_div_interface!(
            ($($multiple_dimension_single_scale_params)*), ($($multiple_dimension_multiple_scale_params)*),
             ($($mul_result_where_clauses)*), ($($mul_min_scale_where_clauses)*), 
             $rescale_behavior, *, +, mul, Mul, $T, $rescale_fn
        );
        quantity_quantity_mul_div_interface!(
            ($($multiple_dimension_single_scale_params)*), ($($multiple_dimension_multiple_scale_params)*), 
            ($($div_result_where_clauses)*), ($($mul_min_scale_where_clauses)*), 
            $rescale_behavior, /, -, div, Div, $T, $rescale_fn
        );
    };
}

#[cfg(feature = "strict")]
generate_arithmetic_ops!(
    // single dimension, single scale
    (const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
    const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize),
    // single dimension, multiple scales
    (const MASS_EXPONENT: isize, const MASS_SCALE_P10_1: isize, const MASS_SCALE_P10_2: isize,
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10_1: isize, const LENGTH_SCALE_P10_2: isize,
    const TIME_EXPONENT: isize, const TIME_SCALE_P2_1: isize, const TIME_SCALE_P3_1: isize, const TIME_SCALE_P5_1: isize,
                                const TIME_SCALE_P2_2: isize, const TIME_SCALE_P3_2: isize, const TIME_SCALE_P5_2: isize),
    // multiple dimension, single scale
    (const MASS_EXPONENT_1: isize, const MASS_EXPONENT_2: isize, const MASS_SCALE_P10: isize,
    const LENGTH_EXPONENT_1: isize, const LENGTH_EXPONENT_2: isize, const LENGTH_SCALE_P10: isize,
    const TIME_EXPONENT_1: isize, const TIME_EXPONENT_2: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize),
    // multiple dimension, multiple scales
    (const MASS_EXPONENT_1: isize, const MASS_SCALE_P10_1: isize,
    const LENGTH_EXPONENT_1: isize, const LENGTH_SCALE_P10_1: isize,
    const TIME_EXPONENT_1: isize, const TIME_SCALE_P2_1: isize, const TIME_SCALE_P3_1: isize, const TIME_SCALE_P5_1: isize,
    const MASS_EXPONENT_2: isize, const MASS_SCALE_P10_2: isize,
    const LENGTH_EXPONENT_2: isize, const LENGTH_SCALE_P10_2: isize,
    const TIME_EXPONENT_2: isize, const TIME_SCALE_P2_2: isize, const TIME_SCALE_P3_2: isize, const TIME_SCALE_P5_2: isize,),
    // inversion where clauses
    ((): IsIsize<{ -MASS_EXPONENT }>,
    (): IsIsize<{ -LENGTH_EXPONENT }>,
    (): IsIsize<{ -TIME_EXPONENT }>),
    // add min scale where clauses
    ((): IsIsize<{ min_mass_scale(MASS_EXPONENT, MASS_SCALE_P10_1, MASS_EXPONENT, MASS_SCALE_P10_2) }>,
    (): IsIsize<{ min_length_scale(LENGTH_EXPONENT, LENGTH_SCALE_P10_1, LENGTH_EXPONENT, LENGTH_SCALE_P10_2) }>,
    (): IsIsize<{ min_time_scale(2, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
    (): IsIsize<{ min_time_scale(3, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
    (): IsIsize<{ min_time_scale(5, TIME_EXPONENT, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                    TIME_EXPONENT, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>),
    // mul min scale where clauses
    ((): IsIsize<{ min_mass_scale(MASS_EXPONENT_1, MASS_SCALE_P10_1, MASS_EXPONENT_2, MASS_SCALE_P10_2) }>,
    (): IsIsize<{ min_length_scale(LENGTH_EXPONENT_1, LENGTH_SCALE_P10_1, LENGTH_EXPONENT_2, LENGTH_SCALE_P10_2) }>,
    (): IsIsize<{ min_time_scale(2, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                    TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
    (): IsIsize<{ min_time_scale(3, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                    TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>,
    (): IsIsize<{ min_time_scale(5, TIME_EXPONENT_1, TIME_SCALE_P2_1, TIME_SCALE_P3_1, TIME_SCALE_P5_1,
                                    TIME_EXPONENT_2, TIME_SCALE_P2_2, TIME_SCALE_P3_2, TIME_SCALE_P5_2) }>),
    // mul result where clauses
    ((): IsIsize<{ MASS_EXPONENT_1 + MASS_EXPONENT_2 }>,
    (): IsIsize<{ LENGTH_EXPONENT_1 + LENGTH_EXPONENT_2 }>,
    (): IsIsize<{ TIME_EXPONENT_1 + TIME_EXPONENT_2 }>),
    // div result where clauses
    ((): IsIsize<{ MASS_EXPONENT_1 - MASS_EXPONENT_2 }>,
    (): IsIsize<{ LENGTH_EXPONENT_1 - LENGTH_EXPONENT_2 }>,
    (): IsIsize<{ TIME_EXPONENT_1 - TIME_EXPONENT_2 }>),
    Strict, f64, rescale_f64
);

// Default if no feature is specified
#[cfg(not(any(
    feature = "strict",
    feature = "smaller_wins",
    feature = "left_hand_wins"
)))]
generate_arithmetic_ops!(Strict);
