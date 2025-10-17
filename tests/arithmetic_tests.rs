#![feature(impl_trait_in_bindings)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use whippyunits::api::rescale;
use whippyunits::default_declarators::*;
use whippyunits::define_generic_dimension;
use whippyunits::quantity;
use whippyunits::unit;
use whippyunits::value;
use whippyunits_core::Unit;

#[test]
fn test_legal_addition() {
    let result = 5.0.meters() + 3.0.meters();
    assert_eq!(value!(result, m), 8.0);
}

#[test]
fn test_legal_addition_assign() {
    let mut m1 = 5.0.meters();

    m1 += 3.0.meters();
    assert_eq!(value!(m1, m), 8.0);
}

#[test]
fn test_legal_subtraction() {
    let result: unit!(m) = 5.0.meters() - 2.0.meters();
    assert_eq!(value!(result, m), 3.0);

    let result: unit!(s) = 30.0.seconds() - 5.0.seconds();
    assert_eq!(value!(result, s), 25.0);
}

#[test]
fn test_scalar_from_radians() {
    let radians = 5.0.radians();
    let square_radians = radians * radians;
    let cube_radians = square_radians * radians;
    let inverse_radians = 1.0 / radians;
    let inverse_square_radians = 1.0 / square_radians;
    let inverse_cube_radians = 1.0 / cube_radians;

    let scalar: f64 = radians.into();
    assert_eq!(scalar, 5.0);
    let scalar: f64 = square_radians.into();
    assert_eq!(scalar, 25.0);
    let scalar: f64 = cube_radians.into();
    assert_eq!(scalar, 125.0);
    let scalar: f64 = inverse_radians.into();
    assert_eq!(scalar, 0.2);
    let scalar: f64 = inverse_square_radians.into();
    assert_eq!(scalar, 0.04);
    let scalar: f64 = inverse_cube_radians.into();
    assert_eq!(scalar, 0.008);
}

#[test]
fn test_radian_erasure() {
    let composite_with_radians = 5.0.radians() / 3.0.seconds();
    let composite_with_radians_erased: unit!(1 / s) = composite_with_radians.into();
    println!(
        "composite_with_radians_erased: {:?}",
        composite_with_radians_erased
    );
    assert_eq!(value!(composite_with_radians_erased, 1 / s), 5.0 / 3.0);
}

#[test]
fn test_dimensionless_to_scalar_conversion() {
    let dimensionless: unit!(1) = quantity!(42.0, 1);
    let scalar_f64: f64 = dimensionless.into();
    assert_eq!(scalar_f64, 42.0);

    let dimensionless_scaled = 100.0.millimeters() / 1.0.meters();
    let scalar_rescaled: f64 = dimensionless_scaled.into();
    assert_eq!(scalar_rescaled, 0.1);

    let dimensionless_i32 = 5.millimeters() / 1.meters();
    let scalar_i32: i32 = dimensionless_i32.into();
    assert_eq!(scalar_i32, 0); // 0.005 rounds to 0 for i32
}

#[test]
fn test_radian_erasure_with_scale() {
    let radians_zero_scale: unit!(rad) = quantity!(3.14159, rad);
    let scalar_f64: f64 = radians_zero_scale.into();
    assert_eq!(scalar_f64, 3.14159);

    let radians_scaled = 180.0.degrees();
    let scalar_rescaled: f64 = radians_scaled.into();
    assert!((scalar_rescaled - std::f64::consts::PI).abs() < 1e-10);

    // Test with different numeric types - use a simpler case
    let radians_simple = quantity!(1, rad, i32);
    let scalar_i32: i32 = radians_simple.into();
    assert_eq!(scalar_i32, 1);
}

#[test]
fn test_scalar_quantity_multiplication() {
    let result: unit!(m) = 3.0 * 5.0.meters();
    assert_eq!(value!(result, m), 15.0);

    let result: unit!(m) = 5.0.meters() * 4.0;
    assert_eq!(value!(result, m), 20.0);
}

