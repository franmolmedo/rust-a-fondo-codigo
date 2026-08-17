//! Capítulos 11 a 14: closures, iteradores, composición y tipos de dominio.

pub mod c11 {
    // SOLUTION: C11-E01
    pub fn read_factor(factor: i32, value: i32) -> (i32, i32) {
        let multiply = |number| number * factor;
        (multiply(value), factor)
    }

    // SOLUTION: C11-E02
    pub fn call_twice_mut<F>(mut operation: F)
    where
        F: FnMut(),
    {
        operation();
        operation();
    }

    pub fn increment_twice(counter: &mut u32) {
        call_twice_mut(|| *counter += 1);
    }

    // SOLUTION: C11-E03
    pub fn move_but_reuse(text: String) -> (usize, usize) {
        let length = move || text.len();
        (length(), length())
    }

    // SOLUTION: C11-E04
    pub fn consume_capture(text: String) -> String {
        let consume = move || text;
        consume()
    }

    // SOLUTION: C11-E05
    pub fn apply_once<T, U, F>(value: T, operation: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        operation(value)
    }

    // SOLUTION: C11-E06
    pub fn repeat<F>(times: usize, mut operation: F)
    where
        F: FnMut(usize),
    {
        for index in 0..times {
            operation(index);
        }
    }

    // SOLUTION: C11-E07
    pub fn prefix_filter(prefix: impl Into<String>) -> impl Fn(&str) -> bool {
        let prefix = prefix.into();
        move |candidate| candidate.starts_with(&prefix)
    }

    // SOLUTION: C11-E08
    pub fn retry<T, E, F>(attempts: std::num::NonZeroUsize, mut operation: F) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
    {
        for attempt in 1..=attempts.get() {
            match operation() {
                Ok(value) => return Ok(value),
                Err(error) if attempt == attempts.get() => return Err(error),
                Err(_) => {}
            }
        }
        unreachable!("NonZeroUsize garantiza al menos un intento")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn captures_determine_fn_capabilities() {
            assert_eq!(read_factor(10, 3), (30, 10));

            let mut counter = 0;
            increment_twice(&mut counter);
            assert_eq!(counter, 2);

            assert_eq!(move_but_reuse(String::from("Rust")), (4, 4));
            assert_eq!(consume_capture(String::from("owned")), "owned");
        }

        #[test]
        fn generic_apis_request_only_the_capability_they_use() {
            let owned = String::from("Rust");
            assert_eq!(apply_once(owned, |value| value.len()), 4);

            let mut visited = Vec::new();
            repeat(3, |index| visited.push(index));
            assert_eq!(visited, [0, 1, 2]);
        }

        #[test]
        fn factories_own_configuration_and_remain_reusable() {
            let is_rust = prefix_filter("ru");
            assert!(is_rust("rust"));
            assert!(is_rust("rules"));
            assert!(!is_rust("book"));
        }

        #[test]
        fn retry_returns_late_success_or_the_last_error() {
            use std::num::NonZeroUsize;

            let mut attempts = 0;
            let result = retry(NonZeroUsize::new(3).unwrap(), || {
                attempts += 1;
                (attempts == 3).then_some("ok").ok_or("todavía no")
            });
            assert_eq!(result, Ok("ok"));
            assert_eq!(attempts, 3);

            let mut error_number = 0;
            let result: Result<(), u32> = retry(NonZeroUsize::new(2).unwrap(), || {
                error_number += 1;
                Err(error_number)
            });
            assert_eq!(result, Err(2));
        }
    }
}

pub mod c12 {
    use std::cell::Cell;
    use std::iter::FusedIterator;
    use std::num::ParseIntError;

    // SOLUTION: C12-E01
    pub fn observed_lengths(values: &[String]) -> Vec<usize> {
        values.iter().map(String::len).collect()
    }

    pub fn consumed_lengths(values: Vec<String>) -> Vec<usize> {
        values.into_iter().map(|value| value.len()).collect()
    }

