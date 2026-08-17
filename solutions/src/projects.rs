//! Núcleos ejecutables de los proyectos de consolidación del capítulo 57.
//!
//! Cada módulo verifica la invariante central de un proyecto. No sustituye el
//! entregable completo con sus crates, adaptadores, CI, documentación y pruebas
//! de integración.

pub mod p01_domain_ids {
    use std::fmt;
    use std::num::NonZeroU64;
    use std::str::FromStr;

    // SOLUTION: C57-P01
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UserId(NonZeroU64);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct OrderId(NonZeroU64);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum IdError {
        NotANumber,
        Zero,
    }

    impl UserId {
        pub fn get(self) -> u64 {
            self.0.get()
        }
    }

    impl OrderId {
        pub fn get(self) -> u64 {
            self.0.get()
        }
    }

    macro_rules! numeric_id {
        ($type:ty) => {
            impl TryFrom<u64> for $type {
                type Error = IdError;

                fn try_from(value: u64) -> Result<Self, Self::Error> {
                    NonZeroU64::new(value).map(Self).ok_or(IdError::Zero)
                }
            }

            impl FromStr for $type {
                type Err = IdError;

                fn from_str(value: &str) -> Result<Self, Self::Err> {
                    value
                        .parse::<u64>()
                        .map_err(|_| IdError::NotANumber)?
                        .try_into()
                }
            }

            impl fmt::Display for $type {
                fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.0.fmt(formatter)
                }
            }
        };
    }

    numeric_id!(UserId);
    numeric_id!(OrderId);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Email(String);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EmailError {
        MissingLocalPart,
        MissingDomain,
        MoreThanOneAtSign,
    }

    impl TryFrom<&str> for Email {
        type Error = EmailError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let mut parts = value.split('@');
            let local = parts.next().unwrap_or_default();
            let domain = parts.next().ok_or(EmailError::MissingDomain)?;
            if parts.next().is_some() {
                return Err(EmailError::MoreThanOneAtSign);
            }
            if local.is_empty() {
                return Err(EmailError::MissingLocalPart);
            }
            if domain.is_empty() {
                return Err(EmailError::MissingDomain);
            }
            Ok(Self(value.to_owned()))
        }
    }

    impl fmt::Display for Email {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(formatter)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Percentage(u8);

    impl TryFrom<u8> for Percentage {
        type Error = u8;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            (value <= 100).then_some(Self(value)).ok_or(value)
        }
    }

    impl Percentage {
        pub fn get(self) -> u8 {
            self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Currency {
        Eur,
        Usd,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Money {
        minor_units: i64,
        currency: Currency,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MoneyError {
        CurrencyMismatch,
        Overflow,
    }

    impl Money {
        pub fn new(minor_units: i64, currency: Currency) -> Self {
            Self {
                minor_units,
                currency,
            }
        }

        pub fn checked_add(self, other: Self) -> Result<Self, MoneyError> {
            if self.currency != other.currency {
                return Err(MoneyError::CurrencyMismatch);
            }
            let minor_units = self
                .minor_units
                .checked_add(other.minor_units)
                .ok_or(MoneyError::Overflow)?;
            Ok(Self::new(minor_units, self.currency))
        }

        pub fn minor_units(self) -> i64 {
            self.minor_units
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn identifiers_round_trip_without_becoming_interchangeable() {
            let user: UserId = "42".parse().unwrap();
            let order: OrderId = "42".parse().unwrap();
            assert_eq!(user.to_string().parse(), Ok(user));
            assert_eq!(order.to_string().parse(), Ok(order));
            assert_eq!(user.get(), order.get());
            assert_eq!("0".parse::<UserId>(), Err(IdError::Zero));
        }

        #[test]
        fn value_objects_reject_their_boundaries() {
            assert_eq!(Percentage::try_from(100).map(Percentage::get), Ok(100));
            assert_eq!(Percentage::try_from(101), Err(101));
            assert_eq!(
                Email::try_from("@example.com"),
                Err(EmailError::MissingLocalPart)
            );
            assert_eq!(Email::try_from("a@b@c"), Err(EmailError::MoreThanOneAtSign));
        }

        #[test]
        fn money_never_mix_currencies_or_wraps() {
            let euro = Money::new(80, Currency::Eur);
            assert_eq!(
                euro.checked_add(Money::new(20, Currency::Eur))
                    .unwrap()
                    .minor_units(),
                100
            );
            assert_eq!(
                euro.checked_add(Money::new(20, Currency::Usd)),
                Err(MoneyError::CurrencyMismatch)
            );
            assert_eq!(
                Money::new(i64::MAX, Currency::Eur).checked_add(Money::new(1, Currency::Eur)),
                Err(MoneyError::Overflow)
            );
        }
    }
}

pub mod p02_record_import {
    use std::collections::HashSet;

    // SOLUTION: C57-P02
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Record<'a> {
        pub key: &'a str,
        pub value: i64,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct OwnedRecord {
        pub key: String,
        pub value: i64,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ImportError {
        pub line: usize,
        pub column: usize,
        pub kind: ImportErrorKind,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ImportErrorKind {
        MissingSeparator,
        EmptyKey,
        InvalidNumber,
        DuplicateKey,
    }

    fn parse_line(line_number: usize, line: &str) -> Result<Record<'_>, ImportError> {
        let (key, value) = line.split_once('=').ok_or(ImportError {
            line: line_number,
            column: line.len() + 1,
            kind: ImportErrorKind::MissingSeparator,
        })?;
        if key.is_empty() {
            return Err(ImportError {
                line: line_number,
                column: 1,
                kind: ImportErrorKind::EmptyKey,
            });
        }
        let value = value.parse().map_err(|_| ImportError {
            line: line_number,
            column: key.len() + 2,
            kind: ImportErrorKind::InvalidNumber,
        })?;
        Ok(Record { key, value })
    }

    pub fn import_fail_fast(input: &str) -> Result<Vec<Record<'_>>, ImportError> {
        let mut seen = HashSet::new();
        input
            .lines()
            .enumerate()
            .map(|(index, line)| {
                let record = parse_line(index + 1, line)?;
                if !seen.insert(record.key) {
                    return Err(ImportError {
                        line: index + 1,
                        column: 1,
                        kind: ImportErrorKind::DuplicateKey,
                    });
                }
                Ok(record)
            })
            .collect()
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ImportReport {
        pub accepted: Vec<OwnedRecord>,
        pub rejected: Vec<ImportError>,
    }

    pub fn import_all(input: &str) -> ImportReport {
        let mut seen = HashSet::new();
        let mut accepted = Vec::new();
        let mut rejected = Vec::new();
        for (index, line) in input.lines().enumerate() {
            match parse_line(index + 1, line) {
                Ok(record) if seen.insert(record.key) => accepted.push(OwnedRecord {
                    key: record.key.to_owned(),
                    value: record.value,
                }),
                Ok(_) => rejected.push(ImportError {
                    line: index + 1,
                    column: 1,
                    kind: ImportErrorKind::DuplicateKey,
                }),
                Err(error) => rejected.push(error),
            }
        }
        ImportReport { accepted, rejected }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fail_fast_returns_borrowed_records_and_first_error() {
            let input = String::from("alpha=1\nbeta=2");
            let records = import_fail_fast(&input).unwrap();
            assert_eq!(
                records[0],
                Record {
                    key: "alpha",
                    value: 1
                }
            );
            assert_eq!(records[0].key.as_ptr(), input.as_ptr());
            assert_eq!(import_fail_fast("ok=1\nbad").unwrap_err().line, 2);
        }

        #[test]
        fn accumulating_mode_keeps_valid_rows_and_every_rejection() {
            let report = import_all("a=1\nbad\na=2\nc=nope\nb=3");
            assert_eq!(
                report
                    .accepted
                    .iter()
                    .map(|record| record.key.as_str())
                    .collect::<Vec<_>>(),
                ["a", "b"]
            );
            assert_eq!(
                report
                    .rejected
                    .iter()
                    .map(|error| error.kind)
                    .collect::<Vec<_>>(),
                [
                    ImportErrorKind::MissingSeparator,
                    ImportErrorKind::DuplicateKey,
                    ImportErrorKind::InvalidNumber
                ]
            );
        }

        #[test]
        fn styles_are_equivalent_for_a_valid_corpus() {
            let input = "a=1\nb=-2\nc=3";
            let borrowed = import_fail_fast(input).unwrap();
            let owned = import_all(input);
            assert!(owned.rejected.is_empty());
            assert!(
                borrowed
                    .iter()
                    .zip(&owned.accepted)
                    .all(|(left, right)| left.key == right.key && left.value == right.value)
            );
        }
    }
}

pub mod p03_ledger_core {
    use std::collections::HashMap;

    // SOLUTION: C57-P03
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AccountId(pub u64);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Account {
        balance: u64,
        active: bool,
    }

    impl Account {
        pub fn active(balance: u64) -> Self {
            Self {
                balance,
                active: true,
            }
        }

        pub fn blocked(balance: u64) -> Self {
            Self {
                balance,
                active: false,
            }
        }

        pub fn balance(self) -> u64 {
            self.balance
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TransferError {
        SameAccount,
        MissingAccount(AccountId),
        Blocked(AccountId),
        InsufficientFunds,
        Overflow,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TransferRecorded {
        pub from: AccountId,
        pub to: AccountId,
        pub amount: u64,
    }

    #[derive(Debug, Default)]
    pub struct Ledger {
        accounts: HashMap<AccountId, Account>,
        events: Vec<TransferRecorded>,
    }

    impl Ledger {
        pub fn insert(&mut self, id: AccountId, account: Account) {
            self.accounts.insert(id, account);
        }

        pub fn account(&self, id: AccountId) -> Option<Account> {
            self.accounts.get(&id).copied()
        }

        pub fn events(&self) -> &[TransferRecorded] {
            &self.events
        }

        pub fn transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: u64,
        ) -> Result<TransferRecorded, TransferError> {
            if from == to {
                return Err(TransferError::SameAccount);
            }
            let source = self
                .accounts
                .get(&from)
                .ok_or(TransferError::MissingAccount(from))?;
            let destination = self
                .accounts
                .get(&to)
                .ok_or(TransferError::MissingAccount(to))?;
            if !source.active {
                return Err(TransferError::Blocked(from));
            }
            if !destination.active {
                return Err(TransferError::Blocked(to));
            }
            let source_after = source
                .balance
                .checked_sub(amount)
                .ok_or(TransferError::InsufficientFunds)?;
            let destination_after = destination
                .balance
                .checked_add(amount)
                .ok_or(TransferError::Overflow)?;

            self.accounts.get_mut(&from).unwrap().balance = source_after;
            self.accounts.get_mut(&to).unwrap().balance = destination_after;
            let event = TransferRecorded { from, to, amount };
            self.events.push(event);
            Ok(event)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn ledger(source: u64, destination: u64) -> Ledger {
            let mut ledger = Ledger::default();
            ledger.insert(AccountId(1), Account::active(source));
            ledger.insert(AccountId(2), Account::active(destination));
            ledger
        }

        #[test]
        fn transfer_conserves_the_total_and_records_one_event() {
            let mut ledger = ledger(70, 30);
            let event = ledger.transfer(AccountId(1), AccountId(2), 25).unwrap();
            assert_eq!(event.amount, 25);
            assert_eq!(
                ledger.account(AccountId(1)).unwrap().balance()
                    + ledger.account(AccountId(2)).unwrap().balance(),
                100
            );
            assert_eq!(ledger.events(), &[event]);
        }

        #[test]
        fn every_failed_transfer_is_atomic() {
            let cases = [
                (ledger(3, 4), AccountId(1), AccountId(2), 5),
                (ledger(3, u64::MAX), AccountId(1), AccountId(2), 1),
                (ledger(3, 4), AccountId(1), AccountId(1), 1),
            ];
            for (mut ledger, from, to, amount) in cases {
                let before = ledger.accounts.clone();
                assert!(ledger.transfer(from, to, amount).is_err());
                assert_eq!(ledger.accounts, before);
                assert!(ledger.events.is_empty());
            }
        }

        #[test]
        fn blocked_and_missing_accounts_are_distinct_domain_errors() {
            let mut ledger = Ledger::default();
            ledger.insert(AccountId(1), Account::blocked(20));
            ledger.insert(AccountId(2), Account::active(0));
            assert_eq!(
                ledger.transfer(AccountId(1), AccountId(2), 1),
                Err(TransferError::Blocked(AccountId(1)))
            );
            assert_eq!(
                ledger.transfer(AccountId(9), AccountId(2), 1),
                Err(TransferError::MissingAccount(AccountId(9)))
            );
        }
    }
}

pub mod p04_job_engine {
    use std::collections::VecDeque;

    // SOLUTION: C57-P04
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct JobId(pub u64);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Submit {
        Accepted,
        Full,
        Closed,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Completion {
        Ok(JobId),
        Panicked(JobId),
    }

    #[derive(Debug)]
    pub struct Coordinator {
        capacity: usize,
        accepting: bool,
        queued: VecDeque<JobId>,
        accepted: usize,
        completed: Vec<Completion>,
    }

    impl Coordinator {
        pub fn new(capacity: usize) -> Self {
            assert!(capacity > 0, "the queue must have positive capacity");
            Self {
                capacity,
                accepting: true,
                queued: VecDeque::new(),
                accepted: 0,
                completed: Vec::new(),
            }
        }

        pub fn submit(&mut self, id: JobId) -> Submit {
            if !self.accepting {
                return Submit::Closed;
            }
            if self.queued.len() == self.capacity {
                return Submit::Full;
            }
            self.queued.push_back(id);
            self.accepted += 1;
            Submit::Accepted
        }

        pub fn close_admission(&mut self) {
            self.accepting = false;
        }

        pub fn run_next(&mut self, panic: bool) -> Option<Completion> {
            let id = self.queued.pop_front()?;
            let result = if panic {
                Completion::Panicked(id)
            } else {
                Completion::Ok(id)
            };
            self.completed.push(result);
            Some(result)
        }

        pub fn drain(&mut self) {
            while self.run_next(false).is_some() {}
        }

        pub fn is_reconciled(&self) -> bool {
            self.queued.len() + self.completed.len() == self.accepted
        }

        pub fn completed(&self) -> &[Completion] {
            &self.completed
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn capacity_and_closed_admission_are_observable() {
            let mut engine = Coordinator::new(2);
            assert_eq!(engine.submit(JobId(1)), Submit::Accepted);
            assert_eq!(engine.submit(JobId(2)), Submit::Accepted);
            assert_eq!(engine.submit(JobId(3)), Submit::Full);
            engine.close_admission();
            assert_eq!(engine.submit(JobId(4)), Submit::Closed);
            assert!(engine.is_reconciled());
        }

        #[test]
        fn graceful_drain_loses_no_accepted_job_and_preserves_fifo() {
            let mut engine = Coordinator::new(4);
            for id in 1..=4 {
                assert_eq!(engine.submit(JobId(id)), Submit::Accepted);
            }
            engine.close_admission();
            engine.drain();
            assert_eq!(
                engine.completed(),
                &[
                    Completion::Ok(JobId(1)),
                    Completion::Ok(JobId(2)),
                    Completion::Ok(JobId(3)),
                    Completion::Ok(JobId(4))
                ]
            );
            assert!(engine.is_reconciled());
        }

        #[test]
        fn a_worker_panic_is_a_distinct_completion() {
            let mut engine = Coordinator::new(1);
            engine.submit(JobId(7));
            assert_eq!(engine.run_next(true), Some(Completion::Panicked(JobId(7))));
            assert!(engine.is_reconciled());
        }
    }
}

pub mod p05_async_crawler {
    use std::collections::{HashMap, HashSet, VecDeque};

    // SOLUTION: C57-P05
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Request {
        pub url: String,
        pub attempt: u8,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Enqueue {
        Accepted,
        Duplicate,
        Closed,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Outcome {
        Success,
        RetryableFailure,
        PermanentFailure,
    }

    #[derive(Debug)]
    pub struct CrawlCoordinator {
        max_in_flight: usize,
        max_retries: u8,
        accepting: bool,
        seen: HashSet<String>,
        pending: VecDeque<Request>,
        in_flight: HashMap<String, u8>,
        finished: Vec<(String, Outcome)>,
    }

    impl CrawlCoordinator {
        pub fn new(max_in_flight: usize, max_retries: u8) -> Self {
            assert!(max_in_flight > 0);
            Self {
                max_in_flight,
                max_retries,
                accepting: true,
                seen: HashSet::new(),
                pending: VecDeque::new(),
                in_flight: HashMap::new(),
                finished: Vec::new(),
            }
        }

        pub fn enqueue(&mut self, url: impl Into<String>) -> Enqueue {
            if !self.accepting {
                return Enqueue::Closed;
            }
            let url = url.into();
            if !self.seen.insert(url.clone()) {
                return Enqueue::Duplicate;
            }
            self.pending.push_back(Request { url, attempt: 0 });
            Enqueue::Accepted
        }

        pub fn start_next(&mut self) -> Option<Request> {
            if self.in_flight.len() == self.max_in_flight {
                return None;
            }
            let request = self.pending.pop_front()?;
            self.in_flight.insert(request.url.clone(), request.attempt);
            Some(request)
        }

        pub fn finish(&mut self, url: &str, outcome: Outcome) -> bool {
            let Some(attempt) = self.in_flight.remove(url) else {
                return false;
            };
            if outcome == Outcome::RetryableFailure && attempt < self.max_retries {
                self.pending.push_back(Request {
                    url: url.to_owned(),
                    attempt: attempt + 1,
                });
            } else {
                self.finished.push((url.to_owned(), outcome));
            }
            true
        }

        pub fn close_admission(&mut self) {
            self.accepting = false;
        }

        pub fn in_flight(&self) -> usize {
            self.in_flight.len()
        }

        pub fn pending(&self) -> usize {
            self.pending.len()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn urls_are_deduplicated_before_admission() {
            let mut crawl = CrawlCoordinator::new(2, 1);
            assert_eq!(crawl.enqueue("https://example.test/a"), Enqueue::Accepted);
            assert_eq!(crawl.enqueue("https://example.test/a"), Enqueue::Duplicate);
            crawl.close_admission();
            assert_eq!(crawl.enqueue("https://example.test/b"), Enqueue::Closed);
            assert_eq!(crawl.pending(), 1);
        }

        #[test]
        fn concurrency_limit_and_retry_budget_are_independent() {
            let mut crawl = CrawlCoordinator::new(1, 1);
            crawl.enqueue("a");
            crawl.enqueue("b");
            let first = crawl.start_next().unwrap();
            assert_eq!(crawl.in_flight(), 1);
            assert!(crawl.start_next().is_none());
            assert!(crawl.finish(&first.url, Outcome::RetryableFailure));
            assert_eq!(crawl.start_next().unwrap().url, "b");
            assert_eq!(crawl.pending(), 1);
        }

        #[test]
        fn unknown_completion_cannot_corrupt_accounting() {
            let mut crawl = CrawlCoordinator::new(2, 2);
            crawl.enqueue("a");
            assert!(!crawl.finish("a", Outcome::Success));
            assert_eq!(crawl.pending(), 1);
            assert_eq!(crawl.in_flight(), 0);
        }
    }
}

pub mod p06_catalog_service {
    use std::collections::HashMap;

    // SOLUTION: C57-P06
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CatalogItem {
        pub id: u64,
        pub name: String,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ApplicationError {
        NotFound,
        InvalidName,
        Unavailable,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ItemDto {
        pub id: u64,
        pub display_name: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct HttpResponse<T> {
        pub status: u16,
        pub body: T,
    }

    #[derive(Debug, Default)]
    pub struct Catalog {
        items: HashMap<u64, CatalogItem>,
    }

    impl Catalog {
        pub fn insert(&mut self, item: CatalogItem) -> Result<(), ApplicationError> {
            if item.name.trim().is_empty() {
                return Err(ApplicationError::InvalidName);
            }
            self.items.insert(item.id, item);
            Ok(())
        }

        pub fn get(&self, id: u64) -> Result<CatalogItem, ApplicationError> {
            self.items
                .get(&id)
                .cloned()
                .ok_or(ApplicationError::NotFound)
        }
    }

    pub fn get_item_http(
        catalog: &Catalog,
        id: u64,
    ) -> HttpResponse<Result<ItemDto, &'static str>> {
        match catalog.get(id) {
            Ok(item) => HttpResponse {
                status: 200,
                body: Ok(ItemDto {
                    id: item.id,
                    display_name: item.name,
                }),
            },
            Err(ApplicationError::NotFound) => HttpResponse {
                status: 404,
                body: Err("catalog.item_not_found"),
            },
            Err(ApplicationError::InvalidName) => HttpResponse {
                status: 422,
                body: Err("catalog.invalid_name"),
            },
            Err(ApplicationError::Unavailable) => HttpResponse {
                status: 503,
                body: Err("catalog.unavailable"),
            },
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn transport_dto_is_distinct_from_the_domain_entity() {
            let mut catalog = Catalog::default();
            catalog
                .insert(CatalogItem {
                    id: 1,
                    name: "Ferris".into(),
                })
                .unwrap();
            assert_eq!(
                get_item_http(&catalog, 1),
                HttpResponse {
                    status: 200,
                    body: Ok(ItemDto {
                        id: 1,
                        display_name: "Ferris".into()
                    })
                }
            );
        }

        #[test]
        fn application_error_is_translated_only_at_the_http_boundary() {
            let catalog = Catalog::default();
            assert_eq!(catalog.get(9), Err(ApplicationError::NotFound));
            assert_eq!(
                get_item_http(&catalog, 9),
                HttpResponse {
                    status: 404,
                    body: Err("catalog.item_not_found")
                }
            );
        }

        #[test]
        fn invalid_domain_input_never_reaches_the_repository() {
            let mut catalog = Catalog::default();
            assert_eq!(
                catalog.insert(CatalogItem {
                    id: 1,
                    name: "  ".into()
                }),
                Err(ApplicationError::InvalidName)
            );
            assert_eq!(catalog.get(1), Err(ApplicationError::NotFound));
        }
    }
}

pub mod p07_catalog_desktop {
    use std::collections::HashSet;

    // SOLUTION: C57-P07
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Capability {
        window: String,
        commands: HashSet<String>,
    }

    impl Capability {
        pub fn new(
            window: impl Into<String>,
            commands: impl IntoIterator<Item = impl Into<String>>,
        ) -> Self {
            Self {
                window: window.into(),
                commands: commands.into_iter().map(Into::into).collect(),
            }
        }

        pub fn allows(&self, window: &str, command: &str) -> bool {
            self.window == window && self.commands.contains(command)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ApplicationError {
        NotFound,
        InvalidInput,
        Internal,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct IpcError {
        pub code: &'static str,
        pub retryable: bool,
    }

    pub fn to_ipc_error(error: ApplicationError) -> IpcError {
        match error {
            ApplicationError::NotFound => IpcError {
                code: "catalog.item_not_found",
                retryable: false,
            },
            ApplicationError::InvalidInput => IpcError {
                code: "catalog.invalid_input",
                retryable: false,
            },
            ApplicationError::Internal => IpcError {
                code: "catalog.internal",
                retryable: true,
            },
        }
    }

    #[derive(Debug, Default)]
    pub struct TaskRegistry {
        accepting: bool,
        running: HashSet<u64>,
    }

    impl TaskRegistry {
        pub fn open() -> Self {
            Self {
                accepting: true,
                running: HashSet::new(),
            }
        }

        pub fn admit(&mut self, id: u64) -> bool {
            self.accepting && self.running.insert(id)
        }

        pub fn close(&mut self) {
            self.accepting = false;
        }

        pub fn finish(&mut self, id: u64) -> bool {
            self.running.remove(&id)
        }

        pub fn is_drained(&self) -> bool {
            self.running.is_empty()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn capability_is_scoped_to_window_and_explicit_commands() {
            let capability = Capability::new("main", ["catalog_get", "catalog_list"]);
            assert!(capability.allows("main", "catalog_get"));
            assert!(!capability.allows("admin", "catalog_get"));
            assert!(!capability.allows("main", "shell_exec"));
        }

        #[test]
        fn ipc_errors_are_stable_and_hide_internal_details() {
            assert_eq!(
                to_ipc_error(ApplicationError::NotFound),
                IpcError {
                    code: "catalog.item_not_found",
                    retryable: false
                }
            );
            assert_eq!(
                to_ipc_error(ApplicationError::Internal),
                IpcError {
                    code: "catalog.internal",
                    retryable: true
                }
            );
        }

        #[test]
        fn closing_desktop_state_rejects_new_tasks_but_drains_owned_ones() {
            let mut tasks = TaskRegistry::open();
            assert!(tasks.admit(1));
            tasks.close();
            assert!(!tasks.admit(2));
            assert!(tasks.finish(1));
            assert!(tasks.is_drained());
        }
    }
}

pub mod p08_native_checksum {
    use std::ffi::{CString, NulError};

    // SOLUTION: C57-P08
    unsafe extern "C" fn raw_checksum(bytes: *const u8, len: usize) -> u32 {
        if len == 0 {
            return 0;
        }
        // SAFETY: el wrapper exige un puntero no nulo, alineado y legible para
        // exactamente `len` bytes; la slice no escapa de esta llamada.
        let bytes = unsafe { std::slice::from_raw_parts(bytes, len) };
        bytes
            .iter()
            .fold(0_u32, |sum, byte| sum.wrapping_add(u32::from(*byte)))
    }

    pub fn checksum(bytes: &[u8]) -> u32 {
        // SAFETY: `as_ptr` es válido para `len` bytes durante la llamada. Para
        // una slice vacía la función raw no desreferencia el puntero.
        unsafe { raw_checksum(bytes.as_ptr(), bytes.len()) }
    }

    pub fn native_label(label: &str) -> Result<CString, NulError> {
        CString::new(label)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CallbackPanicked;

    pub fn callback_firewall(
        callback: impl FnOnce() -> u32 + std::panic::UnwindSafe,
    ) -> Result<u32, CallbackPanicked> {
        std::panic::catch_unwind(callback).map_err(|_| CallbackPanicked)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn safe_wrapper_handles_empty_and_non_empty_buffers() {
            assert_eq!(checksum(&[]), 0);
            assert_eq!(checksum(&[1, 2, 255]), 258);
        }

        #[test]
        fn c_string_conversion_rejects_interior_nul() {
            assert!(native_label("catalog").is_ok());
            assert!(native_label("cat\0alog").is_err());
        }

        #[test]
        fn panic_does_not_cross_the_callback_boundary() {
            assert_eq!(callback_firewall(|| 7), Ok(7));
            assert_eq!(
                callback_firewall(|| panic!("foreign callback failed")),
                Err(CallbackPanicked)
            );
        }
    }
}

pub mod p09_offline_first {
    use std::cmp::Ordering;
    use std::collections::{HashSet, VecDeque};

    // SOLUTION: C57-P09
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChangeId(pub u64);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Version {
        pub counter: u64,
        pub replica: u32,
    }

    impl Ord for Version {
        fn cmp(&self, other: &Self) -> Ordering {
            (self.counter, self.replica).cmp(&(other.counter, other.replica))
        }
    }

    impl PartialOrd for Version {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Change {
        pub id: ChangeId,
        pub base: Version,
        pub version: Version,
        pub replacement: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Document {
        pub body: String,
        pub version: Version,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Apply {
        Applied,
        Duplicate,
        Conflict {
            local: Version,
            incoming_base: Version,
        },
    }

    #[derive(Debug)]
    pub struct SyncState {
        document: Document,
        applied: HashSet<ChangeId>,
        pending: VecDeque<Change>,
        queue_capacity: usize,
    }

    impl SyncState {
        pub fn new(document: Document, queue_capacity: usize) -> Self {
            assert!(queue_capacity > 0);
            Self {
                document,
                applied: HashSet::new(),
                pending: VecDeque::new(),
                queue_capacity,
            }
        }

        pub fn document(&self) -> &Document {
            &self.document
        }

        pub fn enqueue(&mut self, change: Change) -> Result<(), Change> {
            if self.pending.len() == self.queue_capacity {
                return Err(change);
            }
            self.pending.push_back(change);
            Ok(())
        }

        pub fn pop_pending(&mut self) -> Option<Change> {
            self.pending.pop_front()
        }

        pub fn apply(&mut self, change: &Change) -> Apply {
            if self.applied.contains(&change.id) {
                return Apply::Duplicate;
            }
            if self.document.version != change.base {
                return Apply::Conflict {
                    local: self.document.version,
                    incoming_base: change.base,
                };
            }
            self.document = Document {
                body: change.replacement.clone(),
                version: change.version,
            };
            self.applied.insert(change.id);
            Apply::Applied
        }

        pub fn resolve_last_writer_wins(&mut self, change: &Change) {
            if change.version > self.document.version {
                self.document = Document {
                    body: change.replacement.clone(),
                    version: change.version,
                };
            }
            self.applied.insert(change.id);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn version(counter: u64, replica: u32) -> Version {
            Version { counter, replica }
        }

        fn change(id: u64, base: Version, version: Version, body: &str) -> Change {
            Change {
                id: ChangeId(id),
                base,
                version,
                replacement: body.into(),
            }
        }

        #[test]
        fn retry_is_idempotent() {
            let initial = version(0, 1);
            let mut state = SyncState::new(
                Document {
                    body: "old".into(),
                    version: initial,
                },
                2,
            );
            let edit = change(1, initial, version(1, 1), "new");
            assert_eq!(state.apply(&edit), Apply::Applied);
            assert_eq!(state.apply(&edit), Apply::Duplicate);
            assert_eq!(state.document().body, "new");
        }

        #[test]
        fn conflict_does_not_mutate_until_the_explicit_policy_resolves_it() {
            let local = version(2, 1);
            let mut state = SyncState::new(
                Document {
                    body: "local".into(),
                    version: local,
                },
                2,
            );
            let remote = change(7, version(1, 2), version(2, 2), "remote");
            assert_eq!(
                state.apply(&remote),
                Apply::Conflict {
                    local,
                    incoming_base: version(1, 2)
                }
            );
            assert_eq!(state.document().body, "local");
            state.resolve_last_writer_wins(&remote);
            assert_eq!(state.document().body, "remote");
        }

        #[test]
        fn bounded_queue_returns_ownership_when_full() {
            let base = version(0, 1);
            let mut state = SyncState::new(
                Document {
                    body: String::new(),
                    version: base,
                },
                1,
            );
            state.enqueue(change(1, base, version(1, 1), "a")).unwrap();
            let rejected = state
                .enqueue(change(2, base, version(1, 2), "b"))
                .unwrap_err();
            assert_eq!(rejected.id, ChangeId(2));
            assert_eq!(state.pop_pending().unwrap().id, ChangeId(1));
        }
    }
}
