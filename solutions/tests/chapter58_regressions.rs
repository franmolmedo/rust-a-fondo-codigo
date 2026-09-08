use course_solutions::mastery::{
    m04_error_trace as errors, m05_lifecycle as lifecycle, m06_unsafe_audit as unsafe_audit,
};
use std::io;

#[test]
fn c58_retry_requires_a_transient_cause_and_an_idempotent_operation() {
    let error = |kind| {
        errors::ApplicationError::unavailable(
            errors::StorageError::new(io::Error::new(kind, "private detail")),
            7,
        )
    };
    for boundary in [
        errors::Boundary::Http,
        errors::Boundary::Cli,
        errors::Boundary::Ipc,
    ] {
        assert!(!errors::present(&error(io::ErrorKind::TimedOut), boundary).retryable);
        assert!(
            !errors::present(
                &error(io::ErrorKind::PermissionDenied)
                    .with_retry_safety(errors::RetrySafety::Idempotent),
                boundary,
            )
            .retryable
        );
        assert!(
            errors::present(
                &error(io::ErrorKind::TimedOut).with_retry_safety(errors::RetrySafety::Idempotent),
                boundary,
            )
            .retryable
        );
    }
}

#[test]
fn c58_normal_completion_needs_no_deadline_or_cancellation() {
    use lifecycle::{Child, Event};
    assert_eq!(
        lifecycle::audit(&[
            Event::AdmitRequest,
            Event::Spawn(Child::A),
            Event::Spawn(Child::B),
            Event::Join(Child::B),
            Event::Join(Child::A),
            Event::Report,
        ]),
        Ok(())
    );
}

#[test]
fn c58_empty_slice_needs_alignment_but_not_an_allocation() {
    let premises = unsafe_audit::RawSlicePremises {
        empty: true,
        non_null_and_aligned_even_if_empty: true,
        owner_alive: false,
        initialized_for_len: false,
        one_allocation: false,
        byte_size_and_address_range_valid: false,
        aliasing_allows_shared_access: false,
        lifetime_tied_to_owner: false,
    };
    assert_eq!(unsafe_audit::audit(premises), Ok(()));
    assert_eq!(
        unsafe_audit::audit(unsafe_audit::RawSlicePremises {
            empty: false,
            ..premises
        }),
        Err(unsafe_audit::MissingPremise::Owner)
    );
    assert_eq!(
        unsafe_audit::audit(unsafe_audit::RawSlicePremises {
            non_null_and_aligned_even_if_empty: false,
            ..premises
        }),
        Err(unsafe_audit::MissingPremise::Pointer)
    );

    // SAFETY: dangling() supplies a non-null aligned u64 pointer. With zero
    // elements the byte range is empty, no allocation is accessed and no
    // initialization, aliasing, or allocation lifetime obligation remains.
    let empty =
        unsafe { std::slice::from_raw_parts(std::ptr::NonNull::<u64>::dangling().as_ptr(), 0) };
    assert!(empty.is_empty());
}
