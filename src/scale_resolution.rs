use crate::constants::*;
use crate::scale_conversion::*;
use crate::quantity_type::Quantity;

macro_rules! min_max_length_scale {
    ($fn_name:ident, $op:tt) => {
        pub const fn $fn_name(
            length_1: isize, length_2: isize,
        ) -> isize {
            match (length_1, length_2) {
                (LENGTH_UNUSED, _) => length_2,
                (_, LENGTH_UNUSED) => length_1,
                _ => {
                    if length_1 $op length_2 {
                        length_1
                    } else {
                        length_2
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
            mass_1: isize, mass_2: isize,
        ) -> isize {
            match (mass_1, mass_2) {
                (MASS_UNUSED, _) => mass_2,
                (_, MASS_UNUSED) => mass_1,
                _ => {
                    if mass_1 $op mass_2 {
                        mass_1
                    } else {
                        mass_2
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
            p2_1: isize, p3_1: isize, p5_1: isize,
            p2_2: isize, p3_2: isize, p5_2: isize,
        ) -> isize {
            // time scales are aggregate across primes, and we can't just mix-and-match or we end up with nonstandard scale values
            match (p2_1, p3_1, p5_1, p2_2, p3_2, p5_2) { 
                (TIME_UNUSED, _, _, _, _, _)
                | (_, TIME_UNUSED, _, _, _, _)
                | (_, _, TIME_UNUSED, _, _, _) => match which_prime {
                    2 => p2_2,
                    3 => p3_2,
                    5 => p5_2,
                    _ => TIME_UNUSED, // should never happen
                },
                (_, _, _, TIME_UNUSED, _, _)
                | (_, _, _, _, TIME_UNUSED, _)
                | (_, _, _, _, _, TIME_UNUSED) => match which_prime {
                    2 => p2_1,
                    3 => p3_1,
                    5 => p5_1,
                    _ => TIME_UNUSED, // should never happen
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
                            _ => TIME_UNUSED, // should never happen
                        }
                    } else {
                        match which_prime {
                            2 => p2_2,
                            3 => p3_2,
                            5 => p5_2,
                            _ => TIME_UNUSED, // should never happen
                        }
                    }
                }
            }
        }
    }
}

min_max_time_scale!(min_time_scale, <);
min_max_time_scale!(max_time_scale, >);