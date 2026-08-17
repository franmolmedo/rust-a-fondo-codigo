#[test]
fn io_failure_is_preserved_as_source() {
    let error = load_config("/ruta/inexistente").unwrap_err();
    assert!(matches!(error, LoadConfigError::Io(_)));
}
