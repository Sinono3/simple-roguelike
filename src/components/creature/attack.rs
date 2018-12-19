use crate::components::unanimate::WieldableComponent;
use crate::components::{Entity, EntityMap, Component, ComponentType};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AttackComponent {
	pub strength: i32,
	pub wielding: Option<Entity>
}

impl Component for AttackComponent {
	fn purpose() -> ComponentType { ComponentType::Creature }
}
impl AttackComponent {
	pub fn damage(&self, unanimate: &EntityMap) -> i32 {
		if let Some(e) = self.wielding {
			self.strength + unanimate.get::<WieldableComponent>(e).unwrap().damage
		} else {
			self.strength
		}
	}
}
