trait CurrentApi {
    fn metadata<'a>(&'a self) -> impl Sized + use<Self>;
}
