trait Repository {
    type Entity;
    type Error;
    fn find(&self, id: u64) -> Result<Option<Self::Entity>, Self::Error>;
}

#[derive(Debug, PartialEq, Eq)]
enum RequireError<E> {
    Missing { id: u64 },
    Backend(E),
}

fn require<R>(repository: &R, id: u64) -> Result<R::Entity, RequireError<R::Error>>
where
    R: Repository,
{
    repository
        .find(id)
        .map_err(RequireError::Backend)?
        .ok_or(RequireError::Missing { id })
}
