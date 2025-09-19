#![feature(impl_trait_in_bindings)]

use whippyunits::api::rescale_f64;
use whippyunits::unit;
use whippyunits::quantity;
use whippyunits::default_declarators::*;
use whippyunits::define_generic_dimension;

#[test]
fn test_addition_same_scale() {
    let m1 = 5.0.meters();
    
    // Same scale addition should work
    let result = m1 + 3.0.meters();
    assert_eq!(result.value, 8.0);

    println!("result: {:?}", 1.millimeters());
    println!("result: {:?}", 1.millimeters() * 1.seconds());
    println!("result: {:?}", 1.kilograms());
    println!("result: {:?}", 1.milligrams());
    println!("result: {:?}", 1.grams() * 1.seconds());
    println!("result: {:?}", 1.millimeters() * 1.millimeters());
    println!("result: {:?}", 1.millimeters() * 1.meters());
}

#[test]
fn test_add_assign() {
    let mut m1 = 5.0.meters();
    
    // Same scale addition should work
    m1 += 3.0.meters();
    assert_eq!(m1.value, 8.0);
}

#[test]
fn test_subtraction_same_scale() {
    let m1 = 5.0.meters();
    let s1 = 30.0.seconds();
    
    // Same scale subtraction should work
    let result: unit!(m) = m1 - 2.0.meters();
    assert_eq!(result.value, 3.0);
    
    let result: unit!(s) = s1 - 5.0.seconds();
    assert_eq!(result.value, 25.0);
}

// ============================================================================
// Multiplication and Division Tests
// ============================================================================

#[test]
fn test_multiplication_same_scale() {
    let m1 = 5.0.meters();
    let s1 = 30.0.seconds();
    
    // Same scale multiplication should work
    let result: unit!(m) = m1 * 2.0;
    assert_eq!(result.value, 10.0);
    
    let result: unit!(s) = s1 * 3.0;
    assert_eq!(result.value, 90.0);
}

#[test]
fn test_division_same_scale() {
    let m1 = 5.0.meters();
    let s1 = 30.0.seconds();
    
    // Same scale division should work
    let result: unit!(m) = m1 / 2.0;
    assert_eq!(result.value, 2.5);
    
    let result: unit!(s) = s1 / 3.0;
    assert_eq!(result.value, 10.0);
}

#[test]
fn test_quantity_multiplication() {
    let m1 = 5.0.amperes();
    let s1 = 30.0.seconds();
    
    // Multiplying quantities should combine dimensions
    let result= m1 * s1;
    println!("result: {:?}", result);
    // Result should be length * time = distance * time
    assert_eq!(result.value, 150.0); // 5m * 30s = 150 m·s
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
    // let composite_with_radians = 5.0.radians() / 3.0.seconds();
    // let composite_with_radians_erased: unit!(1 / s) = composite_with_radians.into();
    // println!("composite_with_radians_erased: {:?}", composite_with_radians_erased);
    // assert_eq!(composite_with_radians_erased.value, 5.0 / 3.0);
}

#[test]
fn test_quantity_division() {
    let m1 = 5.0.meters();
    let s1 = 30.0.seconds();
    
    // Dividing quantities should combine dimensions
    let result = m1 / s1;
    println!("result: {:?}", result);
    // Result should be length / time = velocity
    assert_eq!(result.value, 5.0 / 30.0); // 5m / 30s = 0.166... m/s
}

#[test]
fn test_scalar_quantity_multiplication() {
    let m1 = 5.0.meters();
    
    // Scalar * Quantity should work
    let result: unit!(m) = 3.0 * m1;
    assert_eq!(result.value, 15.0);
    
    // Quantity * Scalar should work
    let result: unit!(m) = m1 * 4.0; 
    assert_eq!(result.value, 20.0);
}

#[test]
fn test_scalar_quantity_division() {
    let m1 = 5.0.meters();
    
    // Quantity / Scalar should work
    let result: unit!(m) = m1 / 2.0;
    assert_eq!(result.value, 2.5);
    
    // Scalar / Quantity should work (inverts dimensions)
    let result: unit!(1 / m) = 10.0 / m1;
    assert_eq!(result.value, 2.0); // 10 / 5m = 2 m^-1
}

