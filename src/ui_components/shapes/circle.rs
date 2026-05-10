use crate::engine::VelloEngine;
use crate::engine::vello_engine::VelloRenderBackend;
use crate::{Background, Point, TComponent, TE172Engine, TScreen, UiComponentId};

#[derive(Clone)]
pub struct Circle {
    id: UiComponentId,
    center: Point,
    radius: f64,
    background: Background,
}

pub struct CircleConfig {
    pub id: Option<UiComponentId>,
    pub radius: f64,
    pub background: Background,
}

impl Circle {
    pub fn new(config: CircleConfig) -> Self {
        Self {
            id: config
                .id
                .unwrap_or_else(|| crate::utils::component_id_creator("circle")),
            center: Point::new(0.0, 0.0),
            radius: config.radius,
            background: config.background,
        }
    }

    pub fn update_position(&mut self, new_position: Point) {
        self.center = new_position;
    }

    pub fn update_radius(&mut self, new_radius: f64) {
        self.radius = new_radius;
    }

    pub fn update_background(&mut self, new_background: Background) {
        self.background = new_background;
    }

    pub fn get_dimensions(&self) -> crate::ui_components::Dimensions {
        crate::ui_components::Dimensions {
            width: self.radius * 2.0,
            height: self.radius * 2.0,
        }
    }
}

impl TComponent<VelloEngine> for Circle {
    fn draw(&self, screen: &mut <VelloEngine as TE172Engine>::ScreenType, position: Point) {
        let screen_backend: &mut VelloRenderBackend = screen.get_render_backend();
        let actual_center = Point::new(self.center.x + position.x, self.center.y + position.y);

        screen_backend.fill(
            peniko::Fill::NonZero,
            kurbo::Affine::IDENTITY,
            &self.background.to_brush(position, self.get_dimensions()),
            None,
            &kurbo::Circle::new(actual_center, self.radius),
        );
    }

    fn id(&self) -> &str {
        &self.id
    }
}
