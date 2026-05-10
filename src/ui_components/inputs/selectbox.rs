use crate::UiComponentId;

#[derive(Clone)]
pub struct SelectBox {
    pub id: UiComponentId,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub options: Vec<String>,
}