#[test]
fn test_quantity_scalar_multiplication() {
    let m1 = 5.0.meters();
    
    // Quantity * Scalar should work
    let result: unit!(m) = m1 * 4.0;
    assert_eq!(result.value, 20.0);
}

#[test]
fn test_quantity_scalar_division() {
    let m1 = 5.0.meters();
    
    // Quantity / Scalar should work
    let result: unit!(m) = m1 / 2.0;
    assert_eq!(result.value, 2.5);
}

#[test]
fn test_quantity_scalar_multiplication_assign() {
    let mut m1 = 5.0.meters();
    
    // Quantity * Scalar should work
    m1 *= 4.0;
    assert_eq!(m1.value, 20.0);
}


// ============================================================================
// Rescale Tests
// ============================================================================

#[test]
fn test_rescale_length() {
    let m1: unit!(m) = 5.0.meters();
    
    // Rescale from meters to kilometers
    let result: Kilometer = rescale_f64(m1);
    assert_eq!(result.value, 0.005); // 5m = 0.005km
    
    // Rescale from meters to millimeters
    let result: Millimeter = rescale_f64(m1);
    assert_eq!(result.value, 5000.0); // 5m = 5000mm
}

#[test]
fn test_rescale_mass() {    
    // Rescale from grams to kilograms
    let result: Kilogram = rescale_f64(100.0.grams());
    assert_eq!(result.value, 0.1); // 100g = 0.1kg
    
    // Rescale from grams to milligrams
    let result: Milligram = rescale_f64(100.0.grams());
    assert_eq!(result.value, 100000.0); // 100g = 100000mg

    println!("{:?}", 1.kilograms() * 1.meters() * 1.meters() / 1.seconds() / 1.seconds());
}

#[test]
fn test_rescale_time() {
    let s1 = 30.0.seconds();
    
    // Rescale from seconds to minutes
    let result: Minute = rescale_f64(s1);
    assert_eq!(result.value, 0.5); // 30s = 0.5min
    
    // Rescale from seconds to milliseconds
    let result: Millisecond = rescale_f64(s1);
    assert_eq!(result.value, 30000.0); // 30s = 30000ms
}

// ============================================================================
// Edge Cases and Error Handling Tests
// ============================================================================

#[test]
fn test_negative_quantities() {
    let neg_m = (-3.0).meters();
    let pos_m = 5.0.meters();
    
    // Addition with negative
    let result = neg_m + pos_m;
    assert_eq!(result.value, 2.0);
    
    // Subtraction with negative
    let result = pos_m - neg_m;
    assert_eq!(result.value, 8.0);
    
    // Multiplication with negative
    let result = neg_m * 2.0;
    assert_eq!(result.value, -6.0);
}

