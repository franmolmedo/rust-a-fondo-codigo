//! Capítulos 21 a 24: smart pointers, mutación interior, locks y pinning.

pub mod c21 {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::{Rc, Weak};
    use std::sync::Arc;
    use std::thread;

    // SOLUTION: C21-E01
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum List<T> {
        Nil,
        Cons(T, Box<List<T>>),
    }

    impl<T> List<T> {
        pub fn prepend(self, value: T) -> Self {
            Self::Cons(value, Box::new(self))
        }

        pub fn len(&self) -> usize {
            match self {
                Self::Nil => 0,
                Self::Cons(_, tail) => 1 + tail.len(),
            }
        }

        pub fn is_empty(&self) -> bool {
            matches!(self, Self::Nil)
        }
    }

    pub struct Node {
        pub name: String,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    // SOLUTION: C21-E02
    impl Node {
        pub fn new(name: impl Into<String>) -> Rc<Self> {
            Rc::new(Self {
                name: name.into(),
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
            })
        }

        pub fn add_child(parent: &Rc<Self>, child: Rc<Self>) {
            *child.parent.borrow_mut() = Rc::downgrade(parent);
            parent.children.borrow_mut().push(child);
        }

        pub fn parent(&self) -> Option<Rc<Self>> {
            self.parent.borrow().upgrade()
        }

        pub fn child_count(&self) -> usize {
            self.children.borrow().len()
        }
    }

    // SOLUTION: C21-E03
    pub fn reference_count_lifecycle() -> (usize, usize, bool, bool) {
        let value = Rc::new(String::from("shared"));
        let weak = Rc::downgrade(&value);
        let peer = Rc::clone(&value);

        let strong = Rc::strong_count(&value);
        let weak_count = Rc::weak_count(&value);
        let upgrades_while_alive = weak.upgrade().is_some();

        drop(peer);
        drop(value);
        let upgrades_after_last_owner = weak.upgrade().is_some();

        (
            strong,
            weak_count,
            upgrades_while_alive,
            upgrades_after_last_owner,
        )
    }

    // SOLUTION: C21-E04
    pub fn shared_length_from_thread(value: &str) -> usize {
        let shared = Arc::new(value.to_owned());
        let worker_view = Arc::clone(&shared);
        let handle = thread::spawn(move || worker_view.len());
        let measured = handle.join().expect("reader thread panicked");
        assert_eq!(shared.len(), measured);
        measured
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct NodeId(pub u64);

    #[derive(Default)]
    pub struct IdGraph {
        nodes: HashMap<NodeId, String>,
        edges: HashMap<NodeId, Vec<NodeId>>,
    }

    // SOLUTION: C21-E05
    impl IdGraph {
        pub fn insert(&mut self, id: NodeId, label: impl Into<String>) {
            self.nodes.insert(id, label.into());
        }

        pub fn connect(&mut self, from: NodeId, to: NodeId) -> Result<(), &'static str> {
            if !self.nodes.contains_key(&from) || !self.nodes.contains_key(&to) {
                return Err("nodo inexistente");
            }
            self.edges.entry(from).or_default().push(to);
            Ok(())
        }

        pub fn neighbors(&self, id: NodeId) -> impl Iterator<Item = (NodeId, &str)> {
            self.edges
                .get(&id)
                .into_iter()
                .flatten()
                .filter_map(|neighbor| {
                    self.nodes
                        .get(neighbor)
                        .map(|label| (*neighbor, label.as_str()))
                })
        }
    }

    struct CycleNode {
        next: RefCell<Option<Rc<CycleNode>>>,
    }

    // SOLUTION: C21-E07
    pub fn strong_cycle_then_break() -> (usize, usize, bool, bool) {
        let first = Rc::new(CycleNode {
            next: RefCell::new(None),
        });
        let second = Rc::new(CycleNode {
            next: RefCell::new(None),
        });
        *first.next.borrow_mut() = Some(Rc::clone(&second));
        *second.next.borrow_mut() = Some(Rc::clone(&first));

        let counts = (Rc::strong_count(&first), Rc::strong_count(&second));
        let observer = Rc::downgrade(&first);
        drop(first);
        drop(second);

        let survived_external_owners = observer.upgrade().is_some();
        if let Some(first) = observer.upgrade() {
            let second = first.next.borrow_mut().take().expect("cycle link present");
            second.next.borrow_mut().take();
        }
        let released_after_break = observer.upgrade().is_none();

        (
            counts.0,
            counts.1,
            survived_external_owners,
            released_after_break,
        )
    }

