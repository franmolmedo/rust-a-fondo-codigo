let mut values = Vec::new();
let mut push_later = async || {
    let value = load_value().await;
    values.push(value);
};

push_later().await;
