use std::sync::RwLock;

let config = RwLock::new(Config::default());
let port = config.read().unwrap().port;
config.write().unwrap().reload()?;
