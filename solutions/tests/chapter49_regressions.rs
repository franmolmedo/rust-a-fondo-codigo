use course_solutions as renamed_course;

#[test]
fn c49_dsl_uses_the_standard_macro_despite_consumer_shadowing() {
    macro_rules! stringify {
        ($($tokens:tt)*) => {
            "shadowed"
        };
    }
    assert_eq!(stringify!(load), "shadowed");
    let first = String::from("first");
    let second = String::from("second");
    let output = renamed_course::command_values!(load => first; save => second;);
    assert_eq!(
        output,
        [
            ("load", String::from("first")),
            ("save", String::from("second"))
        ]
    );
}

#[test]
fn c49_sum_preserves_order_and_classifier_does_not_evaluate() {
    let mut order = Vec::new();
    let result = renamed_course::sum_once!(
        {
            order.push(1);
            10
        },
        {
            order.push(2);
            20
        },
    );
    assert_eq!((result, order), (30, vec![1, 2]));
    assert_eq!(
        renamed_course::classify_edition_expression!(panic!("not evaluated")),
        renamed_course::organization::c49::EditionExpressionKind::LegacyExpression
    );
}
