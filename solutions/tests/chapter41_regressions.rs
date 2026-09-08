mod book_example {
    type UserId = u64;
    type RepoError = &'static str;

    #[derive(Debug, PartialEq)]
    struct User(UserId);

    include!("../../listings/41/003.rs");
    include!("../../listings/41/006.rs");

    struct Repository;

    impl SendUserRepository for Repository {
        async fn find(&self, id: UserId) -> Result<Option<User>, RepoError> {
            if id == 0 {
                Err("unavailable")
            } else {
                Ok(Some(User(id)))
            }
        }
    }

    #[tokio::test]
    async fn c41_book_spawn_preserves_the_repository_result() {
        let repository = std::sync::Arc::new(Repository);
        assert_eq!(
            spawn_find(repository.clone(), 7).await.unwrap(),
            Ok(Some(User(7)))
        );
        assert_eq!(spawn_find(repository, 0).await.unwrap(), Err("unavailable"));
    }
}
