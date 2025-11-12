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

use whippyunits::dimension_traits::define_generic_dimension;
use whippyunits::op_result;
use whippyunits::output;
use whippyunits::quantity;
use whippyunits::unit;

// Define generic dimensions for the calculation
// Velocity: length per time (L/T)
define_generic_dimension!(Velocity, L / T);

// Curvature: angle per length (A/L)
define_generic_dimension!(Curvature, A / L);

// Acceleration: length per time squared (L/T²)
define_generic_dimension!(Acceleration, L / T ^ 2);

// Inverse radius: inverse length (1/L) - no angular dimension
// This represents 1/radius, which is curvature without angular units
define_generic_dimension!(InverseRadius, 1 / L);

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
/// Centripetal acceleration (L/T²) with angle dimension erased.  Preserves scale structure.
#[op_result]
pub fn centripetal_acceleration<V: Velocity, K: Curvature, A: Acceleration>(
    velocity: V,
    curvature: K,
) -> A
where
    V: Copy,
    [(); V * V * K]:,
    output!(V * V * K): Into<A>,
{
    (velocity * velocity * curvature).into()
}

/// Calculate centripetal acceleration from linear velocity and inverse radius
///
/// Formula: a = v² × (1/r)
///
/// This function accepts inverse radius (1/length) without angular units.
/// This is useful when working with APIs or formulas that use the definition
/// where curvature = 1/radius.
///
/// At the call site, you can use `.into()` erasure to convert from measured
/// curvature with angular units (rad/m, deg/m, etc.) to inverse radius (1/m).
///
/// This function is scale-generic - it works with any combination of scales:
/// - m/s and 1/m → m/s²
/// - mm/s and 1/mm → mm/s²
/// - km/h and 1/km → km/h²
/// - etc.
///
/// The type system ensures dimensional correctness at compile time:
/// - (L/T)² × (1/L) = L²/T² × 1/L = L/T²
///
/// # Arguments
/// * `velocity` - Linear velocity (any length/time scale)
/// * `inverse_radius` - Inverse radius (any 1/length scale, e.g., 1/m, 1/mm)
///
/// # Returns
/// Centripetal acceleration (L/T²)
#[op_result]
pub fn centripetal_acceleration_inverse_radius<V: Velocity, K: InverseRadius, A: Acceleration>(
    velocity: V,
    inverse_radius: K,
) -> A
where
    V: Copy,
    [(); V * V * K = A]:,
{
    velocity * velocity * inverse_radius
}

fn main() {
    println!("Centripetal Acceleration Demo\n");

    // Using radians
    let velocity = quantity!(10.0, m / s);
    let curvature = quantity!(1.0, rad / m);
    let acceleration = centripetal_acceleration::<_, _, unit!(m / s2)>(velocity, curvature);
    println!("Radians: {} at {} → {}", velocity, curvature, acceleration);

    // Using degrees
    let velocity = quantity!(10.0, m / s);
    let curvature = quantity!(10.0, deg / m);
    let acceleration = centripetal_acceleration::<_, _, unit!(deg.m / rad.s2)>(velocity, curvature);
    println!("Degrees: {} at {} → {}", velocity, curvature, acceleration);

    // Using rotations (revolutions)
    let velocity = quantity!(10.0, m / s);
    let curvature = quantity!(0.1, rot / m);
    let acceleration = centripetal_acceleration::<_, _, unit!(rot.m / rad.s2)>(velocity, curvature);
    println!(
        "Rotations: {} at {} → {}",
        velocity, curvature, acceleration
    );

    println!("\n--- Inverse Radius with Erasure ---\n");

    // Using inverse radius with erasure from rad/m
    // The function contract expects inverse radius (1/m), but we have
    // measured curvature with angular units (rad/m). We use `.into()` erasure
    // directly at the call site to convert rad/m → 1/m.
    //
    // Note: Type annotation is still needed because Rust can't infer which concrete
    // type `.into()` should produce from the trait bound alone. However, erasure
    // now works for all scales (not just clean scales), preserving scale structure.
    let velocity = quantity!(10.0, m / s);
    let measured_curvature = quantity!(1.0, rad / m);
    let acceleration = centripetal_acceleration_inverse_radius::<_, unit!(1 / m), unit!(m / s2)>(
        velocity,
        measured_curvature.into(),
    );
    println!(
        "rad/m → 1/m via erasure: {} at {} → {}",
        velocity, measured_curvature, acceleration
    );

    // Using inverse radius with erasure from deg/m
    // Same pattern: deg/m → 1/m via `.into()` erasure at the call site
    // The key difference: deg/m has a Pi component in its scale structure.
    // We use `unit!(deg/rad.m)` to preserve that scale structure while erasing
    // the angular dimension. This is now possible thanks to generalized erasure!
    let velocity = quantity!(10.0, m / s);
    let measured_curvature = quantity!(57.2958, deg / m); // ≈ 1 rad/m
    let acceleration = centripetal_acceleration_inverse_radius::<
        _,
        unit!(deg / rad.m),
        unit!(deg.m / rad.s2),
    >(velocity, measured_curvature.into());
    println!(
        "deg/m → 1/m via erasure: {} at {} → {}",
        velocity, measured_curvature, acceleration
    );
}
