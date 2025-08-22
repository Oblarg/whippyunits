use crate::*;

// ============================================================================
// Extension Traits for Natural Syntax
// ============================================================================

type Millimeter = Quantity<
    1, MILLIMETER_SCALE,
    0, MASS_UNUSED,
    0, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED,
    { RescaleBehavior::Strict }, { CancelledScaleBehavior::Retain },
>;
type Meter = Quantity<
    1, METER_SCALE,
    0, MASS_UNUSED,
    0, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED,
    { RescaleBehavior::Strict }, { CancelledScaleBehavior::Retain },
>;
type Kilometer = Quantity<
    1, KILOMETER_SCALE,
    0, MASS_UNUSED,
    0, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED,
    { RescaleBehavior::Strict }, { CancelledScaleBehavior::Retain },
>;

type Milligram = Quantity<
    0, LENGTH_UNUSED,
    1, MILLIGRAM_SCALE,
    0, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED,
    { RescaleBehavior::Strict }, { CancelledScaleBehavior::Retain },
>;
type Gram = Quantity<
    0, LENGTH_UNUSED,
    1, GRAM_SCALE,
    0, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED,
    { RescaleBehavior::Strict }, { CancelledScaleBehavior::Retain },
>;
type Kilogram = Quantity<
    0, LENGTH_UNUSED,
    1, KILOGRAM_SCALE,
    0, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED, TIME_UNUSED,
    { RescaleBehavior::Strict }, { CancelledScaleBehavior::Retain },
>;

type Millisecond = Quantity<
    0, LENGTH_UNUSED,
    0, MASS_UNUSED,
    1, MILLISECOND_SCALE_P2, MILLISECOND_SCALE_P3, MILLISECOND_SCALE_P5, MILLISECOND_SCALE_ORDER,
    { RescaleBehavior::Strict }, { CancelledScaleBehavior::Retain },
>;
type Second = Quantity<
    0, LENGTH_UNUSED,
    0, MASS_UNUSED,
    1, SECOND_SCALE_P2, SECOND_SCALE_P3, SECOND_SCALE_P5, SECOND_SCALE_ORDER,
    { RescaleBehavior::Strict }, { CancelledScaleBehavior::Retain },
>;
type Minute = Quantity<
    0, LENGTH_UNUSED,
    0, MASS_UNUSED,
    1, MINUTE_SCALE_P2, MINUTE_SCALE_P3, MINUTE_SCALE_P5, MINUTE_SCALE_ORDER,
    { RescaleBehavior::Strict }, { CancelledScaleBehavior::Retain },
>;

pub trait LengthExt {
    fn meters(self) -> Meter;
    fn millimeters(self) -> Millimeter;
    fn kilometers(self) -> Kilometer;
}

impl LengthExt for f64 {
    fn meters(self) -> Meter {
        Quantity::new(self)
    }

    fn millimeters(self) -> Millimeter {
        Quantity::new(self)
    }

    fn kilometers(self) -> Kilometer {
        Quantity::new(self)
    }
}

impl LengthExt for i32 {
    fn meters(self) -> Meter {
        Quantity::new(self as f64)
    }

    fn millimeters(self) -> Millimeter {
        Quantity::new(self as f64)
    }

    fn kilometers(self) -> Kilometer {
        Quantity::new(self as f64)
    }
}

pub trait MassExt {
    fn kilograms(self) -> Kilogram;
    fn milligrams(self) -> Milligram;
    fn grams(self) -> Gram;
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