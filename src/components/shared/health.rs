#[derive(Clone, Debug)]
pub struct HealthComponent(pub i32);

use crate::components::{Component, ComponentType};
impl Component for HealthComponent {
	fn purpose() -> ComponentType { ComponentType::Shared }
}

#[allow(dead_code)]
impl HealthComponent {
	pub fn heal(&mut self, healing: i32) -> i32 {
		self.0 += healing;
		self.0
	}
	pub fn damage(&mut self, damage: i32) -> i32 {
		self.0 -= damage;
		self.0
	}
}
