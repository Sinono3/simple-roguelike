use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct OwnerComponent {
	pub contents: Vec<crate::components::Entity>
}

use crate::components::{Component, ComponentType};
impl Component for OwnerComponent {
	fn purpose() -> ComponentType { ComponentType::Shared }
}
