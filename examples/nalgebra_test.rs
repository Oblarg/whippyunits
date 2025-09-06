use whippyunits::*;
use whippyunits::default_declarators::*;
use whippyunits::unit_macro::*;
use whippyunits::quantity_type::Quantity;
use nalgebra::{Vector3, Matrix3};

fn main() {
    let v1: Vector3<unit!(m)> = Vector3::new(1.0.meters(), 2.0.meters(), 3.0.meters());
    let v2 = v1 + v1;
    println!("v1: {}", v2);
    
    // This should work - same dimensions
    let v3: Vector3<unit!(m)> = Vector3::new(1.0.meters(), 1.0.meters(), 1.0.meters());
    let v4 = v2 + v3;
    println!("v4: {}", v4);

    // This should NOT compile - different dimensions
    // let v_time: Vector3<unit!(s)> = Vector3::new(1.0.seconds(), 1.0.seconds(), 1.0.seconds());
    // let v_mixed = v1 + v_time; // Should fail: Length + Time
}