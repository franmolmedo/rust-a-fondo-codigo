#[derive(course_macro_api::Entity)]
#[entity(id = "user_id")]
struct User {
    user_id: u64,
    name: String,
}
