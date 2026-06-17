// This file tests that invalid operations between quantities with different
// dimensions or incompatible scales fail to compile.
//
// We use trybuild to verify that these operations actually fail at compile time.

#[test]
#[cfg(not(has_generic_const_exprs))]
fn test_compile_failures_stable() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail_stable/*.rs");
}

#[test]
#[cfg(has_generic_const_exprs)]
fn test_compile_failures_cge() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail_cge/*.rs");
}

