let mut value = match balance.lock() {
    Ok(guard) => guard,
    Err(poisoned) => {
        // Decisión consciente: sabemos verificar/restaurar la invariante.
        poisoned.into_inner()
    }
};
*value = 0;
