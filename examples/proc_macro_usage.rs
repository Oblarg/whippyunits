// Example showing how to use the re-exported define_generic_dimension proc macro
// This demonstrates that the proc macro is properly available from the whippyunits crate

use whippyunits::define_generic_dimension;
use whippyunits::default_declarators::*;

// Test the new algebraic syntax
define_generic_dimension!(Velocity, Length / Time);
define_generic_dimension!(Acceleration, Length / Time^2);
define_generic_dimension!(Force, Mass * Length / Time^2);
define_generic_dimension!(Energy, Mass * Length^2 / Time^2);

fn main() {
    // Test that the traits work as expected
    let velocity = 1.0.meters() / 1.0.seconds();
    let acceleration = 1.0.meters() / (1.0.seconds() * 1.0.seconds());
    let force = 1.0.kilograms() * 1.0.meters() / (1.0.seconds() * 1.0.seconds());
    let energy = 1.0.kilograms() * 1.0.meters() * 1.0.meters() / (1.0.seconds() * 1.0.seconds());
    
    // Test Velocity trait
    fn test_velocity<T: Velocity>(_x: T) {}
    test_velocity(velocity);
    
    // Test Acceleration trait
    fn test_acceleration<T: Acceleration>(_x: T) {}
    test_acceleration(acceleration);
    
    // Test Force trait
    fn test_force<T: Force>(_x: T) {}
    test_force(force);
    
    // Test Energy trait
    fn test_energy<T: Energy>(_x: T) {}
    test_energy(energy);
    
    println!("All proc macro tests passed!");
    println!("Velocity: {}", velocity);
    println!("Acceleration: {}", acceleration);
    println!("Force: {}", force);
    println!("Energy: {}", energy);
}
