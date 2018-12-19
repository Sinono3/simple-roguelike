use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct NameComponent(pub String);

use crate::components::{Component, ComponentType};
impl Component for NameComponent {
	fn purpose() -> ComponentType { ComponentType::Shared }
}
