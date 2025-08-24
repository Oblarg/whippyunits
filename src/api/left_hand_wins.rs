use core::ops::{Add, Div, Mul, Sub};

use crate::{
    Quantity,
    aggregate_conversion_factor,
    min_length_scale,
    min_mass_scale,
    min_time_scale,
    left_hand_wins_scale,
    IsIsize,
};

generate_arithmetic_ops!(LeftHandWins);