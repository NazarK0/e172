use crate::engine::VelloEngine;
use crate::engine::vello_engine::RenderParams as EngineRenderParams;
use crate::{Dimensions, Screen};
use peniko::Color;
use std::sync::Arc;
use vello::{RenderParams as VelloRenderParams, Renderer, RendererOptions, Scene};
use winit::{event_loop::ActiveEventLoop, window::Window};

pub struct RenderState {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    renderer: Renderer,
    config: wgpu::SurfaceConfiguration,
    render_texture: wgpu::Texture,
    blitter: wgpu::util::TextureBlitter,
    screens: Vec<Screen<VelloEngine>>,
}

impl RenderState {
    pub fn init(event_loop: &ActiveEventLoop, instance: wgpu::Instance) -> Self {
        let window_attrs = Window::default_attributes().with_title("Vello Winit 0.30");
        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        let (device, queue) =
            pollster::block_on(adapter.request_device(&Default::default())).unwrap();

        let size = window.inner_size();
        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        config.format = wgpu::TextureFormat::Rgba8UnormSrgb;
        surface.configure(&device, &config);

        let renderer = Renderer::new(
            &device,
            RendererOptions {
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
                // On MacOS, we need to use a single thread to avoid issues with the Metal API
                num_init_threads: None,
                pipeline_cache: None,
            },
        )
        .unwrap();

        // Create intermediate render texture with Rgba8Unorm format
        let render_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("render_texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::STORAGE_BINDING,
            view_formats: &[],
        });

        // Create TextureBlitter for copying from intermediate texture to surface
        let blitter = wgpu::util::TextureBlitter::new(&device, config.format);

        Self {
            window,
            surface,
            device,
            queue,
            renderer,
            config,
            render_texture,
            blitter,
            screens: Vec::new(),
        }
    }
    pub fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn get_render_texture(&self) -> &wgpu::Texture {
        &self.render_texture
    }

    pub fn add_screen(&mut self, screen: Screen<VelloEngine>) {
        self.screens.push(screen);
    }

    pub fn get_window_dim(&self) -> Dimensions {
        Dimensions::new(self.config.width, self.config.height)
    }

    pub fn set_window_dim(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);

        // Recreate render texture at new size
        self.render_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("render_texture"),
            size: wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::STORAGE_BINDING,
            view_formats: &[],
        });
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn get_current_texture(&mut self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }

    pub fn render_screens(&mut self, render_params: &EngineRenderParams) {
        use crate::core::TScreen;
        use crate::engine::vello_engine::VelloRenderBackend;

        for screen in self.screens.iter_mut() {
            let mut scene = Scene::new();

            VelloRenderBackend::with_scene(&mut scene, || {
                screen.render();
            });

            let vello_params = VelloRenderParams {
                base_color: Color::WHITE,
                width: screen.dimensions().width(),
                height: screen.dimensions().height(),
                antialiasing_method: render_params.antialiasing_method,
            };

            let texture_view = self.render_texture.create_view(&Default::default());
            let _ = self.renderer.render_to_texture(
                &self.device,
                &self.queue,
                &scene,
                &texture_view,
                &vello_params,
            );
        }
    }

    pub fn get_blitter(&self) -> &wgpu::util::TextureBlitter {
        &self.blitter
    }
}
