use whippyunits::*;

// Test what the macro expands to
type Test1 = unit!(mm);
type Test2 = unit!(s);
type Test3 = unit!(mm * s);

fn main() {
    println!("Test1: {}", std::any::type_name::<Test1>());
    println!("Test2: {}", std::any::type_name::<Test2>());
    println!("Test3: {}", std::any::type_name::<Test3>());
}
