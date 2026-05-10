use crate::core::TE172Engine;
use crate::{Dimensions, Screen, UiComponentId};
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

mod render_state;
use render_state::RenderState;

mod render_backend;
pub use render_backend::VelloRenderBackend;

pub mod config;
pub use config::VelloEngineConfig;

mod render_params;
pub use render_params::RenderParams;

mod screen;

pub struct VelloEngine {
    instance: wgpu::Instance,
    state: Option<Arc<RefCell<RenderState>>>,
    pending_screens: Vec<Screen<VelloEngine>>,
    componet_ids: HashSet<UiComponentId>,
    config: VelloEngineConfig,
}

impl Clone for VelloEngine {
    fn clone(&self) -> Self {
        Self {
            instance: self.instance.clone(),
            state: self.state.clone(),
            pending_screens: self.pending_screens.clone(),
            componet_ids: self.componet_ids.clone(),
            config: self.config.clone(),
        }
    }
}

impl VelloEngine {
    pub fn new(config: Option<VelloEngineConfig>) -> Self {
        VelloEngine {
            instance: wgpu::Instance::default(),
            state: None,
            pending_screens: Vec::new(),
            componet_ids: HashSet::new(),
            config: config.unwrap_or_default(),
        }
    }

    pub fn get_render_params(&self) -> RenderParams {
        RenderParams {
            // base_color: self.config.base_color,
            // width: self.config.width,
            // height: self.config.height,
            antialiasing_method: self.config.antialiasing_method,
        }
    }

    fn check_component_id_uniqueness(&self, id: &str) -> bool {
        !self.componet_ids.contains(id)
    }
}

impl TE172Engine for VelloEngine {
    type ScreenType = Screen<VelloEngine>;

    fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(self).unwrap();
    }

    fn add_screen(&mut self, screen: Self::ScreenType) {
        screen.get_ids().iter().for_each(|id| {
            if !self.check_component_id_uniqueness(id) {
                panic!("Component ID '{}' must be unique across all screens", id);
            }
        });

        self.componet_ids.extend(screen.get_ids());

        if let Some(state) = &self.state {
            // If engine is initialized, add directly to render state
            state.borrow_mut().add_screen(screen);
        } else {
            // Otherwise, store in pending list
            self.pending_screens.push(screen);
        }
    }
    fn get_window_dimensions(&self) -> Option<Dimensions> {
        self.state
            .as_ref()
            .map(|state| state.borrow().get_window_dim())
    }

    fn register_screen(&mut self, screen: &mut Self::ScreenType) {
        let screen_clone = screen.clone();
        self.add_screen(screen_clone);
    }

    fn render_screens(&mut self) {
        let render_params = self.get_render_params();
        if let Some(state) = &self.state {
            state.borrow_mut().render_screens(&render_params);
        }
    }
}

impl ApplicationHandler for VelloEngine {
    // Створення вікна та ініціалізація GPU при запуску
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.state = Some(Arc::new(RefCell::new(RenderState::init(
            event_loop,
            self.instance.clone(),
        ))));

        if let Some(state) = &self.state {
            let mut state = state.borrow_mut();
            for screen in self.pending_screens.drain(..) {
                state.add_screen(screen);
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(state) = &self.state {
                    state.borrow_mut().set_window_dim(size.width, size.height);
                    state.borrow_mut().request_redraw();
                }
            }

            WindowEvent::RedrawRequested => {
                let render_params = self.get_render_params();

                if let Some(state) = &self.state {
                    let mut state = state.borrow_mut();
                    state.render_screens(&render_params);

                    // Blit to surface using TextureBlitter
                    let Ok(surface_texture) = state.get_current_texture() else {
                        return;
                    };

                    let render_texture_view =
                        state.get_render_texture().create_view(&Default::default());
                    let surface_view = surface_texture.texture.create_view(&Default::default());
                    let mut encoder = state
                        .get_device()
                        .create_command_encoder(&Default::default());

                    state.get_blitter().copy(
                        state.get_device(),
                        &mut encoder,
                        &render_texture_view,
                        &surface_view,
                    );
                    state.get_queue().submit(std::iter::once(encoder.finish()));
                    surface_texture.present();
                }
            }
            _ => (),
        }
    }
}
