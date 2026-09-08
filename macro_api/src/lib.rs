//! Runtime API and re-exports for the procedural macro lab.
//!
//! The derive preserves generics and adds no trait bounds for unused fields:
//!
//! ```
//! #[derive(course_macro_api::Entity)]
//! #[entity(id = "id")]
//! struct User<'a, T, const N: usize> {
//!     id: u64,
//!     values: &'a [T; N],
//! }
//!
//! assert_eq!(
//!     <User<'_, String, 1> as course_macro_api::Entity>::id_field(),
//!     "id",
//! );
//! ```
//!
//! An attribute that names a missing field is rejected at compile time:
//!
//! ```compile_fail
//! #[derive(course_macro_api::Entity)]
//! #[entity(id = "missing")]
//! struct Broken {
//!     id: u64,
//! }
//! ```
//!
//! Duplicate options and invalid macro arguments are also rejected:
//!
//! ```compile_fail
//! #[derive(course_macro_api::Entity)]
//! #[entity(id = "id", id = "id")]
//! struct Duplicate { id: u64 }
//! ```
//!
//! ```compile_fail
//! #[course_macro_api::preserve_item(unexpected)]
//! fn answer() -> u32 { 42 }
//! ```
//!
//! ```compile_fail
//! const FIELDS: &[&str] = course_macro_api::field_names!("id");
//! ```

pub use course_macro_lab::{Entity, field_names, preserve_item};

/// Runtime contract implemented by the lab derive.
pub trait Entity: Sized {
    fn entity_name() -> &'static str;
    fn id_field() -> &'static str;
}
