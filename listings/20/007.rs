use std::cell::Cell;

struct Editor<'a> {
    selected: Cell<&'a str>,
}

fn shorten_editor<'short>(editor: Editor<'static>) -> Editor<'short> {
    editor
}

fn main() {}
