use kurbo::Affine;
use peniko::{Brush, Fill};
use std::cell::RefCell;
use vello::Scene;

thread_local! {
    static CURRENT_SCENE: RefCell<Option<*mut Scene>> = const { RefCell::new(None) };
}

pub struct VelloRenderBackend;

impl VelloRenderBackend {
    pub fn new() -> Self {
        Self
    }

    pub fn with_scene<F, R>(scene: &mut Scene, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        CURRENT_SCENE.with(|s| {
            // Set the scene pointer
            {
                let mut s_ref = s.borrow_mut();
                *s_ref = Some(scene as *mut Scene);
            } // Mutable borrow is released here

            // Call the closure while the scene is available
            let result = f();

            // Clear the scene pointer
            {
                let mut s_ref = s.borrow_mut();
                *s_ref = None;
            } // Mutable borrow is released here

            result
        })
    }

    pub fn fill(
        &self,
        fill_rule: Fill,
        affine: Affine,
        brush: &Brush,
        alpha: Option<Affine>,
        shape: &impl kurbo::Shape,
    ) {
        CURRENT_SCENE.with(|s| {
            if let Some(scene_ptr) = *s.borrow() {
                unsafe {
                    (*scene_ptr).fill(fill_rule, affine, brush, alpha, shape);
                }
            }
        });
    }
}

impl Default for VelloRenderBackend {
    fn default() -> Self {
        Self::new()
    }
}
