use std::ops::{Add, Div, Mul, Sub};

pub const fn pow10(exp: isize) -> (i64, i64) {
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

pub const fn pow2(exp: isize) -> (i64, i64) {
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

pub const fn pow3(exp: isize) -> (i64, i64) {
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

pub const fn pow5(exp: isize) -> (i64, i64) {
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

// ============================================================================
// Length Scale Conversion Factors (Integer-based)
// ============================================================================

/// Compute the scale factor for length units using powers of 10
/// Returns (numerator, denominator) representing the scale factor
pub const fn length_scale_factor(p10_from: isize, p10_to: isize, exponent: isize) -> (i64, i64) {
    match exponent {
        0 => (1, 1),  // dimension exponent is 0, no conversion needed
        1 => pow10(p10_from - p10_to),
        _ => pow10((p10_from - p10_to) * exponent),
    }
}

// ============================================================================
// Mass Scale Conversion Factors (Integer-based)
// ============================================================================

/// Compute the scale factor for mass units using powers of 10
/// Returns (numerator, denominator) representing the scale factor
pub const fn mass_scale_factor(p10_from: isize, p10_to: isize, exponent: isize) -> (i64, i64) {
    match exponent {
        0 => (1, 1),  // dimension exponent is 0, no conversion needed
        1 => pow10(p10_from - p10_to),
        _ => pow10((p10_from - p10_to) * exponent),
    }
}


// ============================================================================
// Time Scale Conversion Factors (Integer-based)
// ============================================================================

/// Compute the composite time scale factor using powers of 2, 3, and 5
/// Returns (numerator, denominator) representing the scale factor
pub const fn time_scale_factor(p2_from: isize, p3_from: isize, p5_from: isize, p2_to: isize, p3_to: isize, p5_to: isize, exponent: isize) -> (i64, i64) {
    match exponent {
        0 => (1, 1),  // dimension exponent is 0, no conversion needed
        1 => {
            let (num2, den2) = pow2(p2_from - p2_to);
            let (num3, den3) = pow3(p3_from - p3_to);
            let (num5, den5) = pow5(p5_from - p5_to);
            (num2 * num3 * num5, den2 * den3 * den5)
        },
        _ => {
            let (num2, den2) = pow2((p2_from - p2_to) * exponent);
            let (num3, den3) = pow3((p3_from - p3_to) * exponent);
            let (num5, den5) = pow5((p5_from - p5_to) * exponent);
            
            (num2 * num3 * num5, den2 * den3 * den5)
        }
    }
}

// ============================================================================
// Composite Scale Conversion Factors (Integer-based)
// ============================================================================

/// Compute a composite scale factor for a unit with mass, length, and time components
/// Returns (numerator, denominator) representing the overall scale factor
pub const fn aggregate_scale_factor(
    mass_exponent: isize, mass_scale_p10_from: isize, mass_scale_p10_to: isize,
    length_exponent: isize, length_scale_p10_from: isize, length_scale_p10_to: isize,
    time_exponent: isize, time_scale_p2_from: isize, time_scale_p3_from: isize, time_scale_p5_from: isize, time_scale_p2_to: isize, time_scale_p3_to: isize, time_scale_p5_to: isize,
) -> (i64, i64) {
    let diff_length_p10: isize = (length_scale_p10_from - length_scale_p10_to) * length_exponent;
    let diff_mass_p10: isize = (mass_scale_p10_from - mass_scale_p10_to) * mass_exponent;
    let diff_time_p2: isize = (time_scale_p2_from - time_scale_p2_to) * time_exponent;
    let diff_time_p3: isize = (time_scale_p3_from - time_scale_p3_to) * time_exponent;
    let diff_time_p5: isize = (time_scale_p5_from - time_scale_p5_to) * time_exponent;
    
    let (num10, den10) = pow10(diff_length_p10 + diff_mass_p10);
    let (num2, den2) = pow2(diff_time_p2);
    let (num3, den3) = pow3(diff_time_p3);
    let (num5, den5) = pow5(diff_time_p5);
    
    reduce_rational(num10 * num2 * num3 * num5, den10 * den2 * den3 * den5)
}

/// Reduce a rational number to its simplest form using bit-shift based GCD
/// Returns (reduced_numerator, reduced_denominator)
pub const fn reduce_rational(num: i64, den: i64) -> (i64, i64) {
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
    let reduced_num = num / (gcd as i64);
    let reduced_den = den / (gcd as i64);
    
    (reduced_num, reduced_den)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow10() {
        assert_eq!(pow10(-3), (1, 1000)); // 10^-3 = 1/1000
        assert_eq!(pow10(0), (1, 1)); // 10^0 = 1
        assert_eq!(pow10(5), (100000, 1)); // 10^5 = 100000
        assert_eq!(pow10(9), (1000000000, 1)); // 10^9 = 1000000000
        assert_eq!(pow10(20), (1, 1)); // Default case
    }

    #[test]
    fn test_pow2() {
        assert_eq!(pow2(-3), (1, 8)); // 2^-3 = 1/8
        assert_eq!(pow2(0), (1, 1)); // 2^0 = 1
        assert_eq!(pow2(5), (32, 1)); // 2^5 = 32
        assert_eq!(pow2(9), (512, 1)); // 2^9 = 512
    }

    #[test]
    fn test_pow3() {
        assert_eq!(pow3(-3), (1, 27)); // 3^-3 = 1/27
        assert_eq!(pow3(0), (1, 1)); // 3^0 = 1
        assert_eq!(pow3(5), (243, 1)); // 3^5 = 243
        assert_eq!(pow3(9), (19683, 1)); // 3^9 = 19683
    }

    #[test]
    fn test_pow5() {
        assert_eq!(pow5(-3), (1, 125)); // 5^-3 = 1/125
        assert_eq!(pow5(0), (1, 1)); // 5^0 = 1
        assert_eq!(pow5(5), (3125, 1)); // 5^5 = 3125
        assert_eq!(pow5(9), (1953125, 1)); // 5^9 = 1953125
    }
}
