use crate::constants::*;
use crate::scale_conversion::*;
use crate::quantity_type::Quantity;

macro_rules! min_max_length_scale {
    ($fn_name:ident, $op:tt) => {
        pub const fn $fn_name(
            length_exponent_1: isize, length_scale_1: isize,
            length_exponent_2: isize, length_scale_2: isize,
        ) -> isize {
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

min_max_length_scale!(min_length_scale, <);
min_max_length_scale!(max_length_scale, >);

macro_rules! min_max_mass_scale {
    ($fn_name:ident, $op:tt) => {
        pub const fn $fn_name(
            mass_exponent_1: isize, mass_scale_1: isize,
            mass_exponent_2: isize, mass_scale_2: isize,
        ) -> isize {
            match (mass_exponent_1, mass_exponent_2) {
                (0, _) => mass_scale_2,  // dimension not used, use other scale
                (_, 0) => mass_scale_1,  // dimension not used, use other scale
                _ => {
                    if mass_scale_1 $op mass_scale_2 {
                        mass_scale_1
                    } else {
                        mass_scale_2
                    }
                }
            }
        }
    }
}

min_max_mass_scale!(min_mass_scale, <);
min_max_mass_scale!(max_mass_scale, >);

macro_rules! min_max_time_scale {
    ($fn_name:ident, $op:tt) => {
        pub const fn $fn_name(
            which_prime: isize,
            time_exponent_1: isize,
            p2_1: isize, p3_1: isize, p5_1: isize,
            time_exponent_2: isize,
            p2_2: isize, p3_2: isize, p5_2: isize,
        ) -> isize {
            // time scales are aggregate across primes, and we can't just mix-and-match or we end up with nonstandard scale values
            match (time_exponent_1, time_exponent_2) { 
                (0, _) => match which_prime {  // time dimension not used in first quantity
                    2 => p2_2,
                    3 => p3_2,
                    5 => p5_2,
                    _ => 0, // should never happen, but use 0 for unused
                },
                (_, 0) => match which_prime {  // time dimension not used in second quantity
                    2 => p2_1,
                    3 => p3_1,
                    5 => p5_1,
                    _ => 0, // should never happen, but use 0 for unused
                },
                _ => {
                    if time_conversion_factor(
                        0, 0, 0, p2_1, p3_1, p5_1, 1,
                    ) $op time_conversion_factor(
                        0, 0, 0, p2_2, p3_2, p5_2, 1,
                    ) {
                        match which_prime {
                            2 => p2_1,
                            3 => p3_1,
                            5 => p5_1,
                            _ => 0, // should never happen, but use 0 for unused
                        }
                    } else {
                        match which_prime {
                            2 => p2_2,
                            3 => p3_2,
                            5 => p5_2,
                            _ => 0, // should never happen, but use 0 for unused
                        }
                    }
                }
            }
        }
    }
}

min_max_time_scale!(min_time_scale, <);
min_max_time_scale!(max_time_scale, >);