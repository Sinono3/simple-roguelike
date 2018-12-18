#[derive(Clone, Debug)]
pub struct AttackComponent {
	pub damage: i32
}

use crate::components::{Component, ComponentPurpose};
impl Component for AttackComponent {
	fn purpose() -> ComponentPurpose { ComponentPurpose::Creature }
}
