let fetch = async |id: UserId| -> Result<User, LoadError> {
    load_user(id).await
};

let user = fetch(id).await?;
