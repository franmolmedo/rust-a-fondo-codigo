#[derive(Debug)]
enum ServiceError {
    Http { status: u16, path: String },
    Timeout,
}

#[derive(Debug, PartialEq)]
enum Severity {
    Severe,
    Normal,
}

fn severity(error: &ServiceError) -> Severity {
    match error {
        ServiceError::Http { status, .. } if (500..=599).contains(status) => Severity::Severe,
        ServiceError::Http { .. } | ServiceError::Timeout => Severity::Normal,
    }
}

fn main() {
    let error = ServiceError::Http {
        status: 503,
        path: String::from("/users"),
    };
    assert_eq!(severity(&error), Severity::Severe);
    let outside_5xx = ServiceError::Http {
        status: 600,
        path: String::from("/users"),
    };
    assert_eq!(severity(&outside_5xx), Severity::Normal);
}
