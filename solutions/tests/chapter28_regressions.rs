use course_solutions::organization::c28::{
    BackendFailure, FailingRegistrationPort, FixedClock, RegistrationError, TestBoundary,
    TestLayer, classify_test, numbers_round_trip, parse_key_value, register, token_is_expired,
};
use proptest::prelude::*;
use proptest::test_runner::{Config, TestError, TestRng, TestRunner};
use std::error::Error;

#[test]
fn c28_a_real_adapter_does_not_make_a_test_end_to_end() {
    let adapter = TestBoundary {
        crosses_public_api: true,
        uses_real_external_adapter: true,
        covers_complete_workflow: false,
    };
    assert_eq!(classify_test(adapter), TestLayer::Integration);
    assert_eq!(
        classify_test(TestBoundary {
            covers_complete_workflow: true,
            ..adapter
        }),
        TestLayer::EndToEnd
    );
}

#[test]
fn c28_error_translation_preserves_the_underlying_cause() {
    let error = register(&mut FailingRegistrationPort, 7).unwrap_err();
    assert_eq!(
        error,
        RegistrationError::RepositoryUnavailable(BackendFailure)
    );
    assert!(error.source().unwrap().is::<BackendFailure>());
}

#[test]
fn c28_parser_contract_covers_empty_values_and_unicode_without_indexing_bytes() {
    assert_eq!(parse_key_value("name="), Ok(("name", "")));
    assert_eq!(parse_key_value("=value"), Err("empty key"));
    assert_eq!(parse_key_value("name=a=b"), Ok(("name", "a=b")));
    assert_eq!(parse_key_value("日本語=中文"), Ok(("日本語", "中文")));
    assert_eq!(parse_key_value("\u{0301}"), Err("missing ="));
}

#[test]
fn c28_time_and_number_boundaries_are_explicit() {
    assert!(!token_is_expired(&FixedClock(999), 1_000));
    assert!(token_is_expired(&FixedClock(1_000), 1_000));
    assert!(token_is_expired(&FixedClock(u64::MAX), u64::MAX));
    assert!(numbers_round_trip(&[]).unwrap().is_empty());
    let values = [i32::MIN, -1, 0, 1, i32::MAX];
    assert_eq!(numbers_round_trip(&values).unwrap(), values);
}

#[test]
fn c28_a_deliberate_round_trip_bug_is_reduced_to_a_single_element() {
    let config = Config {
        cases: 32,
        failure_persistence: None,
        max_shrink_iters: 4_096,
        ..Config::default()
    };
    let rng = TestRng::deterministic_rng(config.rng_algorithm);
    let mut runner = TestRunner::new_with_rng(config, rng);
    let outcome = runner.run(&proptest::collection::vec(any::<u8>(), 0..16), |values| {
        // Deliberate bug for this demonstration: lose the last encoded value.
        let mut decoded = values.clone();
        let _ = decoded.pop();
        prop_assert_eq!(decoded, values);
        Ok(())
    });
    match outcome {
        Err(TestError::Fail(_, reduced)) => assert_eq!(reduced, [0]),
        other => panic!("expected a reduced counterexample, got {other:?}"),
    }
}

mod c28_book_io_failure {
    include!("../../listings/28/008.rs");
}

mod c28_book_registration {
    include!("../../listings/28/010.rs");
    include!("../../listings/28/011.rs");
    include!("../../listings/28/012.rs");
}

mod c28_book_round_trip {
    include!("../../listings/28/014.rs");
}

mod c28_book_parser {
    include!("../../listings/28/015.rs");
}
