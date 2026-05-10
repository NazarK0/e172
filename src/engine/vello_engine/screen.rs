use crate::engine::VelloEngine;
use crate::{Point, Screen, TScreen};

impl TScreen<VelloEngine> for Screen<VelloEngine> {
    fn render(&mut self) {
        // Collect component IDs and positions before drawing
        let items: Vec<(String, Point)> = self
            .get_all_components()
            .iter()
            .map(|item| (item.component.id().to_string(), item.position))
            .collect();

        for (id, position) in items {
            // Clone self temporarily to avoid borrow issues
            let components = self.get_all_components().clone();
            if let Some(component) = components.iter().find(|i| i.component.id() == id) {
                component.component.draw(self, position);
            }
        }
    }

    fn get_render_backend<B>(&mut self) -> &mut B {
        use crate::engine::vello_engine::VelloRenderBackend;

        unsafe {
            let backend = Box::new(VelloRenderBackend::new());
            &mut *(Box::leak(backend) as *mut _ as *mut B)
        }
    }
}
