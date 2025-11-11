//! Hello World - Your First Quantity
//!
//! This is the simplest possible whippyunits program.
//! It demonstrates creating a quantity and using it.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use whippyunits::default_declarators::*;
use whippyunits::quantity;

fn main() {
    // The simplest way to create a quantity: using the quantity! macro
    // You can also use method syntax (requires importing default_declarators::*)
    //
    // Key takeaway: A Quantity is a type-safe wrapper around a number
    // that knows its units. The type system prevents you from mixing
    // incompatible units (like adding meters to seconds).

    println!("Hello, WhippyUnits!\n");

    let distance = quantity!(100.0, m);
    println!("I created a distance: {:?}", distance);

    let mass = 5.0.kilograms();
    println!("I created a mass: {:?}\n", mass);

    println!("Next: Learn about core concepts in concepts.rs");
}
