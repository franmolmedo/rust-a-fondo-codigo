use rust_appendix_lab::performance::preview;

fn main() {
    let text = "a€z";
    let limit = 2;
    let broken = std::env::args().any(|argument| argument == "--broken");
    let result = if broken {
        // Intentionally wrong. Run explicitly with --broken to inspect the panic.
        &text[..limit]
    } else {
        preview(text, limit)
    };
    println!("{result}");
}
