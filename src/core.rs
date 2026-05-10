mod app_config;
pub use app_config::AppConfig;

pub mod aliases;

mod component_list;
pub use component_list::{ComponentList, ComponentListItem};

mod t_component;
pub use t_component::TComponent;

mod screen;
pub use screen::{Screen, ScreenList, TScreen};

mod t_engine;
pub use t_engine::TE172Engine;

mod dimensions;
pub use dimensions::Dimensions;

mod constants;
pub use constants::*;

mod app;
pub use app::E172App;

pub mod utils;

pub mod color;
pub use color::Color;

pub mod gradient;
pub use gradient::{
    Gradient, GradientFillMode, GradientType, LinearGradientFlow, RadialGradientFlow,
    SweepGradientFlow,
};

mod background;
pub use background::Background;

mod color_stop_list;
pub use color_stop_list::ColorStopList;
