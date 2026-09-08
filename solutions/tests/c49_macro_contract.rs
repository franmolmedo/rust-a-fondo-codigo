use course_solutions as renamed_course;

renamed_course::make_newtypes!(ExternalId(u64));

#[test]
fn exported_macros_work_from_a_consumer_with_a_dependency_alias() {
    assert_eq!(
        renamed_course::qualified!("macros"),
        "course-solutions::macros",
    );
    assert_eq!(renamed_course::sum_once!(10, 20, 12,), 42);

    assert_eq!(ExternalId(7).0, 7);
}
