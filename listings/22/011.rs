let log = EventLog::new();

let for_button = log.clone();
let on_click = move || for_button.record("click");

on_click();
log.record("shutdown");

assert_eq!(log.snapshot(), vec!["click".to_string(), "shutdown".to_string()]);
