// Misma task: concurrencia sin nueva unidad de fallo ni bounds extra.
let (a, b) = join!(load_user(id), load_permissions(id));

// Task nueva: unidad independiente, con handle, errores propios,
// requisitos Send + 'static y responsabilidad de supervisión.
let handle = spawn(refresh_cache(id));
