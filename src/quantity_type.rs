// Auto-generated Quantity type definition
// Generated from dimensional_metadata.rs
// DO NOT EDIT - This file is auto-generated

use std::f64;
use std::ops::{Add, Div, Mul, Sub};
use nalgebra::Scalar;

#[derive(Clone, Copy, PartialEq)]
pub struct Quantity<
    const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
    const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
    T = f64,
> {
    pub value: T,
}

impl<
    const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
    const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
    T,
>
    single_quantity_type!()
{
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

// ============================================================================
// Display and Debug Implementations
// ============================================================================

use std::fmt;
use crate::print::prettyprint::pretty_print_quantity_value;

impl<
    const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
    const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
    T,
>
    fmt::Display
    for single_quantity_type!()
where
    T: Copy + Into<f64>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pretty = pretty_print_quantity_value(
            self.value.into(),
            MASS_EXPONENT, MASS_SCALE_P10,
            LENGTH_EXPONENT, LENGTH_SCALE_P10,
            TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            false, // Non-verbose mode for Display
        );
        write!(f, "{}", pretty)
    }
}

impl<
    const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
    const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
    T,
>
    fmt::Debug
    for single_quantity_type!()
where
    T: Copy + Into<f64>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pretty = pretty_print_quantity_value(
            self.value.into(),
            MASS_EXPONENT, MASS_SCALE_P10,
            LENGTH_EXPONENT, LENGTH_SCALE_P10,
            TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            true, // Verbose mode for Debug
        );
        write!(f, "{}", pretty)
    }
}

impl<
    const MASS_EXPONENT: isize, const MASS_SCALE_P10: isize,
    const LENGTH_EXPONENT: isize, const LENGTH_SCALE_P10: isize,
    const TIME_EXPONENT: isize, const TIME_SCALE_P2: isize, const TIME_SCALE_P3: isize, const TIME_SCALE_P5: isize,
    T,
>
    From<T> for single_quantity_type!()
{
    fn from(value: T) -> Self {
        Self::new(value.into())
    }
}