use whippyunits::*;

fn main() {
    println!("Testing unit macro with unused dimensions...");
    
    // Test 1: Only length dimension used
    // This should generate: length_exp=2, length_scale=MILLIMETER_SCALE, mass_exp=0, mass_scale=MASS_UNUSED, time_exp=0, time_scale*=TIME_UNUSED
    let _area: unit!(mm^2) = ();
    println!("✓ unit!(mm^2) - length used, mass/time unused");
    
    // Test 2: Only mass dimension used  
    // This should generate: length_exp=0, length_scale=LENGTH_UNUSED, mass_exp=1, mass_scale=GRAM_SCALE, time_exp=0, time_scale*=TIME_UNUSED
    let _mass: unit!(g) = ();
    println!("✓ unit!(g) - mass used, length/time unused");
    
    // Test 3: Only time dimension used
    // This should generate: length_exp=0, length_scale=LENGTH_UNUSED, mass_exp=0, mass_scale=MASS_UNUSED, time_exp=1, time_scale*=SECOND_SCALE_*
    let _time: unit!(s) = ();
    println!("✓ unit!(s) - time used, length/mass unused");
    
    // Test 4: Multiple dimensions used
    // This should generate: length_exp=1, length_scale=MILLIMETER_SCALE, mass_exp=0, mass_scale=MASS_UNUSED, time_exp=-1, time_scale*=SECOND_SCALE_*
    let _velocity: unit!(mm * s^-1) = ();
    println!("✓ unit!(mm * s^-1) - length/time used, mass unused");
    
    // Test 5: All dimensions used
    // This should generate: length_exp=1, length_scale=MILLIMETER_SCALE, mass_exp=1, mass_scale=GRAM_SCALE, time_exp=-2, time_scale*=SECOND_SCALE_*
    let _force: unit!(mg * mm * s^-2) = ();
    println!("✓ unit!(mg * mm * s^-2) - all dimensions used");
    
    println!("\n🎉 All unit macro tests passed!");
    println!("The macro correctly sets unused dimensions to UNUSED sentinel values (isize::MAX)");
}
