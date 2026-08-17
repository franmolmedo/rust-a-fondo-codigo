use std::future::Future;

fn independent_length(text: &str) -> impl Future<Output = usize> + use<> {
    let length = text.len();
    async move { length }
}
