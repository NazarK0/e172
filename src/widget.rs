pub mod rectange;

use crate::engine::{RenderEngine};

pub trait Widget {
    fn init(&mut self, _engine: &RenderEngine) {}
    
    // Потрібен для рекурсивної ініціалізації дерева
    fn name(&self) -> &str;
    fn layout(&mut self, available_space: (u32, u32));
    
    // Тепер ми передаємо RenderPass, щоб віджет міг записувати команди малювання
    fn draw<'a>(&'a self, engine: &'a RenderEngine, render_pass: &mut wgpu::RenderPass<'a>);
    
    fn children(&self) -> &[Box<dyn Widget>];
    fn children_mut(&mut self) -> &mut [Box<dyn Widget>];
}
