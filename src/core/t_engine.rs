use crate::{Dimensions, TScreen};

pub trait TE172Engine: Clone {
    type ScreenType: TScreen<Self>;

    fn run(&mut self);
    fn add_screen(&mut self, screen: Self::ScreenType);
    fn register_screen(&mut self, screen: &mut Self::ScreenType);
    fn render_screens(&mut self);
    fn get_window_dimensions(&self) -> Option<Dimensions>;
}
