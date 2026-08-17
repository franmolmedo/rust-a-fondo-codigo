//! Capítulos de organización, testing, macros y diseño de librerías.

pub mod c25 {
    use std::error::Error;
    use std::fmt::{self, Display, Formatter};
    use std::num::ParseIntError;

    mod domain {
        mod order {
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub struct OrderId(pub u64);

            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Order {
                id: OrderId,
            }

            impl Order {
                pub fn new(id: OrderId) -> Self {
                    Self { id }
                }

                pub fn id(&self) -> OrderId {
                    self.id
                }
            }
        }

        mod user {
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub struct UserId(pub u64);
        }

        // SOLUTION: C25-E01
        pub use order::{Order, OrderId};
        pub use user::UserId;
    }

    pub use domain::{Order, OrderId, UserId};

    mod helper {
        pub(super) fn trim_and_lowercase(value: &str) -> String {
            value.trim().to_lowercase()
        }
    }

    // SOLUTION: C25-E02
    pub(crate) fn normalize_label(value: &str) -> String {
        helper::trim_and_lowercase(value)
    }

    mod relocated {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct StableReport(pub u64);
    }

    // SOLUTION: C25-E04
    pub use relocated::StableReport;

    mod audited {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Service {
            name: String,
        }

        impl Service {
            pub fn new(name: impl Into<String>) -> Self {
                Self {
                    name: InternalPlan::prepare(name.into()),
                }
            }

            pub fn name(&self) -> &str {
                &self.name
            }
        }

        struct InternalPlan;

        impl InternalPlan {
            fn prepare(name: String) -> String {
                super::normalize_label(&name)
            }
        }
    }

    // SOLUTION: C25-E05
    pub use audited::Service as AuditedService;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Email(String);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct InvalidEmail;

    // SOLUTION: C25-E03
    impl Email {
        pub fn parse(value: impl Into<String>) -> Result<Self, InvalidEmail> {
            let value = value.into();
            value
                .contains('@')
                .then_some(Self(value))
                .ok_or(InvalidEmail)
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug)]
    pub struct PortParseError {
        source: ParseIntError,
    }

