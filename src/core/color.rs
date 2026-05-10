use crate::core::aliases::HexColor;

pub use peniko::color::palette::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    color: peniko::Color,
}

impl Color {
    pub fn get(&self) -> peniko::Color {
        self.color
    }

    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            color: peniko::Color::from_rgba8(red, green, blue, alpha),
        }
    }

    pub fn from_hex(color: HexColor) -> Self {
        let color = color.trim_start_matches('#');

        if color.len() != 6 && color.len() != 8 {
            panic!("Hex color must be in the format #RRGGBB or #RRGGBBAA");
        }

        let red = u8::from_str_radix(&color[0..2], 16).unwrap_or(0);
        let green = u8::from_str_radix(&color[2..4], 16).unwrap_or(0);
        let blue = u8::from_str_radix(&color[4..6], 16).unwrap_or(0);

        if color.len() == 6 {
            return Self {
                color: peniko::Color::from_rgb8(red, green, blue),
            };
        } else if color.len() == 8 {
            let alpha = u8::from_str_radix(&color[6..8], 16).unwrap_or(255);

            return Self {
                color: peniko::Color::from_rgba8(red, green, blue, alpha),
            };
        }

        Self {
            color: peniko::Color::from_rgb8(0, 0, 0),
        }
    }

    pub fn from_hex2(color: HexColor, alpha: f64) -> Self {
        let color = color.trim_start_matches('#');

        if color.len() != 6 {
            panic!("Hex color must be in the format #RRGGBB");
        }

        let red = u8::from_str_radix(&color[0..2], 16).unwrap_or(0);
        let green = u8::from_str_radix(&color[2..4], 16).unwrap_or(0);
        let blue = u8::from_str_radix(&color[4..6], 16).unwrap_or(0);

        if alpha < 0.0 || alpha > 1.0 {
            panic!("Alpha value must be between 0.0 and 1.0");
        }

        let alpha = (alpha * 255.0) as u8;

        Self {
            color: peniko::Color::from_rgba8(red, green, blue, alpha),
        }
    }

    pub fn from_hsla(hue: f64, saturation: f64, lightness: f64, alpha: f64) -> Self {
        if hue < 0.0 || hue >= 360.0 {
            panic!("Hue value must be between 0.0 and 360.0");
        }
        if saturation < 0.0 || saturation > 1.0 {
            panic!("Saturation value must be between 0.0 and 1.0");
        }
        if lightness < 0.0 || lightness > 1.0 {
            panic!("Lightness value must be between 0.0 and 1.0");
        }
        if alpha < 0.0 || alpha > 1.0 {
            panic!("Alpha value must be between 0.0 and 1.0");
        }

        let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
        let intermidiate = chroma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let value = lightness - chroma / 2.0;

        let red = (chroma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs()) + value) * 255.0;
        let green = (intermidiate + value) * 255.0;
        let blue = (value) * 255.0;
        let alpha = alpha * 255.0;

        Self {
            color: peniko::Color::from_rgba8(red as u8, green as u8, blue as u8, alpha as u8),
        }
    }

    pub fn from_hsva(hue: f64, saturation: f64, value: f64, alpha: f64) -> Self {
        if hue < 0.0 || hue >= 360.0 {
            panic!("Hue value must be between 0.0 and 360.0");
        }
        if saturation < 0.0 || saturation > 1.0 {
            panic!("Saturation value must be between 0.0 and 1.0");
        }
        if value < 0.0 || value > 1.0 {
            panic!("Value must be between 0.0 and 1.0");
        }
        if alpha < 0.0 || alpha > 1.0 {
            panic!("Alpha value must be between 0.0 and 1.0");
        }

        let chroma = value * saturation;
        let intermidiate = chroma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = value - chroma;

        let red = (chroma + m) * 255.0;
        let green = (intermidiate + m) * 255.0;
        let blue = (m) * 255.0;
        let alpha = alpha * 255.0;

        Self {
            color: peniko::Color::from_rgba8(red as u8, green as u8, blue as u8, alpha as u8),
        }
    }

    pub fn from_css(color: ColorCSS) -> Self {
        match color {
            ColorCSS::Red => Self::from_hex("#FF0000"),
            ColorCSS::Green => Self::from_hex("#00FF00"),
            ColorCSS::Blue => Self::from_hex("#0000FF"),
            ColorCSS::Yellow => Self::from_hex("#FFFF00"),
            ColorCSS::Cyan => Self::from_hex("#00FFFF"),
            ColorCSS::Magenta => Self::from_hex("#FF00FF"),
            ColorCSS::Black => Self::from_hex("#000000"),
            ColorCSS::White => Self::from_hex("#FFFFFF"),
            ColorCSS::Lime => Self::from_hex("#00FF00"),
            ColorCSS::Transparent => Self::from_rgba(0, 0, 0, 0),
        }
    }

    pub fn set_alpha(&self, alpha: f64) -> Self {
        if alpha < 0.0 || alpha > 1.0 {
            panic!("Alpha value must be between 0.0 and 1.0");
        }

        let new_color = self.color.multiply_alpha(alpha as f32);

        Self { color: new_color }
    }

    pub fn set_alpha_mut(&mut self, alpha: f64) {
        // if alpha < 0.0 || alpha > 1.0 {
        //     panic!("Alpha value must be between 0.0 and 1.0");
        // }

        self.color = self.color.with_alpha(alpha as f32);
    }
}

pub enum ColorCSS {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Black,
    White,
    Lime,
    Transparent,
}
