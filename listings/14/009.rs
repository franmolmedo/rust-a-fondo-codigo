#[derive(Debug, PartialEq, Eq)]
enum WriteMode {
    CreateNew,
    Replace,
    Append,
}

fn describe(mode: WriteMode) -> &'static str {
    match mode {
        WriteMode::CreateNew => "falla si ya existe",
        WriteMode::Replace => "sustituye el contenido",
        WriteMode::Append => "conserva y añade",
    }
}

fn main() {
    assert_eq!(describe(WriteMode::CreateNew), "falla si ya existe");
    assert_eq!(describe(WriteMode::Replace), "sustituye el contenido");
    assert_eq!(describe(WriteMode::Append), "conserva y añade");
}
