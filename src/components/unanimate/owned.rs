use crate::components::Entity;
use crate::components::{Component, ComponentType};

#[derive(Clone, Debug)]
pub struct OwnedComponent {
	pub owner: Entity,
}

impl Component for OwnedComponent {
	fn purpose() -> ComponentType { ComponentType::Shared }
}
