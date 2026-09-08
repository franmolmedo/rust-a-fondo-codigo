use course_solutions::async_rust::c42::{Envelope, captured_pair};

fn unrelated_lifetimes<'a, 'b>(
    anchor: &'a (),
    value: &'b str,
) -> impl std::fmt::Debug + use<'a, 'b> {
    captured_pair(anchor, value)
}

#[test]
fn c42_capturing_two_lifetimes_does_not_require_one_to_outlive_the_other() {
    let anchor = ();
    let text = String::from("short");
    assert!(format!("{:?}", unrelated_lifetimes(&anchor, &text)).contains("short"));
}

#[test]
fn c42_bundle_does_not_borrow_the_envelope() {
    let bundle = {
        let envelope = Envelope::<String, 3>::default();
        envelope.bundle("payload")
    };
    assert!(format!("{bundle:?}").contains("payload"));
}
