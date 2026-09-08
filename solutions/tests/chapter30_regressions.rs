use course_solutions::concurrency::c30::{
    CounterWorker, ReceiveError, SumError, Worker, receive_with_timeout, scoped_sum,
};
use std::sync::mpsc;
use std::time::Duration;

mod book_counter {
    include!("../../listings/30/010.rs");

    #[test]
    fn c30_the_book_counter_caps_its_metric_in_every_profile() {
        let (commands, worker) = spawn_counter();
        commands.send(Command::Add(u64::MAX)).unwrap();
        commands.send(Command::Add(1)).unwrap();
        let (reply, answer) = mpsc::channel();
        commands.send(Command::Total(reply)).unwrap();
        assert_eq!(answer.recv().unwrap(), u64::MAX);
        drop(commands);
        assert_eq!(worker.join().unwrap(), u64::MAX);
    }

    #[test]
    fn c30_the_book_counter_drains_commands_after_an_abandoned_query() {
        let (commands, worker) = spawn_counter();
        let (reply, answer) = mpsc::channel();
        drop(answer);
        commands.send(Command::Total(reply)).unwrap();
        commands.send(Command::Add(7)).unwrap();
        drop(commands);
        assert_eq!(worker.join().unwrap(), 7);
    }
}

#[test]
fn c30_scoped_sums_handle_empty_inputs_and_extra_workers() {
    assert_eq!(scoped_sum(&[], 1), Ok(0));
    assert_eq!(scoped_sum(&[7], 8), Ok(7));
    assert_eq!(scoped_sum(&[u64::MAX, 0], usize::MAX), Ok(u64::MAX));
    assert_eq!(scoped_sum(&[1, 2, 3, 4, 5], 2), Ok(15));
}

#[test]
fn c30_zero_workers_is_a_reported_configuration_error() {
    assert_eq!(scoped_sum(&[], 0), Err(SumError::NoWorkers));
    assert_eq!(scoped_sum(&[1], 0), Err(SumError::NoWorkers));
}

#[test]
fn c30_overflow_is_checked_within_chunks_and_when_combining_them() {
    for workers in [1, 2, 4] {
        assert_eq!(scoped_sum(&[u64::MAX, 1], workers), Err(SumError::Overflow));
    }
}

#[test]
fn c30_flush_and_shutdown_keep_every_batch_in_order() {
    for capacity in [0, 1, 4] {
        let worker = Worker::start(capacity);
        assert_eq!(worker.flush(), Ok(0));
        worker.store("first").unwrap();
        assert_eq!(worker.flush(), Ok(1));
        assert_eq!(worker.flush(), Ok(1));
        worker.store("second").unwrap();
        worker.store("third").unwrap();
        assert_eq!(worker.shutdown().unwrap(), ["first", "second", "third"]);
    }
}

#[test]
fn c30_metric_saturation_does_not_prevent_a_later_reset() {
    let worker = CounterWorker::start(1);
    worker.add(u64::MAX).unwrap();
    worker.add(1).unwrap();
    assert_eq!(worker.reset(), Ok(u64::MAX));
    worker.add(1).unwrap();
    assert_eq!(worker.reset(), Ok(1));
    worker.shutdown().unwrap();
}

#[test]
fn c30_replies_stay_with_their_requests_when_read_in_reverse_order() {
    let worker = CounterWorker::start(1);
    worker.add(7).unwrap();
    let first = worker.request_reset().unwrap();
    worker.add(11).unwrap();
    let second = worker.request_reset().unwrap();
    assert_eq!(second.recv().unwrap(), 11);
    assert_eq!(first.recv().unwrap(), 7);
    worker.shutdown().unwrap();
}

#[test]
fn c30_abandoning_a_reply_does_not_cancel_the_reset() {
    let worker = CounterWorker::start(1);
    worker.add(7).unwrap();
    drop(worker.request_reset().unwrap());
    worker.add(3).unwrap();
    assert_eq!(worker.reset(), Ok(3));
    worker.shutdown().unwrap();
}

#[test]
fn c30_a_queued_reply_remains_available_after_the_worker_exits() {
    let worker = CounterWorker::start(1);
    worker.add(42).unwrap();
    let reply = worker.request_reset().unwrap();
    worker.shutdown().unwrap();
    assert_eq!(receive_with_timeout(&reply, Duration::ZERO), Ok(42));
    assert_eq!(
        receive_with_timeout(&reply, Duration::ZERO),
        Err(ReceiveError::Disconnected)
    );
}

#[test]
fn c30_a_timeout_does_not_close_the_reply_channel() {
    let (sender, receiver) = mpsc::channel();
    assert_eq!(
        receive_with_timeout(&receiver, Duration::ZERO),
        Err(ReceiveError::TimedOut)
    );
    sender.send(42).unwrap();
    assert_eq!(receive_with_timeout(&receiver, Duration::ZERO), Ok(42));
}

#[test]
fn c30_failed_sends_return_the_unsent_value() {
    let (sender, receiver) = mpsc::sync_channel(1);
    sender.send(String::from("first")).unwrap();
    assert_eq!(
        sender.try_send(String::from("second")),
        Err(mpsc::TrySendError::Full(String::from("second")))
    );
    drop(receiver);
    assert_eq!(
        sender.send(String::from("third")),
        Err(mpsc::SendError(String::from("third")))
    );
}
