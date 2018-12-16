use super::Component;

#[derive(Clone)]
pub struct AttackComponent { pub damage: i32 }

impl From<AttackComponent> for Component {
	fn from(component: AttackComponent) -> Self { Component::Attack }
}
