use std::future::Future;

async fn compute(input: u32) -> u32 {
    input * 2
}

fn compute_desugared(input: u32) -> impl Future<Output = u32> {
    async move { input * 2 }
}

fn accepts_future(_future: impl Future<Output = u32>) {}

fn main() {
    accepts_future(compute(21));
    accepts_future(compute_desugared(21));
}
