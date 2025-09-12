use crate::generated_quantity_type::Quantity;
use crate::IsI8;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[macro_export]
macro_rules! scalar_quantity_mul_div_interface {
    (($($single_dimension_single_scale_params:tt)*), ($($inversion_where_clauses:tt)*), $T:ty) => {
        impl<
            $($single_dimension_single_scale_params)*,
        >
            Mul<$crate::quantity_type!($T)> for $T
        {
            type Output = $crate::quantity_type!($T);

            fn mul(self: $T, other: Self::Output) -> Self::Output {
                let result_value = self * other.value;
                Self::Output::new(result_value)
            }
        }

        impl<
            $($single_dimension_single_scale_params)*,
        >
            Div<$crate::quantity_type!($T)> for $T
            where
                $($inversion_where_clauses)*
        {
            type Output = $crate::inverse_quantity_type!($T);

            fn div(self: $T, other: $crate::quantity_type!($T)) -> Self::Output {
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
            $trait<$T> for $crate::quantity_type!($T)
        {
            type Output = Self;

            fn $fn(self, other: f64) -> Self::Output {
                Self::new(self.value $op other)
            }
        }
    }
}

#[macro_export]
macro_rules! quantity_scalar_mul_div_assign_interface {
    (($($single_dimension_single_scale_params:tt)*), $op:tt, $fn:ident, $trait:ident, $T:ty) => {
        impl<
            $($single_dimension_single_scale_params)*,
        >
            $trait<$T> for $crate::quantity_type!($T)
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
        ($($output_scale_where_clauses:tt)*),
        Strict, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($single_dimension_single_scale_params)*,
        >
            $trait<$crate::quantity_type!($T)>
            for $crate::quantity_type!($T)
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
        ($($output_scale_where_clauses:tt)*),
        LeftHandWins, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($single_dimension_multiple_scale_params)*,
        >
            $trait<
                $crate::addition_input!(RightHand, $T),
            >
            for $crate::addition_input!(LeftHand, $T)
        {
            type Output = Self;

            fn $fn(
                self,
                other: $crate::addition_input!(RightHand, $T),
            ) -> Self::Output {
               const rescaled_other: Self = $rescale_fn(other);
               Self::Output::new(self.value $op rescaled_other.value)
            }
        }
    };

    // Non-Strict interface (measurement scales can differ) (two sets of scale parameters)
    (
        ($($single_dimension_single_scale_params:tt)*), ($($single_dimension_multiple_scale_params:tt)*),
        ($($output_scale_where_clauses:tt)*),
        SmallerWins, $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($single_dimension_multiple_scale_params)*,
        >
            $trait<
                $crate::addition_input!(RightHand, $T),
            >
            for $crate::addition_input!(LeftHand, $T)
        where
            $($output_scale_where_clauses)*
        {
            type Output = $crate::addition_output!(SmallerWins, $T);

            fn $fn(
                self,
                other: $crate::addition_input!(RightHand, $T),
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
                $crate::quantity_type!($T),
            > for $crate::quantity_type!($T)
        {
            fn $fn(&mut self, other: $crate::quantity_type!($T)) {
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
                $crate::addition_input!(RightHand, $T),
            > for $crate::addition_input!(LeftHand, $T)
        {
            fn $fn(&mut self, other: $crate::addition_input!(RightHand, $T)) {
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
        ($($output_dimension_where_clauses:tt)*), ($($output_scale_where_clauses:tt)*),
        Strict, $op:tt, $log_op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($multiple_dimension_single_scale_params)*,
        >
            $trait<
                $crate::multiplication_input!(Strict, RightHand, $T),
            >
            for $crate::multiplication_input!(Strict, LeftHand, $T)
        where
            $($output_dimension_where_clauses)*
        {
            type Output = $crate::multiplication_output!(Strict, $T, $log_op);

            fn $fn(
                self,
                other: $crate::multiplication_input!(Strict, RightHand, $T),
            ) -> Self::Output {
                Self::Output::new(self.value $op other.value)
            }
        }
    };

    // Non-Strict interface (measurement scales can differ) (two sets of scale parameters)
    (
        ($($multiple_dimension_single_scale_params:tt)*), ($($multiple_dimension_multiple_scale_params:tt)*),
        ($($output_dimension_where_clauses:tt)*), ($($output_scale_where_clauses:tt)*),
        LeftHandWins, $op:tt, $log_op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($multiple_dimension_multiple_scale_params)*,
        >
            $trait<
                $crate::multiplication_input!(LeftHandWins, RightHand, $T),
            >
            for $crate::multiplication_input!(LeftHandWins, LeftHand, $T)
        where
            $($output_dimension_where_clauses)*
        {
            type Output = $crate::multiplication_output!(LeftHandWins, $T, $log_op);

            type OutputScaleOther = $crate::multiplication_output_scale_input!(LeftHandWins, RightHand, $T);

            fn $fn(
                self,
                other: $crate::multiplication_input!(LeftHandWins, RightHand, $T),
            ) -> Self::Output {
                let rescaled_other: OutputScaleOther = $rescale_fn(other);
                Self::Output::new(self.value $op rescaled_other.value)
            }
        }
    };

    // Non-Strict interface (measurement scales can differ) (two sets of scale parameters)
    (
        ($($multiple_dimension_single_scale_params:tt)*), ($($multiple_dimension_multiple_scale_params:tt)*),
        ($($output_dimension_where_clauses:tt)*), ($($output_scale_where_clauses:tt)*),
        SmallerWins, $op:tt, $log_op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($multiple_dimension_multiple_scale_params)*,
        >
            $trait<
                $crate::multiplication_input!(SmallerWins, RightHand, $T),
            >
            for $crate::multiplication_input!(SmallerWins, LeftHand, $T)
        where
            $($output_dimension_where_clauses)*,
            // smaller wins must validate the result of the output scale determination
            $($output_scale_where_clauses)*
        {
            type Output = $crate::multiplication_output!(SmallerWins, $T, $log_op);

            type OutputScaleSelf = $crate::multiplication_output_scale_input!(SmallerWins, LeftHand, $T);

            type OutputScaleOther = $crate::multiplication_output_scale_input!(SmallerWins, RightHand, $T);

            fn $fn(
                self,
                other: $crate::multiplication_input!(SmallerWins, RightHand, $T),
            ) -> Self::Output {
                let rescaled_self: OutputScaleSelf = $rescale_fn(self);
                let rescaled_other: OutputScaleOther = $rescale_fn(other);

                Self::Output::new(rescaled_self.value $op rescaled_other.value)
            }
        }
    };
}

#[macro_export]
macro_rules! _define_arithmetic {
    (($($single_dimension_single_scale_params:tt)*),
     ($($single_dimension_multiple_scale_params:tt)*),
     ($($multiple_dimension_single_scale_params:tt)*),
     ($($multiple_dimension_multiple_scale_params:tt)*),
     ($($inversion_where_clauses:tt)*),
     ($($add_min_scale_where_clauses:tt)*),
     ($($mul_min_scale_where_clauses:tt)*),
     ($($mul_output_dimension_where_clauses:tt)*),
     ($($div_output_dimension_where_clauses:tt)*),
    $rescale_behavior:ident, $T:ty, $rescale_fn:ident) => {
        // scalar-quantity arithmetic operations
        $crate::scalar_quantity_mul_div_interface!(($($single_dimension_single_scale_params)*), ($($inversion_where_clauses)*), $T);

        $crate::quantity_scalar_mul_div_interface!(($($single_dimension_single_scale_params)*), *, mul, Mul, $T);
        $crate::quantity_scalar_mul_div_interface!(($($single_dimension_single_scale_params)*), /, div, Div, $T);

        $crate::quantity_scalar_mul_div_assign_interface!(($($single_dimension_single_scale_params)*), *=, mul_assign, MulAssign, $T);
        $crate::quantity_scalar_mul_div_assign_interface!(($($single_dimension_single_scale_params)*), /=, div_assign, DivAssign, $T);

        // quantity-quantity arithmetic operations
        $crate::quantity_quantity_add_sub_interface!(
            ($($single_dimension_single_scale_params)*), ($($single_dimension_multiple_scale_params)*),
            ($($add_min_scale_where_clauses)*), $rescale_behavior, +, add, Add, $T, $rescale_fn
        );
        $crate::quantity_quantity_add_sub_interface!(
            ($($single_dimension_single_scale_params)*), ($($single_dimension_multiple_scale_params)*),
            ($($add_min_scale_where_clauses)*), $rescale_behavior, -, sub, Sub, $T, $rescale_fn
        );

        $crate::quantity_quantity_add_sub_assign_interface!(
            ($($single_dimension_single_scale_params)*), ($($single_dimension_multiple_scale_params)*),
            $rescale_behavior, +=, add_assign, AddAssign, $T, $rescale_fn
        );
        $crate::quantity_quantity_add_sub_assign_interface!(
            ($($single_dimension_single_scale_params)*), ($($single_dimension_multiple_scale_params)*),
            $rescale_behavior, -=, sub_assign, SubAssign, $T, $rescale_fn
        );

        $crate::quantity_quantity_mul_div_interface!(
            ($($multiple_dimension_single_scale_params)*), ($($multiple_dimension_multiple_scale_params)*),
             ($($mul_output_dimension_where_clauses)*), ($($mul_min_scale_where_clauses)*),
             $rescale_behavior, *, +, mul, Mul, $T, $rescale_fn
        );
        $crate::quantity_quantity_mul_div_interface!(
            ($($multiple_dimension_single_scale_params)*), ($($multiple_dimension_multiple_scale_params)*),
            ($($div_output_dimension_where_clauses)*), ($($mul_min_scale_where_clauses)*),
            $rescale_behavior, /, -, div, Div, $T, $rescale_fn
        );
    };
}