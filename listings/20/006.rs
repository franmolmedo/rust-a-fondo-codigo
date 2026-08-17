use std::cell::Cell;

struct Viewer<'a> {
    view: &'a str,
}

struct Editor<'a> {
    view: &'a str,
    selected: Cell<&'a str>,
}

fn shorten_viewer<'short>(viewer: Viewer<'static>) -> Viewer<'short> {
    viewer
}

fn main() {
    let viewer = Viewer { view: "documento" };
    let shorter = shorten_viewer(viewer);
    assert_eq!(shorter.view, "documento");

    let editor = Editor { view: "documento", selected: Cell::new("doc") };
    assert_eq!(editor.selected.get(), "doc");
    assert_eq!(editor.view, "documento");
}
