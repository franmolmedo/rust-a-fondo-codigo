fn main() {
    let status_code = 404;
    let description = match status_code {
        200 => "ok",
        404 => "not found",
        _ => "other",
    };
    assert_eq!(description, "not found");
}
