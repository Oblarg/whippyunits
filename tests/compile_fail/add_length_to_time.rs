// This should fail to compile: can't add length to time
use whippyunits::*;
use whippyunits::default_declarators::*;

fn main() {
    let length = 5.0.meters();
    let time = 10.0.seconds();
    
    // This should fail to compile: can't add length to time
    let _result = length + time;
}
