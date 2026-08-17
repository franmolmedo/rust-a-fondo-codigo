use std::{future::Future, pin::Pin};

trait DynUserRepository: Send + Sync {
    fn find<'a>(
        &'a self,
        id: UserId,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Option<User>, RepoError>>
                + Send
                + 'a,
        >,
    >;
}
