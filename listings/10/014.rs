fn parse_external_port(input: &str) -> u16 {
    input.parse::<u16>().unwrap() // panic ante un dato inválido esperable
}

fn main() {
    let _ = parse_external_port("dato externo");
}
