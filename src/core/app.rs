use crate::{AppConfig, TE172Engine};

pub struct E172App<E>
where
    E: TE172Engine + Clone,
{
    screens: super::ScreenList<E>,
}

impl<E> E172App<E>
where
    E: TE172Engine + Clone,
{
    pub fn new(config: AppConfig<E>) -> Self {
        let engine = config.engine;
        Self {
            screens: super::ScreenList::new(engine),
        }
    }

    pub fn add_screen(&mut self, screen: E::ScreenType) {
        self.screens.push(screen);
    }

    pub fn run(self) {
        self.screens.render();
    }
}
