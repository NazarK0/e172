mod rectangle;
pub use rectangle::Rectangle;

use crate::engine::RenderEngine;


pub trait Widget {
    fn init(&mut self, _engine: &RenderEngine) {}
    fn name(&self) -> &str;
    fn layout(&mut self, available_space: (u32, u32));
    fn draw<'a>(&'a self, engine: &'a RenderEngine, render_pass: &mut wgpu::RenderPass<'a>, offset: [f32; 2]);
    fn children(&self) -> &[Box<dyn Widget>];
    fn children_mut(&mut self) -> &mut [Box<dyn Widget>];
}
