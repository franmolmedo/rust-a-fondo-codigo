async fn fetch_name(id: UserId) -> Result<String, FetchError> {
    let response = request(id).await?;
    response
        .name()
        .map(str::to_owned)
        .ok_or(FetchError::MissingName)
}
