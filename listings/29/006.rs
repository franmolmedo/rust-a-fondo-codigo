struct ClientBuilder {
    retries: u32,
}

impl ClientBuilder {
    #[must_use]
    pub fn with_retries(mut self, count: u32) -> Self {
        self.retries = count;
        self
    }
}

fn main() {
    let builder = ClientBuilder { retries: 0 };
    let builder = builder.with_retries(3);
    assert_eq!(builder.retries, 3);
}
