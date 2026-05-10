use crate::engine::VelloEngine;
use crate::{
    Background, Point, TComponent, TE172Engine, UiComponentId,
    ui_components::shapes::{Rectangle, RectangleConfig},
};

#[derive(Clone)]
pub struct SquareConfig {
    pub id: Option<UiComponentId>,
    pub side: f64,
    pub background: Background,
}

#[derive(Clone)]
pub struct Square {
    shape: Rectangle,
    #[allow(dead_code)]
    side: f64,
}

impl Square {
    pub fn new(config: SquareConfig) -> Self {
        Self {
            side: config.side,
            shape: Rectangle::new(RectangleConfig {
                id: config.id,
                width: config.side,
                height: config.side,
                background: config.background.clone(),
            }),
        }
    }
}

impl TComponent<VelloEngine> for Square {
    fn draw(&self, screen: &mut <VelloEngine as TE172Engine>::ScreenType, position: Point) {
        self.shape.draw(screen, position);
    }

    fn id(&self) -> &str {
        self.shape.id()
    }
}
