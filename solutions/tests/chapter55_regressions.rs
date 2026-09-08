use course_solutions::organization::c55::{
    Evidence, ReadingStep, ResearchQuestion, TraceEvent, TraceObservation, reading_plan,
    summarize_trace, valid_reading_plan,
};

#[test]
fn c55_source_level_allocations_are_not_runtime_measurements() {
    let observations = [
        Evidence::Signature,
        Evidence::Source,
        Evidence::Measurement,
        Evidence::Inference,
    ]
    .map(|evidence| TraceObservation {
        event: TraceEvent::Allocation,
        evidence,
    });
    let summary = summarize_trace(&observations);
    assert_eq!(summary.allocations, 1);
    assert_eq!(summary.inferred_cost_claims, 3);
}

#[test]
fn c55_reading_plans_allow_extra_checks_but_require_all_core_steps() {
    let question = ResearchQuestion::PublicApi;
    let mut extended = reading_plan(question);
    extended.insert(1, ReadingStep::ReadManifest);
    extended.push(ReadingStep::ReadHistory);
    assert!(valid_reading_plan(question, &extended));

    for omitted in 0..reading_plan(question).len() {
        let mut incomplete = reading_plan(question);
        incomplete.remove(omitted);
        assert!(!valid_reading_plan(question, &incomplete));
    }
    assert!(!valid_reading_plan(question, &[]));
    extended.insert(0, ReadingStep::ReadDocs);
    assert!(!valid_reading_plan(question, &extended));
}
