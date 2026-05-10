use crate::engine::VelloEngine;
use crate::engine::vello_engine::VelloRenderBackend;
use crate::{Background, Point, TComponent, TE172Engine, TScreen, UiComponentId};

#[derive(Clone)]
pub struct Rectangle {
    id: UiComponentId,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    background: Background,
}

pub struct RectangleConfig {
    pub id: Option<UiComponentId>,
    pub width: f64,
    pub height: f64,
    pub background: Background,
}

impl Rectangle {
    pub fn new(config: RectangleConfig) -> Self {
        Self {
            id: config
                .id
                .unwrap_or_else(|| crate::utils::component_id_creator("rectangle")),
            x: 0.0,
            y: 0.0,
            width: config.width,
            height: config.height,
            background: config.background,
        }
    }

    pub fn update_position(&mut self, new_position: Point) {
        self.x = new_position.x;
        self.y = new_position.y;
    }

    pub fn update_dimensions(&mut self, new_width: f64, new_height: f64) {
        self.width = new_width;
        self.height = new_height;
    }

    pub fn update_background(&mut self, new_background: Background) {
        self.background = new_background;
    }

    pub fn get_dimensions(&self) -> crate::ui_components::Dimensions {
        crate::ui_components::Dimensions {
            width: self.width,
            height: self.height,
        }
    }
}

impl TComponent<VelloEngine> for Rectangle {
    fn draw(&self, screen: &mut <VelloEngine as TE172Engine>::ScreenType, position: Point) {
        let screen_backend: &mut VelloRenderBackend = screen.get_render_backend();
        let x1 = self.x + position.x;
        let y1 = self.y + position.y;

        screen_backend.fill(
            peniko::Fill::NonZero,
            kurbo::Affine::IDENTITY,
            &self.background.to_brush(position, self.get_dimensions()),
            None,
            &kurbo::Rect::new(x1, y1, x1 + self.width, y1 + self.height),
        );
    }

    fn id(&self) -> &str {
        &self.id
    }
}
