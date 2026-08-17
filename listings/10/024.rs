#[derive(Debug, PartialEq)]
struct MissingUser {
    detail: String,
}

fn require_user(user: Option<&str>, id: u64) -> Result<&str, MissingUser> {
    user.ok_or_else(|| MissingUser {
        detail: format!("usuario {id} no encontrado"),
    })
}

fn main() {
    assert_eq!(require_user(Some("Ada"), 7), Ok("Ada"));
    assert_eq!(
        require_user(None, 7),
        Err(MissingUser {
            detail: String::from("usuario 7 no encontrado"),
        })
    );
}
