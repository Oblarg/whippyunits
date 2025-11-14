//! Non-Storage Units: Units with Conversion Factors
//!
//! This example demonstrates non-storage units, which are units that have
//! a conversion factor different from 1.0. These units are automatically
//! converted to their "nearest neighbor" power-of-10 multiple of an SI base unit
//! for storage.
//!
//! Nearest-neighbor storage semantics:
//!
//! WhippyUnits uses a logarithmic scale encoding system that only supports powers of 2, 3, 5, and π.
//! This greatly simplifies the type system: arithmetic (as opposed to a syntactic expression tree)
//! naturally normalizes itself, making it much easier to write generic code that reliably arrives
//! at the same representation of a given derived quantity, regardless of *how* it was derived.
//!
//! However, not every unit can be exactly represented in this way; for example,
//! the conversion factor of 30.48 between feet and centimeters cannot be exactly represented
//! by powers of 2, 3, 5, and π.
//!
//! We do not attempt to represent these units in our type system.  Instead
//! we offer convenient ways to *declare* quantities in non-storage units, and to *access* their
//! values in the original non-storage units; but the library itself will represent them internally
//! as their nearest-neighbor power-of-10 multiple of an SI base unit, they retain no "memory"
//! of their declaration unit, and there is a small loss of precision and runtime cost associated
//! with every conversion to or from a non-storage unit.
//!
//! We restrict to powers of 10 as opposed to the fully-general logarithmic scale encoding system
//! because these typically have better human-readable display properties (SI prefixes are always powers of 10).
//! If you need a more-faithful representation of the original unit, you may directly define a custom declarator
//! method that explicitly sets the scale exponents.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// For method syntax (.inches(), .pounds(), etc.)
use whippyunits::default_declarators::*;

// For macro syntax (quantity! macro)
use whippyunits::quantity;

// For accessing values with unit safety

use whippyunits::value;
// For literal syntax (12.0in, 1.0lb, etc.) - requires culit attribute
#[culit::culit(whippyunits::default_declarators::literals)]
fn main() {
    // ========================================================================
    // DECLARATION: Declaring Quantities in Non-Storage Units
    // ========================================================================
    //
    // Non-storage units can be declared using all three syntax options:
    // 1. Method syntax: 3.0.feet()
    // 2. Macro syntax: quantity!(3.0, ft)
    // 3. Literal syntax: 3.0ft (requires culit attribute)
    //
    // All three produce the same result: a quantity that is automatically
    // converted to its nearest-neighbor SI storage unit at declaration time.

    // Method syntax - most familiar, requires importing default_declarators::*
    // 3 feet = 3 * 30.48 = 91.44 cm (stored as decimeters, the nearest
    // power-of-10 multiple: 10^-1 m)

    // Macro syntax - flexible, works everywhere, supports expressions
    // Same as method syntax: 3 feet stored as 91.44 cm (9.144 dm)

    // Literal syntax - most concise, requires culit attribute on function/module
    // Same as method and macro syntax: 3 feet stored as 91.44 cm (9.144 dm)

    // Display the declared quantities
    println!("Declaration Examples:");
    println!("  Method syntax:   {}", 3.0.feet());
    // 1 pound = 0.453592 kg (stored as kilograms, the nearest power-of-10
    // multiple: 10^0 kg)
    println!("  Macro syntax:    {}", quantity!(3.0, ft));
    println!("  Literal syntax:  {}", 3.0ft);

    // More examples of nearest-neighbor storage:
    println!("\nNearest-Neighbor Storage Examples:");
    // 1 yard = 0.9144 m → stored as meters (10^0 m, nearest neighbor)
    println!("  {}", 1.0.yards());
    // 1 mile = 1.609344 km → stored as kilometers (10^3 m, nearest neighbor)
    println!("  {}", 1.0.miles());
    // 1 ounce = 28.3495 g → stored as decagrams (10^-1 kg, nearest neighbor)
    println!("  {}", 1.0.ounces());
    // 1 ton = 0.907185 Mg → stored as megagrams (10^3 kg, nearest neighbor)
    println!("  {}", 1.0.tons());

    // ========================================================================
    // ACCESS: Retrieving Values in Non-Storage Units
    // ========================================================================
    //
    // To access the underlying numeric value, use the value! macro with
    // an explicit unit specification. This is unit-safe: the compiler
    // verifies dimensional compatibility and automatically rescales.
    //
    // The value! macro can access values in:
    // - The storage unit (e.g., decimeters, kilograms)
    // - Any other compatible unit (e.g., meters, grams)

    println!("\nAccess Examples:");
    // Access in the storage unit (decimeters)
    // Returns 9.144 (3 * 30.48 / 10, the stored value)
    println!("  Stored value:   {} dm", value!(3.0.feet(), dm));
    // Access in any compatible unit (meters)
    // Returns 0.9144 (3 * 30.48 / 100, rescaled to meters)
    println!("  Rescaled value: {} m", value!(3.0.feet(), m));
    // Access in the original nonstorage unit (feet)
    // Returns 3.0 (the original value)
    println!("  Original value: {} ft", value!(3.0.feet(), ft));

    // Access in the storage unit (kilograms)
    // Returns 0.453592 (the stored value)
    println!("\n  Stored value:   {} kg", value!(1.0.pounds(), kg));
    // Access in grams
    // Returns 453.592 (rescaled to grams)
    println!("  Rescaled value: {} g", value!(1.0.pounds(), g));
    // Access in the original nonstorage unit (pounds)
    // Returns 1.0 (the original value)
    println!("  Original value: {} lb", value!(1.0.pounds(), lb));
}
