//! Syntax Options: Choosing the Right Way to Create Quantities
//!
//! WhippyUnits provides three ways to create quantities. This example
//! shows when to use each approach.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// For method syntax (.meters(), .seconds(), etc.)
use whippyunits::default_declarators::*;

// For macro syntax (quantity! macro)
use whippyunits::quantity;

// For literal syntax (5.0m, 10s, etc.) - requires culit attribute
// Requires: #[culit::culit(...)] attribute on function/module
// Pros: Most concise, natural reading
// Cons: Requires attribute, only works in annotated scopes
// Best for: Mathematical formulas, quick prototyping, most readable code
#[culit::culit(whippyunits::default_declarators::literals)]
fn main() {
    println!("WhippyUnits Syntax Options");
    println!("==========================\n");

    // ============================================================
    // OPTION 1: Method Syntax (.meters(), .seconds(), etc.)
    // ============================================================
    // Requires: use whippyunits::default_declarators::*;
    // Pros: Familiar Rust syntax, good IDE support
    // Cons: More verbose, requires trait import, limited compound units, non-const

    println!("1. Method Syntax (.meters(), .seconds(), etc.)");
    let _distance = 5.0.meters();
    let _mass = 2.5.kilograms();
    let _time = 30.0.seconds();
    println!();

    // ============================================================
    // OPTION 2: Macro Syntax (quantity! macro)
    // ============================================================
    // Requires: use whippyunits::quantity;
    // Pros: Flexible, works everywhere, arbitrary compound units, const
    // Cons: Macro syntax, less familiar

    println!("2. Macro Syntax (quantity! macro)");
    let _distance = quantity!(5.0, m);
    let _mass = quantity!(2.5, kg);
    let _time = quantity!(30.0, s);

    // Can also do calculations in the macro
    let _area = quantity!(5.0 * 4.0, m ^ 2);
    let _velocity = quantity!(100.0 / 10.0, m / s);

    // Best Practice: For compound units, use compound unit literal expressions
    // ✅ Preferred: quantity!(10.0, m / s)
    // ❌ Avoid: quantity!(10.0, m) / quantity!(1.0, s)
    // This provides better rust-analyzer interaction and more reliable constant folding

    // ============================================================
    // OPTION 3: Literal Syntax (5.0m, 10s, etc.)
    // ============================================================
    // Requires: #[culit::culit(whippyunits::default_declarators::literals)] attribute on function/module
    // Pros: Most concise, natural reading, const
    // Cons: Requires attribute, no compound units
    println!("3. Literal Syntax (5.0m, 10s, etc.)");

    let _distance = 100.0m; // f64 meters
    let _mass = 5kg; // i32 kilograms
    let _time = 10.0s; // f64 seconds
    println!();
}
