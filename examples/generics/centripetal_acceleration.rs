//! Centripetal Acceleration Example: Scale-Generic Geometric Calculation
//!
//! This example demonstrates a scale-generic calculation for centripetal acceleration
//! from linear velocity and curvature. The calculation works with any scale combination:
//! meters/second and 1/meters, millimeters/second and 1/millimeters, etc.
//!
//! Key concepts:
//! - Scale-generic: Works with any scale combination (m/s + rad/m, mm/s + rad/mm, etc.)
//! - Dimensional safety: The type system ensures velocity² × curvature = acceleration
//! - Practical application: Used in vehicle dynamics, robotics, path planning
//!
//! The physics: a = v² / r = v² × (κ / rad)
//! where:
//! - v is linear velocity (L/T)
//! - r is radius of curvature (L)
//! - κ is curvature (A/L) - angle per length
//! - a is centripetal acceleration (L/T²)

use core::ops::{Mul, Div};
use whippyunits::dimension_traits::define_generic_dimension;
use whippyunits::quantity;
use whippyunits::output;
use whippyunits::unit;

// Define generic dimensions for the calculation
// Velocity: length per time (L/T)
define_generic_dimension!(Velocity, L / T);

// Curvature: angle per length (A/L)
define_generic_dimension!(Curvature, A / L);

// Acceleration: length per time squared (L/T²)
define_generic_dimension!(Acceleration, L / T^2);

/// Calculate centripetal acceleration from linear velocity and curvature
///
/// Formula: a = v² × κ
///
/// This function is completely scale-generic - it works with any combination of scales:
/// - m/s and rad/m → m/s²
/// - mm/s and rad/mm → mm/s²
/// - km/h and rad/km → km/h²
/// - etc.
///
/// The type system ensures dimensional correctness at compile time:
/// - (L/T)² × (A/L) = L²/T² × A/L = A×L/T²
/// - Note: When angle is in radians (dimensionless), this simplifies to L/T²
///
/// # Arguments
/// * `velocity` - Linear velocity (any length/time scale)
/// * `curvature` - Curvature (any angle/length scale, e.g., rad/m, deg/mm)
///
/// # Returns
/// Centripetal acceleration (L/T²) with angle dimension erased.
/// The angle dimension is erased by dividing by radians, which is mathematically
/// equivalent to treating radians as dimensionless (since rad = 1).
pub fn centripetal_acceleration<V: Velocity, K: Curvature, A: Acceleration>(
    velocity: V, 
    curvature: K) -> A
where
    V: Mul<V> + Copy,
    output!(V * V): Mul<K> + Copy,
    output!(V * V * K): Div<unit!(rad), Output = A> + Copy,
{

    // in a non-generic context we could use `into` for erasure, but here we have
    // no way to represent the target type generically, so we divide by radians
    (velocity * velocity * curvature) / quantity!(1.0, rad)
}

fn main() {
    println!("Centripetal Acceleration Demo\n");

    // Example 1: Using radians
    let velocity = quantity!(10.0, m / s);
    let curvature = quantity!(1.0, rad / m);
    let acceleration = centripetal_acceleration(velocity, curvature);
    println!("Example 1 (radians): {} at {} → {}", velocity, curvature, acceleration);

    // Example 2: Using degrees
    let velocity = quantity!(10.0, m / s);
    let curvature = quantity!(10.0, deg / m);
    let acceleration = centripetal_acceleration(velocity, curvature);
    println!("Example 2 (degrees): {} at {} → {}", velocity, curvature, acceleration);

    // Example 3: Using rotations (revolutions)
    let velocity = quantity!(10.0, m / s);
    let curvature = quantity!(0.1, rot / m);
    let acceleration = centripetal_acceleration(velocity, curvature);
    println!("Example 3 (rotations): {} at {} → {}", velocity, curvature, acceleration);
}

