trait Sink {
    fn send(&mut self, kind: &str, value: String);
}

#[derive(Default)]
struct MemorySink {
    values: Vec<String>,
}

impl Sink for MemorySink {
    fn send(&mut self, _kind: &str, value: String) {
        self.values.push(value);
    }
}

struct FilteredSink<S, P> {
    inner: S,
    accepts: P,
}

impl<S, P> Sink for FilteredSink<S, P>
where
    S: Sink,
    P: Fn(&str) -> bool,
{
    fn send(&mut self, kind: &str, value: String) {
        if (self.accepts)(kind) {
            self.inner.send(kind, value);
        }
    }
}

fn main() {
    let memory = MemorySink::default();
    let mut sink = FilteredSink { inner: memory, accepts: |kind: &str| kind == "audit" };
    sink.send("debug", String::from("ignorado"));
    sink.send("audit", String::from("guardado"));
    assert_eq!(sink.inner.values, ["guardado"]);
}
