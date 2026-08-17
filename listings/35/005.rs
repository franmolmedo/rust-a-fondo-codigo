fn make_logger(prefix: String) -> impl std::future::Future<Output = ()> {
    async move {
        println!("{prefix}: listo");
    }
}
