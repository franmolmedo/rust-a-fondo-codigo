impl ClientBuilder {
    #[must_use]
    pub fn with_retries(mut self, count: u32) -> Self {
        self.retries = count;
        self
    }
}
