#[derive(Clone, Copy, PartialEq)]
pub struct Quantity<
    const MASS_EXPONENT: i8, const MASS_SCALE_P10: i8,
    const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10: i8,
    const TIME_EXPONENT: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8, 
    T = f64,
> {
    pub value: T,
}

impl<
    const MASS_EXPONENT: i8, const MASS_SCALE_P10: i8,
    const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10: i8,
    const TIME_EXPONENT: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8, 
    T,
>
    Quantity<
        MASS_EXPONENT, MASS_SCALE_P10, 
        LENGTH_EXPONENT, LENGTH_SCALE_P10, 
        TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
        T,
    >
{
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

#[macro_export]
macro_rules! quantity_type {
    () => {
        Quantity<
            MASS_EXPONENT, MASS_SCALE_P10,
            LENGTH_EXPONENT, LENGTH_SCALE_P10,
            TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            T,
        >
    };
    ($T:ty) => {
        Quantity<
            MASS_EXPONENT, MASS_SCALE_P10,
            LENGTH_EXPONENT, LENGTH_SCALE_P10,
            TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5,
            $T,
        >
    };
}