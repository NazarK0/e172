use crate::UiComponentId;
use uuid::Uuid;

pub fn component_id_creator(component: &str) -> UiComponentId {
    format!("{}-{}", component, Uuid::new_v4())
}
