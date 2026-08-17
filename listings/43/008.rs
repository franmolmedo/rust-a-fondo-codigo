use tracing::Instrument;

fn spawn_job(job_id: u64) -> tokio::task::JoinHandle<()> {
    let span = tracing::info_span!("job", job_id);
    tokio::spawn(
        async move {
            tokio::task::yield_now().await;
        }
        .instrument(span),
    )
}
