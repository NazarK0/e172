use crate::TE172Engine;
use crate::engine::VelloEngine;

#[derive(Clone)]
pub struct AppConfig<E: TE172Engine> {
    pub title: String,
    pub engine: E,
}

impl Default for AppConfig<VelloEngine> {
    fn default() -> Self {
        Self {
            title: String::from("E172 App"),
            engine: VelloEngine::new(None),
        }
    }
}
