//! Capítulos 30 a 32: threads, auto traits y atomics.

pub mod c30 {
    use std::sync::mpsc::{self, Receiver, RecvTimeoutError, SyncSender, TrySendError};
    use std::thread::{self, JoinHandle};
    use std::time::Duration;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SumError {
        NoWorkers,
        Overflow,
        WorkerPanicked,
    }

    // SOLUTION: C30-E01
    /// Sum borrowed chunks, checking both partial sums and their combined total.
    pub fn scoped_sum(values: &[u64], workers: usize) -> Result<u64, SumError> {
        if workers == 0 {
            return Err(SumError::NoWorkers);
        }
        let chunk_size = values.len().max(1).div_ceil(workers);
        thread::scope(|scope| {
            values
                .chunks(chunk_size)
                .map(|chunk| {
                    scope.spawn(move || {
                        chunk
                            .iter()
                            .try_fold(0_u64, |total, &value| total.checked_add(value))
                    })
                })
                .collect::<Vec<_>>()
                .into_iter()
                .try_fold(0_u64, |total, handle| {
                    let partial = handle
                        .join()
                        .map_err(|_| SumError::WorkerPanicked)?
                        .ok_or(SumError::Overflow)?;
                    total.checked_add(partial).ok_or(SumError::Overflow)
                })
        })
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ChannelComparison {
        pub unbounded_queued: usize,
        pub bounded_second_send_was_full: bool,
    }

    // SOLUTION: C30-E03
    /// Delay each consumer explicitly, without depending on sleep or scheduling.
    pub fn compare_channel_capacity() -> ChannelComparison {
        let (unbounded_tx, unbounded_rx) = mpsc::channel();
        let (resume_unbounded, unbounded_start) = mpsc::channel();
        let unbounded_worker = thread::spawn(move || {
            unbounded_start.recv().expect("consumer released");
            unbounded_rx.into_iter().count()
        });
        unbounded_tx.send(1_u8).unwrap();
        unbounded_tx.send(2_u8).unwrap();
        drop(unbounded_tx);
        resume_unbounded.send(()).unwrap();
        let unbounded_queued = unbounded_worker
            .join()
            .expect("unbounded consumer panicked");

        let (bounded_tx, bounded_rx) = mpsc::sync_channel(1);
        let (resume_bounded, bounded_start) = mpsc::channel();
        let bounded_worker = thread::spawn(move || {
            bounded_start.recv().expect("consumer released");
            bounded_rx.recv().expect("first value queued")
        });
        bounded_tx.send(1_u8).unwrap();
        let bounded_second_send_was_full =
            matches!(bounded_tx.try_send(2), Err(TrySendError::Full(2)));
        resume_bounded.send(()).unwrap();
        assert_eq!(bounded_worker.join().expect("bounded consumer panicked"), 1);

        ChannelComparison {
            unbounded_queued,
            bounded_second_send_was_full,
        }
    }

    enum Command {
        Store(String),
        Flush(mpsc::Sender<usize>),
        Shutdown(mpsc::Sender<Vec<String>>),
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum WorkerError {
        Closed,
        NoResponse,
        Panicked,
    }

    // SOLUTION: C30-E04
    pub fn join_worker<T>(handle: JoinHandle<T>) -> Result<T, WorkerError> {
        handle.join().map_err(|_| WorkerError::Panicked)
    }

    /// An in-memory batch worker; call `shutdown` to wait for its completion.
    /// Dropping this value closes its command sender but detaches its thread.
    pub struct Worker {
        sender: SyncSender<Command>,
        handle: Option<JoinHandle<()>>,
    }

    // SOLUTION: C30-E02
    impl Worker {
        pub fn start(capacity: usize) -> Self {
            let (sender, receiver) = mpsc::sync_channel(capacity);
            let handle = thread::spawn(move || {
                let mut pending = Vec::new();
                let mut stored = Vec::new();
                while let Ok(command) = receiver.recv() {
                    match command {
                        Command::Store(value) => pending.push(value),
                        Command::Flush(reply) => {
                            stored.append(&mut pending);
                            let _ = reply.send(stored.len());
                        }
                        Command::Shutdown(reply) => {
                            stored.append(&mut pending);
                            let _ = reply.send(stored);
                            break;
                        }
                    }
                }
            });
            Self {
                sender,
                handle: Some(handle),
            }
        }

        pub fn store(&self, value: impl Into<String>) -> Result<(), WorkerError> {
            self.sender
                .send(Command::Store(value.into()))
                .map_err(|_| WorkerError::Closed)
        }

        /// Commit pending values to the in-memory collection, not to a file.
        pub fn flush(&self) -> Result<usize, WorkerError> {
            let (reply, answer) = mpsc::channel();
            self.sender
                .send(Command::Flush(reply))
                .map_err(|_| WorkerError::Closed)?;
            answer.recv().map_err(|_| WorkerError::NoResponse)
        }

        pub fn shutdown(mut self) -> Result<Vec<String>, WorkerError> {
            let (reply, answer) = mpsc::channel();
            let response = self
                .sender
                .send(Command::Shutdown(reply))
                .map_err(|_| WorkerError::Closed)
                .and_then(|()| answer.recv().map_err(|_| WorkerError::NoResponse));
            let joined = self
                .handle
                .take()
                .expect("worker handle present")
                .join()
                .map_err(|_| WorkerError::Panicked);
            joined?;
            response
        }
    }

    enum CounterCommand {
        Add(u64),
        Reset(mpsc::Sender<u64>),
        Shutdown,
    }

    // SOLUTION: C30-E05
    /// A single-owner metric counter that saturates at `u64::MAX`.
    /// Call `shutdown` to wait for completion; dropping it only detaches the thread.
    pub struct CounterWorker {
        sender: SyncSender<CounterCommand>,
        handle: Option<JoinHandle<()>>,
    }

    // SOLUTION: C30-E06
    impl CounterWorker {
        pub fn start(capacity: usize) -> Self {
            let (sender, receiver) = mpsc::sync_channel(capacity);
            let handle = thread::spawn(move || {
                let mut value = 0_u64;
                while let Ok(command) = receiver.recv() {
                    match command {
                        CounterCommand::Add(amount) => value = value.saturating_add(amount),
                        CounterCommand::Reset(reply) => {
                            let previous = std::mem::replace(&mut value, 0);
                            let _ = reply.send(previous);
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

        pub fn add(&self, amount: u64) -> Result<(), WorkerError> {
            self.sender
                .send(CounterCommand::Add(amount))
                .map_err(|_| WorkerError::Closed)
        }

        /// Enqueue a reset and return its dedicated reply receiver.
        /// Waiting for the reply can time out without cancelling the reset.
        /// Enqueuing itself may block; a reply timeout is not an end-to-end deadline.
        pub fn request_reset(&self) -> Result<Receiver<u64>, WorkerError> {
            let (reply, answer) = mpsc::channel();
            self.sender
                .send(CounterCommand::Reset(reply))
                .map_err(|_| WorkerError::Closed)?;
            Ok(answer)
        }

        pub fn reset(&self) -> Result<u64, WorkerError> {
            self.request_reset()?
                .recv()
                .map_err(|_| WorkerError::NoResponse)
        }

        pub fn shutdown(mut self) -> Result<(), WorkerError> {
            let command_sent = self
                .sender
                .send(CounterCommand::Shutdown)
                .map_err(|_| WorkerError::Closed);
            let joined = self
                .handle
                .take()
                .expect("counter worker handle present")
                .join()
                .map_err(|_| WorkerError::Panicked);
            joined?;
            command_sent
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ReceiveError {
        TimedOut,
        Disconnected,
    }

    // SOLUTION: C30-E07
    /// Limit this receive operation, without cancelling an already sent command.
    pub fn receive_with_timeout<T>(
        receiver: &Receiver<T>,
        timeout: Duration,
    ) -> Result<T, ReceiveError> {
        receiver.recv_timeout(timeout).map_err(|error| match error {
            RecvTimeoutError::Timeout => ReceiveError::TimedOut,
            RecvTimeoutError::Disconnected => ReceiveError::Disconnected,
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn scoped_threads_can_borrow_the_input() {
            assert_eq!(scoped_sum(&[1, 2, 3, 4, 5, 6], 3), Ok(21));
        }

        #[test]
        fn bounded_channel_makes_pressure_observable() {
            assert_eq!(
                compare_channel_capacity(),
                ChannelComparison {
                    unbounded_queued: 2,
                    bounded_second_send_was_full: true,
                }
            );
        }

        #[test]
        fn panic_is_distinct_from_a_channel_error() {
            let handle = thread::spawn(|| -> u64 { panic!("worker failed") });
            assert_eq!(join_worker(handle), Err(WorkerError::Panicked));
        }

        #[test]
        fn flush_and_shutdown_are_ordered_by_the_protocol() {
            let worker = Worker::start(2);
            worker.store("one").unwrap();
            worker.store("two").unwrap();
            assert_eq!(worker.flush(), Ok(2));
            assert_eq!(
                worker.shutdown(),
                Ok(vec![String::from("one"), String::from("two")])
            );
        }

        #[test]
        fn each_reset_response_is_correlated_with_its_request() {
            use std::sync::Arc;

            let worker = Arc::new(CounterWorker::start(4));
            worker.add(7).unwrap();
            let first = Arc::clone(&worker);
            let second = Arc::clone(&worker);
            let first = thread::spawn(move || first.reset());
            let second = thread::spawn(move || second.reset());
            let mut replies = [
                first.join().unwrap().unwrap(),
                second.join().unwrap().unwrap(),
            ];
            replies.sort_unstable();
            assert_eq!(replies, [0, 7]);
            let worker = Arc::into_inner(worker).expect("client handles were joined");
            worker.shutdown().unwrap();
        }

        #[test]
        fn timeout_and_disconnect_are_different_protocol_results() {
            let (sender, receiver) = mpsc::channel::<u8>();
            assert_eq!(
                receive_with_timeout(&receiver, Duration::ZERO),
                Err(ReceiveError::TimedOut)
            );
            drop(sender);
            assert_eq!(
                receive_with_timeout(&receiver, Duration::ZERO),
                Err(ReceiveError::Disconnected)
            );
        }
    }
}

pub mod c31 {
    use std::cell::Cell;
    use std::future::Future;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    // SOLUTION: C31-E01
    pub fn assert_send_type<T: Send>() {}

    pub fn assert_sync_type<T: Sync>() {}

    // SOLUTION: C31-E02
    /// Increment a metric, saturating at the type's maximum value.
    pub fn increment_cell_behind_mutex(state: Arc<Mutex<Cell<u32>>>) -> u32 {
        let worker_state = Arc::clone(&state);
        std::thread::spawn(move || {
            let cell = worker_state.lock().expect("state poisoned");
            cell.set(cell.get().saturating_add(1));
        })
        .join()
        .expect("worker panicked");
        state.lock().expect("state poisoned").get()
    }

    // SOLUTION: C31-E03
    pub fn rc_length_before_await(value: Rc<String>) -> impl Future<Output = usize> + Send {
        let length = value.len();
        drop(value);
        async move {
            tokio::task::yield_now().await;
            length
        }
    }

    pub fn send_length(value: &str) -> impl Future<Output = usize> + Send + use<> {
        let length = value.len();
        async move {
            tokio::task::yield_now().await;
            length
        }
    }

    // SOLUTION: C31-E04
    pub fn scoped_borrowed_length(value: &str) -> usize {
        std::thread::scope(|scope| {
            scope
                .spawn(|| value.len())
                .join()
                .expect("scoped worker panicked")
        })
    }

    pub fn spawned_owned_length(value: String) -> std::thread::JoinHandle<usize> {
        std::thread::spawn(move || value.len())
    }

    /// Checklist for the hypothetical uniquely owned handle in exercise 5.
    /// These asserted facts record a review; booleans cannot prove memory safety.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct SendSafetyAudit {
        pub unique_ownership: bool,
        pub thread_agnostic_resource: bool,
        pub single_drop: bool,
        pub synchronized_shared_access: bool,
    }

    // SOLUTION: C31-E05
    pub const fn justifies_send(audit: SendSafetyAudit) -> bool {
        audit.unique_ownership && audit.thread_agnostic_resource && audit.single_drop
    }

    pub const fn justifies_sync(audit: SendSafetyAudit) -> bool {
        audit.synchronized_shared_access
    }

    // SOLUTION: C31-E06
    /// Release the synchronous guard before yielding. The metric saturates.
    /// A guard kept across an await would prevent the future from being Send:
    ///
    /// ```compile_fail
    /// async fn broken(counter: &std::sync::Mutex<u64>) -> u64 {
    ///     let guard = counter.lock().unwrap();
    ///     std::future::ready(()).await;
    ///     *guard
    /// }
    /// fn require_send<T: Send>(_: T) {}
    /// let counter = std::sync::Mutex::new(0);
    /// require_send(broken(&counter));
    /// ```
    pub async fn increment_then_yield(counter: Arc<Mutex<u64>>) -> u64 {
        let after_increment = {
            let mut guard = counter.lock().expect("counter poisoned");
            *guard = guard.saturating_add(1);
            *guard
        };
        tokio::task::yield_now().await;
        after_increment
    }

    pub async fn increment_with_async_mutex(counter: Arc<tokio::sync::Mutex<u64>>) -> u64 {
        let mut guard = counter.lock().await;
        *guard = guard.saturating_add(1);
        tokio::task::yield_now().await;
        *guard
    }

    // SOLUTION: C31-E07
    /// Witnesses for the two asymmetric combinations of auto traits.
    ///
    /// `Cell<u32>` can be moved but not shared through `&Cell<_>`:
    ///
    /// ```compile_fail
    /// fn assert_sync<T: Sync>() {}
    /// assert_sync::<std::cell::Cell<u32>>();
    /// ```
    ///
    /// `MutexGuard` can be `Sync` when its contents are, but is never `Send`:
    ///
    /// ```compile_fail
    /// fn assert_send<T: Send>() {}
    /// assert_send::<std::sync::MutexGuard<'static, u32>>();
    /// ```
    pub struct AutoTraitWitness;

    #[cfg(test)]
    mod tests {
        use super::*;

        fn assert_send<T: Send>(_: &T) {}

        fn assert_sync<T: Sync>(_: &T) {}

        #[test]
        fn standard_types_are_classified_by_their_actual_contracts() {
            assert_send_type::<String>();
            assert_sync_type::<String>();
            assert_send_type::<Cell<u32>>();

            let mutex = Mutex::new(1_u32);
            let guard = mutex.lock().unwrap();
            assert_sync(&guard);
        }

        #[test]
        fn mutex_requires_send_but_not_sync_from_its_payload() {
            let state = Arc::new(Mutex::new(Cell::new(1)));
            assert_eq!(increment_cell_behind_mutex(state), 2);
        }

        #[tokio::test]
        async fn future_captures_only_owned_send_data() {
            let future = rc_length_before_await(Rc::new(String::from("Rust")));
            assert_send(&future);
            assert_eq!(future.await, 4);

            let text = String::from("ownership");
            let borrowed = send_length(&text);
            drop(text);
            assert_eq!(borrowed.await, 9);
        }

        #[test]
        fn scoped_and_unscoped_threads_have_different_lifetime_contracts() {
            let local = String::from("borrowed");
            assert_eq!(scoped_borrowed_length(&local), 8);
            assert_eq!(spawned_owned_length(local).join().unwrap(), 8);
        }

        #[test]
        fn send_and_sync_require_different_safety_proofs() {
            let send_only = SendSafetyAudit {
                unique_ownership: true,
                thread_agnostic_resource: true,
                single_drop: true,
                synchronized_shared_access: false,
            };
            assert!(justifies_send(send_only));
            assert!(!justifies_sync(send_only));
        }

        #[tokio::test]
        async fn mutex_guard_does_not_cross_await() {
            let counter = Arc::new(Mutex::new(0));
            let handle = tokio::spawn(increment_then_yield(Arc::clone(&counter)));
            assert_eq!(handle.await.unwrap(), 1);
            assert_eq!(*counter.lock().unwrap(), 1);
        }

        #[tokio::test]
        async fn async_mutex_guard_can_cross_await_when_the_api_promises_it() {
            let counter = Arc::new(tokio::sync::Mutex::new(0));
            let handle = tokio::spawn(increment_with_async_mutex(Arc::clone(&counter)));
            assert_eq!(handle.await.unwrap(), 1);
            assert_eq!(*counter.lock().await, 1);
        }
    }
}

pub mod c32 {
    use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};
    use std::sync::{Barrier, Mutex};
    use std::thread;

    #[derive(Default)]
    pub struct Metrics {
        requests: AtomicU64,
    }

    // SOLUTION: C32-E01
    impl Metrics {
        pub fn record_request(&self) {
            self.record_requests(1);
        }

        /// Record a batch without wrapping the metric back to zero.
        pub fn record_requests(&self, count: u64) {
            let _ = self
                .requests
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                    Some(current.saturating_add(count))
                });
        }

        pub fn requests(&self) -> u64 {
            self.requests.load(Ordering::Relaxed)
        }
    }

    #[derive(Default)]
    pub struct PublishedValue {
        data: AtomicU64,
        state: AtomicU8,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AlreadyPublished;

    // SOLUTION: C32-E02
    impl PublishedValue {
        /// Publish exactly once: state 0 is empty, 1 is writing, 2 is ready.
        pub fn publish(&self, value: u64) -> Result<(), AlreadyPublished> {
            self.state
                .compare_exchange(0, 1, Ordering::Relaxed, Ordering::Relaxed)
                .map_err(|_| AlreadyPublished)?;
            self.data.store(value, Ordering::Relaxed);
            self.state.store(2, Ordering::Release);
            Ok(())
        }

        pub fn read(&self) -> Option<u64> {
            (self.state.load(Ordering::Acquire) == 2).then(|| self.data.load(Ordering::Relaxed))
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AlreadyZero;

    // SOLUTION: C32-E03
    pub fn decrement_if_positive(value: &AtomicU64) -> Result<u64, AlreadyZero> {
        let mut current = value.load(Ordering::Relaxed);
        loop {
            if current == 0 {
                return Err(AlreadyZero);
            }
            match value.compare_exchange_weak(
                current,
                current - 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return Ok(current - 1),
                Err(observed) => current = observed,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct StampedIndex {
        pub generation: u32,
        pub index: u32,
    }

    impl StampedIndex {
        const fn pack(self) -> u64 {
            ((self.generation as u64) << 32) | self.index as u64
        }

        const fn unpack(value: u64) -> Self {
            Self {
                generation: (value >> 32) as u32,
                index: value as u32,
            }
        }
    }

    pub struct AtomicStampedIndex(AtomicU64);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ReplaceError {
        Changed(StampedIndex),
        GenerationExhausted,
    }

    // SOLUTION: C32-E04
    impl AtomicStampedIndex {
        pub const fn new(index: u32) -> Self {
            Self(AtomicU64::new(
                StampedIndex {
                    generation: 0,
                    index,
                }
                .pack(),
            ))
        }

        pub fn load(&self) -> StampedIndex {
            StampedIndex::unpack(self.0.load(Ordering::Acquire))
        }

        pub fn replace(
            &self,
            expected: StampedIndex,
            new_index: u32,
        ) -> Result<StampedIndex, ReplaceError> {
            let next = StampedIndex {
                generation: expected
                    .generation
                    .checked_add(1)
                    .ok_or(ReplaceError::GenerationExhausted)?,
                index: new_index,
            };
            self.0
                .compare_exchange(
                    expected.pack(),
                    next.pack(),
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
                .map(|_| next)
                .map_err(|observed| ReplaceError::Changed(StampedIndex::unpack(observed)))
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Quota {
        pub total: u64,
        pub available: u64,
        pub reserved: u64,
    }

    pub struct QuotaBook(Mutex<Quota>);

    // SOLUTION: C32-E05
    impl QuotaBook {
        pub const fn new(total: u64) -> Self {
            Self(Mutex::new(Quota {
                total,
                available: total,
                reserved: 0,
            }))
        }

        pub fn reserve(&self, amount: u64) -> bool {
            let mut quota = self.0.lock().expect("quota poisoned");
            if quota.available < amount {
                return false;
            }
            quota.available -= amount;
            quota.reserved += amount;
            true
        }

        pub fn snapshot(&self) -> Quota {
            *self.0.lock().expect("quota poisoned")
        }
    }

    // SOLUTION: C32-E06
    pub fn demonstrate_lost_update() -> (u64, u64) {
        let broken = AtomicU64::new(0);
        let both_have_loaded = Barrier::new(2);
        thread::scope(|scope| {
            for _ in 0..2 {
                let broken = &broken;
                let both_have_loaded = &both_have_loaded;
                scope.spawn(move || {
                    let current = broken.load(Ordering::Relaxed);
                    both_have_loaded.wait();
                    broken.store(current + 1, Ordering::Relaxed);
                });
            }
        });

        let fixed = AtomicU64::new(0);
        thread::scope(|scope| {
            for _ in 0..2 {
                let fixed = &fixed;
                scope.spawn(move || {
                    fixed.fetch_add(1, Ordering::Relaxed);
                });
            }
        });
        (
            broken.load(Ordering::Relaxed),
            fixed.load(Ordering::Relaxed),
        )
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum FlagProtocol {
        Independent,
        PublishesPayload,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum OrderingPair {
        RelaxedRelaxed,
        ReleaseAcquire,
    }

    // SOLUTION: C32-E07
    pub const fn ordering_for(protocol: FlagProtocol) -> OrderingPair {
        match protocol {
            FlagProtocol::Independent => OrderingPair::RelaxedRelaxed,
            FlagProtocol::PublishesPayload => OrderingPair::ReleaseAcquire,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn relaxed_is_enough_for_an_independent_metric() {
            let metrics = Metrics::default();
            metrics.record_request();
            metrics.record_request();
            assert_eq!(metrics.requests(), 2);
        }

        #[test]
        fn release_acquire_publishes_the_payload() {
            let published = PublishedValue::default();
            assert_eq!(published.read(), None);
            published.publish(42).unwrap();
            assert_eq!(published.read(), Some(42));
        }

        #[test]
        fn compare_exchange_updates_one_complete_atomic_state() {
            let value = AtomicU64::new(2);
            assert_eq!(decrement_if_positive(&value), Ok(1));
            assert_eq!(decrement_if_positive(&value), Ok(0));
            assert_eq!(decrement_if_positive(&value), Err(AlreadyZero));
        }

        #[test]
        fn generation_rejects_an_apparent_aba() {
            let top = AtomicStampedIndex::new(7);
            let stale = top.load();
            let at_b = top.replace(stale, 9).unwrap();
            let back_at_a = top.replace(at_b, 7).unwrap();
            assert_eq!(back_at_a.index, stale.index);
            assert_ne!(back_at_a.generation, stale.generation);
            assert_eq!(
                top.replace(stale, 11),
                Err(ReplaceError::Changed(back_at_a))
            );
        }

        #[test]
        fn exhausted_generations_cannot_reintroduce_an_old_stamp() {
            let last = StampedIndex {
                generation: u32::MAX,
                index: 7,
            };
            let top = AtomicStampedIndex(AtomicU64::new(last.pack()));
            assert_eq!(top.replace(last, 9), Err(ReplaceError::GenerationExhausted));
            assert_eq!(top.load(), last);
        }

        #[test]
        fn one_mutex_protects_the_complete_quota_invariant() {
            let quota = QuotaBook::new(10);
            assert!(quota.reserve(7));
            assert!(!quota.reserve(4));
            assert_eq!(
                quota.snapshot(),
                Quota {
                    total: 10,
                    available: 3,
                    reserved: 7,
                }
            );
        }

        #[test]
        fn load_then_store_is_not_an_atomic_increment() {
            assert_eq!(demonstrate_lost_update(), (1, 2));
        }

        #[test]
        fn a_flag_that_publishes_data_needs_a_visibility_edge() {
            assert_eq!(
                ordering_for(FlagProtocol::Independent),
                OrderingPair::RelaxedRelaxed
            );
            assert_eq!(
                ordering_for(FlagProtocol::PublishesPayload),
                OrderingPair::ReleaseAcquire
            );
        }
    }
}
