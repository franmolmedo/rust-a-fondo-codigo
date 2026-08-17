trait Repository {
    async fn find(&self, id: u64) -> Option<u64>;
}

fn choose_at_runtime(repository: &dyn Repository) {
    let _ = repository;
}
