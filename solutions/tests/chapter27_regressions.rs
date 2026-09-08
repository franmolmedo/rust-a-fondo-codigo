use course_solutions::organization::c27::{ProfileMeasurement, compare_profiles};

#[test]
fn c27_equal_profile_measurements_report_a_tie() {
    let measurement = ProfileMeasurement {
        build_millis: 100,
        binary_bytes: 500,
        run_nanos: 50,
    };
    let comparison = compare_profiles(measurement, measurement);
    assert_eq!(comparison.faster_build, "tie");
    assert_eq!(comparison.smaller_binary, "tie");
    assert_eq!(comparison.faster_run, "tie");
}

mod book_optional_json {
    include!("../../listings/27/010.rs");

    #[test]
    fn c27_the_core_type_exists_without_requiring_json() {
        assert_eq!(Order { id: 7 }.id, 7);
    }

    #[cfg(feature = "json")]
    #[test]
    fn c27_the_book_json_api_compiles_and_returns_the_serialization_result() {
        assert_eq!(Order { id: 7 }.to_json().unwrap(), r#"{"id":7}"#);
    }
}
