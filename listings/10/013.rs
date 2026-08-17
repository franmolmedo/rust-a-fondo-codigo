fn main() {
    let port = "8080"
        .parse::<u16>()
        .expect("el literal 8080 está controlado por el programa");

    assert_eq!(port, 8080);
}
