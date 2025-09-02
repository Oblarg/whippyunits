use whippyunits::*;

#[test]
fn test_scoped_prefs() {
    set_unit_preferences!(Milligram, Millimeter, Second);
    let distance = 5.0.meters();
    assert_eq!(distance.value, 5000.0);
}