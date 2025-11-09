//! Branded Declarators
//!
//! Branded declarators add a type-level marker to prevent mixing quantities
//! from different contexts (e.g., different coordinate systems).

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::define_unit_declarators;
use whippyunits::value;

// Create a branded declarator module
define_unit_declarators!(local_coords, LocalBrand);

fn main() {
    println!("Branded Declarators");
    println!("===================\n");

    use local_coords::*;

    // Quantities created with branded declarators have the brand type
    let distance1 = quantity!(5.0, m);
    let distance2 = quantity!(3.0, m);

    // ✅ Same brand: can operate together
    let sum = distance1 + distance2;
    println!("✅ Same brand: {} m + {} m = {} m",
             value!(distance1, m, f64, LocalBrand),
             value!(distance2, m, f64, LocalBrand),
             value!(sum, m, f64, LocalBrand));

    // ❌ Different brand: cannot operate together
    // let default_distance = whippyunits::quantity!(5.0, m);  // Brand: ()
    // let mixed = distance1 + default_distance;  // Compile error!
    println!("\n❌ Cannot mix branded and default quantities");
    println!("   // let mixed = local_coords::quantity!(5.0, m) + whippyunits::quantity!(5.0, m);");
    println!("   // Error: cannot add Quantity<m, f64, LocalBrand> and Quantity<m, f64, ()>");
}

