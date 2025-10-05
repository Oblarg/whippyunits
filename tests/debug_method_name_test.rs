use whippyunits::default_declarators::*;
use whippyunits::*;

whippyunits::define_literals!();

#[test]
fn debug_method_names() {
    // Test what happens when we create the ms literal
    let time = 30.0ms;
    println!("ms literal creates: {}", time);
    
    // Test what happens when we create the mm literal  
    let length = 30.0mm;
    println!("mm literal creates: {}", length);
}
