use course_solutions::compiler::c52::{BuildPhase, TimingSample, dominant_clean_timing};

#[test]
fn c52_large_durations_keep_their_order_without_overflow_or_saturation() {
    let sample = |name: &str, codegen_ms| TimingSample {
        crate_name: name.to_owned(),
        fresh: false,
        front_end_ms: u64::MAX,
        codegen_ms,
        build_script_ms: u64::MAX,
    };
    let result =
        dominant_clean_timing(&[sample("largest", u64::MAX), sample("smaller", u64::MAX - 1)])
            .unwrap();
    assert_eq!(result.crate_name, "largest");
    assert_eq!(result.total_ms, 3 * u128::from(u64::MAX));
    assert_eq!(result.dominant_phase, BuildPhase::BuildScript);
}
