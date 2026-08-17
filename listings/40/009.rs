use std::{future::Future, pin::Pin};

trait DynCallback {
    fn call<'a>(
        &'a mut self,
        input: &'a str,
    ) -> Pin<Box<dyn Future<Output = usize> + 'a>>;
}
