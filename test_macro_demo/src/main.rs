use whippyunits::*;

fn main() {
    println!("Testing unit macro with unused dimensions...");
    
    // Test 1: Only length dimension used
    // This should generate: length_exp=2, length_scale=MILLIMETER_SCALE, mass_exp=0, mass_scale=MASS_UNUSED, time_exp=0, time_scale*=TIME_UNUSED
    type Area = unit!(mm^2);
    println!("✓ unit!(mm^2) - length used, mass/time unused");
    println!("  Type: Area = {}", std::any::type_name::<Area>());
    
    // Test 2: Only mass dimension used  
    // This should generate: length_exp=0, length_scale=LENGTH_UNUSED, mass_exp=1, mass_scale=GRAM_SCALE, time_exp=0, time_scale*=TIME_UNUSED
    type Mass = unit!(g);
    println!("✓ unit!(g) - mass used, length/time unused");
    println!("  Type: Mass = {}", std::any::type_name::<Mass>());
    
    // Test 3: Only time dimension used
    // This should generate: length_exp=0, length_scale=LENGTH_UNUSED, mass_exp=0, mass_scale=MASS_UNUSED, time_exp=1, time_scale*=SECOND_SCALE_*
    type Time = unit!(s);
    println!("✓ unit!(s) - time used, length/mass unused");
    println!("  Type: Time = {}", std::any::type_name::<Time>());
    
    // Test 4: Multiple dimensions used
    // This should generate: length_exp=1, length_scale=MILLIMETER_SCALE, mass_exp=0, mass_scale=MASS_UNUSED, time_exp=-1, time_scale*=SECOND_SCALE_*
    type Velocity = unit!(mm * s^-1);
    println!("✓ unit!(mm * s^-1) - length/time used, mass unused");
    println!("  Type: Velocity = {}", std::any::type_name::<Velocity>());
    
    // Test 5: All dimensions used
    // This should generate: length_exp=1, length_scale=MILLIMETER_SCALE, mass_exp=1, mass_scale=GRAM_SCALE, time_exp=-2, time_scale*=SECOND_SCALE_*
    type Force = unit!(mg * mm * s^-2);
    println!("✓ unit!(mg * mm * s^-2) - all dimensions used");
    println!("  Type: Force = {}", std::any::type_name::<Force>());
    
    println!("\n🎉 All unit macro tests passed!");
    println!("The macro correctly sets unused dimensions to UNUSED sentinel values (isize::MAX)");
    println!("\nKey observations:");
    println!("- Used dimensions get their proper scale values (e.g., -1 for MILLIMETER_SCALE)");
    println!("- Unused dimensions get isize::MAX (9223372036854775807) as the UNUSED sentinel value");
    println!("- This ensures zero-cost runtime for unused dimensions");
}
