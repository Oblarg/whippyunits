use whippyunits::define_unit_declarators;

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
    let energy_f64 = 1.0J; // 1 joule
    let energy_i32 = 1J; // 1 joule (integer)
    let power_f64 = 2.0W; // 2 watts
    let force_f64 = 10.0N; // 10 newtons
    let pressure_f64 = 100.0Pa; // 100 pascals
    let kilojoule_f64 = 1.5kJ; // 1.5 kilojoules
    let milliwatt_f32 = 100.0mW_f32; // 100 milliwatts

    println!("Energy (f64): {}", energy_f64);
    println!("Energy (i32): {}", energy_i32);
    println!("Power (f64): {}", power_f64);
    println!("Force (f64): {}", force_f64);
    println!("Pressure (f64): {}", pressure_f64);
    println!("Kilojoule (f64): {}", kilojoule_f64);
    println!("Milliwatt (f32): {}", milliwatt_f32);

    println!("Compound unit literals with local scale preferences test passed!");
}

#[test]
fn test_local_quantity_macro() {
    use test_scale::*;

    let energy_f64 = quantity!(100.0, J);
    println!("Energy (f64): {:?}", energy_f64);

    let energy_i32 = quantity!(100, J, i32);
    println!("Energy (i32): {:?}", energy_i32);

    let energy_i64 = quantity!(100, J, i64);
    println!("Energy (i64): value = {}", energy_i64.unsafe_value);

    let force_f64 = quantity!(50.0, N);
    println!("Force (f64): {:?}", force_f64);

    let force_i32 = quantity!(50, N, i32);
    println!("Force (i32): {:?}", force_i32);

    let power_f64 = quantity!(25.0, W);
    println!("Power (f64): {:?}", power_f64);

    let power_i64 = quantity!(25, W, i64);
    println!("Power (i64): value = {}", power_i64.unsafe_value);

    let mass_f64 = 1000.0.grams();
    assert_eq!(mass_f64.unsafe_value, 1.0); // 1000 grams = 1 kilogram

    let mass_i32 = 1000i32.grams();
    assert_eq!(mass_i32.unsafe_value, 1); // 1000 grams = 1 kilogram

    let mass_i64 = 1000i64.grams();
    assert_eq!(mass_i64.unsafe_value, 1); // 1000 grams = 1 kilogram

    println!("Local quantity macro with generic storage types test passed!");
}

#[test]
fn test_compound_unit_literal_detection() {
    use test_scale::*;
    let test = quantity!(1.0, kW * h);
    println!("Test: {:?}", test);
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
    let energy_f64 = 1.0J; // 1 joule
    let energy_i32 = 1J; // 1 joule (integer)
    let power_f64 = 2.0W; // 2 watts
    let force_f64 = 10.0N; // 10 newtons
    let pressure_f64 = 100.0Pa; // 100 pascals
    let kilojoule_f64 = 1.5kJ; // 1.5 kilojoules
    let milliwatt_f32 = 100.0mW_f32; // 100 milliwatts
}

define_unit_declarators!(
    branded_defaults,
    Brand
);

#[culit::culit(branded_defaults::literals)]
#[test]
fn test_branded_defaults() {
    let energy_f64 = 1.0J; // 1 joule
    let energy_i32 = 1J; // 1 joule (integer)
    let power_f64 = 2.0W; // 2 watts
    let force_f64 = 10.0N; // 10 newtons
    let pressure_f64 = 100.0Pa; // 100 pascals
    let kilojoule_f64 = 1.5kJ; // 1.5 kilojoules
    let milliwatt_f32 = 100.0mW_f32; // 100 milliwatts
}