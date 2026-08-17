trait Jobs {
    fn pending(&self) -> impl Iterator<Item = u64> + Send;
}

struct Queue(Vec<u64>);

impl Jobs for Queue {
    fn pending(&self) -> impl Iterator<Item = u64> + Send {
        self.0.clone().into_iter()
    }
}

fn require_send<T: Send>(_value: T) {}

fn main() {
    let queue = Queue(vec![10, 20]);
    require_send(queue.pending());
}
