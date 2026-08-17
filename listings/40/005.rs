fn examples() {
    let client = String::from("client");
    let reusable = async move || client.len();
    // `client` pertenece a la closure, pero cada llamada solo lo presta.

    let token = String::from("single-use");
    let consume_once = async move || token;
    // El output mueve `token` fuera: solo `AsyncFnOnce`.

    let _ = (reusable, consume_once);
}
