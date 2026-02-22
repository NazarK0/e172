mod rectangle;
mod loader;
pub use rectangle::Rectangle;
pub use loader::load_ui_from_file;
use serde::Deserialize;

use crate::engine::RenderEngine;

#[derive(Deserialize, Debug)]
pub enum WidgetType {
    Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: [f32; 4],
        #[serde(default)]
        children: Vec<WidgetType>,
    },
    // Тут у майбутньому додасте Button, Text тощо
}


pub trait Widget {
    fn init(&mut self, _engine: &RenderEngine) {}
    fn name(&self) -> &str;
    fn layout(&mut self, available_space: (u32, u32));
    fn draw<'a>(&'a self, engine: &'a RenderEngine, render_pass: &mut wgpu::RenderPass<'a>, offset: [f32; 2]);
    fn children(&self) -> &[Box<dyn Widget>];
    fn children_mut(&mut self) -> &mut [Box<dyn Widget>];
}
