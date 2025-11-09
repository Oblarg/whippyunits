#[macro_export]
#[doc(hidden)]
macro_rules! scalar_quantity_mul_div_interface {
    (($($single_dimension_single_scale_params:tt)*), ($($inversion_where_clauses:tt)*), $T:ty) => {
        impl<
            $($single_dimension_single_scale_params)*
        >
            std::ops::Mul<$crate::quantity_type!($T)> for $T
        {
            type Output = $crate::quantity_type!($T);

            fn mul(self: $T, other: Self::Output) -> Self::Output {
                let result_value = self * other.unsafe_value;
                Self::Output::new(result_value)
            }
        }

        impl<
            $($single_dimension_single_scale_params)*
        >
            std::ops::Div<$crate::quantity_type!($T)> for $T
            where
                $($inversion_where_clauses)*
        {
            type Output = $crate::inverse_quantity_type!($T);

            fn div(self: $T, other: $crate::quantity_type!($T)) -> Self::Output {
                let result_value = self / other.unsafe_value;
                Self::Output::new(result_value)
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! quantity_scalar_mul_div_interface {
    (($($single_dimension_single_scale_params:tt)*), $op:tt, $fn:ident, $trait:ident, $T:ty) => {
        impl<
            $($single_dimension_single_scale_params)*
        >
            std::ops::$trait<$T> for $crate::quantity_type!($T)
        {
            type Output = Self;

            fn $fn(self, other: $T) -> Self::Output {
                Self::new(self.unsafe_value $op other)
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! quantity_scalar_mul_div_assign_interface {
    (($($single_dimension_single_scale_params:tt)*), $op:tt, $fn:ident, $trait:ident, $T:ty) => {
        impl<
            $($single_dimension_single_scale_params)*
        >
            std::ops::$trait<$T> for $crate::quantity_type!($T)
        {
            fn $fn(&mut self, other: $T) {
                self.unsafe_value $op other;
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! quantity_quantity_add_sub_interface {
    // Scale-strict interface (measurement scales must match)
    (
        ($($single_dimension_single_scale_params:tt)*),
        $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($single_dimension_single_scale_params)*
        >
            std::ops::$trait<$crate::quantity_type!($T)>
            for $crate::quantity_type!($T)
        {
            type Output = Self;

            fn $fn(
                self,
                other: Self,
            ) -> Self::Output {
                Self::new(self.unsafe_value $op other.unsafe_value)
            }
        }
    };
}

// AddAssign/SubAssign are scale-strict
#[macro_export]
#[doc(hidden)]
macro_rules! quantity_quantity_add_sub_assign_interface {
    // Scale-strict interface (measurement scales must match)
    (
        ($($single_dimension_single_scale_params:tt)*),
        $op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($single_dimension_single_scale_params)*
        >
            std::ops::$trait<
                $crate::quantity_type!($T),
            > for $crate::quantity_type!($T)
        {
            fn $fn(&mut self, other: $crate::quantity_type!($T)) {
                self.unsafe_value $op other.unsafe_value;
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! quantity_quantity_mul_div_interface {
    // Strict interface (measurement scales must match) (only one set of scale parameters)
    (
        ($($multiple_dimension_multiple_scale_params:tt)*),
        ($($output_dimension_where_clauses:tt)*),
        $op:tt, $log_op:tt, $fn:ident, $trait:ident, $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($multiple_dimension_multiple_scale_params)*
        >
            std::ops::$trait<
                $crate::multiplication_input!(RightHand, $T),
            >
            for $crate::multiplication_input!(LeftHand, $T)
        where
            $($output_dimension_where_clauses)*
        {
            type Output = $crate::multiplication_output!($T, $log_op);

            fn $fn(
                self,
                other: $crate::multiplication_input!(RightHand, $T),
            ) -> Self::Output {
                Self::Output::new(self.unsafe_value $op other.unsafe_value)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! quantity_neg_interface {
    (($($single_dimension_single_scale_params:tt)*), $T:ty) => {
        impl<
            $($single_dimension_single_scale_params)*
        >
            std::ops::Neg for $crate::quantity_type!($T)
        where
            $T: std::ops::Neg<Output = $T>
        {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::new(-self.unsafe_value)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! quantity_quantity_partial_ord_interface {
    // Scale-strict comparison interface (measurement scales must match)
    (
        ($($single_dimension_single_scale_params:tt)*),
        $T:ty, $rescale_fn:ident
    ) => {
        impl<
            $($single_dimension_single_scale_params)*
        >
            std::cmp::PartialOrd<$crate::quantity_type!($T)>
            for $crate::quantity_type!($T)
        where
            $T: PartialOrd,
            Brand: PartialEq,
        {
            fn partial_cmp(&self, other: &$crate::quantity_type!($T)) -> Option<::std::cmp::Ordering> {
                self.unsafe_value.partial_cmp(&other.unsafe_value)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _define_arithmetic_signed {
    (($($single_dimension_single_scale_params:tt)*),
     ($($multiple_dimension_multiple_scale_params:tt)*),
     ($($inversion_where_clauses:tt)*),
     ($($mul_output_dimension_where_clauses:tt)*),
     ($($div_output_dimension_where_clauses:tt)*),
     $T:ty, $rescale_fn:ident) => {
        // scalar-quantity arithmetic operations
        $crate::scalar_quantity_mul_div_interface!(($($single_dimension_single_scale_params)*), ($($inversion_where_clauses)*), $T);

        $crate::quantity_scalar_mul_div_interface!(($($single_dimension_single_scale_params)*), *, mul, Mul, $T);
        $crate::quantity_scalar_mul_div_interface!(($($single_dimension_single_scale_params)*), /, div, Div, $T);

        $crate::quantity_scalar_mul_div_assign_interface!(($($single_dimension_single_scale_params)*), *=, mul_assign, MulAssign, $T);
        $crate::quantity_scalar_mul_div_assign_interface!(($($single_dimension_single_scale_params)*), /=, div_assign, DivAssign, $T);

        // unary operations (only for signed types)
        $crate::quantity_neg_interface!(($($single_dimension_single_scale_params)*), $T);

        // quantity-quantity arithmetic operations
        $crate::quantity_quantity_add_sub_interface!(
            ($($single_dimension_single_scale_params)*),
            +, add, Add, $T, $rescale_fn
        );
        $crate::quantity_quantity_add_sub_interface!(
            ($($single_dimension_single_scale_params)*),
            -, sub, Sub, $T, $rescale_fn
        );

        $crate::quantity_quantity_add_sub_assign_interface!(
            ($($single_dimension_single_scale_params)*),
            +=, add_assign, AddAssign, $T, $rescale_fn
        );
        $crate::quantity_quantity_add_sub_assign_interface!(
            ($($single_dimension_single_scale_params)*),
            -=, sub_assign, SubAssign, $T, $rescale_fn
        );

        $crate::quantity_quantity_mul_div_interface!(
            ($($multiple_dimension_multiple_scale_params)*),
            ($($mul_output_dimension_where_clauses)*),
            *, +, mul, Mul, $T, $rescale_fn
        );
        $crate::quantity_quantity_mul_div_interface!(
            ($($multiple_dimension_multiple_scale_params)*),
            ($($div_output_dimension_where_clauses)*),
            /, -, div, Div, $T, $rescale_fn
        );

        // quantity-quantity comparison operations (scale-strict)
        $crate::quantity_quantity_partial_ord_interface!(
            ($($single_dimension_single_scale_params)*),
            $T, $rescale_fn
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _define_arithmetic {
    (($($single_dimension_single_scale_params:tt)*),
     ($($multiple_dimension_multiple_scale_params:tt)*),
     ($($inversion_where_clauses:tt)*),
     ($($mul_output_dimension_where_clauses:tt)*),
     ($($div_output_dimension_where_clauses:tt)*),
     $T:ty, $rescale_fn:ident) => {
        // scalar-quantity arithmetic operations
        $crate::scalar_quantity_mul_div_interface!(($($single_dimension_single_scale_params)*), ($($inversion_where_clauses)*), $T);

        $crate::quantity_scalar_mul_div_interface!(($($single_dimension_single_scale_params)*), *, mul, Mul, $T);
        $crate::quantity_scalar_mul_div_interface!(($($single_dimension_single_scale_params)*), /, div, Div, $T);

        $crate::quantity_scalar_mul_div_assign_interface!(($($single_dimension_single_scale_params)*), *=, mul_assign, MulAssign, $T);
        $crate::quantity_scalar_mul_div_assign_interface!(($($single_dimension_single_scale_params)*), /=, div_assign, DivAssign, $T);

        // quantity-quantity arithmetic operations
        $crate::quantity_quantity_add_sub_interface!(
            ($($single_dimension_single_scale_params)*),
            +, add, Add, $T, $rescale_fn
        );
        $crate::quantity_quantity_add_sub_interface!(
            ($($single_dimension_single_scale_params)*),
            -, sub, Sub, $T, $rescale_fn
        );

        $crate::quantity_quantity_add_sub_assign_interface!(
            ($($single_dimension_single_scale_params)*),
            +=, add_assign, AddAssign, $T, $rescale_fn
        );
        $crate::quantity_quantity_add_sub_assign_interface!(
            ($($single_dimension_single_scale_params)*),
            -=, sub_assign, SubAssign, $T, $rescale_fn
        );

        $crate::quantity_quantity_mul_div_interface!(
            ($($multiple_dimension_multiple_scale_params)*),
            ($($mul_output_dimension_where_clauses)*),
            *, +, mul, Mul, $T, $rescale_fn
        );
        $crate::quantity_quantity_mul_div_interface!(
            ($($multiple_dimension_multiple_scale_params)*),
            ($($div_output_dimension_where_clauses)*),
            /, -, div, Div, $T, $rescale_fn
        );

        // quantity-quantity comparison operations (scale-strict)
        $crate::quantity_quantity_partial_ord_interface!(
            ($($single_dimension_single_scale_params)*),
            $T, $rescale_fn
        );
    };
}
