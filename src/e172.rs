use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use crate::engine::RenderEngine;

use crate::widget::Widget;

pub struct E172App {
    engine: Option<RenderEngine>,
    title: String,
    root_widget: Option<Box<dyn Widget>>,
}

impl E172App {
    pub fn new(title: &str) -> Self {
        Self {
            engine: None,
            root_widget: None,
            title: title.to_string(),
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(self).unwrap();
    }

    // Метод для встановлення головного інтерфейсу
    pub fn set_content(&mut self, widget: impl Widget + 'static) {
        self.root_widget = Some(Box::new(widget));
    }

    fn draw_recursive<'a>(
        root: &'a Box<dyn Widget>,
        engine: &'a RenderEngine,
        render_pass: &mut wgpu::RenderPass<'a>,
        offset: [f32; 2], // Додаємо цей параметр
    ) {
        root.draw(engine, render_pass, offset);
    }

    fn init_tree_recursive(widget: &mut Box<dyn Widget>, engine: &RenderEngine) {
        widget.init(engine);
        for child in widget.children_mut() {
            Self::init_tree_recursive(child, engine);
        }
    }
}

impl ApplicationHandler for E172App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes().with_title(&self.title);
        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        
        let engine = pollster::block_on(RenderEngine::new(window));
        
        // АВТОМАТИЧНА ІНІЦІАЛІЗАЦІЯ ТУТ
        if let Some(root) = &mut self.root_widget {
            Self::init_tree_recursive(root, &engine);
        }

        self.engine = Some(engine);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::RedrawRequested => {
                // 1. Спочатку перевіряємо чи є у нас все необхідне, 
                // запозичуючи тільки окремі поля через деструктуризацію
                let (engine, root_widget) = match (&mut self.engine, &self.root_widget) {
                    (Some(e), Some(r)) => (e, r),
                    _ => return,
                };

                let frame = engine.surface.get_current_texture().expect("Failed to acquire texture");
                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = engine.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Main Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.1, b: 0.12, a: 1.0 }),
                                store: wgpu::StoreOp::Store,
                            },
                            depth_slice: None, // В wgpu 0.20 для 2D тут зазвичай None
                        })],
                        ..Default::default()
                    });


                    render_pass.set_pipeline(&engine.render_pipeline);
                    render_pass.set_bind_group(0, &engine.globals_bind_group, &[]); // ОБОВ'ЯЗКОВО
                    render_pass.set_vertex_buffer(0, engine.vertex_buffer.slice(..));

                    if let Some(_root) = &self.root_widget {
                        Self::draw_recursive(root_widget, engine, &mut render_pass, [0.0, 0.0]);
                    }
                }

                engine.queue.submit(std::iter::once(encoder.finish()));
                frame.present();
                engine.window.request_redraw();
            }
            winit::event::WindowEvent::Resized(new_size) => {
                if let Some(engine) = &mut self.engine {
                    engine.resize(new_size.width, new_size.height);
                }
            }
            _ => (),
        }
    }

}