#[test]
fn test_scalar_quantity_division() {
    let result: unit!(m) = 5.0.meters() / 2.0;
    assert_eq!(value!(result, m), 2.5);

    let result: unit!(1 / m) = 10.0 / 5.0.meters();
    assert_eq!(value!(result, 1 / m), 2.0); // 10 / 5m = 2 m^-1
}

#[test]
fn test_scalar_quantity_division_with_scale_exponents() {
    // Test that scale exponents are properly inverted in scalar division
    let inverse_km: unit!(1 / km) = 10.0 / 5.0.kilometers();
    assert_eq!(value!(inverse_km, 1 / km), 2.0); // 10 / 5km = 2 km^-1

    // Test rescaling of inverse quantities to ensure scale exponents covary correctly
    let inverse_m: unit!(1 / m) = rescale(inverse_km);
    assert_eq!(value!(inverse_m, 1 / m), 0.002); // 2 km^-1 = 0.002 m^-1 (since 1/km = 0.001/m)
}

#[test]
fn test_quantity_scalar_multiplication() {
    let result: unit!(m) = 5.0.meters() * 4.0;
    assert_eq!(value!(result, m), 20.0);
}

#[test]
fn test_quantity_scalar_division() {
    let result: unit!(m) = 5.0.meters() / 2.0;
    let result: unit!(m) = 5.0.meters() / 2.0;
    assert_eq!(value!(result, m), 2.5);
}

#[test]
fn test_quantity_scalar_multiplication_assign() {
    let mut m1 = 5.0.meters();

    m1 *= 4.0;
    assert_eq!(value!(m1, m), 20.0);
}

#[test]
fn test_rescale_length() {
    let result: Kilometer = rescale(5.0.meters());
    assert_eq!(value!(result, km), 0.005); // 5m = 0.005km

    let result: Millimeter = rescale(5.0.meters());
    assert_eq!(value!(result, mm), 5000.0); // 5m = 5000mm
}

#[test]
fn test_rescale_mass() {
    let result: Kilogram = rescale(100.0.grams());
    assert_eq!(value!(result, kg), 0.1); // 100g = 0.1kg

    let result: Milligram = rescale(100.0.grams());
    assert_eq!(value!(result, mg), 100000.0); // 100g = 100000mg
}

#[test]
fn test_rescale_time() {
    let result: Minute = rescale(30.0.seconds());
    assert_eq!(value!(result, min), 0.5); // 30s = 0.5min

    let result: Millisecond = rescale(30.0.seconds());
    assert_eq!(value!(result, ms), 30000.0); // 30s = 30000ms
}

#[test]
fn test_rescale_macro() {
    use whippyunits::rescale;

    // Test f64 default behavior
    let result = rescale!(quantity!(1.0, m), mm);
    assert_eq!(value!(result, mm), 1000.0);

    // Test i32 behavior (requires third type parameter)
    let result = rescale!(quantity!(1, m, i32), mm, i32);
    assert_eq!(value!(result, mm, i32), 1000);
}

#[test]
fn test_negative_quantities() {
    let result = -5.0.meters() + 7.0.meters();
    assert_eq!(value!(result, m), 2.0);

    let result = 7.0.meters() - -5.0.meters();
    assert_eq!(value!(result, m), 12.0);

    let result = -5.0.meters() * 2.0;
    assert_eq!(value!(result, m), -10.0);
}

#[test]
fn test_large_numbers() {
    let result = 1000000.0.meters() + 0.000001.meters();
    assert_eq!(value!(result, m), 1000000.000001);

    let result = 1000000.0.meters() * 2.0;
    assert_eq!(value!(result, m), 2000000.0);
}

#[test]
fn test_chain_operations() {
    let result = 5.0.meters() + 5.0.meters() - 2.0.meters() * 3.0 / 2.0;
    assert_eq!(value!(result, m), 7.0);
}

#[test]
fn test_generic_dimension() {
    define_generic_dimension!(Velocity, Length / Time);

    let meter_per_second: impl Velocity = quantity!(1.0, m / s);
    println!("{:?}", meter_per_second);
    let kilometer_per_second: impl Velocity = quantity!(1.0, km / s);
    println!("{:?}", kilometer_per_second);
}