    #[derive(Clone)]
    #[allow(clippy::large_enum_variant)] // es el caso base cuya huella se compara con BoxedMessage
    pub enum InlineMessage {
        Small(u8),
        Large([u8; 1024]),
    }

    #[derive(Clone)]
    pub enum BoxedMessage {
        Small(u8),
        Large(Box<[u8; 1024]>),
    }

    // SOLUTION: C21-E06
    pub fn message_sizes() -> (usize, usize) {
        (
            std::mem::size_of::<InlineMessage>(),
            std::mem::size_of::<BoxedMessage>(),
        )
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn box_breaks_recursive_size() {
            let list = List::Nil.prepend(1).prepend(2).prepend(3);
            assert_eq!(list.len(), 3);
        }

        #[test]
        fn weak_parent_does_not_create_an_ownership_cycle() {
            let root = Node::new("root");
            let child = Node::new("child");
            Node::add_child(&root, Rc::clone(&child));
            assert_eq!(root.child_count(), 1);
            assert_eq!(child.parent().unwrap().name, "root");
            assert_eq!(Rc::strong_count(&root), 1);
        }

        #[test]
        fn boxing_the_large_variant_reduces_inline_size() {
            let (inline, boxed) = message_sizes();
            assert!(inline > boxed);
        }

        #[test]
        fn counts_and_weak_upgrade_follow_value_lifetime() {
            assert_eq!(reference_count_lifecycle(), (2, 1, true, false));
        }

        #[test]
        fn arc_moves_an_owner_to_another_thread() {
            assert_eq!(shared_length_from_thread("thread-safe"), 11);
        }

        #[test]
        fn ids_decouple_graph_edges_from_memory_ownership() {
            let mut graph = IdGraph::default();
            graph.insert(NodeId(1), "root");
            graph.insert(NodeId(2), "leaf");
            graph.connect(NodeId(1), NodeId(2)).unwrap();
            assert_eq!(
                graph.neighbors(NodeId(1)).collect::<Vec<_>>(),
                vec![(NodeId(2), "leaf")]
            );
        }

        #[test]
        fn a_strong_cycle_survives_until_one_link_becomes_weak_or_is_removed() {
            assert_eq!(strong_cycle_then_break(), (2, 2, true, true));
        }
    }
}

pub mod c22 {
    use std::cell::{Cell, OnceCell, RefCell};
    use std::collections::HashMap;
    use std::rc::Rc;

    #[derive(Default)]
    pub struct Counter {
        value: Cell<u64>,
    }

    // SOLUTION: C22-E01
    impl Counter {
        pub fn increment(&self) {
            self.value.set(self.value.get() + 1);
        }

        pub fn value(&self) -> u64 {
            self.value.get()
        }
    }

    // SOLUTION: C22-E02
    pub fn conflicting_borrow_panics(values: &RefCell<Vec<u64>>) {
        let reader = values.borrow();
        let _writer = values.borrow_mut();
        drop(reader);
    }

