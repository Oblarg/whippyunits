// This file tests that invalid operations between quantities with different
// dimensions or incompatible scales fail to compile.
//
// We use trybuild to verify that these operations actually fail at compile time.

#[test]
fn test_compile_failures() {
    let t = trybuild::TestCases::new();
    
    // Test that adding quantities with different dimensions fails to compile
    t.compile_fail("tests/compile_fail/*.rs");
}
