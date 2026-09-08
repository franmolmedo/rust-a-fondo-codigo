use course_solutions::unsafe_low_level::c47::{
    CallbackLifecycle, CallbackLifecycleError, CallbackPhase, FFI_CALL_PANIC, FFI_SUM_NULL_INPUT,
    FFI_SUM_OK, ffi_panic_firewall, sum_u32_export,
};

#[test]
fn c47_zero_active_callbacks_still_requires_provider_acknowledgement() {
    for has_callback in [false, true] {
        let mut lifecycle = CallbackLifecycle::registered();
        if has_callback {
            lifecycle.callback_started().unwrap();
        }
        lifecycle.begin_unregister().unwrap();
        if has_callback {
            lifecycle.callback_finished().unwrap();
        }
        assert_eq!(lifecycle.snapshot().phase, CallbackPhase::Unregistering);
        assert_eq!(
            lifecycle.release_context(),
            Err(CallbackLifecycleError::AcknowledgementPending)
        );
        lifecycle.acknowledge_unregister().unwrap();
        assert_eq!(lifecycle.snapshot().phase, CallbackPhase::Drained);
        lifecycle.release_context().unwrap();
        assert_eq!(
            lifecycle.acknowledge_unregister(),
            Err(CallbackLifecycleError::NotAwaitingAcknowledgement)
        );
    }
}

#[test]
fn c47_null_input_error_keeps_output_and_large_elements_sum_as_u64() {
    let mut output = 42_u64;
    // SAFETY: null input is an explicitly handled error; output is valid.
    assert_eq!(
        unsafe { sum_u32_export(std::ptr::null(), 1, &mut output) },
        FFI_SUM_NULL_INPUT
    );
    assert_eq!(output, 42);
    let values = [u32::MAX; 2];
    // SAFETY: live, aligned, disjoint regions, with no concurrent accesses.
    assert_eq!(
        unsafe { sum_u32_export(values.as_ptr(), values.len(), &mut output) },
        FFI_SUM_OK
    );
    assert_eq!(output, 2 * u64::from(u32::MAX));
}

#[test]
fn c47_firewall_does_not_drop_a_panicking_payload() {
    struct PanickingPayload;
    impl Drop for PanickingPayload {
        fn drop(&mut self) {
            panic!("payload destructor must not run");
        }
    }
    assert_eq!(
        ffi_panic_firewall(|| std::panic::panic_any(PanickingPayload)),
        FFI_CALL_PANIC
    );
}
