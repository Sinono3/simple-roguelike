#[derive(Clone, Debug)]
pub struct AttackComponent {
	pub damage: i32
}

use crate::components::{Component, ComponentType};
impl Component for AttackComponent {
	fn purpose() -> ComponentType { ComponentType::Creature }
}
