use crate::engine::VelloEngine;
use crate::engine::vello_engine::VelloRenderBackend;
use crate::{Background, Point, TComponent, TE172Engine, TScreen, UiComponentId};

#[derive(Clone)]
pub struct Triangle {
    id: UiComponentId,
    a: Point,
    b: Point,
    c: Point,
    background: Background,
}

pub struct TriangleConfig {
    pub id: Option<UiComponentId>,
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub background: Background,
}

impl Triangle {
    pub fn new(config: TriangleConfig) -> Self {
        Self {
            id: config
                .id
                .unwrap_or_else(|| crate::utils::component_id_creator("triangle")),
            a: config.a,
            b: config.b,
            c: config.c,
            background: config.background,
        }
    }

    pub fn update_position(&mut self, offset: Point) {
        self.a = Point::new(self.a.x + offset.x, self.a.y + offset.y);
        self.b = Point::new(self.b.x + offset.x, self.b.y + offset.y);
        self.c = Point::new(self.c.x + offset.x, self.c.y + offset.y);
    }

    pub fn update_background(&mut self, new_background: Background) {
        self.background = new_background;
    }

    pub fn get_dimensions(&self) -> crate::ui_components::Dimensions {
        let min_x = self.a.x.min(self.b.x).min(self.c.x);
        let max_x = self.a.x.max(self.b.x).max(self.c.x);
        let min_y = self.a.y.min(self.b.y).min(self.c.y);
        let max_y = self.a.y.max(self.b.y).max(self.c.y);

        crate::ui_components::Dimensions {
            width: max_x - min_x,
            height: max_y - min_y,
        }
    }
}

impl TComponent<VelloEngine> for Triangle {
    fn draw(&self, screen: &mut <VelloEngine as TE172Engine>::ScreenType, position: Point) {
        let screen_backend: &mut VelloRenderBackend = screen.get_render_backend();
        let a = Point::new(self.a.x + position.x, self.a.y + position.y);
        let b = Point::new(self.b.x + position.x, self.b.y + position.y);
        let c = Point::new(self.c.x + position.x, self.c.y + position.y);

        screen_backend.fill(
            peniko::Fill::NonZero,
            kurbo::Affine::IDENTITY,
            &self.background.to_brush(position, self.get_dimensions()),
            None,
            &kurbo::Triangle::new(a, b, c),
        );
    }

    fn id(&self) -> &str {
        &self.id
    }
}
