pub mod inputs;
pub mod shapes;

mod text;
pub use text::Text;

pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}
