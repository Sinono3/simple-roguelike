use crate::components::{Component, ComponentType};

#[derive(Clone, Debug)]
pub struct WieldableComponent {
	pub damage: i32
}

impl Component for WieldableComponent {
	fn purpose() -> ComponentType { ComponentType::Unanimate }
}
