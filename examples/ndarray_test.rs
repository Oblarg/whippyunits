use whippyunits::*;
use whippyunits::default_declarators::*;
use whippyunits::quantity_type::Quantity;
use ndarray::{Array1, Array2};

fn main() {
    let v1: Array1<unit!(m)> = Array1::from(vec![1.0.meters(), 2.0.meters(), 3.0.meters()]);
    let v2: Array1<unit!(m)> = Array1::from(vec![1.0.meters(), 2.0.meters(), 3.0.meters()]);
    println!("v1: {}", v1 + v2);
}