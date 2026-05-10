use crate::TE172Engine;

pub trait TScreen<E: TE172Engine>: Clone {
    fn render(&mut self);
    fn get_render_backend<B>(&mut self) -> &mut B; // where B: RenderBackend<EngineType = E>;
}
