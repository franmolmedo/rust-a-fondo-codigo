fn parse_external_port(input: &str) -> u16 {
    input.parse::<u16>().unwrap() // panic ante input recuperable
}

fn main() {
    let _ = parse_external_port("dato externo");
}
