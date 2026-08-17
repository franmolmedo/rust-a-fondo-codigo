let text = String::from("rust");
let result = length(&text).await;
drop(text);
assert_eq!(result, 4);
