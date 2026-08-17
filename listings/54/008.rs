trait UserStore {
    type Error;

    fn insert_unique(
        &mut self,
        user: User,
    ) -> Result<InsertOutcome, Self::Error>;
}
