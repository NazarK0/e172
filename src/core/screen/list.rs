use crate::TE172Engine;

pub struct ScreenList<E>
where
    E: TE172Engine + Clone,
{
    screens: Vec<E::ScreenType>,
    engine: E,
}

impl<E> ScreenList<E>
where
    E: TE172Engine + Clone,
{
    pub fn new(engine: E) -> Self {
        Self {
            screens: Vec::new(),
            engine: engine.clone(),
        }
    }

    pub fn push(&mut self, screen: E::ScreenType) {
        self.screens.push(screen);
    }

    pub fn render(mut self) {
        for screen in self.screens.drain(..) {
            self.engine.add_screen(screen);
        }

        self.engine.run();
    }
}
