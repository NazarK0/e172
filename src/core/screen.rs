use std::collections::HashSet;

use crate::core::{ComponentList, ComponentListItem};
use crate::{
    Dimensions, Point, TComponent, TE172Engine, UiComponentId, WINDOW_HEIGHT_DEFAULT,
    WINDOW_WIDTH_DEFAULT,
};

mod t_screen;
pub use t_screen::TScreen;

mod list;
pub use list::ScreenList;

#[derive(Clone)]
pub struct Screen<E: TE172Engine> {
    dimensions: Dimensions,
    components: ComponentList<E>,
    componet_ids: HashSet<UiComponentId>,
}

impl<E: TE172Engine> Screen<E> {
    pub fn new(dimensions: Option<Dimensions>) -> Self {
        Self {
            dimensions: dimensions
                .unwrap_or_else(|| Dimensions::new(WINDOW_WIDTH_DEFAULT, WINDOW_HEIGHT_DEFAULT)),
            components: ComponentList::new(),
            componet_ids: HashSet::new(),
        }
    }

    pub fn add_component<C: TComponent<E> + 'static>(&mut self, component: C, position: Point) {
        if !self.componet_ids.insert(component.id().to_string()) {
            panic!(
                "Component ID must be unique within a screen: '{}' is already in use",
                component.id()
            );
        }
        self.components.push(component, position);
    }

    pub fn get_component_by_id(&self, id: &str) -> Option<&ComponentListItem<E>> {
        self.components
            .iter()
            .find(|item| item.component.id() == id)
    }

    pub fn get_component_by_id_mut(&mut self, id: &str) -> Option<&mut ComponentListItem<E>> {
        self.components
            .iter_mut()
            .find(|item| item.component.id() == id)
    }

    pub fn get_all_components(&self) -> &ComponentList<E> {
        &self.components
    }

    pub fn get_all_components_mut(&mut self) -> &mut ComponentList<E> {
        &mut self.components
    }

    pub fn get_ids(&self) -> HashSet<UiComponentId> {
        self.componet_ids.clone()
    }

    pub fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }
}
