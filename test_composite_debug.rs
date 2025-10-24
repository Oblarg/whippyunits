use whippyunits::define_base_units;

// Set up scoped preferences with different units
define_base_units!(
    Kilogram, Meter, Second, Ampere, Kelvin, Mole, Candela, Radian, test_scale
);

#[test]
fn test_composite_dimensions_debug() {
    use test_scale::*;
    
    // Test pascal (pressure = force/area = mass·length⁻¹·time⁻²)
    let pressure = quantity!(100.0, Pa);
    println!("Pascal: {:?}", pressure);
    
    // Test joule (energy = force·length = mass·length²·time⁻²)
    let energy = quantity!(50.0, J);
    println!("Joule: {:?}", energy);
    
    // Test newton (force = mass·length·time⁻²)
    let force = quantity!(25.0, N);
    println!("Newton: {:?}", force);
    
    // Test watt (power = energy/time = mass·length²·time⁻³)
    let power = quantity!(10.0, W);
    println!("Watt: {:?}", power);
    
    // Test with systematic units to see what should be generated
    let pressure_systematic = quantity!(100.0, kg / (m * s * s));
    println!("Pressure systematic: {:?}", pressure_systematic);
    
    let energy_systematic = quantity!(50.0, kg * m * m / (s * s));
    println!("Energy systematic: {:?}", energy_systematic);
}
