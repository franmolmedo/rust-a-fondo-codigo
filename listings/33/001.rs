async fn load_profile(id: UserId) -> Result<Profile, LoadError> {
    let user = load_user(id).await?;
    let permissions = load_permissions(id).await?;
    Ok(Profile { user, permissions })
}
