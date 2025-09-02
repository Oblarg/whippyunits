use crate::quantity_type::Quantity;
use crate::constants::*;

// All unused dimensions now use 0 (SI base unit scale) instead of sentinel values


// ============================================================================
// Extension Traits for Natural Syntax
// ============================================================================

pub type Milligram = Quantity<
    1, MILLIGRAM_SCALE_P10,
    0, 0,
    0, 0, 0, 0,
>;
pub type Gram = Quantity<
    1, GRAM_SCALE_P10,
    0, 0,
    0, 0, 0, 0,
>;
pub type Kilogram = Quantity<
    1, KILOGRAM_SCALE_P10,
    0, 0,
    0, 0, 0, 0,
>;

pub type Millimeter = Quantity<
    0, 0,
    1, MILLIMETER_SCALE_P10,
    0, 0, 0, 0,
>;
pub type Meter = Quantity<
    0, 0,
    1, METER_SCALE_P10,
    0, 0, 0, 0,
>;
pub type Kilometer = Quantity<
    0, 0,
    1, KILOMETER_SCALE_P10,
    0, 0, 0, 0,
>;

pub type Millisecond = Quantity<
    0, 0,
    0, 0,
    1, MILLISECOND_SCALE_P2, MILLISECOND_SCALE_P3, MILLISECOND_SCALE_P5,
>;
pub type Second = Quantity<
    0, 0,
    0, 0,
    1, SECOND_SCALE_P2, SECOND_SCALE_P3, SECOND_SCALE_P5,
>;
pub type Minute = Quantity<
    0, 0,
    0, 0,
    1, MINUTE_SCALE_P2, MINUTE_SCALE_P3, MINUTE_SCALE_P5,
>;

pub trait MassExt {
    fn milligrams(self) -> Milligram;
    fn grams(self) -> Gram;
    fn kilograms(self) -> Kilogram;
}

impl MassExt for f64 {
    fn milligrams(self) -> Milligram {
        Quantity::new(self)
    }

    fn grams(self) -> Gram {
        Quantity::new(self)
    }

    fn kilograms(self) -> Kilogram {
        Quantity::new(self)
    }
}

impl MassExt for i32 {
    fn milligrams(self) -> Milligram {
        Quantity::new(self as f64)
    }

    fn grams(self) -> Gram {
        Quantity::new(self as f64)
    }

    fn kilograms(self) -> Kilogram {
        Quantity::new(self as f64)
    }
}

pub trait LengthExt {
    fn millimeters(self) -> Millimeter;
    fn meters(self) -> Meter;
    fn kilometers(self) -> Kilometer;
}

impl LengthExt for f64 {
    fn millimeters(self) -> Millimeter {
        Quantity::new(self)
    }

    fn meters(self) -> Meter {
        Quantity::new(self)
    }

    fn kilometers(self) -> Kilometer {
        Quantity::new(self)
    }
}

impl LengthExt for i32 {
    fn millimeters(self) -> Millimeter {
        Quantity::new(self as f64)
    }

    fn meters(self) -> Meter {
        Quantity::new(self as f64)
    }

    fn kilometers(self) -> Kilometer {
        Quantity::new(self as f64)
    }
}

pub trait TimeExt {
    fn milliseconds(self) -> Millisecond;
    fn seconds(self) -> Second;
    fn minutes(self) -> Minute;
}

impl TimeExt for f64 {
    fn milliseconds(self) -> Millisecond {
        Quantity::new(self)
    }

    fn seconds(self) -> Second {
        Quantity::new(self)
    }

    fn minutes(self) -> Minute {
        Quantity::new(self)
    }
}

impl TimeExt for i32 {
    fn milliseconds(self) -> Millisecond {
        Quantity::new(self as f64)
    }

    fn seconds(self) -> Second {
        Quantity::new(self as f64)
    }

    fn minutes(self) -> Minute {
        Quantity::new(self as f64)
    }
}

define_unit_macro!(
    0,
    0,
    0, 0, 0
);

pub use unit;