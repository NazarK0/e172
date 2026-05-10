use crate::{Point, TE172Engine};
use dyn_clone::{DynClone, clone_trait_object};

pub trait TComponent<E: TE172Engine>: DynClone {
    fn draw(&self, screen: &mut E::ScreenType, position: Point);
    fn id(&self) -> &str;
}

clone_trait_object!(<E> TComponent<E>);
