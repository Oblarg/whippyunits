use whippyunits::default_declarators::*;
use whippyunits::*;

whippyunits::define_literals!();

#[test]
fn debug_symbols() {
    // Test if ms and mm literals exist by trying to use them
    // This will fail at compile time if they don't exist
    let _time = 30.0ms;
    let _length = 30.0mm;
    
    println!("ms and mm literals exist!");
}
