trait GatUserRepository {
    type Find<'a>: std::future::Future<Output = Option<User>> + 'a
    where
        Self: 'a;

    fn find(&self, id: u64) -> Self::Find<'_>;
}
