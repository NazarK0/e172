use crate::{ColorStopList, Point};

#[derive(Debug, Clone)]
pub struct Gradient {
    gradient_type: GradientType,
    fill_mode: GradientFillMode,
    color_stops: ColorStopList,
}

#[derive(Debug, Clone)]
pub enum GradientType {
    Linear(LinearGradientFlow),
    Radial(RadialGradientFlow),
    Sweep(SweepGradientFlow),
}

#[derive(Debug, Clone)]
pub struct LinearGradientFlow {
    pub angle: f64,
    pub relative_width: f64,
}

#[derive(Debug, Clone)]
pub struct RadialGradientFlow {
    pub relative_center: Point,
    pub relative_radius: f64,
}

#[derive(Debug, Clone)]
pub struct SweepGradientFlow {
    pub relative_center: Point,
    pub start_angle: f64,
    pub end_angle: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GradientFillMode {
    Repeat,
    Pad,
    Reflect,
}

impl Gradient {
    pub fn new(gr_type: GradientType, stops: ColorStopList, fill_mode: GradientFillMode) -> Self {
        Self {
            gradient_type: gr_type,
            fill_mode,
            color_stops: stops,
        }
    }

    pub fn gradient_type(&self) -> &GradientType {
        &self.gradient_type
    }

    pub fn fill_mode(&self) -> &GradientFillMode {
        &self.fill_mode
    }

    pub fn color_stops(&self) -> &ColorStopList {
        &self.color_stops
    }
}
