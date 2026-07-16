#![allow(non_snake_case)]
#![allow(dead_code)]

// Shadow `Vec`/`Result`/`Option`/etc so bare references emitted by an
// unhygienic macro will resolve to one of these instead of the real prelude
// item, causing a compile error.
struct Vec;
struct Result;
struct Option;
struct Some;
struct None;
struct Ok;
struct Err;
struct String;
struct Box;

use whippyunits::default_declarators::*;
use whippyunits::dimension_traits::define_generic_dimension;
use whippyunits::quantity;
use whippyunits::unit;
use whippyunits::value;
use whippyunits::{from_json, from_string};

#[test]
fn test_quantity_and_default_declarators_under_shadowed_prelude() {
    let distance = 5.0.meters();
    let time = 2.0.seconds();

    assert_eq!(value!(distance, m), 5.0);
    assert_eq!(value!(time, s), 2.0);

    let via_macro = quantity!(5.0, m);
    let via_unit_annotation: unit!(m) = via_macro;
    assert_eq!(value!(via_unit_annotation, m), 5.0);
}

#[test]
fn test_arithmetic_under_shadowed_prelude() {
    let sum = 5.0.meters() + 3.0.meters();
    assert_eq!(value!(sum, m), 8.0);

    let mut accum = 1.0.meters();
    accum += 1.0.meters();
    assert_eq!(value!(accum, m), 2.0);

    let product: unit!(m ^ 2) = 5.0.meters() * 5.0.meters();
    assert_eq!(value!(product, m ^ 2), 25.0);
}

#[test]
fn test_comparisons_under_shadowed_prelude() {
    define_generic_dimension!(Length, L);
    fn assert_length<T: Length>(value: T) -> T {
        value
    }

    let short = assert_length(1.0.meters());
    let long = assert_length(2.0.meters());

    assert!(short < long);
    assert!(long > short);
    assert!(short == 1.0.meters());
}

#[test]
fn test_rescale_under_shadowed_prelude() {
    let distance: unit!(mm) = whippyunits::api::rescale(1.0.meters());
    assert_eq!(value!(distance, mm), 1000.0);
}

#[test]
fn test_serialization_under_shadowed_prelude() {
    let length: unit!(m) = from_json!(r#"{"value": 5.0, "unit": "m"}"#, m).unwrap();
    assert_eq!(value!(length, m), 5.0);

    let error = from_json!(r#"{"value": 5.0, "unit": "m"}"#, kg);
    assert!(error.is_err());

    let length: unit!(m) = from_string!("5.0 m", m).unwrap();
    assert_eq!(value!(length, m), 5.0);

    let error = from_string!("5.0 m", kg);
    assert!(error.is_err());
}

mod shadowed_crate_name {
    #![allow(non_camel_case_types)]

    // Shadow whippyunits' dependency crates so that bare references to them
    // by unhygienic macros (e.g. macros using `whippyunits_core` instead of
    // `$crate::__private_whippyunits_core`) will cause a compile error.
    struct whippyunits_core;
    struct whippyunits_proc_macros;
    struct num_traits;

    use ::whippyunits::default_declarators::*;
    use ::whippyunits::from_json;
    use ::whippyunits::quantity;
    use ::whippyunits::unit;
    use ::whippyunits::value;

    #[test]
    fn test_macros_under_shadowed_dependency_crate_names() {
        let distance = 5.0.meters();
        assert_eq!(value!(distance, m), 5.0);

        let via_macro: unit!(m) = quantity!(5.0, m);
        assert_eq!(value!(via_macro, m), 5.0);

        let length: unit!(m) = from_json!(r#"{"value": 5.0, "unit": "m"}"#, m).unwrap();
        assert_eq!(value!(length, m), 5.0);
    }
}
