use whippyunits::define_unit_declarators;
use whippyunits::value;

#[macro_use]
mod utils;

define_unit_declarators!(
    test_scale,
    Kilogram,
    Millimeter,
    Second,
    Ampere,
    Kelvin,
    Mole,
    Candela,
    Radian
);

#[culit::culit(test_scale::literals)]
#[test]
fn test_local_unit_literals() {
    let power_f64 = 1.0mW;
    let power_f32 = 1.0mW_f32;

    let energy_i32 = 1mJ;
    let energy_i64 = 1mJ_i64;

    println!("Power (f64): {:?}", power_f64);
    println!("Power (f32): {:?}", power_f32);
    println!("Energy (i32): {:?}", energy_i32);
    println!("Energy (i64): {:?}", energy_i64);

    assert_approx_eq!(value!(power_f64, uW), 1000.0);
    assert_approx_eq!(value!(power_f32, uW, f32), 1000.0f32);
    assert_eq!(value!(energy_i32, uJ, i32), 1000);
    assert_eq!(value!(energy_i64, uJ, i64), 1000_i64);
}


#[test]
fn test_local_quantity_macro() {
    use test_scale::*;

    let power_f64 = quantity!(1.0, mJ / s);
    let power_f32 = quantity!(1.0, mJ / s, f32);

    let energy_i32 = quantity!(1, mW * s, i32);
    let energy_i64 = quantity!(1, mW * s, i64);

    println!("Power (f64): {:?}", power_f64);
    println!("Power (f32): {:?}", power_f32);
    println!("Energy (i32): {:?}", energy_i32);
    println!("Energy (i64): {:?}", energy_i64);

    assert_approx_eq!(value!(power_f64, uW), 1000.0);
    assert_approx_eq!(value!(power_f32, uW, f32), 1000.0f32);
    assert_eq!(value!(energy_i32, uJ, i32), 1000);
    assert_eq!(value!(energy_i64, uJ, i64), 1000_i64);
}

define_unit_declarators!(
    branded_scale,
    Brand,
    Kilogram,
    Millimeter,
    Second,
    Ampere,
    Kelvin,
    Mole,
    Candela,
    Radian
);

#[culit::culit(branded_scale::literals)]
#[test]
fn test_branded_quantity() {
    use branded_scale::*;
    let power_f64 = 1.0mW;
    let power_f32 = 1.0mW_f32;

    let energy_i32 = 1mJ;
    let energy_i64 = 1mJ_i64;

    println!("Power (f64): {:?}", power_f64);
    println!("Power (f32): {:?}", power_f32);
    println!("Energy (i32): {:?}", energy_i32);
    println!("Energy (i64): {:?}", energy_i64);

    assert_approx_eq!(value!(power_f64, uW, f64, Brand), 1000.0);
    assert_approx_eq!(value!(power_f32, uW, f32, Brand), 1000.0f32);
    assert_eq!(value!(energy_i32, uJ, i32, Brand), 1000);
    assert_eq!(value!(energy_i64, uJ, i64, Brand), 1000_i64);
}

define_unit_declarators!(
    branded_defaults,
    Brand
);

#[culit::culit(branded_defaults::literals)]
#[test]
fn test_branded_defaults() {
    use branded_defaults::*;
    let power_f64 = 1.0mW;
    let power_f32 = 1.0mW_f32;

    let energy_i32 = 1mJ;
    let energy_i64 = 1mJ_i64;

    println!("Power (f64): {:?}", power_f64);
    println!("Power (f32): {:?}", power_f32);
    println!("Energy (i32): {:?}", energy_i32);
    println!("Energy (i64): {:?}", energy_i64);

    // all exactly equal due to lack of implicit rescale
    assert_eq!(value!(power_f64, mW, f64, Brand), 1.0);
    assert_eq!(value!(power_f32, mW, f32, Brand), 1.0f32);
    assert_eq!(value!(energy_i32, mJ, i32, Brand), 1);
    assert_eq!(value!(energy_i64, mJ, i64, Brand), 1_i64);
}