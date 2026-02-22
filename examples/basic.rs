use e172::FrameworkApp; // Замініть my_ui_lib на назву вашого пакету з Cargo.toml
use e172::widget::rectange::Rectangle;

fn main() {
    // 1. Створюємо додаток
    let mut app = FrameworkApp::new("UI Framework Test");

    // 2. Створюємо дерево віджетів
    // На даному етапі Rectangle просто заповнить екран, 
    // оскільки ми ще не додали передачу координат у шейдер.
    let rect = Rectangle::new(100.0, 50.0, [1.0, 0.0, 0.0, 1.0]); // 100x50 пікселів!


    // 3. Встановлюємо контент
    app.set_content(rect);

    // 4. Запускаємо
    app.run();
}
