use crate::components::{Entity, EntityType, Component, ComponentType};

#[derive(Clone, Debug, Deserialize)]
pub struct OwnedComponent {
	pub owner: Entity,
	pub owner_type: EntityType
}

impl Component for OwnedComponent {
	fn purpose() -> ComponentType { ComponentType::Unanimate }
}
