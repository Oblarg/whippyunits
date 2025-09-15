use whippyunits::set_unit_preferences;
use whippyunits::generated_api::*;
use whippyunits::generated_quantity_type::Quantity;

#[test]
fn test_scoped_prefs() {
    set_unit_preferences!(Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian);
    let energy = 1.0.kilograms() * 1.0.meters() * 1.0.meters() / 1.0.seconds() / 1.0.seconds();
    println!("{:?}", energy);
}