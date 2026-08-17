let text = "é";
let whole = &text[0..2];
assert_eq!(whole, "é");
assert_eq!(text.get(0..1), None);
