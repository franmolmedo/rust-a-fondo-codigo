fn make_logger(prefix: String) -> impl std::future::Future<Output = ()> {
    async {
        println!("{prefix}: listo");
    }
    // error[E0373]: async block may outlive the current function,
    // but it borrows `prefix`, which is owned by the current function
}
