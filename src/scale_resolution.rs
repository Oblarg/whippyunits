use crate::scale_conversion::*;
use crate::api::*;

#[macro_export]
macro_rules! define_min_max_scale {
    ($fn_name:ident, $op:tt) => {
        pub const fn $fn_name(
            length_exponent_1: i8, length_scale_1: i8,
            length_exponent_2: i8, length_scale_2: i8,
        ) -> i8 {
            match (length_exponent_1, length_exponent_2) {
                (0, _) => length_scale_2,  // dimension not used, use other scale
                (_, 0) => length_scale_1,  // dimension not used, use other scale
                _ => {
                    if length_scale_1 $op length_scale_2 {
                        length_scale_1
                    } else {
                        length_scale_2
                    }
                }
            }
        }
    }
}

macro_rules! _define_min_max_composite_scale {
    (
        ($($prime_scales:tt)*),
        ($($defer_to_second:tt)*),
        ($($defer_to_first:tt)*),
        ($($compare_scales_let:tt)*),
        ($($compare_scales_if:tt)*),
        ($($compare_scales_first:tt)*),
        ($($compare_scales_second:tt)*),
        $fn:ident, $factor_fn:ident, $exponent1:ident, $exponent2:ident, $op:tt
    ) => {
        pub const fn $fn(
            which_prime: i8,
            $exponent1: i8,
            $($prime_scales)*,
            $exponent2: i8,
        ) -> i8 {
            // time scales are aggregate across primes, and we can't just mix-and-match or we end up with nonstandard scale values
            match ($exponent1, $exponent2) { 
                (0, _) => match which_prime {  // time dimension not used in first quantity
                    $($defer_to_second)*
                },
                (_, 0) => match which_prime {  // time dimension not used in second quantity
                    $($defer_to_first)*
                },
                _ => {
                    $($compare_scales_let)*
                    $($compare_scales_if)* {
                        match which_prime {
                            $($compare_scales_first)*
                        }
                    } else {
                        match which_prime {
                            $($compare_scales_second)*
                        }
                    }
                }
            }
        }
    }
}
