#![feature(impl_trait_in_bindings)]

use whippyunits::api::rescale;
use whippyunits::default_declarators::*;
use whippyunits::quantity;
use whippyunits::unit;

#[test]
fn test_reflexivity() {
    let a = 5.0.meters();

    // Reflexivity: a == a, a <= a, a >= a
    assert_eq!(a, a);
    assert!(a <= a);
    assert!(a >= a);
}

#[test]
fn test_antisymmetry() {
    let a = 5.0.meters();
    let b = 5.0.meters();

    // Antisymmetry: if a <= b and b <= a, then a == b
    assert!(a <= b);
    assert!(b <= a);
    assert_eq!(a, b);
}

#[test]
fn test_transitivity() {
    let a = 5.0.meters();
    let b = 10.0.meters();
    let c = 15.0.meters();

    // Transitivity: if a < b and b < c, then a < c
    assert!(a < b);
    assert!(b < c);
    assert!(a < c);

    // Also for <=
    assert!(a <= b);
    assert!(b <= c);
    assert!(a <= c);
}

#[test]
fn test_ordering_consistency() {
    let a = 5.0.meters();
    let b = 10.0.meters();

    // If a < b, then !(b < a) and !(a > b) and (b > a)
    assert!(a < b);
    assert!(!(b < a));
    assert!(!(a > b));
    assert!(b > a);

    // If a <= b, then !(b < a)
    assert!(a <= b);
    assert!(!(b < a));
}

#[test]
fn test_rescale_preserves_ordering() {
    let a = 1.0.meters();
    let b = 2.0.meters();
    let c = 500.0.millimeters();

    // Rescaling preserves ordering relationships
    // If a < b, then rescale(a) preserves ordering when compared to same scale
    assert!(a < b);

    // Rescale to match target scale (only one side rescaled)
    assert!(rescale(a) > c); // 1000mm > 500mm
    assert!(a > rescale(c)); // 1m > 0.5m
    assert!(rescale(b) > c); // 2000mm > 500mm
    assert!(b > rescale(c)); // 2m > 0.5m

    // Ordering preserved: rescale both to same target scale, then compare
    let a_mm: unit!(mm) = rescale(a);
    let b_mm: unit!(mm) = rescale(b);
    assert!(a_mm < b_mm); // 1000mm < 2000mm
}

#[test]
fn test_rescale_preserves_equality() {
    // Rescaling preserves equality
    assert_eq!(rescale(1.0.meters()), 1000.0.millimeters());
    assert_eq!(1.0.meters(), rescale(1000.0.millimeters()));
    assert_eq!(rescale(60.0.seconds()), 1.0.minutes());
    assert_eq!(60.0.seconds(), rescale(1.0.minutes()));
}

#[test]
fn test_partial_ordering_properties() {
    // Verify PartialOrd properties hold for integer types
    let a: unit!(m, i32) = quantity!(5, m, i32);
    let b: unit!(m, i32) = quantity!(10, m, i32);
    let c: unit!(m, i32) = quantity!(10, m, i32);

    // Transitivity
    assert!(a < b);
    assert!(b <= c);
    assert!(a < c);

    // Antisymmetry
    assert_eq!(b, c);
    assert!(b <= c);
    assert!(c <= b);
}

#[test]
fn test_sorting_consistency() {
    // Verify that sorting produces a consistent total order
    let values = vec![10.0.meters(), 5.0.meters(), 20.0.meters(), 1.0.meters()];

    let mut sorted = values.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Verify ordering is consistent
    for i in 0..sorted.len() - 1 {
        assert!(sorted[i] <= sorted[i + 1]);
        if i > 0 {
            assert!(sorted[i - 1] < sorted[i + 1]);
        }
    }
}

#[test]
fn test_zero_comparison_properties() {
    let zero = 0.0.meters();
    let positive = 5.0.meters();
    let negative = -3.0.meters();

    // Zero is less than positive, greater than negative
    assert!(zero < positive);
    assert!(zero > negative);

    // Zero equals itself
    assert_eq!(zero, 0.0.meters());

    // Transitivity through zero
    assert!(negative < zero);
    assert!(zero < positive);
    assert!(negative < positive);
}

#[test]
fn test_cross_scale_ordering_consistency() {
    // Verify that ordering is consistent across scales when rescaled
    let a_m = 1.0.meters();
    let b_m = 2.0.meters();
    let a_mm = 1000.0.millimeters();
    let b_mm = 2000.0.millimeters();

    // Same ordering should hold regardless of which side is rescaled
    assert!(a_m < b_m);
    assert!(rescale(a_m) < b_mm); // Rescale left side
    assert!(a_mm < rescale(b_m)); // Rescale right side

    // Equality preserved across scales (only one side rescaled)
    assert_eq!(rescale(a_m), a_mm);
    assert_eq!(a_m, rescale(a_mm));
    assert_eq!(rescale(b_m), b_mm);
    assert_eq!(b_m, rescale(b_mm));
}