#[test]
fn test_generic_dimension_expressions() {
    define_generic_dimension!(Force, Mass * Length / Time ^ 2);
    define_generic_dimension!(Energy, Mass * Length ^ 2 / Time ^ 2);
    define_generic_dimension!(Pressure, Mass / Length / Time ^ 2);
    define_generic_dimension!(Power, Mass * Length ^ 2 / Time ^ 3);
    define_generic_dimension!(ElectricField, Mass * Length / Time ^ 3 / Current);
    define_generic_dimension!(Capacitance, Time ^ 4 * Current ^ 2 / Mass / Length ^ 2);

    let force: impl Force = quantity!(1.0, N);
    let energy: impl Energy = quantity!(1.0, J);
    let pressure: impl Pressure = quantity!(1.0, Pa);
    let power: impl Power = quantity!(1.0, W);
    let electric_field: impl ElectricField = quantity!(1.0, V / m);
    let capacitance: impl Capacitance = quantity!(1.0, F);
}

#[test]
fn test_dimension_symbols_in_dsl() {
    define_generic_dimension!(SymbolTest, L, M, T, L ^ 2, M * L ^ 2 / T ^ 2);

    let length: impl SymbolTest = quantity!(1.0, m);
    let mass: impl SymbolTest = quantity!(1.0, kg);
    let time: impl SymbolTest = quantity!(1.0, s);
    let area: impl SymbolTest = quantity!(1.0, m ^ 2);
    let energy: impl SymbolTest = quantity!(1.0, J);
}

#[test]
fn test_mixed_dimension_names_and_symbols() {
    define_generic_dimension!(MixedTest, Length, M, Time, L ^ 2, Mass * L ^ 2 / T ^ 2);
    let length: impl MixedTest = quantity!(1.0, m);
    let mass: impl MixedTest = quantity!(1.0, kg);
    let time: impl MixedTest = quantity!(1.0, s);
    let area: impl MixedTest = quantity!(1.0, m ^ 2);
    let energy: impl MixedTest = quantity!(1.0, J);
}

#[test]
fn test_single_imperial_unit_debug() {
    let volume_us_cup = 1.0.cups();
    println!("1 US cup = {:?}", volume_us_cup);
}

