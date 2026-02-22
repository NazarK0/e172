use e172::{E172App, widget::load_ui_from_file};

fn main() {
    let mut app = E172App::new("RON UI Test");

    // Динамічне завантаження як у QML:
    let root = load_ui_from_file("examples/basic-ron/main.ron");

    app.set_root(root);
    app.run();
}