#[test]
fn test_large_numbers() {
    let large_m = 1000000.0.meters();
    let small_m = 0.000001.meters();
    
    // Addition with large numbers
    let result = large_m + small_m;
    assert_eq!(result.value, 1000000.000001);
    
    // Multiplication with large numbers
    let result = large_m * 2.0;
    assert_eq!(result.value, 2000000.0);
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_chain_operations() {
    let m1 = 5.0.meters();
    
    // Chain multiple operations
    let result = m1 + m1 - 2.0.meters() * 3.0 / 2.0;
    // 5m + 5m - (2m * 3) / 2 = 10m - 3m = 7m
    assert_eq!(result.value, 7.0);
}

// ============================================================================
// SI Prefix Tests
// ============================================================================

#[test]
fn test_si_prefixes_mass() {
    // Test SI prefixes for mass using existing declarators
    let _mg: unit!(mg) = 1.0.milligrams(); // milligram
    let _g: unit!(g) = 1.0.grams();   // gram
    let _kg: unit!(kg) = 1.0.kilograms(); // kilogram
}

#[test]
fn test_si_prefixes_length() {
    // Test all SI prefixes for length (meter base unit)
    let _pm: unit!(pm) = 1.0.picometers(); // picometer
    let _nm: unit!(nm) = 1.0.nanometers(); // nanometer
    let _um: unit!(um) = 1.0.micrometers(); // micrometer (using 'u' as ASCII substitute for μ)
    let _mm: unit!(mm) = 1.0.millimeters(); // millimeter
    let _m: unit!(m) = 1.0.meters();   // meter
    let _km: unit!(km) = 1.0.kilometers(); // kilometer
    let _Mm: unit!(Mm) = 1.0.megameters(); // megameter
    let _Gm: unit!(Gm) = 1.0.gigameters(); // gigameter
    let _Tm: unit!(Tm) = 1.0.terameters(); // terameter
    let _Pm: unit!(Pm) = 1.0.petameters(); // petameter
    let _Em: unit!(Em) = 1.0.exameters(); // exameter
    let _Zm: unit!(Zm) = 1.0.zettameters(); // zettameter
    let _Ym: unit!(Ym) = 1.0.yottameters(); // yottameter
}

#[test]
fn test_si_prefixes_time() {
    // Test all SI prefixes for time (second base unit)
    let _ns: unit!(ns) = 1.0.nanoseconds(); // nanosecond
    let _us: unit!(us) = 1.0.microseconds(); // microsecond (using 'u' as ASCII substitute for μ)
    let _ms: unit!(ms) = 1.0.milliseconds(); // millisecond
    let _s: unit!(s) = 1.0.seconds();   // second
    let _ks: unit!(ks) = 1.0.kiloseconds(); // kilosecond
    let _Ms: unit!(Ms) = 1.0.megaseconds(); // megasecond
    let _Gs: unit!(Gs) = 1.0.gigaseconds(); // gigasecond
    let _Ts: unit!(Ts) = 1.0.teraseconds(); // terasecond
    let _Ps: unit!(Ps) = 1.0.petaseconds(); // petasecond
    let _Es: unit!(Es) = 1.0.exaseconds(); // exasecond
    let _Zs: unit!(Zs) = 1.0.zettaseconds(); // zettasecond
    let _Ys: unit!(Ys) = 1.0.yottaseconds(); // yottasecond
}

#[test]
fn test_si_prefixes_current() {
    // Test all SI prefixes for current (ampere base unit)
    let _pA: unit!(pA) = 1.0.picoamperes(); // picoampere
    let _nA: unit!(nA) = 1.0.nanoamperes(); // nanoampere
    let _uA: unit!(uA) = 1.0.microamperes(); // microampere (using 'u' as ASCII substitute for μ)
    let _mA: unit!(mA) = 1.0.milliamperes(); // milliampere
    let _A: unit!(A) = 1.0.amperes();   // ampere
    let _kA: unit!(kA) = 1.0.kiloamperes(); // kiloampere
    let _MA: unit!(MA) = 1.0.megaamperes(); // megaampere
    let _GA: unit!(GA) = 1.0.gigaamperes(); // gigaampere
    let _TA: unit!(TA) = 1.0.teraamperes(); // teraampere
    let _PA: unit!(PA) = 1.0.petaamperes(); // petaampere
    let _EA: unit!(EA) = 1.0.exaamperes(); // exaampere
    let _ZA: unit!(ZA) = 1.0.zettaamperes(); // zettaampere
    let _YA: unit!(YA) = 1.0.yottaamperes(); // yottaampere
}

#[test]
fn test_si_prefixes_temperature() {
    // Test all SI prefixes for temperature (kelvin base unit)
    let _pK: unit!(pK) = 1.0.picokelvins(); // picokelvin
    let _nK: unit!(nK) = 1.0.nanokelvins(); // nanokelvin
    let _uK: unit!(uK) = 1.0.microkelvins(); // microkelvin (using 'u' as ASCII substitute for μ)
    let _mK: unit!(mK) = 1.0.millikelvins(); // millikelvin
    let _K: unit!(K) = 1.0.kelvins();   // kelvin
    let _kK: unit!(kK) = 1.0.kilokelvins(); // kilokelvin
    let _MK: unit!(MK) = 1.0.megakelvins(); // megakelvin
    let _GK: unit!(GK) = 1.0.gigakelvins(); // gigakelvin
    let _TK: unit!(TK) = 1.0.terakelvins(); // terakelvin
    let _PK: unit!(PK) = 1.0.petakelvins(); // petakelvin
    let _EK: unit!(EK) = 1.0.exakelvins(); // exakelvin
    let _ZK: unit!(ZK) = 1.0.zettakelvins(); // zettakelvin
    let _YK: unit!(YK) = 1.0.yottakelvins(); // yottakelvin
}

#[test]
fn test_si_prefixes_amount() {
    // Test all SI prefixes for amount (mole base unit)
    let _pmol: unit!(pmol) = 1.0.picomoles(); // picomole
    let _nmol: unit!(nmol) = 1.0.nanomoles(); // nanomole
    let _umol: unit!(umol) = 1.0.micromoles(); // micromole (using 'u' as ASCII substitute for μ)
    let _mmol: unit!(mmol) = 1.0.millimoles(); // millimole
    let _mol: unit!(mol) = 1.0.moles();   // mole
    let _kmol: unit!(kmol) = 1.0.kilomoles(); // kilomole
    let _Mmol: unit!(Mmol) = 1.0.megamoles(); // megamole
    let _Gmol: unit!(Gmol) = 1.0.gigamoles(); // gigamole
    let _Tmol: unit!(Tmol) = 1.0.teramoles(); // teramole
    let _Pmol: unit!(Pmol) = 1.0.petamoles(); // petamole
    let _Emol: unit!(Emol) = 1.0.examoles(); // examole
    let _Zmol: unit!(Zmol) = 1.0.zettamoles(); // zettamole
    let _Ymol: unit!(Ymol) = 1.0.yottamoles(); // yottamole
}

#[test]
fn test_si_prefixes_luminosity() {
    // Test all SI prefixes for luminosity (candela base unit)
    let _pcd: unit!(pcd) = 1.0.picocandelas(); // picocandela
    let _ncd: unit!(ncd) = 1.0.nanocandelas(); // nanocandela
    let _ucd: unit!(ucd) = 1.0.microcandelas(); // microcandela (using 'u' as ASCII substitute for μ)
    let _mcd: unit!(mcd) = 1.0.millicandelas(); // millicandela
    let _cd: unit!(cd) = 1.0.candelas();   // candela
    let _kcd: unit!(kcd) = 1.0.kilocandelas(); // kilocandela
    let _Mcd: unit!(Mcd) = 1.0.megacandelas(); // megacandela
    let _Gcd: unit!(Gcd) = 1.0.gigacandelas(); // gigacandela
    let _Tcd: unit!(Tcd) = 1.0.teracandelas(); // teracandela
    let _Pcd: unit!(Pcd) = 1.0.petacandelas(); // petacandela
    let _Ecd: unit!(Ecd) = 1.0.exacandelas(); // exacandela
    let _Zcd: unit!(Zcd) = 1.0.zettacandelas(); // zettacandela
    let _Ycd: unit!(Ycd) = 1.0.yottacandelas(); // yottacandela
}

#[test]
fn test_si_prefixes_angle() {
    // Test all SI prefixes for angle (radian base unit)
    let _prad: unit!(prad) = 1.0.picoradians(); // picoradian
    let _nrad: unit!(nrad) = 1.0.nanoradians(); // nanoradian
    let _urad: unit!(urad) = 1.0.microradians(); // microradian (using 'u' as ASCII substitute for μ)
    let _mrad: unit!(mrad) = 1.0.milliradians(); // milliradian
    let _rad: unit!(rad) = 1.0.radians();   // radian
    let _krad: unit!(krad) = 1.0.kiloradians(); // kiloradian
    let _Mrad: unit!(Mrad) = 1.0.megaradians(); // megaradian
    let _Grad: unit!(Grad) = 1.0.gigaradians(); // gigaradian
    let _Trad: unit!(Trad) = 1.0.teraradians(); // teraradian
    let _Prad: unit!(Prad) = 1.0.petaradians(); // petaradian
    let _Erad: unit!(Erad) = 1.0.exaradians(); // exaradian
    let _Zrad: unit!(Zrad) = 1.0.zettaradians(); // zettaradian
    let _Yrad: unit!(Yrad) = 1.0.yottaradians(); // yottaradian
}

#[test]
fn test_si_prefix_compound_units() {
    // Test SI prefixes in compound units
    let _kg_m_s2: unit!(kg * m / s^2) = 1.0.kilograms() * 1.0.meters() / (1.0.seconds() * 1.0.seconds());
    let _mg_mm_ms2: unit!(mg * mm / ms^2) = 1.0.milligrams() * 1.0.millimeters() / (1.0.milliseconds() * 1.0.milliseconds());
    let _Gg_Gm_Gs2: unit!(Gg * Gm / Gs^2) = 1.0.gigagrams() * 1.0.gigameters() / (1.0.gigaseconds() * 1.0.gigaseconds());
    
    // These should all be the same type (force)
    let force1: unit!(kg * m / s^2) = 1.0.kilograms() * 1.0.meters() / (1.0.seconds() * 1.0.seconds());
    let force2: unit!(mg * mm / ms^2) = 1.0.milligrams() * 1.0.millimeters() / (1.0.milliseconds() * 1.0.milliseconds());

    let area = quantity!(1.0, m^2);
    println!("{:?}", area);

    let frequency = quantity!(1.0, 1 / s);
    println!("{:?}", frequency);
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
fn test_expanded_dimension_dsl_basic() {
    // Test basic SI dimensions
    define_generic_dimension!(BasicDimensions, Mass, Length, Time, Current, Temperature, Amount, Luminosity, Angle);
    
    let mass: impl BasicDimensions = quantity!(1.0, kg);
    let length: impl BasicDimensions = quantity!(1.0, m);
    let time: impl BasicDimensions = quantity!(1.0, s);
    let current: impl BasicDimensions = quantity!(1.0, A);
    let temperature: impl BasicDimensions = quantity!(1.0, K);
    let amount: impl BasicDimensions = quantity!(1.0, mol);
    let luminosity: impl BasicDimensions = quantity!(1.0, cd);
    let angle: impl BasicDimensions = quantity!(1.0, rad);
    
    println!("Basic dimensions test passed");
}

#[test]
fn test_expanded_dimension_dsl_geometric() {
    // Test geometric dimensions
    define_generic_dimension!(GeometricDimensions, Area, Volume, Wavenumber);
    
    let area: impl GeometricDimensions = quantity!(1.0, m^2);
    let volume: impl GeometricDimensions = quantity!(1.0, m^3);
    let wavenumber: impl GeometricDimensions = quantity!(1.0, 1 / m);
    
    println!("Geometric dimensions test passed");
}

#[test]
fn test_expanded_dimension_dsl_kinematic() {
    // Test kinematic dimensions
    define_generic_dimension!(KinematicDimensions, Frequency, Velocity, Acceleration, Jerk);
    
    let frequency: impl KinematicDimensions = quantity!(1.0, 1 / s);
    let velocity: impl KinematicDimensions = quantity!(1.0, m / s);
    let acceleration: impl KinematicDimensions = quantity!(1.0, m / s^2);
    let jerk: impl KinematicDimensions = quantity!(1.0, m / s^3);
    
    println!("Kinematic dimensions test passed");
}

#[test]
fn test_expanded_dimension_dsl_dynamic() {
    // Test dynamic dimensions
    define_generic_dimension!(DynamicDimensions, Momentum, Force, Energy, Power, Action, Pressure);
    
    let momentum: impl DynamicDimensions = quantity!(1.0, kg * m / s);
    let force: impl DynamicDimensions = quantity!(1.0, kg * m / s^2);
    let energy: impl DynamicDimensions = quantity!(1.0, kg * m^2 / s^2);
    let power: impl DynamicDimensions = quantity!(1.0, kg * m^2 / s^3);
    let action: impl DynamicDimensions = quantity!(1.0, kg * m^2 / s);
    let pressure: impl DynamicDimensions = quantity!(1.0, kg / (m * s^2));
    
    println!("Dynamic dimensions test passed");
}

#[test]
fn test_expanded_dimension_dsl_density() {
    // Test density dimensions
    define_generic_dimension!(DensityDimensions, Linear_mass_density, Surface_mass_density, Density);
    
    let linear_density: impl DensityDimensions = quantity!(1.0, kg / m);
    let surface_density: impl DensityDimensions = quantity!(1.0, kg / m^2);
    let volume_density: impl DensityDimensions = quantity!(1.0, kg / m^3);
    
    println!("Density dimensions test passed");
}

#[test]
fn test_expanded_dimension_dsl_electrical() {
    // Test electrical dimensions
    define_generic_dimension!(ElectricalDimensions, Charge, Potential, Resistance, Conductance, Capacitance, Inductance);
    
    let charge: impl ElectricalDimensions = quantity!(1.0, C);
    let potential: impl ElectricalDimensions = quantity!(1.0, V);
    let resistance: impl ElectricalDimensions = quantity!(1.0, Ω);
    let conductance: impl ElectricalDimensions = quantity!(1.0, S);
    let capacitance: impl ElectricalDimensions = quantity!(1.0, F);
    let inductance: impl ElectricalDimensions = quantity!(1.0, H);
    
    println!("Electrical dimensions test passed");
}

#[test]
fn test_expanded_dimension_dsl_thermodynamic() {
    // Test thermodynamic dimensions
    define_generic_dimension!(ThermodynamicDimensions, Entropy, Specific_heat_capacity, Molar_heat_capacity, Thermal_conductivity);
    
    let entropy: impl ThermodynamicDimensions = quantity!(1.0, J / K);
    let specific_heat: impl ThermodynamicDimensions = quantity!(1.0, J / (kg * K));
    let molar_heat: impl ThermodynamicDimensions = quantity!(1.0, J / (mol * K));
    let thermal_conductivity: impl ThermodynamicDimensions = quantity!(1.0, W / (m * K));
    
    println!("Thermodynamic dimensions test passed");
}

#[test]
fn test_expanded_dimension_dsl_chemical() {
    // Test chemical dimensions
    define_generic_dimension!(ChemicalDimensions, Molar_mass, Molar_volume, Molar_concentration, Molar_energy);
    
    let molar_mass: impl ChemicalDimensions = quantity!(1.0, kg / mol);
    let molar_volume: impl ChemicalDimensions = quantity!(1.0, m^3 / mol);
    let molar_concentration: impl ChemicalDimensions = quantity!(1.0, mol / m^3);
    let molar_energy: impl ChemicalDimensions = quantity!(1.0, J / mol);
    
    println!("Chemical dimensions test passed");
}

#[test]
fn test_expanded_dimension_dsl_photometric() {
    // Test photometric dimensions
    define_generic_dimension!(PhotometricDimensions, Illuminance, Luminous_exposure, Luminous_efficacy);
    
    let illuminance: impl PhotometricDimensions = quantity!(1.0, lx);
    let luminous_exposure: impl PhotometricDimensions = quantity!(1.0, lx * s);
    let luminous_efficacy: impl PhotometricDimensions = quantity!(1.0, lm / W);
    
    println!("Photometric dimensions test passed");
}

#[test]
fn test_expanded_dimension_dsl_complex_expressions() {
    // Test complex dimension expressions using arithmetic operations
    define_generic_dimension!(ForceExpression, Mass * Length / Time^2);
    define_generic_dimension!(EnergyExpression, Mass * Length^2 / Time^2);
    define_generic_dimension!(PressureExpression, Mass / Length / Time^2);
    define_generic_dimension!(PowerExpression, Mass * Length^2 / Time^3);
    define_generic_dimension!(ElectricFieldExpression, Mass * Length / Time^3 / Current);
    define_generic_dimension!(CapacitanceExpression, Time^4 * Current^2 / Mass / Length^2);
    
    let force: impl ForceExpression = quantity!(1.0, N);
    let energy: impl EnergyExpression = quantity!(1.0, J);
    let pressure: impl PressureExpression = quantity!(1.0, Pa);
    let power: impl PowerExpression = quantity!(1.0, W);
    let electric_field: impl ElectricFieldExpression = quantity!(1.0, V / m);
    let capacitance: impl CapacitanceExpression = quantity!(1.0, F);
    
    println!("Complex dimension expressions test passed");
}

#[test]
fn test_expanded_dimension_dsl_naming_variations() {
    // Test various naming conventions for different dimensions
    define_generic_dimension!(DensityVariations, VolumeMassDensity, LinearMassDensity);
    define_generic_dimension!(ViscosityVariations, Viscosity, KinematicViscosity);
    define_generic_dimension!(ElectricalVariations, ElectricCharge, ElectricPotential);
    
    let volume_density: impl DensityVariations = quantity!(1.0, kg / m^3);
    let linear_density: impl DensityVariations = quantity!(1.0, kg / m);
    
    let dynamic_viscosity: impl ViscosityVariations = quantity!(1.0, Pa * s);
    let kinematic_viscosity: impl ViscosityVariations = quantity!(1.0, St);
    
    let charge: impl ElectricalVariations = quantity!(1.0, C);
    let potential: impl ElectricalVariations = quantity!(1.0, V);
    
    println!("Naming variations test passed");
}

#[test]
fn test_bespoke_quantity() {
    let kV = quantity!(1.0, V * s / m);
    println!("{:?}", kV);
}
