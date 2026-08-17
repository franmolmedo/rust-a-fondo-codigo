trait Encode {
    fn encode(&self) -> Vec<u8>;
}

impl Encode for u32 {
    fn encode(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

trait Store {
    fn save_bytes(&mut self, key: &str, bytes: &[u8]);
}

#[derive(Default)]
struct MemoryStore {
    last: Option<(String, Vec<u8>)>,
}

impl Store for MemoryStore {
    fn save_bytes(&mut self, key: &str, bytes: &[u8]) {
        self.last = Some((key.to_owned(), bytes.to_vec()));
    }
}

fn save<S, T>(store: &mut S, key: &str, value: &T)
where
    S: Store + ?Sized,
    T: Encode,
{
    store.save_bytes(key, &value.encode());
}

fn main() {
    let mut memory = MemoryStore::default();
    let erased: &mut dyn Store = &mut memory;
    save(erased, "answer", &42_u32);
    assert_eq!(memory.last, Some((String::from("answer"), b"42".to_vec())));
}