    impl Display for PortParseError {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
            formatter.write_str("puerto inválido")
        }
    }

    impl Error for PortParseError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.source)
        }
    }

    // SOLUTION: C25-E06
    pub fn parse_port(input: &str) -> Result<u16, PortParseError> {
        input.parse().map_err(|source| PortParseError { source })
    }

    #[cfg(test)]
    mod checksum {
        // SOLUTION: C25-E07
        fn private_checksum(bytes: &[u8]) -> u8 {
            bytes.iter().fold(0, |accumulator, byte| accumulator ^ byte)
        }

        #[cfg(test)]
        mod tests {
            use super::private_checksum;

            #[test]
            fn a_descendant_unit_test_can_see_the_private_item() {
                assert_eq!(private_checksum(&[1, 2, 3]), 0);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn facade_hides_internal_module_layout() {
            let order = Order::new(OrderId(7));
            assert_eq!(order.id(), OrderId(7));
            let _user_id = UserId(3);
        }

        #[test]
        fn private_field_preserves_validation() {
            let email = Email::parse("ada@example.test").unwrap();
            assert_eq!(email.as_str(), "ada@example.test");
            assert_eq!(Email::parse("invalid"), Err(InvalidEmail));
        }

        #[test]
        fn graduated_visibility_limits_each_helper_to_its_audience() {
            assert_eq!(normalize_label("  RuSt  "), "rust");
        }

        #[test]
        fn a_reexport_keeps_the_public_path_stable_after_a_move() {
            assert_eq!(StableReport(7), StableReport(7));
        }

        #[test]
        fn an_api_audit_exposes_the_service_but_not_its_internal_plan() {
            let service = AuditedService::new("orders");
            assert_eq!(service.name(), "orders");
        }

        #[test]
        fn a_public_error_hides_the_dependency_specific_type() {
            assert_eq!(parse_port("8080").unwrap(), 8080);
            let error = parse_port("not-a-port").unwrap_err();
            assert_eq!(error.to_string(), "puerto inválido");
            assert!(error.source().is_some());
        }
    }
}

pub mod c26 {
    use std::collections::{HashMap, HashSet, VecDeque};

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct WorkspacePlan {
        pub members: Vec<&'static str>,
        pub dependencies: Vec<(&'static str, &'static str)>,
    }

    // SOLUTION: C26-E01
    pub fn reference_workspace() -> WorkspacePlan {
        WorkspacePlan {
            members: vec!["domain", "application", "adapters", "server"],
            dependencies: vec![
                ("application", "domain"),
                ("adapters", "application"),
                ("adapters", "domain"),
                ("server", "adapters"),
            ],
        }
    }

    // SOLUTION: C26-E02
    pub fn dependency_graph_is_acyclic(edges: &[(&str, &str)]) -> bool {
        let mut vertices = HashSet::new();
        let mut indegree = HashMap::new();
        let mut outgoing: HashMap<&str, Vec<&str>> = HashMap::new();

        for &(from, to) in edges {
            vertices.insert(from);
            vertices.insert(to);
            outgoing.entry(from).or_default().push(to);
            *indegree.entry(to).or_insert(0_usize) += 1;
            indegree.entry(from).or_insert(0);
        }

        let mut ready: VecDeque<_> = vertices
            .iter()
            .copied()
            .filter(|vertex| indegree.get(vertex).copied().unwrap_or(0) == 0)
            .collect();
        let mut visited = 0;

        while let Some(vertex) = ready.pop_front() {
            visited += 1;
            if let Some(neighbors) = outgoing.get(vertex) {
                for neighbor in neighbors {
                    let remaining = indegree
                        .get_mut(neighbor)
                        .expect("all destinations have an indegree");
                    *remaining -= 1;
                    if *remaining == 0 {
                        ready.push_back(neighbor);
                    }
                }
            }
        }

        visited == vertices.len()
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct User {
        pub id: u64,
        pub name: String,
    }

    // SOLUTION: C26-E03
    pub trait UserRepository {
        type Error;

        fn find(&self, id: u64) -> Result<Option<User>, Self::Error>;
    }

    pub fn user_name<R>(repository: &R, id: u64) -> Result<Option<String>, R::Error>
    where
        R: UserRepository,
    {
        repository.find(id).map(|user| user.map(|user| user.name))
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct RegisterUserRequest {
        pub id: u64,
        pub name: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct RegisterUser {
        id: u64,
        name: String,
    }

    impl RegisterUser {
        pub fn id(&self) -> u64 {
            self.id
        }

        pub fn name(&self) -> &str {
            &self.name
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ValidationError {
        ZeroId,
        EmptyName,
    }

    // SOLUTION: C26-E04
    impl TryFrom<RegisterUserRequest> for RegisterUser {
        type Error = ValidationError;

        fn try_from(request: RegisterUserRequest) -> Result<Self, Self::Error> {
            if request.id == 0 {
                return Err(ValidationError::ZeroId);
            }
            let name = request.name.trim();
            if name.is_empty() {
                return Err(ValidationError::EmptyName);
            }
            Ok(Self {
                id: request.id,
                name: name.to_owned(),
            })
        }
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct BoundaryEvidence {
        pub enforces_dependency_direction: bool,
        pub has_independent_targets: bool,
        pub is_reused_independently: bool,
        pub is_published_independently: bool,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum BoundaryChoice {
        Module,
        Package,
    }

    // SOLUTION: C26-E05
    pub fn choose_boundary(evidence: BoundaryEvidence) -> BoundaryChoice {
        if evidence.enforces_dependency_direction
            || evidence.has_independent_targets
            || evidence.is_reused_independently
            || evidence.is_published_independently
        {
            BoundaryChoice::Package
        } else {
            BoundaryChoice::Module
        }
    }

    // SOLUTION: C26-E06
    pub fn architecture_violations(dependencies: &[&str], forbidden: &[&str]) -> Vec<String> {
        dependencies
            .iter()
            .filter(|dependency| forbidden.contains(dependency))
            .map(|dependency| (*dependency).to_owned())
            .collect()
    }

    // SOLUTION: C26-E07
    #[derive(Default)]
    pub struct FakeUserRepository {
        users: HashMap<u64, User>,
    }

    impl FakeUserRepository {
        pub fn with_user(mut self, user: User) -> Self {
            self.users.insert(user.id, user);
            self
        }
    }

    impl UserRepository for FakeUserRepository {
        type Error = std::convert::Infallible;

        fn find(&self, id: u64) -> Result<Option<User>, Self::Error> {
            Ok(self.users.get(&id).cloned())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn reference_workspace_has_an_acyclic_inward_graph() {
            let workspace = reference_workspace();
            assert_eq!(workspace.members.len(), 4);
            assert!(dependency_graph_is_acyclic(&workspace.dependencies));
        }

        #[test]
        fn a_reverse_dependency_exposes_the_cycle() {
            let cyclic = [
                ("application", "domain"),
                ("adapters", "application"),
                ("domain", "adapters"),
            ];
            assert!(!dependency_graph_is_acyclic(&cyclic));
        }

        #[test]
        fn dto_conversion_builds_only_a_valid_command() {
            let command = RegisterUser::try_from(RegisterUserRequest {
                id: 7,
                name: String::from("  Ada  "),
            })
            .unwrap();
            assert_eq!(command.id(), 7);
            assert_eq!(command.name(), "Ada");
            assert_eq!(
                RegisterUser::try_from(RegisterUserRequest {
                    id: 0,
                    name: String::from("Ada"),
                }),
                Err(ValidationError::ZeroId)
            );
        }

        #[test]
        fn a_boundary_needs_structural_evidence() {
            assert_eq!(
                choose_boundary(BoundaryEvidence::default()),
                BoundaryChoice::Module
            );
            assert_eq!(
                choose_boundary(BoundaryEvidence {
                    enforces_dependency_direction: true,
                    ..BoundaryEvidence::default()
                }),
                BoundaryChoice::Package
            );
        }

        #[test]
        fn dependency_audit_reports_frameworks_in_the_domain() {
            let violations = architecture_violations(
                &["serde", "axum", "thiserror", "sqlx"],
                &["axum", "sqlx", "tokio"],
            );
            assert_eq!(violations, ["axum", "sqlx"]);
        }

        #[test]
        fn fake_implements_the_consumers_port() {
            let repository = FakeUserRepository::default().with_user(User {
                id: 1,
                name: String::from("Ada"),
            });
            assert_eq!(
                user_name(&repository, 1).unwrap(),
                Some(String::from("Ada"))
            );
        }
    }
}

pub mod c27 {
    use std::collections::BTreeSet;

    // SOLUTION: C27-E01
    #[cfg_attr(feature = "json", derive(serde::Serialize))]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Record {
        pub id: u64,
        pub name: String,
    }

    pub const fn json_feature_enabled() -> bool {
        cfg!(feature = "json")
    }

    #[cfg(feature = "json")]
    pub fn record_json(record: &Record) -> Result<String, serde_json::Error> {
        serde_json::to_string(record)
    }

    fn normalize_record(value: &str) -> String {
        value.trim().to_lowercase()
    }

    // SOLUTION: C27-E02
    pub fn run_import(value: &str) -> String {
        format!("import:{}", normalize_record(value))
    }

    pub fn run_export(value: &str) -> String {
        format!("export:{}", normalize_record(value))
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct FeatureActivation<'a> {
        pub activated_by: &'a str,
        pub feature: &'a str,
    }

    // SOLUTION: C27-E03
    pub fn unified_features(activations: &[FeatureActivation<'_>]) -> BTreeSet<String> {
        activations
            .iter()
            .map(|activation| activation.feature.to_owned())
            .collect()
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct MsrvPolicy<'a> {
        pub declared: &'a str,
        pub tested_toolchains: Vec<&'a str>,
    }

    // SOLUTION: C27-E04
    pub fn msrv_is_verified(policy: &MsrvPolicy<'_>) -> bool {
        policy.tested_toolchains.contains(&policy.declared)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ProfileMeasurement {
        pub build_millis: u64,
        pub binary_bytes: u64,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ProfileComparison {
        pub faster_build: &'static str,
        pub smaller_binary: &'static str,
    }

    // SOLUTION: C27-E05
    pub fn compare_profiles(
        baseline: ProfileMeasurement,
        optimized: ProfileMeasurement,
    ) -> ProfileComparison {
        ProfileComparison {
            faster_build: if baseline.build_millis <= optimized.build_millis {
                "baseline"
            } else {
                "optimized"
            },
            smaller_binary: if baseline.binary_bytes <= optimized.binary_bytes {
                "baseline"
            } else {
                "optimized"
            },
        }
    }

    // SOLUTION: C27-E06
    pub fn build_script_directives(inputs: &[&str]) -> Vec<String> {
        inputs
            .iter()
            .map(|path| format!("cargo::rerun-if-changed={path}"))
            .chain(std::iter::once(String::from(
                "cargo::rustc-env=CATALOG_SCHEMA_VERSION=3",
            )))
            .collect()
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Backend {
        Postgres,
        Sqlite,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct UnknownBackend;

    // SOLUTION: C27-E07
    impl Backend {
        pub fn parse(value: &str) -> Result<Self, UnknownBackend> {
            match value {
                "postgres" => Ok(Self::Postgres),
                "sqlite" => Ok(Self::Sqlite),
                _ => Err(UnknownBackend),
            }
        }

        pub const fn scheme(self) -> &'static str {
            match self {
                Self::Postgres => "postgresql://",
                Self::Sqlite => "sqlite://",
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn feature_probe_matches_the_compilation_configuration() {
            assert_eq!(json_feature_enabled(), cfg!(feature = "json"));
        }

        #[cfg(feature = "json")]
        #[test]
        fn json_feature_compiles_and_serializes_when_enabled() {
            let json = record_json(&Record {
                id: 7,
                name: String::from("Ada"),
            })
            .unwrap();
            assert_eq!(json, r#"{"id":7,"name":"Ada"}"#);
        }

        #[test]
        fn two_binary_entry_points_share_library_behavior() {
            assert_eq!(run_import("  ITEM "), "import:item");
            assert_eq!(run_export("  ITEM "), "export:item");
        }

        #[test]
        fn feature_activations_are_additive() {
            let activations = [
                FeatureActivation {
                    activated_by: "api",
                    feature: "serde",
                },
                FeatureActivation {
                    activated_by: "cli",
                    feature: "metrics",
                },
                FeatureActivation {
                    activated_by: "server",
                    feature: "serde",
                },
            ];
            assert_eq!(
                unified_features(&activations),
                BTreeSet::from([String::from("metrics"), String::from("serde")])
            );
        }

        #[test]
        fn declared_msrv_must_be_in_the_ci_matrix() {
            let policy = MsrvPolicy {
                declared: "1.85",
                tested_toolchains: vec!["1.85", "stable"],
            };
            assert!(msrv_is_verified(&policy));
        }

        #[test]
        fn profile_comparison_preserves_both_tradeoffs() {
            let comparison = compare_profiles(
                ProfileMeasurement {
                    build_millis: 900,
                    binary_bytes: 2_000,
                },
                ProfileMeasurement {
                    build_millis: 1_400,
                    binary_bytes: 1_200,
                },
            );
            assert_eq!(comparison.faster_build, "baseline");
            assert_eq!(comparison.smaller_binary, "optimized");
        }

        #[test]
        fn build_script_names_every_input() {
            assert_eq!(
                build_script_directives(&["schema/catalog.proto"]),
                [
                    "cargo::rerun-if-changed=schema/catalog.proto",
                    "cargo::rustc-env=CATALOG_SCHEMA_VERSION=3",
                ]
            );
        }

        #[test]
        fn runtime_configuration_makes_backend_exclusivity_explicit() {
            assert_eq!(Backend::parse("sqlite").unwrap().scheme(), "sqlite://");
            assert_eq!(Backend::parse("both"), Err(UnknownBackend));
        }
    }
}

pub mod c28 {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Percentage(u8);

    // SOLUTION: C28-E01
    impl Percentage {
        pub const fn new(value: u8) -> Option<Self> {
            if value <= 100 {
                Some(Self(value))
            } else {
                None
            }
        }

        pub const fn get(self) -> u8 {
            self.0
        }
    }

    // SOLUTION: C28-E02
    /// Identificador que solo puede construirse mediante su API pública.
    ///
    /// ```
    /// use course_solutions::organization::c28::OpaqueId;
    /// let id = OpaqueId::new(7).unwrap();
    /// assert_eq!(id.get(), 7);
    /// ```
    ///
    /// ```compile_fail
    /// use course_solutions::organization::c28::OpaqueId;
    /// let _id = OpaqueId(7); // el campo es privado fuera de la crate
    /// ```
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct OpaqueId(u64);

    impl OpaqueId {
        pub const fn new(value: u64) -> Option<Self> {
            if value == 0 { None } else { Some(Self(value)) }
        }

        pub const fn get(self) -> u64 {
            self.0
        }
    }

    pub trait Clock {
        fn now_millis(&self) -> u64;
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct FixedClock(pub u64);

    impl Clock for FixedClock {
        fn now_millis(&self) -> u64 {
            self.0
        }
    }

    // SOLUTION: C28-E03
    pub fn token_is_expired(clock: &impl Clock, expires_at: u64) -> bool {
        clock.now_millis() >= expires_at
    }

    fn encode_numbers(values: &[i32]) -> String {
        values
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }

    fn decode_numbers(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
        if input.is_empty() {
            return Ok(Vec::new());
        }
        input.split(',').map(str::parse).collect()
    }

    // SOLUTION: C28-E04
    pub fn numbers_round_trip(values: &[i32]) -> Result<Vec<i32>, std::num::ParseIntError> {
        decode_numbers(&encode_numbers(values))
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum TestLayer {
        Unit,
        Integration,
        EndToEnd,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct TestBoundary {
        pub crosses_public_api: bool,
        pub uses_real_external_adapter: bool,
    }

    // SOLUTION: C28-E05
    pub const fn classify_test(boundary: TestBoundary) -> TestLayer {
        if boundary.uses_real_external_adapter {
            TestLayer::EndToEnd
        } else if boundary.crosses_public_api {
            TestLayer::Integration
        } else {
            TestLayer::Unit
        }
    }

    // SOLUTION: C28-E06
    pub fn parse_key_value(input: &str) -> Result<(&str, &str), &'static str> {
        let (key, value) = input.split_once('=').ok_or("falta =")?;
        if key.is_empty() {
            return Err("clave vacía");
        }
        Ok((key, value))
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct BackendFailure;

    pub trait RegistrationPort {
        fn store(&mut self, id: u64) -> Result<(), BackendFailure>;
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum RegistrationError {
        RepositoryUnavailable,
    }

    // SOLUTION: C28-E07
    pub fn register(port: &mut impl RegistrationPort, id: u64) -> Result<(), RegistrationError> {
        port.store(id)
            .map_err(|BackendFailure| RegistrationError::RepositoryUnavailable)
    }

    pub struct FailingRegistrationPort;

    impl RegistrationPort for FailingRegistrationPort {
        fn store(&mut self, _id: u64) -> Result<(), BackendFailure> {
            Err(BackendFailure)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use proptest::prelude::*;

        #[test]
        fn unit_test_protects_the_local_invariant() {
            assert_eq!(Percentage::new(100).map(Percentage::get), Some(100));
            assert_eq!(Percentage::new(101), None);
        }

        #[test]
        fn opaque_identifier_is_usable_through_its_public_contract() {
            assert_eq!(OpaqueId::new(9).map(OpaqueId::get), Some(9));
            assert_eq!(OpaqueId::new(0), None);
        }

        #[test]
        fn fixed_clock_makes_expiration_exact() {
            let clock = FixedClock(1_000);
            assert!(!token_is_expired(&clock, 1_001));
            assert!(token_is_expired(&clock, 1_000));
        }

        proptest! {
            #[test]
            fn number_encoding_round_trips(values in proptest::collection::vec(any::<i32>(), 0..64)) {
                prop_assert_eq!(numbers_round_trip(&values).unwrap(), values);
            }
        }

        #[test]
        fn suite_classification_follows_the_observed_boundary() {
            assert_eq!(
                classify_test(TestBoundary {
                    crosses_public_api: true,
                    uses_real_external_adapter: false,
                }),
                TestLayer::Integration
            );
        }

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(1_000))]
            #[test]
            fn parser_never_panics(input in any::<String>()) {
                let _result = parse_key_value(&input);
            }
        }

        #[test]
        fn known_cases_are_structural() {
            assert_eq!(parse_key_value("language=Rust"), Ok(("language", "Rust")));
            assert_eq!(parse_key_value("missing"), Err("falta ="));
        }

        #[test]
        fn failure_fake_exercises_error_translation() {
            let mut port = FailingRegistrationPort;
            assert_eq!(
                register(&mut port, 7),
                Err(RegistrationError::RepositoryUnavailable)
            );
        }
    }
}

pub mod c29 {
    use thiserror::Error;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Port(u16);

    #[derive(Clone, Debug, Error, Eq, PartialEq)]
    pub enum PortError {
        #[error("el puerto cero está reservado")]
        Zero,
        #[error("el puerto no es un entero de 16 bits")]
        Invalid,
    }

    // SOLUTION: C29-E01
    impl Port {
        /// Construye un puerto validado.
        ///
        /// # Examples
        ///
        /// ```
        /// use course_solutions::organization::c29::Port;
        /// let port = Port::new(8080)?;
        /// assert_eq!(port.get(), 8080);
        /// # Ok::<(), course_solutions::organization::c29::PortError>(())
        /// ```
        ///
        /// # Errors
        ///
        /// Devuelve [`PortError::Zero`] cuando el valor es cero.
        pub fn new(value: u16) -> Result<Self, PortError> {
            (value != 0).then_some(Self(value)).ok_or(PortError::Zero)
        }

        pub fn get(&self) -> u16 {
            self.0
        }

        // SOLUTION: C29-E02
        /// Convierte texto en un [`Port`] o devuelve [`PortError`].
        ///
        /// # Errors
        ///
        /// Devuelve [`PortError::Invalid`] si el texto no es numérico y
        /// [`PortError::Zero`] si representa el puerto reservado cero.
        pub fn parse(value: &str) -> Result<Self, PortError> {
            let value = value.parse::<u16>().map_err(|_| PortError::Invalid)?;
            Self::new(value)
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Snapshot(Vec<u8>);

    // SOLUTION: C29-E03
    impl Snapshot {
        pub fn new(bytes: Vec<u8>) -> Self {
            Self(bytes)
        }

        pub fn as_bytes(&self) -> &[u8] {
            &self.0
        }

        pub fn to_vec(&self) -> Vec<u8> {
            self.0.clone()
        }

        pub fn into_bytes(self) -> Vec<u8> {
            self.0
        }
    }

    // SOLUTION: C29-E04
    /// Error de protocolo abierto a variantes futuras.
    ///
    /// Los consumidores externos deben conservar un caso comodín:
    ///
    /// ```compile_fail
    /// use course_solutions::organization::c29::ProtocolError;
    /// fn label(error: ProtocolError) -> &'static str {
    ///     match error {
    ///         ProtocolError::Timeout => "timeout",
    ///         ProtocolError::Rejected => "rejected",
    ///     }
    /// }
    /// ```
    #[non_exhaustive]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ProtocolError {
        Timeout,
        Rejected,
    }

    // SOLUTION: C29-E05
    pub const CI_COMMANDS: [&str; 6] = [
        "cargo fmt --all -- --check",
        "cargo clippy --all-targets --all-features -- -D warnings",
        "cargo test --workspace --all-targets",
        "cargo test --workspace --all-targets --all-features",
        "cargo test --workspace --doc",
        "cargo test --workspace --doc --all-features",
    ];

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct LintPolicy {
        pub name: &'static str,
        pub level: &'static str,
        pub reason: &'static str,
    }

    // SOLUTION: C29-E06
    pub const fn workspace_lints() -> [LintPolicy; 3] {
        [
            LintPolicy {
                name: "unsafe_op_in_unsafe_fn",
                level: "deny",
                reason: "cada operación unsafe debe quedar localizada y visible",
            },
            LintPolicy {
                name: "unused_must_use",
                level: "deny",
                reason: "no se pueden ignorar resultados ni futures importantes",
            },
            LintPolicy {
                name: "clippy::dbg_macro",
                level: "deny",
                reason: "evita trazas accidentales en entregas",
            },
        ]
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum OperationCost {
        BorrowedView,
        AllocatingCopy,
        ConsumingConversion,
    }

    // SOLUTION: C29-E07
    pub fn name_matches_cost(name: &str, cost: OperationCost) -> bool {
        match cost {
            OperationCost::BorrowedView => name.starts_with("as_"),
            OperationCost::AllocatingCopy => name.starts_with("to_"),
            OperationCost::ConsumingConversion => name.starts_with("into_"),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn constructor_reports_its_documented_error() {
            assert_eq!(Port::new(0), Err(PortError::Zero));
        }

        #[test]
        fn intra_doc_contract_matches_parser_errors() {
            assert_eq!(Port::parse("8080").unwrap().get(), 8080);
            assert_eq!(Port::parse("x"), Err(PortError::Invalid));
            assert_eq!(Port::parse("0"), Err(PortError::Zero));
        }

        #[test]
        fn names_make_snapshot_ownership_and_cost_visible() {
            let snapshot = Snapshot::new(vec![1, 2, 3]);
            assert_eq!(snapshot.as_bytes(), &[1, 2, 3]);
            assert_eq!(snapshot.to_vec(), vec![1, 2, 3]);
            assert_eq!(snapshot.into_bytes(), vec![1, 2, 3]);
        }

        #[test]
        fn ci_checks_format_lints_library_tests_and_doctests() {
            assert!(CI_COMMANDS.iter().any(|command| command.contains("fmt")));
            assert!(CI_COMMANDS.iter().any(|command| command.contains("clippy")));
            assert!(CI_COMMANDS.iter().any(|command| command.contains("--doc")));
        }

        #[test]
        fn every_workspace_lint_has_a_local_reason() {
            assert!(workspace_lints().iter().all(|lint| !lint.reason.is_empty()));
        }

        #[test]
        fn allocation_is_not_hidden_behind_as_prefix() {
            assert!(name_matches_cost("to_vec", OperationCost::AllocatingCopy));
            assert!(!name_matches_cost("as_vec", OperationCost::AllocatingCopy));
        }
    }
}

pub mod c49 {
    pub fn qualify(value: &str) -> String {
        format!("course-solutions::{value}")
    }

    pub const fn port_from_literal(value: u16) -> u16 {
        value
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EditionExpressionKind {
        LegacyExpression,
        ConstBlock,
        Placeholder,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ForwardedExpressionKind {
        LiteralThree,
        OpaqueExpression,
    }

    #[macro_export]
    // SOLUTION: C49-E01
    macro_rules! sum_once {
        ($($value:expr),+ $(,)?) => {{
            let mut total = 0;
            $(
                let evaluated_once = $value;
                total += evaluated_once;
            )+
            total
        }};
    }

    #[macro_export]
    // SOLUTION: C49-E02
    macro_rules! make_newtypes {
        ($($name:ident($inner:ty)),+ $(,)?) => {
            $(
                #[derive(Clone, Copy, Debug, Eq, PartialEq)]
                pub struct $name(pub $inner);
            )+
        };
    }

    #[macro_export]
    // SOLUTION: C49-E03
    macro_rules! qualified {
        ($value:expr) => {
            $crate::organization::c49::qualify($value)
        };
    }

    #[macro_export]
    // SOLUTION: C49-E04
    macro_rules! checked_port {
        ($value:literal) => {
            $crate::organization::c49::port_from_literal($value)
        };
        ($($other:tt)*) => {
            compile_error!("checked_port! espera un único literal entero entre 0 y 65535")
        };
    }

    #[macro_export]
    // SOLUTION: C49-E05
    macro_rules! classify_edition_expression {
        (const $value:expr) => {
            $crate::organization::c49::EditionExpressionKind::ConstBlock
        };
        (_) => {
            $crate::organization::c49::EditionExpressionKind::Placeholder
        };
        ($value:expr_2021) => {
            $crate::organization::c49::EditionExpressionKind::LegacyExpression
        };
    }

    #[macro_export]
    macro_rules! classify_direct_expression {
        (3) => {
            $crate::organization::c49::ForwardedExpressionKind::LiteralThree
        };
        ($value:expr) => {
            $crate::organization::c49::ForwardedExpressionKind::OpaqueExpression
        };
    }

    #[macro_export]
    // SOLUTION: C49-E06
    macro_rules! forward_expression {
        ($value:expr) => {
            $crate::classify_direct_expression!($value)
        };
    }

    #[macro_export]
    // SOLUTION: C49-E07
    macro_rules! command_values {
        ($( $name:ident => $value:expr );+ $(;)?) => {{
            let mut output = ::std::vec::Vec::new();
            $(
                let evaluated_once = $value;
                output.push((stringify!($name), evaluated_once));
            )+
            output
        }};
        ($($other:tt)*) => {
            compile_error!("command_values! espera `nombre => expresión;` una o más veces")
        };
    }

    #[cfg(test)]
    mod tests {
        use std::cell::Cell;

        crate::make_newtypes!(UserId(u64), OrderId(u64));

        #[test]
        fn variadic_macro_evaluates_each_expression_once() {
            let calls = Cell::new(0);
            let next = || {
                calls.set(calls.get() + 1);
                10
            };
            assert_eq!(crate::sum_once!(next(), next(),), 20);
            assert_eq!(calls.get(), 2);
        }

        #[test]
        fn repetition_generates_each_newtype() {
            assert_eq!(UserId(1).0, 1);
            assert_eq!(OrderId(2).0, 2);
        }

        #[test]
        fn exported_macro_resolves_helper_through_crate() {
            assert_eq!(crate::qualified!("macro"), "course-solutions::macro");
        }

        #[test]
        fn diagnostic_macro_accepts_its_documented_literal_form() {
            assert_eq!(crate::checked_port!(8080), 8080_u16);
        }

        #[test]
        fn edition_sensitive_arms_keep_const_and_placeholder_distinct() {
            use super::EditionExpressionKind;

            assert_eq!(
                crate::classify_edition_expression!(1 + 2),
                EditionExpressionKind::LegacyExpression,
            );
            assert_eq!(
                crate::classify_edition_expression!(const { 1 + 2 }),
                EditionExpressionKind::ConstBlock,
            );
            assert_eq!(
                crate::classify_edition_expression!(_),
                EditionExpressionKind::Placeholder,
            );
        }

        #[test]
        fn expr_forwarding_is_opaque_to_literal_matching() {
            use super::ForwardedExpressionKind;

            assert_eq!(
                crate::classify_direct_expression!(3),
                ForwardedExpressionKind::LiteralThree,
            );
            assert_eq!(
                crate::forward_expression!(3),
                ForwardedExpressionKind::OpaqueExpression,
            );
        }

        #[test]
        fn structured_dsl_preserves_order_and_single_evaluation() {
            let calls = Cell::new(0);
            let next = || {
                calls.set(calls.get() + 1);
                calls.get()
            };
            let commands = crate::command_values!(load => next(); save => next(););

            assert_eq!(commands, [("load", 1), ("save", 2)]);
            assert_eq!(calls.get(), 2);
        }
    }
}

pub mod c50 {
    #[derive(Debug)]
    pub struct NoEntityImpl;

    // SOLUTION: C50-E01
    #[derive(course_macro_api::Entity)]
    #[entity(id = "id")]
    pub struct MinimalEntity<T> {
        pub id: u64,
        pub payload: T,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SourceSpan {
        pub start: usize,
        pub end: usize,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MacroDiagnostic {
        pub message: String,
        pub span: SourceSpan,
    }

    // SOLUTION: C50-E02
    pub fn validate_id_field(
        fields: &[&str],
        requested: &str,
        attribute_span: SourceSpan,
    ) -> Result<(), MacroDiagnostic> {
        if fields.contains(&requested) {
            Ok(())
        } else {
            Err(MacroDiagnostic {
                message: format!("el campo `{requested}` no existe en esta struct"),
                span: attribute_span,
            })
        }
    }

    // SOLUTION: C50-E03
    #[derive(course_macro_api::Entity)]
    #[entity(id = "id")]
    pub struct GenericEntity<'a, T, const LENGTH: usize>
    where
        T: 'a,
    {
        pub id: u64,
        pub values: &'a [T; LENGTH],
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ProcMacroRisk {
        pub reads_files: bool,
        pub reads_environment: bool,
        pub uses_network: bool,
        pub dependency_unpinned: bool,
        pub source_unaudited: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SupplyDecision {
        Approve,
        Review,
        Reject,
    }

    // SOLUTION: C50-E04
    pub const fn assess_proc_macro_risk(risk: ProcMacroRisk) -> SupplyDecision {
        if risk.uses_network || (risk.source_unaudited && risk.dependency_unpinned) {
            SupplyDecision::Reject
        } else if risk.reads_files
            || risk.reads_environment
            || risk.source_unaudited
            || risk.dependency_unpinned
        {
            SupplyDecision::Review
        } else {
            SupplyDecision::Approve
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MetaprogrammingNeed {
        RuntimeTypedOperation,
        RepeatedRustSyntax,
        CustomGrammarWithPreciseSpans,
        TransformWholeItem,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MetaprogrammingTool {
        Function,
        MacroRules,
        ProceduralMacro,
    }

    // SOLUTION: C50-E05
    pub const fn choose_metaprogramming_tool(need: MetaprogrammingNeed) -> MetaprogrammingTool {
        match need {
            MetaprogrammingNeed::RuntimeTypedOperation => MetaprogrammingTool::Function,
            MetaprogrammingNeed::RepeatedRustSyntax => MetaprogrammingTool::MacroRules,
            MetaprogrammingNeed::CustomGrammarWithPreciseSpans
            | MetaprogrammingNeed::TransformWholeItem => MetaprogrammingTool::ProceduralMacro,
        }
    }

    // SOLUTION: C50-E06
    #[course_macro_api::preserve_item]
    #[inline]
    pub fn preserved_answer() -> u32 {
        42
    }

    // SOLUTION: C50-E07
    pub const ENTITY_FIELDS: &[&str] = course_macro_api::field_names!(id, payload,);

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn derive_adds_no_bound_for_an_unused_generic_field_type() {
            let entity = MinimalEntity {
                id: 1,
                payload: NoEntityImpl,
            };
            assert_eq!(entity.id, 1);
            assert_eq!(
                <MinimalEntity<NoEntityImpl> as course_macro_api::Entity>::id_field(),
                "id",
            );
        }

        #[test]
        fn invalid_field_diagnostic_keeps_the_attribute_value_span() {
            let span = SourceSpan { start: 20, end: 29 };
            assert_eq!(
                validate_id_field(&["id", "name"], "missing", span),
                Err(MacroDiagnostic {
                    message: String::from("el campo `missing` no existe en esta struct"),
                    span,
                }),
            );
        }

        #[test]
        fn derive_preserves_lifetime_const_generic_and_where_clause() {
            let values = [NoEntityImpl];
            let entity = GenericEntity {
                id: 7,
                values: &values,
            };
            assert_eq!(entity.values.len(), 1);
            assert_eq!(
                <GenericEntity<'_, NoEntityImpl, 1> as course_macro_api::Entity>::entity_name(),
                "GenericEntity",
            );
        }

        #[test]
        fn supply_risk_requires_review_or_rejection_when_inputs_expand() {
            assert_eq!(
                assess_proc_macro_risk(ProcMacroRisk {
                    reads_files: false,
                    reads_environment: false,
                    uses_network: false,
                    dependency_unpinned: false,
                    source_unaudited: false,
                }),
                SupplyDecision::Approve,
            );
            assert_eq!(
                assess_proc_macro_risk(ProcMacroRisk {
                    reads_files: true,
                    reads_environment: false,
                    uses_network: false,
                    dependency_unpinned: false,
                    source_unaudited: false,
                }),
                SupplyDecision::Review,
            );
            assert_eq!(
                assess_proc_macro_risk(ProcMacroRisk {
                    reads_files: false,
                    reads_environment: false,
                    uses_network: true,
                    dependency_unpinned: false,
                    source_unaudited: false,
                }),
                SupplyDecision::Reject,
            );
        }

        #[test]
        fn tool_choice_starts_with_the_least_powerful_sufficient_option() {
            assert_eq!(
                choose_metaprogramming_tool(MetaprogrammingNeed::RuntimeTypedOperation),
                MetaprogrammingTool::Function,
            );
            assert_eq!(
                choose_metaprogramming_tool(MetaprogrammingNeed::RepeatedRustSyntax),
                MetaprogrammingTool::MacroRules,
            );
            assert_eq!(
                choose_metaprogramming_tool(MetaprogrammingNeed::CustomGrammarWithPreciseSpans),
                MetaprogrammingTool::ProceduralMacro,
            );
        }

        #[test]
        fn attribute_macro_preserves_the_function_contract() {
            assert_eq!(preserved_answer(), 42);
        }

        #[test]
        fn function_like_macro_parses_identifiers_and_a_trailing_comma() {
            assert_eq!(ENTITY_FIELDS, ["id", "payload"]);
        }
    }
}

pub mod c51 {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct RustVersion {
        pub major: u16,
        pub minor: u16,
        pub patch: u16,
    }

    impl RustVersion {
        pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
            Self {
                major,
                minor,
                patch,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Edition {
        Rust2015,
        Rust2018,
        Rust2021,
        Rust2024,
    }

    impl Edition {
        pub const fn minimum_compiler(self) -> RustVersion {
            match self {
                Self::Rust2015 => RustVersion::new(1, 0, 0),
                Self::Rust2018 => RustVersion::new(1, 31, 0),
                Self::Rust2021 => RustVersion::new(1, 56, 0),
                Self::Rust2024 => RustVersion::new(1, 85, 0),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ToolchainContract {
        pub edition: Edition,
        pub toolchain: RustVersion,
        pub msrv: RustVersion,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ContractError {
        ToolchainBeforeEdition,
        MsrvBeforeEdition,
        ToolchainBeforeMsrv,
    }

    // SOLUTION: C51-E01
    pub fn validate_toolchain_contract(contract: ToolchainContract) -> Result<(), ContractError> {
        let edition_floor = contract.edition.minimum_compiler();
        if contract.toolchain < edition_floor {
            return Err(ContractError::ToolchainBeforeEdition);
        }
        if contract.msrv < edition_floor {
            return Err(ContractError::MsrvBeforeEdition);
        }
        if contract.toolchain < contract.msrv {
            return Err(ContractError::ToolchainBeforeMsrv);
        }
        Ok(())
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FeatureProfile {
        Default,
        NoDefault,
        AllFeatures,
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MigrationCell {
        pub compiler: RustVersion,
        pub features: FeatureProfile,
        pub target: String,
    }

    // SOLUTION: C51-E02
    pub fn migration_matrix(
        compilers: &[RustVersion],
        feature_profiles: &[FeatureProfile],
        targets: &[&str],
    ) -> Vec<MigrationCell> {
        let mut cells = Vec::new();
        for &compiler in compilers {
            for &features in feature_profiles {
                for &target in targets {
                    cells.push(MigrationCell {
                        compiler,
                        features,
                        target: target.to_owned(),
                    });
                }
            }
        }
        cells.sort_unstable();
        cells.dedup();
        cells
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ExprGrammar {
        Edition2021Expr,
        Edition2024Expr,
        Edition2024Expr2021,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MacroInput {
        OrdinaryExpression,
        ConstBlock,
        Placeholder,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SelectedMacroArm {
        FirstGeneralExpression,
        LaterConstSpecific,
        LaterPlaceholderSpecific,
    }

    // SOLUTION: C51-E03
    pub const fn first_matching_macro_arm(
        grammar: ExprGrammar,
        input: MacroInput,
    ) -> SelectedMacroArm {
        match (grammar, input) {
            (_, MacroInput::OrdinaryExpression) | (ExprGrammar::Edition2024Expr, _) => {
                SelectedMacroArm::FirstGeneralExpression
            }
            (_, MacroInput::ConstBlock) => SelectedMacroArm::LaterConstSpecific,
            (_, MacroInput::Placeholder) => SelectedMacroArm::LaterPlaceholderSpecific,
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PublicChange {
        RemoveOrRenameItem,
        TightenGenericBound,
        AddEnumVariant { non_exhaustive: bool },
        AddPublicItem,
        LoosenGenericBound,
        AddDefaultedTraitItem,
        RaiseMsrv,
        BehaviorOnlyFix,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SemverImpact {
        Major,
        Minor,
        PossiblyBreaking,
        BehaviorReview,
    }

    // SOLUTION: C51-E04
    pub const fn classify_public_change(change: PublicChange) -> SemverImpact {
        match change {
            PublicChange::RemoveOrRenameItem
            | PublicChange::TightenGenericBound
            | PublicChange::AddEnumVariant {
                non_exhaustive: false,
            } => SemverImpact::Major,
            PublicChange::AddEnumVariant {
                non_exhaustive: true,
            }
            | PublicChange::AddPublicItem
            | PublicChange::LoosenGenericBound => SemverImpact::Minor,
            PublicChange::AddDefaultedTraitItem | PublicChange::RaiseMsrv => {
                SemverImpact::PossiblyBreaking
            }
            PublicChange::BehaviorOnlyFix => SemverImpact::BehaviorReview,
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ToolchainPolicy<'a> {
        pub declared_msrv: RustVersion,
        pub stable_current: RustVersion,
        pub development_toolchain: RustVersion,
        pub ci_toolchains: &'a [RustVersion],
        pub proposed_msrv: Option<RustVersion>,
        pub msrv_raise_documented: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum PolicyIssue {
        DevelopmentBeforeMsrv,
        MsrvMissingFromCi,
        StableMissingFromCi,
        UndocumentedMsrvRaise,
    }

    // SOLUTION: C51-E05
    pub fn audit_toolchain_policy(policy: ToolchainPolicy<'_>) -> Vec<PolicyIssue> {
        let mut issues = Vec::new();
        if policy.development_toolchain < policy.declared_msrv {
            issues.push(PolicyIssue::DevelopmentBeforeMsrv);
        }
        if !policy.ci_toolchains.contains(&policy.declared_msrv) {
            issues.push(PolicyIssue::MsrvMissingFromCi);
        }
        if !policy.ci_toolchains.contains(&policy.stable_current) {
            issues.push(PolicyIssue::StableMissingFromCi);
        }
        if policy
            .proposed_msrv
            .is_some_and(|proposed| proposed > policy.declared_msrv)
            && !policy.msrv_raise_documented
        {
            issues.push(PolicyIssue::UndocumentedMsrvRaise);
        }
        issues.sort_unstable();
        issues
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MigrationPhase {
        BaselinePending,
        BaselineGreen,
        CompatibilityFixesApplied,
        FixesReviewed,
        ManifestSwitched,
        MatrixGreen,
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct MigrationEvidence {
        pub baseline_green: bool,
        pub lints_applied: bool,
        pub fixes_reviewed: bool,
        pub manifest_switched: bool,
        pub matrix_green: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MigrationGateError {
        MissingEvidence(&'static str),
        AlreadyComplete,
    }

    // SOLUTION: C51-E06
    pub const fn advance_migration(
        phase: MigrationPhase,
        evidence: MigrationEvidence,
    ) -> Result<MigrationPhase, MigrationGateError> {
        match phase {
            MigrationPhase::BaselinePending if evidence.baseline_green => {
                Ok(MigrationPhase::BaselineGreen)
            }
            MigrationPhase::BaselinePending => Err(MigrationGateError::MissingEvidence(
                "baseline verde y reproducible",
            )),
            MigrationPhase::BaselineGreen if evidence.lints_applied => {
                Ok(MigrationPhase::CompatibilityFixesApplied)
            }
            MigrationPhase::BaselineGreen => Err(MigrationGateError::MissingEvidence(
                "lints de compatibilidad aplicados",
            )),
            MigrationPhase::CompatibilityFixesApplied if evidence.fixes_reviewed => {
                Ok(MigrationPhase::FixesReviewed)
            }
            MigrationPhase::CompatibilityFixesApplied => Err(MigrationGateError::MissingEvidence(
                "diff automático revisado semánticamente",
            )),
            MigrationPhase::FixesReviewed if evidence.manifest_switched => {
                Ok(MigrationPhase::ManifestSwitched)
            }
            MigrationPhase::FixesReviewed => Err(MigrationGateError::MissingEvidence(
                "edition cambiada en el manifiesto",
            )),
            MigrationPhase::ManifestSwitched if evidence.matrix_green => {
                Ok(MigrationPhase::MatrixGreen)
            }
            MigrationPhase::ManifestSwitched => Err(MigrationGateError::MissingEvidence(
                "matriz completa en verde",
            )),
            MigrationPhase::MatrixGreen => Err(MigrationGateError::AlreadyComplete),
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct PackageVersion {
        pub major: u16,
        pub minor: u16,
        pub patch: u16,
    }

    impl PackageVersion {
        pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
            Self {
                major,
                minor,
                patch,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DependencyRelease {
        pub version: PackageVersion,
        pub rust_version: Option<RustVersion>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ResolverAudit {
        pub newest_proven_compatible: Option<PackageVersion>,
        pub newer_incompatible_releases: usize,
        pub undeclared_rust_versions: usize,
    }

    // SOLUTION: C51-E07
    pub fn audit_dependency_releases(
        project_msrv: RustVersion,
        releases: &[DependencyRelease],
    ) -> ResolverAudit {
        let newest_proven_compatible = releases
            .iter()
            .filter(|release| {
                release
                    .rust_version
                    .is_some_and(|required| required <= project_msrv)
            })
            .map(|release| release.version)
            .max();

        let newer_incompatible_releases = releases
            .iter()
            .filter(|release| {
                release
                    .rust_version
                    .is_some_and(|required| required > project_msrv)
                    && newest_proven_compatible
                        .is_none_or(|compatible| release.version > compatible)
            })
            .count();

        ResolverAudit {
            newest_proven_compatible,
            newer_incompatible_releases,
            undeclared_rust_versions: releases
                .iter()
                .filter(|release| release.rust_version.is_none())
                .count(),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const RUST_185: RustVersion = RustVersion::new(1, 85, 0);
        const RUST_186: RustVersion = RustVersion::new(1, 86, 0);
        const RUST_190: RustVersion = RustVersion::new(1, 90, 0);

        #[test]
        fn edition_toolchain_and_msrv_are_distinct_but_consistent_contracts() {
            assert_eq!(
                validate_toolchain_contract(ToolchainContract {
                    edition: Edition::Rust2024,
                    toolchain: RUST_190,
                    msrv: RUST_185,
                }),
                Ok(()),
            );
            assert_eq!(
                validate_toolchain_contract(ToolchainContract {
                    edition: Edition::Rust2024,
                    toolchain: RUST_190,
                    msrv: RustVersion::new(1, 84, 0),
                }),
                Err(ContractError::MsrvBeforeEdition),
            );
        }

        #[test]
        fn migration_matrix_covers_and_deduplicates_supported_dimensions() {
            let cells = migration_matrix(
                &[RUST_185, RUST_190, RUST_185],
                &[FeatureProfile::Default, FeatureProfile::NoDefault],
                &["x86_64-unknown-linux-gnu", "wasm32-unknown-unknown"],
            );

            assert_eq!(cells.len(), 8);
            assert!(cells.contains(&MigrationCell {
                compiler: RUST_185,
                features: FeatureProfile::NoDefault,
                target: String::from("wasm32-unknown-unknown"),
            }));
        }

        #[test]
        fn expr_2024_can_shadow_later_const_and_placeholder_arms() {
            assert_eq!(
                first_matching_macro_arm(ExprGrammar::Edition2021Expr, MacroInput::ConstBlock,),
                SelectedMacroArm::LaterConstSpecific,
            );
            assert_eq!(
                first_matching_macro_arm(ExprGrammar::Edition2024Expr, MacroInput::ConstBlock,),
                SelectedMacroArm::FirstGeneralExpression,
            );
            assert_eq!(
                first_matching_macro_arm(ExprGrammar::Edition2024Expr2021, MacroInput::Placeholder,),
                SelectedMacroArm::LaterPlaceholderSpecific,
            );
        }

        #[test]
        fn semver_classification_preserves_the_possibly_breaking_category() {
            assert_eq!(
                classify_public_change(PublicChange::TightenGenericBound),
                SemverImpact::Major,
            );
            assert_eq!(
                classify_public_change(PublicChange::AddEnumVariant {
                    non_exhaustive: true,
                }),
                SemverImpact::Minor,
            );
            assert_eq!(
                classify_public_change(PublicChange::RaiseMsrv),
                SemverImpact::PossiblyBreaking,
            );
        }

        #[test]
        fn toolchain_policy_requires_both_ends_of_the_support_window() {
            let issues = audit_toolchain_policy(ToolchainPolicy {
                declared_msrv: RUST_185,
                stable_current: RUST_190,
                development_toolchain: RUST_186,
                ci_toolchains: &[RUST_186],
                proposed_msrv: Some(RUST_186),
                msrv_raise_documented: false,
            });

            assert_eq!(
                issues,
                [
                    PolicyIssue::MsrvMissingFromCi,
                    PolicyIssue::StableMissingFromCi,
                    PolicyIssue::UndocumentedMsrvRaise,
                ],
            );
        }

        #[test]
        fn migration_gate_cannot_skip_semantic_review() {
            let evidence = MigrationEvidence {
                baseline_green: true,
                lints_applied: true,
                fixes_reviewed: false,
                manifest_switched: true,
                matrix_green: true,
            };

            assert_eq!(
                advance_migration(MigrationPhase::BaselinePending, evidence),
                Ok(MigrationPhase::BaselineGreen),
            );
            assert_eq!(
                advance_migration(MigrationPhase::BaselineGreen, evidence),
                Ok(MigrationPhase::CompatibilityFixesApplied),
            );
            assert_eq!(
                advance_migration(MigrationPhase::CompatibilityFixesApplied, evidence),
                Err(MigrationGateError::MissingEvidence(
                    "diff automático revisado semánticamente",
                )),
            );

            let reviewed = MigrationEvidence {
                fixes_reviewed: true,
                ..evidence
            };
            assert_eq!(
                advance_migration(MigrationPhase::CompatibilityFixesApplied, reviewed),
                Ok(MigrationPhase::FixesReviewed),
            );
            assert_eq!(
                advance_migration(MigrationPhase::FixesReviewed, reviewed),
                Ok(MigrationPhase::ManifestSwitched),
            );
            assert_eq!(
                advance_migration(MigrationPhase::ManifestSwitched, reviewed),
                Ok(MigrationPhase::MatrixGreen),
            );
            assert_eq!(
                advance_migration(MigrationPhase::MatrixGreen, reviewed),
                Err(MigrationGateError::AlreadyComplete),
            );
        }

        #[test]
        fn resolver_audit_separates_proof_incompatibility_and_missing_metadata() {
            let audit = audit_dependency_releases(
                RUST_185,
                &[
                    DependencyRelease {
                        version: PackageVersion::new(2, 0, 0),
                        rust_version: Some(RUST_185),
                    },
                    DependencyRelease {
                        version: PackageVersion::new(2, 1, 0),
                        rust_version: Some(RUST_186),
                    },
                    DependencyRelease {
                        version: PackageVersion::new(2, 2, 0),
                        rust_version: None,
                    },
                ],
            );

            assert_eq!(
                audit,
                ResolverAudit {
                    newest_proven_compatible: Some(PackageVersion::new(2, 0, 0)),
                    newer_incompatible_releases: 1,
                    undeclared_rust_versions: 1,
                },
            );
        }
    }
}

pub mod c53 {
    use std::collections::{BTreeSet, HashMap};
    use std::fmt;
    #[cfg(test)]
    use std::rc::Rc;
    use thiserror::Error;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Source {
        Defaults,
        Named(String),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Config {
        values: HashMap<String, String>,
        source: Source,
    }

    #[derive(Clone, Debug, Default)]
    pub struct ConfigBuilder {
        values: HashMap<String, String>,
        source: Option<Source>,
    }

    #[derive(Clone, Debug, Error, Eq, PartialEq)]
    pub enum ConfigError {
        #[error("falta la fuente de configuración")]
        MissingSource,
        #[error("falta la clave {0}")]
        MissingKey(String),
    }

    pub type Result<T> = std::result::Result<T, ConfigError>;

    // SOLUTION: C53-E01
    impl ConfigBuilder {
        pub fn source(mut self, source: Source) -> Self {
            self.source = Some(source);
            self
        }

        pub fn value(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.values.insert(key.into(), value.into());
            self
        }

        pub fn build(self) -> Result<Config> {
            Ok(Config {
                values: self.values,
                source: self.source.ok_or(ConfigError::MissingSource)?,
            })
        }
    }

    impl Config {
        pub fn get(&self, key: &str) -> Result<&str> {
            self.values
                .get(key)
                .map(String::as_str)
                .ok_or_else(|| ConfigError::MissingKey(key.to_owned()))
        }

        pub fn source(&self) -> &Source {
            &self.source
        }
    }

    #[cfg(test)]
    struct DisplayOnly<'a> {
        label: &'a str,
        _not_send_or_sync: Rc<()>,
    }

    #[cfg(test)]
    impl fmt::Display for DisplayOnly<'_> {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(self.label)
        }
    }

    // SOLUTION: C53-E02
    pub fn labels<T: fmt::Display>(items: &[T]) -> Vec<String> {
        items.iter().map(ToString::to_string).collect()
    }

    #[derive(Debug)]
    struct DependencyParseError {
        detail: String,
    }

    impl fmt::Display for DependencyParseError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(&self.detail)
        }
    }

    impl std::error::Error for DependencyParseError {}

    #[derive(Debug, Error)]
    pub enum LibraryError {
        #[error("entrada inválida en la línea {line}")]
        InvalidEntry {
            line: usize,
            #[source]
            source: Box<dyn std::error::Error + Send + Sync>,
        },
    }

    fn dependency_parse_entry(
        input: &str,
    ) -> std::result::Result<(&str, u16), DependencyParseError> {
        let (key, raw_value) = input.split_once('=').ok_or_else(|| DependencyParseError {
            detail: "falta el separador '='".to_owned(),
        })?;
        let value = raw_value.parse().map_err(|error| DependencyParseError {
            detail: format!("valor inválido: {error}"),
        })?;
        Ok((key, value))
    }

    // SOLUTION: C53-E03
    pub fn parse_entry(line: usize, input: &str) -> std::result::Result<(&str, u16), LibraryError> {
        dependency_parse_entry(input).map_err(|source| LibraryError::InvalidEntry {
            line,
            source: Box::new(source),
        })
    }

    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub enum LibraryFeature {
        Serde,
        Tokio,
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct FeatureSet {
        enabled: BTreeSet<LibraryFeature>,
    }

    impl FeatureSet {
        pub fn singleton(feature: LibraryFeature) -> Self {
            Self {
                enabled: BTreeSet::from([feature]),
            }
        }

        pub fn contains(&self, feature: LibraryFeature) -> bool {
            self.enabled.contains(&feature)
        }

        pub fn union(&self, other: &Self) -> Self {
            Self {
                enabled: self.enabled.union(&other.enabled).copied().collect(),
            }
        }
    }

    // SOLUTION: C53-E04
    pub fn feature_matrix(features: &[LibraryFeature]) -> Vec<FeatureSet> {
        let mut cells = vec![FeatureSet::default()];
        cells.extend(features.iter().copied().map(FeatureSet::singleton));
        let all = features
            .iter()
            .copied()
            .fold(FeatureSet::default(), |set, feature| {
                set.union(&FeatureSet::singleton(feature))
            });
        if !cells.contains(&all) {
            cells.push(all);
        }
        cells
    }

    // SOLUTION: C53-E05
    #[doc = include_str!("ch53_quickstart.md")]
    pub struct ReadmeDoctest;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ApiChange {
        AddItem,
        RemoveItem,
        AddEnumVariant { non_exhaustive: bool },
        TightenBound,
        RemoveAutoTrait,
        AddAdditiveFeature,
        EnableDefaultFeature,
        ChangeDocumentedBehavior,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ReleaseImpact {
        Minor,
        Major,
        PossiblyBreaking,
        BehaviorReview,
    }

    // SOLUTION: C53-E06
    pub fn classify_change(change: ApiChange) -> ReleaseImpact {
        match change {
            ApiChange::AddItem | ApiChange::AddAdditiveFeature => ReleaseImpact::Minor,
            ApiChange::RemoveItem
            | ApiChange::TightenBound
            | ApiChange::RemoveAutoTrait
            | ApiChange::AddEnumVariant {
                non_exhaustive: false,
            } => ReleaseImpact::Major,
            ApiChange::AddEnumVariant {
                non_exhaustive: true,
            }
            | ApiChange::EnableDefaultFeature => ReleaseImpact::PossiblyBreaking,
            ApiChange::ChangeDocumentedBehavior => ReleaseImpact::BehaviorReview,
        }
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct ReleaseEvidence {
        pub public_docs: bool,
        pub doctests: bool,
        pub downstream_tests: bool,
        pub msrv_matrix: bool,
        pub feature_matrix: bool,
        pub package_inspected: bool,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ReleaseBlocker {
        PublicDocs,
        Doctests,
        DownstreamTests,
        MsrvMatrix,
        FeatureMatrix,
        PackageInspection,
    }

    // SOLUTION: C53-E07
    pub fn release_blockers(evidence: ReleaseEvidence) -> Vec<ReleaseBlocker> {
        let checks = [
            (evidence.public_docs, ReleaseBlocker::PublicDocs),
            (evidence.doctests, ReleaseBlocker::Doctests),
            (evidence.downstream_tests, ReleaseBlocker::DownstreamTests),
            (evidence.msrv_matrix, ReleaseBlocker::MsrvMatrix),
            (evidence.feature_matrix, ReleaseBlocker::FeatureMatrix),
            (
                evidence.package_inspected,
                ReleaseBlocker::PackageInspection,
            ),
        ];
        checks
            .into_iter()
            .filter_map(|(passed, blocker)| (!passed).then_some(blocker))
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::error::Error as _;

        #[test]
        fn five_type_facade_covers_the_primary_use_case() {
            let config = ConfigBuilder::default()
                .source(Source::Defaults)
                .value("port", "8080")
                .build()
                .unwrap();
            assert_eq!(config.get("port"), Ok("8080"));
            assert_eq!(config.source(), &Source::Defaults);
        }

        #[test]
        fn labels_require_only_display() {
            let local = String::from("prestado");
            let values = [DisplayOnly {
                label: &local,
                _not_send_or_sync: Rc::new(()),
            }];
            assert_eq!(labels(&values), ["prestado"]);
        }

        #[test]
        fn public_error_preserves_a_private_dependency_cause() {
            let error = parse_entry(7, "port=not-a-number").unwrap_err();
            assert_eq!(error.to_string(), "entrada inválida en la línea 7");
            assert!(error.source().is_some());
        }

        #[test]
        fn feature_matrix_covers_absence_individuals_and_union() {
            let matrix = feature_matrix(&[LibraryFeature::Serde, LibraryFeature::Tokio]);
            assert_eq!(matrix.len(), 4);
            assert_eq!(matrix[0], FeatureSet::default());
            let all = matrix.last().unwrap();
            assert!(all.contains(LibraryFeature::Serde));
            assert!(all.contains(LibraryFeature::Tokio));

            let serde = FeatureSet::singleton(LibraryFeature::Serde);
            let tokio = FeatureSet::singleton(LibraryFeature::Tokio);
            let union = serde.union(&tokio);
            assert!(union.contains(LibraryFeature::Serde));
            assert!(union.contains(LibraryFeature::Tokio));
        }

        #[test]
        fn compatibility_classification_keeps_risky_categories_visible() {
            assert_eq!(classify_change(ApiChange::AddItem), ReleaseImpact::Minor);
            assert_eq!(
                classify_change(ApiChange::AddEnumVariant {
                    non_exhaustive: false,
                }),
                ReleaseImpact::Major
            );
            assert_eq!(
                classify_change(ApiChange::EnableDefaultFeature),
                ReleaseImpact::PossiblyBreaking
            );
            assert_eq!(
                classify_change(ApiChange::ChangeDocumentedBehavior),
                ReleaseImpact::BehaviorReview
            );
        }

        #[test]
        fn release_gate_reports_every_missing_piece_of_evidence() {
            let incomplete = ReleaseEvidence {
                public_docs: true,
                doctests: true,
                downstream_tests: false,
                msrv_matrix: true,
                feature_matrix: false,
                package_inspected: true,
            };
            assert_eq!(
                release_blockers(incomplete),
                [
                    ReleaseBlocker::DownstreamTests,
                    ReleaseBlocker::FeatureMatrix
                ]
            );
            assert!(
                release_blockers(ReleaseEvidence {
                    public_docs: true,
                    doctests: true,
                    downstream_tests: true,
                    msrv_matrix: true,
                    feature_matrix: true,
                    package_inspected: true,
                })
                .is_empty()
            );
        }
    }
}

pub mod c54 {
    use std::collections::BTreeSet;
    use thiserror::Error;

    // SOLUTION: C54-E01
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Overwrite {
        Deny,
        Allow,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Recursion {
        Shallow,
        Recursive,
    }

    pub fn copy_policy(overwrite: Overwrite, recursion: Recursion) -> &'static str {
        match (overwrite, recursion) {
            (Overwrite::Deny, Recursion::Shallow) => "safe-shallow",
            (Overwrite::Deny, Recursion::Recursive) => "safe-recursive",
            (Overwrite::Allow, Recursion::Shallow) => "replace-shallow",
            (Overwrite::Allow, Recursion::Recursive) => "replace-recursive",
        }
    }

    #[derive(Debug, Error)]
    pub enum StorageError {
        #[error("valor inválido")]
        InvalidValue(#[from] std::num::ParseIntError),
    }

    #[derive(Debug, Error)]
    pub enum ApplicationError {
        #[error("fallo al cargar la configuración")]
        Load(#[source] StorageError),
    }

    // SOLUTION: C54-E02
    pub fn load_number(input: &str) -> Result<u64, ApplicationError> {
        input
            .parse()
            .map_err(StorageError::from)
            .map_err(ApplicationError::Load)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum InsertOutcome {
        Inserted,
        Duplicate,
    }

    #[derive(Debug, Default)]
    pub struct InMemoryEmailStore {
        emails: BTreeSet<String>,
    }

    // SOLUTION: C54-E03
    impl InMemoryEmailStore {
        pub fn insert_unique(&mut self, email: impl Into<String>) -> InsertOutcome {
            if self.emails.insert(email.into()) {
                InsertOutcome::Inserted
            } else {
                InsertOutcome::Duplicate
            }
        }

        pub fn len(&self) -> usize {
            self.emails.len()
        }

        pub fn is_empty(&self) -> bool {
            self.emails.is_empty()
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct CloneContext {
        pub callee_needs_ownership: bool,
        pub source_used_after_call: bool,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CloneDecision {
        Borrow,
        Move,
        Clone,
    }

    // SOLUTION: C54-E04
    pub fn classify_clone(context: CloneContext) -> CloneDecision {
        match (
            context.callee_needs_ownership,
            context.source_used_after_call,
        ) {
            (false, _) => CloneDecision::Borrow,
            (true, false) => CloneDecision::Move,
            (true, true) => CloneDecision::Clone,
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum RefactorStep {
        Characterize,
        IntroduceTypes,
        AdaptInternals,
        MigrateCallers,
        RetireLegacyApi,
        DocumentAndMeasure,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct RefactorCommit {
        pub step: RefactorStep,
        pub green: bool,
        pub intentions: u8,
    }

    // SOLUTION: C54-E05
    pub fn reversible_plan() -> [RefactorCommit; 6] {
        [
            RefactorStep::Characterize,
            RefactorStep::IntroduceTypes,
            RefactorStep::AdaptInternals,
            RefactorStep::MigrateCallers,
            RefactorStep::RetireLegacyApi,
            RefactorStep::DocumentAndMeasure,
        ]
        .map(|step| RefactorCommit {
            step,
            green: true,
            intentions: 1,
        })
    }

    pub fn is_reversible_plan(commits: &[RefactorCommit]) -> bool {
        let expected = [
            RefactorStep::Characterize,
            RefactorStep::IntroduceTypes,
            RefactorStep::AdaptInternals,
            RefactorStep::MigrateCallers,
            RefactorStep::RetireLegacyApi,
            RefactorStep::DocumentAndMeasure,
        ];
        commits.len() == expected.len()
            && commits
                .iter()
                .zip(expected)
                .all(|(commit, step)| commit.step == step && commit.green && commit.intentions == 1)
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum RegistrationEvent {
        RejectedInvalid,
        RejectedDuplicate,
        Persisted(String),
        Published(String),
    }

    // SOLUTION: C54-E06
    pub fn characterize_registration(email: &str, already_exists: bool) -> Vec<RegistrationEvent> {
        if !email.contains('@') {
            return vec![RegistrationEvent::RejectedInvalid];
        }
        if already_exists {
            return vec![RegistrationEvent::RejectedDuplicate];
        }
        vec![
            RegistrationEvent::Persisted(email.to_owned()),
            RegistrationEvent::Published(email.to_owned()),
        ]
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct RefactorEvidence {
        pub behavior_preserved: bool,
        pub public_api_reviewed: bool,
        pub concurrency_reviewed: bool,
        pub msrv_green: bool,
        pub reversible_commit: bool,
        pub measurement_required: bool,
        pub measurement_passed: bool,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum RefactorBlocker {
        Behavior,
        PublicApi,
        Concurrency,
        Msrv,
        Reversibility,
        Measurement,
    }

    // SOLUTION: C54-E07
    pub fn refactor_blockers(evidence: RefactorEvidence) -> Vec<RefactorBlocker> {
        let mut blockers = Vec::new();
        if !evidence.behavior_preserved {
            blockers.push(RefactorBlocker::Behavior);
        }
        if !evidence.public_api_reviewed {
            blockers.push(RefactorBlocker::PublicApi);
        }
        if !evidence.concurrency_reviewed {
            blockers.push(RefactorBlocker::Concurrency);
        }
        if !evidence.msrv_green {
            blockers.push(RefactorBlocker::Msrv);
        }
        if !evidence.reversible_commit {
            blockers.push(RefactorBlocker::Reversibility);
        }
        if evidence.measurement_required && !evidence.measurement_passed {
            blockers.push(RefactorBlocker::Measurement);
        }
        blockers
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::error::Error;

        #[test]
        fn enums_make_call_sites_self_describing() {
            assert_eq!(
                copy_policy(Overwrite::Deny, Recursion::Recursive),
                "safe-recursive"
            );
        }

        #[test]
        fn layered_error_preserves_its_source() {
            let error = load_number("not-a-number").unwrap_err();
            assert!(error.source().is_some());
        }

        #[test]
        fn atomic_insert_contract_rejects_the_second_attempt() {
            let mut store = InMemoryEmailStore::default();
            assert_eq!(
                store.insert_unique("ada@example.test"),
                InsertOutcome::Inserted
            );
            assert_eq!(
                store.insert_unique("ada@example.test"),
                InsertOutcome::Duplicate
            );
            assert_eq!(store.len(), 1);
        }

        #[test]
        fn ten_clone_sites_are_classified_from_ownership_needs() {
            let sites = [
                (false, false, CloneDecision::Borrow),
                (false, true, CloneDecision::Borrow),
                (false, false, CloneDecision::Borrow),
                (true, false, CloneDecision::Move),
                (true, false, CloneDecision::Move),
                (true, false, CloneDecision::Move),
                (true, true, CloneDecision::Clone),
                (true, true, CloneDecision::Clone),
                (true, true, CloneDecision::Clone),
                (true, true, CloneDecision::Clone),
            ];
            for (callee_needs_ownership, source_used_after_call, expected) in sites {
                assert_eq!(
                    classify_clone(CloneContext {
                        callee_needs_ownership,
                        source_used_after_call,
                    }),
                    expected
                );
            }
        }

        #[test]
        fn six_commit_plan_is_ordered_green_and_single_purpose() {
            let plan = reversible_plan();
            assert!(is_reversible_plan(&plan));

            let mut invalid = plan;
            invalid[2].green = false;
            assert!(!is_reversible_plan(&invalid));
        }

        #[test]
        fn characterization_covers_success_invalid_and_duplicate() {
            assert_eq!(
                characterize_registration("invalid", false),
                [RegistrationEvent::RejectedInvalid]
            );
            assert_eq!(
                characterize_registration("ada@example.test", true),
                [RegistrationEvent::RejectedDuplicate]
            );
            assert_eq!(
                characterize_registration("ada@example.test", false),
                [
                    RegistrationEvent::Persisted("ada@example.test".to_owned()),
                    RegistrationEvent::Published("ada@example.test".to_owned())
                ]
            );
        }

        #[test]
        fn refactor_gate_requires_measurement_only_for_a_cost_claim() {
            let missing = RefactorEvidence {
                behavior_preserved: true,
                public_api_reviewed: false,
                concurrency_reviewed: true,
                msrv_green: true,
                reversible_commit: true,
                measurement_required: true,
                measurement_passed: false,
            };
            assert_eq!(
                refactor_blockers(missing),
                [RefactorBlocker::PublicApi, RefactorBlocker::Measurement]
            );

            let complete = RefactorEvidence {
                public_api_reviewed: true,
                measurement_passed: true,
                ..missing
            };
            assert!(refactor_blockers(complete).is_empty());
        }
    }
}

pub mod c55 {
    use std::collections::BTreeSet;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Visibility {
        Public,
        Crate,
        Private,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ApiItem {
        pub path: &'static str,
        pub visibility: Visibility,
        pub required_feature: Option<&'static str>,
    }

    // SOLUTION: C55-E01
    pub fn effective_facade(
        items: &[ApiItem],
        enabled_features: &BTreeSet<&str>,
    ) -> Vec<&'static str> {
        items
            .iter()
            .filter(|item| item.visibility == Visibility::Public)
            .filter(|item| {
                item.required_feature
                    .is_none_or(|feature| enabled_features.contains(feature))
            })
            .map(|item| item.path)
            .collect()
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum TraceEvent {
        Borrow,
        Move,
        Clone,
        Allocation,
        DynamicDispatch,
        Effect,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Evidence {
        Signature,
        Source,
        Measurement,
        Inference,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct TraceObservation {
        pub event: TraceEvent,
        pub evidence: Evidence,
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct TraceSummary {
        pub moves: usize,
        pub clones: usize,
        pub allocations: usize,
        pub inferred_cost_claims: usize,
    }

    // SOLUTION: C55-E02
    pub fn summarize_trace(observations: &[TraceObservation]) -> TraceSummary {
        observations
            .iter()
            .fold(TraceSummary::default(), |mut summary, observation| {
                match observation.event {
                    TraceEvent::Move => summary.moves += 1,
                    TraceEvent::Clone => summary.clones += 1,
                    TraceEvent::Allocation => summary.allocations += 1,
                    TraceEvent::Borrow | TraceEvent::DynamicDispatch | TraceEvent::Effect => {}
                }
                if observation.evidence == Evidence::Inference
                    && matches!(
                        observation.event,
                        TraceEvent::Clone | TraceEvent::Allocation | TraceEvent::DynamicDispatch
                    )
                {
                    summary.inferred_cost_claims += 1;
                }
                summary
            })
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum PremiseStatus {
        Proven,
        Pending,
        Contradicted,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct SafetyPremise {
        pub name: &'static str,
        pub status: PremiseStatus,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum UnsafeAudit {
        Verified,
        Incomplete { pending: usize },
        Blocked { contradicted: usize },
    }

    // SOLUTION: C55-E03
    pub fn audit_unsafe(premises: &[SafetyPremise]) -> UnsafeAudit {
        let contradicted = premises
            .iter()
            .filter(|premise| premise.status == PremiseStatus::Contradicted)
            .count();
        if contradicted > 0 {
            return UnsafeAudit::Blocked { contradicted };
        }
        let pending = premises
            .iter()
            .filter(|premise| premise.status == PremiseStatus::Pending)
            .count();
        if pending > 0 {
            UnsafeAudit::Incomplete { pending }
        } else {
            UnsafeAudit::Verified
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ShutdownEvent {
        CloseAdmission,
        DrainAccepted,
        AbortRemaining,
        JoinAll,
        Report,
    }

    // SOLUTION: C55-E04
    pub fn valid_shutdown_trace(events: &[ShutdownEvent]) -> bool {
        let position = |event| events.iter().position(|candidate| *candidate == event);
        let (Some(close), Some(drain), Some(join), Some(report)) = (
            position(ShutdownEvent::CloseAdmission),
            position(ShutdownEvent::DrainAccepted),
            position(ShutdownEvent::JoinAll),
            position(ShutdownEvent::Report),
        ) else {
            return false;
        };
        if !(close < drain && drain < join && join < report) {
            return false;
        }
        position(ShutdownEvent::AbortRemaining).is_none_or(|abort| drain < abort && abort < join)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum DedupInsert {
        Inserted,
        Duplicate,
        Full,
    }

    #[derive(Clone, Debug)]
    pub struct BoundedDeduper<T> {
        capacity: usize,
        values: BTreeSet<T>,
    }

    // SOLUTION: C55-E05
    impl<T: Ord> BoundedDeduper<T> {
        pub fn new(capacity: usize) -> Self {
            Self {
                capacity,
                values: BTreeSet::new(),
            }
        }

        pub fn insert(&mut self, value: T) -> DedupInsert {
            if self.values.contains(&value) {
                return DedupInsert::Duplicate;
            }
            if self.values.len() == self.capacity {
                return DedupInsert::Full;
            }
            self.values.insert(value);
            DedupInsert::Inserted
        }

        pub fn len(&self) -> usize {
            self.values.len()
        }

        pub fn is_empty(&self) -> bool {
            self.values.is_empty()
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum NoteOrigin {
        ExecutedCommand,
        PinnedSourceLine,
        PublicDocumentation,
        DerivedFromImplementation,
        IssueOrCommit,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum EvidenceKind {
        Fact,
        PublicContract,
        Inference,
        HistoricalContext,
    }

    // SOLUTION: C55-E06
    pub fn classify_note(origin: NoteOrigin) -> EvidenceKind {
        match origin {
            NoteOrigin::ExecutedCommand | NoteOrigin::PinnedSourceLine => EvidenceKind::Fact,
            NoteOrigin::PublicDocumentation => EvidenceKind::PublicContract,
            NoteOrigin::DerivedFromImplementation => EvidenceKind::Inference,
            NoteOrigin::IssueOrCommit => EvidenceKind::HistoricalContext,
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ResearchQuestion {
        PublicApi,
        FeatureOrigin,
        UnsafeInvariant,
        AsyncShutdown,
        Performance,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ReadingStep {
        FixRevision,
        ReadDocs,
        ReadManifest,
        ReadFacade,
        InspectMetadata,
        InspectDependencyTree,
        TraceVerticalFlow,
        ReadTests,
        AuditUnsafe,
        ReadHistory,
        RunBenchmark,
    }

    // SOLUTION: C55-E07
    pub fn reading_plan(question: ResearchQuestion) -> Vec<ReadingStep> {
        use ReadingStep::{
            AuditUnsafe, FixRevision, InspectDependencyTree, InspectMetadata, ReadDocs, ReadFacade,
            ReadHistory, ReadManifest, ReadTests, RunBenchmark, TraceVerticalFlow,
        };
        let mut plan = vec![FixRevision];
        match question {
            ResearchQuestion::PublicApi => {
                plan.extend([ReadDocs, ReadFacade, TraceVerticalFlow, ReadTests]);
            }
            ResearchQuestion::FeatureOrigin => {
                plan.extend([
                    ReadManifest,
                    InspectMetadata,
                    InspectDependencyTree,
                    TraceVerticalFlow,
                ]);
            }
            ResearchQuestion::UnsafeInvariant => {
                plan.extend([
                    ReadFacade,
                    TraceVerticalFlow,
                    AuditUnsafe,
                    ReadTests,
                    ReadHistory,
                ]);
            }
            ResearchQuestion::AsyncShutdown => {
                plan.extend([ReadDocs, TraceVerticalFlow, ReadTests, ReadHistory]);
            }
            ResearchQuestion::Performance => {
                plan.extend([ReadDocs, TraceVerticalFlow, RunBenchmark, ReadHistory]);
            }
        }
        plan
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn facade_contains_only_public_items_enabled_for_the_configuration() {
            let items = [
                ApiItem {
                    path: "crate::Client",
                    visibility: Visibility::Public,
                    required_feature: None,
                },
                ApiItem {
                    path: "crate::serde::Document",
                    visibility: Visibility::Public,
                    required_feature: Some("serde"),
                },
                ApiItem {
                    path: "crate::internal::Parser",
                    visibility: Visibility::Crate,
                    required_feature: None,
                },
            ];
            assert_eq!(
                effective_facade(&items, &BTreeSet::new()),
                ["crate::Client"]
            );
            assert_eq!(
                effective_facade(&items, &BTreeSet::from(["serde"])),
                ["crate::Client", "crate::serde::Document"]
            );
        }

        #[test]
        fn trace_summary_keeps_inferences_visible() {
            let summary = summarize_trace(&[
                TraceObservation {
                    event: TraceEvent::Move,
                    evidence: Evidence::Signature,
                },
                TraceObservation {
                    event: TraceEvent::Clone,
                    evidence: Evidence::Source,
                },
                TraceObservation {
                    event: TraceEvent::Allocation,
                    evidence: Evidence::Measurement,
                },
                TraceObservation {
                    event: TraceEvent::Allocation,
                    evidence: Evidence::Inference,
                },
            ]);
            assert_eq!(
                summary,
                TraceSummary {
                    moves: 1,
                    clones: 1,
                    allocations: 2,
                    inferred_cost_claims: 1,
                }
            );
        }

        #[test]
        fn unsafe_audit_distinguishes_pending_from_contradicted() {
            assert_eq!(
                audit_unsafe(&[
                    SafetyPremise {
                        name: "aligned",
                        status: PremiseStatus::Proven,
                    },
                    SafetyPremise {
                        name: "initialized",
                        status: PremiseStatus::Pending,
                    },
                ]),
                UnsafeAudit::Incomplete { pending: 1 }
            );
            assert_eq!(
                audit_unsafe(&[SafetyPremise {
                    name: "exclusive",
                    status: PremiseStatus::Contradicted,
                }]),
                UnsafeAudit::Blocked { contradicted: 1 }
            );
        }

        #[test]
        fn shutdown_closes_drains_optionally_aborts_and_then_joins() {
            assert!(valid_shutdown_trace(&[
                ShutdownEvent::CloseAdmission,
                ShutdownEvent::DrainAccepted,
                ShutdownEvent::AbortRemaining,
                ShutdownEvent::JoinAll,
                ShutdownEvent::Report,
            ]));
            assert!(!valid_shutdown_trace(&[
                ShutdownEvent::CloseAdmission,
                ShutdownEvent::AbortRemaining,
                ShutdownEvent::DrainAccepted,
                ShutdownEvent::JoinAll,
                ShutdownEvent::Report,
            ]));
        }

        #[test]
        fn bounded_deduper_reconstructs_success_duplicate_and_pressure() {
            let mut values = BoundedDeduper::new(2);
            assert_eq!(values.insert("a"), DedupInsert::Inserted);
            assert_eq!(values.insert("a"), DedupInsert::Duplicate);
            assert_eq!(values.insert("b"), DedupInsert::Inserted);
            assert_eq!(values.insert("c"), DedupInsert::Full);
            assert_eq!(values.len(), 2);
        }

        #[test]
        fn ten_notes_keep_fact_contract_inference_and_history_separate() {
            let origins = [
                NoteOrigin::ExecutedCommand,
                NoteOrigin::PinnedSourceLine,
                NoteOrigin::PublicDocumentation,
                NoteOrigin::DerivedFromImplementation,
                NoteOrigin::IssueOrCommit,
                NoteOrigin::ExecutedCommand,
                NoteOrigin::PublicDocumentation,
                NoteOrigin::DerivedFromImplementation,
                NoteOrigin::PinnedSourceLine,
                NoteOrigin::IssueOrCommit,
            ];
            let kinds: Vec<_> = origins.into_iter().map(classify_note).collect();
            assert_eq!(
                kinds,
                [
                    EvidenceKind::Fact,
                    EvidenceKind::Fact,
                    EvidenceKind::PublicContract,
                    EvidenceKind::Inference,
                    EvidenceKind::HistoricalContext,
                    EvidenceKind::Fact,
                    EvidenceKind::PublicContract,
                    EvidenceKind::Inference,
                    EvidenceKind::Fact,
                    EvidenceKind::HistoricalContext,
                ]
            );
        }

        #[test]
        fn reading_plan_is_driven_by_the_question() {
            let features = reading_plan(ResearchQuestion::FeatureOrigin);
            assert_eq!(features[0], ReadingStep::FixRevision);
            assert!(features.contains(&ReadingStep::ReadManifest));
            assert!(features.contains(&ReadingStep::InspectDependencyTree));
            assert!(!features.contains(&ReadingStep::RunBenchmark));

            let unsafe_plan = reading_plan(ResearchQuestion::UnsafeInvariant);
            assert!(unsafe_plan.contains(&ReadingStep::AuditUnsafe));
            assert!(unsafe_plan.contains(&ReadingStep::ReadHistory));
        }
    }
}
