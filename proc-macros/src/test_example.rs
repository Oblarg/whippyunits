// This is just an example of how the macro would be used
// In practice, this would be in the main crate

// Example usage:
// define_generic_dimension!(LengthOrMass, Length, Mass);

// This would expand to:
/*
pub trait LengthOrMass {
    type Unit;
}

impl <
    const LENGTH_SCALE_P10: isize,
> LengthOrMass for Quantity<
    1, LENGTH_SCALE_P10,
    0, 0,
    0, 0, 0, 0,
> {
    type Unit = Self;
}

impl <
    const MASS_SCALE_P10: isize,
> LengthOrMass for Quantity<
    0, 0,
    1, MASS_SCALE_P10,
    0, 0, 0, 0,
> {
    type Unit = Self;
}
*/
