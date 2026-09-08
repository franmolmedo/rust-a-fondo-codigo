use course_solutions::fundamentals::c09::{ServiceError, Severity, severity};

#[test]
fn c09_severe_http_statuses_are_limited_to_5xx() {
    for status in [500, 503, 599] {
        let error = ServiceError::Http {
            status,
            path: String::from("/users"),
        };
        assert_eq!(severity(&error), Severity::Severe);
    }

    for status in [0, 404, 499, 600, u16::MAX] {
        let error = ServiceError::Http {
            status,
            path: String::from("/users"),
        };
        assert_eq!(severity(&error), Severity::Normal);
    }

    assert_eq!(severity(&ServiceError::Timeout), Severity::Normal);
}
