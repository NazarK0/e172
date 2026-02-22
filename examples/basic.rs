use e172::{E172App, rui};

fn main() {
    // 1. Створюємо додаток
    let mut app = E172App::new("UI Framework Test");

    // 2. Створюємо дерево віджетів
    let ui = Box::new(rui! {
        Rectangle {
            color: [0.1, 0.1, 0.1, 1.0],
            width: 800.0,
            height: 600.0,
            children: [
                Rectangle {
                    y: 50.0,
                    x: 50.0,
                    width: 100.0,
                    height: 100.0,
                    color: [1.0, 0.0, 0.0, 1.0],
                },
                Rectangle {
                    color: [0.0, 1.0, 0.0, 1.0],
                    x: 200.0,
                    width: 50.0,
                    height: 50.0,
                    y: 50.0,
                }
            ]
        }
    });

    app.set_root(ui);
    app.run();
}