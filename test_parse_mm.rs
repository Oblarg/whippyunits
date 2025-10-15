fn test_parse_mm() { 
    let result = whippyunits_core::SiPrefix::strip_any_prefix_symbol("mm"); 
    println!("{:?}", result); 
}
