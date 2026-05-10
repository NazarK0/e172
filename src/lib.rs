pub mod engine;
pub use engine::{VelloEngine, VelloEngineConfig};

mod core;
pub use core::{
    AppConfig, Background, Color, ColorStopList, Dimensions, E172App, Gradient, GradientFillMode,
    GradientType, LinearGradientFlow, RadialGradientFlow, Screen, SweepGradientFlow, TComponent,
    TE172Engine, TScreen, WINDOW_HEIGHT_DEFAULT, WINDOW_WIDTH_DEFAULT, color, gradient, utils,
};

pub use core::aliases::*;

pub mod ui_components;
