let mut value = metrics.lock().unwrap_or_else(|mut poisoned| {
    // En este contador de métricas, empezar de nuevo es aceptable.
    **poisoned.get_mut() = 0;
    metrics.clear_poison();
    poisoned.into_inner()
});

*value = value.saturating_add(1);
