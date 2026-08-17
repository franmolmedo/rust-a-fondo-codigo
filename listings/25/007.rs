use std::collections::HashMap;            // tipos: importados directamente
use std::fmt::{self, Display, Formatter}; // `self` trae también el módulo
use crate::domain::order as orders;       // renombrar rutas largas o en conflicto

fn describe(map: &HashMap<String, u64>) -> fmt::Result {
    // funciones ajenas suelen llamarse cualificadas: fmt::format, cmp::min...
    Ok(())
}
