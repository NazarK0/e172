use crate::{Color, Gradient, Point};

#[derive(Debug, Clone)]
pub enum Background {
    Solid(Color),
    Gradient(Gradient),
}

impl Background {
    pub fn to_brush(
        &self,
        position: Point,
        dimensions: crate::ui_components::Dimensions,
    ) -> peniko::Brush {
        match self {
            Background::Solid(color) => peniko::Brush::Solid(*&color.get()),
            Background::Gradient(gradient) => {
                let color_stops = gradient.color_stops().clone();
                let fill_mode = gradient.fill_mode();

                let gradient = match &gradient.gradient_type() {
                    super::GradientType::Linear(flow) => {
                        let angle = flow.angle.to_degrees();
                        let direction = Point::new(
                            angle.to_radians().cos() as f64,
                            angle.to_radians().sin() as f64,
                        );
                        let start = Point::new(
                            position.x - direction.x * flow.relative_width * dimensions.width / 2.0,
                            position.y
                                - direction.y * flow.relative_width * dimensions.height / 2.0,
                        );
                        let end = Point::new(
                            position.x + direction.x * flow.relative_width * dimensions.width / 2.0,
                            position.y
                                + direction.y * flow.relative_width * dimensions.height / 2.0,
                        );

                        peniko::Gradient::new_linear(start, end)
                    }

                    super::GradientType::Radial(flow) => {
                        let center = Point::new(
                            position.x + flow.relative_center.x,
                            position.y + flow.relative_center.y,
                        );
                        let radius: f32 = (flow.relative_radius
                            * dimensions.width.max(dimensions.height)
                            / 2.0) as f32;

                        peniko::Gradient::new_radial(center, radius)
                    }

                    super::GradientType::Sweep(flow) => {
                        let center = Point::new(
                            position.x + flow.relative_center.x,
                            position.y + flow.relative_center.y,
                        );
                        peniko::Gradient::new_sweep(
                            center,
                            flow.start_angle as f32,
                            flow.end_angle as f32,
                        )
                    }
                };

                let gradient =
                    gradient
                        .with_stops(&color_stops.into()[..])
                        .with_extend(match fill_mode {
                            super::GradientFillMode::Repeat => peniko::Extend::Repeat,
                            super::GradientFillMode::Pad => peniko::Extend::Pad,
                            super::GradientFillMode::Reflect => peniko::Extend::Reflect,
                        });

                gradient.into()
            }
        }
    }
}
