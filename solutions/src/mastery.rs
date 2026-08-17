//! Oráculos ejecutables para las ocho pruebas de maestría del capítulo 58.
//!
//! El código comprueba criterios mínimos y contraejemplos. La explicación y la
//! defensa de decisiones siguen perteneciendo a la persona evaluada.

pub mod m01_ownership {
    // SOLUTION: C58-M01
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OwnershipFacts {
        pub source_used_after_call: bool,
        pub callee_only_observes: bool,
        pub callee_needs_independent_mutation: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum OwnershipPlan {
        Move,
        Borrow,
        DeepClone,
    }

    pub fn weakest_plan(facts: OwnershipFacts) -> OwnershipPlan {
        if facts.callee_only_observes {
            OwnershipPlan::Borrow
        } else if facts.source_used_after_call || facts.callee_needs_independent_mutation {
            OwnershipPlan::DeepClone
        } else {
            OwnershipPlan::Move
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
                callee_only_observes: false,
                callee_needs_independent_mutation: false,
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
                    callee_only_observes: true,
                    callee_needs_independent_mutation: false,
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
                    callee_only_observes: false,
                    callee_needs_independent_mutation: true,
                }),
                OwnershipPlan::DeepClone
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
        pub document_id: DocumentId,
        pub action: Action,
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
                    .action,
                Action::SaveDraft
            );
            assert_eq!(
                Command::try_from(request("7", false, true)).unwrap().action,
                Action::ValidateOnly
            );
            assert_eq!(
                Command::try_from(request("7", true, false)).unwrap().action,
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
                    .document_id
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

    #[derive(Debug)]
    pub struct ApplicationError {
        source: StorageError,
        correlation_id: u64,
    }

    impl ApplicationError {
        pub fn unavailable(source: StorageError, correlation_id: u64) -> Self {
            Self {
                source,
                correlation_id,
            }
        }
    }

    impl fmt::Display for ApplicationError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("operation temporarily unavailable")
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
            retryable: true,
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
        DuplicateSpawn,
        MoreThanTwoChildren,
        CancellationBeforeDeadline,
        JoinBeforeSpawn,
        OrphanedChild,
        ReportNotLast,
    }

    pub fn audit(events: &[Event]) -> Result<(), LifecycleError> {
        if events.first() != Some(&Event::AdmitRequest) {
            return Err(LifecycleError::NotAdmittedFirst);
        }
        if events.last() != Some(&Event::Report) {
            return Err(LifecycleError::ReportNotLast);
        }

        let mut spawned = HashSet::new();
        let mut joined = HashSet::new();
        let mut deadline_seen = false;
        for event in events {
            match *event {
                Event::AdmitRequest | Event::Report => {}
                Event::Spawn(child) => {
                    if !spawned.insert(child) {
                        return Err(LifecycleError::DuplicateSpawn);
                    }
                    if spawned.len() > 2 {
                        return Err(LifecycleError::MoreThanTwoChildren);
                    }
                }
                Event::Deadline => deadline_seen = true,
                Event::Cancel(child) => {
                    if !deadline_seen {
                        return Err(LifecycleError::CancellationBeforeDeadline);
                    }
                    if !spawned.contains(&child) {
                        return Err(LifecycleError::JoinBeforeSpawn);
                    }
                }
                Event::Join(child) => {
                    if !spawned.contains(&child) {
                        return Err(LifecycleError::JoinBeforeSpawn);
                    }
                    joined.insert(child);
                }
            }
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
    }
}

pub mod m06_unsafe_audit {
    // SOLUTION: C58-M06
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RawSlicePremises {
        pub owner_alive: bool,
        pub non_null_and_aligned_even_if_empty: bool,
        pub initialized_for_len: bool,
        pub one_allocation: bool,
        pub byte_size_fits_isize: bool,
        pub aliasing_allows_shared_access: bool,
        pub lifetime_tied_to_owner: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MissingPremise {
        Owner,
        Pointer,
        Initialization,
        Allocation,
        Size,
        Aliasing,
        Lifetime,
    }

    pub fn audit(premises: RawSlicePremises) -> Result<(), MissingPremise> {
        let checks = [
            (premises.owner_alive, MissingPremise::Owner),
            (
                premises.non_null_and_aligned_even_if_empty,
                MissingPremise::Pointer,
            ),
            (premises.initialized_for_len, MissingPremise::Initialization),
            (premises.one_allocation, MissingPremise::Allocation),
            (premises.byte_size_fits_isize, MissingPremise::Size),
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
            owner_alive: true,
            non_null_and_aligned_even_if_empty: true,
            initialized_for_len: true,
            one_allocation: true,
            byte_size_fits_isize: true,
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
            premises.non_null_and_aligned_even_if_empty = false;
            assert_eq!(audit(premises), Err(MissingPremise::Pointer));
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
    pub struct Evidence {
        pub field: AuditField,
        pub detail: &'static str,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ApiAudit {
        pub missing: Vec<AuditField>,
        pub duplicated: Vec<AuditField>,
        pub empty: Vec<AuditField>,
    }

    impl ApiAudit {
        pub fn passed(&self) -> bool {
            self.missing.is_empty() && self.duplicated.is_empty() && self.empty.is_empty()
        }
    }

    pub fn review(evidence: &[Evidence]) -> ApiAudit {
        let mut seen = HashSet::new();
        let mut duplicated = Vec::new();
        let mut empty = Vec::new();
        for item in evidence {
            if !seen.insert(item.field) {
                duplicated.push(item.field);
            }
            if item.detail.trim().is_empty() {
                empty.push(item.field);
            }
        }
        let missing = ALL_FIELDS
            .into_iter()
            .filter(|field| !seen.contains(field))
            .collect();
        ApiAudit {
            missing,
            duplicated,
            empty,
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
                    detail: "linked evidence",
                })
                .collect()
        }

        #[test]
        fn ten_distinct_non_empty_answers_pass_the_gate() {
            assert!(review(&complete_evidence()).passed());
        }

        #[test]
        fn no_aplica_without_a_justification_is_empty_evidence() {
            let mut evidence = complete_evidence();
            evidence[7].detail = "  ";
            assert_eq!(review(&evidence).empty, [AuditField::FeatureAndTarget]);
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
        pub choice: &'static str,
        pub alternative: &'static str,
        pub consequence: &'static str,
        pub revisit_when: &'static str,
    }

    pub fn defensible(decisions: &[Decision]) -> bool {
        decisions.len() >= 5
            && decisions.iter().all(|decision| {
                !decision.choice.trim().is_empty()
                    && !decision.alternative.trim().is_empty()
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
        fn five_decisions_need_alternatives_consequences_and_revisit_conditions() {
            let decision = Decision {
                choice: "bounded channel",
                alternative: "unbounded channel",
                consequence: "admission can wait",
                revisit_when: "measured burst changes",
            };
            assert!(defensible(&[decision; 5]));
            let incomplete = Decision {
                alternative: "",
                ..decision
            };
            assert!(!defensible(&[
                decision, decision, decision, decision, incomplete
            ]));
        }
    }
}
