unsafe fn forged_ref<'a, T>(pointer: *const T) -> &'a T {
    // SAFETY: esta línea solo sería correcta si el contrato externo demostrase
    // todas las premisas de referencia, incluido el `'a` elegido.
    unsafe { &*pointer }
}
