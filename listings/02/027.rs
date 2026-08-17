fn label_for(score: u8) -> &'static str {
    let mut label = "suspenso";
    if score >= 50 {
        label = "aprobado";
    }
    label
}

fn main() {
    assert_eq!(label_for(49), "suspenso");
    assert_eq!(label_for(50), "aprobado");
}
