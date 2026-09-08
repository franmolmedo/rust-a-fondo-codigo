//! Executable reference solutions for the fourteen katas in chapter 56.

pub mod k01 {
    #[derive(Debug, Eq, PartialEq)]
    pub struct Job {
        pub id: u64,
        pub ready: bool,
    }

    // SOLUTION: C56-K01
    /// Moves ready jobs into a new vector while preserving the relative order
    /// of both partitions.
    ///
    /// The operation is O(n), does not require `Job: Clone`, and replaces the
    /// queue, so the original allocation and capacity are not guaranteed to remain.
    pub fn drain_ready(queue: &mut Vec<Job>) -> Vec<Job> {
        let (ready, pending): (Vec<_>, Vec<_>) =
            std::mem::take(queue).into_iter().partition(|job| job.ready);
        *queue = pending;
        ready
    }

    pub fn drain_ready_unordered(queue: &mut Vec<Job>) -> Vec<Job> {
        let mut ready = Vec::new();
        let mut index = 0;
        while index < queue.len() {
            if queue[index].ready {
                ready.push(queue.swap_remove(index));
            } else {
                index += 1;
            }
        }
        ready
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn stable_variant_moves_without_cloning_and_preserves_order() {
            let mut queue = vec![
                Job {
                    id: 1,
                    ready: false,
                },
                Job { id: 2, ready: true },
                Job {
                    id: 3,
                    ready: false,
                },
                Job { id: 4, ready: true },
            ];
            let ready = drain_ready(&mut queue);
            assert_eq!(ready.iter().map(|job| job.id).collect::<Vec<_>>(), [2, 4]);
            assert_eq!(queue.iter().map(|job| job.id).collect::<Vec<_>>(), [1, 3]);
        }

        #[test]
        fn stable_variant_handles_empty_none_and_all_ready() {
            let mut empty = Vec::new();
            assert!(drain_ready(&mut empty).is_empty());

            let mut pending = vec![Job {
                id: 1,
                ready: false,
            }];
            assert!(drain_ready(&mut pending).is_empty());
            assert_eq!(pending[0].id, 1);

            let mut ready = vec![Job { id: 2, ready: true }, Job { id: 3, ready: true }];
            assert_eq!(
                drain_ready(&mut ready)
                    .into_iter()
                    .map(|job| job.id)
                    .collect::<Vec<_>>(),
                [2, 3]
            );
            assert!(ready.is_empty());

            let mut duplicate_ids = vec![
                Job { id: 7, ready: true },
                Job {
                    id: 7,
                    ready: false,
                },
            ];
            assert_eq!(drain_ready(&mut duplicate_ids)[0].id, 7);
            assert_eq!(duplicate_ids[0].id, 7);
        }

        #[test]
        fn unordered_variant_preserves_membership_not_order() {
            let mut queue = vec![
                Job {
                    id: 1,
                    ready: false,
                },
                Job { id: 2, ready: true },
                Job {
                    id: 3,
                    ready: false,
                },
                Job { id: 4, ready: true },
            ];
            let mut ready_ids = drain_ready_unordered(&mut queue)
                .into_iter()
                .map(|job| job.id)
                .collect::<Vec<_>>();
            let mut pending_ids = queue.into_iter().map(|job| job.id).collect::<Vec<_>>();
            ready_ids.sort_unstable();
            pending_ids.sort_unstable();
            assert_eq!(ready_ids, [2, 4]);
            assert_eq!(pending_ids, [1, 3]);
        }
    }
}

pub mod k02 {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct User {
        pub name: String,
    }

    // SOLUTION: C56-K02
    /// Returns borrowed names that match `prefix`.
    ///
    /// The output cannot outlive `users`:
    ///
    /// ```compile_fail
    /// use course_solutions::katas::k02::{matching_names, User};
    ///
    /// let names = {
    ///     let users = vec![User { name: "Ada".into() }];
    ///     matching_names(&users, "A")
    /// };
    /// assert_eq!(names, ["Ada"]);
    /// ```
    pub fn matching_names<'a>(users: &'a [User], prefix: &str) -> Vec<&'a str> {
        users
            .iter()
            .map(|user| user.name.as_str())
            .filter(|name| name.starts_with(prefix))
            .collect()
    }

    pub fn matching_names_lazy<'users, 'prefix>(
        users: &'users [User],
        prefix: &'prefix str,
    ) -> impl Iterator<Item = &'users str> + use<'users, 'prefix> {
        users
            .iter()
            .map(|user| user.name.as_str())
            .filter(move |name| name.starts_with(prefix))
    }

    pub fn matching_names_owned(users: &[User], prefix: &str) -> Vec<String> {
        users
            .iter()
            .map(|user| user.name.as_str())
            .filter(|name| name.starts_with(prefix))
            .map(str::to_owned)
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn borrowed_and_owned_outputs_have_distinct_lifecycles() {
            let users = vec![
                User {
                    name: String::from("Ada"),
                },
                User {
                    name: String::from("Alan"),
                },
                User {
                    name: String::from("Grace"),
                },
            ];
            assert_eq!(matching_names(&users, "A"), ["Ada", "Alan"]);
            assert_eq!(
                matching_names_lazy(&users, "G").collect::<Vec<_>>(),
                ["Grace"]
            );
            assert_eq!(matching_names_owned(&users, "A"), ["Ada", "Alan"]);
            assert_eq!(matching_names(&users, ""), ["Ada", "Alan", "Grace"]);
            assert!(matching_names(&users, "Linus").is_empty());
        }

        #[test]
        fn lazy_prefix_can_live_less_than_the_users() {
            let users = vec![
                User {
                    name: String::from("Ada"),
                },
                User {
                    name: String::from("Grace"),
                },
            ];
            {
                let prefix = String::from("A");
                assert_eq!(
                    matching_names_lazy(&users, &prefix).collect::<Vec<_>>(),
                    ["Ada"]
                );
            }
            assert_eq!(users.len(), 2);
        }

        #[test]
        fn owned_result_outlives_both_inputs() {
            let owned = {
                let users = vec![User {
                    name: String::from("Ada"),
                }];
                let prefix = String::from("A");
                matching_names_owned(&users, &prefix)
            };
            assert_eq!(owned, ["Ada"]);
        }

        #[test]
        fn unicode_prefixes_follow_string_prefix_semantics() {
            let users = vec![
                User {
                    name: String::from("Álvaro"),
                },
                User {
                    name: String::from("Ada"),
                },
            ];
            assert_eq!(matching_names(&users, "Ál"), ["Álvaro"]);
        }
    }
}

