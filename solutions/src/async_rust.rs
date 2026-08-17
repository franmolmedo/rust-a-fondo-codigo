//! Capítulos 33 a 43: futures, cancelación, channels y fronteras de runtime.

pub mod c33 {
    use std::future::Future;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum PollEvent {
        Enqueued,
        PollStarted,
        InterestRegistered,
        Pending,
        EventReady,
        Woken,
        Requeued,
        Ready,
    }

    // SOLUTION: C33-E01
    pub const fn reference_poll_cycle() -> [PollEvent; 9] {
        [
            PollEvent::Enqueued,
            PollEvent::PollStarted,
            PollEvent::InterestRegistered,
            PollEvent::Pending,
            PollEvent::EventReady,
            PollEvent::Woken,
            PollEvent::Requeued,
            PollEvent::PollStarted,
            PollEvent::Ready,
        ]
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AsyncLine<'a> {
        pub label: &'a str,
        pub awaits: bool,
    }

    // SOLUTION: C33-E02
    pub fn potential_suspension_points<'a>(lines: &'a [AsyncLine<'a>]) -> Vec<&'a str> {
        lines
            .iter()
            .filter(|line| line.awaits)
            .map(|line| line.label)
            .collect()
    }

    // SOLUTION: C33-E03
    pub async fn join_independent<Left, Right>(
        left: Left,
        right: Right,
    ) -> (Left::Output, Right::Output)
    where
        Left: Future,
        Right: Future,
    {
        tokio::join!(left, right)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Operation {
        SocketRead,
        AsyncTimer,
        BlockingFileRead,
        PasswordHash,
        PersistentBlockingWorker,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum WorkClass {
        AsyncIo,
        Blocking,
        CpuBound,
    }

    // SOLUTION: C33-E04
    pub const fn classify_operation(operation: Operation) -> WorkClass {
        match operation {
            Operation::SocketRead | Operation::AsyncTimer => WorkClass::AsyncIo,
            Operation::BlockingFileRead | Operation::PersistentBlockingWorker => {
                WorkClass::Blocking
            }
            Operation::PasswordHash => WorkClass::CpuBound,
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CompletionPolicy {
        Join,
        CancelThenJoin,
        TransferTo(&'static str),
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct SubtaskPlan {
        pub name: &'static str,
        pub owner: &'static str,
        pub completion: CompletionPolicy,
    }

    // SOLUTION: C33-E05
    pub fn lifecycle_is_explicit(subtasks: &[SubtaskPlan]) -> bool {
        !subtasks.is_empty()
            && subtasks.iter().all(|subtask| {
                !subtask.name.is_empty()
                    && !subtask.owner.is_empty()
                    && match subtask.completion {
                        CompletionPolicy::TransferTo(new_owner) => !new_owner.is_empty(),
                        CompletionPolicy::Join | CompletionPolicy::CancelThenJoin => true,
                    }
            })
    }

    // SOLUTION: C33-E06
    pub async fn lazy_effect(counter: Arc<AtomicUsize>) -> usize {
        counter.fetch_add(1, Ordering::SeqCst) + 1
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SupervisionError {
        Cancelled,
        Panicked,
    }

    // SOLUTION: C33-E07
    pub async fn supervise<T>(handle: tokio::task::JoinHandle<T>) -> Result<T, SupervisionError> {
        handle.await.map_err(|error| {
            if error.is_cancelled() {
                SupervisionError::Cancelled
            } else {
                SupervisionError::Panicked
            }
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use tokio::sync::Barrier;

        #[test]
        fn poll_cycle_registers_before_pending_and_repolls_after_wake() {
            let trace = reference_poll_cycle();
            assert_eq!(trace[2], PollEvent::InterestRegistered);
            assert_eq!(trace[3], PollEvent::Pending);
            assert_eq!(trace[5], PollEvent::Woken);
            assert_eq!(trace[7], PollEvent::PollStarted);
            assert_eq!(trace[8], PollEvent::Ready);
        }

        #[test]
        fn only_await_lines_are_potential_suspension_points() {
            let lines = [
                AsyncLine {
                    label: "validate",
                    awaits: false,
                },
                AsyncLine {
                    label: "load_user.await",
                    awaits: true,
                },
                AsyncLine {
                    label: "build_response",
                    awaits: false,
                },
            ];
            assert_eq!(potential_suspension_points(&lines), ["load_user.await"]);
        }

        #[tokio::test]
        async fn join_polls_both_futures_without_spawning() {
            let gate = Arc::new(Barrier::new(2));
            let left_gate = Arc::clone(&gate);
            let right_gate = Arc::clone(&gate);
            let left = async move {
                left_gate.wait().await;
                20
            };
            let right = async move {
                right_gate.wait().await;
                22
            };
            assert_eq!(join_independent(left, right).await, (20, 22));
        }

        #[test]
        fn five_operations_are_classified_by_what_makes_them_wait_or_work() {
            assert_eq!(
                classify_operation(Operation::SocketRead),
                WorkClass::AsyncIo
            );
            assert_eq!(
                classify_operation(Operation::AsyncTimer),
                WorkClass::AsyncIo
            );
            assert_eq!(
                classify_operation(Operation::BlockingFileRead),
                WorkClass::Blocking
            );
            assert_eq!(
                classify_operation(Operation::PasswordHash),
                WorkClass::CpuBound
            );
            assert_eq!(
                classify_operation(Operation::PersistentBlockingWorker),
                WorkClass::Blocking
            );
        }

        #[test]
        fn every_subtask_has_an_owner_and_a_terminal_policy() {
            let plan = [
                SubtaskPlan {
                    name: "user",
                    owner: "request",
                    completion: CompletionPolicy::Join,
                },
                SubtaskPlan {
                    name: "permissions",
                    owner: "request",
                    completion: CompletionPolicy::CancelThenJoin,
                },
                SubtaskPlan {
                    name: "audit",
                    owner: "request",
                    completion: CompletionPolicy::TransferTo("audit-service"),
                },
            ];
            assert!(lifecycle_is_explicit(&plan));
            assert!(!lifecycle_is_explicit(&[SubtaskPlan {
                completion: CompletionPolicy::TransferTo(""),
                ..plan[0]
            }]));
        }

        #[tokio::test]
        async fn constructing_a_future_does_not_run_its_body() {
            let counter = Arc::new(AtomicUsize::new(0));
            let future = lazy_effect(Arc::clone(&counter));
            assert_eq!(counter.load(Ordering::SeqCst), 0);
            assert_eq!(future.await, 1);
            assert_eq!(counter.load(Ordering::SeqCst), 1);
        }

        #[tokio::test]
        async fn supervised_task_reports_success_and_panic() {
            assert_eq!(supervise(tokio::spawn(async { 42 })).await, Ok(42));
            let panicked = tokio::spawn(async { panic!("task failed") });
            assert_eq!(supervise(panicked).await, Err(SupervisionError::Panicked));
        }
    }
}

pub mod c34 {
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, Waker};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum TimerPollEvent {
        PollStarted,
        WakerRegistered,
        Pending,
        SpuriousWake,
        TimerExpired,
        WakeRequested,
        Ready,
    }

    // SOLUTION: C34-E01
    pub const fn timer_poll_trace() -> [TimerPollEvent; 11] {
        [
            TimerPollEvent::PollStarted,
            TimerPollEvent::WakerRegistered,
            TimerPollEvent::Pending,
            TimerPollEvent::SpuriousWake,
            TimerPollEvent::PollStarted,
            TimerPollEvent::WakerRegistered,
            TimerPollEvent::Pending,
            TimerPollEvent::TimerExpired,
            TimerPollEvent::WakeRequested,
            TimerPollEvent::PollStarted,
            TimerPollEvent::Ready,
        ]
    }

    pub struct Immediate<T>(Option<T>);

    // SOLUTION: C34-E02
    impl<T> Immediate<T> {
        pub fn new(value: T) -> Self {
            Self(Some(value))
        }
    }

    impl<T: Unpin> Future for Immediate<T> {
        type Output = T;

        fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(
                self.get_mut()
                    .0
                    .take()
                    .expect("Immediate no puede sondearse después de Ready"),
            )
        }
    }

    pub struct Map<Inner, Mapper> {
        inner: Option<Pin<Box<Inner>>>,
        mapper: Option<Mapper>,
    }

    impl<Inner, Mapper> Map<Inner, Mapper> {
        pub fn new(inner: Inner, mapper: Mapper) -> Self {
            Self {
                inner: Some(Box::pin(inner)),
                mapper: Some(mapper),
            }
        }
    }

    // El future hijo conserva su pinning tras la indirección; mover `Map` no lo mueve.
    impl<Inner, Mapper> Unpin for Map<Inner, Mapper> {}

    // SOLUTION: C34-E03
    impl<Inner, Mapper, Output> Future for Map<Inner, Mapper>
    where
        Inner: Future,
        Mapper: FnOnce(Inner::Output) -> Output,
    {
        type Output = Output;

        fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.get_mut();
            let inner = this
                .inner
                .as_mut()
                .expect("Map no puede sondearse después de Ready");

            match inner.as_mut().poll(context) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(value) => {
                    this.inner = None;
                    let mapper = this
                        .mapper
                        .take()
                        .expect("Map debe conservar su mapper hasta Ready");
                    Poll::Ready(mapper(value))
                }
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum LostWakeStep {
        CheckedNotReady,
        PublishedReady,
        FoundNoWaker,
        RegisteredTooLate,
        ReturnedPending,
    }

    // SOLUTION: C34-E04
    pub const fn lost_wakeup_trace() -> [LostWakeStep; 5] {
        [
            LostWakeStep::CheckedNotReady,
            LostWakeStep::PublishedReady,
            LostWakeStep::FoundNoWaker,
            LostWakeStep::RegisteredTooLate,
            LostWakeStep::ReturnedPending,
        ]
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum PollWork {
        ReadAtomicFlag,
        BoundedStateTransition,
        LargeIncrementalScan,
        BlockingFileRead,
        PasswordHash,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum WorkPlacement {
        InsidePoll,
        ChunkAndYield,
        Offload,
    }

    // SOLUTION: C34-E05
    pub const fn classify_poll_work(work: PollWork) -> WorkPlacement {
        match work {
            PollWork::ReadAtomicFlag | PollWork::BoundedStateTransition => {
                WorkPlacement::InsidePoll
            }
            PollWork::LargeIncrementalScan => WorkPlacement::ChunkAndYield,
            PollWork::BlockingFileRead | PollWork::PasswordHash => WorkPlacement::Offload,
        }
    }

    #[derive(Clone)]
    pub struct OneShotEvent {
        shared: Arc<Mutex<EventState>>,
    }

    struct EventState {
        ready: bool,
        next_waiter_id: u64,
        waiters: Vec<(u64, Waker)>,
    }

    pub struct EventFuture {
        shared: Arc<Mutex<EventState>>,
        waiter_id: u64,
        completed: bool,
    }

    impl Default for OneShotEvent {
        fn default() -> Self {
            Self::new()
        }
    }

    impl OneShotEvent {
        pub fn new() -> Self {
            Self {
                shared: Arc::new(Mutex::new(EventState {
                    ready: false,
                    next_waiter_id: 0,
                    waiters: Vec::new(),
                })),
            }
        }

        pub fn wait(&self) -> EventFuture {
            let waiter_id = {
                let mut state = self.shared.lock().expect("event mutex poisoned");
                let id = state.next_waiter_id;
                state.next_waiter_id = state
                    .next_waiter_id
                    .checked_add(1)
                    .expect("too many event waiters");
                id
            };

            EventFuture {
                shared: Arc::clone(&self.shared),
                waiter_id,
                completed: false,
            }
        }

        pub fn signal(&self) -> bool {
            let wakers = {
                let mut state = self.shared.lock().expect("event mutex poisoned");
                if state.ready {
                    return false;
                }
                state.ready = true;
                state
                    .waiters
                    .drain(..)
                    .map(|(_, waker)| waker)
                    .collect::<Vec<_>>()
            };

            for waker in wakers {
                waker.wake();
            }
            true
        }

        pub fn waiting_count(&self) -> usize {
            self.shared
                .lock()
                .expect("event mutex poisoned")
                .waiters
                .len()
        }
    }

    // SOLUTION: C34-E06
    impl Future for EventFuture {
        type Output = ();

        fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.get_mut();
            let mut state = this.shared.lock().expect("event mutex poisoned");

            if state.ready {
                state.waiters.retain(|(id, _)| *id != this.waiter_id);
                this.completed = true;
                return Poll::Ready(());
            }

            if let Some((_, registered)) = state
                .waiters
                .iter_mut()
                .find(|(id, _)| *id == this.waiter_id)
            {
                registered.clone_from(context.waker());
            } else {
                state
                    .waiters
                    .push((this.waiter_id, context.waker().clone()));
            }
            Poll::Pending
        }
    }

    // SOLUTION: C34-E07
    impl Drop for EventFuture {
        fn drop(&mut self) {
            if self.completed {
                return;
            }
            self.shared
                .lock()
                .expect("event mutex poisoned")
                .waiters
                .retain(|(id, _)| *id != self.waiter_id);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::task::{Wake, Waker};

        #[derive(Default)]
        struct WakeProbe {
            wakes: AtomicUsize,
        }

        impl Wake for WakeProbe {
            fn wake(self: Arc<Self>) {
                self.wakes.fetch_add(1, Ordering::SeqCst);
            }
        }

        fn probe() -> (Arc<WakeProbe>, Waker) {
            let probe = Arc::new(WakeProbe::default());
            let waker = Waker::from(Arc::clone(&probe));
            (probe, waker)
        }

        #[test]
        fn timer_trace_rechecks_after_a_spurious_wake() {
            let trace = timer_poll_trace();
            assert_eq!(trace[3], TimerPollEvent::SpuriousWake);
            assert_eq!(trace[4], TimerPollEvent::PollStarted);
            assert_eq!(trace[6], TimerPollEvent::Pending);
            assert_eq!(trace[9], TimerPollEvent::PollStarted);
            assert_eq!(trace[10], TimerPollEvent::Ready);
        }

        #[test]
        fn immediate_completes_once_and_then_panics() {
            let mut future = Box::pin(Immediate::new(42));
            let mut context = Context::from_waker(Waker::noop());
            assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(42));

            let second_poll = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                future.as_mut().poll(&mut context)
            }));
            assert!(second_poll.is_err());
        }

        #[tokio::test]
        async fn map_pins_the_child_and_consumes_the_mapper_once() {
            assert_eq!(Map::new(async { 21 }, |value| value * 2).await, 42);
        }

        #[test]
        fn the_lost_wakeup_happens_before_registration() {
            assert_eq!(
                lost_wakeup_trace(),
                [
                    LostWakeStep::CheckedNotReady,
                    LostWakeStep::PublishedReady,
                    LostWakeStep::FoundNoWaker,
                    LostWakeStep::RegisteredTooLate,
                    LostWakeStep::ReturnedPending,
                ]
            );
        }

        #[test]
        fn poll_work_is_bounded_chunked_or_offloaded() {
            assert_eq!(
                classify_poll_work(PollWork::ReadAtomicFlag),
                WorkPlacement::InsidePoll
            );
            assert_eq!(
                classify_poll_work(PollWork::LargeIncrementalScan),
                WorkPlacement::ChunkAndYield
            );
            assert_eq!(
                classify_poll_work(PollWork::BlockingFileRead),
                WorkPlacement::Offload
            );
            assert_eq!(
                classify_poll_work(PollWork::PasswordHash),
                WorkPlacement::Offload
            );
        }

        #[test]
        fn event_handles_signal_before_poll_and_replaces_the_old_waker() {
            let already_ready = OneShotEvent::new();
            assert!(already_ready.signal());
            let mut ready_future = Box::pin(already_ready.wait());
            let mut noop_context = Context::from_waker(Waker::noop());
            assert_eq!(
                ready_future.as_mut().poll(&mut noop_context),
                Poll::Ready(())
            );

            let event = OneShotEvent::new();
            let mut future = Box::pin(event.wait());
            let (old_probe, old_waker) = probe();
            let (current_probe, current_waker) = probe();
            let mut old_context = Context::from_waker(&old_waker);
            let mut current_context = Context::from_waker(&current_waker);

            assert_eq!(future.as_mut().poll(&mut old_context), Poll::Pending);
            assert_eq!(future.as_mut().poll(&mut current_context), Poll::Pending);
            assert!(event.signal());
            assert_eq!(old_probe.wakes.load(Ordering::SeqCst), 0);
            assert_eq!(current_probe.wakes.load(Ordering::SeqCst), 1);
            assert_eq!(future.as_mut().poll(&mut current_context), Poll::Ready(()));
        }

        #[test]
        fn dropping_a_pending_waiter_unregisters_only_that_waiter() {
            let event = OneShotEvent::new();
            let mut future = Box::pin(event.wait());
            let (probe, waker) = probe();
            let mut context = Context::from_waker(&waker);

            assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
            assert_eq!(event.waiting_count(), 1);
            drop(future);
            assert_eq!(event.waiting_count(), 0);

            assert!(event.signal());
            assert_eq!(probe.wakes.load(Ordering::SeqCst), 0);
        }
    }
}

pub mod c35 {
    use std::future::Future;
    use std::num::ParseIntError;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum AsyncState {
        Created,
        WaitingFirst,
        WaitingSecond,
        Done,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct StateSnapshot {
        pub state: AsyncState,
        pub live_fields: &'static str,
    }

    // SOLUTION: C35-E01
    pub const fn two_await_state_trace() -> [StateSnapshot; 4] {
        [
            StateSnapshot {
                state: AsyncState::Created,
                live_fields: "id",
            },
            StateSnapshot {
                state: AsyncState::WaitingFirst,
                live_fields: "first_future",
            },
            StateSnapshot {
                state: AsyncState::WaitingSecond,
                live_fields: "first_output + second_future",
            },
            StateSnapshot {
                state: AsyncState::Done,
                live_fields: "none",
            },
        ]
    }

    // SOLUTION: C35-E02
    pub async fn checksum_before_wait(input: Vec<u8>) -> u64 {
        let checksum = {
            let buffer = input;
            buffer.iter().map(|byte| u64::from(*byte)).sum()
        };
        tokio::task::yield_now().await;
        checksum
    }

    fn require_non_empty(input: &str) -> Result<(), &'static str> {
        if input.is_empty() {
            Err("empty input")
        } else {
            Ok(())
        }
    }

    // SOLUTION: C35-E03
    pub async fn infer_three_outputs(
        input: &str,
    ) -> (u64, Result<u64, ParseIntError>, Result<(), &'static str>) {
        let direct = async { 42_u64 };
        let parsed = async {
            let value = input.parse::<u64>()?;
            Ok::<u64, ParseIntError>(value * 2)
        };
        let validated = async {
            require_non_empty(input)?;
            Ok::<(), &'static str>(())
        };

        (direct.await, parsed.await, validated.await)
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Node {
        pub children: Vec<Node>,
    }

    // SOLUTION: C35-E04
    pub async fn count_nodes(root: Node) -> usize {
        let mut stack = vec![root];
        let mut count = 0;
        while let Some(node) = stack.pop() {
            count += 1;
            stack.extend(node.children);
            tokio::task::yield_now().await;
        }
        count
    }

    struct DropCounter(Arc<AtomicUsize>);

    impl Drop for DropCounter {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    // SOLUTION: C35-E05
    pub async fn pending_with_drop_probe(constructed: Arc<AtomicUsize>, dropped: Arc<AtomicUsize>) {
        let _live_in_first_state = DropCounter(Arc::clone(&dropped));
        constructed.fetch_add(1, Ordering::SeqCst);
        std::future::pending::<()>().await;

        let _only_in_later_state = DropCounter(dropped);
        constructed.fetch_add(1, Ordering::SeqCst);
    }

    struct TracedReady {
        trace: Arc<Mutex<Vec<&'static str>>>,
    }

    impl Future for TracedReady {
        type Output = ();

        fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
            self.trace
                .lock()
                .expect("trace mutex poisoned")
                .push("child: Ready");
            Poll::Ready(())
        }
    }

    // SOLUTION: C35-E06
    pub async fn trace_immediately_ready_await(trace: Arc<Mutex<Vec<&'static str>>>) {
        trace
            .lock()
            .expect("trace mutex poisoned")
            .push("before await");
        TracedReady {
            trace: Arc::clone(&trace),
        }
        .await;
        trace
            .lock()
            .expect("trace mutex poisoned")
            .push("after await");
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct SuspensionInventory {
        pub state: &'static str,
        pub live_bytes: usize,
    }

    // SOLUTION: C35-E07
    pub fn dominant_suspension(states: &[SuspensionInventory]) -> Option<SuspensionInventory> {
        states.iter().copied().max_by_key(|state| state.live_bytes)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::task::Waker;

        #[test]
        fn two_awaits_have_two_possible_suspended_states() {
            let trace = two_await_state_trace();
            assert_eq!(trace[0].state, AsyncState::Created);
            assert_eq!(trace[1].live_fields, "first_future");
            assert_eq!(trace[2].live_fields, "first_output + second_future");
            assert_eq!(trace[3].state, AsyncState::Done);
        }

        #[tokio::test]
        async fn the_buffer_is_consumed_before_the_wait() {
            assert_eq!(checksum_before_wait(vec![10, 20, 12]).await, 42);
        }

        #[tokio::test]
        async fn three_async_blocks_expose_three_output_types() {
            let (direct, parsed, validated) = infer_three_outputs("21").await;
            assert_eq!(direct, 42);
            assert_eq!(parsed, Ok(42));
            assert_eq!(validated, Ok(()));

            let (_, parsed, validated) = infer_three_outputs("").await;
            assert!(parsed.is_err());
            assert_eq!(validated, Err("empty input"));
        }

        #[tokio::test]
        async fn explicit_stack_replaces_recursive_future() {
            let tree = Node {
                children: vec![
                    Node { children: vec![] },
                    Node {
                        children: vec![Node { children: vec![] }],
                    },
                ],
            };
            assert_eq!(count_nodes(tree).await, 4);
        }

        #[test]
        fn cancellation_drops_only_values_constructed_in_the_reached_state() {
            let constructed = Arc::new(AtomicUsize::new(0));
            let dropped = Arc::new(AtomicUsize::new(0));
            let mut future = Box::pin(pending_with_drop_probe(
                Arc::clone(&constructed),
                Arc::clone(&dropped),
            ));
            let mut context = Context::from_waker(Waker::noop());

            assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
            assert_eq!(constructed.load(Ordering::SeqCst), 1);
            assert_eq!(dropped.load(Ordering::SeqCst), 0);

            drop(future);
            assert_eq!(dropped.load(Ordering::SeqCst), 1);
            assert_eq!(constructed.load(Ordering::SeqCst), 1);
        }

        #[tokio::test]
        async fn a_ready_await_continues_without_a_pending_boundary() {
            let trace = Arc::new(Mutex::new(Vec::new()));
            trace_immediately_ready_await(Arc::clone(&trace)).await;
            assert_eq!(
                *trace.lock().expect("trace mutex poisoned"),
                ["before await", "child: Ready", "after await"]
            );
        }

        #[test]
        fn the_largest_live_state_dominates_the_inventory() {
            let states = [
                SuspensionInventory {
                    state: "waiting metadata",
                    live_bytes: 128,
                },
                SuspensionInventory {
                    state: "waiting body",
                    live_bytes: 1024,
                },
                SuspensionInventory {
                    state: "waiting commit",
                    live_bytes: 256,
                },
            ];
            assert_eq!(
                dominant_suspension(&states),
                Some(SuspensionInventory {
                    state: "waiting body",
                    live_bytes: 1024,
                })
            );
            assert_eq!(
                states.iter().map(|state| state.live_bytes).sum::<usize>(),
                1408
            );
        }
    }
}

pub mod c36 {
    use std::future::Future;
    use std::rc::Rc;
    use std::sync::Mutex;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CaptureKind {
        BorrowedInput,
        OwnedInput,
        BorrowedOutput,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct SignatureModel {
        pub source: &'static str,
        pub future_bound: &'static str,
        pub output: &'static str,
        pub capture: CaptureKind,
    }

    // SOLUTION: C36-E01
    pub const fn signature_models() -> [SignatureModel; 3] {
        [
            SignatureModel {
                source: "async fn length(&'a str) -> usize",
                future_bound: "+ 'a",
                output: "usize",
                capture: CaptureKind::BorrowedInput,
            },
            SignatureModel {
                source: "async fn length(String) -> usize",
                future_bound: "+ 'static if fields are 'static",
                output: "usize",
                capture: CaptureKind::OwnedInput,
            },
            SignatureModel {
                source: "async fn first(&'a str) -> Option<&'a str>",
                future_bound: "+ 'a",
                output: "Option<&'a str>",
                capture: CaptureKind::BorrowedOutput,
            },
        ]
    }

    // SOLUTION: C36-E02
    pub fn owned_length(value: &str) -> impl Future<Output = usize> + Send + 'static {
        let owned = value.to_owned();
        async move {
            tokio::task::yield_now().await;
            owned.len()
        }
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct VersionedState {
        pub revision: u64,
        pub prepared: bool,
        pub finished: bool,
    }

    // SOLUTION: C36-E03
    pub async fn update_after_notification<Notification>(
        shared: &Mutex<VersionedState>,
        notification: Notification,
    ) -> bool
    where
        Notification: Future<Output = ()>,
    {
        let expected_revision = {
            let mut state = shared.lock().expect("state mutex poisoned");
            state.revision = state.revision.checked_add(1).expect("revision overflow");
            state.prepared = true;
            state.revision
        };

        notification.await;

        let mut state = shared.lock().expect("state mutex poisoned");
        if state.revision != expected_revision {
            return false;
        }
        state.finished = true;
        true
    }

    // SOLUTION: C36-E04
    pub async fn first_line_borrowed(input: &str) -> Option<&str> {
        tokio::task::yield_now().await;
        input.lines().next()
    }

    pub fn first_line_owned(input: &str) -> impl Future<Output = Option<String>> + Send + 'static {
        let owned = input.to_owned();
        async move {
            tokio::task::yield_now().await;
            owned.lines().next().map(str::to_owned)
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ExecutionRequirements {
        pub must_outlive_caller: bool,
        pub future_send: bool,
        pub future_static: bool,
        pub output_send: bool,
        pub output_static: bool,
        pub intentional_local_affinity: bool,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ExecutionChoice {
        AwaitOrCompose,
        Spawn,
        SpawnLocal,
        Redesign,
    }

    // SOLUTION: C36-E05
    pub const fn choose_execution(requirements: ExecutionRequirements) -> ExecutionChoice {
        if !requirements.must_outlive_caller {
            ExecutionChoice::AwaitOrCompose
        } else if requirements.future_send
            && requirements.future_static
            && requirements.output_send
            && requirements.output_static
        {
            ExecutionChoice::Spawn
        } else if requirements.future_static
            && requirements.output_static
            && requirements.intentional_local_affinity
        {
            ExecutionChoice::SpawnLocal
        } else {
            ExecutionChoice::Redesign
        }
    }

    // SOLUTION: C36-E06
    pub async fn spawn_with_temporary_rc() -> usize {
        tokio::spawn(async {
            let length = {
                let local = Rc::new(String::from("rust"));
                local.len()
            };

            tokio::task::yield_now().await;
            length
        })
        .await
        .expect("temporary Rc task panicked")
    }

    // SOLUTION: C36-E07
    pub async fn run_owned_rc_locally(value: String) -> usize {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async move {
                let value = Rc::new(value);
                tokio::task::spawn_local(async move {
                    tokio::task::yield_now().await;
                    value.len()
                })
                .await
                .expect("local Rc task panicked")
            })
            .await
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn three_signatures_keep_their_lifetime_relationships() {
            let models = signature_models();
            assert_eq!(models[0].future_bound, "+ 'a");
            assert_eq!(models[0].capture, CaptureKind::BorrowedInput);
            assert_eq!(models[1].capture, CaptureKind::OwnedInput);
            assert_eq!(models[2].output, "Option<&'a str>");
        }

        #[tokio::test]
        async fn owned_future_survives_the_input() {
            let future = {
                let local = String::from("owned");
                owned_length(&local)
            };
            assert_eq!(future.await, 5);
        }

        #[tokio::test]
        async fn revision_check_rejects_a_stale_post_await_transition() {
            let stable = Mutex::new(VersionedState::default());
            assert!(update_after_notification(&stable, std::future::ready(())).await);
            assert!(stable.lock().expect("state mutex poisoned").finished);

            let changed = Mutex::new(VersionedState::default());
            let concurrent_change = async {
                let mut state = changed.lock().expect("state mutex poisoned");
                state.revision += 1;
            };
            assert!(!update_after_notification(&changed, concurrent_change).await);
            assert!(!changed.lock().expect("state mutex poisoned").finished);
        }

        #[tokio::test]
        async fn borrowed_and_owned_outputs_offer_different_independence() {
            let input = String::from("first\nsecond");
            assert_eq!(first_line_borrowed(&input).await, Some("first"));

            let owned_future = first_line_owned(&input);
            drop(input);
            assert_eq!(owned_future.await, Some(String::from("first")));
        }

        #[test]
        fn execution_choice_keeps_send_and_static_separate() {
            let base = ExecutionRequirements {
                must_outlive_caller: true,
                future_send: true,
                future_static: true,
                output_send: true,
                output_static: true,
                intentional_local_affinity: false,
            };
            assert_eq!(choose_execution(base), ExecutionChoice::Spawn);
            assert_eq!(
                choose_execution(ExecutionRequirements {
                    must_outlive_caller: false,
                    ..base
                }),
                ExecutionChoice::AwaitOrCompose
            );
            assert_eq!(
                choose_execution(ExecutionRequirements {
                    future_send: false,
                    intentional_local_affinity: true,
                    ..base
                }),
                ExecutionChoice::SpawnLocal
            );
            assert_eq!(
                choose_execution(ExecutionRequirements {
                    future_send: false,
                    future_static: false,
                    intentional_local_affinity: true,
                    ..base
                }),
                ExecutionChoice::Redesign
            );
        }

        #[tokio::test]
        async fn a_temporary_rc_does_not_cross_the_await() {
            assert_eq!(spawn_with_temporary_rc().await, 4);
        }

        #[tokio::test(flavor = "current_thread")]
        async fn an_owned_rc_can_cross_await_inside_a_local_task() {
            assert_eq!(run_owned_rc_locally(String::from("local")).await, 5);
        }
    }
}

pub mod c37 {
    use std::cell::Cell;
    use std::future::Future;
    use std::marker::PhantomPinned;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, Waker};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum PinScenario {
        OrdinaryAwait,
        TemporaryManualPoll,
        ReusedSelectBranch,
        OwnedStoredFuture,
        HeterogeneousOwnedFuture,
        AddressInsensitiveValue,
        HandWrittenSelfReference,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum PinStrategy {
        AwaitNormally,
        StackPin,
        BoxPin,
        Redesign,
    }

    // SOLUTION: C37-E01
    pub const fn choose_pin_strategy(scenario: PinScenario) -> PinStrategy {
        match scenario {
            PinScenario::OrdinaryAwait | PinScenario::AddressInsensitiveValue => {
                PinStrategy::AwaitNormally
            }
            PinScenario::TemporaryManualPoll | PinScenario::ReusedSelectBranch => {
                PinStrategy::StackPin
            }
            PinScenario::OwnedStoredFuture | PinScenario::HeterogeneousOwnedFuture => {
                PinStrategy::BoxPin
            }
            PinScenario::HandWrittenSelfReference => PinStrategy::Redesign,
        }
    }

    // SOLUTION: C37-E02
    pub type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
    pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

    pub fn boxed_value(value: u64) -> BoxFuture<'static, u64> {
        Box::pin(async move { value })
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ProjectionKind {
        Pinned,
        Mutable,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct FieldProjection {
        pub field: &'static str,
        pub projection: ProjectionKind,
    }

    // SOLUTION: C37-E03
    pub const fn parent_projection_plan() -> [FieldProjection; 3] {
        [
            FieldProjection {
                field: "child",
                projection: ProjectionKind::Pinned,
            },
            FieldProjection {
                field: "output",
                projection: ProjectionKind::Mutable,
            },
            FieldProjection {
                field: "state",
                projection: ProjectionKind::Mutable,
            },
        ]
    }

    struct AddressSensitive {
        value: u64,
        _pin: PhantomPinned,
    }

    // SOLUTION: C37-E04
    pub fn moved_handle_addresses(value: u64) -> (usize, usize, u64) {
        let pinned = Box::pin(AddressSensitive {
            value,
            _pin: PhantomPinned,
        });
        let before = pinned.as_ref().get_ref() as *const AddressSensitive as usize;

        let moved_handle = pinned;
        let after = moved_handle.as_ref().get_ref() as *const AddressSensitive as usize;
        (before, after, moved_handle.as_ref().get_ref().value)
    }

    struct StepFuture {
        pending_rounds: usize,
        polls: usize,
    }

    impl Future for StepFuture {
        type Output = usize;

        fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.get_mut();
            this.polls += 1;
            if this.pending_rounds == 0 {
                Poll::Ready(this.polls)
            } else {
                this.pending_rounds -= 1;
                context.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    // SOLUTION: C37-E05
    pub fn polls_to_completion(pending_rounds: usize) -> usize {
        let mut future = std::pin::pin!(StepFuture {
            pending_rounds,
            polls: 0,
        });
        let mut context = Context::from_waker(Waker::noop());

        loop {
            if let Poll::Ready(polls) = future.as_mut().poll(&mut context) {
                return polls;
            }
        }
    }

    pub struct AddrTracker {
        previous: Cell<Option<usize>>,
        _pin: PhantomPinned,
    }

    impl Default for AddrTracker {
        fn default() -> Self {
            Self::new()
        }
    }

    impl AddrTracker {
        pub const fn new() -> Self {
            Self {
                previous: Cell::new(None),
                _pin: PhantomPinned,
            }
        }

        pub fn observe(self: Pin<&Self>) -> bool {
            let current = self.get_ref() as *const Self as usize;
            match self.previous.replace(Some(current)) {
                Some(previous) => previous == current,
                None => true,
            }
        }
    }

    // SOLUTION: C37-E06
    pub fn local_address_tracker_is_stable() -> bool {
        let tracker = std::pin::pin!(AddrTracker::new());
        tracker.as_ref().observe() && tracker.as_ref().observe()
    }

    struct PinnedDropProbe {
        id: u8,
        drops: Arc<Mutex<Vec<u8>>>,
        _pin: PhantomPinned,
    }

    impl PinnedDropProbe {
        fn new(id: u8, drops: Arc<Mutex<Vec<u8>>>) -> Self {
            Self {
                id,
                drops,
                _pin: PhantomPinned,
            }
        }
    }

    impl Drop for PinnedDropProbe {
        fn drop(&mut self) {
            self.drops
                .lock()
                .expect("drop trace mutex poisoned")
                .push(self.id);
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct PinSetEvidence {
        pub address_stable: bool,
        pub drops_after_set: Vec<u8>,
        pub drops_after_final_drop: Vec<u8>,
    }

    // SOLUTION: C37-E07
    pub fn pin_set_evidence() -> PinSetEvidence {
        let drops = Arc::new(Mutex::new(Vec::new()));
        let mut pinned = Box::pin(PinnedDropProbe::new(1, Arc::clone(&drops)));
        let before = pinned.as_ref().get_ref() as *const PinnedDropProbe as usize;

        pinned
            .as_mut()
            .set(PinnedDropProbe::new(2, Arc::clone(&drops)));
        let after = pinned.as_ref().get_ref() as *const PinnedDropProbe as usize;
        let drops_after_set = drops.lock().expect("drop trace mutex poisoned").clone();

        drop(pinned);
        let drops_after_final_drop = drops.lock().expect("drop trace mutex poisoned").clone();

        PinSetEvidence {
            address_stable: before == after,
            drops_after_set,
            drops_after_final_drop,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn pinning_strategy_follows_storage_and_lifetime_needs() {
            assert_eq!(
                choose_pin_strategy(PinScenario::OrdinaryAwait),
                PinStrategy::AwaitNormally
            );
            assert_eq!(
                choose_pin_strategy(PinScenario::ReusedSelectBranch),
                PinStrategy::StackPin
            );
            assert_eq!(
                choose_pin_strategy(PinScenario::HeterogeneousOwnedFuture),
                PinStrategy::BoxPin
            );
            assert_eq!(
                choose_pin_strategy(PinScenario::HandWrittenSelfReference),
                PinStrategy::Redesign
            );
        }

        #[tokio::test]
        async fn boxed_alias_erases_the_future_type() {
            assert_eq!(boxed_value(7).await, 7);
        }

        #[test]
        fn only_the_child_is_structurally_pinned() {
            assert_eq!(
                parent_projection_plan(),
                [
                    FieldProjection {
                        field: "child",
                        projection: ProjectionKind::Pinned,
                    },
                    FieldProjection {
                        field: "output",
                        projection: ProjectionKind::Mutable,
                    },
                    FieldProjection {
                        field: "state",
                        projection: ProjectionKind::Mutable,
                    },
                ]
            );
        }

        #[test]
        fn moving_the_pin_box_handle_keeps_the_pointee_address() {
            let (before, after, value) = moved_handle_addresses(42);
            assert_eq!(before, after);
            assert_eq!(value, 42);
        }

        #[test]
        fn the_same_future_accumulates_progress_across_polls() {
            assert_eq!(polls_to_completion(3), 4);
        }

        #[test]
        fn a_locally_pinned_tracker_observes_one_address() {
            assert!(local_address_tracker_is_stable());
        }

        #[test]
        fn pin_set_drops_the_old_value_before_reusing_its_location() {
            assert_eq!(
                pin_set_evidence(),
                PinSetEvidence {
                    address_stable: true,
                    drops_after_set: vec![1],
                    drops_after_final_drop: vec![1, 2],
                }
            );
        }
    }
}

pub mod c38 {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::time::Duration;
    use tokio::sync::mpsc;

    // SOLUTION: C38-E01
    pub async fn try_join_independent<Left, Right, A, B, E>(
        left: Left,
        right: Right,
    ) -> Result<(A, B), E>
    where
        Left: Future<Output = Result<A, E>>,
        Right: Future<Output = Result<B, E>>,
    {
        tokio::try_join!(left, right)
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum TimeoutError<E> {
        DeadlineExceeded,
        Operation(E),
    }

    // SOLUTION: C38-E02
    pub async fn with_timeout<F, T, E>(
        duration: Duration,
        operation: F,
    ) -> Result<T, TimeoutError<E>>
    where
        F: Future<Output = Result<T, E>>,
    {
        tokio::time::timeout(duration, operation)
            .await
            .map_err(|_| TimeoutError::DeadlineExceeded)?
            .map_err(TimeoutError::Operation)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CancellationClass {
        Restartable,
        Compensable,
        ProtectedFromCaller,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AwaitAudit {
        pub operation: &'static str,
        pub progress_owner: &'static str,
        pub class: CancellationClass,
    }

    // SOLUTION: C38-E03
    pub const fn checkout_cancellation_audit() -> [AwaitAudit; 3] {
        [
            AwaitAudit {
                operation: "load_cart",
                progress_owner: "database",
                class: CancellationClass::Restartable,
            },
            AwaitAudit {
                operation: "reserve_stock",
                progress_owner: "inventory service",
                class: CancellationClass::Compensable,
            },
            AwaitAudit {
                operation: "persist_charge",
                progress_owner: "supervised payment task",
                class: CancellationClass::ProtectedFromCaller,
            },
        ]
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Command {
        Store(u64),
        Shutdown,
    }

    // SOLUTION: C38-E04
    pub async fn run_draining_worker(mut commands: mpsc::Receiver<Command>) -> Vec<u64> {
        let mut stored = Vec::new();
        while let Some(command) = commands.recv().await {
            match command {
                Command::Store(value) => stored.push(value),
                Command::Shutdown => {
                    commands.close();
                    while let Some(Command::Store(value)) = commands.recv().await {
                        stored.push(value);
                    }
                    break;
                }
            }
        }
        stored
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum FailurePolicy {
        AbortAndJoin,
        CollectAll,
        Isolate,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ResultOrder {
        Completion,
        Input,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct GroupPolicy {
        pub task_count: usize,
        pub max_in_flight: usize,
        pub on_failure: FailurePolicy,
        pub result_order: ResultOrder,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum GroupPolicyError {
        NoTasks,
        ZeroConcurrency,
        LimitExceedsTaskCount,
    }

    // SOLUTION: C38-E05
    pub const fn validate_group_policy(
        policy: GroupPolicy,
    ) -> Result<GroupPolicy, GroupPolicyError> {
        if policy.task_count == 0 {
            return Err(GroupPolicyError::NoTasks);
        }
        if policy.max_in_flight == 0 {
            return Err(GroupPolicyError::ZeroConcurrency);
        }
        if policy.max_in_flight > policy.task_count {
            return Err(GroupPolicyError::LimitExceedsTaskCount);
        }
        Ok(policy)
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Deadline(tokio::time::Instant);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct DeadlineExpired;

    impl Deadline {
        pub fn after(budget: Duration) -> Self {
            Self(tokio::time::Instant::now() + budget)
        }

        // SOLUTION: C38-E06
        pub fn remaining(self) -> Result<Duration, DeadlineExpired> {
            self.0
                .checked_duration_since(tokio::time::Instant::now())
                .filter(|remaining| !remaining.is_zero())
                .ok_or(DeadlineExpired)
        }
    }

    #[derive(Debug, Default)]
    struct PendingOnce {
        polls: usize,
    }

    impl Future for PendingOnce {
        type Output = usize;

        fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            let state = self.get_mut();
            state.polls += 1;
            if state.polls == 1 {
                context.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(state.polls)
            }
        }
    }

    // SOLUTION: C38-E07
    pub async fn preserve_external_future_after_select() -> (bool, usize) {
        let future = PendingOnce::default();
        tokio::pin!(future);

        let lost_first_race = tokio::select! {
            biased;
            polls = &mut future => return (false, polls),
            _ = std::future::ready(()) => true,
        };

        (lost_first_race, future.await)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn try_join_reports_success_and_the_observed_error() {
            assert_eq!(
                try_join_independent(async { Ok::<_, &'static str>(2) }, async { Ok(3) }).await,
                Ok((2, 3))
            );

            let never = std::future::pending::<Result<u64, &'static str>>();
            assert_eq!(
                try_join_independent(async { Err::<u64, _>("left") }, never).await,
                Err("left")
            );
        }

        #[tokio::test(start_paused = true)]
        async fn timeout_distinguishes_deadline_from_operation_error() {
            let slow = async {
                tokio::time::sleep(Duration::from_secs(10)).await;
                Ok::<_, &'static str>(1)
            };
            assert_eq!(
                with_timeout(Duration::from_secs(1), slow).await,
                Err(TimeoutError::DeadlineExceeded)
            );
            assert_eq!(
                with_timeout(Duration::from_secs(1), async { Err::<u64, _>("backend") }).await,
                Err(TimeoutError::Operation("backend"))
            );
        }

        #[test]
        fn cancellation_audit_names_progress_owner_and_recovery_policy() {
            let audit = checkout_cancellation_audit();
            assert_eq!(audit[0].class, CancellationClass::Restartable);
            assert_eq!(audit[1].class, CancellationClass::Compensable);
            assert_eq!(
                audit[2],
                AwaitAudit {
                    operation: "persist_charge",
                    progress_owner: "supervised payment task",
                    class: CancellationClass::ProtectedFromCaller,
                }
            );
        }

        #[tokio::test]
        async fn shutdown_drains_commands_already_in_the_bounded_channel() {
            let (sender, receiver) = mpsc::channel(4);
            sender.send(Command::Store(1)).await.unwrap();
            sender.send(Command::Shutdown).await.unwrap();
            sender.send(Command::Store(2)).await.unwrap();
            drop(sender);
            assert_eq!(run_draining_worker(receiver).await, [1, 2]);
        }

        #[test]
        fn group_policy_is_explicit_and_bounded() {
            let policy = GroupPolicy {
                task_count: 10,
                max_in_flight: 3,
                on_failure: FailurePolicy::AbortAndJoin,
                result_order: ResultOrder::Completion,
            };
            assert_eq!(validate_group_policy(policy), Ok(policy));
            assert_eq!(
                validate_group_policy(GroupPolicy {
                    max_in_flight: 0,
                    ..policy
                }),
                Err(GroupPolicyError::ZeroConcurrency)
            );
        }

        #[tokio::test(start_paused = true)]
        async fn every_hop_receives_only_the_remaining_deadline_budget() {
            let deadline = Deadline::after(Duration::from_secs(10));
            assert_eq!(deadline.remaining(), Ok(Duration::from_secs(10)));

            tokio::time::advance(Duration::from_secs(4)).await;
            assert_eq!(deadline.remaining(), Ok(Duration::from_secs(6)));

            tokio::time::advance(Duration::from_secs(6)).await;
            assert_eq!(deadline.remaining(), Err(DeadlineExpired));
        }

        #[tokio::test]
        async fn losing_select_branch_does_not_drop_an_external_future() {
            assert_eq!(preserve_external_future_after_select().await, (true, 2));
        }
    }
}

pub mod c39 {
    use tokio::sync::{broadcast, mpsc, oneshot, watch};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct CapacityInput {
        pub peak_per_second: u64,
        pub service_per_second: u64,
        pub burst_millis: u64,
        pub bytes_per_item: u64,
        pub memory_budget_bytes: u64,
        pub max_in_flight: usize,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct CapacityPlan {
        pub queue_capacity: usize,
        pub max_in_flight: usize,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CapacityError {
        ZeroMessageSize,
        ZeroParallelism,
        ArithmeticOverflow,
        InsufficientMemory {
            required_slots: u64,
            affordable_slots: u64,
        },
    }

    // SOLUTION: C39-E01
    pub fn plan_bounded_stage(input: CapacityInput) -> Result<CapacityPlan, CapacityError> {
        if input.bytes_per_item == 0 {
            return Err(CapacityError::ZeroMessageSize);
        }
        if input.max_in_flight == 0 {
            return Err(CapacityError::ZeroParallelism);
        }

        let excess_per_second = input
            .peak_per_second
            .saturating_sub(input.service_per_second);
        let backlog_millis = excess_per_second
            .checked_mul(input.burst_millis)
            .ok_or(CapacityError::ArithmeticOverflow)?;
        let required_slots = backlog_millis
            .checked_add(999)
            .ok_or(CapacityError::ArithmeticOverflow)?
            / 1_000;
        let required_slots = required_slots.max(1);
        let affordable_slots = input.memory_budget_bytes / input.bytes_per_item;

        if required_slots > affordable_slots {
            return Err(CapacityError::InsufficientMemory {
                required_slots,
                affordable_slots,
            });
        }

        Ok(CapacityPlan {
            queue_capacity: usize::try_from(required_slots)
                .map_err(|_| CapacityError::ArithmeticOverflow)?,
            max_in_flight: input.max_in_flight,
        })
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ArithmeticError {
        Overflow,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CalculatorError {
        ServiceClosed,
        ResponseCancelled,
        Domain(ArithmeticError),
    }

    enum Command {
        Add {
            left: i64,
            right: i64,
            reply: oneshot::Sender<Result<i64, ArithmeticError>>,
        },
    }

    pub struct Calculator {
        commands: mpsc::Sender<Command>,
    }

    // SOLUTION: C39-E02
    impl Calculator {
        pub fn start(capacity: usize) -> Self {
            let (commands, mut receiver) = mpsc::channel(capacity);
            tokio::spawn(async move {
                while let Some(command) = receiver.recv().await {
                    match command {
                        Command::Add { left, right, reply } => {
                            let result = left.checked_add(right).ok_or(ArithmeticError::Overflow);
                            let _ = reply.send(result);
                        }
                    }
                }
            });
            Self { commands }
        }

        pub async fn add(&self, left: i64, right: i64) -> Result<i64, CalculatorError> {
            let (reply, answer) = oneshot::channel();
            self.commands
                .send(Command::Add { left, right, reply })
                .await
                .map_err(|_| CalculatorError::ServiceClosed)?;
            answer
                .await
                .map_err(|_| CalculatorError::ResponseCancelled)?
                .map_err(CalculatorError::Domain)
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum DeliveryOrder {
        Input,
        Completion,
    }

    // SOLUTION: C39-E03
    pub fn delivered_ids(completion_order: &[usize], order: DeliveryOrder) -> Vec<usize> {
        let mut delivered = completion_order.to_vec();
        if order == DeliveryOrder::Input {
            delivered.sort_unstable();
        }
        delivered
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum LagPolicy {
        Abort,
        Resync,
        SkipEphemeral,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum BroadcastEvent<T> {
        Item(T),
        ResyncRequired { missed: u64 },
        Skipped { missed: u64 },
        Closed,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct BroadcastGap {
        pub missed: u64,
    }

    // SOLUTION: C39-E04
    pub async fn receive_broadcast<T: Clone>(
        receiver: &mut broadcast::Receiver<T>,
        policy: LagPolicy,
    ) -> Result<BroadcastEvent<T>, BroadcastGap> {
        match receiver.recv().await {
            Ok(item) => Ok(BroadcastEvent::Item(item)),
            Err(broadcast::error::RecvError::Closed) => Ok(BroadcastEvent::Closed),
            Err(broadcast::error::RecvError::Lagged(missed)) => match policy {
                LagPolicy::Abort => Err(BroadcastGap { missed }),
                LagPolicy::Resync => Ok(BroadcastEvent::ResyncRequired { missed }),
                LagPolicy::SkipEphemeral => Ok(BroadcastEvent::Skipped { missed }),
            },
        }
    }

    // SOLUTION: C39-E05
    pub async fn close_and_drain<T>(receiver: &mut mpsc::Receiver<T>) -> Vec<T> {
        receiver.close();
        let mut drained = Vec::new();
        while let Some(item) = receiver.recv().await {
            drained.push(item);
        }
        drained
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AdmissionClosed;

    // SOLUTION: C39-E06
    pub async fn reserve_then_build<T, Build>(
        sender: &mpsc::Sender<T>,
        build: Build,
    ) -> Result<(), AdmissionClosed>
    where
        Build: FnOnce() -> T,
    {
        let permit = sender.reserve().await.map_err(|_| AdmissionClosed)?;
        permit.send(build());
        Ok(())
    }

    // SOLUTION: C39-E07
    pub async fn next_watch_value<T: Clone>(
        receiver: &mut watch::Receiver<T>,
    ) -> Result<T, watch::error::RecvError> {
        receiver.changed().await?;
        Ok(receiver.borrow_and_update().clone())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::atomic::{AtomicUsize, Ordering};

        #[test]
        fn capacity_plan_fits_burst_and_rejects_an_impossible_budget() {
            let input = CapacityInput {
                peak_per_second: 120,
                service_per_second: 100,
                burst_millis: 1_500,
                bytes_per_item: 256,
                memory_budget_bytes: 8_192,
                max_in_flight: 8,
            };
            assert_eq!(
                plan_bounded_stage(input),
                Ok(CapacityPlan {
                    queue_capacity: 30,
                    max_in_flight: 8,
                })
            );
            assert_eq!(
                plan_bounded_stage(CapacityInput {
                    memory_budget_bytes: 29 * 256,
                    ..input
                }),
                Err(CapacityError::InsufficientMemory {
                    required_slots: 30,
                    affordable_slots: 29,
                })
            );
        }

        #[tokio::test]
        async fn oneshot_preserves_success_and_domain_error_layers() {
            let calculator = Calculator::start(2);
            assert_eq!(calculator.add(20, 22).await, Ok(42));
            assert_eq!(
                calculator.add(i64::MAX, 1).await,
                Err(CalculatorError::Domain(ArithmeticError::Overflow))
            );
        }

        #[test]
        fn unordered_delivery_does_not_wait_for_the_slow_first_item() {
            let completion_order = [1, 2, 0];
            assert_eq!(
                delivered_ids(&completion_order, DeliveryOrder::Completion),
                [1, 2, 0]
            );
            assert_eq!(
                delivered_ids(&completion_order, DeliveryOrder::Input),
                [0, 1, 2]
            );
        }

        #[tokio::test]
        async fn lag_is_an_explicit_event_and_the_receiver_can_continue() {
            let (sender, mut receiver) = broadcast::channel(2);
            sender.send(10).unwrap();
            sender.send(20).unwrap();
            sender.send(30).unwrap();

            assert_eq!(
                receive_broadcast(&mut receiver, LagPolicy::Resync).await,
                Ok(BroadcastEvent::ResyncRequired { missed: 1 })
            );
            assert_eq!(
                receive_broadcast(&mut receiver, LagPolicy::Resync).await,
                Ok(BroadcastEvent::Item(20))
            );
        }

        #[tokio::test]
        async fn a_permit_reserved_before_close_is_still_drained() {
            let (sender, mut receiver) = mpsc::channel(2);
            sender.send(1).await.unwrap();
            let permit = sender.reserve().await.unwrap();

            receiver.close();
            permit.send(2);
            drop(sender);

            assert_eq!(close_and_drain(&mut receiver).await, [1, 2]);
        }

        #[tokio::test]
        async fn an_expensive_message_is_built_only_after_admission() {
            let builds = AtomicUsize::new(0);
            let (sender, mut receiver) = mpsc::channel(1);
            reserve_then_build(&sender, || {
                builds.fetch_add(1, Ordering::SeqCst);
                7
            })
            .await
            .unwrap();
            assert_eq!(receiver.recv().await, Some(7));

            drop(receiver);
            assert_eq!(
                reserve_then_build(&sender, || {
                    builds.fetch_add(1, Ordering::SeqCst);
                    9
                })
                .await,
                Err(AdmissionClosed)
            );
            assert_eq!(builds.load(Ordering::SeqCst), 1);
        }

        #[tokio::test]
        async fn watch_coalesces_pending_updates_into_the_latest_value() {
            let (sender, mut receiver) = watch::channel(0);
            sender.send(1).unwrap();
            sender.send(2).unwrap();

            assert_eq!(next_watch_value(&mut receiver).await.unwrap(), 2);
            assert!(!receiver.has_changed().unwrap());

            drop(sender);
            assert!(next_watch_value(&mut receiver).await.is_err());
        }
    }
}

pub mod c40 {
    use std::future::Future;
    use std::pin::Pin;

    // SOLUTION: C40-E01
    pub async fn call_shared<F>(callback: &F) -> usize
    where
        F: AsyncFn() -> usize,
    {
        callback().await
    }

    pub async fn call_mut<F>(callback: &mut F) -> usize
    where
        F: AsyncFnMut() -> usize,
    {
        callback().await
    }

    pub async fn call_once<F>(callback: F) -> String
    where
        F: AsyncFnOnce() -> String,
    {
        callback().await
    }

    // SOLUTION: C40-E02
    pub async fn push_with_lending_closure(values: &mut Vec<u64>, value: u64) -> usize {
        let mut push = async || {
            std::future::ready(()).await;
            values.push(value);
            values.len()
        };
        push().await
    }

    // SOLUTION: C40-E03
    pub async fn visit_all<F>(items: &[String], visitor: F) -> Vec<usize>
    where
        F: for<'a> AsyncFn(&'a str) -> usize,
    {
        let mut outputs = Vec::with_capacity(items.len());
        for item in items {
            outputs.push(visitor(item).await);
        }
        outputs
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum RetryError<E> {
        NoAttempts,
        Exhausted { attempts: usize, last: E },
    }

    // SOLUTION: C40-E04
    pub async fn retry<F, T, E>(mut operation: F, max_attempts: usize) -> Result<T, RetryError<E>>
    where
        F: AsyncFnMut() -> Result<T, E>,
    {
        if max_attempts == 0 {
            return Err(RetryError::NoAttempts);
        }

        for attempt in 1..=max_attempts {
            match operation().await {
                Ok(value) => return Ok(value),
                Err(last) if attempt == max_attempts => {
                    return Err(RetryError::Exhausted {
                        attempts: attempt,
                        last,
                    });
                }
                Err(_) => {}
            }
        }
        unreachable!("max_attempts is non-zero and the loop always returns")
    }

    pub trait DynCallback {
        fn call<'a>(&'a mut self, input: &'a str) -> Pin<Box<dyn Future<Output = usize> + 'a>>;
    }

    impl<F> DynCallback for F
    where
        F: for<'a> AsyncFnMut(&'a str) -> usize,
    {
        fn call<'a>(&'a mut self, input: &'a str) -> Pin<Box<dyn Future<Output = usize> + 'a>> {
            Box::pin(self(input))
        }
    }

    // SOLUTION: C40-E05
    pub async fn run_dyn_callbacks(
        callbacks: &mut [Box<dyn DynCallback>],
        input: &str,
    ) -> Vec<usize> {
        let mut outputs = Vec::with_capacity(callbacks.len());
        for callback in callbacks {
            outputs.push(callback.call(input).await);
        }
        outputs
    }

    // SOLUTION: C40-E06
    pub async fn call_mut_sequentially<F>(callback: &mut F) -> (usize, usize)
    where
        F: AsyncFnMut() -> usize,
    {
        let first = callback().await;
        let second = callback().await;
        (first, second)
    }

    pub type SendFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

    // SOLUTION: C40-E07
    pub async fn spawn_boxed_callback<F, T>(callback: F) -> Result<T, tokio::task::JoinError>
    where
        F: FnOnce() -> SendFuture<T> + Send + 'static,
        T: Send + 'static,
    {
        tokio::spawn(callback()).await
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn the_minimal_async_call_trait_matches_capture_use() {
            let base = 40;
            let shared = async || base + 2;
            assert_eq!(call_shared(&shared).await, 42);
            assert_eq!(call_shared(&shared).await, 42);

            let mut count = 0;
            let mut mutable = async || {
                count += 1;
                count
            };
            assert_eq!(call_mut(&mut mutable).await, 1);
            assert_eq!(call_mut(&mut mutable).await, 2);

            let token = String::from("consumed");
            let once = async move || token;
            assert_eq!(call_once(once).await, "consumed");
        }

        #[tokio::test]
        async fn async_closure_future_can_borrow_a_mutable_capture() {
            let mut values = vec![1, 2];
            assert_eq!(push_with_lending_closure(&mut values, 3).await, 3);
            assert_eq!(values, [1, 2, 3]);
        }

        #[tokio::test]
        async fn higher_ranked_callback_accepts_each_fresh_borrow() {
            async fn length(value: &str) -> usize {
                value.len()
            }

            let items = vec![String::from("Rust"), String::from("async")];
            assert_eq!(visit_all(&items, length).await, [4, 5]);
        }

        #[tokio::test]
        async fn retry_reuses_mutable_state_and_preserves_the_last_error() {
            let mut attempts = 0;
            let operation = async || {
                attempts += 1;
                if attempts < 3 { Err(attempts) } else { Ok(42) }
            };

            assert_eq!(retry(operation, 3).await, Ok(42));
            assert_eq!(attempts, 3);

            assert_eq!(
                retry(async || Err::<u64, _>("offline"), 2).await,
                Err(RetryError::Exhausted {
                    attempts: 2,
                    last: "offline",
                })
            );
        }

        #[tokio::test]
        async fn boxed_trait_erases_heterogeneous_async_callback_types() {
            let suffix = String::from("!");
            let mut callbacks: Vec<Box<dyn DynCallback>> = vec![
                Box::new(async |input: &str| input.len()),
                Box::new(async move |input: &str| input.len() + suffix.len()),
            ];

            assert_eq!(run_dyn_callbacks(&mut callbacks, "Rust").await, [4, 5]);
        }

        #[tokio::test]
        async fn mutable_lending_calls_release_the_borrow_between_awaits() {
            let mut state = 0;
            let mut callback = async || {
                state += 1;
                state
            };

            assert_eq!(call_mut_sequentially(&mut callback).await, (1, 2));
        }

        #[tokio::test]
        async fn boxed_send_future_makes_the_spawn_contract_explicit() {
            let value = String::from("owned by task");
            let callback = move || Box::pin(async move { value.len() }) as SendFuture<usize>;

            assert_eq!(spawn_boxed_callback(callback).await.unwrap(), 13);
        }
    }
}

pub mod c41 {
    use std::cell::RefCell;
    use std::future::{Future, Ready, ready};
    use std::pin::Pin;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct User {
        pub id: u64,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SaveError {
        Duplicate,
    }

    #[derive(Default)]
    pub struct MemoryRepository {
        users: Vec<User>,
    }

    impl MemoryRepository {
        pub fn new(users: Vec<User>) -> Self {
            Self { users }
        }
    }

    // Este contrato es deliberadamente local; no promete `Send` para sus futures.
    #[allow(async_fn_in_trait)]
    pub trait NativeUserRepository {
        async fn find(&self, id: u64) -> Option<User>;
        async fn save(&mut self, user: User) -> Result<(), SaveError>;
    }

    // SOLUTION: C41-E01
    impl NativeUserRepository for MemoryRepository {
        async fn find(&self, id: u64) -> Option<User> {
            self.users.iter().find(|user| user.id == id).cloned()
        }

        async fn save(&mut self, user: User) -> Result<(), SaveError> {
            if self.users.iter().any(|stored| stored.id == user.id) {
                return Err(SaveError::Duplicate);
            }
            self.users.push(user);
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    pub struct LocalSequence {
        value: Rc<RefCell<u64>>,
    }

    #[allow(async_fn_in_trait)]
    pub trait LocalNext {
        async fn next(&self) -> u64;
    }

    impl LocalNext for LocalSequence {
        async fn next(&self) -> u64 {
            let mut value = self.value.borrow_mut();
            *value += 1;
            *value
        }
    }

    #[derive(Default)]
    pub struct ThreadSafeSequence {
        value: AtomicU64,
    }

    pub trait SendNext: Send + Sync {
        fn next(&self) -> impl Future<Output = u64> + Send;
    }

    // SOLUTION: C41-E02
    impl SendNext for ThreadSafeSequence {
        async fn next(&self) -> u64 {
            self.value.fetch_add(1, Ordering::SeqCst) + 1
        }
    }

    // SOLUTION: C41-E03
    pub trait DynUserRepository: Send + Sync {
        fn find(&self, id: u64) -> Pin<Box<dyn Future<Output = Option<User>> + Send + '_>>;
    }

    impl DynUserRepository for MemoryRepository {
        fn find(&self, id: u64) -> Pin<Box<dyn Future<Output = Option<User>> + Send + '_>> {
            Box::pin(async move { self.users.iter().find(|user| user.id == id).cloned() })
        }
    }

    pub async fn find_dyn(repository: &dyn DynUserRepository, id: u64) -> Option<User> {
        repository.find(id).await
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum AsyncTraitShape {
        NativeLocal,
        OpaqueSend,
        BoxedDynamic,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct CostProfile {
        pub runtime_selection: bool,
        pub heap_allocation_per_call: bool,
        pub monomorphized: bool,
        pub future_send_guaranteed: bool,
    }

    // SOLUTION: C41-E04
    pub const fn cost_profile(shape: AsyncTraitShape) -> CostProfile {
        match shape {
            AsyncTraitShape::NativeLocal => CostProfile {
                runtime_selection: false,
                heap_allocation_per_call: false,
                monomorphized: true,
                future_send_guaranteed: false,
            },
            AsyncTraitShape::OpaqueSend => CostProfile {
                runtime_selection: false,
                heap_allocation_per_call: false,
                monomorphized: true,
                future_send_guaranteed: true,
            },
            AsyncTraitShape::BoxedDynamic => CostProfile {
                runtime_selection: true,
                heap_allocation_per_call: true,
                monomorphized: false,
                future_send_guaranteed: true,
            },
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SavePhase {
        BeforeExternalWrite,
        CommitMayHaveHappened,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum DropOutcome {
        NoExternalEffect,
        OutcomeUnknown { recovery: &'static str },
    }

    // SOLUTION: C41-E05
    pub const fn save_drop_outcome(phase: SavePhase) -> DropOutcome {
        match phase {
            SavePhase::BeforeExternalWrite => DropOutcome::NoExternalEffect,
            SavePhase::CommitMayHaveHappened => DropOutcome::OutcomeUnknown {
                recovery: "consultar por idempotency key",
            },
        }
    }

    pub trait SpawnUserRepository: Send + Sync + 'static {
        fn find(&self, id: u64) -> impl Future<Output = Option<User>> + Send;
    }

    impl SpawnUserRepository for MemoryRepository {
        async fn find(&self, id: u64) -> Option<User> {
            self.users.iter().find(|user| user.id == id).cloned()
        }
    }

    // SOLUTION: C41-E06
    pub fn spawn_find<R>(repository: Arc<R>, id: u64) -> tokio::task::JoinHandle<Option<User>>
    where
        R: SpawnUserRepository,
    {
        tokio::spawn(async move { repository.find(id).await })
    }

    pub trait GatUserRepository {
        type Find<'a>: Future<Output = Option<User>> + 'a
        where
            Self: 'a;

        fn find(&self, id: u64) -> Self::Find<'_>;
    }

    impl GatUserRepository for MemoryRepository {
        type Find<'a>
            = Ready<Option<User>>
        where
            Self: 'a;

        fn find(&self, id: u64) -> Self::Find<'_> {
            ready(self.users.iter().find(|user| user.id == id).cloned())
        }
    }

    // SOLUTION: C41-E07
    pub async fn find_with_named_future<R>(repository: &R, id: u64) -> Option<User>
    where
        R: GatUserRepository + Sync,
        for<'a> R::Find<'a>: Send,
    {
        repository.find(id).await
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn native_async_trait_keeps_the_in_memory_repository_unboxed() {
            let mut repository = MemoryRepository::default();
            assert_eq!(NativeUserRepository::find(&repository, 7).await, None);
            assert_eq!(
                NativeUserRepository::save(&mut repository, User { id: 7 }).await,
                Ok(())
            );
            assert_eq!(
                NativeUserRepository::find(&repository, 7).await,
                Some(User { id: 7 })
            );
            assert_eq!(
                NativeUserRepository::save(&mut repository, User { id: 7 }).await,
                Err(SaveError::Duplicate)
            );
        }

        #[tokio::test(flavor = "current_thread")]
        async fn local_and_send_variants_make_different_promises() {
            let local = LocalSequence::default();
            assert_eq!(LocalNext::next(&local).await, 1);
            assert_eq!(LocalNext::next(&local).await, 2);

            let thread_safe = ThreadSafeSequence::default();
            assert_eq!(SendNext::next(&thread_safe).await, 1);
            assert_eq!(SendNext::next(&thread_safe).await, 2);
        }

        #[tokio::test]
        async fn boxed_future_makes_the_trait_dyn_compatible() {
            let repository: Box<dyn DynUserRepository> =
                Box::new(MemoryRepository::new(vec![User { id: 7 }]));
            assert_eq!(find_dyn(repository.as_ref(), 7).await, Some(User { id: 7 }));
        }

        #[test]
        fn dispatch_profiles_keep_costs_and_guarantees_separate() {
            assert_eq!(
                cost_profile(AsyncTraitShape::OpaqueSend),
                CostProfile {
                    runtime_selection: false,
                    heap_allocation_per_call: false,
                    monomorphized: true,
                    future_send_guaranteed: true,
                }
            );
            assert!(cost_profile(AsyncTraitShape::BoxedDynamic).runtime_selection);
            assert!(cost_profile(AsyncTraitShape::BoxedDynamic).heap_allocation_per_call);
        }

        #[test]
        fn cancellation_contract_marks_the_uncertain_commit_window() {
            assert_eq!(
                save_drop_outcome(SavePhase::BeforeExternalWrite),
                DropOutcome::NoExternalEffect
            );
            assert_eq!(
                save_drop_outcome(SavePhase::CommitMayHaveHappened),
                DropOutcome::OutcomeUnknown {
                    recovery: "consultar por idempotency key",
                }
            );
        }

        #[tokio::test]
        async fn send_future_contract_supports_a_multithread_spawn() {
            let repository = Arc::new(MemoryRepository::new(vec![User { id: 8 }]));
            assert_eq!(
                spawn_find(repository, 8).await.unwrap(),
                Some(User { id: 8 })
            );
        }

        #[tokio::test]
        async fn gat_exposes_the_future_family_to_consumer_bounds() {
            fn require_send<F: Future + Send>(future: F) -> F {
                future
            }

            let repository = MemoryRepository::new(vec![User { id: 9 }]);
            let future = require_send(find_with_named_future(&repository, 9));
            assert_eq!(future.await, Some(User { id: 9 }));
        }
    }
}

pub mod c42 {
    use std::fmt::Debug;
    use std::future::Future;
    use std::marker::PhantomData;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Edition {
        Rust2021,
        Rust2024,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum OpaqueContext {
        FreeOrInherent,
        RpititOrTraitImpl,
        AsyncFunction,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum GenericKind {
        TypeOrConst,
        Lifetime,
    }

    // SOLUTION: C42-E01
    pub const fn implicitly_captures(
        edition: Edition,
        context: OpaqueContext,
        generic: GenericKind,
        lifetime_appears_in_opaque_bounds: bool,
    ) -> bool {
        match (edition, context, generic) {
            (_, _, GenericKind::TypeOrConst) => true,
            (_, OpaqueContext::RpititOrTraitImpl, GenericKind::Lifetime)
            | (_, OpaqueContext::AsyncFunction, GenericKind::Lifetime)
            | (Edition::Rust2024, OpaqueContext::FreeOrInherent, GenericKind::Lifetime) => true,
            (Edition::Rust2021, OpaqueContext::FreeOrInherent, GenericKind::Lifetime) => {
                lifetime_appears_in_opaque_bounds
            }
        }
    }

    // SOLUTION: C42-E02
    pub fn independent_length(text: &str) -> impl Copy + Debug + PartialEq<usize> + use<> {
        text.len()
    }

    // SOLUTION: C42-E03
    pub fn named_transform<T>(_: &str, value: T) -> impl Debug + PartialEq<T> + use<T>
    where
        T: Debug + PartialEq<T>,
    {
        value
    }

    // SOLUTION: C42-E04
    // El nombre `'a` es deliberado: el ejercicio audita el capture set `use<'a, T>`.
    #[allow(clippy::needless_lifetimes)]
    pub fn captured_pair<'a, T>(anchor: &'a (), value: T) -> impl Debug + use<'a, T>
    where
        T: Debug,
    {
        (anchor, value)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct MigrationCase {
        pub hidden_type_borrows_input: bool,
        pub callers_require_independence: bool,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum MigrationDecision {
        AcceptImplicitCapture,
        PreserveWithUseBound,
        RedesignContract,
    }

    // SOLUTION: C42-E05
    pub const fn migration_decision(case: MigrationCase) -> MigrationDecision {
        match (
            case.hidden_type_borrows_input,
            case.callers_require_independence,
        ) {
            (true, true) => MigrationDecision::RedesignContract,
            (true, false) | (false, false) => MigrationDecision::AcceptImplicitCapture,
            (false, true) => MigrationDecision::PreserveWithUseBound,
        }
    }

    // SOLUTION: C42-E06
    pub async fn borrowed_async_length(text: &str) -> usize {
        tokio::task::yield_now().await;
        text.len()
    }

    pub fn independent_async_length(text: &str) -> impl Future<Output = usize> + use<> {
        let length = text.len();
        async move {
            tokio::task::yield_now().await;
            length
        }
    }

    pub struct Envelope<T, const N: usize> {
        marker: PhantomData<T>,
    }

    impl<T, const N: usize> Default for Envelope<T, N> {
        fn default() -> Self {
            Self {
                marker: PhantomData,
            }
        }
    }

    impl<T, const N: usize> Envelope<T, N> {
        // SOLUTION: C42-E07
        pub fn bundle<U>(&self, value: U) -> impl Debug + use<T, U, N>
        where
            U: Debug,
        {
            (PhantomData::<T>, [(); N], value)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn capture_matrix_separates_edition_context_and_generic_kind() {
            assert!(implicitly_captures(
                Edition::Rust2021,
                OpaqueContext::FreeOrInherent,
                GenericKind::TypeOrConst,
                false,
            ));
            assert!(!implicitly_captures(
                Edition::Rust2021,
                OpaqueContext::FreeOrInherent,
                GenericKind::Lifetime,
                false,
            ));
            assert!(implicitly_captures(
                Edition::Rust2021,
                OpaqueContext::FreeOrInherent,
                GenericKind::Lifetime,
                true,
            ));
            assert!(implicitly_captures(
                Edition::Rust2024,
                OpaqueContext::FreeOrInherent,
                GenericKind::Lifetime,
                false,
            ));
            assert!(implicitly_captures(
                Edition::Rust2021,
                OpaqueContext::RpititOrTraitImpl,
                GenericKind::Lifetime,
                false,
            ));
            assert!(implicitly_captures(
                Edition::Rust2021,
                OpaqueContext::AsyncFunction,
                GenericKind::Lifetime,
                false,
            ));
        }

        #[test]
        fn use_empty_proves_that_the_return_survives_the_input() {
            let text = String::from("rust");
            let length = independent_length(&text);
            drop(text);
            assert_eq!(length, 4);
        }

        #[test]
        fn named_type_parameter_allows_excluding_only_the_input_lifetime() {
            let context = String::from("temporary context");
            let transformed = named_transform(&context, String::from("owned result"));
            drop(context);
            assert_eq!(transformed, String::from("owned result"));
        }

        #[test]
        fn capture_permission_does_not_add_an_outlives_bound() {
            let anchor = ();
            let short = String::from("short");
            let pair = captured_pair(&anchor, short.as_str());
            assert!(format!("{pair:?}").contains("short"));
        }

        #[test]
        fn migration_audit_exposes_a_real_contract_conflict() {
            assert_eq!(
                migration_decision(MigrationCase {
                    hidden_type_borrows_input: false,
                    callers_require_independence: true,
                }),
                MigrationDecision::PreserveWithUseBound,
            );
            assert_eq!(
                migration_decision(MigrationCase {
                    hidden_type_borrows_input: true,
                    callers_require_independence: false,
                }),
                MigrationDecision::AcceptImplicitCapture,
            );
            assert_eq!(
                migration_decision(MigrationCase {
                    hidden_type_borrows_input: true,
                    callers_require_independence: true,
                }),
                MigrationDecision::RedesignContract,
            );
        }

        #[tokio::test]
        async fn desugared_future_can_publish_real_independence() {
            let borrowed_text = String::from("borrowed");
            assert_eq!(borrowed_async_length(&borrowed_text).await, 8);

            let independent_text = String::from("independent");
            let future = independent_async_length(&independent_text);
            drop(independent_text);
            assert_eq!(future.await, 11);
        }

        #[test]
        fn precise_capture_lists_outer_and_method_generics() {
            let envelope = Envelope::<String, 3>::default();
            let bundle = envelope.bundle("payload");
            let rendered = format!("{bundle:?}");
            assert!(rendered.contains("payload"));
            assert!(rendered.contains("[(), (), ()]"));
        }
    }
}

pub mod c43 {
    use std::collections::BTreeSet;
    use std::future::Future;
    use std::num::NonZeroUsize;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use tokio::sync::Semaphore;
    use tokio::task::JoinSet;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Account {
        balance: u64,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum AccountError {
        ZeroAmount,
        Overflow,
    }

    impl Account {
        pub fn new(balance: u64) -> Self {
            Self { balance }
        }

        pub fn deposit(&mut self, amount: u64) -> Result<u64, AccountError> {
            if amount == 0 {
                return Err(AccountError::ZeroAmount);
            }
            self.balance = self
                .balance
                .checked_add(amount)
                .ok_or(AccountError::Overflow)?;
            Ok(self.balance)
        }

        pub fn balance(&self) -> u64 {
            self.balance
        }
    }

    pub trait AccountRepository {
        type Error;

        fn load(&self) -> impl Future<Output = Result<Account, Self::Error>> + Send;
        fn save(&self, account: Account) -> impl Future<Output = Result<(), Self::Error>> + Send;
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum DepositError<E> {
        Repository(E),
        Domain(AccountError),
    }

    // SOLUTION: C43-E01
    pub async fn deposit<R>(repository: &R, amount: u64) -> Result<u64, DepositError<R::Error>>
    where
        R: AccountRepository + Sync,
    {
        let mut account = repository.load().await.map_err(DepositError::Repository)?;
        let balance = account.deposit(amount).map_err(DepositError::Domain)?;
        repository
            .save(account)
            .await
            .map_err(DepositError::Repository)?;
        Ok(balance)
    }

    #[derive(Default)]
    pub struct MemoryRepository {
        account: tokio::sync::Mutex<Option<Account>>,
    }

    impl MemoryRepository {
        pub fn new(account: Account) -> Self {
            Self {
                account: tokio::sync::Mutex::new(Some(account)),
            }
        }
    }

    impl AccountRepository for MemoryRepository {
        type Error = &'static str;

        async fn load(&self) -> Result<Account, Self::Error> {
            self.account.lock().await.clone().ok_or("missing")
        }

        async fn save(&self, account: Account) -> Result<(), Self::Error> {
            *self.account.lock().await = Some(account);
            Ok(())
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub enum RuntimeNeed {
        MultiThreadExecutor,
        Macros,
        Timers,
        Channels,
        Network,
        Signals,
        PausedClockTests,
    }

    // SOLUTION: C43-E02
    pub fn minimal_tokio_features(needs: &[RuntimeNeed]) -> BTreeSet<&'static str> {
        let mut features = BTreeSet::new();
        for need in needs {
            match need {
                RuntimeNeed::MultiThreadExecutor => {
                    features.insert("rt-multi-thread");
                }
                RuntimeNeed::Macros => {
                    features.insert("macros");
                }
                RuntimeNeed::Timers => {
                    features.insert("time");
                }
                RuntimeNeed::Channels => {
                    features.insert("sync");
                }
                RuntimeNeed::Network => {
                    features.insert("net");
                }
                RuntimeNeed::Signals => {
                    features.insert("signal");
                }
                RuntimeNeed::PausedClockTests => {
                    features.insert("test-util");
                    features.insert("time");
                }
            }
        }
        features
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ShutdownReport {
        pub completed: usize,
        pub failed: usize,
        pub aborted: usize,
        pub clean: bool,
    }

    fn record_join(
        result: Result<(), tokio::task::JoinError>,
        completed: &mut usize,
        failed: &mut usize,
    ) {
        if result.is_ok() {
            *completed += 1;
        } else {
            *failed += 1;
        }
    }

    // SOLUTION: C43-E03
    pub async fn drain_jobs_with_deadline(
        durations: Vec<Duration>,
        deadline: Duration,
    ) -> ShutdownReport {
        let mut tasks = JoinSet::new();
        for duration in durations {
            tasks.spawn(async move {
                tokio::time::sleep(duration).await;
            });
        }

        let mut completed = 0;
        let mut failed = 0;
        let drained = tokio::time::timeout(deadline, async {
            while let Some(result) = tasks.join_next().await {
                record_join(result, &mut completed, &mut failed);
            }
        })
        .await
        .is_ok();

        if drained {
            return ShutdownReport {
                completed,
                failed,
                aborted: 0,
                clean: failed == 0,
            };
        }

        while let Some(result) = tasks.try_join_next() {
            record_join(result, &mut completed, &mut failed);
        }
        let aborted = tasks.len();
        tasks.abort_all();
        while tasks.join_next().await.is_some() {}

        ShutdownReport {
            completed,
            failed,
            aborted,
            clean: false,
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum AppFailure {
        InvalidInput,
        NotFound,
        Conflict,
        TemporarilyUnavailable,
        Internal(&'static str),
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct HttpProblem {
        pub status: u16,
        pub code: &'static str,
        pub message: &'static str,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct IpcProblem {
        pub code: &'static str,
        pub retryable: bool,
        pub message: &'static str,
    }

    // SOLUTION: C43-E04
    pub const fn to_http_problem(failure: AppFailure) -> HttpProblem {
        match failure {
            AppFailure::InvalidInput => HttpProblem {
                status: 422,
                code: "invalid_input",
                message: "la entrada no es válida",
            },
            AppFailure::NotFound => HttpProblem {
                status: 404,
                code: "not_found",
                message: "el recurso no existe",
            },
            AppFailure::Conflict => HttpProblem {
                status: 409,
                code: "conflict",
                message: "la operación entra en conflicto con el estado actual",
            },
            AppFailure::TemporarilyUnavailable => HttpProblem {
                status: 503,
                code: "temporarily_unavailable",
                message: "servicio temporalmente no disponible",
            },
            AppFailure::Internal(_) => HttpProblem {
                status: 500,
                code: "internal",
                message: "error interno",
            },
        }
    }

    pub const fn to_ipc_problem(failure: AppFailure) -> IpcProblem {
        match failure {
            AppFailure::InvalidInput => IpcProblem {
                code: "INVALID_INPUT",
                retryable: false,
                message: "la entrada no es válida",
            },
            AppFailure::NotFound => IpcProblem {
                code: "NOT_FOUND",
                retryable: false,
                message: "el recurso no existe",
            },
            AppFailure::Conflict => IpcProblem {
                code: "CONFLICT",
                retryable: false,
                message: "conflicto de estado",
            },
            AppFailure::TemporarilyUnavailable => IpcProblem {
                code: "TEMPORARILY_UNAVAILABLE",
                retryable: true,
                message: "servicio temporalmente no disponible",
            },
            AppFailure::Internal(_) => IpcProblem {
                code: "INTERNAL",
                retryable: false,
                message: "error interno",
            },
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Layer {
        Domain,
        Application,
        Adapter,
        Lifecycle,
        Binary,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Dependency {
        StdFuture,
        Tokio,
        Axum,
        Tauri,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ArchitecturePolicy {
        pub application_is_runtime_specific: bool,
    }

    // SOLUTION: C43-E05
    pub const fn dependency_allowed(
        layer: Layer,
        dependency: Dependency,
        policy: ArchitecturePolicy,
    ) -> bool {
        match dependency {
            Dependency::StdFuture => !matches!(layer, Layer::Domain),
            Dependency::Tokio => match layer {
                Layer::Domain => false,
                Layer::Application => policy.application_is_runtime_specific,
                Layer::Adapter | Layer::Lifecycle | Layer::Binary => true,
            },
            Dependency::Axum | Dependency::Tauri => {
                matches!(layer, Layer::Adapter | Layer::Binary)
            }
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct BlockingBatch {
        pub word_counts: Vec<usize>,
        pub observed_peak_parallelism: usize,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum BlockingBatchError {
        WorkerPanicked,
    }

    // SOLUTION: C43-E06
    pub async fn parse_word_counts_bounded(
        documents: Vec<String>,
        max_parallel: NonZeroUsize,
    ) -> Result<BlockingBatch, BlockingBatchError> {
        let semaphore = Arc::new(Semaphore::new(max_parallel.get()));
        let active = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::with_capacity(documents.len());

        for (index, document) in documents.into_iter().enumerate() {
            let permit = Arc::clone(&semaphore)
                .acquire_owned()
                .await
                .expect("the owned semaphore is never closed");
            let active = Arc::clone(&active);
            let peak = Arc::clone(&peak);
            handles.push(tokio::task::spawn_blocking(move || {
                let _permit = permit;
                let now_active = active.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(now_active, Ordering::SeqCst);
                let count = document.split_whitespace().count();
                active.fetch_sub(1, Ordering::SeqCst);
                (index, count)
            }));
        }

        let mut indexed = Vec::with_capacity(handles.len());
        for handle in handles {
            indexed.push(
                handle
                    .await
                    .map_err(|_| BlockingBatchError::WorkerPanicked)?,
            );
        }
        indexed.sort_unstable_by_key(|(index, _)| *index);

        Ok(BlockingBatch {
            word_counts: indexed.into_iter().map(|(_, count)| count).collect(),
            observed_peak_parallelism: peak.load(Ordering::SeqCst),
        })
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum RetryOutcome {
        Succeeded,
        Exhausted,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct RetryReport {
        pub outcome: RetryOutcome,
        pub attempts: usize,
        pub elapsed: Duration,
    }

    // SOLUTION: C43-E07
    pub async fn retry_with_schedule(
        failures_before_success: usize,
        schedule: &[Duration],
    ) -> RetryReport {
        let started = tokio::time::Instant::now();
        let mut failures_remaining = failures_before_success;
        let mut attempts = 0;

        loop {
            attempts += 1;
            if failures_remaining == 0 {
                return RetryReport {
                    outcome: RetryOutcome::Succeeded,
                    attempts,
                    elapsed: started.elapsed(),
                };
            }
            failures_remaining -= 1;

            let Some(delay) = schedule.get(attempts - 1) else {
                return RetryReport {
                    outcome: RetryOutcome::Exhausted,
                    attempts,
                    elapsed: started.elapsed(),
                };
            };
            tokio::time::sleep(*delay).await;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn synchronous_entity_is_independent_from_async_application() {
            let mut entity = Account::new(10);
            assert_eq!(entity.deposit(0), Err(AccountError::ZeroAmount));
            assert_eq!(entity.deposit(5), Ok(15));

            let repository = MemoryRepository::new(Account::new(100));
            assert_eq!(deposit(&repository, 25).await, Ok(125));
        }

        #[test]
        fn feature_plan_contains_only_requested_runtime_capabilities() {
            let features = minimal_tokio_features(&[
                RuntimeNeed::MultiThreadExecutor,
                RuntimeNeed::Macros,
                RuntimeNeed::Timers,
                RuntimeNeed::Channels,
            ]);
            assert_eq!(
                features,
                ["macros", "rt-multi-thread", "sync", "time"]
                    .into_iter()
                    .collect(),
            );
            assert!(!features.contains("net"));
            assert!(!features.contains("signal"));

            let test_features = minimal_tokio_features(&[RuntimeNeed::PausedClockTests]);
            assert_eq!(test_features, ["test-util", "time"].into_iter().collect());
        }

        #[tokio::test(start_paused = true)]
        async fn shutdown_drains_until_deadline_then_aborts_the_remainder() {
            let timed_out = drain_jobs_with_deadline(
                vec![Duration::from_millis(1), Duration::from_millis(100)],
                Duration::from_millis(10),
            )
            .await;
            assert_eq!(
                timed_out,
                ShutdownReport {
                    completed: 1,
                    failed: 0,
                    aborted: 1,
                    clean: false,
                },
            );

            let clean = drain_jobs_with_deadline(
                vec![Duration::from_millis(1), Duration::from_millis(2)],
                Duration::from_millis(10),
            )
            .await;
            assert_eq!(
                clean,
                ShutdownReport {
                    completed: 2,
                    failed: 0,
                    aborted: 0,
                    clean: true,
                },
            );
        }

        #[test]
        fn transport_mappings_hide_internal_details() {
            let failure = AppFailure::Internal("database password appeared in driver error");
            assert_eq!(
                to_http_problem(failure),
                HttpProblem {
                    status: 500,
                    code: "internal",
                    message: "error interno",
                },
            );
            assert_eq!(
                to_ipc_problem(failure),
                IpcProblem {
                    code: "INTERNAL",
                    retryable: false,
                    message: "error interno",
                },
            );
            assert!(to_ipc_problem(AppFailure::TemporarilyUnavailable).retryable);
        }

        #[test]
        fn dependency_matrix_keeps_runtime_and_transports_out_of_domain() {
            let neutral = ArchitecturePolicy {
                application_is_runtime_specific: false,
            };
            assert!(!dependency_allowed(
                Layer::Domain,
                Dependency::Tokio,
                neutral
            ));
            assert!(!dependency_allowed(
                Layer::Application,
                Dependency::Tokio,
                neutral,
            ));
            assert!(dependency_allowed(
                Layer::Application,
                Dependency::StdFuture,
                neutral,
            ));
            assert!(dependency_allowed(
                Layer::Adapter,
                Dependency::Axum,
                neutral
            ));
            assert!(!dependency_allowed(
                Layer::Application,
                Dependency::Tauri,
                neutral,
            ));

            let runtime_specific = ArchitecturePolicy {
                application_is_runtime_specific: true,
            };
            assert!(dependency_allowed(
                Layer::Application,
                Dependency::Tokio,
                runtime_specific,
            ));
        }

        #[tokio::test]
        async fn blocking_work_is_bounded_and_results_keep_input_order() {
            let batch = parse_word_counts_bounded(
                vec![
                    String::from("uno dos"),
                    String::from("tres"),
                    String::from("cuatro cinco seis"),
                ],
                NonZeroUsize::new(2).unwrap(),
            )
            .await
            .unwrap();

            assert_eq!(batch.word_counts, [2, 1, 3]);
            assert!((1..=2).contains(&batch.observed_peak_parallelism));
        }

        #[tokio::test(start_paused = true)]
        async fn paused_clock_verifies_backoff_without_wall_clock_sleep() {
            let report =
                retry_with_schedule(2, &[Duration::from_millis(5), Duration::from_millis(10)])
                    .await;
            assert_eq!(
                report,
                RetryReport {
                    outcome: RetryOutcome::Succeeded,
                    attempts: 3,
                    elapsed: Duration::from_millis(15),
                },
            );
        }
    }
}
