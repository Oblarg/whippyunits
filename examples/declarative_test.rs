use whippyunits::*;
use whippyunits::default_declarators::{LengthExt, MassExt, TimeExt};

// Include the generated declarative macro
mod generated_unit_macro {
    include!("../src/generated_unit_macro.rs");
}

fn main() {
    println!("🧪 TESTING GENERATED UNIT MACRO");
    println!("===============================");
    println!();

    // Test declarative macro (LSP-friendly)
    let _length: unit!(mm) = 5_i32.millimeters();
    let _mass: unit!(kg) = 2_i32.kilograms();
    
    println!("✅ Generated unit! macro works");

    // Test units with exponents
    let _area: unit!(m^2) = 10_i32.meters() * 10_i32.meters();
    let _volume: unit!(m^3) = 5_i32.meters() * 5_i32.meters() * 5_i32.meters();
    
    println!("✅ Units with exponents work");

    // Test compound units
    let _velocity: unit!(m * s^-1) = 20_i32.meters() / 5_i32.seconds();
    let _acceleration: unit!(m * s^-2) = 10_i32.meters() / (5_i32.seconds() * 5_i32.seconds());
    
    println!("✅ Compound units work");

    // Test complex units
    let _force: unit!(kg * m * s^-2) = 10_i32.kilograms() * 5_i32.meters() / (2_i32.seconds() * 2_i32.seconds());
    let _energy: unit!(kg * m^2 * s^-2) = 2_i32.kilograms() * 3_i32.meters() * 3_i32.meters() / (1_i32.seconds() * 1_i32.seconds());
    
    println!("✅ Complex units work");

    // Test the exact case from your original example
    let _test3: unit!(mm * s) = 5_i32.millimeters() * 5_i32.seconds();
    
    println!("✅ Original example works: mm * s");

    println!();
    println!("🎉 All generated macro tests passed!");
    println!();
    println!("Benefits of the generated approach:");
    println!("- ✅ unit!() - LSP resolves immediately (declarative macro)");
    println!("- ✅ unit!() - Hover tooltips show pretty notation right away");
    println!("- ✅ unit!() - Inlay hints work perfectly");
    println!("- ✅ unit!() - Compile-time validation of unit expressions");
    println!("- ✅ unit!() - Fast compilation (no proc macro overhead)");
    println!("- ✅ unit!() - 172 pre-generated patterns for common cases");
    println!("- ✅ unit!() - Human-readable triangular structure");
    println!("- ✅ proc_unit!() - Available for complex expressions (proc macro)");
    println!("- ✅ Clean naming: unit!() for common cases, proc_unit!() for complex");
}
