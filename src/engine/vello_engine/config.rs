use vello::AaConfig;

#[derive(Clone)]
pub struct VelloEngineConfig {
    pub antialiasing_method: AaConfig,
}

impl Default for VelloEngineConfig {
    fn default() -> Self {
        Self {
            antialiasing_method: AaConfig::Msaa16,
        }
    }
}