#[test]
fn test_imperial_units() {
    let length_inches = 1.0.inches();
    let length_feet = 1.0.feet();
    let length_yards = 1.0.yards();
    let length_miles = 1.0.miles();

    println!("1 inches = {:?}", length_inches);
    println!("1 foot = {:?}", length_feet);
    println!("1 yard = {:?}", length_yards);
    println!("1 mile = {:?}", length_miles);

    assert_eq!(value!(length_inches, cm), Unit::INCH.conversion_factor);
    assert_eq!(value!(length_feet, dm), Unit::FOOT.conversion_factor);
    assert_eq!(value!(length_yards, m), Unit::YARD.conversion_factor);
    assert_eq!(value!(length_miles, km), Unit::MILE.conversion_factor);

    let mass_ounces = 1.0.ounces();
    let mass_pounds = 1.0.pounds();
    let mass_stones = 1.0.stone();
    let mass_tons = 1.0.tons();

    println!("1 ounces = {:?}", mass_ounces);
    println!("1 pound = {:?}", mass_pounds);
    println!("1 stone = {:?}", mass_stones);
    println!("1 ton = {:?}", mass_tons);


    assert_eq!(value!(mass_ounces, dag), Unit::OUNCE.conversion_factor);
    assert_eq!(value!(mass_pounds, kg), Unit::POUND.conversion_factor);
    // Stones are stored as 10 kg (decakilogram), but there's no standard SI symbol for 10 kg
    // So we test by converting to kg and adjusting for the 10x scale factor
    assert_eq!(value!(mass_stones, kg) / 10.0, Unit::STONE.conversion_factor);
    assert_eq!(value!(mass_tons, Mg), Unit::TON.conversion_factor);

    let temp_fahrenheit = 1.0.fahrenheit();
    let temp_rankine = 1.0.rankine();

    println!("1°F = {:?}", temp_fahrenheit);
    println!("1°R = {:?}", temp_rankine);

    // For temperature units with affine offsets, we need to check the stored value directly
    // since value! macro is for unit conversions, not affine transformations
    // The stored value is: (1 * conversion_factor) + affine_offset
    assert_eq!(value!(temp_fahrenheit, K), 1.0 * Unit::FAHRENHEIT.conversion_factor + Unit::FAHRENHEIT.affine_offset);
    assert_eq!(value!(temp_rankine, K), Unit::RANKINE.conversion_factor);

    let volume_us_gallon = 1.0.gallons();
    let volume_uk_gallon = 1.0.uk_gallons();
    let volume_us_quart = 1.0.quarts();
    let volume_uk_quart = 1.0.uk_quarts();
    let volume_us_pint = 1.0.pints();
    let volume_uk_pint = 1.0.uk_pints();
    let volume_us_cup = 1.0.cups();
    let volume_uk_cup = 1.0.uk_cups();
    let volume_us_fluid_ounce = 1.0.fluid_ounces();
    let volume_uk_fluid_ounce = 1.0.uk_fluid_ounces();
    let volume_us_tablespoon = 1.0.tablespoons();
    let volume_uk_tablespoon = 1.0.uk_tablespoons();
    let volume_us_teaspoon = 1.0.teaspoons();
    let volume_uk_teaspoon = 1.0.uk_teaspoons();

    println!("1 US gallon = {:?}", volume_us_gallon);
    println!("1 UK gallon = {:?}", volume_uk_gallon);
    println!("1 US quart = {:?}", volume_us_quart);
    println!("1 UK quart = {:?}", volume_uk_quart);
    println!("1 US pint = {:?}", volume_us_pint);
    println!("1 UK pint = {:?}", volume_uk_pint);
    println!("1 US cup = {:?}", volume_us_cup);
    println!("1 UK cup = {:?}", volume_uk_cup);
    println!("1 US fluid ounce = {:?}", volume_us_fluid_ounce);
    println!("1 UK fluid ounce = {:?}", volume_uk_fluid_ounce);
    println!("1 US tablespoon = {:?}", volume_us_tablespoon);
    println!("1 UK tablespoon = {:?}", volume_uk_tablespoon);
    println!("1 US teaspoon = {:?}", volume_us_teaspoon);
    println!("1 UK teaspoon = {:?}", volume_uk_teaspoon);
}

#[test]
fn test_custom_formatting() {
    let distance = 5000.0.meters();
    let mass = 2.5.kilograms();
    let time = 90.0.seconds();

    println!("  Distance: {}", distance);
    println!("  Mass: {}", mass);
    println!("  Time: {}", time);

    assert_eq!(format!("{}", distance.fmt("km")), "5 km");
    assert_eq!(format!("{}", distance.fmt("cm")), "500000 cm");
    assert_eq!(format!("{}", distance.fmt("mm")), "5000000 mm");
    assert_eq!(format!("{}", distance.fmt("ft")), "16404.199475065616 ft");
    assert_eq!(format!("{}", distance.fmt("mi")), "3.1068559611866697 mi");

    println!("Mass conversions:");
    assert_eq!(format!("{}", mass.fmt("g")), "2500 g");
    assert_eq!(format!("{}", mass.fmt("kg")), "2.5 kg");
    assert_eq!(format!("{}", mass.fmt("oz")), "88.18490487395103 oz");
    assert_eq!(format!("{}", mass.fmt("lb")), "5.511556554621939 lb");

    println!("Time conversions:");
    assert_eq!(format!("{}", time.fmt("s")), "90 s");
    assert_eq!(format!("{}", time.fmt("min")), "1.5 min");
    assert_eq!(format!("{}", time.fmt("h")), "0.025 h");

    println!("Precision formatting:");
    assert_eq!(format!("{:.2}", distance.fmt("km")), "5.00 km");
    assert_eq!(format!("{:.0}", distance.fmt("cm")), "500000 cm");
    assert_eq!(format!("{:.1}", mass.fmt("g")), "2500.0 g");

    println!("Error cases:");
    assert!(distance.fmt("kg").to_string().contains("Error")); // Wrong dimension
    assert!(distance.fmt("unknown_unit").to_string().contains("Error")); // Unknown unit

    assert_eq!(format!("{}", distance.fmt("kilometer")), "5 kilometer");
    assert_eq!(format!("{}", mass.fmt("gram")), "2500 g");

    println!("Custom formatting tests passed!");
}

