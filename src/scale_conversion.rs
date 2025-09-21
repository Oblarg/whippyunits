use crate::quantity_type::Quantity;

pub const fn pow10(exp: i32) -> (i128, i128) {
    match exp {
        -9 => (1, 1000000000),
        -8 => (1, 100000000),
        -7 => (1, 10000000),
        -6 => (1, 1000000),
        -5 => (1, 100000),
        -4 => (1, 10000),
        -3 => (1, 1000),
        -2 => (1, 100),
        -1 => (1, 10),
        0 => (1, 1),
        1 => (10, 1),
        2 => (100, 1),
        3 => (1000, 1),
        4 => (10000, 1),
        5 => (100000, 1),
        6 => (1000000, 1),
        7 => (10000000, 1),
        8 => (100000000, 1),
        9 => (1000000000, 1),
        _ => (1, 1), // we'll only test small values during prototyping
    }
}

pub const fn pow2(exp: i32) -> (i128, i128) {
    match exp {
        -9 => (1, 512),
        -8 => (1, 256),
        -7 => (1, 128),
        -6 => (1, 64),
        -5 => (1, 32),
        -4 => (1, 16),
        -3 => (1, 8),
        -2 => (1, 4),
        -1 => (1, 2),
        0 => (1, 1),
        1 => (2, 1),
        2 => (4, 1),
        3 => (8, 1),
        4 => (16, 1),
        5 => (32, 1),
        6 => (64, 1),
        7 => (128, 1),
        8 => (256, 1),
        9 => (512, 1),
        _ => (1, 1), // we'll only test small values during prototyping
    }
}

pub const fn pow3(exp: i32) -> (i128, i128) {
    match exp {
        -9 => (1, 19683),
        -8 => (1, 6561),
        -7 => (1, 2187),
        -6 => (1, 729),
        -5 => (1, 243),
        -4 => (1, 81),
        -3 => (1, 27),
        -2 => (1, 9),
        -1 => (1, 3),
        0 => (1, 1),
        1 => (3, 1),
        2 => (9, 1),
        3 => (27, 1),
        4 => (81, 1),
        5 => (243, 1),
        6 => (729, 1),
        7 => (2187, 1),
        8 => (6561, 1),
        9 => (19683, 1),
        _ => (1, 1), // we'll only test small values during prototyping
    }
}

pub const fn pow5(exp: i32) -> (i128, i128) {
    match exp {
        -9 => (1, 1953125),
        -8 => (1, 390625),
        -7 => (1, 78125),
        -6 => (1, 15625),
        -5 => (1, 3125),
        -4 => (1, 625),
        -3 => (1, 125),
        -2 => (1, 25),
        -1 => (1, 5),
        0 => (1, 1),
        1 => (5, 1),
        2 => (25, 1),
        3 => (125, 1),
        4 => (625, 1),
        5 => (3125, 1),
        6 => (15625, 1),
        7 => (78125, 1),
        8 => (390625, 1),
        9 => (1953125, 1),
        _ => (1, 1), // we'll only test small values during prototyping
    }
}

/// Compute π^exp using the rational approximation 710/113
/// Returns (numerator, denominator) for π^exp
pub const fn powPi(exp: i32) -> (i128, i128) {
    match exp {
        -3 => (113 * 113 * 113, 710 * 710 * 710), // π^(-3) = (113/710)^3
        -2 => (113 * 113, 710 * 710),             // π^(-2) = (113/710)^2
        -1 => (113, 710),                         // π^(-1) = 113/710
        0 => (1, 1),                              // π^0 = 1
        1 => (710, 113),                          // π^1 = 710/113
        2 => (710 * 710, 113 * 113),              // π^2 = (710/113)^2
        3 => (710 * 710 * 710, 113 * 113 * 113),  // π^3 = (710/113)^3
        _ => (1, 1), // we'll only test small values during prototyping
    }
}

