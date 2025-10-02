//! Simple test to verify custom literals work

use whippyunits::default_declarators::*;
use whippyunits::*;

whippyunits::define_literals!();

#[culit::culit]
#[test]
fn test_simple_custom_literals() {
    // Test float literals with float suffixes (these go to float module)
    let distance = 100.0m_f64;
    let mass = 5.5kg_f64;
    let time = 30.0s_f64;

    // Test integer literals with integer suffixes (these go to int module)
    let distance_i32 = 10m;

    // These should now create proper unit types using the quantity! macro
    // We can test that they have the correct values by accessing the .value field
    assert_eq!(distance.value, 100.0);
    assert_eq!(mass.value, 5.5);
    assert_eq!(time.value, 30.0);
    assert_eq!(distance_i32.value, 10);

    // Test that they are actually proper unit types with correct dimensions
    // distance should be length (m), mass should be mass (kg), time should be time (s)
    println!("Distance: {} (should be length)", distance.value);
    println!("Mass: {} (should be mass)", mass.value);
    println!("Time: {} (should be time)", time.value);
    println!("Distance: {} (should be length)", distance_i32.value);

    println!("Mass prettyprint: {}", mass);
    println!("Distance prettyprint: {}", distance);
    println!("Time prettyprint: {}", time);

    println!("Custom literals test passed!");
}

#[culit::culit]
#[test]
fn test_compound_unit_custom_literals() {
    // Test compound unit literals (J, W, N, Pa, Hz, C, V, F, Ω, S, H, T, Wb, lm, lx)
    let energy = 1.5J_f64;        // Joules
    let power = 2.0W_f64;         // Watts
    let force = 10.0N_f64;        // Newtons
    let pressure = 100.0Pa_f64;   // Pascals
    let frequency = 50.0Hz_f64;   // Hertz
    let charge = 1.0C_f64;        // Coulombs
    let voltage = 12.0V_f64;      // Volts
    let capacitance = 0.1F_f64;   // Farads
    let resistance = 100.0Ω_f64; // Ohms
    let conductance = 0.01S_f64;  // Siemens
    let inductance = 0.5H_f64;    // Henrys
    let magnetic_field = 1.0T_f64; // Tesla
    let magnetic_flux = 0.1Wb_f64; // Weber
    let luminous_flux = 100.0lm_f64; // Lumen
    let illuminance = 50.0lx_f64; // Lux

    // Test integer compound unit literals
    let energy_i32 = 5J;
    let power_i32 = 10W;
    let force_i32 = 20N;

    // Test that they have the correct values
    assert_eq!(energy.value, 1.5);
    assert_eq!(power.value, 2.0);
    assert_eq!(force.value, 10.0);
    assert_eq!(pressure.value, 100.0);
    assert_eq!(frequency.value, 50.0);
    assert_eq!(charge.value, 1.0);
    assert_eq!(voltage.value, 12.0);
    assert_eq!(capacitance.value, 0.1);
    assert_eq!(resistance.value, 100.0);
    assert_eq!(conductance.value, 0.01);
    assert_eq!(inductance.value, 0.5);
    assert_eq!(magnetic_field.value, 1.0);
    assert_eq!(magnetic_flux.value, 0.1);
    assert_eq!(luminous_flux.value, 100.0);
    assert_eq!(illuminance.value, 50.0);

    assert_eq!(energy_i32.value, 5);
    assert_eq!(power_i32.value, 10);
    assert_eq!(force_i32.value, 20);

    // Test prefixed compound units
    let kilojoule = 1.5kJ_f64;    // kilojoules
    let milliwatt = 100.0mW_f64;  // milliwatts
    let kilonewton = 5.0kN_f64;   // kilonewtons

    assert_eq!(kilojoule.value, 1.5);
    assert_eq!(milliwatt.value, 100.0);
    assert_eq!(kilonewton.value, 5.0);

    println!("Energy: {} (should be energy)", energy);
    println!("Power: {} (should be power)", power);
    println!("Force: {} (should be force)", force);
    println!("Pressure: {} (should be pressure)", pressure);
    println!("Frequency: {} (should be frequency)", frequency);
    println!("Charge: {} (should be charge)", charge);
    println!("Voltage: {} (should be voltage)", voltage);
    println!("Capacitance: {} (should be capacitance)", capacitance);
    println!("Resistance: {} (should be resistance)", resistance);
    println!("Conductance: {} (should be conductance)", conductance);
    println!("Inductance: {} (should be inductance)", inductance);
    println!("Magnetic field: {} (should be magnetic field)", magnetic_field);
    println!("Magnetic flux: {} (should be magnetic flux)", magnetic_flux);
    println!("Luminous flux: {} (should be luminous flux)", luminous_flux);
    println!("Illuminance: {} (should be illuminance)", illuminance);

    println!("Kilojoule: {} (should be energy)", kilojoule);
    println!("Milliwatt: {} (should be power)", milliwatt);
    println!("Kilonewton: {} (should be force)", kilonewton);

    println!("Compound unit custom literals test passed!");
}

#[culit::culit]
#[test]
fn test_angular_unit_custom_literals() {
    // Test all angular unit literals with float suffixes
    let radian = 1.0rad_f64;        // radian (base SI unit)
    let degree = 90.0deg_f64;       // degree (π/180 rad)
    let rotation = 0.5rot_f64;      // rotation (2π rad)
    let turn = 0.25turn_f64;        // turn (2π rad)
    let arcsecond = 3600.0arcsec_f64; // arcsecond (π/(180*3600) rad)
    let arcminute = 60.0arcmin_f64;   // arcminute (π/(180*60) rad)
    let gon = 100.0gon_f64;         // gon (π/200 rad)
    let gradian = 100.0grad_f64;    // gradian (π/200 rad)

    // Test integer angular unit literals
    let radian_i32 = 1rad;
    let degree_i32 = 90deg;
    let rotation_i32 = 1rot;
    let turn_i32 = 1turn;

    // Test that they have the correct values
    assert_eq!(radian.value, 1.0);
    assert_eq!(degree.value, 90.0);
    assert_eq!(rotation.value, 0.5);
    assert_eq!(turn.value, 0.25);
    assert_eq!(arcsecond.value, 3600.0);
    assert_eq!(arcminute.value, 60.0);
    assert_eq!(gon.value, 100.0);
    assert_eq!(gradian.value, 100.0);

    assert_eq!(radian_i32.value, 1);
    assert_eq!(degree_i32.value, 90);
    assert_eq!(rotation_i32.value, 1);
    assert_eq!(turn_i32.value, 1);

    // Test pretty printing
    println!("Radian: {} (should be angle)", radian);
    println!("Degree: {} (should be angle)", degree);
    println!("Rotation: {} (should be angle)", rotation);
    println!("Turn: {} (should be angle)", turn);
    println!("Arcsecond: {} (should be angle)", arcsecond);
    println!("Arcminute: {} (should be angle)", arcminute);
    println!("Gon: {} (should be angle)", gon);
    println!("Gradian: {} (should be angle)", gradian);

    println!("Radian (i32): {} (should be angle)", radian_i32);
    println!("Degree (i32): {} (should be angle)", degree_i32);
    println!("Rotation (i32): {} (should be angle)", rotation_i32);
    println!("Turn (i32): {} (should be angle)", turn_i32);

    // Test conversions between angular units
    // Note: These conversions require proper scale factor handling
    // For now, just test that the units can be created and have correct values
    println!("90 degrees: {} (should be angle with scale factors)", degree);
    println!("0.5 rotations: {} (should be angle with scale factors)", rotation);
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