#[test]
fn test_i32_quantity_declarators() {
    let mass_i32 = 5.grams();
    let length_i32 = 10.meters();

    let area_i32 = length_i32 * length_i32;
    println!("i32 Area: {:?}", area_i32);

    assert_eq!(value!(mass_i32, g, i32), 5i32);
    assert_eq!(value!(length_i32, m, i32), 10i32);
    assert_eq!(value!(area_i32, m^2, i32), 100i32);

    let mass_f64 = 5.0.grams();
    let length_f64 = 10.0.meters();

    assert_eq!(value!(mass_f64, g), 5.0f64);
    assert_eq!(value!(length_f64, m), 10.0f64);
}

#[test]
fn test_unit_macro_with_different_types() {
    let length_f64: unit!(m) = 5.0.meters();
    assert_eq!(value!(length_f64, m), 5.0f64);

    let length_i32: unit!(m, i32) = 5.meters();
    assert_eq!(value!(length_i32, m, i32), 5i32);

    let area_i32: unit!(m ^ 2, i32) = 5.meters() * 10.meters();
    assert_eq!(value!(area_i32, m^2, i32), 50i32);

    let _length_type_check: unit!(m, i32) = 5.meters();
    let _area_type_check: unit!(m ^ 2, i32) = 5.meters() * 10.meters();
}

// #[test]
// fn test_imperial_declarators_generic_storage_types() {
//     let length_f64: default_declarators::Centimeter<f64> = 12.0.inches();
//     assert_eq!(length_f64.unsafe_value, 12.0 * 2.54);

//     let length_i32: default_declarators::Centimeter<i32> = 12i32.inches();
//     assert_eq!(length_i32.unsafe_value, (12.0 * 2.54) as i32);

//     let length_i64: default_declarators::Centimeter<i64> = 12i64.inches();
//     assert_eq!(length_i64.unsafe_value, (12.0 * 2.54) as i64);

//     let mass_f64: default_declarators::Kilogram<f64> = 2.0.pounds();
//     assert_eq!(mass_f64.unsafe_value, 2.0 * 0.45359237);

//     let mass_i32: default_declarators::Kilogram<i32> = 2i32.pounds();
//     assert_eq!(mass_i32.unsafe_value, (2.0 * 0.45359237) as i32);

//     let temp_f64: Fahrenheit<f64> = 32.0.fahrenheit();
//     assert_eq!(temp_f64.unsafe_value, 32.0 * 5.0 / 9.0 + 255.3722222222222);

//     let temp_i32: Fahrenheit<i32> = 32i32.fahrenheit();
//     assert_eq!(
//         temp_i32.unsafe_value,
//         (32.0 * 5.0 / 9.0 + 255.3722222222222) as i32
//     );
// }

