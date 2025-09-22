//! Test the culit proc macro directly

use whippyunits::culit;

#[culit]
fn test_proc_macro() {
    // Test that the proc macro generates the custom_literal module
    // and that we can use custom literals
    let distance = 100m_f64;
    let mass = 5.5kg_f64;
    let time = 30s_f64;
    
    // These are currently placeholder implementations
    assert_eq!(distance, 100.0);
    assert_eq!(mass, 5.5);
    assert_eq!(time, 30.0);
    
    println!("Proc macro test passed!");
}

#[test]
fn run_proc_macro_test() {
    test_proc_macro();
}
