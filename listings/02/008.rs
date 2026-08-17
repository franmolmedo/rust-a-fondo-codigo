fn main() {
    let score = 70;
    let label = if score >= 50 {
        "aprobado"
    } else {
        "suspenso"
    };
    assert_eq!(label, "aprobado");
}
