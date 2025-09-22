//! Demo showing custom literals for whippyunits
//! 
//! This example demonstrates how to use custom literals like `100m_f64`, `5.5kg_f64`, etc.
//! with the culit crate integration.

use whippyunits::culit;

#[culit]
fn main() {
    println!("=== Whippyunits Custom Literals Demo ===\n");
    
    // Basic length measurements
    println!("Length measurements:");
    let distance1 = 100m_f64;  // 100 meters as f64
    let distance2 = 5.5km_f64; // 5.5 kilometers as f64
    let distance3 = 2500mm_f64; // 2500 millimeters as f64
    
    println!("  Distance 1: {:?}", distance1);
    println!("  Distance 2: {:?}", distance2);
    println!("  Distance 3: {:?}", distance3);
    
    // Mass measurements
    println!("\nMass measurements:");
    let mass1 = 75kg_f64;      // 75 kilograms as f64
    let mass2 = 1500g_f64;     // 1500 grams as f64
    let mass3 = 500mg_f64;     // 500 milligrams as f64
    
    println!("  Mass 1: {:?}", mass1);
    println!("  Mass 2: {:?}", mass2);
    println!("  Mass 3: {:?}", mass3);
    
    // Time measurements
    println!("\nTime measurements:");
    let time1 = 30s_f64;       // 30 seconds as f64
    let time2 = 2.5min_f64;    // 2.5 minutes as f64
    let time3 = 1h_f64;        // 1 hour as f64
    
    println!("  Time 1: {:?}", time1);
    println!("  Time 2: {:?}", time2);
    println!("  Time 3: {:?}", time3);
    
    // Integer literals
    println!("\nInteger measurements:");
    let count1 = 10m_i32;      // 10 meters as i32
    let count2 = 100kg_i64;    // 100 kilograms as i64
    let count3 = 5000ms_u32;   // 5000 milliseconds as u32
    
    println!("  Count 1: {:?}", count1);
    println!("  Count 2: {:?}", count2);
    println!("  Count 3: {:?}", count3);
    
    // Different numeric types
    println!("\nDifferent numeric types:");
    let float32_val = 3.14m_f32;   // f32
    let int32_val = 42kg_i32;      // i32
    let int64_val = 1000s_i64;     // i64
    let uint32_val = 500A_u32;     // u32
    let uint64_val = 2000K_u64;    // u64
    
    println!("  Float32: {:?}", float32_val);
    println!("  Int32: {:?}", int32_val);
    println!("  Int64: {:?}", int64_val);
    println!("  Uint32: {:?}", uint32_val);
    println!("  Uint64: {:?}", uint64_val);
    
    // Electrical units
    println!("\nElectrical measurements:");
    let current1 = 2.5A_f64;       // 2.5 amperes
    let current2 = 500mA_f64;      // 500 milliamperes
    let voltage = 12V_f64;         // 12 volts (if defined)
    
    println!("  Current 1: {:?}", current1);
    println!("  Current 2: {:?}", current2);
    println!("  Voltage: {:?}", voltage);
    
    // Temperature units
    println!("\nTemperature measurements:");
    let temp1 = 20K_f64;           // 20 kelvin
    let temp2 = 300mK_f64;         // 300 millikelvin
    
    println!("  Temperature 1: {:?}", temp1);
    println!("  Temperature 2: {:?}", temp2);
    
    // Amount units
    println!("\nAmount measurements:");
    let amount1 = 1mol_f64;        // 1 mole
    let amount2 = 500mmol_f64;     // 500 millimoles
    
    println!("  Amount 1: {:?}", amount1);
    println!("  Amount 2: {:?}", amount2);
    
    // Luminosity units
    println!("\nLuminosity measurements:");
    let lum1 = 100cd_f64;          // 100 candela
    let lum2 = 50mcd_f64;          // 50 millicandela
    
    println!("  Luminosity 1: {:?}", lum1);
    println!("  Luminosity 2: {:?}", lum2);
    
    // Angle units
    println!("\nAngle measurements:");
    let angle1 = 1.57rad_f64;      // 1.57 radians
    let angle2 = 90deg_f64;        // 90 degrees
    let angle3 = 1000mrad_f64;     // 1000 milliradians
    
    println!("  Angle 1: {:?}", angle1);
    println!("  Angle 2: {:?}", angle2);
    println!("  Angle 3: {:?}", angle3);
    
    println!("\n=== Demo completed successfully! ===");
    println!("Note: These are placeholder implementations.");
    println!("In a real implementation, these would create actual unit types");
    println!("with proper dimensional analysis and type safety.");
}
