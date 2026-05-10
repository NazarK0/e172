use kurbo::Point;
use peniko::Brush;

#[derive(Clone)]
pub struct Text {
    pub id: String,
    pub content: String,
    pub position: Point,
    pub font_size: f32,
    pub background: Brush, // Можна використовувати для підкладки
    pub fontcolor: Brush,
}

impl Text {
    pub fn new(
        id: impl Into<String>,
        content: impl Into<String>,
        position: Point,
        font_size: f32,
        background: Brush,
        fontcolor: Brush,
    ) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            position,
            font_size,
            background,
            fontcolor,
        }
    }
}
