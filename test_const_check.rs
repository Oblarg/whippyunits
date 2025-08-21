#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

use std::marker::ConstParamTy;

// Helper for const expressions in where bounds
pub struct ConstCheck<const CHECK: bool>;
pub trait True {}
impl True for ConstCheck<true> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum LengthScale {
    Millimeter = 1,
    Meter = 2,
    Kilometer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum MassScale {
    Milligram = 1,
    Gram = 2,
    Kilogram = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum AdditionBehavior {
    SmallerWins,
    LeftHandWins,
    LargerWins,
    Strict,
}

// Helper functions for scale operations
const fn min_scale(a: LengthScale, b: LengthScale) -> LengthScale {
    if (a as u32) <= (b as u32) { a } else { b }
}

const fn max_scale(a: LengthScale, b: LengthScale) -> LengthScale {
    if (a as u32) >= (b as u32) { a } else { b }
}

const fn scale_diff(from: LengthScale, to: LengthScale) -> i32 {
    (from as i32) - (to as i32)
}

const fn pow1000(exp: i32) -> f64 {
    match exp {
        -2 => 0.000001,
        -1 => 0.001,
        0 => 1.0,
        1 => 1000.0,
        2 => 1000000.0,
        _ => panic!("Power too large for f64 precision"),
    }
}

// Time scale conversion using prime factorization
const fn pow2(exp: i32) -> f64 {
    match exp {
        -3 => 0.125,
        -2 => 0.25,
        -1 => 0.5,
        0 => 1.0,
        1 => 2.0,
        2 => 4.0,
        3 => 8.0,
        _ => panic!("Power too large for f64 precision"),
    }
}

const fn pow3(exp: i32) -> f64 {
    match exp {
        -2 => 0.1111111111111111,
        -1 => 0.3333333333333333,
        0 => 1.0,
        1 => 3.0,
        2 => 9.0,
        _ => panic!("Power too large for f64 precision"),
    }
}

const fn pow5(exp: i32) -> f64 {
    match exp {
        -2 => 0.04,
        -1 => 0.2,
        0 => 1.0,
        1 => 5.0,
        2 => 25.0,
        _ => panic!("Power too large for f64 precision"),
    }
}

// Compute conversion factor for length scales
const fn length_conversion_factor(from: LengthScale, to: LengthScale) -> f64 {
    let diff = scale_diff(from, to);
    pow1000(diff)
}

// Compute conversion factor for time scales using prime factorization
const fn time_conversion_factor(from_p2: i32, from_p3: i32, from_p5: i32, to_p2: i32, to_p3: i32, to_p5: i32) -> f64 {
    let d2 = from_p2 - to_p2;
    let d3 = from_p3 - to_p3;
    let d5 = from_p5 - to_p5;
    pow2(d2) * pow3(d3) * pow5(d5)
}

// Systematic parameter organization
#[derive(Debug, Clone, Copy)]
struct Quantity<
    // Distance dimension and scale
    const DISTANCE_EXPONENT: i32,
    const DISTANCE_SCALE: LengthScale,
    
    // Time dimension and scale (prime exponents)
    const TIME_EXPONENT: i32,
    const TIME_P2: i32, const TIME_P3: i32, const TIME_P5: i32,
    
    // Mass dimension and scale
    const MASS_EXPONENT: i32,
    const MASS_SCALE: MassScale,
    
    // Addition behavior
    const ADDITION_BEHAVIOR: AdditionBehavior
> {
    value: f64,
}

impl<
    const DISTANCE_EXPONENT: i32,
    const DISTANCE_SCALE: LengthScale,
    const TIME_EXPONENT: i32,
    const TIME_P2: i32, const TIME_P3: i32, const TIME_P5: i32,
    const MASS_EXPONENT: i32,
    const MASS_SCALE: MassScale,
    const ADDITION_BEHAVIOR: AdditionBehavior
> Quantity<DISTANCE_EXPONENT, DISTANCE_SCALE, TIME_EXPONENT, TIME_P2, TIME_P3, TIME_P5, MASS_EXPONENT, MASS_SCALE, ADDITION_BEHAVIOR> {
    fn new(value: f64) -> Self {
        Self { value }
    }
}

// Helper function to determine result scale based on addition behavior
const fn result_length_scale(a: LengthScale, b: LengthScale, behavior: AdditionBehavior) -> LengthScale {
    match behavior {
        AdditionBehavior::SmallerWins => min_scale(a, b),
        AdditionBehavior::LargerWins => max_scale(a, b),
        AdditionBehavior::LeftHandWins => a,
        AdditionBehavior::Strict => a, // Should be same as b
    }
}

// Test addition with systematic parameters
use std::ops::Add;

impl<
    const DISTANCE_EXPONENT1: i32,
    const DISTANCE_SCALE1: LengthScale,
    const TIME_EXPONENT1: i32,
    const TIME_P2_1: i32, const TIME_P3_1: i32, const TIME_P5_1: i32,
    const MASS_EXPONENT1: i32,
    const MASS_SCALE1: MassScale,
    const ADDITION_BEHAVIOR1: AdditionBehavior,
    const DISTANCE_EXPONENT2: i32,
    const DISTANCE_SCALE2: LengthScale,
    const TIME_EXPONENT2: i32,
    const TIME_P2_2: i32, const TIME_P3_2: i32, const TIME_P5_2: i32,
    const MASS_EXPONENT2: i32,
    const MASS_SCALE2: MassScale,
    const ADDITION_BEHAVIOR2: AdditionBehavior
> Add<Quantity<DISTANCE_EXPONENT2, DISTANCE_SCALE2, TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, MASS_EXPONENT2, MASS_SCALE2, ADDITION_BEHAVIOR2>>
for Quantity<DISTANCE_EXPONENT1, DISTANCE_SCALE1, TIME_EXPONENT1, TIME_P2_1, TIME_P3_1, TIME_P5_1, MASS_EXPONENT1, MASS_SCALE1, ADDITION_BEHAVIOR1>
where
    [(); { result_length_scale(DISTANCE_SCALE1, DISTANCE_SCALE2, ADDITION_BEHAVIOR1) } as usize]:,
{
    type Output = Quantity<
        DISTANCE_EXPONENT1,
        { result_length_scale(DISTANCE_SCALE1, DISTANCE_SCALE2, ADDITION_BEHAVIOR1) },
        TIME_EXPONENT1,
        TIME_P2_1, TIME_P3_1, TIME_P5_1,
        MASS_EXPONENT1,
        MASS_SCALE1,
        ADDITION_BEHAVIOR1
    >;

    fn add(self, other: Quantity<DISTANCE_EXPONENT2, DISTANCE_SCALE2, TIME_EXPONENT2, TIME_P2_2, TIME_P3_2, TIME_P5_2, MASS_EXPONENT2, MASS_SCALE2, ADDITION_BEHAVIOR2>) -> Self::Output {
        let result_scale = result_length_scale(DISTANCE_SCALE1, DISTANCE_SCALE2, ADDITION_BEHAVIOR1);
        
        // Compute conversion factors at compile time
        let factor1 = length_conversion_factor(DISTANCE_SCALE1, result_scale);
        let factor2 = length_conversion_factor(DISTANCE_SCALE2, result_scale);
        
        // One factor will be 1.0 (for the operand already in result units)
        // The other will be the actual conversion factor
        let result_value = self.value * factor1 + other.value * factor2;
        
        Quantity::new(result_value)
    }
}

fn main() {
    // Test length conversion factors
    println!("Millimeter to Meter factor: {}", length_conversion_factor(LengthScale::Millimeter, LengthScale::Meter));
    println!("Meter to Millimeter factor: {}", length_conversion_factor(LengthScale::Meter, LengthScale::Millimeter));
    
    // Test time conversion factors
    println!("Second to Minute factor: {}", time_conversion_factor(0, 0, 0, 2, 1, 1));
    println!("Minute to Second factor: {}", time_conversion_factor(2, 1, 1, 0, 0, 0));
    println!("Minute to Hour factor: {}", time_conversion_factor(2, 1, 1, 4, 2, 2));
    
    // Test systematic addition
    let meters = Quantity::<1, { LengthScale::Meter }, 0, 0, 0, 0, 0, { MassScale::Gram }, { AdditionBehavior::SmallerWins }>::new(5.0);
    let millimeters = Quantity::<1, { LengthScale::Millimeter }, 0, 0, 0, 0, 0, { MassScale::Gram }, { AdditionBehavior::SmallerWins }>::new(300.0);
    
    let result = meters + millimeters;
    println!("5 meters + 300 millimeters = {} (in smaller scale)", result.value);
    
    // Test the other direction
    let result2 = millimeters + meters;
    println!("300 millimeters + 5 meters = {} (in smaller scale)", result2.value);
} 