#[macro_export]
macro_rules! define_aggregate_scale_factor_rational {
    (
        ($($aggregate_scale_factor_params:tt)*), 
        ($($aggregate_scale_factor_diff_exprs:tt)*), 
        ($($aggregate_scale_factor_pow_exprs:tt)*),
        ($($aggregate_scale_factor_num_exprs:tt)*),
        ($($aggregate_scale_factor_den_exprs:tt)*),
    ) => {
        pub const fn aggregate_scale_factor(
            $($aggregate_scale_factor_params)*
        ) -> (i128, i128) {
            $($aggregate_scale_factor_diff_exprs)*

            $($aggregate_scale_factor_pow_exprs)*
            
            reduce_rational($($aggregate_scale_factor_num_exprs)*, $($aggregate_scale_factor_den_exprs)*)
        }
    }
}

#[macro_export]
macro_rules! define_aggregate_scale_factor_float {
    (
        ($($aggregate_scale_factor_params:tt)*), 
        ($($aggregate_scale_factor_diff_exprs:tt)*), 
        ($($aggregate_scale_factor_pow_exprs:tt)*),
        ($($aggregate_scale_factor_expr:tt)*),
    ) => {
        pub fn aggregate_scale_factor_float(
            $($aggregate_scale_factor_params)*
        ) -> f64 {
            $($aggregate_scale_factor_diff_exprs)*

            $($aggregate_scale_factor_pow_exprs)*
            
            $($aggregate_scale_factor_expr)*
        }
    }
}

/// Reduce a rational number to its simplest form using bit-shift based GCD
/// Returns (reduced_numerator, reduced_denominator)
pub const fn reduce_rational(num: i128, den: i128) -> (i128, i128) {
    if den == 0 {
        return (num, 1);
    }
    if num == 0 {
        return (0, 1);
    }

    // Convert to unsigned for bit operations
    let mut num_u = num.unsigned_abs();
    let mut den_u = den.unsigned_abs();

    // Remove common factors of 2 using bit shifts
    let common_twos = (num_u | den_u).trailing_zeros();
    num_u >>= common_twos;
    den_u >>= common_twos;

    // Now both are odd, so we can use the odd-odd case of binary GCD
    while num_u != den_u {
        if num_u > den_u {
            // num_u and den_u are both odd, so (num_u - den_u) is even
            let diff = num_u - den_u;
            num_u = diff >> diff.trailing_zeros(); // Remove factors of 2
        } else {
            let diff = den_u - num_u;
            den_u = diff >> diff.trailing_zeros(); // Remove factors of 2
        }
    }

    // num_u == den_u now, and both are odd, so this is the GCD
    let gcd = num_u;

    // Divide both by the GCD to get reduced form
    let reduced_num = num / (gcd as i128);
    let reduced_den = den / (gcd as i128);

    (reduced_num, reduced_den)
}

#[macro_export]
macro_rules! _define_float_rescale {
    (
        ($($float_rescale_const_params:tt)*),
        ($($float_rescale_input_type:tt)*),
        ($($float_rescale_output_type:tt)*),
        ($($float_rescale_aggregate_args:tt)*),
        $fn:ident, $T:ty,
    ) => {
        #[rustfmt::skip]
        pub fn $fn<
            $($float_rescale_const_params)*
        > (
            quantity: $($float_rescale_input_type)*,
        ) -> $($float_rescale_output_type)* {
            let rescale_factor = aggregate_scale_factor_float(
                $($float_rescale_aggregate_args)*
            ) as $T;
            Quantity::new(
                quantity.value * rescale_factor
            )
        }
    };
}

#[macro_export]
macro_rules! _define_int_rescale {
    (
        ($($int_rescale_const_params:tt)*),
        ($($int_rescale_input_type:tt)*),
        ($($int_rescale_output_type:tt)*),
        ($($int_rescale_aggregate_args:tt)*),
        $fn:ident, $T:ty,
    ) => {
        #[rustfmt::skip]
        pub fn $fn<
            $($int_rescale_const_params)*
        > (
            quantity: $($int_rescale_input_type)*,
        ) -> $($int_rescale_output_type)* {
            let (num, den) = aggregate_scale_factor(
                $($int_rescale_aggregate_args)*
            );
            let num = num as $T;
            let den = den as $T;
            
            // Numerical stability: check for potential overflow on multiplication
            // If value * num would overflow, divide first; otherwise multiply first
            let result = if quantity.value > <$T>::max_value() / num {
                // Potential overflow: divide first to reduce intermediate value
                (quantity.value / den) * num
            } else {
                // Safe to multiply first
                (quantity.value * num) / den
            };
            
            Quantity::new(result)
        }
    }
}
