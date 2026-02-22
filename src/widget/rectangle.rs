mod builder;

use crate::{engine::RenderEngine, shaders::rect_uniform::RectUniform, widget::Widget};
use wgpu::util::DeviceExt;
use builder::RectangleBuilder;


pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4],
    pub children: Vec<Box<dyn Widget>>,
    // private params
    bind_group: Option<wgpu::BindGroup>,
    uniform_buffer: Option<wgpu::Buffer>,
}

impl Rectangle {
    pub fn new(width: f32, height: f32, color: [f32; 4]) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width,
            height,
            color,
            children: Vec::new(),
            bind_group: None,
            uniform_buffer: None,
        }
    }

    pub fn builder() -> RectangleBuilder {
        RectangleBuilder::default()
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn init_gpu(&mut self, engine: &RenderEngine) {
        let uniform_data = RectUniform {
            position: [self.x, self.y],
            size: [self.width, self.height],
            color: self.color,
        };

        let buffer = engine.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rectangle Buffer"),
            contents: bytemuck::cast_slice(&[uniform_data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, // COPY_DST дозволяє оновлення
        });

        let bind_group = engine.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &engine.rect_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        });

        self.uniform_buffer = Some(buffer);
        self.bind_group = Some(bind_group);
    }

    pub fn sync_with_gpu(&self, engine: &RenderEngine, global_offset: [f32; 2]) {
        if let Some(buffer) = &self.uniform_buffer {
            let data = RectUniform {
                position: [self.x + global_offset[0], self.y + global_offset[1]],
                size: [self.width, self.height],
                color: self.color,
            };

            engine.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[data]));
        }
    }
    
    pub fn add_child(&mut self, child: impl Widget + 'static) {
        self.children.push(Box::new(child));
    }
}

impl Widget for Rectangle {
    fn init(&mut self, engine: &RenderEngine) {
        self.init_gpu(engine);
    }

    

    fn name(&self) -> &str { "Rectangle" }

    fn layout(&mut self, _space: (u32, u32)) {
        for child in &mut self.children {
            // TODO: Realize layout logic here
            child.layout(_space);
        }
    }

    fn draw<'a>(&'a self, engine: &'a RenderEngine, render_pass: &mut wgpu::RenderPass<'a>, offset: [f32; 2]) {
        // Update GPU data before drawing
        self.sync_with_gpu(engine, offset);

        if let Some(bind_group) = &self.bind_group {
            render_pass.set_bind_group(1, bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        }

        let my_global_pos = [offset[0] + self.x, offset[1] + self.y];
        for child in &self.children {
            child.draw(engine, render_pass, my_global_pos);
        }
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut self.children
    }

    fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }
}
