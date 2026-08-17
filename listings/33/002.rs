// Capítulo 38: ambos futures avanzan de forma alternada en la misma task.
let (user, permissions) = join!(load_user(id), load_permissions(id));