pub mod k03 {
    #[derive(Debug, Eq, PartialEq)]
    pub struct Account {
        pub balance: u64,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Money(pub u64);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum TransferError {
        SameAccount,
        OutOfRange,
        InsufficientFunds,
        Overflow,
    }

    // SOLUTION: C56-K03
    pub fn transfer(
        accounts: &mut [Account],
        from: usize,
        to: usize,
        amount: Money,
    ) -> Result<(), TransferError> {
        if from == to {
            return Err(TransferError::SameAccount);
        }
        if from >= accounts.len() || to >= accounts.len() {
            return Err(TransferError::OutOfRange);
        }
        let (source, destination) = if from < to {
            let (left, right) = accounts.split_at_mut(to);
            (&mut left[from], &mut right[0])
        } else {
            let (left, right) = accounts.split_at_mut(from);
            (&mut right[0], &mut left[to])
        };
        let source_after = source
            .balance
            .checked_sub(amount.0)
            .ok_or(TransferError::InsufficientFunds)?;
        let destination_after = destination
            .balance
            .checked_add(amount.0)
            .ok_or(TransferError::Overflow)?;
        source.balance = source_after;
        destination.balance = destination_after;
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn total(accounts: &[Account]) -> u128 {
            accounts
                .iter()
                .map(|account| u128::from(account.balance))
                .sum()
        }

        #[test]
        fn failed_transfer_changes_neither_account() {
            let mut accounts = [Account { balance: 10 }, Account { balance: 20 }];
            let initial_total = total(&accounts);
            assert_eq!(
                transfer(&mut accounts, 0, 1, Money(11)),
                Err(TransferError::InsufficientFunds)
            );
            assert_eq!(accounts, [Account { balance: 10 }, Account { balance: 20 }]);
            assert_eq!(total(&accounts), initial_total);
            transfer(&mut accounts, 1, 0, Money(5)).unwrap();
            assert_eq!(accounts, [Account { balance: 15 }, Account { balance: 15 }]);
            assert_eq!(total(&accounts), initial_total);

            transfer(&mut accounts, 0, 1, Money(15)).unwrap();
            assert_eq!(accounts, [Account { balance: 0 }, Account { balance: 30 }]);
            assert_eq!(total(&accounts), initial_total);
        }

        #[test]
        fn invalid_indices_and_same_account_are_rejected_without_changes() {
            let mut accounts = [Account { balance: 10 }, Account { balance: 20 }];
            let initial_total = total(&accounts);
            assert_eq!(
                transfer(&mut accounts, 0, 0, Money(1)),
                Err(TransferError::SameAccount)
            );
            assert_eq!(
                transfer(&mut accounts, 2, 0, Money(1)),
                Err(TransferError::OutOfRange)
            );
            assert_eq!(
                transfer(&mut accounts, 0, 2, Money(1)),
                Err(TransferError::OutOfRange)
            );
            assert_eq!(
                transfer(&mut accounts, 2, 3, Money(1)),
                Err(TransferError::OutOfRange)
            );
            assert_eq!(accounts, [Account { balance: 10 }, Account { balance: 20 }]);

            transfer(&mut accounts, 0, 1, Money(0)).unwrap();
            assert_eq!(accounts, [Account { balance: 10 }, Account { balance: 20 }]);
            assert_eq!(total(&accounts), initial_total);
        }

        #[test]
        fn destination_overflow_preserves_both_balances() {
            let mut accounts = [Account { balance: 1 }, Account { balance: u64::MAX }];
            let initial_total = total(&accounts);
            assert_eq!(
                transfer(&mut accounts, 0, 1, Money(1)),
                Err(TransferError::Overflow)
            );
            assert_eq!(
                accounts,
                [Account { balance: 1 }, Account { balance: u64::MAX }]
            );
            assert_eq!(total(&accounts), initial_total);
        }
    }
}

pub mod k04 {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Verb {
        Get,
        Set,
        Delete,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Command<'a> {
        pub verb: Verb,
        pub arguments: Vec<&'a str>,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ParseError<'a> {
        Empty,
        UnknownVerb { position: usize, verb: &'a str },
    }

    // SOLUTION: C56-K04
    /// Parses a command without copying its argument tokens.
    ///
    /// Error positions are byte offsets into `input`, matching Rust string
    /// slicing conventions rather than character or display-column indices.
    pub fn parse_command(input: &str) -> Result<Command<'_>, ParseError<'_>> {
        let mut tokens = input.split_whitespace();
        let raw_verb = tokens.next().ok_or(ParseError::Empty)?;
        let verb = match raw_verb {
            "get" => Verb::Get,
            "set" => Verb::Set,
            "delete" => Verb::Delete,
            unknown => {
                return Err(ParseError::UnknownVerb {
                    position: input.find(unknown).unwrap_or(0),
                    verb: unknown,
                });
            }
        };
        Ok(Command {
            verb,
            arguments: tokens.collect(),
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn command_borrows_tokens_and_error_keeps_position() {
            let input = String::from("set language Rust");
            let command = parse_command(&input).unwrap();
            assert_eq!(command.arguments, ["language", "Rust"]);
            assert_eq!(parse_command("get").unwrap().verb, Verb::Get);
            assert_eq!(parse_command("delete key").unwrap().verb, Verb::Delete);
            assert_eq!(
                parse_command("  unknown value"),
                Err(ParseError::UnknownVerb {
                    position: 2,
                    verb: "unknown"
                })
            );
        }

        #[test]
        fn empty_and_unicode_arguments_have_explicit_semantics() {
            assert_eq!(parse_command("   "), Err(ParseError::Empty));
            let command = parse_command("set label 🦀").unwrap();
            assert_eq!(command.verb, Verb::Set);
            assert_eq!(command.arguments, ["label", "🦀"]);
            assert_eq!(
                parse_command("\u{2003}unknown value"),
                Err(ParseError::UnknownVerb {
                    position: 3,
                    verb: "unknown"
                })
            );
        }
    }
}

pub mod k05 {
    use std::io::{self, Read};
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum ParseConfigError {
        #[error("invalid port {value:?}")]
        InvalidPort {
            value: String,
            #[source]
            source: std::num::ParseIntError,
        },
    }

    #[derive(Debug, Error)]
    pub enum LoadConfigError {
        #[error("failed to read configuration")]
        Io(#[from] io::Error),
        #[error("failed to parse configuration")]
        Parse(#[from] ParseConfigError),
    }

    #[derive(Clone, Debug, Error, Eq, PartialEq)]
    pub enum ApplyConfigError {
        #[error("the port is reserved")]
        ReservedPort,
    }

    #[derive(Debug, Error)]
    pub enum ConfigurePortError {
        #[error(transparent)]
        Load(#[from] LoadConfigError),
        #[error(transparent)]
        Apply(#[from] ApplyConfigError),
    }

    // SOLUTION: C56-K05
    pub fn parse_port(input: &str) -> Result<u16, ParseConfigError> {
        let value = input.trim();
        value
            .parse()
            .map_err(|source| ParseConfigError::InvalidPort {
                value: value.to_owned(),
                source,
            })
    }

    pub fn load_port(mut reader: impl Read) -> Result<u16, LoadConfigError> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        Ok(parse_port(&input)?)
    }

    pub fn apply_port(port: u16) -> Result<u16, ApplyConfigError> {
        if port < 1024 {
            return Err(ApplyConfigError::ReservedPort);
        }
        Ok(port)
    }

    pub fn configure_port(reader: impl Read) -> Result<u16, ConfigurePortError> {
        let port = load_port(reader)?;
        Ok(apply_port(port)?)
    }

    pub fn exit_code(error: &ConfigurePortError) -> u8 {
        match error {
            ConfigurePortError::Load(LoadConfigError::Io(_)) => 74,
            ConfigurePortError::Load(LoadConfigError::Parse(_)) => 65,
            ConfigurePortError::Apply(ApplyConfigError::ReservedPort) => 78,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::error::Error;

        #[test]
        fn errors_are_decided_by_variants_not_messages() {
            let parse = parse_port("not-a-port").unwrap_err();
            assert!(matches!(
                &parse,
                ParseConfigError::InvalidPort { value, .. } if value == "not-a-port"
            ));
            let load = LoadConfigError::from(parse);
            assert!(matches!(load, LoadConfigError::Parse(_)));
            assert!(load.source().is_some());
            assert_eq!(apply_port(80), Err(ApplyConfigError::ReservedPort));
        }

        struct FailingReader;

        impl Read for FailingReader {
            fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "fixture denied",
                ))
            }
        }

        #[test]
        fn loading_preserves_io_and_parse_causes() {
            let io_error = load_port(FailingReader).unwrap_err();
            assert!(matches!(io_error, LoadConfigError::Io(_)));
            assert_eq!(
                io_error
                    .source()
                    .and_then(|source| source.downcast_ref::<io::Error>())
                    .map(io::Error::kind),
                Some(io::ErrorKind::PermissionDenied)
            );

            let parse_error = load_port("not-a-port".as_bytes()).unwrap_err();
            assert!(matches!(parse_error, LoadConfigError::Parse(_)));
            let parse_source = parse_error.source().expect("parse source");
            assert!(parse_source.source().is_some());
        }

        #[test]
        fn valid_loading_and_domain_application_are_separate() {
            assert_eq!(load_port("8080".as_bytes()).unwrap(), 8080);
            assert_eq!(apply_port(8080), Ok(8080));
            assert_eq!(apply_port(443), Err(ApplyConfigError::ReservedPort));
            assert_eq!(configure_port("8080".as_bytes()).unwrap(), 8080);

            let parse = configure_port("invalid".as_bytes()).unwrap_err();
            assert_eq!(exit_code(&parse), 65);
            let domain = configure_port("443".as_bytes()).unwrap_err();
            assert_eq!(exit_code(&domain), 78);
            let io = configure_port(FailingReader).unwrap_err();
            assert_eq!(exit_code(&io), 74);
        }
    }
}

pub mod k06 {
    use std::collections::HashMap;

    pub trait Clock {
        fn now(&self) -> u64;
    }

    pub trait IdGenerator {
        fn next_id(&mut self) -> Option<u64>;
    }

    pub trait Repository {
        type Error;

        fn save(&mut self, id: u64, created_at: u64) -> Result<(), Self::Error>;
    }

    #[derive(Debug, Eq, PartialEq, thiserror::Error)]
    pub enum CreateError<E> {
        #[error("identifier sequence exhausted")]
        IdsExhausted,
        #[error("repository failed: {0}")]
        Repository(#[source] E),
    }

    // SOLUTION: C56-K06
    pub fn create<R, C, I>(
        repository: &mut R,
        clock: &C,
        ids: &mut I,
    ) -> Result<u64, CreateError<R::Error>>
    where
        R: Repository,
        C: Clock,
        I: IdGenerator,
    {
        let id = ids.next_id().ok_or(CreateError::IdsExhausted)?;
        repository
            .save(id, clock.now())
            .map_err(CreateError::Repository)?;
        Ok(id)
    }

    pub fn create_with_id_fn<R, C, F>(
        repository: &mut R,
        clock: &C,
        next_id: &mut F,
    ) -> Result<u64, CreateError<R::Error>>
    where
        R: Repository,
        C: Clock,
        F: FnMut() -> Option<u64>,
    {
        let id = next_id().ok_or(CreateError::IdsExhausted)?;
        repository
            .save(id, clock.now())
            .map_err(CreateError::Repository)?;
        Ok(id)
    }

    pub struct FixedClock(pub u64);

    impl Clock for FixedClock {
        fn now(&self) -> u64 {
            self.0
        }
    }

    pub struct Sequence {
        next: Option<u64>,
    }

    impl Sequence {
        pub const fn new(first: u64) -> Self {
            Self { next: Some(first) }
        }
    }

    impl IdGenerator for Sequence {
        fn next_id(&mut self) -> Option<u64> {
            let current = self.next?;
            self.next = current.checked_add(1);
            Some(current)
        }
    }

    #[derive(Default)]
    pub struct MemoryRepository(pub HashMap<u64, u64>);

    /// Test-double law: saving the same ID again replaces its timestamp.
    impl Repository for MemoryRepository {
        type Error = std::convert::Infallible;

        fn save(&mut self, id: u64, created_at: u64) -> Result<(), Self::Error> {
            self.0.insert(id, created_at);
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn three_minimal_fakes_make_the_use_case_deterministic() {
            let mut repository = MemoryRepository::default();
            let mut ids = Sequence::new(10);
            assert_eq!(create(&mut repository, &FixedClock(99), &mut ids), Ok(10));
            assert_eq!(repository.0.get(&10), Some(&99));
        }

        #[test]
        fn id_capability_can_be_a_closure_when_no_named_contract_is_needed() {
            let mut repository = MemoryRepository::default();
            let mut next: Option<u64> = Some(20);
            let mut ids = || {
                let id = next?;
                next = id.checked_add(1);
                Some(id)
            };
            assert_eq!(
                create_with_id_fn(&mut repository, &FixedClock(7), &mut ids),
                Ok(20)
            );
            assert_eq!(
                create_with_id_fn(&mut repository, &FixedClock(8), &mut ids),
                Ok(21)
            );
            assert_eq!(repository.0.get(&20), Some(&7));
            assert_eq!(repository.0.get(&21), Some(&8));
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        struct SaveFailed;

        struct FailingRepository;

        impl Repository for FailingRepository {
            type Error = SaveFailed;

            fn save(&mut self, _id: u64, _created_at: u64) -> Result<(), Self::Error> {
                Err(SaveFailed)
            }
        }

        #[test]
        fn repository_error_keeps_its_concrete_type() {
            assert_eq!(
                create(
                    &mut FailingRepository,
                    &FixedClock(1),
                    &mut Sequence::new(1)
                ),
                Err(CreateError::Repository(SaveFailed))
            );
        }

        #[test]
        fn memory_repository_obeys_last_write_wins() {
            let mut repository = MemoryRepository::default();
            repository.save(7, 10).unwrap();
            repository.save(7, 20).unwrap();
            assert_eq!(repository.0.len(), 1);
            assert_eq!(repository.0.get(&7), Some(&20));
        }
    }
}

pub mod k07 {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Transform {
        Trim,
        Lowercase,
        Prefix(String),
    }

    // SOLUTION: C56-K07
    impl Transform {
        pub fn apply(&self, input: &str) -> String {
            match self {
                Self::Trim => input.trim().to_owned(),
                Self::Lowercase => input.to_lowercase(),
                Self::Prefix(prefix) => format!("{prefix}{input}"),
            }
        }
    }

    pub trait TransformStep {
        fn apply(&self, input: &str) -> String;
    }

    impl TransformStep for Transform {
        fn apply(&self, input: &str) -> String {
            self.apply(input)
        }
    }

    pub fn run_enum(steps: &[Transform], input: &str) -> String {
        steps
            .iter()
            .fold(input.to_owned(), |value, step| step.apply(&value))
    }

    pub fn run_dynamic(steps: &[Box<dyn TransformStep>], input: &str) -> String {
        steps
            .iter()
            .fold(input.to_owned(), |value, step| step.apply(&value))
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum PipelineNeed {
        ClosedAndSerializable,
        OpenAtRuntime,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum DispatchStyle {
        Enum,
        TraitObject,
    }

    pub const fn choose_dispatch(need: PipelineNeed) -> DispatchStyle {
        match need {
            PipelineNeed::ClosedAndSerializable => DispatchStyle::Enum,
            PipelineNeed::OpenAtRuntime => DispatchStyle::TraitObject,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn closed_and_open_dispatch_can_have_equal_semantics() {
            let enums = [
                Transform::Trim,
                Transform::Lowercase,
                Transform::Prefix("lang:".into()),
            ];
            let dynamics: Vec<Box<dyn TransformStep>> = enums
                .iter()
                .cloned()
                .map(|step| Box::new(step) as Box<dyn TransformStep>)
                .collect();
            assert_eq!(run_enum(&enums, " Rust "), "lang:rust");
            assert_eq!(run_dynamic(&dynamics, " Rust "), "lang:rust");
        }

        struct Suffix(&'static str);

        impl TransformStep for Suffix {
            fn apply(&self, input: &str) -> String {
                format!("{input}{}", self.0)
            }
        }

        #[test]
        fn dynamic_pipeline_accepts_an_external_step() {
            let steps: Vec<Box<dyn TransformStep>> =
                vec![Box::new(Transform::Trim), Box::new(Suffix("!"))];
            assert_eq!(run_dynamic(&steps, " Rust "), "Rust!");
            assert_eq!(
                choose_dispatch(PipelineNeed::ClosedAndSerializable),
                DispatchStyle::Enum
            );
            assert_eq!(
                choose_dispatch(PipelineNeed::OpenAtRuntime),
                DispatchStyle::TraitObject
            );
        }

        #[test]
        fn both_empty_pipelines_preserve_the_input() {
            assert_eq!(run_enum(&[], " Rust "), " Rust ");
            assert_eq!(run_dynamic(&[], " Rust "), " Rust ");
        }
    }
}

pub mod k08 {
    pub struct ChunksExact<'a, T> {
        remaining: &'a [T],
        remainder: &'a [T],
        size: usize,
    }

    // SOLUTION: C56-K08
    impl<'a, T> ChunksExact<'a, T> {
        /// # Panics
        /// Panics when `size` is zero.
        pub fn new(values: &'a [T], size: usize) -> Self {
            assert!(size > 0, "chunk size must be greater than zero");
            let complete_length = values.len() - values.len() % size;
            let (remaining, remainder) = values.split_at(complete_length);
            Self {
                remaining,
                remainder,
                size,
            }
        }

        pub fn remainder(&self) -> &'a [T] {
            self.remainder
        }
    }

    impl<'a, T> Iterator for ChunksExact<'a, T> {
        type Item = &'a [T];

        fn next(&mut self) -> Option<Self::Item> {
            if self.remaining.is_empty() {
                return None;
            }
            let (next, remaining) = self.remaining.split_at(self.size);
            self.remaining = remaining;
            Some(next)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let length = self.remaining.len() / self.size;
            (length, Some(length))
        }
    }

    impl<T> ExactSizeIterator for ChunksExact<'_, T> {}

    impl<T> std::iter::FusedIterator for ChunksExact<'_, T> {}

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn iterator_yields_only_complete_non_overlapping_windows() {
            let values = [1, 2, 3, 4, 5];
            let mut chunks = ChunksExact::new(&values, 2);
            assert_eq!(
                chunks.by_ref().collect::<Vec<_>>(),
                [&[1, 2][..], &[3, 4][..]]
            );
            assert_eq!(chunks.remainder(), &[5]);
        }

        #[test]
        fn size_hint_and_fused_behavior_match_remaining_chunks() {
            let values = [1, 2, 3, 4, 5];
            let mut chunks = ChunksExact::new(&values, 2);
            assert_eq!(chunks.len(), 2);
            assert_eq!(chunks.next(), Some(&[1, 2][..]));
            assert_eq!(chunks.len(), 1);
            assert_eq!(chunks.next(), Some(&[3, 4][..]));
            assert_eq!(chunks.next(), None);
            assert_eq!(chunks.next(), None);
            assert_eq!(chunks.remainder(), &[5]);
        }

        #[test]
        fn chunk_larger_than_input_yields_only_a_remainder() {
            let values = [1, 2];
            let mut chunks = ChunksExact::new(&values, 3);
            assert_eq!(chunks.next(), None);
            assert_eq!(chunks.remainder(), &[1, 2]);

            let mut empty = ChunksExact::<u8>::new(&[], 1);
            assert_eq!(empty.next(), None);
            assert!(empty.remainder().is_empty());

            let values = [1, 2, 3];
            let chunks = ChunksExact::new(&values, 1).collect::<Vec<_>>();
            assert_eq!(chunks, [&[1][..], &[2][..], &[3][..]]);

            let values = [1, 2, 3, 4];
            let chunks = ChunksExact::new(&values, 2);
            assert!(chunks.remainder().is_empty());
        }

        #[test]
        #[should_panic(expected = "chunk size must be greater than zero")]
        fn zero_chunk_size_is_rejected() {
            let _ = ChunksExact::new(&[1, 2], 0);
        }
    }
}

pub mod k09 {
    use std::sync::mpsc::{self, SyncSender};
    use std::thread::{self, JoinHandle};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct StateSnapshot {
        pub value: i64,
    }

    enum Command {
        Apply(i64, mpsc::Sender<Result<(), WorkerError>>),
        Snapshot(mpsc::Sender<StateSnapshot>),
        Shutdown,
        #[cfg(test)]
        Crash,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum WorkerError {
        Closed,
        NoResponse,
        Panicked,
        Overflow,
    }

    #[derive(Clone)]
    pub struct WorkerClient {
        sender: SyncSender<Command>,
    }

    pub struct Worker {
        client: WorkerClient,
        handle: Option<JoinHandle<()>>,
    }

    impl WorkerClient {
        /// Waits for the worker to apply the change or reject it without mutation.
        pub fn apply(&self, change: i64) -> Result<(), WorkerError> {
            let (reply, answer) = mpsc::channel();
            self.sender
                .send(Command::Apply(change, reply))
                .map_err(|_| WorkerError::Closed)?;
            answer.recv().map_err(|_| WorkerError::NoResponse)?
        }

        pub fn snapshot(&self) -> Result<StateSnapshot, WorkerError> {
            let (reply, answer) = mpsc::channel();
            self.sender
                .send(Command::Snapshot(reply))
                .map_err(|_| WorkerError::Closed)?;
            answer.recv().map_err(|_| WorkerError::NoResponse)
        }

        #[cfg(test)]
        fn crash(&self) -> Result<(), WorkerError> {
            self.sender
                .send(Command::Crash)
                .map_err(|_| WorkerError::Closed)
        }
    }

    // SOLUTION: C56-K09
    impl Worker {
        pub fn start(capacity: usize) -> Self {
            let (sender, receiver) = mpsc::sync_channel(capacity);
            let handle = thread::spawn(move || {
                let mut value: i64 = 0;
                while let Ok(command) = receiver.recv() {
                    match command {
                        Command::Apply(change, reply) => {
                            let result = match value.checked_add(change) {
                                Some(next) => {
                                    value = next;
                                    Ok(())
                                }
                                None => Err(WorkerError::Overflow),
                            };
                            let _ = reply.send(result);
                        }
                        Command::Snapshot(reply) => {
                            let _ = reply.send(StateSnapshot { value });
                        }
                        Command::Shutdown => break,
                        #[cfg(test)]
                        Command::Crash => panic!("worker crash fixture"),
                    }
                }
            });
            Self {
                client: WorkerClient { sender },
                handle: Some(handle),
            }
        }

        pub fn client(&self) -> WorkerClient {
            self.client.clone()
        }

        pub fn apply(&self, change: i64) -> Result<(), WorkerError> {
            self.client.apply(change)
        }

        pub fn snapshot(&self) -> Result<StateSnapshot, WorkerError> {
            self.client.snapshot()
        }

        /// Joins without a deadline. `Drop` uses the same blocking cleanup,
        /// but cannot return a worker error to the caller.
        pub fn shutdown(mut self) -> Result<(), WorkerError> {
            let send_failed = self.client.sender.send(Command::Shutdown).is_err();
            let joined = self.handle.take().expect("worker handle present").join();
            if joined.is_err() {
                Err(WorkerError::Panicked)
            } else if send_failed {
                Err(WorkerError::Closed)
            } else {
                Ok(())
            }
        }
    }

    impl Drop for Worker {
        fn drop(&mut self) {
            if let Some(handle) = self.handle.take() {
                let _ = self.client.sender.send(Command::Shutdown);
                let _ = handle.join();
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn snapshot_is_owned_and_shutdown_is_joined() {
            let worker = Worker::start(2);
            worker.apply(40).unwrap();
            worker.apply(2).unwrap();
            assert_eq!(worker.snapshot(), Ok(StateSnapshot { value: 42 }));
            worker.shutdown().unwrap();
        }

        #[test]
        fn retained_client_observes_closed_after_joined_shutdown() {
            let worker = Worker::start(1);
            let client = worker.client();
            worker.shutdown().unwrap();
            assert_eq!(client.apply(1), Err(WorkerError::Closed));
            assert_eq!(client.snapshot(), Err(WorkerError::Closed));
        }

        #[test]
        fn worker_panic_is_reported_by_join_not_as_a_command_error() {
            let worker = Worker::start(1);
            worker.client().crash().unwrap();
            assert_eq!(worker.shutdown(), Err(WorkerError::Panicked));
        }
    }
}

pub mod k10 {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU64, Ordering};

    pub struct Metrics(AtomicU64);

    // SOLUTION: C56-K10
    impl Metrics {
        pub fn new() -> Self {
            Self(AtomicU64::new(0))
        }

        /// Counts modulo 2^64; it does not publish unrelated state.
        pub fn increment(&self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }

        pub fn value(&self) -> u64 {
            self.0.load(Ordering::Relaxed)
        }
    }

    impl Default for Metrics {
        fn default() -> Self {
            Self::new()
        }
    }

    pub struct Balances(Mutex<(u64, u64)>);

    impl Balances {
        pub fn new(source: u64, destination: u64) -> Self {
            Self(Mutex::new((source, destination)))
        }

        pub fn transfer(&self, amount: u64) -> bool {
            let mut balances = self.0.lock().expect("balances poisoned");
            let Some(source) = balances.0.checked_sub(amount) else {
                return false;
            };
            let Some(destination) = balances.1.checked_add(amount) else {
                return false;
            };
            balances.0 = source;
            balances.1 = destination;
            true
        }

        pub fn snapshot(&self) -> (u64, u64) {
            *self.0.lock().expect("balances poisoned")
        }
    }

    pub fn broken_two_atomic_interleaving(
        initial_source: u64,
        initial_destination: u64,
        amount: u64,
    ) -> (u64, u64) {
        let source = AtomicU64::new(initial_source);
        let destination = AtomicU64::new(initial_destination);

        let actor_a_observed = source.load(Ordering::Relaxed);
        let actor_b_observed = source.load(Ordering::Relaxed);
        if actor_a_observed >= amount {
            source.fetch_sub(amount, Ordering::Relaxed);
            destination.fetch_add(amount, Ordering::Relaxed);
        }
        if actor_b_observed >= amount {
            source.fetch_sub(amount, Ordering::Relaxed);
            destination.fetch_add(amount, Ordering::Relaxed);
        }
        (
            source.load(Ordering::Relaxed),
            destination.load(Ordering::Relaxed),
        )
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::Arc;
        use std::thread;

        #[test]
        fn metric_is_independent_but_balance_invariant_is_grouped() {
            let metrics = Metrics::new();
            metrics.increment();
            assert_eq!(metrics.value(), 1);

            let balances = Balances::new(10, 20);
            assert!(balances.transfer(4));
            assert_eq!(balances.snapshot(), (6, 24));
        }

        #[test]
        fn relaxed_metric_counts_concurrent_independent_events() {
            let metrics = Arc::new(Metrics::new());
            let handles = (0..4)
                .map(|_| {
                    let metrics = Arc::clone(&metrics);
                    thread::spawn(move || {
                        for _ in 0..1_000 {
                            metrics.increment();
                        }
                    })
                })
                .collect::<Vec<_>>();
            for handle in handles {
                handle.join().expect("metric worker");
            }
            assert_eq!(metrics.value(), 4_000);
        }

        #[test]
        fn two_independent_atomics_do_not_protect_the_balance_invariant() {
            let broken = broken_two_atomic_interleaving(10, 0, 7);
            assert!(broken.0 > 10);
            assert_eq!(broken.1, 14);

            let balances = Balances::new(10, 0);
            assert!(balances.transfer(7));
            assert!(!balances.transfer(7));
            assert_eq!(balances.snapshot(), (3, 7));
        }

        #[test]
        fn destination_overflow_leaves_locked_state_unchanged() {
            let balances = Balances::new(1, u64::MAX);
            assert!(!balances.transfer(1));
            assert_eq!(balances.snapshot(), (1, u64::MAX));
        }

        #[test]
        fn metric_wraparound_is_part_of_the_declared_policy() {
            let metrics = Metrics(AtomicU64::new(u64::MAX));
            metrics.increment();
            assert_eq!(metrics.value(), 0);
        }
    }
}

pub mod k11 {
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tokio::sync::{Notify, Semaphore};

    struct AdmissionState {
        accepting: bool,
        active: usize,
    }

    struct ImportState {
        admission: Mutex<AdmissionState>,
        idle: Notify,
    }

    #[derive(Clone)]
    pub struct Importer {
        downloads: Arc<Semaphore>,
        persists: Arc<Semaphore>,
        state: Arc<ImportState>,
    }

    pub struct AdmittedImport {
        downloads: Arc<Semaphore>,
        persists: Arc<Semaphore>,
        state: Arc<ImportState>,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ImportError {
        Closed,
        Invalid,
        NotAccepting,
        TooManyActive,
        Deadline,
    }

    // SOLUTION: C56-K11
    impl Importer {
        /// # Panics
        /// Panics if a limit exceeds `Semaphore::MAX_PERMITS`.
        /// Zero permits intentionally block that stage until cancellation.
        pub fn new(download_limit: usize, persist_limit: usize) -> Self {
            Self {
                downloads: Arc::new(Semaphore::new(download_limit)),
                persists: Arc::new(Semaphore::new(persist_limit)),
                state: Arc::new(ImportState {
                    admission: Mutex::new(AdmissionState {
                        accepting: true,
                        active: 0,
                    }),
                    idle: Notify::new(),
                }),
            }
        }

        pub fn admit(&self) -> Result<AdmittedImport, ImportError> {
            let mut admission = self
                .state
                .admission
                .lock()
                .expect("import admission state poisoned");
            if !admission.accepting {
                return Err(ImportError::NotAccepting);
            }
            admission.active = admission
                .active
                .checked_add(1)
                .ok_or(ImportError::TooManyActive)?;
            drop(admission);
            Ok(AdmittedImport {
                downloads: Arc::clone(&self.downloads),
                persists: Arc::clone(&self.persists),
                state: Arc::clone(&self.state),
            })
        }

        pub fn close_admission(&self) {
            self.state
                .admission
                .lock()
                .expect("import admission state poisoned")
                .accepting = false;
        }

        pub fn active_imports(&self) -> usize {
            self.state
                .admission
                .lock()
                .expect("import admission state poisoned")
                .active
        }

        /// Waits for all admitted tokens to be dropped; does not abort tasks.
        /// Retaining an unused token can prevent this future from completing.
        pub async fn shutdown(self) {
            self.close_admission();
            loop {
                let notified = self.state.idle.notified();
                tokio::pin!(notified);
                notified.as_mut().enable();
                if self.active_imports() == 0 {
                    return;
                }
                notified.await;
            }
        }

        pub async fn import(&self, input: &str) -> Result<String, ImportError> {
            self.admit()?.run(input).await
        }

        pub async fn import_with_timeout(
            &self,
            input: &str,
            duration: Duration,
        ) -> Result<String, ImportError> {
            let admitted = self.admit()?;
            tokio::time::timeout(duration, admitted.run(input))
                .await
                .map_err(|_| ImportError::Deadline)?
        }
    }

    impl AdmittedImport {
        /// Simulates download and persistence; no network or durable write occurs.
        pub async fn run(self, input: &str) -> Result<String, ImportError> {
            let download = self
                .downloads
                .acquire()
                .await
                .map_err(|_| ImportError::Closed)?;
            tokio::task::yield_now().await;
            let downloaded = input.to_owned();
            drop(download);

            let decoded = downloaded.trim().to_owned();
            if decoded.is_empty() {
                return Err(ImportError::Invalid);
            }

            let persist = self
                .persists
                .acquire()
                .await
                .map_err(|_| ImportError::Closed)?;
            tokio::task::yield_now().await;
            drop(persist);
            Ok(decoded)
        }
    }

    impl Drop for AdmittedImport {
        fn drop(&mut self) {
            let became_idle = {
                let mut admission = self
                    .state
                    .admission
                    .lock()
                    .expect("import admission state poisoned");
                admission.active = admission
                    .active
                    .checked_sub(1)
                    .expect("active import count underflow");
                admission.active == 0
            };
            if became_idle {
                self.state.idle.notify_waiters();
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn stages_have_separate_concurrency_budgets() {
            let importer = Importer::new(4, 2);
            assert_eq!(importer.downloads.available_permits(), 4);
            assert_eq!(importer.persists.available_permits(), 2);
            assert_eq!(
                importer.import("  record ").await,
                Ok(String::from("record"))
            );
            assert_eq!(importer.import("   ").await, Err(ImportError::Invalid));
        }

        #[tokio::test]
        async fn close_rejects_new_work_but_admitted_work_can_finish() {
            let importer = Importer::new(1, 1);
            let admitted = importer.admit().unwrap();
            importer.close_admission();

            assert_eq!(importer.import("new").await, Err(ImportError::NotAccepting));
            assert_eq!(admitted.run("accepted").await, Ok("accepted".to_owned()));
            assert_eq!(importer.active_imports(), 0);
        }

        #[tokio::test]
        async fn shutdown_waits_until_admitted_work_has_finished() {
            let importer = Importer::new(1, 1);
            let held = Arc::clone(&importer.downloads)
                .acquire_owned()
                .await
                .unwrap();
            let admitted = importer.admit().unwrap();
            let work = tokio::spawn(admitted.run("accepted"));
            tokio::task::yield_now().await;

            let first_shutdown = tokio::spawn(importer.clone().shutdown());
            let second_shutdown = tokio::spawn(importer.shutdown());
            tokio::task::yield_now().await;
            assert!(!first_shutdown.is_finished());
            assert!(!second_shutdown.is_finished());

            drop(held);
            first_shutdown.await.unwrap();
            second_shutdown.await.unwrap();
            assert_eq!(work.await.unwrap(), Ok("accepted".to_owned()));
        }

        #[tokio::test(start_paused = true)]
        async fn one_deadline_covers_waiting_for_stage_capacity() {
            let importer = Importer::new(1, 1);
            let held = importer.downloads.acquire().await.unwrap();
            assert_eq!(
                importer
                    .import_with_timeout("record", Duration::from_secs(1))
                    .await,
                Err(ImportError::Deadline)
            );
            assert_eq!(importer.active_imports(), 0);
            drop(held);
            assert_eq!(importer.import("record").await, Ok("record".to_owned()));
        }

        #[tokio::test]
        async fn a_closed_stage_reports_closed_without_leaking_admission() {
            let importer = Importer::new(1, 1);
            importer.downloads.close();
            assert_eq!(importer.import("record").await, Err(ImportError::Closed));
        }
    }
}

pub mod k12 {
    use std::future::Future;
    use std::rc::Rc;

    // SOLUTION: C56-K12
    pub fn owned_send_future(value: &str) -> impl Future<Output = String> + Send + 'static + use<> {
        let owned = value.to_owned();
        async move {
            tokio::task::yield_now().await;
            owned
        }
    }

    pub async fn scoped_send_future(value: u64) -> u64 {
        let copied = {
            let local = Rc::new(value);
            *local
        };
        tokio::task::yield_now().await;
        copied
    }

    /// Keeps `Rc` across the suspension point, so the future is local.
    ///
    /// ```compile_fail
    /// let future = course_solutions::katas::k12::local_future("local");
    /// tokio::spawn(future);
    /// ```
    pub fn local_future(value: &str) -> impl Future<Output = String> + 'static + use<> {
        let local = Rc::new(value.to_owned());
        async move {
            tokio::task::yield_now().await;
            (*local).clone()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
        async fn owned_data_can_be_spawned_on_a_multithread_runtime() {
            let handle = tokio::spawn(owned_send_future("Send"));
            assert_eq!(handle.await.unwrap(), "Send");
        }

        fn assert_send<T: Send>(_: &T) {}

        #[tokio::test]
        async fn non_send_value_can_be_consumed_before_await() {
            let future = scoped_send_future(42);
            assert_send(&future);
            assert_eq!(tokio::spawn(future).await.unwrap(), 42);
        }

        #[tokio::test(flavor = "current_thread")]
        async fn rc_across_await_runs_on_a_local_set() {
            let local = tokio::task::LocalSet::new();
            let output = local
                .run_until(async {
                    tokio::task::spawn_local(local_future("local"))
                        .await
                        .unwrap()
                })
                .await;
            assert_eq!(output, "local");
        }
    }
}

pub mod k13 {
    use std::panic::UnwindSafe;
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;

    /// The wrapper remains thread-affine until the native contract proves
    /// that its handle may cross thread boundaries.
    ///
    /// ```compile_fail
    /// fn require_send<T: Send>() {}
    /// require_send::<course_solutions::katas::k13::DemonstrationBuffer>();
    /// ```
    ///
    /// The borrowed view cannot outlive its wrapper:
    ///
    /// ```compile_fail
    /// use course_solutions::katas::k13::DemonstrationBuffer;
    /// let bytes = {
    ///     let buffer = DemonstrationBuffer::new(vec![1]);
    ///     buffer.as_slice()
    /// };
    /// assert_eq!(bytes, [1]);
    /// ```
    ///
    /// Shared references cannot be sent to other threads either:
    ///
    /// ```compile_fail
    /// fn require_sync<T: Sync>() {}
    /// require_sync::<course_solutions::katas::k13::DemonstrationBuffer>();
    /// ```
    pub struct DemonstrationBuffer {
        inner: crate::unsafe_low_level::c47::Buffer,
        drops: Arc<AtomicUsize>,
    }

    // SOLUTION: C56-K13
    impl DemonstrationBuffer {
        pub fn new(bytes: Vec<u8>) -> Self {
            Self::try_new(bytes, false).expect("the infallible simulated constructor returned null")
        }

        pub fn try_new(
            bytes: Vec<u8>,
            should_fail: bool,
        ) -> Result<Self, crate::unsafe_low_level::c47::BufferCreationFailed> {
            let drops = Arc::new(AtomicUsize::new(0));
            let inner = crate::unsafe_low_level::c47::Buffer::try_new(
                bytes,
                Arc::clone(&drops),
                should_fail,
            )?;
            Ok(Self { inner, drops })
        }

        pub fn as_slice(&self) -> &[u8] {
            self.inner.as_slice()
        }

        pub fn drop_counter(&self) -> Arc<AtomicUsize> {
            Arc::clone(&self.drops)
        }
    }

    pub fn callback_boundary<F>(operation: F) -> i32
    where
        F: FnOnce() -> i32 + UnwindSafe,
    {
        crate::unsafe_low_level::c47::ffi_panic_firewall(operation)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::atomic::Ordering;

        #[test]
        fn wrapper_owns_the_handle_and_borrows_its_slice() {
            let buffer = DemonstrationBuffer::new(vec![4, 2]);
            let drops = buffer.drop_counter();
            assert_eq!(buffer.as_slice(), [4, 2]);
            drop(buffer);
            assert_eq!(drops.load(Ordering::SeqCst), 1);
        }

        #[test]
        fn null_creation_is_converted_to_an_error() {
            assert!(DemonstrationBuffer::try_new(Vec::new(), true).is_err());
        }

        #[test]
        fn callback_boundary_converts_panics_without_unwinding_outward() {
            assert_eq!(callback_boundary(|| 7), 7);
            assert_eq!(
                callback_boundary(|| panic!("ffi callback fixture")),
                crate::unsafe_low_level::c47::FFI_CALL_PANIC
            );
        }
    }
}

pub mod k14 {
    /// Returns early from the caller and evaluates the condition and error at
    /// most once.
    ///
    /// ```
    /// use course_solutions::ensure_course;
    ///
    /// fn positive(value: i64) -> Result<i64, &'static str> {
    ///     ensure_course!(value > 0, "must be positive",);
    ///     Ok(value)
    /// }
    ///
    /// assert_eq!(positive(2), Ok(2));
    /// assert_eq!(positive(0), Err("must be positive"));
    /// ```
    ///
    /// ```compile_fail
    /// use course_solutions::ensure_course;
    /// ensure_course!(true);
    /// ```
    #[macro_export]
    // SOLUTION: C56-K14
    macro_rules! ensure_course {
        ($condition:expr, $error:expr $(,)?) => {{
            if !$condition {
                return ::core::result::Result::Err($error);
            }
        }};
    }

    #[macro_export]
    macro_rules! ensure_with_context_course {
        ($condition:expr, $error:expr, $context:expr $(,)?) => {{
            if !$condition {
                return ::core::result::Result::Err(::std::format!("{}: {}", $context, $error));
            }
        }};
    }

    pub fn positive(value: i64) -> Result<i64, &'static str> {
        crate::ensure_course!(value > 0, "must be positive",);
        Ok(value)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cell::Cell;

        #[test]
        fn macro_returns_early_and_evaluates_condition_once() {
            let calls = Cell::new(0);
            let result = (|| {
                crate::ensure_course!(
                    {
                        calls.set(calls.get() + 1);
                        false
                    },
                    "stop",
                );
                Ok::<_, &'static str>(())
            })();
            assert_eq!(result, Err("stop"));
            assert_eq!(calls.get(), 1);
            assert_eq!(positive(3), Ok(3));
        }

        #[test]
        fn error_and_context_are_lazy_and_evaluated_once() {
            let error_calls = Cell::new(0);
            let context_calls = Cell::new(0);
            let success = (|| {
                crate::ensure_with_context_course!(
                    true,
                    {
                        error_calls.set(error_calls.get() + 1);
                        "error"
                    },
                    {
                        context_calls.set(context_calls.get() + 1);
                        "context"
                    },
                );
                Ok::<_, String>(())
            })();
            assert_eq!(success, Ok(()));
            assert_eq!((error_calls.get(), context_calls.get()), (0, 0));

            let failure = (|| {
                crate::ensure_with_context_course!(
                    false,
                    {
                        error_calls.set(error_calls.get() + 1);
                        "error"
                    },
                    {
                        context_calls.set(context_calls.get() + 1);
                        "context"
                    },
                );
                Ok::<_, String>(())
            })();
            assert_eq!(failure, Err("context: error".to_owned()));
            assert_eq!((error_calls.get(), context_calls.get()), (1, 1));
        }

        #[test]
        fn macro_hygiene_does_not_shadow_caller_bindings() {
            let condition = "caller value";
            let result = (|| {
                crate::ensure_course!(condition.len() == 12, "wrong length");
                Ok::<_, &'static str>(condition)
            })();
            assert_eq!(result, Ok("caller value"));
        }
    }
}
