let shared = Rc::new(5);
let value = *shared; // se extrae aquí: el future captura un i32

let future = async move {
    pause().await;
    value
};
require_send(future);
