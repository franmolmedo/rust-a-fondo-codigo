use std::marker::PhantomData;

struct Disconnected;
struct Connected;
struct Client<State>(PhantomData<State>);

impl Client<Disconnected> {
    fn new() -> Self { Self(PhantomData) }
}

impl Client<Connected> {
    fn send(&self) {}
}

fn main() {
    Client::<Disconnected>::new().send();
    // no existe send para Client<Disconnected>
}
