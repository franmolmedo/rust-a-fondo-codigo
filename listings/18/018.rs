use std::future::Future;

async fn compute(input: u32) -> u64 {
    u64::from(input) * 2
}

fn compute_desugared(input: u32) -> impl Future<Output = u64> {
    async move { u64::from(input) * 2 }
}

fn accepts_future(_future: impl Future<Output = u64>) {}

fn main() {
    accepts_future(compute(21));
    accepts_future(compute_desugared(21));
}
