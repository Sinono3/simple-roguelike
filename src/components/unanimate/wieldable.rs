use crate::components::{Component, ComponentType};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct WieldableComponent {
	pub damage: i32
}

impl Component for WieldableComponent {
	fn purpose() -> ComponentType { ComponentType::Unanimate }
}
