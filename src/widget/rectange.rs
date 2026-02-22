use crate::{engine::{RenderEngine}, widget::Widget};
use wgpu::util::DeviceExt;

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4],
    pub children: Vec<Box<dyn Widget>>,
    // Ми робимо це поле внутрішнім
    bind_group: Option<wgpu::BindGroup>, 
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
        }
    }

    

    // Метод для встановлення позиції (зручно для користувача)
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn init_gpu(&mut self, engine: &RenderEngine) {
        let uniform_data = crate::shaders::rect_uniform::RectUniform {
            position: [self.x, self.y],    // Переконайтеся, що назви збігаються з вашим RectUniform
            size: [self.width, self.height],
            color: self.color,
        };

        let buffer = engine.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rectangle Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform_data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // ВИПРАВЛЕНО: назва методу create_bind_group
        let bind_group = engine.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &engine.rect_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Rectangle Bind Group"),
        });

        self.bind_group = Some(bind_group);
        
        // Ініціалізуємо дітей (якщо вони теж потребують GPU ресурсів)
        // Але наразі ми фокусуємося на поточному об'єкті
    }

    pub fn add_child(&mut self, child: impl Widget + 'static) {
        self.children.push(Box::new(child));
    }
}

impl Widget for Rectangle {
    fn init(&mut self, engine: &RenderEngine) {
        // Викликаємо ваш існуючий метод для GPU ресурсів
        self.init_gpu(engine);
    }

    

    fn name(&self) -> &str { "Rectangle" }

    fn layout(&mut self, _space: (u32, u32)) {
        for child in &mut self.children {
            // Тут буде логіка відносного позиціонування
        }
    }

    fn draw<'a>(&'a self, engine: &'a RenderEngine, render_pass: &mut wgpu::RenderPass<'a>) {
        if let Some(bind_group) = &self.bind_group {
            render_pass.set_bind_group(1, bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        }
        
        // ВАЖЛИВО: Рекурсивне малювання дітей тепер контролюється у FrameworkApp::draw_recursive,
        // тому тут ми просто малюємо самого себе.
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut self.children
    }

    fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }
}
