use course_solutions::katas::{k06, k09, k11, k12};
use std::future::Future;
use std::task::{Context, Poll, Waker};

#[test]
fn c56_identifiers_exhaust_without_wrapping_or_overwriting() {
    let mut ids = k06::Sequence::new(u64::MAX);
    let mut repository = k06::MemoryRepository::default();
    let clock = k06::FixedClock(7);
    assert_eq!(k06::create(&mut repository, &clock, &mut ids), Ok(u64::MAX));
    for _ in 0..2 {
        assert_eq!(
            k06::create(&mut repository, &clock, &mut ids),
            Err(k06::CreateError::IdsExhausted)
        );
    }
    assert_eq!(repository.0.len(), 1);
    assert_eq!(repository.0.get(&u64::MAX), Some(&7));
    assert_eq!(
        k06::create_with_id_fn(&mut repository, &clock, &mut || None),
        Err(k06::CreateError::IdsExhausted)
    );
}

#[test]
fn c56_worker_reports_overflow_without_dying_or_changing_state() {
    for limit in [i64::MIN, i64::MAX] {
        for capacity in [0, 1] {
            let worker = k09::Worker::start(capacity);
            worker.apply(limit).unwrap();
            assert_eq!(
                worker.apply(limit.signum()),
                Err(k09::WorkerError::Overflow)
            );
            assert_eq!(worker.snapshot(), Ok(k09::StateSnapshot { value: limit }));
            worker.apply(-limit.signum()).unwrap();
            assert_eq!(
                worker.snapshot(),
                Ok(k09::StateSnapshot {
                    value: limit - limit.signum()
                })
            );
            worker.shutdown().unwrap();
        }
    }
}

#[test]
fn c56_cancelled_permit_wait_releases_its_admission_token() {
    let importer = k11::Importer::new(0, 1);
    let mut work = Box::pin(importer.admit().unwrap().run("pending"));
    let mut context = Context::from_waker(Waker::noop());
    assert_eq!(work.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(importer.active_imports(), 1);
    drop(work);
    assert_eq!(importer.active_imports(), 0);
}

#[test]
fn c56_shutdown_waits_even_for_an_admitted_token_not_yet_started() {
    let importer = k11::Importer::new(1, 1);
    let token = importer.admit().unwrap();
    let mut shutdown = Box::pin(importer.clone().shutdown());
    let mut context = Context::from_waker(Waker::noop());
    assert_eq!(shutdown.as_mut().poll(&mut context), Poll::Pending);
    assert!(matches!(
        importer.admit(),
        Err(k11::ImportError::NotAccepting)
    ));
    drop(token);
    assert_eq!(shutdown.as_mut().poll(&mut context), Poll::Ready(()));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn c56_owned_future_can_outlive_a_non_static_argument() {
    let future = {
        let input = String::from("owned");
        k12::owned_send_future(&input)
    };
    assert_eq!(tokio::spawn(future).await.unwrap(), "owned");
}

#[tokio::test(flavor = "current_thread")]
async fn c56_local_future_owns_its_input_without_becoming_send() {
    let future = {
        let input = String::from("local");
        k12::local_future(&input)
    };
    let local = tokio::task::LocalSet::new();
    assert_eq!(local.run_until(future).await, "local");
}

#[test]
fn c56_exported_macros_ignore_shadowed_prelude_names() {
    #[allow(non_snake_case)]
    fn Err(_: &str) -> Result<(), &'static str> {
        Ok(())
    }
    macro_rules! format {
        ($($tokens:tt)*) => {
            "shadowed".to_owned()
        };
    }
    assert_eq!(Err("test helper"), Ok(()));
    assert_eq!(format!("test helper"), "shadowed");

    let simple = (|| {
        course_solutions::ensure_course!(false, "failure");
        Ok::<_, &'static str>(())
    })();
    assert_eq!(simple, Result::Err("failure"));

    let contextual = (|| {
        course_solutions::ensure_with_context_course!(false, "failure", "context");
        Ok::<_, String>(())
    })();
    assert_eq!(contextual, Result::Err("context: failure".to_owned()));
}
