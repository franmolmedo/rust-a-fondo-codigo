fn make_job() -> impl std::future::Future<Output = Result<u64, Error>> {
    async {
        let value = load().await?;
        Ok(value + 1)
    }
}
