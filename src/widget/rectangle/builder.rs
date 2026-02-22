use crate::widget::{Rectangle, Widget};

#[derive(Default)]
pub struct RectangleBuilder {
    x: f32, y: f32, w: f32, h: f32,
    color: [f32; 4],
    children: Vec<Box<dyn Widget>>,
}

impl RectangleBuilder {
    pub fn x(mut self, x: f32) -> Self { self.x = x; self }
    pub fn y(mut self, y: f32) -> Self { self.y = y; self }
    pub fn width(mut self, w: f32) -> Self { self.w = w; self }
    pub fn height(mut self, h: f32) -> Self { self.h = h; self }
    pub fn color(mut self, c: [f32; 4]) -> Self { self.color = c; self }
    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }
    pub fn build(self) -> Rectangle {
        let mut r = Rectangle::new(self.w, self.h, self.color);
        r.x = self.x; r.y = self.y;
        r.children = self.children;
        r
    }
}