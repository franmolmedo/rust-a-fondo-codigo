//! Small models, not replacements for the application's integration tests.

#[cfg(test)]
mod tests {
    use loom::sync::Arc;
    use loom::sync::atomic::{AtomicUsize, Ordering};
    use loom::thread;

    fn model_counter(atomic_increment: bool) {
        loom::model(move || {
            let counter = Arc::new(AtomicUsize::new(0));
            let workers: Vec<_> = (0..2)
                .map(|_| {
                    let counter = Arc::clone(&counter);
                    thread::spawn(move || {
                        if atomic_increment {
                            counter.fetch_add(1, Ordering::SeqCst);
                        } else {
                            let previous = counter.load(Ordering::SeqCst);
                            counter.store(previous + 1, Ordering::SeqCst);
                        }
                    })
                })
                .collect();
            for worker in workers {
                worker.join().unwrap();
            }
            assert_eq!(counter.load(Ordering::SeqCst), 2, "lost increment");
        });
    }

    #[test]
    fn atomic_increment_preserves_both_updates() {
        model_counter(true);
    }

    #[test]
    #[should_panic(expected = "lost increment")]
    fn separate_load_store_loses_an_update() {
        model_counter(false);
    }
}
