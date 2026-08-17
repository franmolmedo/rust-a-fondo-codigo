use std::sync::Arc;

let shared: Arc<String> = Arc::new(String::from("rust"));
assert_eq!(shared.len(), 4);   // String::len, a través del Arc

let view: &str = &shared;      // Arc<String> -> &String -> &str, dos pasos de coerción
assert_eq!(view, "rust");
