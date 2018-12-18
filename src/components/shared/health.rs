#[derive(Clone, Debug)]
pub struct HealthComponent(pub i32);

use crate::components::{Component, ComponentPurpose};
impl Component for HealthComponent {
	fn purpose() -> ComponentPurpose { ComponentPurpose::Shared }
}

#[allow(dead_code)]
impl HealthComponent {
	pub fn heal(&mut self, healing: i32) {
		self.0 += healing;
	}
	pub fn damage(&mut self, damage: i32) {
		self.0 -= damage;
	}
}