#[test]
fn test_all_types_arithmetic_available() {
    let length_f64: unit!(m, f64) = 5.0.meters();
    let area_f64: unit!(m ^ 2, f64) = length_f64 * length_f64;
    assert_eq!(value!(area_f64, m^2), 25.0f64);

    let length_i32: unit!(m, i32) = 5i32.meters();
    let area_i32: unit!(m ^ 2, i32) = length_i32 * length_i32;
    assert_eq!(value!(area_i32, m^2, i32), 25i32);

    let length_f32: unit!(m, f32) = quantity!(5.0, m, f32);
    let area_f32: unit!(m ^ 2, f32) = length_f32 * length_f32;
    assert_eq!(value!(area_f32, m^2, f32), 25.0f32);

    let length_i8: unit!(m, i8) = quantity!(5, m, i8);
    let area_i8: unit!(m ^ 2, i8) = length_i8 * length_i8;
    assert_eq!(value!(area_i8, m^2, i8), 25i8);

    let length_i16: unit!(m, i16) = quantity!(5, m, i16);
    let area_i16: unit!(m ^ 2, i16) = length_i16 * length_i16;
    assert_eq!(value!(area_i16, m^2, i16), 25i16);

    let length_i64: unit!(m, i64) = quantity!(5, m, i64);
    let area_i64: unit!(m ^ 2, i64) = length_i64 * length_i64;
    assert_eq!(value!(area_i64, m^2, i64), 25i64);

    let length_i128: unit!(m, i128) = quantity!(5, m, i128);
    let area_i128: unit!(m ^ 2, i128) = length_i128 * length_i128;
    assert_eq!(value!(area_i128, m^2, i128), 25i128);

    let length_u8: unit!(m, u8) = quantity!(5, m, u8);
    let area_u8: unit!(m ^ 2, u8) = length_u8 * length_u8;
    assert_eq!(value!(area_u8, m^2, u8), 25u8);

    let length_u16: unit!(m, u16) = quantity!(5, m, u16);
    let area_u16: unit!(m ^ 2, u16) = length_u16 * length_u16;
    assert_eq!(value!(area_u16, m^2, u16), 25u16);

    let length_u32: unit!(m, u32) = quantity!(5, m, u32);
    let area_u32: unit!(m ^ 2, u32) = length_u32 * length_u32;
    assert_eq!(value!(area_u32, m^2, u32), 25u32);

    let length_u64: unit!(m, u64) = quantity!(5, m, u64);
    let area_u64: unit!(m ^ 2, u64) = length_u64 * length_u64;
    assert_eq!(value!(area_u64, m^2, u64), 25u64);

    let length_u128: unit!(m, u128) = quantity!(5, m, u128);
    let area_u128: unit!(m ^ 2, u128) = length_u128 * length_u128;
    assert_eq!(value!(area_u128, m^2, u128), 25u128);
}

#[test]
fn test_all_types_with_quantity_macro() {
    let length_f32: unit!(m, f32) = quantity!(5.0, m, f32);
    let length_f64: unit!(m, f64) = quantity!(5.0, m, f64);

    let length_i8: unit!(m, i8) = quantity!(5, m, i8);
    let length_i16: unit!(m, i16) = quantity!(5, m, i16);
    let length_i32: unit!(m, i32) = quantity!(5, m, i32);
    let length_i64: unit!(m, i64) = quantity!(5, m, i64);
    let length_i128: unit!(m, i128) = quantity!(5, m, i128);

    let length_u8: unit!(m, u8) = quantity!(5, m, u8);
    let length_u16: unit!(m, u16) = quantity!(5, m, u16);
    let length_u32: unit!(m, u32) = quantity!(5, m, u32);
    let length_u64: unit!(m, u64) = quantity!(5, m, u64);
    let length_u128: unit!(m, u128) = quantity!(5, m, u128);

    let area_f32: unit!(m ^ 2, f32) = length_f32 * length_f32;
    let area_f64: unit!(m ^ 2, f64) = length_f64 * length_f64;
    let area_i8: unit!(m ^ 2, i8) = length_i8 * length_i8;
    let area_i16: unit!(m ^ 2, i16) = length_i16 * length_i16;
    let area_i32: unit!(m ^ 2, i32) = length_i32 * length_i32;
    let area_i64: unit!(m ^ 2, i64) = length_i64 * length_i64;
    let area_i128: unit!(m ^ 2, i128) = length_i128 * length_i128;
    let area_u8: unit!(m ^ 2, u8) = length_u8 * length_u8;
    let area_u16: unit!(m ^ 2, u16) = length_u16 * length_u16;
    let area_u32: unit!(m ^ 2, u32) = length_u32 * length_u32;
    let area_u64: unit!(m ^ 2, u64) = length_u64 * length_u64;
    let area_u128: unit!(m ^ 2, u128) = length_u128 * length_u128;

    assert_eq!(value!(area_f32, m^2, f32), 25.0f32);
    assert_eq!(value!(area_f64, m^2), 25.0f64);
    assert_eq!(value!(area_i8, m^2, i8), 25i8);
    assert_eq!(value!(area_i16, m^2, i16), 25i16);
    assert_eq!(value!(area_i32, m^2, i32), 25i32);
    assert_eq!(value!(area_i64, m^2, i64), 25i64);
    assert_eq!(value!(area_i128, m^2, i128), 25i128);
    assert_eq!(value!(area_u8, m^2, u8), 25u8);
    assert_eq!(value!(area_u16, m^2, u16), 25u16);
    assert_eq!(value!(area_u32, m^2, u32), 25u32);
    assert_eq!(value!(area_u64, m^2, u64), 25u64);
    assert_eq!(value!(area_u128, m^2, u128), 25u128);

    let mass_f32: unit!(kg, f32) = quantity!(2.0, kg, f32);
    let mass_f64: unit!(kg, f64) = quantity!(2.0, kg, f64);
    let mass_i32: unit!(kg, i32) = quantity!(2, kg, i32);
    let mass_u32: unit!(kg, u32) = quantity!(2, kg, u32);

    let force_f32: unit!(N, f32) = mass_f32 * quantity!(9.81, m / s ^ 2, f32);
    let force_f64: unit!(N, f64) = mass_f64 * quantity!(9.81, m / s ^ 2, f64);
    let force_i32: unit!(N, i32) = mass_i32 * quantity!(9, m / s ^ 2, i32);
    let force_u32: unit!(N, u32) = mass_u32 * quantity!(9, m / s ^ 2, u32);

    assert!((value!(force_f32, N, f32) - 19.62f32).abs() < 0.01f32);
    assert!((value!(force_f64, N) - 19.62f64).abs() < 0.01f64);
    assert_eq!(value!(force_i32, N, i32), 18i32); // 2 * 9 = 18
    assert_eq!(value!(force_u32, N, u32), 18u32); // 2 * 9 = 18
}

