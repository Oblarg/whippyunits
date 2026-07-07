use whippyunits::unit;

pub fn parse_length(json: &str) -> unit!(m) {
    whippyunits::from_json!(json, m).unwrap()
}
