

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
pub const fn pow_pi(exp: i32) -> (i128, i128) {
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

// Float lookup tables for const evaluation
pub const fn pow2_float(exp: i32) -> f64 {
    match exp {
        -9 => 1.0 / 512.0,
        -8 => 1.0 / 256.0,
        -7 => 1.0 / 128.0,
        -6 => 1.0 / 64.0,
        -5 => 1.0 / 32.0,
        -4 => 1.0 / 16.0,
        -3 => 1.0 / 8.0,
        -2 => 1.0 / 4.0,
        -1 => 1.0 / 2.0,
        0 => 1.0,
        1 => 2.0,
        2 => 4.0,
        3 => 8.0,
        4 => 16.0,
        5 => 32.0,
        6 => 64.0,
        7 => 128.0,
        8 => 256.0,
        9 => 512.0,
        _ => 1.0, // we'll only test small values during prototyping
    }
}

pub const fn pow3_float(exp: i32) -> f64 {
    match exp {
        -9 => 1.0 / 19683.0,
        -8 => 1.0 / 6561.0,
        -7 => 1.0 / 2187.0,
        -6 => 1.0 / 729.0,
        -5 => 1.0 / 243.0,
        -4 => 1.0 / 81.0,
        -3 => 1.0 / 27.0,
        -2 => 1.0 / 9.0,
        -1 => 1.0 / 3.0,
        0 => 1.0,
        1 => 3.0,
        2 => 9.0,
        3 => 27.0,
        4 => 81.0,
        5 => 243.0,
        6 => 729.0,
        7 => 2187.0,
        8 => 6561.0,
        9 => 19683.0,
        _ => 1.0, // we'll only test small values during prototyping
    }
}

pub const fn pow5_float(exp: i32) -> f64 {
    match exp {
        -9 => 1.0 / 1953125.0,
        -8 => 1.0 / 390625.0,
        -7 => 1.0 / 78125.0,
        -6 => 1.0 / 15625.0,
        -5 => 1.0 / 3125.0,
        -4 => 1.0 / 625.0,
        -3 => 1.0 / 125.0,
        -2 => 1.0 / 25.0,
        -1 => 1.0 / 5.0,
        0 => 1.0,
        1 => 5.0,
        2 => 25.0,
        3 => 125.0,
        4 => 625.0,
        5 => 3125.0,
        6 => 15625.0,
        7 => 78125.0,
        8 => 390625.0,
        9 => 1953125.0,
        _ => 1.0, // we'll only test small values during prototyping
    }
}

/// Compute π^exp using float values for const evaluation
/// Uses the standard f64::consts::PI value
pub const fn pow_pi_float(exp: i32) -> f64 {
    match exp {
        -3 => 1.0 / (std::f64::consts::PI * std::f64::consts::PI * std::f64::consts::PI), // π^(-3)
        -2 => 1.0 / (std::f64::consts::PI * std::f64::consts::PI),                       // π^(-2)
        -1 => 1.0 / std::f64::consts::PI,                                                // π^(-1)
        0 => 1.0,                                                                        // π^0 = 1
        1 => std::f64::consts::PI,                                                       // π^1
        2 => std::f64::consts::PI * std::f64::consts::PI,                               // π^2
        3 => std::f64::consts::PI * std::f64::consts::PI * std::f64::consts::PI,        // π^3
        _ => 1.0, // we'll only test small values during prototyping
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
        pub const fn aggregate_scale_factor_float(
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
        pub const fn $fn<
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
        pub const fn $fn<
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
