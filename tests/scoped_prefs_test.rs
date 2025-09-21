use whippyunits::define_base_units;
use whippyunits::api::*;

#[test]
fn test_scoped_prefs() {
    define_base_units!(Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian);
    let energy = 1.0.kilograms() * 1.0.meters() * 1.0.meters() / 1.0.seconds() / 1.0.seconds();
    println!("{:?}", energy);
}