use core::ops::{Add, Div, Mul, Sub};

use crate::{
    Quantity, IsIsize, aggregate_conversion_factor,
};

generate_arithmetic_ops!(Strict);