//! API runtime y reexportaciones del laboratorio de procedural macros.
//!
//! El derive conserva generics y no exige traits de los campos que no usa:
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
//! Un atributo que nombra un campo inexistente se rechaza en compilación:
//!
//! ```compile_fail
//! #[derive(course_macro_api::Entity)]
//! #[entity(id = "missing")]
//! struct Broken {
//!     id: u64,
//! }
//! ```

pub use course_macro_lab::{Entity, field_names, preserve_item};

/// Contrato runtime implementado por el derive del laboratorio.
pub trait Entity: Sized {
    fn entity_name() -> &'static str;
    fn id_field() -> &'static str;
}
