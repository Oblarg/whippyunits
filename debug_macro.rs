use whippyunits::*;

fn main() {
    // Let's see what the macro actually generates
    type Test1 = unit!(mg * mm * s^-2);
    type Test2 = unit!(mm * s^-1);
    type Test3 = unit!(mg * mm);
    
    println!("Test1 type: {}", std::any::type_name::<Test1>());
    println!("Test2 type: {}", std::any::type_name::<Test2>());
    println!("Test3 type: {}", std::any::type_name::<Test3>());
}
