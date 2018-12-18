use crate::components::{Entity, EntityType, Component, ComponentType};

#[derive(Clone, Debug)]
pub struct OwnedComponent {
	pub owner: Entity,
	pub entity_type: EntityType
}

impl Component for OwnedComponent {
	fn purpose() -> ComponentType { ComponentType::Unanimate }
}
