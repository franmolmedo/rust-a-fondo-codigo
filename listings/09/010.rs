fn age_group(age: u8) -> &'static str {
    match age {
        0..=12 => "child",
        13..=17 => "teenager",
        18..=64 => "adult",
        65..=u8::MAX => "senior",
    }
}

fn character_group(character: char) -> &'static str {
    match character {
        'a'..='z' => "lowercase ASCII",
        'A'..='Z' => "uppercase ASCII",
        '0'..='9' => "digit ASCII",
        _ => "other",
    }
}

fn main() {
    assert_eq!(age_group(17), "teenager");
    assert_eq!(character_group('ñ'), "other");
}
