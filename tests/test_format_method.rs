//! Test the new format method for quantities

use whippyunits::default_declarators::*;
use whippyunits::*;

whippyunits::define_literals!();

#[culit::culit]
#[test]
fn test_format_method() {
    // Test various unit conversions
    let distance = 1000.0m_f64;
    
    // Test length conversions
    let km_result = distance.fmt("km");
    println!("1000m in km: {}", km_result);
    assert_eq!(format!("{}", km_result), "1 km");
    
    let ft_result = distance.fmt("ft");
    println!("1000m in ft: {}", ft_result);
    assert_eq!(format!("{}", ft_result), "3280.839895013123 ft");
    
    let mi_result = distance.fmt("mi");
    println!("1000m in mi: {}", mi_result);
    assert_eq!(format!("{}", mi_result), "0.621371192237334 mi");
    
    let mass = 1.0kg_f64;
    
    // Test mass conversions
    let g_result = mass.fmt("g");
    println!("1kg in g: {}", g_result);
    assert_eq!(format!("{}", g_result), "1000 g");
    
    let lb_result = mass.fmt("lb");
    println!("1kg in lb: {}", lb_result);
    assert_eq!(format!("{}", lb_result), "2.2046226218487757 lb");
    
    let oz_result = mass.fmt("oz");
    println!("1kg in oz: {}", oz_result);
    assert_eq!(format!("{}", oz_result), "35.27396194958041 oz");


    let error_result = distance.fmt("kg"); // Should fail - length vs mass
    println!("1000m in kg: {}", error_result);
    assert_eq!(format!("{}", error_result), "Error: Dimension mismatch: cannot convert from m to kg");

    println!("Testing precision specifiers:");
    println!("No precision: {}", distance.fmt("km"));
    println!("2 decimal places: {:.2}", distance.fmt("km"));
    println!("0 decimal places: {:.0}", distance.fmt("km"));
    println!("4 decimal places: {:.4}", distance.fmt("km"));
    
}
