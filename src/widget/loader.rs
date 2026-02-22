use crate::widget::rectangle::{Rectangle};
use crate::widget::{Widget, WidgetType};


pub fn load_ui_from_file(path: &str) -> Box<dyn Widget> {
    let content = std::fs::read_to_string(path).expect("Failed to read UI file");
    let root_data: WidgetType = ron::from_str(&content).expect("Failed to parse RON");

    build_widget(root_data)
}

fn build_widget(data: WidgetType) -> Box<dyn Widget> {
    match data {
        WidgetType::Rectangle { x, y, width, height, color, children } => {
            let mut rect = Rectangle::new(width, height, color);
            rect.set_x(x);
            rect.set_y(y);
            
            for child_data in children {
                rect.add_child(build_widget(child_data));
            }
            Box::new(rect)
        }
    }
}
