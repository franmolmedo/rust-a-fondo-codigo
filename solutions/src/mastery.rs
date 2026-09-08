//! Executable oracles for the eight mastery assessments in chapter 58.
//!
//! The code checks minimum criteria and counterexamples. Explanations and the
//! defense of design decisions still belong to the person being assessed.

pub mod m01_ownership {
    // SOLUTION: C58-M01
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CalleeUse {
        Observe,
        TakeOwnership,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OwnershipFacts {
        pub source_used_after_call: bool,
        pub callee_use: CalleeUse,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum OwnershipPlan {
        Move,
        Borrow,
        DeepClone,
    }

    /// Compares a fixed consuming or observing API for an independent `String`.
    /// Other types may share state when cloned, or return ownership to the caller.
    pub fn weakest_plan(facts: OwnershipFacts) -> OwnershipPlan {
        match (facts.callee_use, facts.source_used_after_call) {
            (CalleeUse::Observe, _) => OwnershipPlan::Borrow,
            (CalleeUse::TakeOwnership, false) => OwnershipPlan::Move,
            (CalleeUse::TakeOwnership, true) => OwnershipPlan::DeepClone,
        }
    }

    pub fn consume_length(value: String) -> usize {
        value.len()
    }

    pub fn observe_length(value: &str) -> usize {
        value.len()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn move_replaces_clone_when_the_source_has_no_future_owner() {
            let facts = OwnershipFacts {
                source_used_after_call: false,
                callee_use: CalleeUse::TakeOwnership,
            };
            assert_eq!(weakest_plan(facts), OwnershipPlan::Move);
            assert_eq!(consume_length(String::from("rust")), 4);
        }

        #[test]
        fn observation_needs_only_a_borrow_and_preserves_the_owner() {
            let value = String::from("ferris");
            assert_eq!(
                weakest_plan(OwnershipFacts {
                    source_used_after_call: true,
                    callee_use: CalleeUse::Observe,
                }),
                OwnershipPlan::Borrow
            );
            assert_eq!(observe_length(&value), 6);
            assert_eq!(value, "ferris");
        }

        #[test]
        fn independent_mutation_is_a_contract_for_a_deep_copy() {
            assert_eq!(
                weakest_plan(OwnershipFacts {
                    source_used_after_call: true,
                    callee_use: CalleeUse::TakeOwnership,
                }),
                OwnershipPlan::DeepClone
            );

            let source = String::from("draft");
            let mut independent = source.clone();
            independent.push_str("-published");
            assert_eq!(source, "draft");
            assert_eq!(independent, "draft-published");
        }

        #[test]
        fn ownership_can_move_when_only_the_callee_needs_to_mutate() {
            assert_eq!(
                weakest_plan(OwnershipFacts {
                    source_used_after_call: false,
                    callee_use: CalleeUse::TakeOwnership,
                }),
                OwnershipPlan::Move
            );
        }
    }
}

pub mod m02_domain_types {
    use std::num::NonZeroU64;

    // SOLUTION: C58-M02
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct LegacyRequest {
        pub document_id: String,
        pub publish: bool,
        pub dry_run: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DocumentId(NonZeroU64);

    impl DocumentId {
        pub fn get(self) -> u64 {
            self.0.get()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Action {
        SaveDraft,
        ValidateOnly,
        Publish,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Command {
        document_id: DocumentId,
        action: Action,
    }

    impl Command {
        pub fn document_id(self) -> DocumentId {
            self.document_id
        }

        pub fn action(self) -> Action {
            self.action
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CommandError {
        InvalidDocumentId,
        ContradictoryModes,
    }

    impl TryFrom<LegacyRequest> for Command {
        type Error = CommandError;

        fn try_from(request: LegacyRequest) -> Result<Self, Self::Error> {
            let raw_id = request
                .document_id
                .parse::<u64>()
                .map_err(|_| CommandError::InvalidDocumentId)?;
            let document_id = NonZeroU64::new(raw_id)
                .map(DocumentId)
                .ok_or(CommandError::InvalidDocumentId)?;
            let action = match (request.publish, request.dry_run) {
                (false, false) => Action::SaveDraft,
                (false, true) => Action::ValidateOnly,
                (true, false) => Action::Publish,
                (true, true) => return Err(CommandError::ContradictoryModes),
            };
            Ok(Self {
                document_id,
                action,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn request(id: &str, publish: bool, dry_run: bool) -> LegacyRequest {
            LegacyRequest {
                document_id: id.into(),
                publish,
                dry_run,
            }
        }

        #[test]
        fn enum_replaces_three_meaningful_boolean_states() {
            assert_eq!(
                Command::try_from(request("7", false, false))
                    .unwrap()
                    .action(),
                Action::SaveDraft
            );
            assert_eq!(
                Command::try_from(request("7", false, true))
                    .unwrap()
                    .action(),
                Action::ValidateOnly
            );
            assert_eq!(
                Command::try_from(request("7", true, false))
                    .unwrap()
                    .action(),
                Action::Publish
            );
        }

        #[test]
        fn contradictory_modes_never_enter_the_domain() {
            assert_eq!(
                Command::try_from(request("7", true, true)),
                Err(CommandError::ContradictoryModes)
            );
        }

        #[test]
        fn identifier_newtype_rejects_syntax_and_zero_at_the_boundary() {
            assert_eq!(
                Command::try_from(request("zero", false, false)),
                Err(CommandError::InvalidDocumentId)
            );
            assert_eq!(
                Command::try_from(request("0", false, false)),
                Err(CommandError::InvalidDocumentId)
            );
            assert_eq!(
                Command::try_from(request("9", false, false))
                    .unwrap()
                    .document_id()
                    .get(),
                9
            );
        }
    }
}

pub mod m03_dispatch {
    // SOLUTION: C58-M03
    pub trait Transform {
        fn apply(&self, input: &str) -> String;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Prefix(pub String);

    impl Transform for Prefix {
        fn apply(&self, input: &str) -> String {
            format!("{}{input}", self.0)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Uppercase;

    impl Transform for Uppercase {
        fn apply(&self, input: &str) -> String {
            input.to_uppercase()
        }
    }

    pub fn apply_static<T: Transform>(transform: &T, input: &str) -> String {
        transform.apply(input)
    }

    pub fn apply_dynamic(transform: &dyn Transform, input: &str) -> String {
        transform.apply(input)
    }

    pub fn configured_transform(use_prefix: bool) -> Box<dyn Transform> {
        if use_prefix {
            Box::new(Prefix("id:".into()))
        } else {
            Box::new(Uppercase)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn static_and_dynamic_dispatch_preserve_shared_semantics() {
            let transform = Prefix("doc:".into());
            assert_eq!(apply_static(&transform, "7"), "doc:7");
            assert_eq!(apply_dynamic(&transform, "7"), "doc:7");
        }

        #[test]
        fn runtime_configuration_can_select_a_boxed_implementation() {
            assert_eq!(configured_transform(true).apply("8"), "id:8");
            assert_eq!(configured_transform(false).apply("rust"), "RUST");
        }

        #[test]
        fn generic_caller_selects_a_concrete_type_without_boxing_it() {
            let transform = Uppercase;
            assert_eq!(apply_static(&transform, "ferris"), "FERRIS");
            assert_eq!(std::mem::size_of_val(&transform), 0);
        }
    }
}

pub mod m04_error_trace {
    use std::error::Error;
    use std::fmt;
    use std::io;

    // SOLUTION: C58-M04
    #[derive(Debug)]
    pub struct StorageError {
        source: io::Error,
    }

    impl StorageError {
        pub fn new(source: io::Error) -> Self {
            Self { source }
        }
    }

    impl fmt::Display for StorageError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("storage unavailable")
        }
    }

    impl Error for StorageError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.source)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RetrySafety {
        Unknown,
        Idempotent,
    }

    #[derive(Debug)]
    pub struct ApplicationError {
        source: StorageError,
        correlation_id: u64,
        retry_safety: RetrySafety,
    }

    impl ApplicationError {
        pub fn unavailable(source: StorageError, correlation_id: u64) -> Self {
            Self {
                source,
                correlation_id,
                retry_safety: RetrySafety::Unknown,
            }
        }

        /// Declares a protocol property; it does not prove idempotency.
        pub fn with_retry_safety(mut self, retry_safety: RetrySafety) -> Self {
            self.retry_safety = retry_safety;
            self
        }
    }

    impl fmt::Display for ApplicationError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("operation unavailable")
        }
    }

    impl Error for ApplicationError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.source)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Boundary {
        Http,
        Cli,
        Ipc,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PublicError {
        pub protocol_code: u16,
        pub stable_code: &'static str,
        pub correlation_id: u64,
        pub retryable: bool,
    }

    pub fn present(error: &ApplicationError, boundary: Boundary) -> PublicError {
        let protocol_code = match boundary {
            Boundary::Http => 503,
            Boundary::Cli => 75,
            Boundary::Ipc => 1,
        };
        PublicError {
            protocol_code,
            stable_code: "service.unavailable",
            correlation_id: error.correlation_id,
            retryable: error.retry_safety == RetrySafety::Idempotent
                && matches!(
                    error.source.source.kind(),
                    io::ErrorKind::Interrupted
                        | io::ErrorKind::WouldBlock
                        | io::ErrorKind::TimedOut
                ),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn failure() -> ApplicationError {
            ApplicationError::unavailable(
                StorageError::new(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "C:\\private\\ledger.db",
                )),
                42,
            )
        }

        #[test]
        fn internal_chain_preserves_the_syscall_class_and_cause() {
            let error = failure();
            let storage = error.source().unwrap();
            let io = storage
                .source()
                .unwrap()
                .downcast_ref::<io::Error>()
                .unwrap();
            assert_eq!(io.kind(), io::ErrorKind::PermissionDenied);
        }

        #[test]
        fn public_presentations_keep_stable_meaning_and_correlation() {
            let error = failure();
            assert_eq!(present(&error, Boundary::Http).protocol_code, 503);
            assert_eq!(present(&error, Boundary::Cli).protocol_code, 75);
            assert_eq!(
                present(&error, Boundary::Ipc).stable_code,
                "service.unavailable"
            );
            assert_eq!(present(&error, Boundary::Http).correlation_id, 42);
        }

        #[test]
        fn private_path_is_not_part_of_any_public_error() {
            let public = format!("{:?}", present(&failure(), Boundary::Http));
            assert!(!public.contains("private"));
            assert!(!public.contains("ledger.db"));
        }
    }
}

pub mod m05_lifecycle {
    use std::collections::HashSet;

    // SOLUTION: C58-M05
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Child {
        A,
        B,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Event {
        AdmitRequest,
        Spawn(Child),
        Deadline,
        Cancel(Child),
        Join(Child),
        Report,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LifecycleError {
        NotAdmittedFirst,
        DuplicateAdmission,
        DuplicateSpawn,
        MissingChild,
        SpawnAfterDeadline,
        DuplicateDeadline,
        CancellationBeforeDeadline,
        CancelBeforeSpawn,
        CancelAfterJoin,
        DuplicateCancel,
        JoinBeforeSpawn,
        DuplicateJoin,
        OrphanedChild,
        ReportNotLast,
        DuplicateReport,
    }

    /// Checks this exercise's event protocol; it does not spawn or await tasks.
    pub fn audit(events: &[Event]) -> Result<(), LifecycleError> {
        if events.first() != Some(&Event::AdmitRequest) {
            return Err(LifecycleError::NotAdmittedFirst);
        }
        if events.last() != Some(&Event::Report) {
            return Err(LifecycleError::ReportNotLast);
        }

        let mut spawned = HashSet::new();
        let mut cancelled = HashSet::new();
        let mut joined = HashSet::new();
        let mut deadline_seen = false;
        let mut report_seen = false;
        let mut admission_seen = false;
        for event in events {
            match *event {
                Event::AdmitRequest => {
                    if admission_seen {
                        return Err(LifecycleError::DuplicateAdmission);
                    }
                    admission_seen = true;
                }
                Event::Report => {
                    if report_seen {
                        return Err(LifecycleError::DuplicateReport);
                    }
                    report_seen = true;
                }
                Event::Spawn(child) => {
                    if deadline_seen {
                        return Err(LifecycleError::SpawnAfterDeadline);
                    }
                    if !spawned.insert(child) {
                        return Err(LifecycleError::DuplicateSpawn);
                    }
                }
                Event::Deadline => {
                    if deadline_seen {
                        return Err(LifecycleError::DuplicateDeadline);
                    }
                    deadline_seen = true;
                }
                Event::Cancel(child) => {
                    if !deadline_seen {
                        return Err(LifecycleError::CancellationBeforeDeadline);
                    }
                    if !spawned.contains(&child) {
                        return Err(LifecycleError::CancelBeforeSpawn);
                    }
                    if joined.contains(&child) {
                        return Err(LifecycleError::CancelAfterJoin);
                    }
                    if !cancelled.insert(child) {
                        return Err(LifecycleError::DuplicateCancel);
                    }
                }
                Event::Join(child) => {
                    if !spawned.contains(&child) {
                        return Err(LifecycleError::JoinBeforeSpawn);
                    }
                    if !joined.insert(child) {
                        return Err(LifecycleError::DuplicateJoin);
                    }
                }
            }
        }
        if spawned.len() != 2 {
            return Err(LifecycleError::MissingChild);
        }
        if joined != spawned {
            return Err(LifecycleError::OrphanedChild);
        }
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn timeout_cancels_and_joins_both_owned_children() {
            assert_eq!(
                audit(&[
                    Event::AdmitRequest,
                    Event::Spawn(Child::A),
                    Event::Spawn(Child::B),
                    Event::Deadline,
                    Event::Cancel(Child::A),
                    Event::Cancel(Child::B),
                    Event::Join(Child::A),
                    Event::Join(Child::B),
                    Event::Report,
                ]),
                Ok(())
            );
        }

        #[test]
        fn a_missing_join_exposes_an_orphan() {
            assert_eq!(
                audit(&[
                    Event::AdmitRequest,
                    Event::Spawn(Child::A),
                    Event::Spawn(Child::B),
                    Event::Deadline,
                    Event::Cancel(Child::A),
                    Event::Cancel(Child::B),
                    Event::Join(Child::A),
                    Event::Report,
                ]),
                Err(LifecycleError::OrphanedChild)
            );
        }

        #[test]
        fn cancellation_before_the_deadline_is_rejected() {
            assert_eq!(
                audit(&[
                    Event::AdmitRequest,
                    Event::Spawn(Child::A),
                    Event::Cancel(Child::A),
                    Event::Join(Child::A),
                    Event::Report,
                ]),
                Err(LifecycleError::CancellationBeforeDeadline)
            );
        }

        #[test]
        fn duplicate_join_does_not_hide_a_broken_trace() {
            assert_eq!(
                audit(&[
                    Event::AdmitRequest,
                    Event::Spawn(Child::A),
                    Event::Join(Child::A),
                    Event::Join(Child::A),
                    Event::Report,
                ]),
                Err(LifecycleError::DuplicateJoin)
            );
        }

        #[test]
        fn an_intermediate_report_is_not_accepted() {
            assert_eq!(
                audit(&[
                    Event::AdmitRequest,
                    Event::Spawn(Child::A),
                    Event::Join(Child::A),
                    Event::Report,
                    Event::Report,
                ]),
                Err(LifecycleError::DuplicateReport)
            );
        }

        #[test]
        fn work_cannot_start_after_the_deadline() {
            assert_eq!(
                audit(&[
                    Event::AdmitRequest,
                    Event::Deadline,
                    Event::Spawn(Child::A),
                    Event::Report,
                ]),
                Err(LifecycleError::SpawnAfterDeadline)
            );
        }

        #[test]
        fn the_trace_requires_both_declared_children() {
            assert_eq!(
                audit(&[
                    Event::AdmitRequest,
                    Event::Spawn(Child::A),
                    Event::Join(Child::A),
                    Event::Report,
                ]),
                Err(LifecycleError::MissingChild)
            );
        }

        #[test]
        fn a_request_is_admitted_exactly_once() {
            assert_eq!(
                audit(&[Event::AdmitRequest, Event::AdmitRequest, Event::Report,]),
                Err(LifecycleError::DuplicateAdmission)
            );
        }
    }
}

pub mod m06_unsafe_audit {
    // SOLUTION: C58-M06
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RawSlicePremises {
        pub empty: bool,
        pub owner_alive: bool,
        pub non_null_and_aligned_even_if_empty: bool,
        pub initialized_for_len: bool,
        pub one_allocation: bool,
        pub byte_size_and_address_range_valid: bool,
        pub aliasing_allows_shared_access: bool,
        pub lifetime_tied_to_owner: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MissingPremise {
        Owner,
        Pointer,
        Initialization,
        Allocation,
        SizeAndRange,
        Aliasing,
        Lifetime,
    }

    pub fn audit(premises: RawSlicePremises) -> Result<(), MissingPremise> {
        // An empty slice needs no allocation or initialized elements, but its
        // pointer must still be non-null and aligned. These are declarations,
        // not measurements or proofs about a real pointer.
        if premises.empty {
            return if premises.non_null_and_aligned_even_if_empty {
                Ok(())
            } else {
                Err(MissingPremise::Pointer)
            };
        }
        let checks = [
            (premises.owner_alive, MissingPremise::Owner),
            (
                premises.non_null_and_aligned_even_if_empty,
                MissingPremise::Pointer,
            ),
            (premises.initialized_for_len, MissingPremise::Initialization),
            (premises.one_allocation, MissingPremise::Allocation),
            (
                premises.byte_size_and_address_range_valid,
                MissingPremise::SizeAndRange,
            ),
            (
                premises.aliasing_allows_shared_access,
                MissingPremise::Aliasing,
            ),
            (premises.lifetime_tied_to_owner, MissingPremise::Lifetime),
        ];
        checks
            .into_iter()
            .find_map(|(satisfied, missing)| (!satisfied).then_some(Err(missing)))
            .unwrap_or(Ok(()))
    }

    pub fn complete_contract() -> RawSlicePremises {
        RawSlicePremises {
            empty: false,
            owner_alive: true,
            non_null_and_aligned_even_if_empty: true,
            initialized_for_len: true,
            one_allocation: true,
            byte_size_and_address_range_valid: true,
            aliasing_allows_shared_access: true,
            lifetime_tied_to_owner: true,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn complete_wrapper_contract_can_be_approved() {
            assert_eq!(audit(complete_contract()), Ok(()));
        }

        #[test]
        fn arbitrary_raw_pointer_api_fails_at_the_first_unproved_premise() {
            let mut premises = complete_contract();
            premises.owner_alive = false;
            premises.lifetime_tied_to_owner = false;
            assert_eq!(audit(premises), Err(MissingPremise::Owner));
        }

        #[test]
        fn zero_length_does_not_waive_non_null_alignment_contract() {
            let mut premises = complete_contract();
            premises.empty = true;
            premises.non_null_and_aligned_even_if_empty = false;
            assert_eq!(audit(premises), Err(MissingPremise::Pointer));
        }

        #[test]
        fn every_individual_missing_premise_is_reported() {
            type BreakPremise = fn(&mut RawSlicePremises);
            let cases: [(BreakPremise, MissingPremise); 7] = [
                (|value| value.owner_alive = false, MissingPremise::Owner),
                (
                    |value| value.non_null_and_aligned_even_if_empty = false,
                    MissingPremise::Pointer,
                ),
                (
                    |value| value.initialized_for_len = false,
                    MissingPremise::Initialization,
                ),
                (
                    |value| value.one_allocation = false,
                    MissingPremise::Allocation,
                ),
                (
                    |value| value.byte_size_and_address_range_valid = false,
                    MissingPremise::SizeAndRange,
                ),
                (
                    |value| value.aliasing_allows_shared_access = false,
                    MissingPremise::Aliasing,
                ),
                (
                    |value| value.lifetime_tied_to_owner = false,
                    MissingPremise::Lifetime,
                ),
            ];

            for (break_premise, expected) in cases {
                let mut premises = complete_contract();
                break_premise(&mut premises);
                assert_eq!(audit(premises), Err(expected));
            }
        }
    }
}

pub mod m07_api_review {
    use std::collections::HashSet;

    // SOLUTION: C58-M07
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum AuditField {
        States,
        Ownership,
        FailureAndCancellation,
        BlockingAndWaiting,
        Bounds,
        Cost,
        Semver,
        FeatureAndTarget,
        UnsafePremises,
        Tests,
    }

    pub const ALL_FIELDS: [AuditField; 10] = [
        AuditField::States,
        AuditField::Ownership,
        AuditField::FailureAndCancellation,
        AuditField::BlockingAndWaiting,
        AuditField::Bounds,
        AuditField::Cost,
        AuditField::Semver,
        AuditField::FeatureAndTarget,
        AuditField::UnsafePremises,
        AuditField::Tests,
    ];

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EvidenceDetail {
        Claim(&'static str),
        NotApplicable { reason: &'static str },
    }

    impl EvidenceDetail {
        fn is_substantive(self) -> bool {
            match self {
                Self::Claim(detail) => !detail.trim().is_empty(),
                Self::NotApplicable { reason } => !reason.trim().is_empty(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Evidence {
        pub field: AuditField,
        pub detail: EvidenceDetail,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ApiAudit {
        pub missing: Vec<AuditField>,
        pub duplicated: Vec<AuditField>,
        pub unsubstantiated: Vec<AuditField>,
    }

    impl ApiAudit {
        pub fn passed(&self) -> bool {
            self.missing.is_empty() && self.duplicated.is_empty() && self.unsubstantiated.is_empty()
        }
    }

    pub fn review(evidence: &[Evidence]) -> ApiAudit {
        let mut seen = HashSet::new();
        let mut duplicated = Vec::new();
        let mut unsubstantiated = Vec::new();
        for item in evidence {
            if !seen.insert(item.field) {
                duplicated.push(item.field);
            }
            if !item.detail.is_substantive() {
                unsubstantiated.push(item.field);
            }
        }
        let missing = ALL_FIELDS
            .into_iter()
            .filter(|field| !seen.contains(field))
            .collect();
        ApiAudit {
            missing,
            duplicated,
            unsubstantiated,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn complete_evidence() -> Vec<Evidence> {
            ALL_FIELDS
                .into_iter()
                .map(|field| Evidence {
                    field,
                    detail: EvidenceDetail::Claim("linked evidence"),
                })
                .collect()
        }

        #[test]
        fn ten_distinct_non_empty_answers_pass_the_gate() {
            assert!(review(&complete_evidence()).passed());
        }

        #[test]
        fn not_applicable_without_a_justification_is_empty_evidence() {
            let mut evidence = complete_evidence();
            evidence[7].detail = EvidenceDetail::NotApplicable { reason: "" };
            assert_eq!(
                review(&evidence).unsubstantiated,
                [AuditField::FeatureAndTarget]
            );
        }

        #[test]
        fn not_applicable_with_a_reason_is_substantive_evidence() {
            let mut evidence = complete_evidence();
            evidence[7].detail = EvidenceDetail::NotApplicable {
                reason: "the crate has no optional features",
            };
            assert!(review(&evidence).passed());
        }

        #[test]
        fn an_empty_claim_is_unsubstantiated() {
            let mut evidence = complete_evidence();
            evidence[9].detail = EvidenceDetail::Claim("   ");
            assert_eq!(review(&evidence).unsubstantiated, [AuditField::Tests]);
        }

        #[test]
        fn duplicated_answer_does_not_cover_a_missing_dimension() {
            let mut evidence = complete_evidence();
            evidence[9].field = AuditField::States;
            let audit = review(&evidence);
            assert_eq!(audit.duplicated, [AuditField::States]);
            assert_eq!(audit.missing, [AuditField::Tests]);
            assert!(!audit.passed());
        }
    }
}

pub mod m08_consolidation {
    use std::collections::HashSet;

    // SOLUTION: C58-M08
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Architecture {
        pub frontiers: u8,
        pub domain_independent: bool,
        pub persistence_port: bool,
        pub bounded_concurrency: bool,
        pub cancellation_and_shutdown: bool,
        pub translated_errors: bool,
        pub documented_and_tested_api: bool,
        pub ffi_enabled: bool,
        pub ffi_safety_contract: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Obligation {
        TwoFrontiers,
        IndependentDomain,
        PersistencePort,
        BoundedConcurrency,
        CancellationAndShutdown,
        TranslatedErrors,
        DocumentedAndTestedApi,
        FfiSafety,
    }

    pub fn missing_obligations(architecture: Architecture) -> Vec<Obligation> {
        let mut missing = Vec::new();
        if architecture.frontiers < 2 {
            missing.push(Obligation::TwoFrontiers);
        }
        if !architecture.domain_independent {
            missing.push(Obligation::IndependentDomain);
        }
        if !architecture.persistence_port {
            missing.push(Obligation::PersistencePort);
        }
        if !architecture.bounded_concurrency {
            missing.push(Obligation::BoundedConcurrency);
        }
        if !architecture.cancellation_and_shutdown {
            missing.push(Obligation::CancellationAndShutdown);
        }
        if !architecture.translated_errors {
            missing.push(Obligation::TranslatedErrors);
        }
        if !architecture.documented_and_tested_api {
            missing.push(Obligation::DocumentedAndTestedApi);
        }
        if architecture.ffi_enabled && !architecture.ffi_safety_contract {
            missing.push(Obligation::FfiSafety);
        }
        missing
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Decision {
        pub context: &'static str,
        pub choice: &'static str,
        pub alternative: &'static str,
        pub consequence: &'static str,
        pub revisit_when: &'static str,
    }

    /// Checks record shape, not the validity of the decisions or alternatives.
    pub fn decision_records_complete(decisions: &[Decision]) -> bool {
        let mut contexts = HashSet::new();
        decisions.len() >= 5
            && decisions.iter().all(|decision| {
                !decision.context.trim().is_empty()
                    && contexts.insert(decision.context.trim())
                    && !decision.choice.trim().is_empty()
                    && !decision.alternative.trim().is_empty()
                    && decision.choice.trim() != decision.alternative.trim()
                    && !decision.consequence.trim().is_empty()
                    && !decision.revisit_when.trim().is_empty()
            })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn complete() -> Architecture {
            Architecture {
                frontiers: 2,
                domain_independent: true,
                persistence_port: true,
                bounded_concurrency: true,
                cancellation_and_shutdown: true,
                translated_errors: true,
                documented_and_tested_api: true,
                ffi_enabled: false,
                ffi_safety_contract: false,
            }
        }

        #[test]
        fn complete_small_architecture_passes_without_optional_ffi() {
            assert!(missing_obligations(complete()).is_empty());
        }

        #[test]
        fn optional_ffi_becomes_mandatory_to_justify_when_enabled() {
            let mut architecture = complete();
            architecture.ffi_enabled = true;
            assert_eq!(missing_obligations(architecture), [Obligation::FfiSafety]);
            architecture.ffi_safety_contract = true;
            assert!(missing_obligations(architecture).is_empty());
        }

        #[test]
        fn five_distinct_decisions_need_complete_reasoning() {
            let admission = Decision {
                context: "admission",
                choice: "bounded channel",
                alternative: "unbounded channel",
                consequence: "admission can wait",
                revisit_when: "measured burst changes",
            };
            let decisions = [
                admission,
                Decision {
                    context: "persistence",
                    choice: "in-memory repository",
                    alternative: "database adapter",
                    consequence: "data does not survive process shutdown",
                    revisit_when: "durability becomes a requirement",
                },
                Decision {
                    context: "http boundary",
                    choice: "typed public errors",
                    alternative: "expose internal error text",
                    consequence: "transport maintains an explicit error mapping",
                    revisit_when: "the public error contract changes",
                },
                Decision {
                    context: "task ownership",
                    choice: "supervised task set",
                    alternative: "detached tasks",
                    consequence: "the supervisor must collect every result",
                    revisit_when: "work moves into another process",
                },
                Decision {
                    context: "shutdown",
                    choice: "drain accepted work",
                    alternative: "cancel all accepted work immediately",
                    consequence: "shutdown waits for accepted operations",
                    revisit_when: "a hard shutdown deadline is required",
                },
            ];
            assert!(decision_records_complete(&decisions));

            assert!(!decision_records_complete(&[admission; 5]));
            let incomplete = Decision {
                alternative: "",
                ..decisions[4]
            };
            let mut incomplete_decisions = decisions;
            incomplete_decisions[4] = incomplete;
            assert!(!decision_records_complete(&incomplete_decisions));

            let mut identical_alternative = decisions;
            identical_alternative[0].alternative = " bounded channel ";
            assert!(!decision_records_complete(&identical_alternative));
        }
    }
}