#[test]
fn test_prefixed_aggregate_quantities() {
    let energy_kj: unit!(kJ) = quantity!(1.0, kJ);
    let energy_mj: unit!(MJ) = quantity!(1.0, MJ);
    let energy_gj: unit!(GJ) = quantity!(1.0, GJ);

    let power_kw: unit!(kW) = quantity!(1.0, kW);
    let power_mw: unit!(MW) = quantity!(1.0, MW);
    let power_gw: unit!(GW) = quantity!(1.0, GW);

    let force_kn: unit!(kN) = quantity!(1.0, kN);
    let force_mn: unit!(MN) = quantity!(1.0, MN);

    let pressure_kpa: unit!(kPa) = quantity!(1.0, kPa);
    let pressure_mpa: unit!(MPa) = quantity!(1.0, MPa);

    let time_seconds = quantity!(1.0, s);
    let energy_from_power: unit!(kJ) = power_kw * time_seconds; // kW * s = kJ

    let mass_kg = quantity!(1.0, kg);
    let acceleration = quantity!(9.81, m / s ^ 2);
    let force_from_mass: unit!(N) = mass_kg * acceleration;

    let area_m2 = quantity!(1.0, m ^ 2);
    let pressure_from_force: unit!(kPa) = force_kn / area_m2; // kN / m² = kPa

    let energy_j: unit!(J) = quantity!(1000.0, J);
    let energy_kj_2: unit!(kJ) = quantity!(1.0, kJ);

    assert_eq!(value!(energy_j, J), 1000.0);
    assert_eq!(value!(energy_kj_2, kJ), 1.0);

    let power_w: unit!(W) = quantity!(1000.0, W);
    let power_kw_2: unit!(kW) = quantity!(1.0, kW);

    assert_eq!(value!(power_w, W), 1000.0);
    assert_eq!(value!(power_kw_2, kW), 1.0);

    let kilowatt_hour: unit!(kW * h) = quantity!(1.0, kW * h);
    assert_eq!(value!(kilowatt_hour, kW * h), 1.0);
}
