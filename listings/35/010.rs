fn make_job() -> impl std::future::Future<Output = Result<u64, Error>> {
    async {
        let value = load().await?;
        value.checked_add(1).ok_or(Error::Overflow)
    }
}
