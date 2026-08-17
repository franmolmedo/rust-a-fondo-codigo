//! Soluciones ejecutables de referencia para *Rust a fondo*.
//!
//! Cada marcador `SOLUTION` corresponde a una ficha del capítulo 59. Las
//! implementaciones buscan enseñar el contrato esencial; no pretenden imponer
//! una única arquitectura válida para los ejercicios abiertos.

pub mod abstraction;
pub mod async_rust;
pub mod compiler;
pub mod concurrency;
pub mod functional;
pub mod fundamentals;
pub mod katas;
pub mod mastery;
pub mod memory;
pub mod organization;
pub mod projects;
pub mod unsafe_low_level;

/// Versión del formato de soluciones, útil para herramientas del libro.
pub const SOLUTIONS_SCHEMA: u32 = 1;

#[cfg(test)]
mod tests {
    use super::SOLUTIONS_SCHEMA;

    #[test]
    fn schema_is_stable() {
        assert_eq!(SOLUTIONS_SCHEMA, 1);
    }
}
