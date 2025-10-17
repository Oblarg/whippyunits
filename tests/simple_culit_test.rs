//! Simple test to verify custom literals work

use whippyunits::default_declarators::*;
use whippyunits::*;

whippyunits::define_literals!();

#[culit::culit]
#[test]
fn test_simple_custom_literals() {
    // Test float literals with float suffixes (these go to float module)
    let distance = 100.0m;
    let mass = 5.5kg;
    let time = 30.0s;

    println!("Time: {}", time);

    // Test integer literals with integer suffixes (these go to int module)
    let distance_i32 = 10m;

    // These should now create proper unit types using the quantity! macro
    // We can test that they have the correct values by accessing the .unsafe_value field
    assert_eq!(distance.unsafe_value, 100.0);
    assert_eq!(mass.unsafe_value, 5.5);
    assert_eq!(time.unsafe_value, 30.0);
    assert_eq!(distance_i32.unsafe_value, 10);

    // Test that they are actually proper unit types with correct dimensions
    // distance should be length (m), mass should be mass (kg), time should be time (s)
    println!("Distance: {} (should be length)", distance.unsafe_value);
    println!("Mass: {} (should be mass)", mass.unsafe_value);
    println!("Time: {} (should be time)", time.unsafe_value);
    println!("Distance: {} (should be length)", distance_i32.unsafe_value);

    println!("Mass prettyprint: {}", mass);
    println!("Distance prettyprint: {}", distance);
    println!("Time prettyprint: {}", time);

    println!("Custom literals test passed!");
}

#[culit::culit]
#[test]
fn test_compound_unit_custom_literals() {
    // Test compound unit literals (J, W, N, Pa, Hz, C, V, F, Ω, S, H, T, Wb, lm, lx)
    let energy = 1.5J; // Joules
    let power = 2.0W; // Watts
    let force = 10.0N; // Newtons
    let pressure = 100.0Pa; // Pascals
    let frequency = 50.0Hz; // Hertz
    let charge = 1.0C; // Coulombs
    let voltage = 12.0V; // Volts
    let capacitance = 0.1F; // Farads
    let resistance = 100.0Ω; // Ohms
    let conductance = 0.01S; // Siemens
    let inductance = 0.5H; // Henrys
    let magnetic_field = 1.0T; // Tesla
    let magnetic_flux = 0.1Wb; // Weber
    // let luminous_flux = 100.0lm; // Lumen - removed, use cd instead
    let illuminance = 50.0lx; // Lux

    // Test integer compound unit literals
    let energy_i32 = 5J;
    let power_i32 = 10W;
    let force_i32 = 20N;

    // Test that they have the correct values
    assert_eq!(energy.unsafe_value, 1.5);
    assert_eq!(power.unsafe_value, 2.0);
    assert_eq!(force.unsafe_value, 10.0);
    assert_eq!(pressure.unsafe_value, 100.0);
    assert_eq!(frequency.unsafe_value, 50.0);
    assert_eq!(charge.unsafe_value, 1.0);
    assert_eq!(voltage.unsafe_value, 12.0);
    assert_eq!(capacitance.unsafe_value, 0.1);
    assert_eq!(resistance.unsafe_value, 100.0);
    assert_eq!(conductance.unsafe_value, 0.01);
    assert_eq!(inductance.unsafe_value, 0.5);
    assert_eq!(magnetic_field.unsafe_value, 1.0);
    assert_eq!(magnetic_flux.unsafe_value, 0.1);
    assert_eq!(illuminance.unsafe_value, 50.0);

    assert_eq!(energy_i32.unsafe_value, 5);
    assert_eq!(power_i32.unsafe_value, 10);
    assert_eq!(force_i32.unsafe_value, 20);

    // Test prefixed compound units
    let kilojoule = 1.5kJ; // kilojoules
    let milliwatt = 100.0mW; // milliwatts
    let kilonewton = 5.0kN; // kilonewtons

    assert_eq!(kilojoule.unsafe_value, 1.5);
    assert_eq!(milliwatt.unsafe_value, 100.0);
    assert_eq!(kilonewton.unsafe_value, 5.0);

    println!("Energy: {:?} (should be energy)", energy);
    println!("Power: {:?} (should be power)", power);
    println!("Force: {:?} (should be force)", force);
    println!("Pressure: {:?} (should be pressure)", pressure);
    println!("Frequency: {:?} (should be frequency)", frequency);
    println!("Charge: {:?} (should be charge)", charge);
    println!("Voltage: {:?} (should be voltage)", voltage);
    println!("Capacitance: {:?} (should be capacitance)", capacitance);
    println!("Resistance: {:?} (should be resistance)", resistance);
    println!("Conductance: {:?} (should be conductance)", conductance);
    println!("Inductance: {:?} (should be inductance)", inductance);
    println!(
        "Magnetic field: {:?} (should be magnetic field)",
        magnetic_field
    );
    println!(
        "Magnetic flux: {:?} (should be magnetic flux)",
        magnetic_flux
    );
    println!("Illuminance: {:?} (should be illuminance)", illuminance);

    println!("Kilojoule: {:?} (should be energy)", kilojoule);
    println!("Milliwatt: {:?} (should be power)", milliwatt);
    println!("Kilonewton: {:?} (should be force)", kilonewton);

    println!("Compound unit custom literals test passed!");
}

