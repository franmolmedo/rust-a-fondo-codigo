fn assign<'a>(slot: &mut &'a str, value: &'a str) {
    *slot = value;
}

fn main() {
    let mut slot: &'static str = "válido siempre";
    {
        let temporary = String::from("temporal");
        assign(&mut slot, &temporary);
    }
    println!("{slot}");
}