    // SOLUTION: C12-E02
    pub fn doubled_evens(values: &[i32]) -> Vec<i32> {
        values
            .iter()
            .copied()
            .filter(|value| value % 2 == 0)
            .map(|value| value * 2)
            .collect()
    }

    // SOLUTION: C12-E03
    pub fn lazy_map_observation(values: &[i32]) -> (usize, usize, Vec<i32>) {
        let calls = Cell::new(0);
        let pipeline = values.iter().copied().map(|value| {
            calls.set(calls.get() + 1);
            value * 2
        });
        let before_consuming = calls.get();
        let result = pipeline.collect();
        (before_consuming, calls.get(), result)
    }

    // SOLUTION: C12-E04
    pub fn parse_numbers(input: &str) -> Result<Vec<i32>, ParseIntError> {
        input.split_whitespace().map(str::parse::<i32>).collect()
    }

    // SOLUTION: C12-E05
    pub fn first_non_empty_line(input: &str) -> Option<&str> {
        input.lines().map(str::trim).find(|line| !line.is_empty())
    }

    // SOLUTION: C12-E06
    pub fn checked_sum(values: impl IntoIterator<Item = i64>) -> Option<i64> {
        values.into_iter().try_fold(0_i64, i64::checked_add)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Countdown {
        next: u32,
    }

    // SOLUTION: C12-E07
    impl Countdown {
        pub fn new(start: u32) -> Self {
            Self { next: start }
        }
    }

    impl Iterator for Countdown {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.next == 0 {
                return None;
            }
            let current = self.next;
            self.next -= 1;
            Some(current)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.next as usize;
            (remaining, Some(remaining))
        }
    }

    impl ExactSizeIterator for Countdown {}
    impl FusedIterator for Countdown {}

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct User {
        pub name: String,
        pub active: bool,
    }

    // SOLUTION: C12-E08
    pub fn active_names(users: &[User]) -> Vec<&str> {
        users
            .iter()
            .filter(|user| user.active)
            .map(|user| user.name.as_str())
            .collect()
    }

    pub fn active_names_owned(users: &[User]) -> Vec<String> {
        users
            .iter()
            .filter(|user| user.active)
            .map(|user| user.name.clone())
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn entry_mode_and_laziness_are_observable() {
            let names = vec![String::from("Ada"), String::from("Grace")];
            assert_eq!(observed_lengths(&names), [3, 5]);
            assert_eq!(names[0], "Ada");
            assert_eq!(consumed_lengths(names), [3, 5]);

            assert_eq!(lazy_map_observation(&[1, 2, 3]), (0, 3, vec![2, 4, 6]));
        }

        #[test]
        fn pipelines_transform_find_and_fail_fast() {
            assert_eq!(doubled_evens(&[1, 2, 3, 4]), [4, 8]);
            assert_eq!(parse_numbers("10 20 -3"), Ok(vec![10, 20, -3]));
            assert_eq!(first_non_empty_line("\n  \n Rust"), Some("Rust"));
        }

        #[test]
        fn fallible_fold_stops_on_overflow() {
            assert_eq!(checked_sum([1, 2, 3]), Some(6));
            assert_eq!(checked_sum([i64::MAX, 1]), None);
        }

        #[test]
        fn custom_iterator_advances_and_stays_finished() {
            let mut countdown = Countdown::new(3);
            assert_eq!(countdown.len(), 3);
            assert_eq!(countdown.next(), Some(3));
            assert_eq!(countdown.len(), 2);
            assert_eq!(countdown.by_ref().collect::<Vec<_>>(), [2, 1]);
            assert_eq!(countdown.next(), None);
        }

        #[test]
        fn borrowed_and_owned_queries_have_distinct_lifetimes() {
            let users = [
                User {
                    name: String::from("Ada"),
                    active: true,
                },
                User {
                    name: String::from("Grace"),
                    active: false,
                },
            ];
            assert_eq!(active_names(&users), ["Ada"]);
            assert_eq!(active_names_owned(&users), [String::from("Ada")]);
        }
    }
}

