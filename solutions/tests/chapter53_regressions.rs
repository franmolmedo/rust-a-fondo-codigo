use course_solutions::organization::c53::parse_entry;
use std::error::Error as _;

#[test]
fn c53_error_wrapping_preserves_the_original_parse_error() {
    let error = parse_entry(7, "port=65536").unwrap_err();
    assert_eq!(error.to_string(), "invalid entry at line 7");
    let original = error.source().unwrap().source().unwrap();
    let parse = original.downcast_ref::<std::num::ParseIntError>().unwrap();
    assert_eq!(parse.kind(), &std::num::IntErrorKind::PosOverflow);
    let separator = parse_entry(1, "port").unwrap_err();
    assert!(separator.source().unwrap().source().is_none());
    assert_eq!(parse_entry(1, "port=65535").unwrap(), ("port", u16::MAX));
}