    pub fn read_then_clear(values: &RefCell<Vec<u64>>) -> usize {
        let previous_len = {
            let reader = values.borrow();
            reader.len()
        };
        values.borrow_mut().clear();
        previous_len
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum UpdateError {
        Busy,
    }

    // SOLUTION: C22-E03
    pub fn try_push(values: &RefCell<Vec<u64>>, value: u64) -> Result<(), UpdateError> {
        let mut writer = values.try_borrow_mut().map_err(|_| UpdateError::Busy)?;
        writer.push(value);
        Ok(())
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Event(pub String);

    #[derive(Clone, Default)]
    pub struct EventLog {
        events: Rc<RefCell<Vec<Event>>>,
    }

    // SOLUTION: C22-E04
    impl EventLog {
        pub fn record(&self, event: Event) {
            self.events.borrow_mut().push(event);
        }

        pub fn snapshot(&self) -> Vec<Event> {
            self.events.borrow().clone()
        }
    }

    pub struct Settings {
        raw_port: String,
        parsed_port: OnceCell<u16>,
    }

    // SOLUTION: C22-E05
    impl Settings {
        pub fn new(raw_port: impl Into<String>) -> Self {
            Self {
                raw_port: raw_port.into(),
                parsed_port: OnceCell::new(),
            }
        }

        pub fn port(&self) -> u16 {
            *self
                .parsed_port
                .get_or_init(|| self.raw_port.trim().parse().unwrap_or(8080))
        }
    }

    // SOLUTION: C22-E06
    pub fn get_or_insert(cache: &RefCell<HashMap<String, u64>>, key: &str, default: u64) -> u64 {
        let cached = cache.borrow().get(key).copied();
        match cached {
            Some(value) => value,
            None => {
                cache.borrow_mut().insert(key.to_owned(), default);
                default
            }
        }
    }

    // SOLUTION: C22-E07
    pub fn get_or_insert_2024(
        cache: &RefCell<HashMap<String, u64>>,
        key: &str,
        default: u64,
    ) -> u64 {
        if let Some(value) = cache.borrow().get(key) {
            *value
        } else {
            cache.borrow_mut().insert(key.to_owned(), default);
            default
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn cell_updates_copy_state_through_shared_reference() {
            let counter = Counter::default();
            counter.increment();
            counter.increment();
            assert_eq!(counter.value(), 2);
        }

        #[test]
        fn refcell_guards_do_not_escape_the_api() {
            let log = EventLog::default();
            let other_handle = log.clone();
            log.record(Event(String::from("started")));
            other_handle.record(Event(String::from("finished")));
            assert_eq!(log.snapshot().len(), 2);
        }

        #[test]
        fn once_cell_encodes_compute_once() {
            let settings = Settings::new(" 9090 ");
            assert_eq!(settings.port(), 9090);
            assert_eq!(settings.port(), 9090);
        }

        #[test]
        #[should_panic(expected = "already borrowed")]
        fn overlapping_dynamic_borrows_are_detected() {
            conflicting_borrow_panics(&RefCell::new(vec![1]));
        }

        #[test]
        fn shortening_the_guard_repairs_the_conflict() {
            let values = RefCell::new(vec![1, 2, 3]);
            assert_eq!(read_then_clear(&values), 3);
            assert!(values.borrow().is_empty());
        }

        #[test]
        fn a_recoverable_conflict_is_an_error_value() {
            let values = RefCell::new(vec![1]);
            let reader = values.borrow();
            assert_eq!(try_push(&values, 2), Err(UpdateError::Busy));
            drop(reader);
            assert_eq!(try_push(&values, 2), Ok(()));
        }

        #[test]
        fn reading_before_matching_releases_the_temporary_guard() {
            let cache = RefCell::new(HashMap::new());
            assert_eq!(get_or_insert(&cache, "answer", 42), 42);
            assert_eq!(get_or_insert(&cache, "answer", 99), 42);
        }

        #[test]
        fn edition_2024_if_let_drops_the_scrutinee_before_else() {
            let cache = RefCell::new(HashMap::new());
            assert_eq!(get_or_insert_2024(&cache, "answer", 42), 42);
            assert_eq!(get_or_insert_2024(&cache, "answer", 99), 42);
        }
    }
}

pub mod c23 {
    use std::collections::{HashMap, VecDeque};
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::mpsc::{self, SyncSender};
    use std::sync::{Arc, Condvar, Mutex, RwLock};
    use std::thread::{self, JoinHandle};
    use std::time::{Duration, Instant};

    // SOLUTION: C23-E01
    pub fn parallel_counter(threads: usize, increments: usize) -> u64 {
        let counter = Arc::new(Mutex::new(0_u64));
        let handles: Vec<_> = (0..threads)
            .map(|_| {
                let counter = Arc::clone(&counter);
                thread::spawn(move || {
                    for _ in 0..increments {
                        *counter.lock().expect("counter poisoned") += 1;
                    }
                })
            })
            .collect();
        for handle in handles {
            handle.join().expect("counter thread panicked");
        }
        *counter.lock().expect("counter poisoned")
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum TransferError {
        MissingAccount(u64),
        InsufficientFunds,
        SameAccount,
    }

    #[derive(Default)]
    pub struct Ledger {
        balances: Mutex<HashMap<u64, u64>>,
    }

    impl Ledger {
        pub fn insert(&self, id: u64, balance: u64) {
            self.balances
                .lock()
                .expect("ledger poisoned")
                .insert(id, balance);
        }

        // SOLUTION: C23-E02
        pub fn transfer(&self, from: u64, to: u64, amount: u64) -> Result<(), TransferError> {
            if from == to {
                return Err(TransferError::SameAccount);
            }
            let mut balances = self.balances.lock().expect("ledger poisoned");
            let source = *balances
                .get(&from)
                .ok_or(TransferError::MissingAccount(from))?;
            let destination = *balances.get(&to).ok_or(TransferError::MissingAccount(to))?;
            let source_after = source
                .checked_sub(amount)
                .ok_or(TransferError::InsufficientFunds)?;
            let destination_after = destination
                .checked_add(amount)
                .expect("balance overflow in exercise");
            balances.insert(from, source_after);
            balances.insert(to, destination_after);
            Ok(())
        }

        pub fn balance(&self, id: u64) -> Option<u64> {
            self.balances
                .lock()
                .expect("ledger poisoned")
                .get(&id)
                .copied()
        }
    }

    // SOLUTION: C23-E03
    pub fn poison_and_recover() -> u64 {
        let state = Mutex::new(10_u64);
        let caught = catch_unwind(AssertUnwindSafe(|| {
            let mut value = state.lock().expect("initial lock succeeds");
            *value = 99;
            panic!("simulated interrupted transition");
        }));
        assert!(caught.is_err());

        let mut value = state
            .lock()
            .expect_err("the mutex must be poisoned")
            .into_inner();
        *value = 0;
        state.clear_poison();
        *value
    }

    #[derive(Clone, Copy, Debug)]
    pub struct LockWorkload {
        pub readers: usize,
        pub writers: usize,
        pub iterations: usize,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct LockComparison {
        pub mutex_elapsed: Duration,
        pub rwlock_elapsed: Duration,
        pub mutex_value: usize,
        pub rwlock_value: usize,
    }

    fn run_mutex_workload(workload: LockWorkload) -> (Duration, usize) {
        let value = Arc::new(Mutex::new(0_usize));
        let started = Instant::now();
        let mut handles = Vec::new();

        for _ in 0..workload.writers {
            let value = Arc::clone(&value);
            handles.push(thread::spawn(move || {
                for _ in 0..workload.iterations {
                    *value.lock().expect("benchmark mutex poisoned") += 1;
                }
            }));
        }
        for _ in 0..workload.readers {
            let value = Arc::clone(&value);
            handles.push(thread::spawn(move || {
                for _ in 0..workload.iterations {
                    std::hint::black_box(*value.lock().expect("benchmark mutex poisoned"));
                }
            }));
        }
        for handle in handles {
            handle.join().expect("benchmark participant panicked");
        }
        let elapsed = started.elapsed();
        let final_value = *value.lock().expect("benchmark mutex poisoned");
        (elapsed, final_value)
    }

    fn run_rwlock_workload(workload: LockWorkload) -> (Duration, usize) {
        let value = Arc::new(RwLock::new(0_usize));
        let started = Instant::now();
        let mut handles = Vec::new();

        for _ in 0..workload.writers {
            let value = Arc::clone(&value);
            handles.push(thread::spawn(move || {
                for _ in 0..workload.iterations {
                    *value.write().expect("benchmark rwlock poisoned") += 1;
                }
            }));
        }
        for _ in 0..workload.readers {
            let value = Arc::clone(&value);
            handles.push(thread::spawn(move || {
                for _ in 0..workload.iterations {
                    std::hint::black_box(*value.read().expect("benchmark rwlock poisoned"));
                }
            }));
        }
        for handle in handles {
            handle.join().expect("benchmark participant panicked");
        }
        let elapsed = started.elapsed();
        let final_value = *value.read().expect("benchmark rwlock poisoned");
        (elapsed, final_value)
    }

    // SOLUTION: C23-E04
    pub fn compare_locks(workload: LockWorkload) -> LockComparison {
        let (mutex_elapsed, mutex_value) = run_mutex_workload(workload);
        let (rwlock_elapsed, rwlock_value) = run_rwlock_workload(workload);
        LockComparison {
            mutex_elapsed,
            rwlock_elapsed,
            mutex_value,
            rwlock_value,
        }
    }

    pub struct Account {
        id: u64,
        balance: Mutex<u64>,
    }

    impl Account {
        pub fn new(id: u64, balance: u64) -> Self {
            Self {
                id,
                balance: Mutex::new(balance),
            }
        }

        pub fn balance(&self) -> u64 {
            *self.balance.lock().expect("account poisoned")
        }
    }

    // SOLUTION: C23-E06
    pub fn transfer_ordered(
        from: &Account,
        to: &Account,
        amount: u64,
    ) -> Result<(), TransferError> {
        if from.id == to.id {
            return Err(TransferError::SameAccount);
        }

        let (first, second) = if from.id < to.id {
            (from, to)
        } else {
            (to, from)
        };
        let mut first_guard = first.balance.lock().expect("account poisoned");
        let mut second_guard = second.balance.lock().expect("account poisoned");
        let (source, destination) = if from.id < to.id {
            (&mut *first_guard, &mut *second_guard)
        } else {
            (&mut *second_guard, &mut *first_guard)
        };

        let source_after = source
            .checked_sub(amount)
            .ok_or(TransferError::InsufficientFunds)?;
        let destination_after = destination
            .checked_add(amount)
            .expect("balance overflow in exercise");
        *source = source_after;
        *destination = destination_after;
        Ok(())
    }

    enum CounterCommand {
        Add(u64),
        Snapshot(mpsc::Sender<u64>),
        Shutdown,
    }

    pub struct OwnedCounter {
        sender: SyncSender<CounterCommand>,
        handle: Option<JoinHandle<()>>,
    }

    // SOLUTION: C23-E05
    impl OwnedCounter {
        pub fn start(capacity: usize) -> Self {
            let (sender, receiver) = mpsc::sync_channel(capacity);
            let handle = thread::spawn(move || {
                let mut value = 0;
                while let Ok(command) = receiver.recv() {
                    match command {
                        CounterCommand::Add(amount) => value += amount,
                        CounterCommand::Snapshot(reply) => {
                            let _ = reply.send(value);
                        }
                        CounterCommand::Shutdown => break,
                    }
                }
            });
            Self {
                sender,
                handle: Some(handle),
            }
        }

        pub fn add(&self, amount: u64) -> Result<(), &'static str> {
            self.sender
                .send(CounterCommand::Add(amount))
                .map_err(|_| "worker detenido")
        }

        pub fn snapshot(&self) -> Result<u64, &'static str> {
            let (reply, answer) = mpsc::channel();
            self.sender
                .send(CounterCommand::Snapshot(reply))
                .map_err(|_| "worker detenido")?;
            answer.recv().map_err(|_| "worker sin respuesta")
        }

        pub fn shutdown(mut self) -> thread::Result<()> {
            let _ = self.sender.send(CounterCommand::Shutdown);
            self.handle.take().expect("handle present").join()
        }
    }

    pub struct BlockingQueue<T> {
        values: Mutex<VecDeque<T>>,
        available: Condvar,
    }

    // SOLUTION: C23-E07
    impl<T> BlockingQueue<T> {
        pub fn new() -> Self {
            Self {
                values: Mutex::new(VecDeque::new()),
                available: Condvar::new(),
            }
        }

        pub fn push(&self, value: T) {
            self.values.lock().expect("queue poisoned").push_back(value);
            self.available.notify_one();
        }

        pub fn pop_blocking(&self) -> T {
            let mut values = self.values.lock().expect("queue poisoned");
            loop {
                if let Some(value) = values.pop_front() {
                    return value;
                }
                values = self.available.wait(values).expect("queue poisoned");
            }
        }
    }

    impl<T> Default for BlockingQueue<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn mutex_counter_waits_for_every_thread() {
            assert_eq!(parallel_counter(4, 1_000), 4_000);
        }

        #[test]
        fn ledger_transition_is_all_or_nothing() {
            let ledger = Ledger::default();
            ledger.insert(1, 100);
            ledger.insert(2, 50);
            ledger.transfer(1, 2, 40).unwrap();
            assert_eq!(ledger.balance(1), Some(60));
            assert_eq!(ledger.balance(2), Some(90));
            assert_eq!(
                ledger.transfer(1, 2, 100),
                Err(TransferError::InsufficientFunds)
            );
            assert_eq!(ledger.balance(1), Some(60));
            assert_eq!(ledger.balance(2), Some(90));
        }

        #[test]
        fn state_can_have_one_owner_and_a_command_protocol() {
            let worker = OwnedCounter::start(4);
            worker.add(2).unwrap();
            worker.add(5).unwrap();
            assert_eq!(worker.snapshot(), Ok(7));
            worker.shutdown().unwrap();
        }

        #[test]
        fn condvar_queue_wakes_without_timing_assumptions() {
            let queue = Arc::new(BlockingQueue::new());
            let waiting = Arc::clone(&queue);
            let handle = thread::spawn(move || waiting.pop_blocking());
            queue.push("ready");
            assert_eq!(handle.join().unwrap(), "ready");
        }

        #[test]
        fn poisoning_is_a_policy_decision_not_silent_success() {
            assert_eq!(poison_and_recover(), 0);
        }

        #[test]
        fn a_lock_benchmark_checks_equal_work_without_assuming_a_winner() {
            let workload = LockWorkload {
                readers: 2,
                writers: 1,
                iterations: 200,
            };
            let comparison = compare_locks(workload);
            assert_eq!(comparison.mutex_value, 200);
            assert_eq!(comparison.rwlock_value, 200);
            std::hint::black_box(comparison.mutex_elapsed);
            std::hint::black_box(comparison.rwlock_elapsed);
        }

        #[test]
        fn crossed_transfers_follow_one_global_lock_order() {
            let first = Arc::new(Account::new(1, 100));
            let second = Arc::new(Account::new(2, 100));

            let a = Arc::clone(&first);
            let b = Arc::clone(&second);
            let left_to_right = thread::spawn(move || transfer_ordered(&a, &b, 10));

            let a = Arc::clone(&first);
            let b = Arc::clone(&second);
            let right_to_left = thread::spawn(move || transfer_ordered(&b, &a, 20));

            left_to_right.join().unwrap().unwrap();
            right_to_left.join().unwrap().unwrap();
            assert_eq!(first.balance(), 110);
            assert_eq!(second.balance(), 90);
        }
    }
}

pub mod c24 {
    use std::future::Future;
    use std::marker::PhantomPinned;
    use std::pin::Pin;
    use std::task::{Context, Poll, Waker};

    // SOLUTION: C24-E01
    pub fn string_buffer_survives_move(value: &str) -> bool {
        let original = value.to_owned();
        let buffer_before = original.as_ptr();
        let moved = original;
        let buffer_after = moved.as_ptr();
        buffer_before == buffer_after
    }

    // SOLUTION: C24-E02
    pub fn poll_a_pinned_future() -> Poll<u32> {
        let future = async { 42_u32 };
        let mut pinned = std::pin::pin!(future);
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        pinned.as_mut().poll(&mut context)
    }

    pub struct AddressSensitive {
        label: String,
        _pin: PhantomPinned,
    }

    // SOLUTION: C24-E03
    impl AddressSensitive {
        pub fn new(label: impl Into<String>) -> Pin<Box<Self>> {
            Box::pin(Self {
                label: label.into(),
                _pin: PhantomPinned,
            })
        }

        pub fn label(self: Pin<&Self>) -> &str {
            &self.get_ref().label
        }
    }

    pub struct Counted<F> {
        future: F,
        polls: u32,
    }

    impl<F> Counted<F> {
        pub fn new(future: F) -> Self {
            Self { future, polls: 0 }
        }

        pub fn polls(&self) -> u32 {
            self.polls
        }

        // SOLUTION: C24-E04
        pub fn project(self: Pin<&mut Self>) -> (Pin<&mut F>, &mut u32) {
            // SAFETY: `future` se clasifica como campo estructuralmente
            // pinneado y `polls` como campo movible independiente.
            unsafe { Self::project_unchecked(self) }
        }

        // SOLUTION: C24-E05
        /// Proyecta los dos campos sin moverlos fuera de `self`.
        ///
        /// # Safety
        ///
        /// El caller debe mantener `future` estructuralmente pinneado durante
        /// toda la vida de la proyección: no puede reemplazarlo, extraerlo ni
        /// implementar un `Drop` que lo mueva. `polls` no participa en ninguna
        /// invariante de dirección y puede exponerse como `&mut u32`.
        unsafe fn project_unchecked(self: Pin<&mut Self>) -> (Pin<&mut F>, &mut u32) {
            // SAFETY: la precondición prohíbe mover `future` mediante este &mut.
            let this = unsafe { self.get_unchecked_mut() };
            // SAFETY: `future` hereda el pinning estructural de `self`.
            let future = unsafe { Pin::new_unchecked(&mut this.future) };
            (future, &mut this.polls)
        }
    }

    // SOLUTION: C24-E06
    impl<F> Future for Counted<F>
    where
        F: Future,
    {
        type Output = F::Output;

        fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            let (future, polls) = self.project();
            *polls += 1;
            future.poll(context)
        }
    }

    // SOLUTION: C24-E07
    /// Intercambia valores pinneados únicamente cuando mover `T` es seguro.
    ///
    /// ```compile_fail
    /// use course_solutions::memory::c24::{AddressSensitive, swap_unpin};
    /// let mut left = AddressSensitive::new("left");
    /// let mut right = AddressSensitive::new("right");
    /// swap_unpin(left.as_mut(), right.as_mut()); // PhantomPinned: !Unpin
    /// ```
    pub fn swap_unpin<T: Unpin>(mut left: Pin<&mut T>, mut right: Pin<&mut T>) {
        std::mem::swap(left.as_mut().get_mut(), right.as_mut().get_mut());
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        struct PendingOnce(bool);

        impl Future for PendingOnce {
            type Output = &'static str;

            fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
                if self.0 {
                    Poll::Ready("done")
                } else {
                    self.0 = true;
                    context.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        }

        #[test]
        fn wrapper_counts_each_poll() {
            let mut counted = Counted::new(PendingOnce(false));
            let waker = Waker::noop();
            let mut context = Context::from_waker(waker);
            assert!(Pin::new(&mut counted).poll(&mut context).is_pending());
            assert_eq!(
                Pin::new(&mut counted).poll(&mut context),
                Poll::Ready("done")
            );
            assert_eq!(counted.polls(), 2);
        }

        #[test]
        fn moving_string_keeps_its_heap_buffer() {
            assert!(string_buffer_survives_move("rust"));
        }

        #[test]
        fn a_stack_future_is_pinned_before_poll() {
            assert_eq!(poll_a_pinned_future(), Poll::Ready(42));
        }

        #[test]
        fn phantom_pinned_marks_the_type_without_creating_self_references() {
            let value = AddressSensitive::new("stable");
            assert_eq!(value.as_ref().label(), "stable");
        }

        #[test]
        fn projection_supports_a_non_unpin_async_future() {
            let mut counted = Box::pin(Counted::new(async { 7_u32 }));
            let waker = Waker::noop();
            let mut context = Context::from_waker(waker);
            assert_eq!(counted.as_mut().poll(&mut context), Poll::Ready(7));
            assert_eq!(counted.as_ref().get_ref().polls(), 1);
        }

        #[test]
        fn unpin_is_the_gate_that_allows_swapping() {
            let mut left = 1_u32;
            let mut right = 2_u32;
            swap_unpin(Pin::new(&mut left), Pin::new(&mut right));
            assert_eq!((left, right), (2, 1));
        }
    }
}