#[culit::culit]
#[test]
fn test_angular_unit_custom_literals() {
    // Test all angular unit literals with float suffixes
    let radian = 1.0rad; // radian (base SI unit)
    let degree = 90.0deg; // degree (π/180 rad)
    let rotation = 0.5rot; // rotation (2π rad)
    let turn = 0.25turn; // turn (2π rad)
    let arcsecond = 3600.0arcsec; // arcsecond (π/(180*3600) rad)
    let arcminute = 60.0arcmin; // arcminute (π/(180*60) rad)
    // let gon = 100.0gon; // gon (π/200 rad) - temporarily disabled
    let gradian = 100.0grad; // gradian (π/200 rad)

    // Test integer angular unit literals
    let radian_i32 = 1rad;
    let degree_i32 = 90deg;
    let rotation_i32 = 1rot;
    let turn_i32 = 1turn;

    // Test that they have the correct values
    assert_eq!(radian.unsafe_value, 1.0);
    assert_eq!(degree.unsafe_value, 90.0);
    assert_eq!(rotation.unsafe_value, 0.5);
    assert_eq!(turn.unsafe_value, 0.25);
    assert_eq!(arcsecond.unsafe_value, 3600.0);
    assert_eq!(arcminute.unsafe_value, 60.0);
    // assert_eq!(gon.unsafe_value, 100.0); // temporarily disabled
    assert_eq!(gradian.unsafe_value, 100.0);

    assert_eq!(radian_i32.unsafe_value, 1);
    assert_eq!(degree_i32.unsafe_value, 90);
    assert_eq!(rotation_i32.unsafe_value, 1);
    assert_eq!(turn_i32.unsafe_value, 1);

    // Test pretty printing
    println!("Radian: {} (should be angle)", radian);
    println!("Degree: {} (should be angle)", degree);
    println!("Rotation: {} (should be angle)", rotation);
    println!("Turn: {} (should be angle)", turn);
    println!("Arcsecond: {} (should be angle)", arcsecond);
    println!("Arcminute: {} (should be angle)", arcminute);
    // println!("Gon: {} (should be angle)", gon); // temporarily disabled
    println!("Gradian: {} (should be angle)", gradian);

    println!("Radian (i32): {} (should be angle)", radian_i32);
    println!("Degree (i32): {} (should be angle)", degree_i32);
    println!("Rotation (i32): {} (should be angle)", rotation_i32);
    println!("Turn (i32): {} (should be angle)", turn_i32);

    // Test conversions between angular units
    // Note: These conversions require proper scale factor handling
    // For now, just test that the units can be created and have correct values
    println!(
        "90 degrees: {} (should be angle with scale factors)",
        degree
    );
    println!(
        "0.5 rotations: {} (should be angle with scale factors)",
        rotation
    );
    println!("0.25 turns: {} (should be angle with scale factors)", turn);

    println!("Angular unit custom literals test passed!");
}

#[culit::culit]
#[test]
fn test_inline_trig() {
    // Test cross-type conversion directly in the sin function call
    let result = f64::sin(90deg.into());
    println!("sin(90deg) = {}", result);
    // sin(90°) should be 1.0
    assert!((result - 1.0).abs() < 1e-10);
}
