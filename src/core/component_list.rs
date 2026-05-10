use crate::{Point, TComponent, TE172Engine};

pub struct ComponentListItem<E: TE172Engine> {
    pub position: Point,
    pub component: Box<dyn TComponent<E>>,
}

impl<E: TE172Engine> Clone for ComponentListItem<E> {
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            component: dyn_clone::clone_box(&*self.component),
        }
    }
}

#[derive(Clone)]
pub struct ComponentList<E: TE172Engine>(Vec<ComponentListItem<E>>);

impl<E: TE172Engine> ComponentList<E> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push<C: TComponent<E> + 'static>(&mut self, component: C, position: Point) {
        self.0.push(ComponentListItem {
            position,
            component: Box::new(component),
        });
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ComponentListItem<E>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, ComponentListItem<E>> {
        self.0.iter_mut()
    }
}
