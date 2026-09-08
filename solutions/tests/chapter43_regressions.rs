use course_solutions::async_rust::c43::*;
use std::num::NonZeroUsize;

#[tokio::test]
async fn c43_stale_saves_do_not_lose_another_deposit() {
    let repository = MemoryRepository::new(Account::new(100));
    let mut first = repository.load().await.unwrap();
    let mut second = repository.load().await.unwrap();
    first.account.deposit(10).unwrap();
    second.account.deposit(20).unwrap();
    repository.save(first).await.unwrap();
    assert_eq!(
        repository.save(second).await,
        Err(MemoryRepositoryError::Conflict)
    );
    assert_eq!(repository.load().await.unwrap().account.balance(), 110);
    assert_eq!(deposit(&repository, 20).await, Ok(130));
}

#[tokio::test]
async fn c43_batch_checks_the_semaphore_limit_and_handles_empty_input() {
    assert_eq!(
        parse_word_counts_bounded(vec![], NonZeroUsize::new(usize::MAX).unwrap()).await,
        Err(BlockingBatchError::ParallelismTooLarge)
    );
    let batch = parse_word_counts_bounded(vec![], NonZeroUsize::new(1).unwrap())
        .await
        .unwrap();
    assert!(batch.word_counts.is_empty());
    assert_eq!(batch.observed_peak_parallelism, 0);
}

#[test]
fn c43_current_thread_executor_has_an_explicit_feature() {
    assert_eq!(
        minimal_tokio_features(&[RuntimeNeed::CurrentThreadExecutor]),
        ["rt"].into_iter().collect()
    );
}
