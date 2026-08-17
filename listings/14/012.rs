use std::marker::PhantomData;

#[derive(Debug)]
struct Disconnected;

#[derive(Debug)]
struct Connected;

#[derive(Debug)]
struct Client<State> {
    endpoint: String,
    state: PhantomData<State>,
}

impl Client<Disconnected> {
    fn new(endpoint: impl Into<String>) -> Self {
        Self { endpoint: endpoint.into(), state: PhantomData }
    }

    fn connect(self) -> Client<Connected> {
        Client { endpoint: self.endpoint, state: PhantomData }
    }
}

impl Client<Connected> {
    fn send(&self, payload: &[u8]) -> usize {
        payload.len()
    }
}

impl<State> Client<State> {
    fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

fn main() {
    let client = Client::<Disconnected>::new("local");
    assert_eq!(client.endpoint(), "local");
    let client = client.connect();
    assert_eq!(client.send(b"ping"), 4);
}
