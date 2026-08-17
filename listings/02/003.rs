fn main() {
    let score = 70;

    let label;
    if score >= 50 {
        label = "aprobado";
    } else {
        label = "suspenso";
    }
    println!("{label}");
}
