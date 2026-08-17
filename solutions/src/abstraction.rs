//! Capítulos 15 a 20: traits, genéricos, GATs, opacos y dispatch.

pub mod c15 {
    use std::cell::Cell;
    use std::collections::HashMap;
    use std::fmt;
    use std::rc::Rc;

    // SOLUTION: C15-E01
    pub trait Clock {
        fn now_millis(&self) -> u64;
    }

    #[derive(Clone, Copy, Debug)]
    pub struct FixedClock(pub u64);

    impl Clock for FixedClock {
        fn now_millis(&self) -> u64 {
            self.0
        }
    }

    #[derive(Debug)]
    pub struct AdvancingClock {
        next: Cell<u64>,
        step: u64,
    }

    impl AdvancingClock {
        pub fn new(first: u64, step: u64) -> Self {
            Self {
                next: Cell::new(first),
                step,
            }
        }
    }

    impl Clock for AdvancingClock {
        fn now_millis(&self) -> u64 {
            let current = self.next.get();
            self.next.set(current.saturating_add(self.step));
            current
        }
    }

    pub fn is_expired(clock: &impl Clock, deadline: u64) -> bool {
        clock.now_millis() >= deadline
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ExternalId(pub u64);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct DisplayId(pub ExternalId);

    // SOLUTION: C15-E02
    impl From<ExternalId> for DisplayId {
        fn from(value: ExternalId) -> Self {
            Self(value)
        }
    }

    impl fmt::Display for DisplayId {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "ID-{}", self.0.0)
        }
    }

    // SOLUTION: C15-E03
    pub trait StrExt {
        fn non_blank(&self) -> Option<&str>;
    }

    impl StrExt for str {
        fn non_blank(&self) -> Option<&str> {
            let trimmed = self.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        }
    }

    // SOLUTION: C15-E04
    pub trait Job {
        fn name(&self) -> &str;

        fn finish(self) -> String
        where
            Self: Sized,
        {
            format!("finalizado: {}", self.name())
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ImportJob(pub String);

    impl Job for ImportJob {
        fn name(&self) -> &str {
            &self.0
        }
    }

    pub fn inspect(job: &dyn Job) -> &str {
        job.name()
    }

    // SOLUTION: C15-E05
    pub trait KeyValueRepository: Default {
        type Error;

        fn save(&mut self, key: String, value: String) -> Result<(), Self::Error>;
        fn find(&self, key: &str) -> Result<Option<&str>, Self::Error>;
        fn delete(&mut self, key: &str) -> Result<bool, Self::Error>;
    }

    #[derive(Default)]
    pub struct MemoryRepository {
        values: HashMap<String, String>,
    }

    impl KeyValueRepository for MemoryRepository {
        type Error = std::convert::Infallible;

        fn save(&mut self, key: String, value: String) -> Result<(), Self::Error> {
            self.values.insert(key, value);
            Ok(())
        }

        fn find(&self, key: &str) -> Result<Option<&str>, Self::Error> {
            Ok(self.values.get(key).map(String::as_str))
        }

        fn delete(&mut self, key: &str) -> Result<bool, Self::Error> {
            Ok(self.values.remove(key).is_some())
        }
    }

    pub fn assert_repository_contract<R>()
    where
        R: KeyValueRepository,
        R::Error: fmt::Debug,
    {
        let mut repository = R::default();
        repository
            .save(String::from("language"), String::from("Rust"))
            .unwrap();
        assert_eq!(repository.find("language").unwrap(), Some("Rust"));

        repository
            .save(String::from("language"), String::from("Rust 2024"))
            .unwrap();
        assert_eq!(repository.find("language").unwrap(), Some("Rust 2024"));

        assert!(repository.delete("language").unwrap());
        assert!(!repository.delete("language").unwrap());
    }

    pub struct Shared<T> {
        inner: Rc<T>,
    }

    impl<T> Shared<T> {
        pub fn new(value: T) -> Self {
            Self {
                inner: Rc::new(value),
            }
        }

        pub fn shares_allocation_with(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.inner, &other.inner)
        }
    }

    // SOLUTION: C15-E06
    impl<T> Clone for Shared<T> {
        fn clone(&self) -> Self {
            Self {
                inner: Rc::clone(&self.inner),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum MessageKind {
        Info,
        Warning,
        Error,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Message {
        pub kind: MessageKind,
        pub body: String,
    }

    // SOLUTION: C15-E07
    pub trait MessageSink {
        fn send(&mut self, message: Message);

        fn info(&mut self, body: &str) {
            self.send(Message {
                kind: MessageKind::Info,
                body: body.to_owned(),
            });
        }

        fn warning(&mut self, body: &str) {
            self.send(Message {
                kind: MessageKind::Warning,
                body: body.to_owned(),
            });
        }

        fn error(&mut self, body: &str) {
            self.send(Message {
                kind: MessageKind::Error,
                body: body.to_owned(),
            });
        }

        fn send_all(&mut self, messages: &[Message]) {
            for message in messages {
                self.send(message.clone());
            }
        }
    }

    #[derive(Default)]
    pub struct RecordingSink {
        pub messages: Vec<Message>,
    }

    impl MessageSink for RecordingSink {
        fn send(&mut self, message: Message) {
            self.messages.push(message);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn both_clock_implementations_are_deterministic() {
            assert!(is_expired(&FixedClock(100), 99));
            assert!(!is_expired(&FixedClock(100), 101));

            let advancing = AdvancingClock::new(10, 5);
            assert!(!is_expired(&advancing, 12));
            assert!(is_expired(&advancing, 12));
        }

        #[test]
        fn local_newtype_owns_the_external_formatting_contract() {
            let display = DisplayId::from(ExternalId(7));
            assert_eq!(display.to_string(), "ID-7");
        }

        #[test]
        fn extension_trait_returns_a_borrowed_non_blank_view() {
            assert_eq!("  Rust  ".non_blank(), Some("Rust"));
            assert_eq!("   ".non_blank(), None);
            assert_eq!("".non_blank(), None);
        }

        #[test]
        fn sized_method_is_used_only_on_the_concrete_job() {
            let job = ImportJob(String::from("users"));
            assert_eq!(inspect(&job), "users");
            assert_eq!(job.finish(), "finalizado: users");
        }

        #[test]
        fn repository_obeys_every_documented_law() {
            assert_repository_contract::<MemoryRepository>();
        }

        #[test]
        fn manual_clone_does_not_require_the_inner_type_to_be_clone() {
            struct Connection;

            let shared = Shared::new(Connection);
            let copy = shared.clone();
            assert!(shared.shares_allocation_with(&copy));
        }

        #[test]
        fn one_required_method_provides_four_defaults() {
            let mut sink = RecordingSink::default();
            sink.info("started");
            sink.warning("slow");
            sink.error("failed");
            sink.send_all(&[
                Message {
                    kind: MessageKind::Info,
                    body: String::from("retry"),
                },
                Message {
                    kind: MessageKind::Info,
                    body: String::from("done"),
                },
            ]);

            assert_eq!(sink.messages.len(), 5);
            assert_eq!(sink.messages[0].kind, MessageKind::Info);
            assert_eq!(sink.messages[1].kind, MessageKind::Warning);
            assert_eq!(sink.messages[2].kind, MessageKind::Error);
            assert_eq!(sink.messages[4].body, "done");
        }
    }
}

pub mod c16 {
    use std::ops::Add;
    use std::path::Path;
    use std::str::FromStr;

    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub struct Score(u32);

    impl Score {
        pub fn new(value: u32) -> Self {
            Self(value)
        }

        pub fn get(&self) -> u32 {
            self.0
        }
    }

    // SOLUTION: C16-E01
    pub fn max_ref<'a, T: Ord>(left: &'a T, right: &'a T) -> &'a T {
        if left >= right { left } else { right }
    }

    // SOLUTION: C16-E02
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
        cells: [[T; COLS]; ROWS],
    }

    impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
        pub fn new(cells: [[T; COLS]; ROWS]) -> Self {
            Self { cells }
        }

        pub fn get(&self, row: usize, column: usize) -> Option<&T> {
            self.cells.get(row).and_then(|line| line.get(column))
        }

        pub fn dimensions(&self) -> (usize, usize) {
            (ROWS, COLS)
        }
    }

    pub trait Operation {
        fn apply(&self, value: i64) -> i64;
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Increase(pub i64);

    impl Operation for Increase {
        fn apply(&self, value: i64) -> i64 {
            value + self.0
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Scale(pub i64);

    impl Operation for Scale {
        fn apply(&self, value: i64) -> i64 {
            value * self.0
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum OperationKind {
        Increase(i64),
        Scale(i64),
    }

    impl OperationKind {
        pub fn apply(self, value: i64) -> i64 {
            match self {
                Self::Increase(amount) => value + amount,
                Self::Scale(factor) => value * factor,
            }
        }
    }

    // SOLUTION: C16-E03
    pub fn apply_static(operation: &impl Operation, value: i64) -> i64 {
        operation.apply(value)
    }

    pub fn apply_closed(operation: OperationKind, value: i64) -> i64 {
        operation.apply(value)
    }

    pub fn apply_dynamic(operation: &dyn Operation, value: i64) -> i64 {
        operation.apply(value)
    }

    // SOLUTION: C16-E04
    pub fn file_name(path: impl AsRef<Path>) -> Option<String> {
        file_name_core(path.as_ref())
    }

    fn file_name_core(path: &Path) -> Option<String> {
        path.file_name()?.to_str().map(str::to_owned)
    }

    // SOLUTION: C16-E05
    pub fn parse_pair<T>(left: &str, right: &str) -> Result<(T, T), T::Err>
    where
        T: FromStr,
    {
        Ok((left.parse()?, right.parse()?))
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Versioned<T> {
        pub version: u64,
        pub value: T,
    }

    // SOLUTION: C16-E06
    impl<T> Versioned<T> {
        pub fn map<U>(self, operation: impl FnOnce(T) -> U) -> Versioned<U> {
            Versioned {
                version: self.version,
                value: operation(self.value),
            }
        }
    }

    // SOLUTION: C16-E07
    pub fn twice<T>(value: T) -> T
    where
        T: Add<Output = T> + Copy,
    {
        value + value
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Cents(pub u64);

    impl Add for Cents {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            Self(self.0 + other.0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn borrowed_maximum_does_not_require_clone() {
            let left = Score::new(10);
            let right = Score::new(30);
            assert_eq!(max_ref(&left, &right).get(), 30);
        }

        #[test]
        fn dimensions_are_part_of_the_matrix_type() {
            let matrix = Matrix::<_, 2, 3>::new([[1, 2, 3], [4, 5, 6]]);
            assert_eq!(matrix.dimensions(), (2, 3));
            assert_eq!(matrix.get(1, 2), Some(&6));
            assert_eq!(matrix.get(2, 0), None);
        }

        #[test]
        fn three_dispatch_strategies_preserve_the_operation_semantics() {
            let increase = Increase(5);
            assert_eq!(apply_static(&increase, 10), 15);
            assert_eq!(apply_closed(OperationKind::Increase(5), 10), 15);
            assert_eq!(apply_dynamic(&increase, 10), 15);

            let scale = Scale(3);
            assert_eq!(apply_static(&scale, 10), 30);
            assert_eq!(apply_closed(OperationKind::Scale(3), 10), 30);
            assert_eq!(apply_dynamic(&scale, 10), 30);
        }

        #[test]
        fn generic_path_wrapper_delegates_to_one_concrete_core() {
            assert_eq!(file_name("reports/book.md"), Some(String::from("book.md")));
            assert_eq!(
                file_name(Path::new("notes.txt")),
                Some(String::from("notes.txt"))
            );
            assert_eq!(file_name(Path::new("")), None);
        }

        #[test]
        fn destination_type_drives_generic_parsing() {
            assert_eq!(parse_pair::<u16>("80", "443"), Ok((80, 443)));
            let coordinates: (i64, i64) = parse_pair("-3", "7").unwrap();
            assert_eq!(coordinates, (-3, 7));
            assert!(parse_pair::<u32>("bad", "7").is_err());
        }

        #[test]
        fn map_changes_the_value_but_preserves_version() {
            let original = Versioned {
                version: 7,
                value: String::from("Rust"),
            };
            assert_eq!(
                original.map(|value| value.len()),
                Versioned {
                    version: 7,
                    value: 4
                }
            );
        }

        #[test]
        fn implementing_add_satisfies_the_bound_used_by_twice() {
            assert_eq!(twice(Cents(25)), Cents(50));
        }
    }
}

pub mod c17 {
    // SOLUTION: C17-E01
    pub trait Repository {
        type Entity;
        type Error;

        fn find(&self, id: u64) -> Result<Option<Self::Entity>, Self::Error>;
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct User {
        pub id: u64,
        pub name: String,
    }

    #[derive(Default)]
    pub struct UserRepository {
        users: Vec<User>,
    }

    impl UserRepository {
        pub fn new(users: Vec<User>) -> Self {
            Self { users }
        }
    }

    impl Repository for UserRepository {
        type Entity = User;
        type Error = std::convert::Infallible;

        fn find(&self, id: u64) -> Result<Option<Self::Entity>, Self::Error> {
            Ok(self.users.iter().find(|user| user.id == id).cloned())
        }
    }

    // SOLUTION: C17-E02
    pub fn total_text_len<'a, I>(items: I) -> usize
    where
        I: IntoIterator<Item = &'a str>,
    {
        items.into_iter().map(str::len).sum()
    }

    // SOLUTION: C17-E03
    pub trait ViewStore {
        type View<'a>
        where
            Self: 'a;

        fn view(&self, index: usize) -> Option<Self::View<'_>>;
    }

    impl ViewStore for Vec<String> {
        type View<'a> = &'a str;

        fn view(&self, index: usize) -> Option<Self::View<'_>> {
            self.get(index).map(String::as_str)
        }
    }

    // SOLUTION: C17-E04
    pub trait LendingIterator {
        type Item<'a>
        where
            Self: 'a;

        // El lifetime explícito muestra la relación entre cada préstamo y el GAT.
        #[allow(clippy::needless_lifetimes)]
        fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
    }

    pub struct SliceLender<'slice, T> {
        slice: &'slice mut [T],
        position: usize,
    }

    impl<'slice, T> SliceLender<'slice, T> {
        pub fn new(slice: &'slice mut [T]) -> Self {
            Self { slice, position: 0 }
        }
    }

    impl<T> LendingIterator for SliceLender<'_, T> {
        type Item<'a>
            = &'a mut T
        where
            Self: 'a;

        fn next(&mut self) -> Option<&mut T> {
            let index = self.position;
            self.position = self.position.saturating_add(1);
            self.slice.get_mut(index)
        }
    }

    pub trait Parser {
        type Output;
        type Error;

        fn parse(&self, input: &str) -> Result<Self::Output, Self::Error>;
    }

    pub struct U32Parser;

    // SOLUTION: C17-E05
    impl Parser for U32Parser {
        type Output = u32;
        type Error = std::num::ParseIntError;

        fn parse(&self, input: &str) -> Result<u32, Self::Error> {
            input.parse()
        }
    }

    pub fn parse_twice<P>(
        parser: &P,
        left: &str,
        right: &str,
    ) -> Result<(P::Output, P::Output), P::Error>
    where
        P: Parser,
    {
        Ok((parser.parse(left)?, parser.parse(right)?))
    }

    // SOLUTION: C17-E06
    pub struct WindowsMut<'slice, T> {
        slice: &'slice mut [T],
        size: usize,
        position: usize,
    }

    impl<'slice, T> WindowsMut<'slice, T> {
        pub fn new(slice: &'slice mut [T], size: usize) -> Self {
            Self {
                slice,
                size,
                position: 0,
            }
        }
    }

    impl<T> LendingIterator for WindowsMut<'_, T> {
        type Item<'a>
            = &'a mut [T]
        where
            Self: 'a;

        fn next(&mut self) -> Option<&mut [T]> {
            let start = self.position;
            let end = start.checked_add(self.size)?;
            if self.size == 0 || end > self.slice.len() {
                return None;
            }
            self.position += 1;
            Some(&mut self.slice[start..end])
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum RequireError<E> {
        Backend(E),
        NotFound { id: u64 },
    }

    // SOLUTION: C17-E07
    pub fn require<R>(repository: &R, id: u64) -> Result<R::Entity, RequireError<R::Error>>
    where
        R: Repository,
    {
        repository
            .find(id)
            .map_err(RequireError::Backend)?
            .ok_or(RequireError::NotFound { id })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn repository_fixes_entity_and_error_for_its_implementation() {
            let repo = UserRepository::new(vec![User {
                id: 1,
                name: String::from("Ada"),
            }]);
            assert_eq!(repo.find(1).unwrap().unwrap().name, "Ada");
            assert_eq!(repo.find(2).unwrap(), None);
        }

        #[test]
        fn explicit_item_lifetime_accepts_arrays_and_vectors_of_views() {
            assert_eq!(total_text_len(["Rust", "GAT"]), 7);
            let values: Vec<_> = ["Ada", "Grace"].into_iter().map(String::from).collect();
            let views = values.iter().map(String::as_str).collect::<Vec<_>>();
            assert_eq!(total_text_len(views), 8);
        }

        #[test]
        fn gat_view_is_tied_to_each_borrow_of_the_store() {
            let values = vec![String::from("zero-copy")];
            assert_eq!(values.view(0), Some("zero-copy"));
            assert_eq!(values.view(1), None);
        }

        #[test]
        fn slice_lender_returns_one_mutable_borrow_at_a_time() {
            let mut values = [1, 2, 3];
            let mut lender = SliceLender::new(&mut values);
            *lender.next().unwrap() += 10;
            *lender.next().unwrap() += 20;
            assert_eq!(values, [11, 22, 3]);
        }

        #[test]
        fn associated_output_is_canonical_for_the_parser_impl() {
            assert_eq!(parse_twice(&U32Parser, "10", "20"), Ok((10, 20)));
            assert!(parse_twice(&U32Parser, "bad", "20").is_err());
        }

        #[test]
        fn overlapping_windows_can_be_used_successively() {
            let mut values = [1, 2, 3, 4];
            let mut windows = WindowsMut::new(&mut values, 2);
            windows.next().unwrap()[1] = 20;
            windows.next().unwrap()[1] = 30;
            assert_eq!(values, [1, 20, 30, 4]);
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        struct BackendUnavailable;

        struct FailingRepository;

        impl Repository for FailingRepository {
            type Entity = User;
            type Error = BackendUnavailable;

            fn find(&self, _id: u64) -> Result<Option<User>, BackendUnavailable> {
                Err(BackendUnavailable)
            }
        }

        #[test]
        fn require_adds_not_found_without_erasing_backend_errors() {
            let repo = UserRepository::new(vec![User {
                id: 1,
                name: String::from("Ada"),
            }]);
            assert_eq!(require(&repo, 1).unwrap().name, "Ada");
            assert_eq!(require(&repo, 2), Err(RequireError::NotFound { id: 2 }));
            assert_eq!(
                require(&FailingRepository, 1),
                Err(RequireError::Backend(BackendUnavailable))
            );
        }
    }
}

pub mod c18 {
    use std::fmt::{Debug, Display};

    // SOLUTION: C18-E01
    pub fn positive_doubled(values: &[i32]) -> impl Iterator<Item = i32> + '_ {
        values
            .iter()
            .copied()
            .filter(|value| *value > 0)
            .map(|value| value * 2)
    }

    // SOLUTION: C18-E02
    #[derive(Clone, Debug)]
    pub enum EitherIter<A, B> {
        Left(A),
        Right(B),
    }

    impl<T, A, B> Iterator for EitherIter<A, B>
    where
        A: Iterator<Item = T>,
        B: Iterator<Item = T>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::Left(iterator) => iterator.next(),
                Self::Right(iterator) => iterator.next(),
            }
        }
    }

    pub fn numbers(reversed: bool) -> impl Iterator<Item = u32> {
        if reversed {
            EitherIter::Right((0..3).rev())
        } else {
            EitherIter::Left(0..3)
        }
    }

    pub fn boxed_numbers(reversed: bool) -> Box<dyn Iterator<Item = u32>> {
        if reversed {
            Box::new((0..3).rev())
        } else {
            Box::new(0..3)
        }
    }

    // SOLUTION: C18-E03
    pub fn render_pair(left: impl Display, right: impl Display) -> String {
        format!("{left} | {right}")
    }

    pub fn equal<T: PartialEq>(left: T, right: T) -> bool {
        left == right
    }

    // SOLUTION: C18-E04
    pub fn text_length(text: &str) -> impl Copy + Debug + PartialEq<usize> + use<> {
        text.len()
    }

    // SOLUTION: C18-E05
    pub trait Catalog {
        fn owned_names(&self) -> impl Iterator<Item = String> + Send;
    }

    pub struct VecCatalog(pub Vec<String>);

    impl Catalog for VecCatalog {
        fn owned_names(&self) -> impl Iterator<Item = String> + Send {
            self.0.clone().into_iter()
        }
    }

    pub fn count_send_names<C: Catalog>(catalog: &C) -> usize {
        fn require_send<T: Send>(value: T) -> T {
            value
        }

        require_send(catalog.owned_names()).count()
    }

    // SOLUTION: C18-E06
    pub fn minimum_length(minimum: usize) -> impl Fn(&str) -> bool {
        move |value| value.len() >= minimum
    }

    // SOLUTION: C18-E07
    #[derive(Clone, Debug)]
    pub enum Either3<A, B, C> {
        First(A),
        Second(B),
        Third(C),
    }

    impl<T, A, B, C> Iterator for Either3<A, B, C>
    where
        A: Iterator<Item = T>,
        B: Iterator<Item = T>,
        C: Iterator<Item = T>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::First(iterator) => iterator.next(),
                Self::Second(iterator) => iterator.next(),
                Self::Third(iterator) => iterator.next(),
            }
        }
    }

    pub fn three_way(
        branch: u8,
    ) -> Either3<std::ops::Range<u32>, std::iter::Rev<std::ops::Range<u32>>, std::iter::Once<u32>>
    {
        match branch {
            0 => Either3::First(0..3),
            1 => Either3::Second((0..3).rev()),
            _ => Either3::Third(std::iter::once(42)),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn rpit_hides_a_borrowing_iterator_pipeline() {
            assert_eq!(
                positive_doubled(&[-2, 3, 0, 5]).collect::<Vec<_>>(),
                [6, 10]
            );
        }

        #[test]
        fn enum_and_box_unify_distinct_iterator_types() {
            assert_eq!(numbers(false).collect::<Vec<_>>(), [0, 1, 2]);
            assert_eq!(numbers(true).collect::<Vec<_>>(), [2, 1, 0]);
            assert_eq!(boxed_numbers(false).collect::<Vec<_>>(), [0, 1, 2]);
            assert_eq!(boxed_numbers(true).collect::<Vec<_>>(), [2, 1, 0]);
        }

        #[test]
        fn independent_apit_parameters_accept_different_types() {
            assert_eq!(render_pair(7, "days"), "7 | days");
            assert!(equal(String::from("Rust"), String::from("Rust")));
        }

        #[test]
        fn precise_capture_keeps_the_result_independent_from_the_input() {
            let result;
            {
                let text = String::from("Rust");
                result = text_length(&text);
            }
            assert_eq!(result, 4);
        }

        #[test]
        fn rpitit_contract_guarantees_a_send_iterator() {
            let catalog = VecCatalog(vec![String::from("Rust"), String::from("Cargo")]);
            assert_eq!(count_send_names(&catalog), 2);
        }

        #[test]
        fn opaque_closure_owns_its_configuration() {
            let validate = minimum_length(4);
            assert!(validate("Rust"));
            assert!(!validate("C"));
        }

        #[test]
        fn three_variant_enum_preserves_every_branch() {
            assert_eq!(three_way(0).collect::<Vec<_>>(), [0, 1, 2]);
            assert_eq!(three_way(1).collect::<Vec<_>>(), [2, 1, 0]);
            assert_eq!(three_way(2).collect::<Vec<_>>(), [42]);
        }
    }
}

pub mod c19 {
    // SOLUTION: C19-E01
    pub trait Renderer {
        fn render(&self) -> String;
    }

    pub struct Text(pub String);
    pub struct Number(pub i64);

    impl Renderer for Text {
        fn render(&self) -> String {
            self.0.clone()
        }
    }

    impl Renderer for Number {
        fn render(&self) -> String {
            self.0.to_string()
        }
    }

    pub fn render_all(renderers: &[Box<dyn Renderer>]) -> Vec<String> {
        renderers.iter().map(|renderer| renderer.render()).collect()
    }

    // SOLUTION: C19-E02
    pub trait Encode {
        fn encode(&self) -> Vec<u8>;
    }

    impl Encode for u32 {
        fn encode(&self) -> Vec<u8> {
            self.to_string().into_bytes()
        }
    }

    pub trait ByteStore {
        fn save_bytes(&mut self, key: &str, bytes: &[u8]);
    }

    #[derive(Default)]
    pub struct MemoryByteStore {
        pub entries: Vec<(String, Vec<u8>)>,
    }

    impl ByteStore for MemoryByteStore {
        fn save_bytes(&mut self, key: &str, bytes: &[u8]) {
            self.entries.push((key.to_owned(), bytes.to_vec()));
        }
    }

    pub fn save<S, T>(store: &mut S, key: &str, value: &T)
    where
        S: ByteStore + ?Sized,
        T: Encode,
    {
        store.save_bytes(key, &value.encode());
    }

    // SOLUTION: C19-E03
    pub trait Repository {
        fn find_name(&self, id: u64) -> Option<String>;
    }

    pub struct MemoryRepository;

    impl Repository for MemoryRepository {
        fn find_name(&self, id: u64) -> Option<String> {
            (id == 1).then(|| String::from("Ada"))
        }
    }

    pub struct StaticService<R> {
        repository: R,
    }

    impl<R> StaticService<R> {
        pub fn new(repository: R) -> Self {
            Self { repository }
        }
    }

    impl<R: Repository> StaticService<R> {
        pub fn name(&self, id: u64) -> Option<String> {
            self.repository.find_name(id)
        }
    }

    pub struct DynamicService {
        repository: Box<dyn Repository>,
    }

    impl DynamicService {
        pub fn new(repository: Box<dyn Repository>) -> Self {
            Self { repository }
        }

        pub fn name(&self, id: u64) -> Option<String> {
            self.repository.find_name(id)
        }
    }

    // SOLUTION: C19-E04
    pub struct BorrowedText<'a>(pub &'a str);

    impl Renderer for BorrowedText<'_> {
        fn render(&self) -> String {
            self.0.to_owned()
        }
    }

    pub fn boxed_view<'a>(text: &'a str) -> Box<dyn Renderer + 'a> {
        Box::new(BorrowedText(text))
    }

    // SOLUTION: C19-E05
    pub trait Describe {
        fn describe(&self) -> String;
    }

    impl Describe for Text {
        fn describe(&self) -> String {
            format!("text:{}", self.0.len())
        }
    }

    impl Describe for Number {
        fn describe(&self) -> String {
            format!("number:{}", self.0)
        }
    }

    pub fn describe_all(values: &[Box<dyn Describe>]) -> Vec<String> {
        values.iter().map(|value| value.describe()).collect()
    }

    pub trait Sink {
        fn send(&mut self, kind: &str, value: String);
    }

    #[derive(Default)]
    pub struct MemorySink {
        pub values: Vec<String>,
    }

    impl Sink for MemorySink {
        fn send(&mut self, _kind: &str, value: String) {
            self.values.push(value);
        }
    }

    // SOLUTION: C19-E06
    pub struct FilteredSink<S, P> {
        inner: S,
        predicate: P,
    }

    impl<S, P> FilteredSink<S, P> {
        pub fn new(inner: S, predicate: P) -> Self {
            Self { inner, predicate }
        }

        pub fn into_inner(self) -> S {
            self.inner
        }
    }

    impl<S, P> Sink for FilteredSink<S, P>
    where
        S: Sink,
        P: Fn(&str) -> bool,
    {
        fn send(&mut self, kind: &str, value: String) {
            if (self.predicate)(kind) {
                self.inner.send(kind, value);
            }
        }
    }

    // SOLUTION: C19-E07
    pub trait Operation {
        fn apply(&self, value: i64) -> i64;
    }

    pub struct Increase(pub i64);
    pub struct Scale(pub i64);
    pub struct Negate;

    impl Operation for Increase {
        fn apply(&self, value: i64) -> i64 {
            value + self.0
        }
    }

    impl Operation for Scale {
        fn apply(&self, value: i64) -> i64 {
            value * self.0
        }
    }

    impl Operation for Negate {
        fn apply(&self, value: i64) -> i64 {
            -value
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum OperationKind {
        Increase(i64),
        Scale(i64),
        Negate,
    }

    impl OperationKind {
        pub fn apply(self, value: i64) -> i64 {
            match self {
                Self::Increase(amount) => value + amount,
                Self::Scale(factor) => value * factor,
                Self::Negate => -value,
            }
        }
    }

    pub fn run_closed(operations: &[OperationKind], value: i64) -> Vec<i64> {
        operations
            .iter()
            .map(|operation| operation.apply(value))
            .collect()
    }

    pub fn run_dynamic(operations: &[Box<dyn Operation>], value: i64) -> Vec<i64> {
        operations
            .iter()
            .map(|operation| operation.apply(value))
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn trait_objects_store_heterogeneous_values() {
            let values: Vec<Box<dyn Renderer>> =
                vec![Box::new(Text(String::from("Rust"))), Box::new(Number(2024))];
            assert_eq!(render_all(&values), ["Rust", "2024"]);
        }

        #[test]
        fn generic_encoding_stays_outside_the_dyn_compatible_store() {
            let mut store = MemoryByteStore::default();
            let erased: &mut dyn ByteStore = &mut store;
            save(erased, "answer", &42_u32);
            assert_eq!(store.entries, [(String::from("answer"), b"42".to_vec())]);
        }

        #[test]
        fn static_and_dynamic_services_preserve_repository_semantics() {
            let static_service = StaticService::new(MemoryRepository);
            let dynamic_service = DynamicService::new(Box::new(MemoryRepository));
            assert_eq!(static_service.name(1).as_deref(), Some("Ada"));
            assert_eq!(dynamic_service.name(1).as_deref(), Some("Ada"));
            assert_eq!(static_service.name(2), None);
            assert_eq!(dynamic_service.name(2), None);
        }

        #[test]
        fn trait_object_can_borrow_instead_of_requiring_static_data() {
            let text = String::from("borrowed");
            let renderer = boxed_view(&text);
            assert_eq!(renderer.render(), "borrowed");
        }

        #[test]
        fn domain_operation_replaces_type_downcasts() {
            let values: Vec<Box<dyn Describe>> =
                vec![Box::new(Text(String::from("Rust"))), Box::new(Number(2024))];
            assert_eq!(describe_all(&values), ["text:4", "number:2024"]);
        }

        #[test]
        fn wrapper_composes_filtering_with_an_existing_sink() {
            let mut sink = FilteredSink::new(MemorySink::default(), |kind: &str| kind == "audit");
            sink.send("debug", String::from("ignored"));
            sink.send("audit", String::from("stored"));
            assert_eq!(sink.into_inner().values, ["stored"]);
        }

        #[test]
        fn enum_and_trait_objects_apply_the_same_three_operations() {
            let closed = [
                OperationKind::Increase(5),
                OperationKind::Scale(3),
                OperationKind::Negate,
            ];
            let dynamic: Vec<Box<dyn Operation>> =
                vec![Box::new(Increase(5)), Box::new(Scale(3)), Box::new(Negate)];
            assert_eq!(run_closed(&closed, 10), [15, 30, -10]);
            assert_eq!(run_dynamic(&dynamic, 10), run_closed(&closed, 10));
        }
    }
}

pub mod c20 {
    use std::cell::Cell;
    use std::marker::PhantomData;

    // SOLUTION: C20-E01
    pub fn shorten<'short>(value: &'static str) -> &'short str {
        value
    }

    // SOLUTION: C20-E02
    pub fn assign<'a>(slot: &mut &'a str, value: &'a str) {
        *slot = value;
    }

    // SOLUTION: C20-E03
    pub fn visit_text<F>(values: &[String], visitor: F)
    where
        F: for<'a> Fn(&'a str),
    {
        for value in values {
            visitor(value);
        }
    }

    // SOLUTION: C20-E04
    pub struct Owns<T> {
        id: usize,
        marker: PhantomData<T>,
    }

    impl<T> Owns<T> {
        pub fn new(id: usize) -> Self {
            Self {
                id,
                marker: PhantomData,
            }
        }

        pub fn id(&self) -> usize {
            self.id
        }
    }

    pub struct Consumes<T> {
        id: usize,
        marker: PhantomData<fn(T)>,
    }

    impl<T> Consumes<T> {
        pub fn new(id: usize) -> Self {
            Self {
                id,
                marker: PhantomData,
            }
        }

        pub fn id(&self) -> usize {
            self.id
        }
    }

    // SOLUTION: C20-E05
    pub fn with_local<F>(callback: F) -> usize
    where
        F: for<'a> Fn(&'a str) -> usize,
    {
        let local = String::from("internal");
        callback(&local)
    }

    // SOLUTION: C20-E06
    pub struct Viewer<'a> {
        pub view: &'a str,
    }

    pub struct Editor<'a> {
        pub view: &'a str,
        pub selected: Cell<&'a str>,
    }

    pub fn shorten_viewer<'short>(viewer: Viewer<'static>) -> Viewer<'short> {
        viewer
    }

    // SOLUTION: C20-E07
    pub fn identity(value: &str) -> &str {
        value
    }

    pub fn apply_identity<F>(callback: F, value: &str) -> String
    where
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        callback(value).to_owned()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cell::RefCell;

        #[test]
        fn static_reference_can_be_shortened_to_the_required_scope() {
            // El tipo concreto hace visible que la salida elidida se liga al anchor.
            #[allow(clippy::ptr_arg)]
            fn tied_to(_anchor: &String) -> &str {
                shorten("long lived")
            }

            let anchor = String::from("anchor");
            assert_eq!(tied_to(&anchor), "long lived");
        }

        #[test]
        fn mutable_slot_accepts_a_value_with_its_declared_lifetime() {
            let first = String::from("first");
            let second = String::from("second");
            let mut slot: &str = &first;
            assign(&mut slot, &second);
            assert_eq!(slot, "second");
        }

        #[test]
        fn callback_accepts_a_fresh_borrow_for_every_call() {
            let seen = RefCell::new(Vec::new());
            let values = vec![String::from("A"), String::from("B")];
            visit_text(&values, |value| seen.borrow_mut().push(value.to_owned()));
            assert_eq!(seen.into_inner(), ["A", "B"]);
        }

        #[test]
        fn phantom_markers_express_different_relations_without_adding_size() {
            assert_eq!(
                std::mem::size_of::<Owns<String>>(),
                std::mem::size_of::<usize>()
            );
            assert_eq!(
                std::mem::size_of::<Consumes<String>>(),
                std::mem::size_of::<usize>()
            );
            assert_eq!(Owns::<String>::new(1).id(), 1);
            assert_eq!(Consumes::<String>::new(2).id(), 2);
        }

        #[test]
        fn higher_ranked_callback_accepts_data_created_inside_the_callee() {
            assert_eq!(with_local(str::len), 8);
            assert_eq!(with_local(|value: &str| value.chars().count()), 8);
        }

        #[test]
        fn covariant_view_can_shorten_while_editor_keeps_one_exact_lifetime() {
            let viewer = Viewer { view: "document" };
            assert_eq!(shorten_viewer(viewer).view, "document");

            let editor = Editor {
                view: "document",
                selected: Cell::new("doc"),
            };
            assert_eq!(editor.view, "document");
            assert_eq!(editor.selected.get(), "doc");
        }

        #[test]
        fn general_identity_returns_a_borrow_from_each_input() {
            let local = String::from("Rust");
            assert_eq!(apply_identity(identity, &local), "Rust");
        }
    }
}