pub mod c13 {
    use std::num::ParseIntError;
    use thiserror::Error;

    // SOLUTION: C13-E01
    pub fn total_selected(values: &[u64]) -> u64 {
        values
            .iter()
            .copied()
            .map(|value| value * 2)
            .filter(|value| value % 3 == 0)
            .sum()
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Record {
        pub id: u64,
        pub name: String,
    }

    #[derive(Clone, Debug, Error, Eq, PartialEq)]
    pub enum ImportError {
        #[error("línea {line}: faltan campos")]
        MissingField { line: usize },
        #[error("línea {line}: id inválido")]
        InvalidId { line: usize },
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ImportReport {
        pub accepted: Vec<Record>,
        pub rejected: Vec<ImportError>,
    }

    fn parse_line(line_number: usize, line: &str) -> Result<Record, ImportError> {
        let (raw_id, name) = line
            .split_once(',')
            .ok_or(ImportError::MissingField { line: line_number })?;
        let id = raw_id
            .trim()
            .parse()
            .map_err(|_| ImportError::InvalidId { line: line_number })?;
        Ok(Record {
            id,
            name: name.trim().to_owned(),
        })
    }

    // SOLUTION: C13-E02
    pub fn import_fail_fast(input: &str) -> Result<Vec<Record>, ImportError> {
        input
            .lines()
            .enumerate()
            .map(|(index, line)| parse_line(index + 1, line))
            .collect()
    }

    pub fn import_all(input: &str) -> ImportReport {
        input.lines().enumerate().fold(
            ImportReport {
                accepted: Vec::new(),
                rejected: Vec::new(),
            },
            |mut report, (index, line)| {
                match parse_line(index + 1, line) {
                    Ok(record) => report.accepted.push(record),
                    Err(error) => report.rejected.push(error),
                }
                report
            },
        )
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct User {
        pub name: String,
        pub active: bool,
    }

    // SOLUTION: C13-E03
    pub fn active_names(users: &[User]) -> Vec<&str> {
        users
            .iter()
            .filter(|user| user.active)
            .map(|user| user.name.as_str())
            .collect()
    }

    pub fn active_names_owned(users: &[User]) -> Vec<String> {
        users
            .iter()
            .filter(|user| user.active)
            .map(|user| user.name.clone())
            .collect()
    }

    // SOLUTION: C13-E04
    pub fn total<I>(items: I) -> u64
    where
        I: IntoIterator<Item = u64>,
    {
        items.into_iter().sum()
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ReviewReport {
        pub accepted: Vec<u64>,
        pub rejected: Vec<u64>,
        pub total_cents: u64,
    }

    // SOLUTION: C13-E05
    pub fn review(requests: &[(u64, u64)]) -> ReviewReport {
        let mut report = ReviewReport {
            accepted: Vec::new(),
            rejected: Vec::new(),
            total_cents: 0,
        };

        for &(id, amount) in requests {
            if amount == 0 {
                report.rejected.push(id);
                continue;
            }
            report.accepted.push(id);
            report.total_cents += amount;
        }
        report
    }

    #[derive(Clone, Debug, Error, Eq, PartialEq)]
    pub enum ConfigError {
        #[error("falta PORT")]
        MissingPort,
        #[error("PORT no es un u16 válido")]
        InvalidPort,
    }

    // SOLUTION: C13-E06
    pub fn optional_port(value: Option<&str>) -> Result<Option<u16>, ParseIntError> {
        value.map(str::parse::<u16>).transpose()
    }

    pub fn required_port(value: Option<&str>) -> Result<u16, ConfigError> {
        value
            .ok_or(ConfigError::MissingPort)?
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidPort)
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Order {
        pub total_cents: u64,
    }

    // SOLUTION: C13-E07
    pub trait OrderIteratorExt: Iterator<Item = Order> + Sized {
        fn billable(self) -> impl Iterator<Item = Order> {
            self.filter(|order| order.total_cents > 0)
        }
    }

    impl<I> OrderIteratorExt for I where I: Iterator<Item = Order> {}

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn one_pipeline_materializes_only_the_final_result() {
            assert_eq!(total_selected(&[1, 2, 3, 4, 6]), 18);
        }

        #[test]
        fn policies_distinguish_first_error_from_complete_report() {
            let input = "1,Ada\ninvalid,Grace\n3,Linus";
            assert_eq!(
                import_fail_fast(input),
                Err(ImportError::InvalidId { line: 2 })
            );
            let report = import_all(input);
            assert_eq!(report.accepted.len(), 2);
            assert_eq!(report.rejected, [ImportError::InvalidId { line: 2 }]);
        }

        #[test]
        fn ownership_and_generic_sources_are_explicit() {
            let users = [
                User {
                    name: String::from("Ada"),
                    active: true,
                },
                User {
                    name: String::from("Grace"),
                    active: false,
                },
            ];
            assert_eq!(active_names(&users), ["Ada"]);
            assert_eq!(active_names_owned(&users), [String::from("Ada")]);
            assert_eq!(total([100, 200]), 300);
            assert_eq!(total(vec![300, 400]), 700);
            assert_eq!(total((1..=3).map(|value| value * 10)), 60);
        }

        #[test]
        fn a_named_loop_exposes_multiple_outputs() {
            assert_eq!(
                review(&[(1, 500), (2, 0)]),
                ReviewReport {
                    accepted: vec![1],
                    rejected: vec![2],
                    total_cents: 500,
                }
            );
        }

        #[test]
        fn configuration_contracts_distinguish_absence() {
            assert_eq!(optional_port(None), Ok(None));
            assert_eq!(optional_port(Some("8080")), Ok(Some(8080)));
            assert!(optional_port(Some("bad")).is_err());
            assert_eq!(required_port(None), Err(ConfigError::MissingPort));
            assert_eq!(required_port(Some("bad")), Err(ConfigError::InvalidPort));
        }

        #[test]
        fn extension_trait_adds_a_lazy_domain_stage() {
            let total: u64 = vec![
                Order { total_cents: 0 },
                Order { total_cents: 500 },
                Order { total_cents: 300 },
            ]
            .into_iter()
            .billable()
            .map(|order| order.total_cents)
            .sum();
            assert_eq!(total, 800);
        }
    }
}

pub mod c14 {
    use std::marker::PhantomData;
    use thiserror::Error;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct UserId(u64);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct OrderId(u64);

    // SOLUTION: C14-E01
    impl UserId {
        pub fn new(value: u64) -> Self {
            Self(value)
        }

        pub fn get(self) -> u64 {
            self.0
        }
    }

    impl OrderId {
        pub fn new(value: u64) -> Self {
            Self(value)
        }

        pub fn get(self) -> u64 {
            self.0
        }
    }

    pub fn user_route(id: UserId) -> String {
        format!("/users/{}", id.get())
    }

    pub fn order_route(id: OrderId) -> String {
        format!("/orders/{}", id.get())
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Draft;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Verified;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Registration<State> {
        email: String,
        state: PhantomData<State>,
    }

    #[derive(Clone, Debug, Error, Eq, PartialEq)]
    pub enum VerifyError {
        #[error("token inválido")]
        InvalidToken,
    }

    // SOLUTION: C14-E02
    impl Registration<Draft> {
        pub fn new(email: impl Into<String>) -> Self {
            Self {
                email: email.into(),
                state: PhantomData,
            }
        }

        pub fn verify(self, token: &str) -> Result<Registration<Verified>, (Self, VerifyError)> {
            if token != "known-token" {
                return Err((self, VerifyError::InvalidToken));
            }
            Ok(Registration {
                email: self.email,
                state: PhantomData,
            })
        }
    }

    impl Registration<Verified> {
        pub fn email(&self) -> &str {
            &self.email
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum OrderState {
        Pending,
        Paid,
        Shipped,
        Cancelled,
    }

    #[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
    pub enum CancelError {
        #[error("el pedido ya estaba cancelado")]
        AlreadyCancelled,
        #[error("un pedido enviado ya no puede cancelarse")]
        TooLate,
    }

    // SOLUTION: C14-E03
    impl OrderState {
        pub fn cancel(&mut self) -> Result<(), CancelError> {
            match self {
                Self::Pending | Self::Paid => {
                    *self = Self::Cancelled;
                    Ok(())
                }
                Self::Shipped => Err(CancelError::TooLate),
                Self::Cancelled => Err(CancelError::AlreadyCancelled),
            }
        }
    }

    pub struct ApiKey(String);

    #[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
    #[error("{field} debe medir {expected} bytes; se recibieron {actual}")]
    pub struct ConstructionError {
        pub field: &'static str,
        pub expected: usize,
        pub actual: usize,
    }

    // SOLUTION: C14-E04
    impl ApiKey {
        pub const LENGTH: usize = 16;

        pub fn parse(raw: &str) -> Result<Self, ConstructionError> {
            if raw.len() != Self::LENGTH {
                return Err(ConstructionError {
                    field: "api_key",
                    expected: Self::LENGTH,
                    actual: raw.len(),
                });
            }
            Ok(Self(raw.to_owned()))
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum WriteMode {
        CreateNew,
        Replace,
        Append,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct WritePlan {
        pub must_not_exist: bool,
        pub truncate: bool,
        pub append: bool,
    }

    // SOLUTION: C14-E05
    pub fn write_plan(mode: WriteMode) -> WritePlan {
        match mode {
            WriteMode::CreateNew => WritePlan {
                must_not_exist: true,
                truncate: false,
                append: false,
            },
            WriteMode::Replace => WritePlan {
                must_not_exist: false,
                truncate: true,
                append: false,
            },
            WriteMode::Append => WritePlan {
                must_not_exist: false,
                truncate: false,
                append: true,
            },
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Cents(u64);

    impl Cents {
        pub fn new(value: u64) -> Self {
            Self(value)
        }

        pub fn get(self) -> u64 {
            self.0
        }
    }

    #[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
    #[error("saldo insuficiente: disponible {available}, solicitado {requested}")]
    pub struct InsufficientAmount {
        pub available: u64,
        pub requested: u64,
    }

    // SOLUTION: C14-E06
    impl Cents {
        pub fn checked_sub(self, other: Self) -> Result<Self, InsufficientAmount> {
            self.0
                .checked_sub(other.0)
                .map(Self)
                .ok_or(InsufficientAmount {
                    available: self.0,
                    requested: other.0,
                })
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Email(String);

    impl Email {
        fn parse(raw: String) -> Result<Self, ()> {
            raw.split_once('@')
                .is_some_and(|(local, domain)| !local.is_empty() && domain.contains('.'))
                .then_some(Self(raw))
                .ok_or(())
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct CountryCode([u8; 2]);

    impl CountryCode {
        fn parse(raw: &str) -> Result<Self, ()> {
            let bytes = raw.as_bytes();
            if bytes.len() != 2 || !bytes.iter().all(u8::is_ascii_alphabetic) {
                return Err(());
            }
            Ok(Self([
                bytes[0].to_ascii_uppercase(),
                bytes[1].to_ascii_uppercase(),
            ]))
        }

        pub fn as_str(&self) -> &str {
            std::str::from_utf8(&self.0).expect("invariante: dos letras ASCII")
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AdultAge(u8);

    impl AdultAge {
        fn parse(value: u8) -> Result<Self, ()> {
            (value >= 18).then_some(Self(value)).ok_or(())
        }

        pub fn get(self) -> u8 {
            self.0
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct RawSignup {
        pub email: String,
        pub country: String,
        pub age: u8,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Signup {
        email: Email,
        country: CountryCode,
        age: AdultAge,
    }

    impl Signup {
        pub fn email(&self) -> &Email {
            &self.email
        }

        pub fn country(&self) -> CountryCode {
            self.country
        }

        pub fn age(&self) -> AdultAge {
            self.age
        }
    }

    #[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
    pub enum SignupError {
        #[error("correo inválido")]
        Email,
        #[error("código de país inválido")]
        Country,
        #[error("edad inferior al mínimo")]
        Age,
    }

    // SOLUTION: C14-E07
    impl TryFrom<RawSignup> for Signup {
        type Error = SignupError;

        fn try_from(raw: RawSignup) -> Result<Self, Self::Error> {
            Ok(Self {
                email: Email::parse(raw.email).map_err(|()| SignupError::Email)?,
                country: CountryCode::parse(&raw.country).map_err(|()| SignupError::Country)?,
                age: AdultAge::parse(raw.age).map_err(|()| SignupError::Age)?,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn distinct_ids_cannot_be_mixed_by_the_type_system() {
            assert_eq!(user_route(UserId::new(7)), "/users/7");
            assert_eq!(order_route(OrderId::new(7)), "/orders/7");
        }

        #[test]
        fn typestate_returns_the_draft_after_a_failed_transition() {
            let draft = Registration::<Draft>::new("ada@example.test");
            let (draft, error) = draft.verify("wrong-token").unwrap_err();
            assert_eq!(error, VerifyError::InvalidToken);
            let verified = draft.verify("known-token").unwrap();
            assert_eq!(verified.email(), "ada@example.test");
        }

        #[test]
        fn cancellation_changes_only_valid_runtime_states() {
            let mut pending = OrderState::Pending;
            assert_eq!(pending.cancel(), Ok(()));
            assert_eq!(pending, OrderState::Cancelled);
            assert_eq!(pending.cancel(), Err(CancelError::AlreadyCancelled));

            let mut shipped = OrderState::Shipped;
            assert_eq!(shipped.cancel(), Err(CancelError::TooLate));
            assert_eq!(shipped, OrderState::Shipped);
        }

        #[test]
        fn construction_errors_never_contain_the_secret() {
            let secret = "too-short";
            let error = ApiKey::parse(secret).err().unwrap();
            assert_eq!(error.actual, secret.len());
            assert!(!error.to_string().contains(secret));
            assert!(!format!("{error:?}").contains(secret));
            assert_eq!(ApiKey::parse("0123456789abcdef").unwrap().len(), 16);
        }

        #[test]
        fn write_modes_map_to_unambiguous_plans() {
            assert_eq!(
                write_plan(WriteMode::CreateNew),
                WritePlan {
                    must_not_exist: true,
                    truncate: false,
                    append: false,
                }
            );
            assert!(write_plan(WriteMode::Replace).truncate);
            assert!(write_plan(WriteMode::Append).append);
        }

        #[test]
        fn subtraction_never_saturates_silently() {
            assert_eq!(
                Cents::new(10).checked_sub(Cents::new(10)),
                Ok(Cents::new(0))
            );
            assert_eq!(
                Cents::new(3).checked_sub(Cents::new(10)),
                Err(InsufficientAmount {
                    available: 3,
                    requested: 10,
                })
            );
        }

        fn valid_raw_signup() -> RawSignup {
            RawSignup {
                email: String::from("ada@example.com"),
                country: String::from("es"),
                age: 36,
            }
        }

        #[test]
        fn dto_conversion_checks_every_invariant() {
            let signup = Signup::try_from(valid_raw_signup()).unwrap();
            assert_eq!(signup.email().as_str(), "ada@example.com");
            assert_eq!(signup.country().as_str(), "ES");
            assert_eq!(signup.age().get(), 36);

            let mut invalid_email = valid_raw_signup();
            invalid_email.email = String::from("invalid");
            assert_eq!(Signup::try_from(invalid_email), Err(SignupError::Email));

            let mut invalid_country = valid_raw_signup();
            invalid_country.country = String::from("ESP");
            assert_eq!(Signup::try_from(invalid_country), Err(SignupError::Country));

            let mut invalid_age = valid_raw_signup();
            invalid_age.age = 17;
            assert_eq!(Signup::try_from(invalid_age), Err(SignupError::Age));
        }
    }
}
